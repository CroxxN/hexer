// TODO: Remove this item
#![allow(unused_variables, dead_code)]

use getopts;
use std::env;

pub struct Opts {
    column: i32,
    pipe: bool,
    cannonical: bool,
    line: Linestyle,
    colors: bool,
    stats: bool,
}

enum Linestyle {
    Hex,
    Int,
}

pub fn cli() -> String {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Hexer help message");
    opts.optflag("v", "version", "hexer version");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        println!("hex anything");
        std::process::exit(1);
    }
    if matches.opt_present("v") {
        println!("Version: 0.0.1");
        std::process::exit(1);
    }
    if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("Not enough options");
        std::process::exit(1);
    }
}
