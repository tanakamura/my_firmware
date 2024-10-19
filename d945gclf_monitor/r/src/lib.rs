#![no_std]

use core::panic::PanicInfo;
mod console;

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rmain() {
    console::console_loop();
}

pub use crate::console::runbin;
