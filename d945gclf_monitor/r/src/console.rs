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

const UART_BASE: u16 = 0x3f8;
const UART_DATA: u16 = UART_BASE;
const UART_LSR: u16 = UART_BASE + 5;

//const LOADBIN_BASE:u32 = 0x80000;

const LOADBIN_BASE:u32 = 0x40_0000;
//static mut LOADBIN_BASE: [u8; 128] = [1; 128];

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

fn loadbin() {
    let len = uart_get32();
    let checksum = uart_get8();
    let mut sum = 0;

    for i in 0..len {
        let c = uart_get8();
        unsafe { core::ptr::write_volatile((LOADBIN_BASE + i) as *mut u8, c) };
        sum ^= c;
    }

    if sum == checksum {
        uart_put8(0xfe);
    } else {
        uart_put8(0xff);
    }
}

#[inline(never)]
pub unsafe extern "C" fn runbin() {
    //let entry = unsafe { core::mem::transmute::<u32, fn()->u32>(LOADBIN_BASE) };
    let entry = unsafe {
        core::mem::transmute::<u32, unsafe extern "C" fn() -> u32>(LOADBIN_BASE)
    };
    x86::fence::mfence();
    unsafe { uart_put32(*(LOADBIN_BASE as *const u32)) };
    let v = unsafe { entry() };
    unsafe { uart_put32(v) };
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

            LOADBIN => loadbin(),
            RUNBIN => unsafe { runbin() },

            _ => uart_put8(0xff),
        }
    }
}
