extern crate dronekit;

use std::thread;
use dronekit::vehicle::{Vehicle, connect};

fn tick(vehicle: &mut Vehicle) {
    println!("tick. location: {:?}", vehicle.location_global);
}

fn vehicle_loop<F>(mut vehicle: Vehicle, fps: i32, func: F)
    where F: Fn(&mut Vehicle) {
    loop {
        thread::sleep_ms((1000.0 / (fps as f32)) as u32);
        vehicle.update();
        func(&mut vehicle);
    }
}

fn main() {
    let mut vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));
    vehicle_loop(vehicle, 10, tick);
}
