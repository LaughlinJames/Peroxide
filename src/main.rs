#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

// #[test_case]
// fn failed_assertion(){
//     serial_print!("failing assertion... ");
//     assert_eq!(1,2);
//     serial_println!("[ok]");
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
