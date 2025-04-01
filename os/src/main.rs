// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust
//
#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;
use core::usize;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const SCREEN_SIZE: usize = 80 * 50;
// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    // starting buffer ffor vga of x86 systems and creating a raw mutable pointer
    let text = b"Anupama!";
    let mut x = 0;

    loop {
        write_character(text, 0, 10, x + 1, 10);
        x = (x + 1) % 70;
        for _ in 0..1000000 {
            unsafe {
                asm!("NOP");
            }
        }
        clear_screen(2);
    }
}

/*
 *
 * Occupies two bytes : One for character itself another for attribute byte
 * Bit  76543210
        ||||||||
        |||||^^^-fore colour
        ||||^----fore colour bright bit
        |^^^-----back colour
        ^--------back colour bright bit OR enables blinking Text
*
* */

fn write_character(text: &[u8], forecolor: u8, backcolor: u8, x: u8, y: u8) {
    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let index = ((y as isize * 80) + (x as isize + i as isize)) * 2;
            *VGA_BUFFER.offset(index) = byte;
            *VGA_BUFFER.offset(index + 1) = (backcolor << 4) | (forecolor & 0x0F);
        }
    }
}

pub fn clear_screen(background: u8) {
    let attribute = (background << 4) | 0x07;
    let blank_char = b' ';

    unsafe {
        for i in 0..SCREEN_SIZE {
            *VGA_BUFFER.add(i) = blank_char;
            *VGA_BUFFER.offset(i as isize + 1) = attribute;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
