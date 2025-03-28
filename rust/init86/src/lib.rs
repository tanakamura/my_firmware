#![no_std]

// see asm/modes.s
#[repr(packed, C)]
#[derive(Copy, Clone)]
pub struct X86State {
    pub eax: u32, // 0
    pub ecx: u32, // 1
    pub edx: u32, // 2
    pub ebx: u32, // 3
    pub esp: u32, // 4

    // do not use followed regs. these regs may clobbered
    pub ebp: u32,    // 5
    pub esi: u32,    // 6
    pub edi: u32,    // 7
    pub eflags: u32, // 8
    pub cs: u32,     // 9
    pub eip: u32,    // 10
    pub ds: u32,     // 11
                     //12
}

unsafe extern "C" {
    pub fn reset();
    pub fn enter_to_16_asm();
    static mut state16_regs: X86State;
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

pub fn set_16state(st: &X86State) {
    unsafe {
        let p = &raw mut state16_regs as *mut X86State;
        *p = *st;
    }
}

pub fn get_16state() -> X86State {
    unsafe {
        let p = &raw const state16_regs as *const X86State;
        *p
    }
}
