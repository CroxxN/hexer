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
[options] <file>

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

// #[derive(Clone)]
pub struct HexOpts {
    _column: i32,
    _pipe: bool,
    cannonical: bool,
    nstats: bool,
    file: String,
}

impl HexOpts {
    fn new() -> Self {
        Self {
            _column: 8,
            _pipe: false,
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
    let mut nocan = false;

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
