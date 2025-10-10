// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust

#![no_std]
#![no_main]
#![feature(breakpoint)]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)] // test_runner is entry point after it reaches test_main
#![reexport_test_harness_main = "test_main"]

use core::{arch::breakpoint, panic::PanicInfo};
use os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    os::init();
    breakpoint();
    #[cfg(test)]
    test_main();
    println!("I can still see this, no error!");

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
