use pic8259_simple::ChainedPics;
use spin;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use x86_64::structures::idt::PageFaultErrorCode;

use gdt;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

lazy_static! {
    static ref timer_count: spin::Mutex<usize> = spin::Mutex::new(0usize);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    let mut timer = timer_count.lock();
    *timer += 1;
    if *timer % 1000 == 0 {
        print!(".");
    }
    unsafe {
        PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    print!("{} ", scancode);
    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1;
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

fn set_clock_speed(frequency: u16) {
    const STANDARD_FREQUENCY: usize = 1_193_182;
    let mut port1: Port<u8> = Port::new(0x43);
    let mut port2: Port<u8> = Port::new(0x40);
    let speed_rate: usize = STANDARD_FREQUENCY / frequency as usize;
    assert!(speed_rate <= 65536usize);
    let bytes = speed_rate.to_le_bytes();
    unsafe {
        port1.write(0x34u8);
        port2.write(bytes[0]);
        port2.write(bytes[1]);
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
          idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);
        idt[usize::from(KEYBOARD_INTERRUPT_ID)].set_handler_fn(keyboard_interrupt_handler);
        set_clock_speed(1000);
        idt
    };
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
