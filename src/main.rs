// TODO: Remove this item
#![allow(unused_variables, dead_code)]

// TODO: Import BGREEN
mod bytestyle;
mod coloropt;
mod colors;
mod common;
mod hexutil;
mod linestyle;

use colors::*;
// use hexutil;
use getopts;
use std::env;

const HELP: &'static str = "Usage:
 hexer [options] <file>

Print bytes of a file in different formats and colors.

Options:
-h, --help         Print this help message
-v, --version      Print current version
-C, --canonical    Print ascii-equivalent(if available) side-by-side

Arguments: 
    <file1><file2>...

See hexer(1).";

const VERSION: &'static str = "v0.0.1";

pub struct HexOpts {
    column: i32,
    pipe: bool,
    cannonical: bool,
    line: linestyle::Linestyle,
    colors: bool,
    stats: bool,
}

impl HexOpts {
    fn new() -> Self {
        Self {
            column: 8,
            pipe: false,
            cannonical: true,
            line: linestyle::Hex,
            colors: true,
            stats: true,
        }
    }
    fn set_column(&mut self, column: i32) {
        self.column = column;
    }
    fn set_pipe(&mut self) {
        self.pipe = true;
    }
    fn set_cannonical(&mut self) {
        self.cannonical = false;
    }
    fn set_line(&mut self) {
        self.line = Linestyle::Int;
    }
    fn set_colors(&mut self) {
        self.colors = false;
    }
    fn set_stats(&mut self) {
        self.stats = false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help message");
    opts.optflag("v", "version", "Print hexer version.");
    opts.optflag("c", "no-canonical", "Disables interpreted ascii printing");
    opts.optflag("", "color", "Enable color mode (Default)");
    opts.optopt(
        "l",
        "linestyle",
        "formatter to use while displaying line",
        "hexer --linestyle=octal <file>",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{BRED}Error{END}: {}", e);
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
    let hexopts = HexOpts::new();
    let file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("{BRED}Error{END}: Required argument <file>\n");
        println!("{HELP}");
        std::process::exit(1);
    };
}
