#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate bootloader;
#[macro_use]
extern crate lazy_static;
extern crate pic8259_simple;
extern crate spin;
extern crate volatile;
extern crate x86_64;

use core::panic::PanicInfo;

use bootloader::{BootInfo};
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use memory::BootInfoFrameAllocator;

#[macro_use]
mod vga;
mod gdt;
mod interrupts;
mod memory;

fn main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    loop {
        x86_64::instructions::hlt();
    }
}

entry_point!(main);

/// This function is called on paqnic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
