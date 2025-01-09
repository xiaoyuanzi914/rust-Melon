#[macro_use]
extern crate c2rust_bitfields;

mod mln_queue;

use std::ffi::CStr;
use std::ptr;

fn main() {
    let mut i: i32 = 10;
    let q;

    // 初始化队列
    unsafe {
        q = mln_queue::mln_queue_init(10, ptr::null_mut());
        if q.is_null() {
            eprintln!("queue init failed.");
            return;
        }

        // 向队列中添加数据
        mln_queue::mln_queue_append(q, &mut i as *mut i32 as *mut libc::c_void);

        // 获取队列中的数据并打印
        let value = mln_queue::mln_queue_get(q) as *mut i32;
        if !value.is_null() {
            println!("{}", *value);
        }

        // 销毁队列
        mln_queue::mln_queue_destroy(q);
    }
}
