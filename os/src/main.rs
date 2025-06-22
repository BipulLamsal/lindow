// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust
//
//

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "main_test"]
use crate::serial::*;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use vga_buffer::WRITER;
mod serial;
mod vga_buffer;

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

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(test)]
    main_test();
    helloworld();
    loop {
        // panic!("This is the example of an error")
    }
}

fn helloworld() {
    for u in 0..25 {
        println!("Hello world {}!", u);
        for _ in 0..10000000 {
            unsafe {
                asm!("NOP");
            }
        }
    }
    println!("My name is Bipul Lamsal.");
    println!("Fuck this shit is going crazy");
    println!("Fuck this shit is going crazy");
    println!("Fuck this shit is going crazy");
}

/*
 * Occupies two bytes : One for character itself another for attribute byte
 * Bit  76543210
        ||||||||
        |||||^^^-fore colour
        ||||^----fore colour bright bit
        |^^^-----back colour
        ^--------back colour bright bit OR enables blinking Text
*
* */

// fn write_character(text: &[u8], forecolor: u8, backcolor: u8, x: u8, y: u8, blink: bool) {
//     unsafe {
//         for (i, &byte) in text.iter().enumerate() {
//             let index = ((y as isize * 80) + (x as isize + i as isize)) * 2;
//             *VGA_BUFFER.offset(index) = byte;
//             let mut attribute = (backcolor << 4) | (forecolor & 0x0F);
//             if blink {
//                 attribute |= 0x80;
//             }
//             *VGA_BUFFER.offset(index + 1) = attribute;
//         }
//     }
// }
//
//

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut w = WRITER.lock();
    w.terminate();
    write!(w, "{}", info).unwrap();
    loop {}
}

// our panic handler in test mode
////////////////////////////////
//                            //
//                            //
//                            //
////////////////////////////////
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
