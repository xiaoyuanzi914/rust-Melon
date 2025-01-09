#[macro_use]
extern crate c2rust_bitfields;

mod mln_list;
mod mln_utils;

use std::ptr;
use std::ffi::{CStr, CString};
use libc::{calloc, free};

use crate::mln_list::{mln_list_t, mln_list_null, mln_list_add, mln_list_head, mln_list_next, mln_container_of};
use crate::mln_utils::*; // 引用 mln_utils 中的内容

#[repr(C)]
struct test_t {
    val: i32,
    node: mln_list_t,
}

fn main() {
    let mut sentinel = mln_list_null();  // 创建一个空链表

    for i in 0..3 {
        unsafe {
            let t: *mut test_t = calloc(1, std::mem::size_of::<test_t>()) as *mut test_t;
            if t.is_null() {
                eprintln!("Memory allocation failed");
                return;
            }
            (*t).val = i;
            mln_list_add(&mut sentinel, &mut (*t).node);
        }
    }

    unsafe {
        let mut t = mln_container_of(mln_list_head(&sentinel), test_t, node);
        while !t.is_null() {
            println!("{}", (*t).val);
            t = mln_container_of(mln_list_next(&(*t).node), test_t, node);
        }
    }

    unsafe {
        let mut t = mln_container_of(mln_list_head(&sentinel), test_t, node);
        while !t.is_null() {
            let fr = t;
            t = mln_container_of(mln_list_next(&(*t).node), test_t, node);
            free(fr as *mut libc::c_void);
        }
    }
}
