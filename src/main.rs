#![no_std]
#![no_main]

#![feature(ptr_internals)]

#[macro_use]
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga::clear_screen();

    println!("Hello, kernel!");

    println!("Did not crash!");

    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}