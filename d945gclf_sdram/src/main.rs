#![no_main]
#![no_std]

use core::panic::PanicInfo;
use x86::io::{inb, outb};

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

fn uart_puts(s:&str) {
    for c in s.bytes() {
        putchar(c as u8);
    }
    putchar(b'\r');
    putchar(b'\n');
}



#[no_mangle]
pub extern "C" fn rmain() -> ! {
    uart_puts("Hello from SDRAM!!!");
    loop {}
}
