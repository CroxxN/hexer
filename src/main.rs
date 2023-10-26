// TODO: Remove this item
#![allow(unused_variables, dead_code)]

// TODO: Import BGREEN
mod bytestyle;
mod coloropt;
mod colors;
mod common;
mod hexutil;
mod linestyle;

use coloropt::Hexwrite;
use colors::*;
// use hexutil;
use getopts;
use hexutil::hexdump;
use std::env;

const HELP: &'static str = "Usage:
 hexer [options] <file>

Print bytes of a file in different formats and colors.

Options:
-h, --help          Print this help message
-v, --version       Print current version
-c, --no-canonical  Print ascii-equivalent(if available) side-by-side
-l, --linestyle     Print line in a specific format(hex, int, oct)
-b, --bytestyle     Print bytes in a specific format(hex, int, oct)
-C, --no-color      Don't display colors
--no-stats          Don't display stats at the end of the dump

Arguments: 
    <file1><file2>...

See hexer(1).";

const VERSION: &'static str = "v0.0.1";

#[derive(Clone)]
pub struct HexOpts {
    column: i32,
    pipe: bool,
    cannonical: bool,
    nstats: bool,
    file: String,
}

impl HexOpts {
    fn new() -> Self {
        Self {
            column: 8,
            pipe: false,
            cannonical: true,
            nstats: true,
            file: String::new(),
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
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("\n{BRED}Error{END}: {}", e);
            println!("{HELP}");
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        println!("{HELP}");
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
        println!("{HELP}");
        std::process::exit(1);
    };

    let mut linestyle = linestyle::Linestyle::Hex;
    let mut bytestyle = bytestyle::Bytestyle::BHex;
    let mut stats = true;
    let mut nocan = false;

    if let Some(line) = matches.opt_str("l") {
        linestyle = line.into();
    }
    if let Some(byte) = matches.opt_str("b") {
        bytestyle = byte.into();
    }
    if matches.opt_present("no-stats") {
        stats = false
    }
    if matches.opt_present("c") {
        nocan = true;
    }
    let stdout_hndle = std::io::stdout().lock();
    let mut hexopts = HexOpts::new();
    hexopts.file = file;
    hexopts.nstats = stats;
    hexopts.cannonical = !nocan;
    let printer = if matches.opt_present("no-color") {
        coloropt::Colorstyle::NColor(coloropt::NColor::new(stdout_hndle, linestyle, bytestyle))
    } else {
        coloropt::Colorstyle::Color(coloropt::Color::new(stdout_hndle, linestyle, bytestyle))
    };
    hexdump(hexopts, printer);
}
