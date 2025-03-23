#![no_main]
#![no_std]

use x86::time::rdtsc;

use common::uart;
use common::println;

fn now() -> f64 {
    let t = unsafe { rdtsc() };
    t as f64 / 1.6e9
}

#[link_section = ".text.start"]
#[no_mangle]
extern "C" fn _start() -> i32 {
    for _ in 0..5 {
        unsafe {
            let length = 1024 * 1024 * 4;
            let mem_begin = 0x20000000 as *mut u32;
            let mem_end = (0x20000000 + length) as *mut u32;

            let mem_begin2 = mem_end;
            //let mem_end2 = (mem_begin2 as usize + length) as *mut u32;
            //let nword = length / 4;

            let t0 = now();
            core::intrinsics::write_bytes(mem_begin as *mut u8, 0, length);
            let t1 = now();

            let d = t1 - t0;
            let bytes_per_sec = (length as f64) / d;
            //println!("length={} time={}s", length, d);
            println!("memset {}[MiB/s]", bytes_per_sec / (1024.0 * 1024.0));

            let t0 = now();
            core::intrinsics::copy(mem_begin as *const u8, mem_begin2 as *mut u8, length);
            let t1 = now();

            let d = t1 - t0;
            let bytes_per_sec = ((length*2) as f64) / d;
            //println!("length={} time={}s", length, d);
            println!("memcpy {}[MiB/s]", bytes_per_sec / (1024.0 * 1024.0));
        }
    }

    0
}
