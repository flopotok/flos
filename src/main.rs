#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub fn function_that_requires_std() {
    // optionally enable std
    std::println!("standard library enabled");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// no c -> rust runtime to call main so we need to define our own entry point
// extern "C" -> use C calling convention
// _start -> entry point, conventional name for most systems
// -> ! -> diverging function, never returns because it will be called by bootloader directly
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}