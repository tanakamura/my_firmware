#![no_std]

#[repr(packed, C)]
#[derive(Copy, Clone)]
pub struct State16 {
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    eip: u32,
    eflags: u32,
}

unsafe extern "C" {
    pub fn reset();
    pub fn enter_to_16_asm();
    static mut state16_regs: State16;
}

pub fn keep_syms() {
    unsafe {
        reset();
    }
}

pub fn enter_to_16() {
    unsafe {
        enter_to_16_asm();
    }
}

pub fn set_16state(st: &State16) {
    unsafe {
        let p = &raw mut state16_regs as *mut State16;
        *p = *st;
    }
}

pub fn get_16state() -> State16 {
    unsafe {
        let p = &raw const state16_regs as *const State16;
        *p
    }
}
