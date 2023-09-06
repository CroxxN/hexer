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
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let mut buf = BufReader::new(file);
    let mut buffer: [u8; 8] = [0; 8];
    println!();
    while let Ok(_) = buf.read_exact(&mut buffer) {
        position += 1;
        print!("\x1b[1;32m{:0>4}\x1b[0m  ", position);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".     "),

                0xff => print!("##  "),
                _ => print!("{:<02x} ", byte),
            }
        }
        print!("\t|\t");
        for byte in &buffer {
            if let Some(c) = char::from_u32(*byte as u32) {
                if c.is_alphanumeric() {
                    print!("{} ", c);
                } else {
                    print!(". ");
                }
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
