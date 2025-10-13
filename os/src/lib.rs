#![feature(abi_x86_interrupt)]
#![feature(breakpoint)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "main_test"]
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub use crate::serial::_print;
use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

// basically loops for a random memory location and screams me read withtout the compiler
// optimization take it away well this idea gives me page fault so big no

/// Let x86 asm code do the cpu burning thing for us
pub fn delay(mut count: u64) {
    // let ptr: *mut u8 = core::ptr::null_mut();
    unsafe {
        core::arch::asm!(
            "2:",
            "sub {0}, 1",
            "jnz 2b",
            inout(reg) count,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0xAB, // 87
    Failed = 0xBA,  // 117
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port: Port<u32> = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self);
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    main_test();
    loop {} // safely inside the kernel runtime and doesnot collect garbage from the memory
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
