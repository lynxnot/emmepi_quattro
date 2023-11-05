//
use std::{
    fs::File,
    io::{BufReader, Read},
    process,
};

use emmepi_quattro::container::mp4::AtomHeader;

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

        println!("{:?}: {}", header.kind, header.size);

        if let Err(e) = reader.seek_relative((header.size - 8).into()) {
            println!("error: {}", e);
            break;
        }
    }
}
