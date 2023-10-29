macro_rules! hexer_write{
    ($dst:expr, $($arg:tt)*) => {
        // use std::format_args;
        if let Err(_) = $dst.write_fmt(::std::format_args!($($arg)*)){
            // Silently exit. FOR NOW.
            ::std::process::exit(0);
        }
    };
}

pub(crate) use hexer_write;
