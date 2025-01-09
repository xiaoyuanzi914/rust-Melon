#[macro_use]
extern crate c2rust_bitfields;

mod mln_md5;

use std::ffi::CString;
use std::ptr;

use crate::mln_md5::{mln_md5_init, mln_md5_calc, mln_md5_tostring};

fn main() {
    let text = "Hello";
    let mut output = [0u8; 33];  // Output buffer initialized to 0

    // 创建 MD5 计算上下文
    let mut m = mln_md5::mln_md5_t::default();

    // 初始化 MD5
    unsafe {
        mln_md5_init(&mut m);
    }

    // 计算 MD5
    unsafe {
        mln_md5_calc(&mut m, text.as_ptr() as *const u8, text.len() as u64, 1);
    }

    // 将计算结果转换为字符串
    unsafe {
        mln_md5_tostring(&mut m, output.as_mut_ptr() as *mut i8, output.len() as u64);
    }

    // 输出 MD5 字符串
    let output_str = match CString::new(output.clone()) {
        Ok(s) => s.to_string_lossy().into_owned(),
        Err(_) => String::from("Invalid output"),
    };

    println!("{}", output_str);
}
