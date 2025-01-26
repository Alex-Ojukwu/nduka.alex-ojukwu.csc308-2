#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod framebuffer;

// Define the `print!` macro
macro_rules! print {
    ($writer:expr, $text:expr) => {{
        let mut chars = $text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                // Handle escape sequences
                match chars.peek() {
                    Some('n') => {
                        $writer.write_char('\n');
                        chars.next(); // Consume the 'n'
                    }
                    Some('t') => {
                        $writer.write_char('\t');
                        chars.next(); // Consume the 't'
                    }
                    Some('c') => {
                        chars.next(); // Consume the 'c'
                        if let Some(color_char) = chars.next() {
                            if let Some(color) = color_char.to_digit(16) {
                                $writer.set_color(color as u8);
                            }
                        }
                    }
                    _ => $writer.write_char(c),
                }
            } else {
                $writer.write_char(c);
            }
        }
    }};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let mut writer = framebuffer::FrameBufferWriter::new();

    // Set the color to red for the first message
    writer.set_color(0x04); // Red color
    let message = "Yokoso Watashi No Soul Society";

    // Position the message at the bottom-right
    let message_length = message.len();
    let start_x = 80 - message_length; // SCREEN_WIDTH - message length
    let start_y = 25 - 1;              // SCREEN_HEIGHT - 1 (last row)

    writer.set_cursor(start_x, start_y);
    for c in message.chars() {
        writer.write_char(c);
    }

    // Reset to default color
    writer.set_color(0x07);

    // Use the `print!` macro for additional text
    print!(writer, "\\nWatashi no Reiatsu Jikaishiro!\\n");
    print!(writer, "This is an example \\t tabbed text.");
    print!(writer, "\\n\\c2Now in Green Color!\\n");

    loop {}
}
