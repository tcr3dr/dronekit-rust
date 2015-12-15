extern crate mio;
extern crate bit_vec;

use mavlink::*;
use crc16;
use std::iter::FromIterator;

use std::num::Wrapping;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use std::fs::File;
use std::io::BufReader;
use mio::{TryRead, TryWrite};
use mio::tcp::TcpStream;
use mio::util::Slab;
use bytes::Buf;
use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;
use std::collections::{HashMap, VecDeque};
use std::iter::repeat;
use std::cmp::max;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, TryRecvError};
use std::cell::RefCell;
use std::rc::Rc;
use eventual::{Future, Async};
use bit_vec::BitVec;

pub const CLIENT: mio::Token = mio::Token(0);

pub type UpdaterList = Vec<Box<FnMut(DkMessage) -> bool>>;

#[derive(Debug)]
struct MavPacket {
    seq: u8,
    system_id: u8,
    component_id: u8,
    message_id: u8,
    data: Vec<u8>,
    checksum: u16,
}

impl MavPacket {
    fn new(payload: &[u8]) -> MavPacket {
        let mut cur = Cursor::new(payload);
        cur.set_position(2);
        MavPacket {
            seq: cur.read_u8().unwrap(),
            system_id: cur.read_u8().unwrap(),
            component_id: cur.read_u8().unwrap(),
            message_id: cur.read_u8().unwrap(),
            data: payload[6..payload.len()-2].to_vec(),
            checksum: {
                cur.set_position((payload.len() - 2) as u64);
                cur.read_u16::<LittleEndian>().unwrap()
            },
        }
    }

    fn parse(&self) -> Option<DkMessage> {
        DkMessage::parse(self.message_id, &self.data)
    }

    fn encode_nocrc(&self) -> Vec<u8> {
        let mut pkt: Vec<u8> = vec![
            0xfe, self.data.len() as u8, self.seq,
            self.system_id, self.component_id, self.message_id,
        ];
        pkt.extend(&self.data);
        pkt
    }

    fn encode(&self) -> Vec<u8> {
        let mut pkt = self.encode_nocrc();
        pkt.push((self.checksum & 0xff) as u8);
        pkt.push((self.checksum >> 8) as u8);
        pkt
    }

    fn calc_crc(&self) -> u16 {
        let mut crc = crc16::State::<crc16::MCRF4XX>::new();
        crc.update(&self.encode_nocrc()[1..]);
        crc.update(&[DkMessage::extra_crc(self.message_id)]);
        crc.get()
    }

    fn update_crc(&mut self) {
        self.checksum = self.calc_crc();
    }

    fn check_crc(&self) -> bool {
        self.calc_crc() == self.checksum
    }
}

pub fn parse_mavlink_string(buf: &[u8]) -> String {
    buf.iter()
        .take_while(|a| **a != 0)
        .map(|x| *x as char)
        .collect::<String>()
}

pub struct DkHandler {
    pub socket: TcpStream,
    pub buf: Vec<u8>,
    pub vehicle_tx: Sender<DkHandlerRx>,
    pub watchers: UpdaterList,
}

pub enum DkHandlerRx {
    RxCork,
    RxMessage(DkMessage),
}

pub enum DkHandlerMessage {
    TxMessage(Vec<u8>),
    TxWatcher(Box<FnMut(DkMessage) -> bool + Send>),
    TxCork,
    TxUncork,
}

impl DkHandler {
    fn dispatch(&mut self, pkt: DkMessage) {
        let pkt2 = pkt.clone();
        self.vehicle_tx.send(DkHandlerRx::RxMessage(pkt));

        let mut ups = self.watchers.split_off(0);
        for mut x in ups.into_iter() {
            if x(pkt2.clone()) {
                self.watchers.push(x);
            }
        }
    }

    pub fn register(&mut self, event_loop: &mut mio::EventLoop<DkHandler>) {
        event_loop.register_opt(&self.socket, CLIENT,
            mio::EventSet::readable(),
            mio::PollOpt::edge()).unwrap();
    }

    pub fn deregister(&mut self, event_loop: &mut mio::EventLoop<DkHandler>) {
        event_loop.deregister(&self.socket);
    }
}

impl mio::Handler for DkHandler {
    type Timeout = ();
    type Message = DkHandlerMessage;

    fn ready(&mut self, event_loop: &mut mio::EventLoop<DkHandler>, token: mio::Token, events: mio::EventSet) {
        match token {
            CLIENT => {
                // Only receive readable events
                assert!(events.is_readable());

                // println!("the socket socket is ready to accept a connection");
                // match self.socket.accept() {
                //     Ok(Some(socket)) => {
                //         println!("accepted a socket, exiting program");
                //         event_loop.shutdown();
                //     }
                //     Ok(None) => {
                //         println!("the socket socket wasn't actually ready");
                //     }
                //     Err(e) => {
                //         println!("listener.accept() errored: {}", e);
                //         event_loop.shutdown();
                //     }
                // }

                match self.socket.try_read_buf(&mut self.buf) {
                    Ok(Some(0)) => {
                        unimplemented!();
                    }
                    Ok(Some(n)) => {
                        // crc16::State::<crc16::MCRF4XX>::calculate()
                        let mut start: usize = 0;
                        loop {
                            match self.buf[start..].iter().position(|&x| x == 0xfe) {
                                Some(i) => {
                                    // println!("from: {:?} {:?}", start + i, self.buf);
                                    if start + i + 8 > self.buf.len() {
                                        break;
                                    }

                                    let len = self.buf[start + i + 1] as usize;

                                    if start + i + 8 + len > self.buf.len() {
                                        break;
                                    }

                                    let packet;
                                    {
                                        let pktbuf = &self.buf[(start + i)..(start + i + 8 + len)];
                                        packet = MavPacket::new(pktbuf);

                                        // println!("ok {:?}", pktbuf);

                                        if !packet.check_crc() {
                                            // println!("failed CRC!");
                                            start += i + 1;
                                            continue;
                                        }
                                    }

                                    // handle packet
                                    if let Some(pkt) = packet.parse() {
                                        self.dispatch(pkt);
                                    }
                                    
                                    // change this
                                    start += i + 8 + len;
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                        self.buf = self.buf.split_off(start);

                        // Re-register the socket with the event loop. The current
                        // state is used to determine whether we are currently reading
                        // or writing.
                        // self.reregister(event_loop);
                    }
                    Ok(None) => {
                        // self.reregister(event_loop);
                    }
                    Err(e) => {
                        panic!("got an error trying to read; err={:?}", e);
                    }
                }
            }
            _ => panic!("Received unknown token"),
        }
    }

    fn notify(&mut self, event_loop: &mut mio::EventLoop<DkHandler>, message: DkHandlerMessage) {
        match message {
            DkHandlerMessage::TxMessage(msg) => {
                self.socket.try_write_buf(&mut Cursor::new(msg));
            }
            DkHandlerMessage::TxWatcher(func) => {
                self.watchers.push(func);
            }
            DkHandlerMessage::TxCork => {
                self.deregister(event_loop);
                self.vehicle_tx.send(DkHandlerRx::RxCork);
            }
            DkHandlerMessage::TxUncork => {
                self.register(event_loop);
            }
        }
    }
}

pub struct VehicleConnection {
    pub tx: mio::Sender<DkHandlerMessage>,
    pub rx: Receiver<DkHandlerRx>,
    pub msg_id: u8,
    pub started: bool,
    pub buffer: VecDeque<DkMessage>,
}

impl VehicleConnection {
    // fn tick(&mut self) {
    //     println!("tick. location: {:?}", self.vehicle.location_global);
    // }

    pub fn cork(&mut self) -> Vec<DkMessage> {
        self.tx.send(DkHandlerMessage::TxCork);

        loop {
            match self.rx.recv() {
                Ok(DkHandlerRx::RxCork) => {
                    break;
                }
                Ok(DkHandlerRx::RxMessage(msg)) => {
                    self.buffer.push_back(msg);
                }
                _ => {},
            }
        }

        self.buffer.clone().into_iter().collect()
    }

    pub fn uncork(&mut self) {
        self.tx.send(DkHandlerMessage::TxUncork);
    }

    pub fn recv(&mut self) -> Result<DkHandlerRx, RecvError> {
        if let Some(msg) = self.buffer.pop_front() {
            Ok(DkHandlerRx::RxMessage(msg))
        } else {
            self.rx.recv()
        }
    }

    pub fn try_recv(&mut self) -> Result<DkHandlerRx, TryRecvError> {
        if let Some(msg) = self.buffer.pop_front() {
            Ok(DkHandlerRx::RxMessage(msg))
        } else {
            self.rx.try_recv()
        }
    }

    pub fn send(&mut self, data: DkMessage) {
        let mut pkt = MavPacket {
            seq: self.msg_id,
            system_id: 255,
            component_id: 0,
            message_id: data.message_id(),
            data: data.serialize(),
            checksum: 0,
        };
        pkt.update_crc();
        let out = pkt.encode();
        // let outlen = out.len();

        self.msg_id = self.msg_id.wrapping_add(1);

        // println!(">>> {:?}", out);
        // let mut cur = Cursor::new(out);
        self.tx.send(DkHandlerMessage::TxMessage(out));
        // (outlen, self.socket.try_write_buf(&mut cur))
    }
}
