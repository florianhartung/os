#![no_std]

use spin::{Mutex, Once};

use core::{fmt::Write, panic::PanicInfo};

use crate::vga::{buffer::VgaBuffer, writer::VgaWriter};

mod vga;

static VGA_BUFFER: Once<Mutex<VgaBuffer>> = Once::new();

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    {
        // SAFETY: The method is called once and exclusive access to VGA buffer is
        // ensured.
        let vga_buffer = unsafe { vga::buffer::take() };
        VGA_BUFFER.call_once(|| Mutex::new(vga_buffer));
    }

    let mut vga_buffer_guard = VGA_BUFFER.wait().lock();
    let mut writer = VgaWriter::new(&mut *vga_buffer_guard);

    writeln!(writer, "Hello").unwrap();
    writeln!(writer, "  World!").unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vga_buffer = VGA_BUFFER.wait();
    // TODO this is likely unsound. we might be able to call vga::buffer::take()
    // a second time?
    unsafe { vga_buffer.force_unlock() };
    let mut vga_buffer_guard = vga_buffer.lock();
    let mut writer = VgaWriter::new(&mut *vga_buffer_guard);

    let _ = write!(writer, "{info}");
    loop {}
}
