#![no_std]

extern crate init86;

pub use init86::get_service_func_table;
pub use init86::ServiceFuncTable;
pub use init86::X86State;
use linked_list_allocator::LockedHeap;

static ALLOCATOR_16: LockedHeap = LockedHeap::empty();

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
pub extern "C" fn enter_to_16() {
    unsafe {
        enter_to_16_asm();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_16state(st: &X86State) {
    unsafe {
        let p = &raw mut state16_regs_flat32 as *mut X86State;
        *p = *st;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_16state() -> X86State {
    unsafe {
        let p = &raw const state16_regs_flat32 as *const X86State;
        *p
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc_from_16(size: usize) -> *mut u8 {
    unsafe {
        let mut heap16 = ALLOCATOR_16.lock();
        let ptr = heap16
            .allocate_first_fit(core::alloc::Layout::from_size_align(size, 8).unwrap())
            .unwrap();
        ptr.as_ptr()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_to_16(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR_16.lock().deallocate(
            core::ptr::NonNull::new(ptr).unwrap(),
            core::alloc::Layout::from_size_align(size, 8).unwrap(),
        );
    }
}

unsafe extern "C" {
    static mut __end16_flat32: u8;
    static mut __end16_in_segment: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn init_heap16() {
    // Initialize the heap allocator for real mode
    unsafe {
        let heap16_start: *mut u8 = &raw mut __end16_flat32;
        let heap16_size: usize = 32 * 1024 as usize;

        if heap16_size > 0 {
            ALLOCATOR_16.lock().init(heap16_start, heap16_size);
        }
    }
}
