extern crate xml;
extern crate byteorder;
extern crate time;
extern crate mio;
extern crate bytes;
extern crate crc16;
extern crate eventual;
extern crate bit_vec;

pub mod mavlink;
pub mod vehicle;
pub mod parser;
pub mod connection;

use connection::{VehicleConnection, DkHandler};
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, TryRecvError};
use mio::tcp::TcpStream;
use std::collections::{HashMap, VecDeque};
use std::thread;

pub fn connect(address: SocketAddr) -> VehicleConnection {
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

    // let sender = event_loop.channel();
    // // Send the notification from another thread

    let (tx, rx) = channel();
    let vehicle_tx = event_loop.channel();

    thread::spawn(move || {
        println!("running pingpong socket");
        let mut handler = DkHandler {
            socket: socket,
            buf: vec![],
            vehicle_tx: tx,
            watchers: vec![],
        };
        handler.register(&mut event_loop);
        event_loop.run(&mut handler);
    });

    return VehicleConnection {
        tx: vehicle_tx,
        rx: rx,
        msg_id: 0,
        started: false,
        buffer: VecDeque::new(),
    };
}
