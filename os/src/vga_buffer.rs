use core::{
    char,
    fmt::{Arguments, Result, Write},
};

// TODO
// 1. scrolling screen
// 2. maybe println and print macro

use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// the size of color is 8 bits
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
// the color code accepts 8 bits value
pub struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        let attribute = ((background as u8) << 4) | ((foreground as u8) & 0x0F);
        ColorCode(attribute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[allow(dead_code)]
// screen char is exactly 16 bits value with the ascii value itself and the property for now the
// blink is missing
// #[repr(C)] guarantees that ascii_character and color_code are laid out exactly in memory in that order, 1 byte each.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
// transparent accepts only one value and acts a wrapper so if multiple value needs to be
// represnted use repr c
#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    // 2D array
}

#[allow(dead_code)]
pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

#[allow(dead_code)]
pub struct WriterBuilder {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

#[allow(dead_code)]
impl WriterBuilder {
    pub fn column_position(&mut self, value: usize) -> &mut Self {
        self.column_position = value;
        self
    }
    pub fn row_position(&mut self, value: usize) -> &mut Self {
        self.row_position = value;
        self
    }
    pub fn color_code(&mut self, value: ColorCode) -> &mut Self {
        self.color_code = value;
        self
    }
    pub fn buffer(&mut self, value: &'static mut Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
    pub fn build(self) -> Writer {
        Writer {
            column_position: self.column_position,
            row_position: self.row_position,
            color_code: self.color_code,
            buffer: self.buffer,
        }
    }
}
impl Default for WriterBuilder {
    fn default() -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            // let me mutate the thing at that address.
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
        }
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            b'\r' => {
                self.carriage_return();
            }
            b'\t' => {
                self.horizontal_tab();
            }

            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn carriage_return(&mut self) {
        self.column_position = 0;
    }
    fn horizontal_tab(&mut self) {
        let tab_size = 8;
        if (self.column_position + tab_size) >= BUFFER_WIDTH {
            self.new_line();
        } else {
            self.column_position += tab_size;
        }
    }
    fn new_line(&mut self) {
        self.row_position += 1;
        self.row_position %= BUFFER_HEIGHT;
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // since vga buffer doesn't accept utf-8 so actually we are making sure that our bytes
            // are the acutally valid code page 437
            match byte {
                0x20..=0x7e | b'\n' | b'\t' | b'\r' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    // pub fn clean(&mut self) {
    // }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
    fn write_char(&mut self, c: char) -> Result {
        // utf-8 rust implementation has characters more than 1 byte
        if c.is_ascii() {
            self.write_byte(c as u8);
        } else {
            self.write_byte(0xfe);
        }
        Ok(())
    }
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        core::fmt::write(self, args)
    }
}
