#![no_std] // dont link the Rust standard library
#![no_main] // disable main fn entry point for this program

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world!";

// this fn is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle/destroy the name of this function
// linker looks for _start by default - this is the entry point
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 +1) = 0xb;
        }
    }

    loop {}
}