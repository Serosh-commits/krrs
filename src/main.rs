#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod gdt;
mod idt;
mod pic;
mod keyboard;
mod memory;
mod heap;
mod shell;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    vga::init();
    println!("Rust kernel booted");

    gdt::init();
    idt::init();
    pic::init();
    x86_64::instructions::interrupts::enable();

    let phys_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_offset) };
    let mut frame_alloc = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };

    heap::init(&mut mapper, &mut frame_alloc).expect("heap init failed");

    let boxed = alloc::boxed::Box::new([0u8; 4096]);
    println!("heap test: {:p}", boxed.as_ptr());

    let mut shell = shell::Shell::new();
    print!("> ");

    loop {
        use x86_64::instructions::{hlt, interrupts};
        
        interrupts::disable();
        let key = keyboard::pop_key();
        interrupts::enable();

        if let Some(c) = key {
            shell.input(c);
        } else {
            hlt();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error(layout: alloc::alloc::Layout) -> ! {
    println!("[ALLOC ERROR] {:?}", layout);
    loop {
        x86_64::instructions::hlt();
    }
}
