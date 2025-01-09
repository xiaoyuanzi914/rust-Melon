#[macro_use]
extern crate c2rust_bitfields;

mod mln_prime_generator;

use std::ffi::CStr;
use std::ptr;

fn main() {
    // 调用 mln_prime_generate 函数并打印结果
    let result = unsafe { mln_prime_generator::mln_prime_generate(32765) };
    println!("{}", result);
}
