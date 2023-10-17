use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

macro_rules! hexer_write{
    ($dst:expr, $($arg:tt)*) => {
        // use std::format_args;
        _ = $dst.write_fmt(::std::format_args!($($arg)*))
    };
}

fn main() {
    let args = match env::args().nth(1) {
        Some(args) => args,
        _ => {
            println!("Usage: hexer <filename>");
            return;
        }
    };

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
        println!("\x1b[1;31mError: Failed to read file size\x1b[0m");
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
        hexer_write!(&mut stdout_hdle, "\x1b[1;32m{:0>6}\x1b[0m  ", position);
        position += 1;

        for i in 0..rs {
            match buffer[i] {
                0x00 => hexer_write!(&mut stdout_hdle, "\x1b[1;31m00 \x1b[0m"),
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
        "\n\x1b[1;32m{}\x1b[0m of \x1b[1;31m{}\x1b[0m bytes displayed in \x1b[1;31m{}\x1b[0m lines\n",
        args,
        size,
        position
    );
}
