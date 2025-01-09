#[macro_use]
extern crate c2rust_bitfields;
mod mln_aes;  // 导入 mln_aes.rs 文件

use crate::mln_aes::{mln_aes_t, mln_aes_init, mln_aes_encrypt, mln_aes_decrypt};
use std::io::{self, Write};

fn main() {
    // 初始化 AES 结构体
    let mut aes: mln_aes_t = unsafe { std::mem::zeroed() }; // 使用零初始化

    // 设置 AES 密钥（128 位）
    let mut key: Vec<u8> = "abcdefghijklmnop".bytes().collect();  // 128位密钥
    let key_ptr = key.as_mut_ptr();
    let bits: u32 = 0;  // 对应128位密钥

    // 初始化 AES
    let init_result = unsafe { mln_aes_init(&mut aes, key_ptr, bits) };

    if init_result != 0 {
        panic!("AES init failed with error code: {}", init_result);
    }

    println!("AES initialization succeeded.");

    // 加密/解密操作的数据（替换为与C代码相同的数据 "1234567890123456"）
    let mut s: Vec<u8> = "1234567890123456".bytes().collect();  // 明文数据 "1234567890123456"
    let s_ptr = s.as_mut_ptr();  // 获取数据的原始指针

    // 加密操作
    unsafe {
        println!("Before encryption: {:?}", s);
        mln_aes_encrypt(&mut aes, s_ptr);  // 执行 AES 加密
        println!("After encryption: {:?}", s);

        println!("Decrypting...");
        mln_aes_decrypt(&mut aes, s_ptr);  // 执行 AES 解密
        println!("After decryption: {:?}", s);
    }
}