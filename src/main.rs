extern crate dronekit;
extern crate mio;
extern crate bytes;
extern crate crc16;
extern crate byteorder;

use dronekit::*;

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

const CLIENT: mio::Token = mio::Token(0);

struct MavPacket {
    payload: Vec<u8>,
}

trait Parsable {
    fn parse(payload: &[u8]) -> Self;
}

#[derive(Clone, Copy, Debug)]
struct MAV_HEARTBEAT {
    mtype: u8,
    autopilot: u8,
    base_mode: u8,
    custom_mode: u32,
    system_status: u8,
    mavlink_version: u8,
}

impl Parsable for MAV_HEARTBEAT {
    fn parse(payload: &[u8]) -> MAV_HEARTBEAT {
        let mut cur = Cursor::new(payload);
        MAV_HEARTBEAT {
            custom_mode: cur.read_u32::<LittleEndian>().unwrap(),
            mtype: cur.read_u8().unwrap(),
            autopilot: cur.read_u8().unwrap(),
            base_mode: cur.read_u8().unwrap(),
            system_status: cur.read_u8().unwrap(),
            mavlink_version: cur.read_u8().unwrap(),
        }
    }
}

impl MavPacket {
    fn new(payload: &[u8]) -> MavPacket {
        MavPacket {
            payload: payload.into(),
        }
    }

    fn parse<A: Parsable>(&self) -> A {
        println!("parse? {:?}", self.payload);
        A::parse(&self.payload[6..])
    }

    fn check_crc(&self) -> bool {
        let mut crc = crc16::State::<crc16::MCRF4XX>::new();
        crc.update(&self.payload[1..self.payload.len()-2]);
        crc.update(&[50]);
        let pktcrc = Cursor::new(&self.payload[self.payload.len()-2..]).read_u16::<LittleEndian>().unwrap();
        println!("match crc {:?} against {:?}", crc.get(), pktcrc);
        crc.get() == pktcrc
    }
}

struct Pong {
    socket: TcpStream,
    buf: Vec<u8>,
}

impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Pong>, token: mio::Token, events: mio::EventSet) {
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

                                    let packet = MavPacket::new(&self.buf[(start + i)..(start + i + 8 + len)]);

                                    println!("packet {:?}", packet.parse::<MAV_HEARTBEAT>());

                                    if !packet.check_crc() {
                                        start += i + 1;
                                        continue;
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

    println!("running pingpong socket");
    event_loop.run(&mut Pong { socket: socket, buf: vec![] });
}

pub fn main() {
    let file = File::open("solo.xml").unwrap();
    let file = BufReader::new(file);
    let profile = parse_profile(Box::new(file));
    run("127.0.0.1:5760".parse().unwrap());
}
