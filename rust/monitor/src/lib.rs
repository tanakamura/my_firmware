#![no_main]
#![no_std]

use x86::{
    io::{inb, inl, inw, outb, outl, outw},
    msr::{rdmsr, wrmsr},
};

const INIT: u8 = 127;

const READ8: u8 = 2;
const READ16: u8 = 3;
const READ32: u8 = 4;

const WRITE8: u8 = 5;
const WRITE16: u8 = 6;
const WRITE32: u8 = 7;

const IN8: u8 = 8;
const IN16: u8 = 9;
const IN32: u8 = 10;

const OUT8: u8 = 11;
const OUT16: u8 = 12;
const OUT32: u8 = 13;

const RDMSR: u8 = 14;
const WRMSR: u8 = 15;

const LOADBIN: u8 = 16;
const RUNBIN: u8 = 17;

const LOADBIN16: u8 = 18;
const RUNBIN16: u8 = 19;

const UART_BASE: u16 = 0x3f8;
const UART_DATA: u16 = UART_BASE;
const UART_LSR: u16 = UART_BASE + 5;

const LOADBIN_BASE: u32 = 0x20000000;
const LOADBIN16_BASE: u32 = 0x0001_0000; // seg=0x1000, off=0000

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
fn uart_get16() -> u16 {
    let mut v: u16 = 0;
    v |= uart_get8() as u16;
    v |= (uart_get8() as u16) << 8;
    return v;
}

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
fn uart_put16(c: u16) {
    uart_put8((c >> 0) as u8);
    uart_put8((c >> 8) as u8);
}
fn uart_put32(c: u32) {
    uart_put8((c >> 0) as u8);
    uart_put8((c >> 8) as u8);
    uart_put8((c >> 16) as u8);
    uart_put8((c >> 24) as u8);
}

fn read8() {
    let addr = uart_get32() as *const u8;
    let v = unsafe { core::ptr::read_volatile(addr) };
    uart_put8(v);
}
fn read16() {
    let addr = uart_get32() as *const u16;
    let v = unsafe { core::ptr::read_volatile(addr) };
    uart_put16(v);
}
fn read32() {
    let addr = uart_get32() as *const u32;
    let v = unsafe { core::ptr::read_volatile(addr) };
    uart_put32(v);
}

fn in8() {
    let port = uart_get16();
    let val = unsafe { inb(port) };
    uart_put8(val);
}
fn in16() {
    let port = uart_get16();
    let val = unsafe { inw(port) };
    uart_put16(val);
}
fn in32() {
    let port = uart_get16();
    let val = unsafe { inl(port) };
    uart_put32(val);
}

fn out8() {
    let port = uart_get16();
    let val = uart_get8();
    unsafe { outb(port, val) };
    uart_put8(0xfe);
}
fn out16() {
    let port = uart_get16();
    let val = uart_get16();
    unsafe { outw(port, val) };
    uart_put8(0xfe);
}
fn out32() {
    let port = uart_get16();
    let val = uart_get32();
    unsafe { outl(port, val) };
    uart_put8(0xfe);
}

fn write8() {
    let addr = uart_get32() as *mut u8;
    let val = uart_get8();
    unsafe { core::ptr::write_volatile(addr, val) };
    uart_put8(0xfe);
}
fn write16() {
    let addr = uart_get32() as *mut u16;
    let val = uart_get16();
    unsafe { core::ptr::write_volatile(addr, val) };
    uart_put8(0xfe);
}
fn write32() {
    let addr = uart_get32() as *mut u32;
    let val = uart_get32();
    unsafe { core::ptr::write_volatile(addr, val) };
    uart_put8(0xfe);
}

fn rdmsr64() {
    let addr = uart_get32();
    let val = unsafe { rdmsr(addr) };

    uart_put32(val as u32);
    uart_put32((val >> 32) as u32);
}

fn wrmsr64() {
    let addr = uart_get32();
    let mut val: u64 = 0;
    val |= uart_get32() as u64;
    val |= (uart_get32() as u64) << 32;

    unsafe { wrmsr(addr, val) };

    uart_put8(0xfe);
}

fn loadbin(off: u32) {
    let len = uart_get32();
    let checksum = uart_get8();
    let mut sum: u8 = 0;

    for i in 0..len {
        let c = uart_get8();
        unsafe {
            *((off + i) as *mut u8) = c;
        };
    }

    for i in 0..len {
        let c = unsafe { *((off + i) as *const u8) };
        sum ^= c;
    }

    if sum == checksum {
        uart_put8(0xfe);
    } else {
        uart_put8(0xff);
    }
}

fn loadbin32() {
    loadbin(LOADBIN_BASE);
}

fn loadbin16() {
    loadbin(LOADBIN16_BASE);
}

fn runbin32() {
    let entry = unsafe { core::mem::transmute::<u32, unsafe extern "C" fn() -> u32>(LOADBIN_BASE) };
    x86::fence::mfence();
    let v = unsafe { entry() };
    uart_put8(0xff); // EOF
    uart_put32(v);
}

fn runbin16() {
    let size = core::mem::size_of::<init86::X86State>() as isize;
    let mut st: init86::X86State = unsafe { core::mem::zeroed() };

    unsafe {
        let st_ptr = (&raw mut st) as *mut u8;
        for i in 0..size {
            *(st_ptr.offset(i)) = uart_get8();
        }
    }

    let service_func = init86::get_service_func_table();
    unsafe {
        ((*service_func).set_16state)(&st);
        ((*service_func).enter_to_16)();
    }

    uart_put8(0xff); // EOF

    unsafe {
        let mut st = ((*service_func).get_16state)();
        let st_ptr = (&raw mut st) as *mut u8;
        for i in 0..size {
            uart_put8(*st_ptr.offset(i));
        }
    }
}

pub fn console_loop() {
    loop {
        let c = uart_get8();

        match c {
            INIT => uart_put8(1),
            READ8 => read8(),
            READ16 => read16(),
            READ32 => read32(),

            WRITE8 => write8(),
            WRITE16 => write16(),
            WRITE32 => write32(),

            IN8 => in8(),
            IN16 => in16(),
            IN32 => in32(),

            OUT8 => out8(),
            OUT16 => out16(),
            OUT32 => out32(),

            RDMSR => rdmsr64(),
            WRMSR => wrmsr64(),

            LOADBIN => loadbin32(),
            RUNBIN => runbin32(),

            LOADBIN16 => loadbin16(),
            RUNBIN16 => runbin16(),

            _ => uart_put8(0xff),
        }
    }
}
