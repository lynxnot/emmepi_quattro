//
use std::{
    fs::File,
    io::{BufReader, Read},
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
    println!("\n** emmepi_quattro **\n");

    const SAMPLE_FILE: &str = "./samples/sample-earth.mp4";
    //const SAMPLE_FILE: &str = "./samples/sample-animation.mp4";
    let f = File::open(SAMPLE_FILE).unwrap_or_else(|error| {
        println!(
            "Cound not open file: ErrorKind::{:?}, {}",
            error.kind(),
            error.to_string()
        );
        process::exit(1);
    });

    let mut reader = BufReader::new(f);
    let mut header_buf = [0; 8];
    
    while let Ok(()) = reader.read_exact(&mut header_buf) {

        let header = AtomHeader::new(&header_buf);

        println!(
            "{}: {}",
            String::from_utf8_lossy(&header.atom_type[..]),
            header.atom_size
        );

        if let Err(e) = reader.seek_relative((header.atom_size - 8).into()) {
            println!("error: {}", e);
            break;
        }
    }
    
}
