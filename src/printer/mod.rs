use crate::bytestyle::Bytestyle;
use crate::colors::*;
use crate::common::hexer_write;
use crate::linestyle::Linestyle;
use crate::Stat;
use std::io::Write;

pub trait Hexwrite<'a> {
    // TODO: Return error
    fn write_line(&mut self, position: usize);
    fn write_bytes(&mut self, data: &u8);
    fn write_stats(&mut self, stats: Stat);
}

struct Color<'a> {
    stdout: std::io::StdoutLock<'a>,
    linefmt: Box<dyn Linestyle>,
    bytefmt: Box<dyn Bytestyle>,
}

struct NColor<'b> {
    stdout: std::io::StdoutLock<'b>,
    linefmt: Box<dyn Linestyle>,
    bytefmt: Box<dyn Bytestyle>,
}

impl<'a> Hexwrite<'a> for Color<'a> {
    fn write_line(&mut self, position: usize) {
        hexer_write!(self.stdout, "{BGREEN}");
        self.linefmt.print(&mut self.stdout, position);
        hexer_write!(self.stdout, "{END}");
    }
    fn write_bytes(&mut self, data: &u8) {
        match *data {
            0x00 => hexer_write!(&mut self.stdout, "{BRED}00{END} "),
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

pub fn new_color(
    stdout: std::io::StdoutLock,
    linefmt: Box<dyn Linestyle>,
    bytefmt: Box<dyn Bytestyle>,
) -> Box<dyn Hexwrite + '_> {
    Box::new(Color {
        stdout,
        linefmt,
        bytefmt,
    })
}

pub fn new_ncolor(
    stdout: std::io::StdoutLock,
    linefmt: Box<dyn Linestyle>,
    bytefmt: Box<dyn Bytestyle>,
) -> Box<dyn Hexwrite + '_> {
    Box::new(NColor {
        stdout,
        linefmt,
        bytefmt,
    })
}
