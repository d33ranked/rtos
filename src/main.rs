#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rtos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rtos::println;





#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rtos::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }



    

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rtos::hlt_loop();
    
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rtos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rtos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}







