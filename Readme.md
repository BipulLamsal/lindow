# Lindow OS

 [![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
 [![Architecture](https://img.shields.io/badge/arch-x86__64-blue.svg)](https://en.wikipedia.org/wiki/X86-64)
 [![Status](https://img.shields.io/badge/status-in_progress-yellow.svg)](#roadmap)

 A bare-metal operating system written from scratch in Rust for the `x86_64` architecture. This is a learning project designed to explore low-level systems programming, OS development concepts, and Rust's `no_std` environment.

 ## Roadmap

 ### Completed
 - [x] Basic VGA text buffer output
 - [x] Serial port for debugging
 - [x] CPU exception handling (IDT)
 - [x] GDT & TSS for double fault handling
 ### Planned
 - [ ] Keyboard input (PS/2)
 - [ ] Memory paging and virtual memory
 - [ ] Heap memory allocator
 - [ ] Preemptive multitasking and scheduling
 - [ ] A basic file system
 - [ ] A simple shell interface

 ## Architecture

 The diagram below shows the high-level structure of Lindow OS running within the QEMU emulator.

 ```
 +---------------------------------------------+
 |              Lindow OS (Rust)               |
 |                  (no_std)                   |
 +---------------------------------------------+
 |   VGA Buffer | Serial I/O |   Interrupts    |
 |   (Display)  |   (UART)   |      (IDT)      |
 +--------------+------------+-----------------+
 |         GDT & TSS (Memory Safety)           |
 +---------------------------------------------+
 |              Bootloader (v0.9)              |
 +---------------------------------------------+
 |                    QEMU                     |
 |          (x86_64 Emulator/VM)               |
 +---------------------------------------------+
 ```

 ## Core Components

 - **VGA Buffer** (`src/vga_buffer.rs`): A simple driver for writing to the text-mode display. The buffer is memory-mapped at `0xb8000`.
 - **Serial Port** (`src/serial.rs`): Implements basic I/O for a UART 16550 serial port. All `println!` macros are redirected here, allowing debug output to be viewed on the host terminal.
 - **Interrupts** (`src/interrupts.rs`): Sets up the Interrupt Descriptor Table (IDT) and defines handlers for CPU exceptions like breakpoints, page faults, and double faults.
 - **GDT/TSS** (`src/gdt.rs`): Configures the Global Descriptor Table (GDT) and a Task State Segment (TSS). This is crucial for providing a separate, known-good stack for the double fault handler to prevent a triple fault.

 ## Building and Running

 ### Prerequisites

 First, install the necessary Rust target and the `bootimage` tool.

 ```sh
 # Add the target for building a bare-metal x86_64 OS
 rustup target add x86_64-unknown-none

 # Install the bootimage tool to create a bootable disk image
 cargo install bootimage
 ```

 ### Run the OS

 Once the prerequisites are installed, you can build and run the operating system in QEMU with a single command from the project root.

 ```sh
 cargo run
 ```

 Alternatively, you can use the provided helper script:

 ```sh
 ./run.sh
 ```

 ## Documentation

 For more detailed technical notes on x86 architecture, interrupts, memory management, and other OS concepts explored in this project, please see the `docs/` directory.
