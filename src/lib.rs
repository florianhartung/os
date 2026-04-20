#![no_std]

use spin::{Mutex, Once};

use core::{fmt::Write, panic::PanicInfo};

use crate::vga::{buffer::VgaBuffer, writer::VgaWriter};

mod vga;

static VGA_WRITER: Once<Mutex<VgaWriter>> = Once::new();

macro_rules! print {
    ($($arg:tt)*) => {{
            let mut vga_writer_guard = VGA_WRITER.get()
                .expect("writer to be initialized")
                .lock();
            vga_writer_guard.write_fmt(format_args!($($arg)*))
                .expect("VGA writes to always be successful");
    }};
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    {
        // SAFETY: The method is called once and exclusive access to VGA buffer is
        // ensured.
        let vga_buffer = unsafe { vga::buffer::take() };
        let vga_writer = Mutex::new(VgaWriter::new(vga_buffer));
        VGA_WRITER.call_once(|| vga_writer);
    }

    print!("Hello world");

    // writeln!(writer, "  World!").unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("\n{info}");
    loop {}
}
