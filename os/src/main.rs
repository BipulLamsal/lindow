// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust
//
#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;
const SCREEN_SIZE: usize = 80 * 50;

mod vga_buffer;

// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    // starting buffer ffor vga of x86 systems and creating a raw mutable pointer
    let text = b"BIPUL LAMSAL!";
    let x = 0;

    loop {
        write_character(text, 0, 10, x + 1, 10, true);
        // x = (x + 1) % 70;
        for _ in 0..1000000 {
            unsafe {
                asm!("NOP");
            }
        }
        clear_screen(vga_buffer::Color::Red);
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

fn write_character(text: &[u8], forecolor: u8, backcolor: u8, x: u8, y: u8, blink: bool) {
    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let index = ((y as isize * 80) + (x as isize + i as isize)) * 2;
            *VGA_BUFFER.offset(index) = byte;
            let mut attribute = (backcolor << 4) | (forecolor & 0x0F);
            if blink {
                attribute |= 0x80;
            }
            *VGA_BUFFER.offset(index + 1) = attribute;
        }
    }
}

fn clear_screen(background: vga_buffer::Color) {
    unsafe {
        for i in 0..(SCREEN_SIZE) {
            let char_offset = i * 2;
            let attr_offset = char_offset + 1;
            *VGA_BUFFER.add(char_offset) = b' ';
            *VGA_BUFFER.add(attr_offset) = (background as u8) << 4;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
