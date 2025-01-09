#[macro_use]
extern crate c2rust_bitfields;

mod mln_stack;

use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;

#[repr(C)]
struct Data {
    data1: *mut c_void,
    data2: *mut c_void,
}

unsafe extern "C" fn copy(d: *mut Data, data: *mut c_void) -> *mut c_void {
    let dup = alloc(Layout::new::<Data>()) as *mut Data;
    if dup.is_null() {
        return ptr::null_mut();
    }
    *dup = *d;
    dup as *mut c_void
}

fn main() {
    unsafe {
        let mut st1: *mut mln_stack::mln_stack_t = ptr::null_mut();
        let mut st2: *mut mln_stack::mln_stack_t = ptr::null_mut();
        let mut d: *mut Data;

        // Initialize the stack with free and copy functions
        st1 = mln_stack::mln_stack_init(Some(free), Some(copy));
        assert!(!st1.is_null());

        // Push 3 data items to the stack
        for _ in 0..3 {
            d = alloc(Layout::new::<Data>()) as *mut Data;
            assert!(!d.is_null());
            assert_eq!(mln_stack::mln_stack_push(st1, d), 0);
        }

        // Duplicate the stack
        st2 = mln_stack::mln_stack_dup(st1, ptr::null_mut());
        assert!(!st2.is_null());

        // Pop and free one data item from the stack
        d = mln_stack::mln_stack_pop(st1);
        assert!(!d.is_null());
        dealloc(d as *mut u8, Layout::new::<Data>());

        // Destroy both stacks
        mln_stack::mln_stack_destroy(st1);
        mln_stack::mln_stack_destroy(st2);
    }
}
