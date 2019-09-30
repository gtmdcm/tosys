#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate bootloader;
#[macro_use]
extern crate lazy_static;
extern crate pic8259_simple;
extern crate spin;
extern crate volatile;
extern crate x86_64;
extern crate alloc;
extern crate linked_list_allocator;

use core::panic::PanicInfo;

use bootloader::BootInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use memory::BootInfoFrameAllocator;
use alloc::boxed::Box;
use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;
use alloc::string::ToString;

#[macro_use]
mod vga;
mod gdt;
mod interrupts;
mod memory;
mod allocator;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

fn main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);

    println!("heap_value at {:p}", x);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    println!("It did not crash!");
    let string_test = "faq".to_string();
    println!("{:p}:{}", string_test.as_str(), string_test);
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

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}