// TODO: Import BGREEN
mod bytestyle;
mod colors;
mod common;
mod hexutil;
mod linestyle;
mod printer;

use colors::*;
// use hexutil;
use getopts;
use hexutil::hexdump;
use std::env;

const HELP: &'static str = "Usage: hexer [options] <file>

Print bytes of a file in different formats and colors.

Options:
-h, --help          Print this help message
-v, --version       Print current version
-c, --no-canonical  Print ascii-equivalent(if available) side-by-side
-l, --linestyle     Print line in a specific format(hex, int, oct, null)
-b, --bytestyle     Print bytes in a specific format(hex, int, oct)
-C, --no-color      Don't display colors
-s, --column-size   Number of bytes displayed in one row
-g, --gap-size      Insert a gap between every <n> bytes    
--no-stats          Don't display stats at the end of the dump
--byte2img          Plot the bytes to image
--byte2img-only     Just plot the bytes to image and nothing more

Arguments: 
    <file1> <file2>...

See hexer(1).";

const VERSION: &'static str = "v0.0.1";

pub struct HexOpts {
    column: u16,
    gap: u16,
    _pipe: bool,
    cannonical: bool,
    nstats: bool,
    file: String,
    byte2img: bool,
    imgpath: String,
}

pub struct Stat<'a> {
    args: &'a str,
    size: u64,
    position: usize,
}

impl<'a> Stat<'a> {
    pub fn new(args: &'a str, size: u64, position: usize) -> Self {
        Self {
            args,
            size,
            position,
        }
    }
}

impl HexOpts {
    fn new() -> Self {
        Self {
            column: 16,
            gap: 1,
            _pipe: false,
            cannonical: true,
            nstats: true,
            file: String::new(),
            byte2img: false,
            // Default imgpath
            imgpath: "output.png".to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = getopts::Options::new();

    // CLI options description
    opts.optflag("h", "help", "print this help message");
    opts.optflag("v", "version", "Print hexer version.");
    opts.optflag("c", "no-canonical", "Disables interpreted ascii printing");
    opts.optflag("S", "no-stats", "Don't show stats");
    opts.optflag("C", "no-color", "Disable color");
    opts.optopt(
        "l",
        "linestyle",
        "formatter to use while displaying line",
        "hexer --linestyle=octal <file>",
    );
    opts.optopt(
        "b",
        "bytestyle",
        "formatter to use while displaying bytes",
        "hexer --bytestyle=hex <file>",
    );
    opts.optopt(
        "s",
        "column-size",
        "Number of bytes displayed in one row",
        "hexer --column-size 16 <file>",
    );
    opts.optopt(
        "g",
        "gap-size",
        "Insert gap between every <n> bytes",
        "hexer -g2 [options] <file>",
    );

    opts.optflagopt(
        "",
        "byte2img",
        "plot the bytes in a 256x256 cartesian plane",
        "hexer --byte2img=<path_to_image> <file>",
    );
    opts.optflagopt(
        "",
        "byte2img-only",
        "only plot the bytes in a 256x256 cartesian plane",
        "hexer --byte2img-only=<path_to_image> <file>",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("\n{BRED}Error{END}: {}", e);
            println!("{} {HELP}", &program_name);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        println!("{} {HELP}", &program_name);
        std::process::exit(0);
    }
    if matches.opt_present("v") {
        println!("{BGREEN}{VERSION}{END}");
        std::process::exit(0);
    }

    let file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("\n{BRED}Error{END}: Required argument <file>\n");
        println!("{} {HELP}", &program_name);
        std::process::exit(1);
    };

    let linestyle;
    let bytestyle;
    let mut stats = true;

    if let Some(path) = matches.opt_str("byte2img-only") {
        hexutil::byte2img(&file, &path);
        return;
    }
    if matches.opt_present("byte2img-only") {
        hexutil::byte2img(&file, "output.png");
        return;
    }

    let stdout_hndle = std::io::stdout();
    let mut hexopts = HexOpts::new();
    hexopts.file = file;

    if let Some(line) = matches.opt_str("l") {
        linestyle = linestyle::from_str(&line);
    } else {
        linestyle = Box::new(linestyle::Hex)
    }

    if let Some(byte) = matches.opt_str("b") {
        bytestyle = bytestyle::from_str(&byte);
    } else {
        bytestyle = Box::new(bytestyle::BHex)
    }

    if let Some(col) = matches.opt_str("s") {
        if let Ok(d) = col.parse::<u16>() {
            hexopts.column = d;
        } else {
            println!("\n{BYELLOW}Warn: Invalid Column size: Using default = 16{END}");
        }
    }
    if let Some(g) = matches.opt_str("g") {
        if let Ok(g) = g.parse::<u16>() {
            if g > 0 {
                hexopts.gap = g;
            }
        } else {
            println!("\n{BYELLOW}Warn: Invalid gap size: Using default = 1{END}");
        }
    }

    if matches.opt_present("no-stats") {
        stats = false
    }

    hexopts.nstats = stats;

    if matches.opt_present("c") {
        hexopts.cannonical = false;
    }

    if matches.opt_present("byte2img") {
        hexopts.byte2img = true;
    }
    if let Some(path) = matches.opt_str("byte2img") {
        hexopts.imgpath = path;
    }

    let printer;

    if matches.opt_present("no-color") {
        printer = printer::new_ncolor(stdout_hndle.lock(), linestyle, bytestyle);
    } else {
        printer = printer::new_color(stdout_hndle.lock(), linestyle, bytestyle)
    }

    hexdump(hexopts, printer);
}
