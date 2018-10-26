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

use bootloader_precompiled::bootinfo;
use core::panic::PanicInfo;
use interrupts::init_idt;
use x86_64::PhysAddr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::PageTableEntry;
use x86_64::structures::paging::PageTableFlags;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::RecursivePageTable;
use x86_64::ux::u9;
use x86_64::VirtAddr;

#[macro_use]
mod vga;
mod gdt;
mod interrupts;


fn main(bootinfo: &'static bootinfo::BootInfo) -> ! {
//    meminfo(bootinfo);
    gdt::init();
    interrupts::init_idt();
    loop {
        x86_64::instructions::hlt();
    }
}

fn meminfo(bootinfo: &'static bootinfo::BootInfo) {
    println!("p4_table_addr: {:?}", bootinfo.p4_table_addr);
    for index in 0..bootinfo.memory_map.len() - 1 {
        println!("{:?}", bootinfo.memory_map[index]);
    }
    let page_table = unsafe { &mut *(bootinfo.p4_table_addr as *mut PageTable) };
    page_table[2].set_addr(PhysAddr::new(bootinfo.memory_map[12].range.start_addr()), PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    for i in 0..=2 {
        unsafe {
            let pt = &*(bootinfo.p4_table_addr as *mut PageTable);
            println!("{:?}", pt[i]);
        }
    }
    unsafe {
        let pt = &*(bootinfo.p4_table_addr as *mut PageTable);
        println!("{:?}", pt[511]);
    }
    let addr = VirtAddr::new(bootinfo.p4_table_addr & 0x1FFFFFFFFFF | 0x03);
    println!("{:?}/{:?}/{:?}/{:?}/{:?}", addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index(), addr.page_offset());
    let addr = VirtAddr::new(bootinfo.p4_table_addr);
    println!("{:?}/{:?}/{:?}/{:?}/{:?}", addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index(), addr.page_offset());
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
