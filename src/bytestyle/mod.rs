use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

pub trait Bytestyle {
    fn print(&self, stdout: &mut StdoutLock, data: &u8);
}

pub struct BHex;
pub struct BOct;
pub struct BInt;

pub fn from_str(value: &str) -> Box<dyn Bytestyle> {
    match value {
        "x" | "hex" => Box::new(BHex),
        "o" | "octal" => Box::new(BOct),
        _ => Box::new(BInt),
    }
}

impl Bytestyle for BHex {
    fn print(&self, stdout: &mut StdoutLock, data: &u8) {
        hexer_write!(stdout, "{:<02x} ", *data);
    }
}
impl Bytestyle for BInt {
    fn print(&self, stdout: &mut StdoutLock, data: &u8) {
        hexer_write!(stdout, "{:<02} ", *data);
    }
}
impl Bytestyle for BOct {
    fn print(&self, stdout: &mut StdoutLock, data: &u8) {
        hexer_write!(stdout, "{:<02o} ", *data);
    }
}
