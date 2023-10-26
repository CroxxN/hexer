// TODO: delete this

use crate::bytestyle::Bytestyle;
use crate::colors::*;
use crate::common::hexer_write;
use crate::linestyle::Linestyle;
use std::io::Write;

pub struct Stat {
    args: String,
    size: u64,
    position: usize,
}

impl Stat {
    pub fn new(args: String, size: u64, position: usize) -> Self {
        Self {
            args,
            size,
            position,
        }
    }
}

pub trait Hexwrite<'a> {
    // Return error
    fn new(stdout: std::io::StdoutLock<'a>, linefmt: Linestyle, bytefmt: Bytestyle) -> Self;
    fn write_line(&mut self, position: usize);
    fn write_bytes(&mut self, data: &u8);
    fn write_stats(&mut self, stats: Stat);
}

pub enum Colorstyle<'a> {
    Color(Color<'a>),
    NColor(NColor<'a>),
}

impl<'a> Colorstyle<'a> {
    pub fn printline(&mut self, position: usize) {
        match self {
            Colorstyle::Color(c) => c.write_line(position),
            Colorstyle::NColor(nc) => nc.write_line(position),
        }
    }
    pub fn printbyte(&mut self, byte: &u8) {
        match self {
            Colorstyle::Color(c) => c.write_bytes(byte),
            Colorstyle::NColor(nc) => nc.write_bytes(byte),
        }
    }
    pub fn printstats(&mut self, stats: Stat) {
        match self {
            Colorstyle::Color(c) => c.write_stats(stats),
            Colorstyle::NColor(nc) => nc.write_stats(stats),
        }
    }
}

pub struct Color<'a> {
    stdout: std::io::StdoutLock<'a>,
    // linefmt: Box<dyn Writeline>,
    linefmt: Linestyle,
    bytefmt: Bytestyle,
}

pub struct NColor<'b> {
    stdout: std::io::StdoutLock<'b>,
    linefmt: Linestyle,
    bytefmt: Bytestyle,
}

impl<'a> Hexwrite<'a> for Color<'a> {
    fn new(stdout: std::io::StdoutLock<'a>, linefmt: Linestyle, bytefmt: Bytestyle) -> Self {
        Self {
            stdout,
            linefmt,
            bytefmt,
        }
    }
    fn write_line(&mut self, position: usize) {
        hexer_write!(self.stdout, "{BGREEN}");
        // write_line(&mut self.stdout, position, self.linefmt);
        self.linefmt.print(&mut self.stdout, position);
        hexer_write!(self.stdout, "{END}");
        // hexer_write!(self.stdout, "{BGREEN}{:0>6}{END}", T::get_line());
    }
    fn write_bytes(&mut self, data: &u8) {
        match *data {
            0x00 => hexer_write!(&mut self.stdout, "{BRED}00{END}"),
            _ => self.bytefmt.print(&mut self.stdout, data),
        }
    }
    fn write_stats(&mut self, stats: Stat) {
        hexer_write!(
            &mut self.stdout,
            "\n{BGREEN}{}{END} of {BGREEN}{}{END} bytes displayed in {BGREEN}{}{END} lines\n",
            stats.args,
            stats.size,
            stats.position
        );
    }
}

impl<'b> Hexwrite<'b> for NColor<'b> {
    fn new(stdout: std::io::StdoutLock<'b>, linefmt: Linestyle, bytefmt: Bytestyle) -> Self {
        Self {
            stdout,
            linefmt,
            bytefmt,
        }
    }
    fn write_line(&mut self, position: usize) {
        self.linefmt.print(&mut self.stdout, position);
    }
    fn write_bytes(&mut self, data: &u8) {
        self.bytefmt.print(&mut self.stdout, data);
    }
    fn write_stats(&mut self, stats: Stat) {
        hexer_write!(
            &mut self.stdout,
            "\n{} of {} bytes displayed in {} lines\n",
            stats.args,
            stats.size,
            stats.position
        );
    }
}
