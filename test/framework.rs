#[macro_use]
extern crate c2rust_bitfields;

mod mln_framework;
mod mln_log;

use std::ffi::CString;
use std::env;

use crate::mln_framework::{mln_framework_init, mln_framework_attr};

fn main() {
    println!("NOTE: This test has memory leak because we don't release memory of log, conf and multiprocess-related stuffs.");
    println!("In fact, `mln_framework_init` should be the last function called in `main`, so we don't need to release anything.");

    let args: Vec<String> = env::args().collect();
    let argc = args.len() as i32;
    let argv: Vec<*const i8> = args.iter()
        .map(|s| CString::new(s.as_str()).unwrap().into_raw())
        .collect();

    let cattr = mln_framework_attr {
        argc,
        argv: argv.as_ptr(),
        global_init: std::ptr::null_mut(),
        main_thread: std::ptr::null_mut(),
        master_process: std::ptr::null_mut(),
        worker_process: std::ptr::null_mut(),
    };

    let result = unsafe { mln_framework_init(&cattr) };
    if result != 0 {
        eprintln!("mln_framework_init failed with error code: {}", result);
    }
}
