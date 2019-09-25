#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hello::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    
}

#[panic_handler]
fn painc(info: &PanicInfo) -> ! {
    hello::test_panic_handler(info);
}

#[test_case]
fn test_println(){
    hello::serial_print!("test_println...");
    hello::println!("test_println output");
    hello::serial_println!("[ok]");
}