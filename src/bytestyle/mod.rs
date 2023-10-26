use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

// pub trait Writebyte {
//     fn write_bin(&self, stdout: &mut StdoutLock, data: u8);
// }

#[derive(Clone)]
pub enum Bytestyle {
    BHex,
    BOct,
    BInt,
}

impl From<String> for Bytestyle {
    // TODO: Change option to result
    fn from(value: String) -> Self {
        match value.as_str() {
            "x" | "hex" => Self::BHex,
            "o" | "octal" => Self::BOct,
            _ => Self::BInt,
        }
    }
}

impl Bytestyle {
    pub fn print(&self, stdout: &mut StdoutLock, data: &u8) {
        match self {
            Self::BHex => hexer_write!(stdout, "{:<02x} ", *data),
            // 1 byte can translate upto ~3 digits
            Self::BOct => hexer_write!(stdout, "{:<03o} ", *data),
            Self::BInt => hexer_write!(stdout, "{:<03} ", *data),
        }
    }
}

// TODO: Implement for binary? Just make the binaries 8 bits long because u8
