pub mod writer;
pub mod constants;

pub use writer::{FrameBufferWriter, BORDER_PADDING, TextColor};

#[macro_export]
macro_rules! print {
    ($writer:expr, $text:expr) => {{
        use core::fmt::Write;
        let mut text = $text;
        
        // Handle escape sequences
        while let Some(pos) = text.find('\\') {
            let (before, rest) = text.split_at(pos);
            let mut rest_iter = rest.chars();
            rest_iter.next(); // Skip '\'

            match rest_iter.next() {
                Some('n') => {
                    write!($writer, "{}", before).unwrap();
                    $writer.newline();
                    text = rest_iter.as_str();
                }
                Some('t') => {
                    write!($writer, "{}", before).unwrap();
                    $writer.write_tab();
                    text = rest_iter.as_str();
                }
                Some('c') => {
                    if let Some(end_pos) = rest_iter.as_str().find(' ') {
                        let (color_name, remaining) = rest_iter.as_str().split_at(end_pos);
                        if let Some(color) = TextColor::from_str(color_name) {
                            $writer.set_color(color);
                        }
                        text = remaining;
                    } else {
                        text = "";
                    }
                }
                _ => {
                    write!($writer, "{}\\", before).unwrap();
                    text = rest_iter.as_str();
                }
            }
        }

        // Write the remaining text
        write!($writer, "{}", text).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    ($writer:expr, $text:expr) => {{
        print!($writer, $text);
        $writer.newline();
    }};
}
