#[macro_use]
extern crate c2rust_bitfields;

mod mln_string;  // 导入 mln_string.rs 文件
mod mln_base64;  // 导入 mln_base64.rs 文件

use crate::mln_string::{mln_string_t, mln_string_nset};
use crate::mln_base64::{mln_base64_encode, mln_base64_decode, mln_base64_free};
use std::io::{self, Write};

fn main() {
    let text = mln_string_t {
        data: b"Hello".as_ptr() as *const u8, // 字符串 "Hello"
        len: 5,
    };
    let mut tmp: mln_string_t = unsafe { std::mem::zeroed() };
    let mut p1: *mut u8 = std::ptr::null_mut();
    let mut p2: *mut u8 = std::ptr::null_mut();
    let mut len1: usize = 0;
    let mut len2: usize = 0;

    // Base64 编码
    unsafe {
        if mln_base64_encode(text.data, text.len, &mut p1, &mut len1) < 0 {
            eprintln!("encode failed");
            return;
        }
    }
    unsafe {
        mln_string_nset(&mut tmp, p1, len1);
        // 打印编码后的字符串
        std::io::stdout().write_all(&tmp.data).unwrap();
        std::io::stdout().write_all(b"\n").unwrap();
    }

    // Base64 解码
    unsafe {
        if mln_base64_decode(p1, len1, &mut p2, &mut len2) < 0 {
            eprintln!("decode failed");
            return;
        }
    }
    unsafe {
        mln_string_nset(&mut tmp, p2, len2);
        // 打印解码后的字符串
        std::io::stdout().write_all(&tmp.data).unwrap();
        std::io::stdout().write_all(b"\n").unwrap();
    }

    // 释放内存
    unsafe {
        mln_base64_free(p1);
        mln_base64_free(p2);
    }
}
