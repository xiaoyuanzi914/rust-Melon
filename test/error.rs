#[macro_use]
extern crate c2rust_bitfields;

mod mln_error;  // 导入 mln_error.rs 文件

use crate::mln_error::{mln_error_init, mln_error_string, RET, CODE};
use crate::mln_string::mln_string;  // 假设 mln_string 结构体已存在

use std::ffi::CString;
use std::ptr;

fn main() {
    let mut msg = [0u8; 1024];

    // 文件和错误信息数组
    let files = [
        mln_string("a.c"),
    ];
    let errs = [
        mln_string("Success"),
        mln_string("No memory"),
    ];

    // 初始化错误信息
    unsafe {
        mln_error_init(
            files.as_ptr(),
            errs.as_ptr(),
            files.len() as i32,
            errs.len() as i32
        );
    }

    // 打印错误信息
    unsafe {
        println!(
            "{:x} {} [{}]",
            RET(0),
            CODE(RET(0)),
            mln_error_string(RET(0), msg.as_mut_ptr() as *mut i8, msg.len() as i32)
        );
        println!(
            "{:x} {} [{}]",
            RET(1),
            CODE(RET(1)),
            mln_error_string(RET(1), msg.as_mut_ptr() as *mut i8, msg.len() as i32)
        );
        println!(
            "{:x} {} [{}]",
            RET(2),
            CODE(RET(2)),
            mln_error_string(RET(2), msg.as_mut_ptr() as *mut i8, msg.len() as i32)
        );
        println!(
            "{:x} {} [{}]",
            RET(0xff),
            CODE(RET(0xff)),
            mln_error_string(RET(0xff), msg.as_mut_ptr() as *mut i8, msg.len() as i32)
        );
    }
}
