// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust
//
//
#![no_std]
#![no_main]
use core::panic::PanicInfo;

// define bare metal main function to run within os runtime
#[no_mangle]
extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
