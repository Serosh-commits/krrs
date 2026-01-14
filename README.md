# krrs - A Minimal Rust Kernel

This is a hobby project where I'm building an x86_64 operating system kernel from scratch using Rust. No standard library, no existing OS underneath—just bare metal and raw code.

The goal here was to learn how things actually work at the lowest level: memory mapping, interrupt handling, and talking to hardware directly.

## Current State

It's currently a functional minimal kernel that boots and drops you into a basic interactive shell.

- **VGA Text Mode Driver**: Custom implementation for screen output (supporting `print!` and `println!`).
- **Interrupts**: Full IDT setup. Handles hardware interrupts (timer, keyboard) and exceptions (double faults, page faults).
- **GDT**: Sets up a Global Descriptor Table and a Task State Segment (TSS) for safe exception handling.
- **Memory Management**:
  - Sets up paging and recursive mapping.
  - Custom `BootInfoFrameAllocator` to manage physical memory frames.
- **Heap Allocator**: Integrated `linked_list_allocator` so I can use dynamic types like `Box`, `Vec`, and `String` in kernel land.
- **Shell**: A simple interactive shell that processes keyboard input and executes basic commands (`help`, `ver`, `cls`).

## Prerequisites

You'll need a couple of things to get this building:

1. **Rust Nightly**: Since I'm using a bunch of unstable features.
   ```bash
   rustup default nightly
   rustup component add rust-src llvm-tools-preview
   ```
2. **Bootimage tool**: For turning the kernel bin into a bootable disk image.
   ```bash
   cargo install bootimage
   ```
3. **QEMU**: To actually run the thing.

## How to Run

Just clone the repo and run:

```bash
cargo run
```

This will build the kernel, use `bootimage` to create the bootable `.bin`, and launch it in QEMU.

## Project Structure

- `src/main.rs`: The kernel entry point.
- `src/vga.rs`: VGA text buffer driver.
- `src/gdt.rs`: Global Descriptor Table & TSS.
- `src/idt.rs`: Interrupt Descriptor Table.
- `src/memory.rs`: Paging and frame allocation.
- `src/heap.rs`: Heap allocation setup.
- `src/keyboard.rs`: Keyboard scancode processing.
- `src/shell.rs`: The interactive shell logic.

## Credits/References

Mostly following the "Writing an OS in Rust" series by Philipp Oppermann as a starting point and tweaking things from there.
