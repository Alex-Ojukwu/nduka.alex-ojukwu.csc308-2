#![no_std]
#![no_main]

mod writer;

use bootloader_api::{config::Mapping, BootloaderConfig, BootInfo};
use writer::{FrameBufferWriter, TextColor};
use x86_64::instructions::hlt;
use core::fmt::Write;

bootloader_api::entry_point!(kernel_main);

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .unwrap_or_else(|| loop { hlt(); });
    let fb_info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, fb_info);

    // Use a raw string literal so that our custom escape sequences are preserved.
    print!(
        &mut frame_buffer_writer,
        r"watashi no Soul Society!\nTesting Testing Tester Tested.\n\cBlue Blue Text\tTabbed Text"
    );

    loop {
        hlt();
    }
}

macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {{
        // Format the input into a single string.
        let formatted = format!($($arg)*);
        let mut chars = formatted.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        'n' => $writer.newline(),
                        't' => $writer.write_tab(),
                        'c' => {
                            // Gather alphabetic characters for the color name.
                            let mut color_name = String::new();
                            while let Some(&ch) = chars.peek() {
                                if ch.is_alphabetic() {
                                    color_name.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Some(color) = TextColor::from_str(&color_name) {
                                $writer.set_color(color);
                            }
                        }
                        other => $writer.write_char(other),
                    }
                }
            } else {
                $writer.write_char(c);
            }
        }
    }};
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
