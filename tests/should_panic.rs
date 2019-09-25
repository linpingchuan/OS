#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hello::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    hello::serial_println!("[test did not panic]");
    hello::exit_qemu(hello::QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hello::serial_println!("[ok]");
    hello::exit_qemu(hello::QemuExitCode::Success);
    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    hello::serial_println!("Running {} tests",tests.len());
    for test in tests{
        test();
        hello::serial_println!("[test did not panic]");
        hello::exit_qemu(hello::QemuExitCode::Failed);
    }
    hello::exit_qemu(hello::QemuExitCode::Success);
}

fn should_fail(){
    hello::serial_println!("should_fail...");
    assert_eq!(0,1 )
}