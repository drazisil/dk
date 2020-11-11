#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(ptr_internals)]

extern crate volatile;
extern crate multiboot2;

pub mod interrupts;

#[macro_use]
pub mod vga_buffer;
pub mod memory;
mod serial;

use core::panic::PanicInfo;
pub use crate::memory::{AreaFrameAllocator,FrameAllocator};

pub fn init() {
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    println!("Hello World{}", "!");

    init();

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel start:{}, kernel_end:{}", kernel_start, kernel_end);
    println!("multiboot start:{}, multiboot_end:{}", multiboot_start, multiboot_end);
   
    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());
    
    println!("{:?}", frame_allocator.allocate_frame());
    
    for i in 0..4 {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }


    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    println!("It did not crash!");
    loop {}
}


/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}