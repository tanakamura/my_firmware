#![no_main]
#![no_std]

extern crate common;

#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> i32 {
    monitor::console_loop();
    0
}
