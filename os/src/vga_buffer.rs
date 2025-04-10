use core::{fmt::Write, usize};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        let attribute = ((background as u8) << 4) | ((foreground as u8) & 0x0F);
        ColorCode(attribute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
#[allow(dead_code)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
    blink: bool,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
impl WriterBuilder {
    fn column_position(&mut self, value: usize) -> &mut Self {
        self.column_position = value;
        self
    }
    fn row_position(&mut self, value: usize) -> &mut Self {
        self.row_position = value;
        self
    }
    fn color_code(&mut self, value: ColorCode) -> &mut Self {
        self.color_code = value;
        self
    }
    fn buffer(&mut self, value: &'static mut Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
    fn build(&'static mut self) -> Writer {
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
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
        }
    }
}
impl WriterBuilder {}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[self.row_position][self.column_position] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                    blink: false,
                };
                self.column_position += 1;
            }
        }
    }
    pub fn new_line(&mut self) {
        self.row_position += 1;
        self.row_position %= BUFFER_HEIGHT;
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // since vga buffer doesn't accept utf-8 so actually we are making sure that our bytes
            // are the acutally valid code page 437
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}
