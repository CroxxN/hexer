// Get the compiler to stop yelling at unused colors
#![allow(dead_code)]

// reset everything to normal sequence
pub const END: &'static str = "\x1b[0m";

// Normal foreground colors
pub const BLACK: &'static str = "\x1b[30m";
pub const RED: &'static str = "\x1b[31m";
pub const GREEN: &'static str = "\x1b[32m";
pub const YELLOW: &'static str = "\x1b[33m";
pub const BLUE: &'static str = "\x1b[34m";
pub const MAGENTA: &'static str = "\x1b[35m";
pub const CYAN: &'static str = "\x1b[36m";
pub const WHITE: &'static str = "\x1b[37m";
pub const DEFAULT: &'static str = "\x1b[39m";

// Bold aixterm colors. See https://sites.ualberta.ca/dept/chemeng/AIX-43/share/man/info/C/a_doc_lib/cmds/aixcmds1/aixterm.htm

pub const BBLACK: &'static str = "\x1b[1;30m";
pub const BRED: &'static str = "\x1b[1;31m";
pub const BGREEN: &'static str = "\x1b[1;32m";
pub const BYELLOW: &'static str = "\x1b[1;33m";
pub const BBLUE: &'static str = "\x1b[1;34m";
pub const BMAGENTA: &'static str = "\x1b[1;35m";
pub const BCYAN: &'static str = "\x1b[1;36m";
pub const BWHITE: &'static str = "\x1b[1;37m";
