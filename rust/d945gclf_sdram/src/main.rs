#![no_main]
#![no_std]

use core::panic::PanicInfo;
use x86::io::{inb, outb};

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const UART_BASE: u16 = 0x3f8;
const UART_DATA: u16 = UART_BASE;
const UART_LSR: u16 = UART_BASE + 5;

fn uart_get8() -> u8 {
    loop {
        unsafe {
            if (inb(UART_LSR) & (1 << 0)) != 0 {
                return inb(UART_DATA);
            }
        }
    }
}
fn uart_get32() -> u32 {
    let mut v: u32 = 0;
    v |= uart_get8() as u32;
    v |= (uart_get8() as u32) << 8;
    v |= (uart_get8() as u32) << 16;
    v |= (uart_get8() as u32) << 24;
    return v;
}

fn putchar(c: u8) {
    loop {
        unsafe {
            if (inb(UART_LSR) & (1 << 5)) != 0 {
                /* (1<<5) : THRE */
                outb(UART_DATA, c);
                return;
            }
        }
    }
}

fn uart_puts(s: &str) {
    for c in s.bytes() {
        putchar(c as u8);
    }
    putchar(b'\r');
    putchar(b'\n');
}

pub extern "C" fn keep_syms() {
    init86::keep_syms();
}

#[unsafe(no_mangle)]
pub extern "C" fn rmain() -> ! {
    loop {
        uart_puts("Hello from SDRAM!!!");
        uart_puts(
            "Please send BINARY file from UART. files are loaded at 0x10000000 and jump to it.",
        );

        let len = uart_get32();
        let sum8 = uart_get8();
        let mut recv_sum8: u8 = 0;

        for i in 0..len {
            let c = uart_get8();
            unsafe {
                *(0x10000000 as *mut u8).offset(i as isize) = c;
            }
            recv_sum8 = recv_sum8 ^ c;
        }

        if sum8 != recv_sum8 {
            uart_puts("Checksum error");
        } else {
            uart_puts("Checksum OK");
            let entry_fn: extern "C" fn() -> i32 = unsafe { core::mem::transmute(0x10000000) };
            let _ret = entry_fn();
            uart_puts("Program returned");
        }
    }
}
