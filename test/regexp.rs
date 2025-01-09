#[macro_use]
extern crate c2rust_bitfields;

mod mln_regexp;

use std::ffi::CStr;
use std::ptr;
use std::io::{self, Write};

fn main() {
    let text = b"dabcdeadcbef".to_vec();
    let exp = b"a.c.e".to_vec();
    let mut res: *mut mln_regexp::mln_reg_match_result_t = std::ptr::null_mut();
    let mut s: *mut mln_regexp::mln_string_t;
    let mut i: i32;
    let mut n: i32;

    unsafe {
        // 创建正则匹配结果对象
        res = mln_regexp::mln_reg_match_result_new(1);
        assert!(!res.is_null());

        // 执行正则匹配
        n = mln_regexp::mln_reg_match(exp.as_ptr() as *const libc::c_char, text.as_ptr() as *const libc::c_char, res);
        assert!(n != 0);
        println!("{} matched", n);

        // 获取匹配结果
        s = mln_regexp::mln_reg_match_result_get(res);
        for i in 0..n {
            let matched_str = CStr::from_ptr((*s.offset(i as isize)).data as *const i8);
            io::stdout().write_all(matched_str.to_bytes()).unwrap();
            io::stdout().write_all(b"\n").unwrap();
        }

        // 释放匹配结果
        mln_regexp::mln_reg_match_result_free(res);

        // 检查正则表达式是否与文本相等
        let equal = mln_regexp::mln_reg_equal(exp.as_ptr() as *const libc::c_char, text.as_ptr() as *const libc::c_char);
        assert_eq!(equal, 0);
    }
}
