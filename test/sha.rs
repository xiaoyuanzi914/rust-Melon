#[macro_use]
extern crate c2rust_bitfields;

mod mln_sha;

use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;

fn main() {
    let mut s: mln_sha::mln_sha256_t = mln_sha::mln_sha256_t { state: [0; 32], count: [0; 2], buffer: [0; 64] };
    let mut text: [c_char; 1024] = [0; 1024];

    unsafe {
        mln_sha::mln_sha256_init(&mut s);
        mln_sha::mln_sha256_calc(&mut s, b"Hello".as_ptr() as *const u8, 5, 1);  // 5 is the length of "Hello"
        mln_sha::mln_sha256_tostring(&s, text.as_mut_ptr() as *mut c_char, 1023);
    }

    // Convert C string to Rust string and print
    let result = unsafe {
        CStr::from_ptr(text.as_ptr())
            .to_str()
            .unwrap_or("Invalid UTF-8")
    };
    
    println!("{}", result);
}
