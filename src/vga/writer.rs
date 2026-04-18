use crate::vga::buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, VgaBuffer, VgaChar, VgaColor};

pub struct VgaWriter<'a> {
    column_position: usize,
    fg_bg_colors: (VgaColor, VgaColor),
    vga_buffer: &'a mut VgaBuffer,
}

impl<'a> VgaWriter<'a> {
    pub fn new(vga_buffer: &'a mut VgaBuffer) -> Self {
        Self {
            column_position: 0,
            fg_bg_colors: (VgaColor::White, VgaColor::Black),
            vga_buffer,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            other_byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let screen_char =
                    VgaChar::new(other_byte, self.fg_bg_colors.0, self.fg_bg_colors.1);

                self.vga_buffer
                    .try_write(row, self.column_position, screen_char);

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row_index in 1..BUFFER_HEIGHT {
            for column_index in 0..BUFFER_WIDTH {
                let value = self
                    .vga_buffer
                    .try_read(row_index, column_index)
                    .expect("indices to be within bounds");

                self.vga_buffer
                    .try_write(row_index - 1, column_index, value);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row_index: usize) {
        let blank_char = VgaChar::new(b' ', VgaColor::White, VgaColor::Black);

        for column_index in 0..BUFFER_WIDTH {
            self.vga_buffer
                .try_write(row_index, column_index, blank_char);
        }
    }
}

impl<'a> core::fmt::Write for VgaWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }

        Ok(())
    }
}
