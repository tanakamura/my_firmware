#![no_std]

use core::panic::PanicInfo;
use x86::io::{inb, outb};

const UART_BASE: u16 = 0x3f8;
const UART_DATA: u16 = UART_BASE;
const UART_LSR: u16 = UART_BASE + 5;

const SMBUS_DEV: u8 = 31;
const SMBUS_FUNC: u8 = 3;
const SMBASE: u16 = 0x1000;

const HST_STS: u16 = 0x00;
const HST_CNT: u16 = 0x02;
const HST_CMD: u16 = 0x03;
const XMIT_SLVA: u16 = 0x04;
const HST_D0: u16 = 0x05;

const HST_STS_BUS_ERR: u8 = 1 << 3;
const HST_STS_DEV_ERR: u8 = 1 << 2;
const HST_STS_INTR: u8 = 1 << 1;
const HST_STS_BUSY: u8 = 1 << 0;

fn uart_put8(c: u8) {
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
        uart_put8(c);
    }
}

fn uart_put_hex8(n: u8) {
    let hex = b"0123456789abcdef";
    uart_put8(hex[(n >> 4) as usize]);
    uart_put8(hex[(n & 0xf) as usize]);
}

fn uart_put_hex32(n: u32) {
    uart_put_hex8((n >> 24) as u8);
    uart_put_hex8((n >> 16) as u8);
    uart_put_hex8((n >> 8) as u8);
    uart_put_hex8(n as u8);
}

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const PCIEXBAR: u32 = 0xf000_0000;

fn pci_config_write8(bus: u8, dev: u8, func: u8, offset: u8, data: u8) {
    let address = PCIEXBAR
        | ((bus as u32) << 20)
        | ((dev as u32) << 15)
        | ((func as u32) << 12)
        | (offset as u32);
    unsafe {
        core::ptr::write_volatile(address as *mut u8, data);
    }
}

fn pci_config_read8(bus: u8, dev: u8, func: u8, offset: u8) -> u8 {
    let address = PCIEXBAR
        | ((bus as u32) << 20)
        | ((dev as u32) << 15)
        | ((func as u32) << 12)
        | (offset as u32);
    unsafe { core::ptr::read_volatile(address as *mut u8) }
}

fn pci_config_write16(bus: u8, dev: u8, func: u8, offset: u8, data: u16) {
    let address = PCIEXBAR
        | ((bus as u32) << 20)
        | ((dev as u32) << 15)
        | ((func as u32) << 12)
        | (offset as u32);
    unsafe {
        core::ptr::write_volatile(address as *mut u16, data as u16);
    }
}

fn pci_config_write32(bus: u8, dev: u8, func: u8, offset: u8, data: u32) {
    let address = PCIEXBAR
        | ((bus as u32) << 20)
        | ((dev as u32) << 15)
        | ((func as u32) << 12)
        | (offset as u32);
    unsafe {
        core::ptr::write_volatile(address as *mut u32, data as u32);
    }
}

fn wait_for_not_busy() {
    unsafe {
        loop {
            let b = inb(SMBASE + HST_STS);
            //uart_puts("wait for busy HST_STS: ");
            //uart_put_hex8(b);
            //uart_put8(b'\r');
            //uart_put8(b'\n');
            if (b & HST_STS_BUSY) == 0 {
                return;
            }
            if (b & (HST_STS_DEV_ERR | HST_STS_BUS_ERR)) != 0 {
                uart_puts("smbus error\r\n");
                loop {}
            }
        }
    }
}

fn smbus_read_byte(slv_addr: u8, data_addr: u8) -> u8 {
    unsafe {
        wait_for_not_busy();

        outb(SMBASE + HST_STS, 0xff); // clear status
        outb(SMBASE + XMIT_SLVA, (slv_addr << 1) | 1); // read from slv_addr dev
        outb(SMBASE + HST_CMD, data_addr); // read from data_addr
        outb(SMBASE + HST_CNT, (2 << 2) | (1 << 6)); // start read byte

        wait_for_not_busy();

        for _ in 0..1024 * 1024 {
            let b = inb(SMBASE + HST_STS);
            //uart_puts("HST_STS: ");
            //uart_put_hex8(b);
            //uart_put8(b'\r');
            //uart_put8(b'\n');

            if b & HST_STS_INTR != 0 {
                return inb(SMBASE + HST_D0);
            }
        }
    }

    uart_puts("smbus_read_byte timeout\r\n");

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rmain() {
    pci_config_write16(0, SMBUS_DEV, SMBUS_FUNC, 0x04, 1); // enable io access
    pci_config_write32(0, SMBUS_DEV, SMBUS_FUNC, 0x20, SMBASE as u32); // set SMBASE
    pci_config_write8(0, SMBUS_DEV, SMBUS_FUNC, 0x40, 1); // enable SMBus
    outb(SMBASE + HST_CNT, 0); // init CNT
    inb(SMBASE + HST_CNT);

    let spd_len = smbus_read_byte(0x50, 0x00);
    uart_puts("spd_len: ");
    uart_put_hex8(spd_len);
    uart_puts("\r\n");

    for i in 0..spd_len {
        if i % 16 == 0 {
            uart_put8(b'\r');
            uart_put8(b'\n');
            uart_put_hex32(i as u32);
            uart_put8(b' ');
        }

        let b = smbus_read_byte(0x50, i);
        uart_put_hex8(b);
        uart_put8(b' ');
    }

    loop {}
}
