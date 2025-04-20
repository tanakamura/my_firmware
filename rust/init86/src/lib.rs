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
    pub eflags: u32, // 8

    pub es: u32, // 9
    pub ss: u32, // 10

    pub eip: u32, // 11
    pub ds: u32,  // 12
    pub cs: u32,  // 13
                  // 14
}

pub struct ServiceFuncTable {
    pub set_16state: extern "C" fn(&X86State),
    pub get_16state: extern "C" fn() -> X86State,
    pub enter_to_16: extern "C" fn(),
    pub alloc_from_16: extern "C" fn(size: usize) -> *mut u8,
    pub free_to_16: extern "C" fn(ptr: *mut u8, size: usize),
}

pub fn get_service_func_table() -> *const ServiceFuncTable {
    return (0xf0000 + 0) as *const ServiceFuncTable;
}
