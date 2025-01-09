#[macro_use]
extern crate c2rust_bitfields;

mod mln_bignum;  // 导入 mln_bignum.rs 文件
mod mln_string;  // 导入 mln_string.rs 文件

use crate::mln_bignum::{mln_bignum_t, mln_bignum_zero, mln_bignum_init, mln_bignum_assign, mln_bignum_pwr};
use crate::mln_string::{mln_string_t, mln_string_free, mln_bignum_tostring};
use std::io::{self, Write};

fn main() {
    // 初始化大数
    let mut n1: mln_bignum_t = mln_bignum_zero();  // 使用零初始化
    let mut n2: mln_bignum_t = mln_bignum_zero();

    // 初始化大数 n1
    unsafe { mln_bignum_init(&mut n1) };

    // 为 n1 和 n2 分配值
    unsafe {
        mln_bignum_assign(&mut n1, "10".as_ptr() as *const u8, 2);
        mln_bignum_assign(&mut n2, "30".as_ptr() as *const u8, 2);
    }

    // 计算 n1 的 n2 次方
    unsafe {
        mln_bignum_pwr(&mut n1, &mut n2, std::ptr::null_mut());
    }

    // 将 n1 转换为字符串
    let s: mln_string_t = unsafe { mln_bignum_tostring(&n1) };

    // 输出字符串
    unsafe {
        std::io::stdout().write_all(&s.data).unwrap();
        std::io::stdout().write_all(b"\n").unwrap();
    }

    // 释放字符串内存
    unsafe { mln_string_free(&s) };
}
