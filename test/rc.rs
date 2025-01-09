#[macro_use]
extern crate c2rust_bitfields;

mod mln_rc;

use std::ffi::CStr;
use std::ptr;
use std::io::Write;

fn main() {
    let mut s: [u8; 256] = [0; 256];
    let text: &mut [u8] = b"Hello".to_mut();

    unsafe {
        // 初始化 RC4 算法
        mln_rc::mln_rc4_init(s.as_mut_ptr(), "this is a key".as_ptr() as *const libc::c_char, "this is a key".len() as libc::size_t);

        // 进行加密和解密
        mln_rc::mln_rc4_calc(s.as_mut_ptr(), text.as_mut_ptr(), text.len() as libc::size_t);
        println!("{}", String::from_utf8_lossy(text));

        mln_rc::mln_rc4_calc(s.as_mut_ptr(), text.as_mut_ptr(), text.len() as libc::size_t);
        println!("{}", String::from_utf8_lossy(text));
    }
}
