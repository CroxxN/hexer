// Get the compiler to stop yelling at unused colors
#![allow(dead_code)]

// reset everything to normal sequence
pub const END: &str = "\x1b[0m";

// Normal foreground colors
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const DEFAULT: &str = "\x1b[39m";

// Bold aixterm colors. See https://sites.ualberta.ca/dept/chemeng/AIX-43/share/man/info/C/a_doc_lib/cmds/aixcmds1/aixterm.htm

pub const BBLACK: &str = "\x1b[1;30m";
pub const BRED: &str = "\x1b[1;31m";
pub const BGREEN: &str = "\x1b[1;32m";
pub const BYELLOW: &str = "\x1b[1;33m";
pub const BBLUE: &str = "\x1b[1;34m";
pub const BMAGENTA: &str = "\x1b[1;35m";
pub const BCYAN: &str = "\x1b[1;36m";
pub const BWHITE: &str = "\x1b[1;37m";
