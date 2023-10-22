// TODO: delete this

use crate::bytestyle::*;
use crate::colors::*;
use crate::common::hexer_write;
use crate::linestyle::*;
use std::io::Write;

struct Stat {
    args: u32,
    size: u32,
    position: u32,
}

pub trait Hexwrite {
    // Return error
    fn write_line(&mut self, position: u32);
    fn write_stdout(&mut self, data: &u8);
    fn write_stats(&mut self, stats: Stat);
}

pub struct Color<'a, T, U>
where
    T: Writeline,
    U: Writebyte,
{
    stdout: std::io::StdoutLock<'a>,
    // linefmt: Box<dyn Writeline>,
    linefmt: T,
    bytefmt: U,
}

pub struct NColor<'a, T, U> {
    stdout: std::io::StdoutLock<'a>,
    linefmt: T,
    bytefmt: U,
}

impl<'a, T, U> Hexwrite for Color<'a, T, U>
where
    T: Writeline,
    U: Writebyte,
{
    fn write_line(&mut self, position: u32) {
        hexer_write!(self.stdout, "{BGREEN}");
        // write_line(&mut self.stdout, position, self.linefmt);
        self.linefmt.write_line(&mut self.stdout, position);
        hexer_write!(self.stdout, "{END}");
        // hexer_write!(self.stdout, "{BGREEN}{:0>6}{END}", T::get_line());
    }
    fn write_stdout(&mut self, data: &u8) {
        match *data {
            0x00 => hexer_write!(&mut self.stdout, "{BRED}00{END}"),
            _ => hexer_write!(&mut self.stdout, "{}", *data),
        }
    }
    fn write_stats(&mut self, stats: Stat) {
        hexer_write!(
            self.stdout,
            "\n{BGREEN}{}{END} of {BGREEN}{}{END} bytes displayed in {BGREEN}{}{END} lines\n",
            stats.args,
            stats.size,
            stats.position
        );
    }
}

impl<'a, T, U> Hexwrite for NColor<'a, T, U>
where
    T: Writeline,
    U: Writebyte,
{
    fn write_line(&mut self, position: u32) {
        self.linefmt.write_line(&mut self.stdout, position);
    }
    fn write_stdout(&mut self, data: &u8) {
        self.bytefmt.write_bin(&mut self.stdout, *data);
    }
    fn write_stats(&mut self, stats: Stat) {
        hexer_write!(
            self.stdout,
            "\n{} of {} bytes  displayed in {} lines\n",
            stats.args,
            stats.size,
            stats.position
        );
    }
}