#![no_std]
#![no_main]
#![feature(ptr_internals)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga::clear_screen();

    println!("Hello, kernel!");

    #[cfg(test)]
    test_main();

    println!("Did not crash!");

    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}