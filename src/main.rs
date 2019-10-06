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

    hello::init();

    // x86_64::instructions::interrupts::int3();
    // trigger a page fault
    unsafe{
        *(0xdeadbeef as *mut u64)=42;
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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

