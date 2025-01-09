#[macro_use]
extern crate c2rust_bitfields;

mod mln_matrix;

use std::ptr;
use libc::{c_void, strerror};

use crate::mln_matrix::{mln_matrix_new, mln_matrix_dump, mln_matrix_inverse, mln_matrix_free};

fn main() {
    let data: [f64; 9] = [1.0, 1.0, 1.0, 1.0, 2.0, 4.0, 2.0, 8.0, 64.0];

    // 创建一个新的矩阵
    let a = mln_matrix_new(3, 3, &data, 1);
    if a.is_null() {
        eprintln!("init matrix failed");
        return;
    }
    
    // 打印矩阵
    unsafe { mln_matrix_dump(a) };

    // 求矩阵的逆
    let b = mln_matrix_inverse(a);
    unsafe { mln_matrix_free(a) };

    // 处理逆矩阵计算失败
    if b.is_null() {
        eprintln!("inverse failed: {}", unsafe { strerror(libc::errno) });
        return;
    }

    // 打印逆矩阵
    unsafe { mln_matrix_dump(b) };

    // 释放矩阵
    unsafe { mln_matrix_free(b) };
}
