#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;

mod vga;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga::print_something();
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}