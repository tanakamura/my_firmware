#![no_std]

// see asm/modes.s
#[repr(C)]
#[derive(Copy, Clone)]
pub struct X86State {
    pub eax: u32, // 0
    pub ecx: u32, // 1
    pub edx: u32, // 2
    pub ebx: u32, // 3
    pub esp: u32, // 4

    pub ebp: u32,    // 5
    pub esi: u32,    // 6
    pub edi: u32,    // 7
    pub eflags: u32, // 8, clobberd, unable to pass to real mode

    pub es: u32, // 9
    pub ss: u32, // 10

    pub eip: u32, // 11
    pub ds: u32,  // 12
    pub cs: u32,  // 13
                  // 14
}

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

pub struct ServiceFuncTable {
    pub set_16state: extern "C" fn(&X86State),
    pub get_16state: extern "C" fn() -> X86State,
    pub enter_to_16: extern "C" fn(),
}

pub fn get_service_func_table() -> *const ServiceFuncTable {
    return (0x400 + 0) as *const ServiceFuncTable;
}
