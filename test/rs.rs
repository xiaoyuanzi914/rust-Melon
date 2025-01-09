#[macro_use]
extern crate c2rust_bitfields;

mod mln_rs;

use std::ptr;
use std::io::{self, Write};

const COL: usize = 10;
const ROW: usize = 10;
const K: usize = 2;

fn main() {
    let origin: Vec<u8> = b"0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_vec();
    let mut err: [*mut u8; ROW + K] = [ptr::null_mut(); ROW + K];
    let mut res: *mut mln_rs::mln_rs_result_t = ptr::null_mut();
    let mut dres: *mut mln_rs::mln_rs_result_t = ptr::null_mut();
    let mut i: i32;
    let mut j: usize;

    unsafe {
        // 编码过程
        res = mln_rs::mln_rs_encode(origin.as_ptr() as *const libc::c_uint8, COL as i32, ROW as i32, K as i32);
        assert!(!res.is_null());

        println!("res->num={}, res->len={}", (*res).num, (*res).len);

        // 获取编码结果
        for i in 0..mln_rs::mln_rs_result_get_num(res) {
            err[i as usize] = mln_rs::mln_rs_result_get_data_by_index(res, i);
        }
        err[0] = ptr::null_mut();
        err[1] = ptr::null_mut();

        // 解码过程
        dres = mln_rs::mln_rs_decode(err.as_mut_ptr(), COL as i32, ROW as i32, K as i32);
        assert!(!dres.is_null());

        // 输出解码结果
        for i in 0..mln_rs::mln_rs_result_get_num(dres) {
            for j in 0..COL {
                print!("{}", *mln_rs::mln_rs_result_get_data_by_index(dres, i).offset(j as isize));
            }
            println!();
        }

        // 释放内存
        mln_rs::mln_rs_result_free(res);
        mln_rs::mln_rs_result_free(dres);
    }
}
