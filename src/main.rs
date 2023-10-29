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

const HELP: &'static str = "Usage:
[options] <file>

Print bytes of a file in different formats and colors.

Options:
-h, --help          Print this help message
-v, --version       Print current version
-c, --no-canonical  Print ascii-equivalent(if available) side-by-side
-l, --linestyle     Print line in a specific format(hex, int, oct)
-b, --bytestyle     Print bytes in a specific format(hex, int, oct)
-C, --no-color      Don't display colors
-s, --column-size   Number of bytes displayed in one row
--no-stats          Don't display stats at the end of the dump
--byte2img          Interpret the bytes in image

Arguments: 
    <file1><file2>...

See hexer(1).";

const VERSION: &'static str = "v0.0.1";

// #[derive(Clone)]
pub struct HexOpts {
    column: u16,
    _pipe: bool,
    cannonical: bool,
    nstats: bool,
    file: String,
    byte2img: bool,
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
            column: 8,
            _pipe: false,
            cannonical: true,
            nstats: true,
            file: String::new(),
            byte2img: false,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = getopts::Options::new();
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
    opts.optflag(
        "",
        "byte2img",
        "plot the bytes in a 256x256 cartesian plane",
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

    let stdout_hndle = std::io::stdout();
    let mut hexopts = HexOpts::new();

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
        hexopts.column = if let Ok(d) = col.parse::<u16>() {
            d
        } else {
            println!("{BYELLOW}Warn: Invalid Column size: Using default = 8{END}");
            8
        }
    }
    if matches.opt_present("no-stats") {
        stats = false
    }
    hexopts.file = file;
    hexopts.nstats = stats;
    if matches.opt_present("c") {
        hexopts.cannonical = false;
    }
    if matches.opt_present("byte2img") {
        hexopts.byte2img = true;
    }
    let printer;
    if matches.opt_present("no-color") {
        printer = printer::new_ncolor(stdout_hndle.lock(), linestyle, bytestyle);
    } else {
        printer = printer::new_color(stdout_hndle.lock(), linestyle, bytestyle)
    }
    // let printer = if matches.opt_present("no-color") {
    //     coloropt::Hexwrite:
    // } else {
    //     coloropt::Colorstyle::Color(coloropt::Color::new(stdout_hndle, linestyle, bytestyle))
    // };
    hexdump(hexopts, printer);
}
