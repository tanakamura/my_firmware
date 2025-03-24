#![no_std]

extern "C" {
    pub fn reset();
}

pub fn keep_syms() {
    unsafe {
        reset();
    }
}
