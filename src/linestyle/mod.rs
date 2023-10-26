use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

pub trait Linestyle {
    fn print(&self, stdout: &mut StdoutLock, position: usize);
}

pub struct Hex;
pub struct Int;
pub struct Oct;

// impl Line<>
// Trait object. Yay
pub fn from_str(value: &str) -> Box<dyn Linestyle> {
    match value {
        "x" | "hex" => Box::new(Hex),
        "o" | "octal" => Box::new(Oct),
        _ => Box::new(Int),
    }
}

impl Linestyle for Hex {
    fn print(&self, stdout: &mut StdoutLock, position: usize) {
        hexer_write!(stdout, "{:#08x}", position);
    }
}
impl Linestyle for Int {
    fn print(&self, stdout: &mut StdoutLock, position: usize) {
        hexer_write!(stdout, "{:#06}", position);
    }
}
impl Linestyle for Oct {
    fn print(&self, stdout: &mut StdoutLock, position: usize) {
        hexer_write!(stdout, "{:#08o}", position);
    }
}
