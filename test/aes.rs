#[macro_use]
extern crate c2rust_bitfields;

mod mln_string;  // 导入 mln_string.rs 文件
mod mln_aes;     // 导入 mln_aes.rs 文件

use crate::mln_string::{mln_string_t, mln_string_set};
use crate::mln_aes::{mln_aes_t, mln_aes_init, mln_aes_encrypt, mln_aes_decrypt};
use std::io::{self, Write};

fn main() {
    // 初始化 AES 结构体
    let mut aes: mln_aes_t = unsafe { std::mem::zeroed() }; // 使用零初始化

    // 设置 AES 密钥（128 位）
    let mut key: Vec<u8> = "abcdefghijklmnop".bytes().collect(); // 128 位密钥
    let key_ptr = key.as_ptr() as *const u8;
    let bits: u32 = 0;  // 对应 128 位密钥

    // 初始化 AES
    let init_result = unsafe { mln_aes_init(&mut aes, key_ptr, bits) };

    if init_result != 0 {
        eprintln!("aes init failed");
        return;
    }

    // 设置明文数据
    let mut s: mln_string_t = mln_string_t {
        data: "1234567890123456".as_ptr() as *const u8,
        len: 16,
    };

    // 加密操作
    unsafe {
        if mln_aes_encrypt(&aes, s.data) < 0 {
            eprintln!("aes encrypt failed");
            return;
        }

        // 打印加密后的数据
        std::io::stdout().write_all(&s.data).unwrap();
        std::io::stdout().write_all(b"\n").unwrap();
    }

    // 解密操作
    unsafe {
        if mln_aes_decrypt(&aes, s.data) < 0 {
            eprintln!("aes decrypt failed");
            return;
        }

        // 打印解密后的数据
        std::io::stdout().write_all(&s.data).unwrap();
        std::io::stdout().write_all(b"\n").unwrap();
    }
}
