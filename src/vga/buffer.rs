use num_enum::{IntoPrimitive, TryFromPrimitive};

use core::{hint, ptr};

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

/// Returns an exclusively owned [`VgaBuffer`]
///
/// # Safety
///
/// The caller must ensure that this method is only ever called once.
/// Also there must not be any other reads or writes to the VGA buffer.
pub unsafe fn take() -> VgaBuffer {
    VgaBuffer(())
}

pub struct VgaBuffer(());

impl VgaBuffer {
    /// Writes a [`VgaChar`] into the VGA buffer
    ///
    /// # Safety
    ///
    /// - `row_index` must be smaller than [`BUFFER_HEIGHT`]
    /// - `column_index` must be smaller than [`BUFFER_WIDTH`]
    pub unsafe fn write_unchecked(
        &mut self,
        row_index: usize,
        column_index: usize,
        new_value: VgaChar,
    ) {
        if row_index >= BUFFER_HEIGHT {
            // SAFETY: Caller ensures row index to be within bounds
            unsafe { hint::unreachable_unchecked() };
        }

        if column_index >= BUFFER_WIDTH {
            // SAFETY: Caller ensures column index to be within bounds
            unsafe { hint::unreachable_unchecked() };
        }

        // There is no need for provenance when performing volatile writes to
        // I/O mapped memory.
        let vga_buffer: *mut VgaChar = ptr::without_provenance_mut(0xB8000);

        // SAFETY: Both safety requirements are met. ScreenChar is 2 bytes
        // large. The caller guarantees the indices to be within bounds.
        // Therefore, the maximum offset is 2*(25-1)*(80-1) bytes which also fits in an
        // isize.
        let value = unsafe { vga_buffer.add(row_index * BUFFER_WIDTH + column_index) };

        // SAFETY: The targeted memory is not a Rust allocation. Therefore, it
        // is not required for it to be valid for writes. However, the ownership
        // of the current `VgaBuffer` object guarantees exclusive access to the
        // VGA buffer memory region.
        unsafe { ptr::write_volatile(value, new_value) };
    }

    /// Writes a [`VgaChar`] into the VGA buffer
    pub fn try_write(&mut self, row_index: usize, column_index: usize, new_value: VgaChar) {
        if row_index < BUFFER_HEIGHT && column_index < BUFFER_WIDTH {
            // SAFETY: Both indices are within bounds
            unsafe { self.write_unchecked(row_index, column_index, new_value) };
        }
    }

    /// Reads a [`VgaChar`] from the VGA buffer
    ///
    /// # Safety
    ///
    /// - `row_index` must be smaller than [`BUFFER_HEIGHT`]
    /// - `column_index` must be smaller than [`BUFFER_WIDTH`]
    pub unsafe fn read_unchecked(&self, row_index: usize, column_index: usize) -> VgaChar {
        if row_index >= BUFFER_HEIGHT {
            // SAFETY: Caller ensures row index to be within bounds
            unsafe { hint::unreachable_unchecked() };
        }

        if column_index >= BUFFER_WIDTH {
            // SAFETY: Caller ensures column index to be within bounds
            unsafe { hint::unreachable_unchecked() };
        }

        // There is no need for provenance when performing volatile writes to
        // I/O mapped memory.
        let vga_buffer: *const VgaChar = ptr::without_provenance_mut(0xB8000);

        // SAFETY: Both safety requirements are met. ScreenChar is 2 bytes
        // large. The caller guarantees the indices to be within bounds.
        // Therefore, the maximum offset is 2*(25-1)*(80-1) bytes which also fits in an
        // isize.
        let value = unsafe { vga_buffer.add(row_index * BUFFER_WIDTH + column_index) };

        // SAFETY: The targeted memory is not a Rust allocation. Therefore, it
        // is not required for it to be valid for reads. However, the ownership
        // of the current `VgaBuffer` object guarantees shared access to the VGA
        // buffer memory region.
        unsafe { ptr::read_volatile(value) }
    }

    /// Reads a ['VgaChar`] from the VGA buffer
    pub fn try_read(&mut self, row_index: usize, column_index: usize) -> Option<VgaChar> {
        (row_index < BUFFER_HEIGHT && column_index < BUFFER_WIDTH).then(|| {
            // SAFETY: Both indices are within bounds
            unsafe { self.read_unchecked(row_index, column_index) }
        })
    }
}

#[repr(u8)]
#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum VgaColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VgaChar {
    /// # Safety
    ///
    /// Must be less than 128
    ascii_char: u8,
    attributes: u8,
}

impl VgaChar {
    pub fn new(ascii_char: u8, foreground: VgaColor, background: VgaColor) -> Self {
        // TODO Is is always correct to just ignore the MSB? Be safe for now
        let ascii_char = ascii_char & 0x7F;

        let attributes = (u8::from(foreground)) | (u8::from(background)) << 4;

        Self {
            ascii_char,
            attributes,
        }
    }
}
