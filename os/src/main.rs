// learn about procedural macros
// file descriptor provided by the operating system.
// understanding how the error handling works in rust

#![no_std]
#![no_main]
#![feature(breakpoint)]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)] // test_runner is entry point after it reaches test_main
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

// lazy_static! {
//     static ref COUNT: Mutex<i32> = Mutex::new(0);
// }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    os::init();

    // let ptr: *mut u8 = 0x0032F as *mut u8;
    // unsafe {
    //     *ptr = 10;
    // }

    fn recursion(c: Option<i32>) {
        // println!("{}", c);
        // c += 1;
        // os::delay(1);
        recursion(None);
    }

    recursion(None);
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

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
