#[macro_use]
extern crate c2rust_bitfields;

mod mln_fheap;
mod mln_log;

use std::ptr;
use std::ffi::CString;
use std::assert;

use crate::mln_fheap::{mln_fheap_t, mln_fheap_node_t, mln_fheap_new, mln_fheap_node_init, mln_fheap_inline_insert, mln_fheap_minimum, mln_fheap_node_key, mln_fheap_node_new, mln_fheap_inline_free};
use crate::mln_log::{mln_log};

#[repr(C)]
struct Ud {
    val: i32,
    node: mln_fheap_node_t, //自定义数据结构的成员
}

#[inline]
unsafe fn container_cmp_handler(key1: *const libc::c_void, key2: *const libc::c_void) -> i32 {
    let key1 = key1 as *const Ud;
    let key2 = key2 as *const Ud;
    if (*key1).val < (*key2).val {
        0
    } else {
        1
    }
}

fn fheap_container_test() -> i32 {
    let mut fh: *mut mln_fheap_t;
    let mut min = Ud { val: 0, node: mln_fheap_node_t { next: ptr::null_mut(), prev: ptr::null_mut() } };
    let mut data1 = Ud { val: 1, node: mln_fheap_node_t { next: ptr::null_mut(), prev: ptr::null_mut() } };
    let mut data2 = Ud { val: 2, node: mln_fheap_node_t { next: ptr::null_mut(), prev: ptr::null_mut() } };
    let mut fn_ = ptr::null_mut();

    unsafe {
        fh = mln_fheap_new(&min, ptr::null_mut());
        if fh.is_null() {
            eprintln!("fheap init failed.");
            return -1;
        }

        mln_fheap_node_init(&mut data1.node, &data1);
        mln_fheap_node_init(&mut data2.node, &data2);
        mln_fheap_inline_insert(fh, &mut data1.node, container_cmp_handler);
        mln_fheap_inline_insert(fh, &mut data2.node, container_cmp_handler);

        fn_ = mln_fheap_minimum(fh);
        //两种方式获取自定义数据
        println!("{}", (*mln_fheap_node_key(fn_)).val);
        println!("{}", (*mln_fheap_node_key(fn_)).val);

        mln_fheap_inline_free(fh, container_cmp_handler, ptr::null_mut());
    }

    0
}

#[inline]
unsafe fn cmp_handler(key1: *const libc::c_void, key2: *const libc::c_void) -> i32 {
    let key1 = key1 as *const i32;
    let key2 = key2 as *const i32;
    if *key1 < *key2 {
        0
    } else {
        1
    }
}

fn fheap_test() -> i32 {
    let mut i = 10;
    let mut min = 0;
    let mut fh: *mut mln_fheap_t;
    let mut fn_ = ptr::null_mut();

    unsafe {
        fh = mln_fheap_new(&min, ptr::null_mut());
        if fh.is_null() {
            eprintln!("fheap init failed.");
            return -1;
        }

        fn_ = mln_fheap_node_new(fh, &i);
        if fn_.is_null() {
            eprintln!("fheap node init failed.");
            return -1;
        }

        mln_fheap_inline_insert(fh, fn_, cmp_handler);

        fn_ = mln_fheap_minimum(fh);
        println!("{}", *mln_fheap_node_key(fn_));

        mln_fheap_inline_free(fh, cmp_handler, ptr::null_mut());
    }

    0
}

fn main() {
    unsafe {
        assert!(fheap_test() == 0);
        assert!(fheap_container_test() == 0);
    }
}
