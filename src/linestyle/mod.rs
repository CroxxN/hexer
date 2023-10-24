use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

pub struct Hex;
pub struct Int;
pub struct Oct;

pub enum Linestyle {
    Hex,
    Int,
    Oct,
}

impl From<String> for Linestyle {
    // TODO: Change option to result
    fn from(value: String) -> Self {
        match value.as_str() {
            "x" | "hex" => Self::Hex,
            "o" | "octal" => Self::Oct,
            _ => Self::Int,
        }
    }
}

impl Linestyle {
    pub fn print(&self, stdout: &mut StdoutLock, position: u32) {
        match self {
            Linestyle::Hex => hexer_write!(stdout, "{:0>6x}", position),
            Linestyle::Int => hexer_write!(stdout, "{:0>6}", position),
            Linestyle::Oct => hexer_write!(stdout, "{:0>6o}", position),
        }
    }
}
