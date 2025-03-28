#![no_main]
#![no_std]

extern crate common;
extern crate init86;

#[unsafe(no_mangle)]
pub extern "C" fn rmain() -> ! {
    monitor::console_loop();
    loop {}
}
