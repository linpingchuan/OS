#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hello::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hello::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello 林平川 {}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hello::test_panic_handler(info)
}

