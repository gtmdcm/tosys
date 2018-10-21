#![no_std]
#![no_main]

#[macro_use]
extern crate bootloader_precompiled;

use bootloader_precompiled::bootinfo;
use core::panic::PanicInfo;

mod drive;

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
    for i in 0...14 {
        drive::vga::VGA
            .print_uint(i)
            .print_char(b':')
            .print(MEMORY_REGION_TYPE[bootinfo.memory_map[i as usize].region_type as usize])
            .print_char(b',')
            .print_uint(bootinfo.memory_map[i as usize].range.start_frame_number as u64)
            .print_char(b't')
            .print_uint(bootinfo.memory_map[i as usize].range.end_frame_number as u64);
    }
    loop {}
}

entry_point!(main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
