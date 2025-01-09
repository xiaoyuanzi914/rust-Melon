#[macro_use]
extern crate c2rust_bitfields;

mod mln_array; // 导入 mln_array.rs 文件

use crate::mln_array::{mln_array_t, mln_array_init, mln_array_push, mln_array_pushn, mln_array_elts, mln_array_nelts, mln_array_destroy};
use std::io::{self, Write};

#[repr(C)]
struct TestT {
    i1: i32,
    i2: i32,
}

fn main() {
    let mut arr: mln_array_t = unsafe { std::mem::zeroed() }; // 初始化 mln_array_t 结构体

    // 初始化数组
    unsafe {
        mln_array_init(&mut arr, std::ptr::null_mut(), std::mem::size_of::<TestT>(), 1);
    }

    // 推送第一个元素
    let t = unsafe { mln_array_push(&mut arr) };
    if t.is_null() {
        return;
    }
    unsafe { (*t).i1 = 0 };

    // 推送多个元素
    let t = unsafe { mln_array_pushn(&mut arr, 9) };
    for i in 0..9 {
        unsafe { (*t.offset(i as isize)).i1 = i as i32 + 1 };
    }

    // 打印数组中的元素
    unsafe {
        let t = mln_array_elts(&arr);
        let n = mln_array_nelts(&arr);
        for i in 0..n {
            println!("{}", (*t.offset(i as isize)).i1);
        }
    }

    // 销毁数组
    unsafe {
        mln_array_destroy(&mut arr);
    }
}
