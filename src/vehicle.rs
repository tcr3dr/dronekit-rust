extern crate mio;
extern crate bit_vec;

use mavlink::*;

use std::collections::{HashMap};
use std::iter::repeat;
use std::cell::RefCell;
use std::rc::Rc;
use eventual::{Future};
use bit_vec::BitVec;

use connection::{VehicleConnection, parse_mavlink_string};

pub enum VehicleMode {
    LOITER,
    GUIDED,
}

#[derive(Clone)]
pub struct Parameters {
    values: HashMap<String, f32>,
    indexes: Vec<Option<String>>,
    missing: BitVec,
    connection: Rc<RefCell<VehicleConnection>>,
}

impl Parameters {
    pub fn new(connection: Rc<RefCell<VehicleConnection>>) -> Parameters {
        Parameters {
            values: HashMap::new(),
            indexes: vec![],
            missing: BitVec::new(),
            connection: connection,
        }
    }

    fn resize(&mut self, len: u16) {
        if self.indexes.len() != len as usize {
            self.values = HashMap::new();
            self.indexes = repeat(None).take(len as usize).collect();
            self.missing = BitVec::from_elem(len as usize, false);
        }
    }

    fn assign(&mut self, index: u16, name: &str, value: f32) {
        self.values.insert(name.into(), value);
        if index != 65535 {
            self.indexes[index as usize] = Some(name.into());
            self.missing.set(index as usize, true);
        }
    }

    pub fn get(&self, name: &str) -> Option<&f32> {
        self.values.get(name)
    }

    pub fn set(&mut self, name: &str, value: f32) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();
        
        let name_closure: String = name.into();
        conn.complete(tx, Box::new(move |msg| {
            if let DkMessage::PARAM_VALUE(data) = msg {
                parse_mavlink_string(&data.param_id) == name_closure && data.param_value == value
            } else {
                false
            }
        }));

        conn.send(DkMessage::PARAM_SET(PARAM_SET_DATA {
            param_value: value,
            target_system: 0,
            target_component: 0,
            param_id: name.chars().chain(repeat(0 as char)).take(16).map(|x| x as u8).collect(),
            param_type: 0,
        }));

        future
    }

    pub fn complete(&self) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        // Create the bit vector
        if self.missing.len() > 0 && self.missing.all() {
            tx.complete(());
        } else {
            let mut conn = self.connection.borrow_mut();
            let mut missing = self.missing.clone();
            conn.complete(tx, Box::new(move |msg| {
                if let DkMessage::PARAM_VALUE(data) = msg {
                    // Resize the array if a new parameter is sent.
                    if data.param_count as usize != missing.len() {
                        missing = BitVec::from_elem(data.param_count as usize, false);
                    }

                    // If we have a complete set, indicate as such.
                    missing.set(data.param_index as usize, true);
                    missing.all()
                } else {
                    false
                }
            }));
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

trait Distance<T> {
    fn distance_to(&self, &T) -> f32;
}


#[derive(Clone, Debug)]
pub struct LocationGlobalRelative {
    pub alt: i32,
    pub lat: i32,
    pub lon: i32,
}

#[derive(Clone, Debug)]
pub struct LocationGlobal {
    pub alt: i32,
    pub lat: i32,
    pub lon: i32,
}

#[derive(Clone, Debug)]
pub struct LocationLocal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl LocationLocal {
    fn translate(&self, x: f32, y: f32, z: f32) -> LocationLocal {
        LocationLocal {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

impl Distance<LocationLocal> for LocationLocal {
    fn distance_to(&self, to: &LocationLocal) -> f32 {
        ((self.x - to.x).powi(2) + (self.y - to.y).powi(2) + (self.z - to.z).powi(2)).sqrt()
    }
}

// #[derive(Clone, Debug)]
pub struct Vehicle {
    pub parameters: Parameters,
    pub location_global: Option<LocationGlobal>,
    pub location_global_relative: Option<LocationGlobalRelative>,
    pub location_local: Option<LocationLocal>,
    connection: Rc<RefCell<VehicleConnection>>,
    master_heartbeat: bool,
}

impl Vehicle {
    pub fn new(conn: VehicleConnection) -> Vehicle {
        let connection = Rc::new(RefCell::new(conn));
        Vehicle {
            parameters: Parameters::new(connection.clone()),
            location_global: None,
            location_global_relative: None,
            location_local: None,
            connection: connection,
            master_heartbeat: false,
        }
    }

    pub fn update(&mut self, wait: bool) {
        if wait {
            let val = {
                self.connection.borrow_mut().recv()
            };
            if let Ok(msg) = val {
                self.on_message(msg);
            } else {
                return;
            }
        }

        // Get remaining queued packets
        loop {
            let val = {
                self.connection.borrow_mut().try_recv()
            };
            if let Ok(msg) = val {
                self.on_message(msg);
            } else {
                break;
            }
        }
    }

    pub fn init (&mut self) {
        self.send_heartbeat();
        while !self.master_heartbeat {
            self.update(true);
        }

        self.request_parameters();
        self.request_stream();
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

    fn request_parameters(&mut self) {
        self.connection.borrow_mut().send(DkMessage::PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA {
            target_system: 0,
            target_component: 0,
        }));
    }

    fn request_stream(&mut self) {
        self.connection.borrow_mut().send(DkMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA {
            target_system: 0,
            target_component: 0,
            req_stream_id: 0,
            req_message_rate: 10,
            start_stop: 1,
        }));
    }

    fn on_message(&mut self, pkt: DkMessage) {
        match pkt {
            DkMessage::HEARTBEAT(..) => {
                self.send_heartbeat();
                // self.connection.borrow_mut().send(DkMessage::MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA {
                //     target_system: 0,
                //     target_component: 0,
                // }));
                self.master_heartbeat = true;
            }
            DkMessage::STATUSTEXT(data) => {
                let text = parse_mavlink_string(&data.text);
                println!("<<< [{:?}] {:?}", data.severity, text);
            }
            DkMessage::PARAM_VALUE(data) => {
                self.parameters.resize(data.param_count);
                self.parameters.assign(data.param_index, &parse_mavlink_string(&data.param_id), data.param_value);
            }
            DkMessage::ATTITUDE(..) => {
                // println!("roll: {:?}\tpitch: {:?}\tyaw: {:?}", data.roll, data.pitch, data.yaw);
            }
            DkMessage::GLOBAL_POSITION_INT(data) => {
                self.location_global = Some(LocationGlobal {
                    lat: data.lat,
                    lon: data.lon,
                    alt: data.alt,
                });
                self.location_global_relative = Some(LocationGlobalRelative {
                    lat: data.lat,
                    lon: data.lon,
                    alt: data.relative_alt,
                });
            }
            DkMessage::LOCAL_POSITION_NED(data) => {
                self.location_local = Some(LocationLocal {
                    x: data.x,
                    y: data.y,
                    z: data.z,
                });
            }
            _ => {
                // println!("dunno: {:?}", pkt);
            }
        }
    }

    pub fn set_mode(&mut self, mode: VehicleMode) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();
        
        conn.complete(tx, Box::new(move |msg| {
            if let DkMessage::HEARTBEAT(data) = msg {
                (data.base_mode & 1 != 0) && (data.custom_mode == 4)
            } else {
                false
            }
        }));

        conn.send(DkMessage::SET_MODE(SET_MODE_DATA {
            target_system: 0,
            base_mode: 1,
            custom_mode: 4,
        }));

        future
    }

    pub fn arm(&mut self) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();

        let mut ack = false;
        let mut armed = false;
        
        conn.complete(tx, Box::new(move |msg| {
            if let DkMessage::COMMAND_ACK(data) = msg {
                ack = ack || data.command == 400;
            } else if let DkMessage::HEARTBEAT(data) = msg {
                armed = armed || ((data.base_mode & 128) != 0);
            }
            ack && armed
        }));

        conn.send(DkMessage::COMMAND_LONG(COMMAND_LONG_DATA {
            target_system: 0,
            target_component: 0,
            command: 400,
            confirmation: 0,
            param1: 1.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
        }));

        future
    }

    pub fn takeoff(&mut self, target_alt: f32) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();

        let mut ack = false;
        let mut is_active = false;
        
        conn.complete(tx, Box::new(move |msg| {
            match msg {
                DkMessage::HEARTBEAT(data) => {
                    is_active = is_active || data.system_status == 4;
                }
                DkMessage::COMMAND_ACK(data) => {
                    ack = ack || (data.command == 22);
                }
                // DkMessage::LOCAL_POSITION_NED(data) => {
                //     alt_achieved = alt_achieved || ((target_alt + data.z).abs() < 2.0);
                // }
                _ => ()
            }
            ack && is_active
        }));

        conn.send(DkMessage::COMMAND_LONG(COMMAND_LONG_DATA {
            target_system: 0,
            target_component: 0,
            command: 22,
            confirmation: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: target_alt,
        }));

        future
    }

    pub fn retry(&mut self) {
        let mut conn = self.connection.borrow_mut();
        conn.send(DkMessage::SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA {
            time_boot_ms: 10,
            target_system: 0,
            target_component: 0,
            coordinate_frame: 1,
            type_mask: 0b0000_111_111_000_000,
            x: -20.0,
            y: -20.0,
            z: -30.0,
            vx: 0.1,
            vy: 0.1,
            vz: 0.1,
            afx: 0.0,
            afy: 0.0,
            afz: 0.0,
            yaw: 0.0,
            yaw_rate: 0.0,
        }));
    }

    pub fn goto(&mut self) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();

        let mut ack = false;
        let mut is_active = false;
        
        conn.complete(tx, Box::new(move |msg| {
            match msg {
                DkMessage::LOCAL_POSITION_NED(data) => {
                    println!("local pos {:?}", data);
                    // alt_achieved = alt_achieved || ((target_alt + data.z).abs() < 2.0);
                }
                _ => ()
            }
            ack && is_active
        }));

        conn.send(DkMessage::SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA {
            time_boot_ms: 0,
            target_system: 0,
            target_component: 0,
            coordinate_frame: 1,
            type_mask: 0b0000_111_111_000_000,
            x: -20.0,
            y: -20.0,
            z: -30.0,
            vx: 0.1,
            vy: 0.1,
            vz: 0.1,
            afx: 0.0,
            afy: 0.0,
            afz: 0.0,
            yaw: 0.0,
            yaw_rate: 0.0,
        }));

        future
    }

    pub fn wait_alt(&mut self, target_alt: f32) -> Future<(), ()> {
        let (tx, future) = Future::<(), ()>::pair();

        let mut conn = self.connection.borrow_mut();

        conn.complete(tx, Box::new(move |msg| {
            if let DkMessage::LOCAL_POSITION_NED(data) = msg {
                ((target_alt + data.z).abs() < 2.0)
            } else {
                false
            }
        }));

        future
    }

    pub fn spin(&mut self) {
        loop {
            self.update(true)
        }
    }
}
