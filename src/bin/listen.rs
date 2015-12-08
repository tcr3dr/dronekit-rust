extern crate dronekit;

use std::thread;
use dronekit::vehicle::{Vehicle, connect};

pub fn main() {
    let mut vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));

    loop {
        thread::sleep_ms((1000.0 / 10.0) as u32);
        vehicle.update();
        println!("tick. location: {:?}", vehicle.location_global);
    }
}
