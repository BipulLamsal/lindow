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
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use vga_buffer::WRITER;
mod vga_buffer;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test()
    }
}

// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(test)]
    main_test();
    loop {
        // helloworld();
        // panic!("This is the example of an error")
    }
}
fn helloworld() {
    for _ in 0..1000 {
        unsafe {
            asm!("NOP");
        }
    }
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
    println!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut w = WRITER.lock();
    w.terminate();
    write!(w, "{}", info).unwrap();
    loop {}
}
