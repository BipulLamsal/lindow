// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust
//
#![no_std]
#![no_main]
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;

mod vga_buffer;
use crate::vga_buffer::*;

// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    loop {
        for _ in 0..1000000 {
            unsafe {
                helloworld();
                asm!("NOP");
            }
        }
    }
}

fn helloworld() {
    let mut writer = WriterBuilder::default().build();
    write!(writer, "Hello {}!", "world").unwrap();
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
