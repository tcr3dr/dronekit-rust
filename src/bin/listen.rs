extern crate dronekit;
extern crate eventual;

use dronekit::vehicle::{Vehicle};
use dronekit::{connect};
use eventual::Async;

fn vehicle_loop(mut vehicle: Vehicle) {

    vehicle.init();

    vehicle.parameters.complete().await().unwrap();
    vehicle.update(true);

    println!("params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    
    vehicle.parameters.set("FS_EKF_THRESH", 30.0).await().unwrap();
    println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    vehicle.update(true);
    println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));

    vehicle.parameters.set("FS_EKF_THRESH", 20.0).await().unwrap();
    println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    vehicle.update(true);
    println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));

    vehicle.spin();
}

fn main() {
    let vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));
    vehicle_loop(vehicle);
}
