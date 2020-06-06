#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(peroxide::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use peroxide::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    peroxide::init();

    // fn stack_overflow(){
    //     stack_overflow();
    // }
    //
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

// This function is called on panic when not running tests.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop{}
}

// This function is called on panic when running tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    peroxide::test_panic_handler(info);
}
