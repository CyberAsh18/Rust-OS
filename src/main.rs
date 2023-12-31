#![no_std]
#![no_main]

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point, the linker looks for a function called '_start' by default
    loop {}
}

