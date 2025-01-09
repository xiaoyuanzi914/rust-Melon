#[macro_use]
extern crate c2rust_bitfields;

mod mln_alloc_bindings {
    include!("mln_alloc_bindings.rs");
}

mod mln_chain;  // 导入 mln_chain.rs 文件

use mln_alloc_bindings::*;
use crate::mln_chain::{mln_chain_t, mln_chain_new, mln_chain_pool_release, mln_buf_t, mln_buf_new};
use std::ffi::CStr;
use std::ptr;
use std::io::{self, Write};

fn main() {
    let pool: *mut mln_alloc_t;

    // 初始化内存池
    unsafe {
        pool = mln_alloc_init(ptr::null_mut());
        if pool.is_null() {
            eprintln!("pool init failed.");
            return;
        }
    }

    // 创建链表和缓冲区
    let mut c: *mut mln_chain_t;
    let mut b: *mut mln_buf_t;
    unsafe {
        c = mln_chain_new(pool);
        if c.is_null() {
            eprintln!("chain_new failed.");
            return;
        }

        b = mln_buf_new(pool);
        if b.is_null() {
            eprintln!("buf_new failed.");
            return;
        }

        (*c).buf = b;
    }

    // 分配缓冲区
    let buf: *mut u8;
    unsafe {
        buf = mln_alloc_m(pool, 1024) as *mut u8;
        if buf.is_null() {
            eprintln!("buffer allocate failed.");
            return;
        }

        (*b).left_pos = buf;
        (*b).pos = buf;
        (*b).start = buf;
        (*b).last = unsafe { buf.offset(1024) };
        (*b).end = unsafe { buf.offset(1024) };
        (*b).in_memory = 1;

        // 写入数据
        *buf = 'H' as u8;
        *unsafe { buf.offset(1) } = 'i' as u8;
    }

    // 释放链表和内存池
    unsafe {
        mln_chain_pool_release(c);
        mln_alloc_destroy(pool);
    }
}
