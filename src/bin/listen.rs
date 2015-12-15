extern crate dronekit;
extern crate eventual;

use std::thread;
use dronekit::vehicle::{Vehicle, connect};
use eventual::Async;

fn tick(vehicle: &mut Vehicle) {
    println!("tick. location: {:?}", vehicle.location_global);
}

fn vehicle_loop<F>(mut vehicle: Vehicle, fps: i32, func: F)
    where F: Fn(&mut Vehicle) {
    // loop {
    //     thread::sleep_ms((1000.0 / (fps as f32)) as u32);
    //     vehicle.update(false);
    //     func(&mut vehicle);
    // }

    vehicle.init();

    vehicle.parameters.complete().await();
    vehicle.update(true);

    println!("params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    
    vehicle.parameters.set("FS_EKF_THRESH", 30.0).await();
    println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    vehicle.update(true);
    println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));

    vehicle.parameters.set("FS_EKF_THRESH", 20.0).await();
    println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    vehicle.update(true);
    println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
}

fn main() {
    let mut vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));
    vehicle_loop(vehicle, 10, tick);
}
