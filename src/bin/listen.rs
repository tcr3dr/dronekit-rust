extern crate dronekit;
extern crate mio;
extern crate bytes;
extern crate crc16;
extern crate byteorder;
extern crate time;

use dronekit::*;
use dronekit::mavlink::*;

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
use std::collections::HashMap;
use std::iter::repeat;
use std::cmp::max;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

const CLIENT: mio::Token = mio::Token(0);

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

fn parse_mavlink_string(buf: &[u8]) -> String {
    buf.iter()
        .take_while(|a| **a != 0)
        .map(|x| *x as char)
        .collect::<String>()
}

#[derive(Clone, Debug)]
struct LocationGlobal {
    alt: i32, lat: i32, lon: i32
}

#[derive(Clone, Debug)]
struct Vehicle {
    parameters: Parameters,
    location_global: Option<LocationGlobal>,
}

impl Vehicle {
    fn new() -> Vehicle {
        Vehicle {
            parameters: Parameters::new(),
            location_global: None,
        }
    }
}

struct DkHandler {
    socket: TcpStream,
    buf: Vec<u8>,
    vehicle_tx: Sender<DkMessage>,
}

#[derive(Clone, Debug)]
struct Parameters {
    values: HashMap<String, f32>,
    indexes: Vec<Option<String>>
}

impl Parameters {
    fn new() -> Parameters {
        Parameters {
            values: HashMap::new(),
            indexes: vec![],
        }
    }

    fn resize(&mut self, len: u16) {
        if self.indexes.len() != len as usize {
            self.values = HashMap::new();
            self.indexes = repeat(None).take(len as usize).collect();
        }
    }

    fn set(&mut self, index: u16, name: String, value: f32) {
        self.values.insert(name.clone(), value);
        self.indexes[index as usize] = Some(name);
    }

    fn complete(&self) -> bool {
        self.indexes.iter().position(|x| x.is_none()).is_none()
    }

    fn remaining(&self) -> usize {
        self.indexes.iter().filter(|x| x.is_some()).count()
    }

    fn available(&self) -> usize {
        self.indexes.iter().filter(|x| x.is_some()).count()
    }
}

struct DkThread {
    vehicle: Vehicle,
    vehicle_tx: mio::Sender<Vec<u8>>,
    msg_id: u8,
    started: bool,
}

impl DkThread {
    fn tick(&mut self) {
        println!("tick. location: {:?}", self.vehicle.location_global);
    }

    fn send(&mut self, data: DkMessage) {
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
        self.vehicle_tx.send(out);
        // (outlen, self.socket.try_write_buf(&mut cur))
    }

    fn on_message(&mut self, pkt: DkMessage) {
        match pkt {
            DkMessage::HEARTBEAT(..) => {
                self.send(DkMessage::HEARTBEAT(HEARTBEAT_DATA {
                    custom_mode: 0,
                    mavtype: 6,
                    autopilot: 8,
                    base_mode: 0,
                    system_status: 0,
                    mavlink_version: 0x3,
                }));

                // if let Ok(Some(n)) = res {
                //     if n != outlen {
                //         println!("ERROR: only wrote {:?}", n);
                //     }
                // } else {
                //     println!("ERROR: didnt write anything");
                // }

                if !self.started {
                    self.started = true;

                    self.send(DkMessage::PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA {
                        target_system: 0,
                        target_component: 0,
                    }));

                    self.send(DkMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA {
                        target_system: 0,
                        target_component: 0,
                        req_stream_id: 0,
                        req_message_rate: 100,
                        start_stop: 1,
                    }));
                    // println!("start params {:?}", res);
                }
            },
            DkMessage::STATUSTEXT(data) => {
                let text = parse_mavlink_string(&data.text);
                println!("<<< [{:?}] {:?}", data.severity, text);
            },
            DkMessage::PARAM_VALUE(data) => {
                self.vehicle.parameters.resize(data.param_count);
                self.vehicle.parameters.set(data.param_index, parse_mavlink_string(&data.param_id), data.param_value);
            },
            DkMessage::ATTITUDE(data) => {
                // println!("roll: {:?}\tpitch: {:?}\tyaw: {:?}", data.roll, data.pitch, data.yaw);
            },
            DkMessage::GLOBAL_POSITION_INT(data) => {
                self.vehicle.location_global = Some(LocationGlobal {
                    lat: data.lat,
                    lon: data.lon,
                    alt: data.alt,
                });
            },
            _ => {
                // println!("dunno: {:?}", pkt);
            },
        }
    }
}

impl mio::Handler for DkHandler {
    type Timeout = ();
    type Message = Vec<u8>;

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
                                        self.vehicle_tx.send(pkt);
                                    }
                                    
                                    // change this
                                    start += i + 8 + len;
                                },
                                None => {
                                    self.buf = self.buf.split_off(start);
                                    break;
                                }
                            }
                        }

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

    fn notify(&mut self, event_loop: &mut mio::EventLoop<DkHandler>, msg: Vec<u8>) {
        self.socket.try_write_buf(&mut Cursor::new(msg));
    }
}

fn run(address: SocketAddr) {
    // Create a new event loop, panic if this fails.
    let socket = match TcpStream::connect(&address) {
        Ok(socket) => socket,
        Err(e) => {
            // If the connect fails here, then usually there is something
            // wrong locally. Though, on some operating systems, attempting
            // to connect to a localhost address completes immediately.
            panic!("failed to create socket; err={:?}", e);
        }
    };

    let mut event_loop = mio::EventLoop::new().unwrap();
    event_loop.register_opt(&socket, CLIENT,
        mio::EventSet::readable(),
        mio::PollOpt::edge()).unwrap();

    // let sender = event_loop.channel();
    // // Send the notification from another thread

    let (tx, rx) = channel();
    let vehicle_tx = event_loop.channel();

    thread::spawn(move || {
        let mut t = DkThread {
            vehicle: Vehicle::new(),
            vehicle_tx: vehicle_tx,
            msg_id: 0,
            started: false,
        };
        loop {
            thread::sleep_ms((1000.0 / 10.0) as u32);

            for i in 0..256 {
                if let Ok(pkt) = rx.try_recv() {
                    t.on_message(pkt);
                }
            }
            t.tick();
        }
    });

    thread::spawn(move || {
        println!("running pingpong socket");
        event_loop.run(&mut DkHandler {
            socket: socket,
            buf: vec![],
            vehicle_tx: tx,
        });
    }).join();
}

pub fn main() {
    let file = File::open("solo.xml").unwrap();
    let file = BufReader::new(file);
    let profile = parse_profile(Box::new(file));

    run("127.0.0.1:5760".parse().unwrap());
}
