#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel logic
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
