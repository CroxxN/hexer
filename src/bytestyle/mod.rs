use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

pub trait Writebyte {
    fn write_bin(&self, stdout: &mut StdoutLock, data: u8);
}

pub struct BHex;
pub struct BOct;
pub struct BInt;

impl Writebyte for BHex {
    fn write_bin(&self, stdout: &mut StdoutLock, data: u8) {
        hexer_write!(stdout, "{:<02x}", data);
    }
}
impl Writebyte for BOct {
    fn write_bin(&self, stdout: &mut StdoutLock, data: u8) {
        hexer_write!(stdout, "{:<02o}", data);
    }
}
impl Writebyte for BInt {
    fn write_bin(&self, stdout: &mut StdoutLock, data: u8) {
        hexer_write!(stdout, "{:<02}", data);
    }
}

// TODO: Implement for binary? Just make the binaries 8 bits long because u8
