use crate::common::hexer_write;
use std::io::{StdoutLock, Write};

pub struct Hex;
pub struct Int;
pub struct Oct;

pub trait Writeline {
    fn write_line(&self, stdout: &mut StdoutLock, position: u32);
}

impl Writeline for Hex {
    fn write_line(&self, stdout: &mut StdoutLock, position: u32) {
        hexer_write!(stdout, "{:0>6x}", position);
    }
}

impl Writeline for Int {
    fn write_line(&self, stdout: &mut StdoutLock, position: u32) {
        hexer_write!(stdout, "{:0>6}", position);
    }
}

impl Writeline for Oct {
    fn write_line(&self, stdout: &mut StdoutLock, position: u32) {
        hexer_write!(stdout, "{:0>6o}", position);
    }
}
