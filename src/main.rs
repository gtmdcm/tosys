#![no_std]
#![no_main]

#[macro_use]
extern crate bootloader_precompiled;

use bootloader_precompiled::bootinfo;
use core::panic::PanicInfo;
use memory::free_page;
use memory::pages_available;

mod drive;
mod memory;

fn main(bootinfo: &'static bootinfo::BootInfo) -> ! {
    const MEMORY_REGION_TYPE: [&[u8; 11]; 14] = [
        b"Usable     ",
        b"InUse      ",
        b"Reserved   ",
        b"??         ",
        b"??         ",
        b"bad        ",
        b"kernel     ",
        b"KernelStack",
        b"PageTable  ",
        b"Bootloader ",
        b"FrameZero  ",
        b"Empty      ",
        b"BootInfo   ",
        b"Package    "
    ];
    let mut vga: drive::vga::Vga = drive::vga::Vga::new();
    vga.print_uint(bootinfo.memory_map.len() as u64)
        .println(b"");
    for i in 0..=14 {
        vga.print_uint(i)
            .print_char(b':')
            .print(MEMORY_REGION_TYPE[bootinfo.memory_map[i as usize].region_type as usize])
            .print_char(b',')
            .print_uint(bootinfo.memory_map[i as usize].range.start_frame_number as u64)
            .print(b" to ")
            .print_uint(bootinfo.memory_map[i as usize].range.end_frame_number as u64)
            .println(b"");
    }
    for i in 0..10000000 {}
    unsafe {
        vga.print_uint(*((1048584 * 4096) as *mut u64));
        for i in 0..10000000 {}
    }
//    free_page((1069) as *mut u8);
//    vga.print_uint((1070 * 4096) as *mut u8 as u64);
    loop {}
}

entry_point!(main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
