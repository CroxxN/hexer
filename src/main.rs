use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args = match env::args().nth(1) {
        Some(args) => args,
        _ => {
            return eprintln!("Usage: hexer <filename>");
        }
    };
    let mut position = 0usize;

    let file = match File::open(args) {
        Ok(path) => path,
        Err(e) => return eprintln!("{e}"),
    };
    let mut buf = BufReader::new(file);
    let mut buffer: [u8; 16] = [0; 16];
    while let Ok(_) = buf.read_exact(&mut buffer) {
        position += 1;
        print!("{:0>4}  ", position);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".     "),

                0xff => print!("##  "),
                _ => print!("{:#02x} ", byte),
            }
        }
        println!();
    }
}
