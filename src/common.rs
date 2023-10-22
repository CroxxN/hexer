macro_rules! hexer_write{
    ($dst:expr, $($arg:tt)*) => {
        // use std::format_args;
        _ = $dst.write_fmt(::std::format_args!($($arg)*))
    };
}

pub(crate) use hexer_write;
