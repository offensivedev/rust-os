#![no_std] // dont link the Rust standard library
#![no_main] // disable main fn entry point for this program

use core::panic::PanicInfo;

// this fn is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle/destroy the name of this function
pub extern "C" fn _start() -> ! {
    // linker looks for _start by default - this is the entry point
    loop {}
}