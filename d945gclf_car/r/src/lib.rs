#![no_std]

use core::panic::PanicInfo;
use x86::io::{outb,inb};

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const UART_BASE:u16 = 0x3f8;
const UART_DATA:u16 = UART_BASE;
const UART_LSR:u16 = UART_BASE+5;

fn putchar(c:u8) {
    loop {
        unsafe {
            if (inb(UART_LSR) & (1 << 5)) != 0 { /* (1<<5) : THRE */
                outb(UART_DATA, c);
                return;
            }
        }
    }
}

fn puts(s:&str) {
    for c in s.bytes() {
        putchar(c as u8);
    }
    putchar(b'\r');
    putchar(b'\n');
}

fn print_hex(v:u32) {
    let table = b"0123456789ABCDEF";
    for i in 0..8 {
        putchar(table[((v>>((7-i)*4))&0xf) as usize]);
    }
    putchar(b'\r');
    putchar(b'\n');
}

#[no_mangle]
pub unsafe extern "C" fn rmain() {
    puts("Hello, World from Rust!!!!!! Hello!!!!!!");
    let ptr = 0xc0000 as *mut u32;
    let stack_size = 16*1024;
    let fill_size = (20*1024-stack_size)/4;
    let fill_range = core::slice::from_raw_parts_mut(ptr, fill_size);
    for v in fill_range.into_iter().enumerate() {
        *v.1 = v.0 as u32;
    }
    for v in fill_range.into_iter().enumerate() {
        if *v.1 != v.0 as u32 {
            puts("fail");
            print_hex(v.0 as u32);
            print_hex(*v.1);
            return;
        }
    }
    puts("OK!");
}

