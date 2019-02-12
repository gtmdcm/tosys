#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate pic8259_simple;
extern crate spin;
extern crate volatile;
extern crate x86_64;

use core::panic::PanicInfo;

use bootloader_precompiled::bootinfo;

#[macro_use]
mod vga;
mod gdt;
mod interrupts;
mod memory;

fn main(bootinfo: &'static bootinfo::BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    let mut recursive_page_table = memory::init(bootinfo.p4_table_addr as usize);
    let mut frame_allocator = memory::init_frame_allocator(&bootinfo.memory_map);
    memory::create_mapping(&mut recursive_page_table, &mut frame_allocator);
    unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e) };

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
