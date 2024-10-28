#![no_main]
#![no_std]

use x86::controlregs::{Cr0, Cr4};
use x86::msr::{rdmsr, wrmsr};
use x86::time::rdtsc;

mod uart;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn now() -> f64 {
    let t = unsafe { rdtsc() };
    t as f64 / 1.6e9
}

#[link_section = ".text.start"]
#[no_mangle]
extern "C" fn _start() -> i32 {
    // enable sse
    let mut cr0 = unsafe { x86::controlregs::cr0() };
    cr0 = cr0 - Cr0::CR0_EMULATE_COPROCESSOR;
    cr0 = cr0 | Cr0::CR0_MONITOR_COPROCESSOR;
    unsafe {
        x86::controlregs::cr0_write(cr0);
    }

    let mut cr4 = unsafe { x86::controlregs::cr4() };
    cr4 = cr4 | Cr4::CR4_ENABLE_SSE | Cr4::CR4_UNMASKED_SSE;
    unsafe {
        x86::controlregs::cr4_write(cr4);
    }

    for i in 0..5 {
        unsafe {
            let t0 = now();

            let mem_begin = 0x20000000 as *mut u32;
            let mem_end = (0x20000000 + 1024 * 1024 * 8) as *mut u32;

            let nword = (mem_end as usize - mem_begin as usize) / 4;
            let slice = core::slice::from_raw_parts_mut(mem_begin, nword);

            for i in 0..nword {
                slice[i] = i as u32;
            }

            let t1 = now();

            let d = t1 - t0;
            let bytes_per_sec = ((nword * 4) as f64) / d;
            println!("memset {}[MiB/s]", bytes_per_sec / (1024.0 * 1024.0));

            let t0 = now();
            for i in 0..nword {
                if slice[i] != (i as u32) {
                    println!("error: slice[{}] = {}", i, slice[i]);
                }
            }
            let t1 = now();
            let d = t1 - t0;

            let bytes_per_sec = ((nword * 4) as f64) / d;
            println!("verify {}[MiB/s]", bytes_per_sec / (1024.0 * 1024.0));
        }
    }

    for i in 0..2 {
        unsafe {
            let mtrr0_val = rdmsr(0x200 + i * 2);
            let mtrr0_mask = rdmsr(0x201 + i * 2);

            println!("mtrr{}: {:x} {:x}", i, mtrr0_val, mtrr0_mask);
        }
    }

//    unsafe {
//        wrmsr(0x200 + 0 * 2, 0);
//        wrmsr(0x201 + 1 * 2, 0);
//
//        /* enable flash cache */
//        wrmsr(0x200 + 1 * 2, 0xfffc0006);
//        wrmsr(0x201 + 1 * 2, 0xfffc0000 | (1 << 11));
//    }

    0
}
