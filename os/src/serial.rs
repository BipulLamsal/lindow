use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

pub static SERIAL_COM_1: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
    const SERIAL_IO_PORT: u16 = 0x3F8;
    let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    serial_port.init();
    Mutex::new(serial_port)
});

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n");
    };
    ($fmt:expr) => {
        $crate::serial_print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*);
    };
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_COM_1
        .lock()
        .write_fmt(args)
        .expect("Serial I/O failed to write");
}
