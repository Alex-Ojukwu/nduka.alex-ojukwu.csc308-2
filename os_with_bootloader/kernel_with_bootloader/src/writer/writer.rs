use crate::writer::constants;

use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{fmt, ptr};

/// Additional spacing configurations.
pub const BORDER_PADDING: usize = 1;

/// Supported text colors.
#[derive(Copy, Clone)]
pub enum TextColor {
    White,
    Red,
    Green,
    Blue,
    Yellow,
}

impl TextColor {
    /// Converts a string (case-insensitively) into a TextColor.
    pub fn from_str(color: &str) -> Option<Self> {
        if color.eq_ignore_ascii_case("white") {
            Some(TextColor::White)
        } else if color.eq_ignore_ascii_case("red") {
            Some(TextColor::Red)
        } else if color.eq_ignore_ascii_case("green") {
            Some(TextColor::Green)
        } else if color.eq_ignore_ascii_case("blue") {
            Some(TextColor::Blue)
        } else if color.eq_ignore_ascii_case("yellow") {
            Some(TextColor::Yellow)
        } else {
            None
        }
    }
}

/// Retrieves the raster of the given char or a backup char.
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Failed to load backup char raster"))
}

/// A writer for logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    color: TextColor,
}

impl FrameBufferWriter {
    /// Creates a new writer with the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_pos: BORDER_PADDING,
            y_pos: BORDER_PADDING,
            color: TextColor::White,
        };
        writer.clear();
        writer
    }

    /// Dynamically sets the cursor position.
    pub fn set_cursor_position(&mut self, x: isize, y: isize) {
        if x < 0 || y < 0 {
            self.x_pos = BORDER_PADDING;
            self.y_pos = BORDER_PADDING;
        } else {
            self.x_pos = (x as usize).min(self.width().saturating_sub(BORDER_PADDING));
            self.y_pos = (y as usize).min(self.height().saturating_sub(BORDER_PADDING));
        }
    }

    /// Clears the entire framebuffer.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    /// Retrieves the framebuffer width.
    pub fn width(&self) -> usize {
        self.info.width
    }

    /// Retrieves the framebuffer height.
    pub fn height(&self) -> usize {
        self.info.height
    }

    /// Advances to a new line.
    pub fn newline(&mut self) {
        self.y_pos += CHAR_RASTER_HEIGHT.val() + 2;
        self.carriage_return();
        if self.y_pos >= self.height() {
            self.scroll();
        }
    }

    /// Resets the position to the start of the line.
    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    /// Scrolls the screen content upward by one line.
    fn scroll(&mut self) {
        let row_height = CHAR_RASTER_HEIGHT.val() + 2;
        let screen_bytes = self.info.width * self.info.bytes_per_pixel;
        let row_bytes = row_height * self.info.stride * self.info.bytes_per_pixel;

        self.framebuffer.copy_within(row_bytes..screen_bytes, 0);

        let start_of_last_row = screen_bytes - row_bytes;
        self.framebuffer[start_of_last_row..screen_bytes].fill(0);

        self.y_pos = self.height() - row_height - BORDER_PADDING;
    }

    /// Writes a single character to the framebuffer.
    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            '\t' => self.write_tab(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos = self.y_pos + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.scroll();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    /// Writes a tab space.
    pub fn write_tab(&mut self) {
        self.x_pos += font_constants::CHAR_RASTER_WIDTH * 4;
        if self.x_pos >= self.width() {
            self.newline();
        }
    }

    /// Renders a character to the framebuffer.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width();
    }

    /// Writes a pixel at the specified position with the given intensity.
    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        // Choose the color ordering based on pixel format.
        // Note: The alpha channel is set to 0xFF (opaque).
        let color = match self.info.pixel_format {
            // For PixelFormat::Rgb, assume ordering: R, G, B, A.
            bootloader_api::info::PixelFormat::Rgb => match self.color {
                TextColor::White  => [intensity, intensity, intensity, 0xFF],
                TextColor::Red    => [intensity, 0, 0, 0xFF],
                TextColor::Green  => [0, intensity, 0, 0xFF],
                TextColor::Blue   => [0, 0, intensity, 0xFF],
                TextColor::Yellow => [intensity, intensity, 0, 0xFF],
            },
            // For PixelFormat::Bgr, assume ordering: B, G, R, A.
            bootloader_api::info::PixelFormat::Bgr => match self.color {
                TextColor::White  => [intensity, intensity, intensity, 0xFF],
                TextColor::Red    => [0, 0, intensity, 0xFF],
                TextColor::Green  => [0, intensity, 0, 0xFF],
                TextColor::Blue   => [intensity, 0, 0, 0xFF],
                TextColor::Yellow => [0, intensity, intensity, 0xFF],
            },
            // Fallback for other pixel formats.
            _ => match self.color {
                TextColor::White  => [intensity, intensity, intensity, 0xFF],
                TextColor::Red    => [intensity, 0, 0, 0xFF],
                TextColor::Green  => [0, intensity, 0, 0xFF],
                TextColor::Blue   => [0, 0, intensity, 0xFF],
                TextColor::Yellow => [intensity, intensity, 0, 0xFF],
            },
        };

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);

        unsafe {
            ptr::read_volatile(&self.framebuffer[byte_offset]);
        }
    }

    /// Changes the text color.
    pub fn set_color(&mut self, color: TextColor) {
        self.color = color;
    }
}

// Safety guarantees for multithreading.
unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
