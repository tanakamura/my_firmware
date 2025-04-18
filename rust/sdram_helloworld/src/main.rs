#![no_main]
#![no_std]

use x86::time::rdtsc;

extern crate alloc;
//use alloc::vec::Vec;
use common::println;
use common::uart;

//fn now() -> f64 {
//    let t = unsafe { rdtsc() };
//    t as f64 / 1.6e9
//}

#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> i32 {
    println!("Hello, world! from sdram_helloworld");

    common::common_init_from_sdram();
    //
    //    let mut v = Vec::new();
    //    v.push(1);
    //    println!("v = {:?}", v.as_mut_ptr());

    0
}
