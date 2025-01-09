#[macro_use]
extern crate c2rust_bitfields;

mod mln_log;
mod mln_conf;

use std::ptr;
use libc::{c_void};

use crate::mln_log::{mln_log, mln_log_init, mln_log_destroy};
use crate::mln_conf::{mln_conf_load, mln_conf_free};

fn main() {
    // 调试信息输出
    unsafe {
        mln_log(mln_log::LogLevel::Debug, "This will be outputted to stderr\n");

        mln_conf_load(); // 加载配置

        // 初始化日志
        assert!(mln_log_init(ptr::null_mut()) == 0);

        // 输出日志信息到stderr和日志文件
        mln_log(mln_log::LogLevel::Debug, "This will be outputted to stderr and log file\n");

        // 销毁日志
        mln_log_destroy();

        // 释放配置资源
        mln_conf_free();
    }
}
