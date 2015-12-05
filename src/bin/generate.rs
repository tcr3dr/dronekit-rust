extern crate dronekit;
extern crate mio;
extern crate bytes;
extern crate crc16;
extern crate byteorder;

use dronekit::*;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

use std::fs::File;
use std::io::BufReader;
use mio::{TryRead, TryWrite};
use mio::tcp::TcpStream;
use mio::util::Slab;
use bytes::Buf;
use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;

pub fn main() {
    let file = File::open("common.xml").unwrap();
    let file = BufReader::new(file);
    let profile = parse_profile(Box::new(file));

    println!("use std::io::Cursor;");
    println!("use byteorder::{{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt}};");
    println!("");

    println!("pub trait Parsable {{");
    println!("    fn parse(payload: &[u8]) -> Self;");
    println!("    fn serialize(&self) -> Vec<u8>;");
    println!("}}");
    println!("");

    for item in &profile.messages {
        let mut f = item.fields.clone();
        f.sort_by(|a, b| a.mavtype.compare(&b.mavtype));

        println!("#[derive(Clone, Debug)]");
        println!("pub struct {}_DATA {{", item.name);
        for field in &f {
        	let fname = if field.name == "type" {
        		"mavtype".into()
        	} else {
        		field.name.clone()
        	};

            println!("    pub {}: {},", fname, field.mavtype.rust_type());
        }
        println!("}}");
        println!("");

        println!("impl Parsable for {}_DATA {{", item.name);
        println!("    fn parse(payload: &[u8]) -> {}_DATA {{", item.name);
        println!("        let mut cur = Cursor::new(payload);");
        println!("        {}_DATA {{", item.name);
        for field in &f {
        	let fname = if field.name == "type" {
        		"mavtype".into()
        	} else {
        		field.name.clone()
        	};
            match field.mavtype {
                MavType::Char | MavType::UInt8 | MavType::Int8 | MavType::UInt8MavlinkVersion => {
                    println!("            {}: cur.read_{}().unwrap(),", fname, field.mavtype.rust_type());
                },
                MavType::Array(ref t, size) => {
                    println!("            {}: vec![", fname);
                    for i in 0..size {
                        match *t.clone() {
                            MavType::Char | MavType::UInt8 | MavType::Int8 | MavType::UInt8MavlinkVersion => {
                                println!("                cur.read_{}().unwrap(),", t.rust_type());
                            },
                            MavType::Array(t, size) => {
                                panic!("error");
                            },
                            _ => {
                                println!("                cur.read_{}::<LittleEndian>().unwrap(),", t.rust_type());
                            },
                        }
                    }
                    println!("            ],");
                },
                _ => {
                    println!("            {}: cur.read_{}::<LittleEndian>().unwrap(),", fname, field.mavtype.rust_type());
                },
            }
        }
        println!("        }}");
        println!("    }}");
        println!("    fn serialize(&self) -> Vec<u8> {{");
        println!("        let mut wtr = vec![];");
        for field in &f {
            let fname = if field.name == "type" {
                "mavtype".into()
            } else {
                field.name.clone()
            };
            match field.mavtype {
                MavType::Char | MavType::UInt8 | MavType::Int8 | MavType::UInt8MavlinkVersion => {
                    println!("        wtr.write_{}(self.{}).unwrap();", field.mavtype.rust_type(), fname);
                },
                MavType::Array(ref t, size) => {
                    for i in 0..size {
                        match *t.clone() {
                            MavType::Char | MavType::UInt8 | MavType::Int8 | MavType::UInt8MavlinkVersion => {
                                println!("        wtr.write_{}(self.{}[{}]).unwrap();", t.rust_type(), fname, i);
                            },
                            MavType::Array(t, size) => {
                                panic!("error");
                            }
                            _ => {
                                println!("        wtr.write_{}::<LittleEndian>(self.{}[{}]).unwrap();", t.rust_type(), fname, i);
                            },
                        }
                    }
                },
                _ => {
                    println!("        wtr.write_{}::<LittleEndian>(self.{}).unwrap();", field.mavtype.rust_type(), fname);
                },
            }
        }
        println!("        wtr");
        println!("    }}");
        println!("}}");
        println!("");
    }

    println!("#[derive(Clone, Debug)]");
    println!("pub enum DkMessage {{");
    for item in &profile.messages {
        println!("  {}({}_DATA),", item.name, item.name);
    }
    println!("}}");
    println!("");

    println!("impl DkMessage {{");
    println!("    pub fn parse(id: u8, payload: &[u8]) -> Option<DkMessage> {{");
    println!("        match id {{");
    for item in &profile.messages {
            println!("            {} => Some(DkMessage::{}({}_DATA::parse(payload))),", item.id, item.name, item.name);
        }
    println!("            _ => None,");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");
}
