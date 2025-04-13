use core::{
    char,
    fmt::{Arguments, Result, Write},
};

// TODO
// 1. scrolling screen
// 2. maybe println and print macro [x]
// 3. clearing the whole screen [x]
// 4. clearning the whole row [X]
// 5. making the last row as terminal style warning (maybe i need to seperate thee writer and
//    different eol modes but for now it is okay to have only one kind)

use spin::lazy::Lazy;
use spin::Mutex;
use volatile::Volatile;

//
// mut static is unsafe so mutex would be better soultion here
pub static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| Mutex::new(WriterBuilder::default().build()));
pub static EOLWRITER: Lazy<Mutex<Writer>> = Lazy::new(|| {
    let writer = WriterBuilder::default()
        .row_position(BUFFER_HEIGHT - 1)
        .color_code(ColorCode::new(Color::White, Color::Red))
        .build();
    Mutex::new(writer)
});

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

impl ScreenChar {
    fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code,
        }
    }
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
    pub fn column_position(mut self, value: usize) -> Self {
        self.column_position = value;
        self
    }
    pub fn row_position(mut self, value: usize) -> Self {
        self.row_position = value;
        self
    }
    pub fn color_code(mut self, value: ColorCode) -> Self {
        self.color_code = value;
        self
    }
    pub fn buffer(mut self, value: &'static mut Buffer) -> Self {
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
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // scroll  everythign up so row starts at 1 because row 1 is copied to row 0
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            // clear the last row
            self.clear_current_row();
        }
        self.column_position = 0;
    }
    fn clear_current_row(&mut self) {
        self.clear_row(self.row_position);
    }
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        if row >= BUFFER_HEIGHT {
            return; // invalid row, do nothing
        }

        let blank_value = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank_value);
        }
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

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
