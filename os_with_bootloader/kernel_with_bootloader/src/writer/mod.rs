pub mod writer;
pub mod constants;

pub use writer::{FrameBufferWriter, BORDER_PADDING, TextColor}; 
#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        write!($writer, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    ($writer:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!($writer, $($arg)*).unwrap();
    }};
}

