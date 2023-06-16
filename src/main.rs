// src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(smite::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use smite::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    smite::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "Hello, world! / some numbers: {i} {f}...",
        i = 42,
        f = 1.337
    );

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
