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

use connection::{VehicleConnection, parse_mavlink_string, DkHandlerMessage, DkHandlerRx};

#[derive(Clone)]
pub struct Parameters {
    values: HashMap<String, f32>,
    indexes: Vec<Option<String>>,
    connection: Rc<RefCell<VehicleConnection>>,
}

impl Parameters {
    pub fn new(connection: Rc<RefCell<VehicleConnection>>) -> Parameters {
        Parameters {
            values: HashMap::new(),
            indexes: vec![],
            connection: connection,
        }
    }

    fn resize(&mut self, len: u16) {
        if self.indexes.len() != len as usize {
            self.values = HashMap::new();
            self.indexes = repeat(None).take(len as usize).collect();
        }
    }

    fn assign(&mut self, index: u16, name: &str, value: f32) {
        self.values.insert(name.into(), value);
        if index != 65535 {
            self.indexes[index as usize] = Some(name.into());
        }
    }

    pub fn get(&self, name: &str) -> Option<&f32> {
        self.values.get(name)
    }

    pub fn set(&mut self, name: &str, value: f32) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();
        
        let mut txlock = Some(tx);
        let name_closure: String = name.clone().into();
        self.connection.borrow_mut().tx.send(DkHandlerMessage::TxWatcher(Box::new(move |msg| {
            if let DkMessage::PARAM_VALUE(data) = msg {
                if parse_mavlink_string(&data.param_id) == name_closure {
                    if data.param_value == value {
                        if let Some(tx) = txlock.take() {
                            tx.complete(());
                        }
                        return false;
                    }
                }
            }
            true
        })));

        self.connection.borrow_mut().send(DkMessage::PARAM_SET(PARAM_SET_DATA {
            param_value: value,
            target_system: 0,
            target_component: 0,
            param_id: name.chars().chain(repeat(0 as char)).take(16).map(|x| x as u8).collect(),
            param_type: 0,
        }));
        future
    }

    // pub fn sync() {
    //     self.vehicle_tx.send(DkHandlerMessage::TxSync);
    // }

    pub fn complete(&self) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        // Create the bit vector
        let mut filledlist: BitVec = BitVec::from_iter(self.indexes.iter().map(|x| x.is_some()));
        if self.indexes.len() > 0 && filledlist.all() {
            tx.complete(());
        } else {
            let mut conn = self.connection.borrow_mut();
            let buffer = conn.cork();

            let mut txlock = Some(tx);
            let mut watch = move |msg| {
                // println!("watchers! {:?}", msg);
                if let DkMessage::PARAM_VALUE(data) = msg {
                    if data.param_count as usize != filledlist.len() {
                        filledlist = BitVec::from_elem(data.param_count as usize, false);
                    }
                    filledlist.set(data.param_index as usize, true);
                    if filledlist.all() {
                        if let Some(tx) = txlock.take() {
                            tx.complete(());
                        }
                        return false;
                    }
                }
                true
            };

            if !buffer.into_iter().any(|x| !watch(x)) {
                conn.tx.send(DkHandlerMessage::TxWatcher(Box::new(watch)));
            }
            
            conn.uncork();
        }

        future
    }

    pub fn remaining(&self) -> usize {
        self.indexes.iter().filter(|x| x.is_some()).count()
    }

    pub fn available(&self) -> usize {
        self.indexes.iter().filter(|x| x.is_some()).count()
    }
}



#[derive(Clone, Debug)]
pub struct LocationGlobal {
    pub alt: i32,
    pub lat: i32,
    pub lon: i32,
}

// #[derive(Clone, Debug)]
pub struct Vehicle {
    pub parameters: Parameters,
    pub location_global: Option<LocationGlobal>,
    connection: Rc<RefCell<VehicleConnection>>,
}

impl Vehicle {
    pub fn new(conn: VehicleConnection) -> Vehicle {
        let connection = Rc::new(RefCell::new(conn));
        Vehicle {
            parameters: Parameters::new(connection.clone()),
            location_global: None,
            connection: connection,
        }
    }

    pub fn update(&mut self, wait: bool) {
        if wait {
            let val = {
                self.connection.borrow_mut().recv()
            };
            if let Ok(pkt) = val {
                if let DkHandlerRx::RxMessage(msg) = pkt {
                    self.on_message(msg);
                }
            } else {
                return;
            }
        }

        // Get remaining queued packets
        loop {
            let val = {
                self.connection.borrow_mut().try_recv()
            };
            if let Ok(pkt) = val {
                if let DkHandlerRx::RxMessage(msg) = pkt {
                    self.on_message(msg);
                }
            } else {
                break;
            }
        }
    }

    pub fn init (&mut self) {
        self.send_heartbeat();
        while !self.connection.borrow().started {
            self.update(true);
        }
    }

    fn send_heartbeat(&mut self) {
        self.connection.borrow_mut().send(DkMessage::HEARTBEAT(HEARTBEAT_DATA {
            custom_mode: 0,
            mavtype: 6,
            autopilot: 8,
            base_mode: 0,
            system_status: 0,
            mavlink_version: 0x3,
        }));
    }

    fn on_message(&mut self, pkt: DkMessage) {
        let pkt2 = pkt.clone();
        match pkt {
            DkMessage::HEARTBEAT(..) => {
                self.send_heartbeat();

                // if let Ok(Some(n)) = res {
                //     if n != outlen {
                //         println!("ERROR: only wrote {:?}", n);
                //     }
                // } else {
                //     println!("ERROR: didnt write anything");
                // }

                if !self.connection.borrow().started {
                    self.connection.borrow_mut().started = true;

                    self.connection.borrow_mut().send(DkMessage::PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA {
                        target_system: 0,
                        target_component: 0,
                    }));

                    self.connection.borrow_mut().send(DkMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA {
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
                self.parameters.resize(data.param_count);
                self.parameters.assign(data.param_index, &parse_mavlink_string(&data.param_id), data.param_value);
            },
            DkMessage::ATTITUDE(data) => {
                // println!("roll: {:?}\tpitch: {:?}\tyaw: {:?}", data.roll, data.pitch, data.yaw);
            },
            DkMessage::GLOBAL_POSITION_INT(data) => {
                self.location_global = Some(LocationGlobal {
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

    pub fn spin(&mut self) {
        loop {
            self.update(true)
        }
    }
}
