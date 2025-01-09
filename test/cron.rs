#[macro_use]
extern crate c2rust_bitfields;

mod mln_cron;  // 导入 mln_cron.rs 文件
mod mln_string; // 导入 mln_string.rs 文件

use crate::mln_cron::{mln_cron_parse};
use crate::mln_string::{mln_string_t, mln_string_nset};

use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CString;
use std::ptr;

fn main() {
    // 测试字符串设置
    let p = "* * * * *";
    let mut s: mln_string_t = unsafe { std::mem::zeroed() };
    mln_string_nset(&mut s, p.as_ptr() as *const i8, p.len());

    // 获取当前时间
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    
    // 解析 cron 表达式
    let next = unsafe { mln_cron_parse(&s, now) };

    // 输出结果
    println!("{} {} {}", now, next, unsafe { ctime(&next) });
}

extern "C" {
    fn ctime(time: *const i64) -> *const i8;
}
