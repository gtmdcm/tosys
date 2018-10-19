#![no_std]
#![no_main]

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

mod drive;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut dummy = 0;
    for j in 0..10000 {
        dummy ^= j;
    }
    drive::vga::VGA.print_int(-123456);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
