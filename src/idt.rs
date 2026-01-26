use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::structures::idt::PageFaultErrorCode;
use crate::gdt;
use pic8259::ChainedPics;
use spin::{Mutex, Lazy};

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

use core::sync::atomic::{AtomicU64, Ordering};
pub static TICKS: AtomicU64 = AtomicU64::new(0);

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Irq {
    Timer = PIC1_OFFSET,
    Keyboard,
}

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint);
    unsafe {
        idt.double_fault.set_handler_fn(double_fault)
            .set_stack_index(gdt::DOUBLE_FAULT_IST);
    }
    idt.page_fault.set_handler_fn(page_fault);
    idt[Irq::Timer as usize].set_handler_fn(timer);
    idt[Irq::Keyboard as usize].set_handler_fn(keyboard);
    idt
});

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint(frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame);
}

extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", frame);
}

extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", code);
    println!("{:#?}", frame);
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn timer(_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    unsafe {
        PICS.lock().notify_end_of_interrupt(Irq::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard(_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(Irq::Keyboard as u8);
    }
}

pub fn init_pics() {
    unsafe { PICS.lock().initialize(); }
    unsafe { PICS.lock().write_masks(0xfc, 0xff); }
}
