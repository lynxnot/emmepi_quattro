//
use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    process,
};

struct AtomHeader {
    atom_size: u32,
    atom_type: [u8; 4],
}

impl AtomHeader {
    fn new(buf: &[u8]) -> AtomHeader {
        AtomHeader {
            atom_size: u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
            atom_type: [buf[4], buf[5], buf[6], buf[7]],
        }
    }
}

fn main() {
    println!("Hello, world!");

    let f = File::open("./samples/sample-earth.mp4").unwrap_or_else(|error| {
        println!(
            "Cound not open file: ErrorKind::{:?}, {}",
            error.kind(),
            error.to_string()
        );
        process::exit(1);
    });

    let mut reader = BufReader::new(f);
    let mut buf = [0; 8];
    reader
        .read_exact(&mut buf)
        .expect("Could not read atom header");

    let header = AtomHeader::new(&buf);

    println!("size: {}", header.atom_size);
    println!(
        "atom_type: {}",
        String::from_utf8_lossy(&header.atom_type[..])
    );
    println!("stream position: {:?}", reader.stream_position().unwrap());
}
