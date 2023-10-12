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

    let file = match File::open(&args) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let size = if let Ok(m) = file.metadata() {
        m.len()
    } else {
        println!("\x1b[1;31mError: Failed to read file size\x1b[0m");
        return;
    };
    let mut buf = BufReader::new(file);

    let mut buffer: [u8; 8] = [0; 8];
    println!();
    while let Ok(rs) = buf.read(&mut buffer) {
        // if EOF, return
        if rs == 0 {
            break;
        }
        print!("\x1b[1;32m{:0>6}\x1b[0m  ", position);
        position += 1;

        for byte in &buffer {
            match *byte {
                0x00 => print!("\x1b[1;31m00 \x1b[0m"),
                _ => print!("{:<02x} ", byte),
            }
        }
        print!("  |  ");

        for byte in &buffer {
            if *byte == 0 {
                print!(". ");
                continue;
            }
            if let Some(c) = char::from_u32(*byte as u32) {
                if !c.is_whitespace() {
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
    println!(
        "\n\x1b[1;32m{}\x1b[0m of \x1b[1;31m{}\x1b[0m bytes displayed in \x1b[1;31m{}\x1b[0m lines",
        args, size, position
    );
}
