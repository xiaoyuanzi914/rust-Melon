#[macro_use]
extern crate c2rust_bitfields;

mod mln_des;  // 导入 mln_des.rs 文件

use crate::mln_des::{mln_3des_t, mln_3des_init, mln_3des_buf};

use std::ffi::CString;
use std::ptr;

fn main() {
    let mut d: mln_3des_t = unsafe { std::mem::zeroed() };
    let mut text: [u8; 9] = [0; 9];
    let mut cipher: [u8; 9] = [0; 9];

    // 初始化 3DES
    unsafe {
        mln_3des_init(&mut d, 0xffff, 0xff120000);
    }

    // 加密文本
    unsafe {
        mln_3des_buf(&d, b"Hi Tom!!".as_ptr() as *const i8, 11, cipher.as_mut_ptr(), cipher.len(), 0, 1);
    }

    // 解密文本
    unsafe {
        mln_3des_buf(&d, cipher.as_ptr(), cipher.len() - 1, text.as_mut_ptr(), text.len(), 0, 0);
    }

    // 输出解密后的结果
    println!("{}", unsafe { std::str::from_utf8_unchecked(&text) });
}
