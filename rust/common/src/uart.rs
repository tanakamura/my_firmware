use x86::io::{inb, outb};

use core::fmt;
use core::result::Result::Ok;

pub struct UartWriter;

const UART_BASE: u16 = 0x3f8;
const UART_DATA: u16 = UART_BASE;
const UART_LSR: u16 = UART_BASE + 5;

pub fn putchar(c: u8) {
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

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            putchar(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (core::fmt::write(&mut uart::UartWriter, format_args!($($arg)*)).unwrap());
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}
