extern crate dronekit;
extern crate eventual;

use dronekit::vehicle::{Vehicle, VehicleMode};
use dronekit::{connect};
use eventual::{Async, join};

// fn vehicle_loop(mut vehicle: Vehicle) {

//     vehicle.init();

//     vehicle.parameters.complete().await().unwrap();
//     vehicle.update(true);

//     println!("params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
    
//     join((
//         vehicle.parameters.set("FS_GCS_ENABLE", 1.0),
//         vehicle.parameters.set("FS_EKF_THRESH", 20.0),
//     )).await().unwrap();
//     println!("-----");
//     println!("old params {:?}", vehicle.parameters.get("FS_GCS_ENABLE"));
//     println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
//     vehicle.update(true);
//     println!("new params {:?}", vehicle.parameters.get("FS_GCS_ENABLE"));
//     println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));

//     join((
//         vehicle.parameters.set("FS_GCS_ENABLE", 0.0),
//         vehicle.parameters.set("FS_EKF_THRESH", 30.0),
//     )).await().unwrap();
//     println!("-----");
//     println!("old params {:?}", vehicle.parameters.get("FS_GCS_ENABLE"));
//     println!("old params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));
//     vehicle.update(true);
//     println!("new params {:?}", vehicle.parameters.get("FS_GCS_ENABLE"));
//     println!("new params {:?}", vehicle.parameters.get("FS_EKF_THRESH"));

//     vehicle.spin();
// }

fn vehicle_loop(mut vehicle: Vehicle) {
    vehicle.init();

    vehicle.parameters.complete().await().unwrap();

    join((
        vehicle.parameters.set("FS_GCS_ENABLE", 0.0),
        vehicle.parameters.set("FS_EKF_THRESH", 100.0),
    )).await().unwrap();
    vehicle.update(true);

    println!("Setting mode...");
    vehicle.set_mode(VehicleMode::GUIDED).await().unwrap();

    println!("Arming...");
    vehicle.arm().await().unwrap();

    println!("spinning.");

    vehicle.spin();
}

fn main() {
    let vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));
    vehicle_loop(vehicle);
}
