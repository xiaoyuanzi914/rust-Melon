

// src/main.rs
mod mln_alloc_bindings {
    include!("mln_alloc_bindings.rs");
}

use mln_alloc_bindings::*;
use std::ffi::CStr;
use std::ptr;
use std::io::{self, Write};

fn main() {
    let mut pool: *mut mln_alloc_t = ptr::null_mut();

    // 初始化内存池
    unsafe {
        pool = mln_alloc_init(ptr::null_mut());
        if pool.is_null() {
            eprintln!("pool init failed");
            return;
        }
    }

    // 分配内存
    let p = unsafe { mln_alloc_m(pool, 6) as *mut u8 };
    if p.is_null() {
        eprintln!("alloc failed");
        return;
    }

    // 写入数据
    unsafe {
        std::ptr::copy_nonoverlapping("hello".as_ptr(), p, 5);
        *p.add(5) = 0;
    }

    // 打印字符串
    unsafe {
        let cstr = CStr::from_ptr(p as *const i8);
        println!("{}", cstr.to_str().unwrap());
    }

    // 释放内存
    unsafe {
        mln_alloc_free(p as *mut std::ffi::c_void);
        mln_alloc_destroy(pool);
    }
}
