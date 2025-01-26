const VGA_BUFFER: usize = 0xb8000;
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

pub struct FrameBufferWriter {
    cursor_x: usize,
    cursor_y: usize,
    color: u8, // Text color
}

impl FrameBufferWriter {
    pub fn new() -> Self {
        FrameBufferWriter {
            cursor_x: 0,
            cursor_y: 0,
            color: 0x07, // Default light gray
        }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.cursor_x = x;
            self.cursor_y = y;
        }
    }

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
                if self.cursor_y >= SCREEN_HEIGHT {
                    self.scroll_up();
                    self.cursor_y = SCREEN_HEIGHT - 1;
                }
            }
            '\t' => {
                let next_tab = (self.cursor_x / 4 + 1) * 4;
                while self.cursor_x < next_tab && self.cursor_x < SCREEN_WIDTH {
                    self.write_char(' '); // Recursively handle tab expansion
                }
            }
            _ => {
                if self.cursor_y >= SCREEN_HEIGHT {
                    self.scroll_up();
                    self.cursor_y = SCREEN_HEIGHT - 1;
                }

                let offset = 2 * (self.cursor_y * SCREEN_WIDTH + self.cursor_x);
                unsafe {
                    let vga_buffer = VGA_BUFFER as *mut u8;
                    *vga_buffer.offset(offset as isize) = c as u8;
                    *vga_buffer.offset(offset as isize + 1) = self.color;
                }

                self.cursor_x += 1;
                if self.cursor_x >= SCREEN_WIDTH {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
            }
        }
    }

    fn scroll_up(&mut self) {
        unsafe {
            let vga_buffer = VGA_BUFFER as *mut u8;
            for row in 1..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let dest_offset = 2 * ((row - 1) * SCREEN_WIDTH + col);
                    let src_offset = 2 * (row * SCREEN_WIDTH + col);
                    *vga_buffer.offset(dest_offset as isize) =
                        *vga_buffer.offset(src_offset as isize);
                    *vga_buffer.offset(dest_offset as isize + 1) =
                        *vga_buffer.offset(src_offset as isize + 1);
                }
            }
            // Clear the last row
            for col in 0..SCREEN_WIDTH {
                let offset = 2 * ((SCREEN_HEIGHT - 1) * SCREEN_WIDTH + col);
                *vga_buffer.offset(offset as isize) = b' ' as u8;
                *vga_buffer.offset(offset as isize + 1) = 0x07; // Default color
            }
        }
    }
}
