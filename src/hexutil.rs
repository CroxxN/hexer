use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

// use colors::{BGREEN, BRED, END, GREEN, RED};
use crate::colors::*;
use crate::common::hexer_write;

// TODO: Clean up
pub fn hexdump() {
    // let args = match env::args().nth(1) {
    //     Some(args) => args,
    //     _ => {
    //         println!("Usage: hexer <filename>");
    //         return;
    //     }
    // };

    let args = "TODO";

    let file = match File::open(&args) {
        Ok(path) => path,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let size = if let Ok(m) = file.metadata() {
        m.len()
    } else {
        println!("{BRED}Error: Failed to read file size{END}");
        return;
    };
    /* Create a stdout handle so that if piped to a pager, and the
    user quits the pager, hexer won't panic, but quitely exit
    */
    let mut stdout_hdle = std::io::stdout().lock();
    // for in-built paging functionality
    // WIP:
    let _pager_handle = match env::var("PAGER") {
        Ok(pg) => pg,
        Err(_) => "less".to_string(),
    };

    let mut position = 0usize;
    let mut buf = BufReader::new(file);

    // Number of column to display in one line
    let divisions = 16;
    let denominator = 0u8;

    let mut buffer = vec![denominator; divisions];
    println!();
    while let Ok(rs) = buf.read(&mut buffer) {
        // if EOF, return
        if rs == 0 {
            break;
        }
        hexer_write!(&mut stdout_hdle, "{GREEN}{:0>6}{END}  ", position);
        position += 1;

        for i in 0..rs {
            match buffer[i] {
                0x00 => hexer_write!(&mut stdout_hdle, "{RED}00 {END}"),
                _ => hexer_write!(&mut stdout_hdle, "{:<02x} ", buffer[i]),
            }
        }

        for _ in 0..(divisions - rs) {
            // Three little spaces. One for the separator, two for the placeholder.
            hexer_write!(&mut stdout_hdle, "   ");
        }
        hexer_write!(&mut stdout_hdle, "  |  ");

        for i in 0..rs {
            if buffer[i] == 0 {
                hexer_write!(&mut stdout_hdle, ". ");
                continue;
            }
            if let Some(c) = char::from_u32(buffer[i] as u32) {
                if !c.is_whitespace() {
                    hexer_write!(&mut stdout_hdle, "{} ", c);
                } else {
                    hexer_write!(&mut stdout_hdle, ". ");
                }
            } else {
                hexer_write!(&mut stdout_hdle, ". ");
            }
        }
        hexer_write!(&mut stdout_hdle, "\n");
    }
    hexer_write!(
        &mut stdout_hdle,
        "\n{BGREEN}{}{END} of {BGREEN}{}{END} bytes displayed in {BGREEN}{}{END} lines\n",
        args,
        size,
        position
    );
}
