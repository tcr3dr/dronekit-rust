extern crate dronekit;
extern crate eventual;

use dronekit::vehicle::{Vehicle, VehicleMode, LocationLocal};
use dronekit::connect;
use eventual::{Async, join};

fn main() {
    let mut vehicle = Vehicle::new(connect("127.0.0.1:5760".parse().unwrap()));
    vehicle.init();

    vehicle.parameters.complete().await().unwrap();

    join((vehicle.parameters.set("FS_GCS_ENABLE", 0.0),
          vehicle.parameters.set("FS_EKF_THRESH", 100.0)))
        .await()
        .unwrap();
    vehicle.update(true);

    println!("sleeping");
    ::std::thread::sleep_ms(3000);

    println!("Setting mode...");
    vehicle.set_mode(VehicleMode::GUIDED).await().unwrap();

    println!("Arming...");
    vehicle.arm().await().unwrap();

    println!("Takeoff...");
    vehicle.takeoff(30.0).await().unwrap();

    println!("Elevating...");
    vehicle.wait_alt(30.0).await().unwrap();

    vehicle.set_airspeed().await().unwrap();

    vehicle.update(true);
    println!("Current location: {:?}", vehicle.location_local);

    println!("Goto (-50, -50, -30) in NED coordinates...");
    vehicle.goto(LocationLocal {
        x: -50.0,
        y: -50.0,
        z: -30.0,
    }).await().unwrap();

    vehicle.update(true);
    println!("Current location: {:?}", vehicle.location_local);

    println!("done!");
    // vehicle.spin();
}
