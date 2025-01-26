#![no_std]
#![no_main]

mod writer;

use bootloader_api::{config::Mapping, BootloaderConfig, BootInfo};
use writer::{FrameBufferWriter, TextColor};
use x86_64::instructions::hlt;
use core::fmt::Write; // Required for the macros

// Use the `entry_point` macro to register the entry point function.
bootloader_api::entry_point!(kernel_main);

// Define a custom bootloader configuration.
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

// Define the entry point function.
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Retrieve framebuffer info and buffer
    let framebuffer = boot_info.framebuffer.as_mut().unwrap_or_else(|| {
        loop {
            hlt(); // Halt if framebuffer is unavailable
        }
    });

    let frame_buffer_info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    // Initialize the FrameBufferWriter
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    // Center "Hello World!" on the screen
    let text_1 = "Mente Cuerpo y Alma!";
    let text_1_width = text_1.len() * writer::constants::font_constants::CHAR_RASTER_WIDTH;
    let text_1_height = writer::constants::font_constants::CHAR_RASTER_HEIGHT.val();
    let text_1_x = (frame_buffer_writer.width() - text_1_width) / 2;
    let text_1_y = (frame_buffer_writer.height() - text_1_height) / 2;

    frame_buffer_writer.set_cursor_position(text_1_x as isize, text_1_y as isize);
    frame_buffer_writer.set_color(TextColor::Green);
    writeln!(frame_buffer_writer, "{}", text_1).unwrap();

    // Center the tabbed line below "Hello World!"
    let text_2 = "This is an example of\t tabbed Line";
    let text_2_width = text_2.len() * writer::constants::font_constants::CHAR_RASTER_WIDTH;
    let text_2_height = writer::constants::font_constants::CHAR_RASTER_HEIGHT.val();
    let text_2_x = (frame_buffer_writer.width() - text_2_width) / 2;
    let text_2_y = text_1_y + text_1_height + 5; // Add 5 pixels as vertical spacing

    frame_buffer_writer.set_cursor_position(text_2_x as isize, text_2_y as isize);
    frame_buffer_writer.set_color(TextColor::Yellow);
    writeln!(frame_buffer_writer, "{}", text_2).unwrap();

    // Halt the CPU to prevent unnecessary busy looping
    loop {
        hlt();
    }
}

// Define the panic handler to handle panics gracefully.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt(); // Halt in an infinite loop on panic.
    }
}
