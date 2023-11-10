use image;
// image is already using rayon so just adding it here to use par_iter.
// Not REALLY neccessary, but it's almost free so why not?
use rayon::prelude::*;
use std::env;
use std::fs::{read_link, File};
use std::io::{BufReader, Read, Write};
use std::os::unix::prelude::OsStrExt;

use crate::colors::*;
use crate::common::hexer_write;
use crate::printer::Hexwrite;
use crate::HexOpts;
use crate::Stat;

pub fn read_symlink(path: &str, mut printer: Box<dyn Hexwrite>) {
    if let Ok(sp) = read_link(path) {
        println!(); // okay to use println because even if used with pipe, it immediately exists without allowing the pipe to be broken
        printer.write_line(0);
        printer.write_bytes(sp.as_os_str().as_bytes(), sp.as_os_str().len(), 1);
        print!("  {BCYAN}|{END}  ");
        sp.as_os_str()
            .as_bytes()
            .into_iter()
            .for_each(|b| print!("{}", *b as char));
        println!();
        printer.write_stats(Stat {
            args: &format!("Symlink {path}"),
            size: path.len() as u64,
            position: 1,
        });
    }
}

// TODO: Clean up + Seperate functions
pub fn hexdump(opts: &HexOpts, printer: &mut dyn Hexwrite) {
    let file = match File::open(&opts.file) {
        Ok(path) => path,
        Err(e) => {
            println!("\n{BRED}Error:{END} {e}");
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
    // let divisions = 16;
    let denominator = opts.gap as usize;
    let divisions = opts.column as usize;

    let mut buffer = vec![0u8; divisions];
    println!();
    while let Ok(rs) = buf.read(&mut buffer) {
        // if EOF, return
        if rs == 0 {
            break;
        }
        printer.write_line(position);
        position += 1;

        // TODO: let the byte implementation handel the spacing.
        // I.E: send 8-16 bytes each time to the printbyte implementation
        // for i in 0..rs {
        //     // opts.byte.print(&mut stdout_hdle, &buffer[i]);
        //     printer.write_bytes(&buffer[i]);
        // }
        // buffer.iter().for_each(|v| {
        //     printer.write_bytes(v);
        // });
        printer.write_bytes(&buffer, rs, denominator);

        if opts.cannonical {
            for _ in 0..(divisions - rs) {
                // Three little spaces. One for the separator, two for the placeholder.
                hexer_write!(&mut stdout_hdle, "   ");
            }
            hexer_write!(&mut stdout_hdle, "  {BCYAN}|{END}  ");

            for i in 0..rs {
                if buffer[i] < 32 {
                    hexer_write!(&mut stdout_hdle, ". ");
                    continue;
                }
                if let Some(c) = char::from_u32(buffer[i] as u32) {
                    if !c.is_whitespace()
                    /*&& c.is_ascii()*/
                    {
                        hexer_write!(&mut stdout_hdle, "{} ", c);
                    } else {
                        hexer_write!(&mut stdout_hdle, ". ");
                    }
                } else {
                    hexer_write!(&mut stdout_hdle, ". ");
                }
            }
        }
        hexer_write!(&mut stdout_hdle, "\n");
    }
    if opts.nstats {
        let stats = Stat::new(&opts.file, size, position);
        printer.write_stats(stats);
    }
    if opts.byte2img {
        byte2img(&opts.file, &opts.imgpath);
    }
}

pub fn byte2img(file: &str, img_save_path: &str) {
    let mut const_array = [[0usize; 256]; 256];
    let mut pixls = image::ImageBuffer::new(256, 256);

    // have to use a underscore to supress the `never read` warning
    let mut _bytes = Vec::new();

    _bytes = match std::fs::read(file) {
        Ok(f) => f,
        Err(e) => {
            println!("{BRED}Error: Failed to open file:{END} {}", e);
            return;
        }
    };

    let maxx;
    for i in 0.._bytes.len() - 1 {
        let ft = _bytes[i] as usize;
        let sc = _bytes[i + 1] as usize;
        const_array[ft][sc] = const_array[ft][sc] + 1;
    }
    if let Some(m) = const_array.par_iter().flatten().max() {
        // x > y => ln(x) > ln(y)
        if *m < 1 {
            maxx = 1f32;
        } else {
            maxx = (*m as f32).ln();
        }
    } else {
        return;
    }
    for (i, j, pix) in pixls.enumerate_pixels_mut() {
        let res = (const_array[i as usize][j as usize] as f32).ln() / maxx;
        let res = (res * 255.) as u8;
        *pix = image::Rgba([0xFF, 0xFF, 0xFF, res]);
    }
    if let Err(e) = pixls.save_with_format(img_save_path, image::ImageFormat::Png) {
        println!("\n{BRED}Failed to save image{END}: {}", e);
    } else {
        println!("\n{BGREEN}Saved image to {}{END}", img_save_path);
    }
}
