#![no_std]

extern crate init86;

pub use init86::get_service_func_table;
pub use init86::ServiceFuncTable;
pub use init86::X86State;

unsafe extern "C" {
    pub fn reset();
    pub fn enter_to_16_asm();
    static mut state16_regs_flat32: X86State;
}

pub fn keep_syms() {
    unsafe {
        reset();
    }
}

#[unsafe(no_mangle)]
extern "C" fn enter_to_16() {
    unsafe {
        enter_to_16_asm();
    }
}

#[unsafe(no_mangle)]
extern "C" fn set_16state(st: &X86State) {
    unsafe {
        let p = &raw mut state16_regs_flat32 as *mut X86State;
        *p = *st;
    }
}

#[unsafe(no_mangle)]
extern "C" fn get_16state() -> X86State {
    unsafe {
        let p = &raw const state16_regs_flat32 as *const X86State;
        *p
    }
}
