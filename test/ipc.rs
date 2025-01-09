#[macro_use]
extern crate c2rust_bitfields;

mod mln_framework;
mod mln_ipc;
mod mln_conf;
mod mln_log;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use libc::sleep;
use crate::mln_framework::{mln_framework_init, mln_framework_attr};
use crate::mln_ipc::{mln_ipc_worker_send_prepare, mln_ipc_master_send_prepare, mln_ipc_handler_register};
use crate::mln_conf::{mln_conf, mln_conf_domain_t, mln_conf_cmd_t};
use crate::mln_log::mln_log;

static mut ipc_cb: *mut mln_ipc_cb_t = ptr::null_mut();
static me_framework_mode: mln_string_t = mln_string("multiprocess");
static me_framework_conf: mln_conf_item_t = mln_conf_item { conf_type: CONF_STR, val: mln_string_t::from("multiprocess") };
static me_workerproc_conf: mln_conf_item_t = mln_conf_item { conf_type: CONF_INT, val: mln_int_t(1) };

unsafe extern "C" fn master_ipc_handler(ev: *mut mln_event_t, f_ptr: *mut std::ffi::c_void, buf: *mut std::ffi::c_void, len: u32, pdata: *mut *mut std::ffi::c_void) {
    mln_log(debug, "received: [%s]", buf as *const i8);
    exit(0);
}

unsafe extern "C" fn worker_ipc_timer_handler(ev: *mut mln_event_t, data: *mut std::ffi::c_void) {
    exit(0);
}

unsafe extern "C" fn worker_ipc_handler(ev: *mut mln_event_t, f_ptr: *mut std::ffi::c_void, buf: *mut std::ffi::c_void, len: u32, pdata: *mut *mut std::ffi::c_void) {
    mln_log(debug, "received: [%s]", buf as *const i8);
    assert!(mln_ipc_worker_send_prepare(ev, 255, "bcd", 4) == 0);
    assert!(mln_event_timer_set(ev, 3000, None, worker_ipc_timer_handler) != ptr::null_mut());
}

unsafe fn global_init() -> i32 {
    let cf = mln_conf();
    let cd = cf.search("main");
    let mut cc = cd.search("framework");
    if cc.is_null() {
        cc = cd.insert("framework");
    }
    assert!(cc.update(&me_framework_conf, 1) == 0);

    cc = cd.search("worker_proc");
    if cc.is_null() {
        cc = cd.insert("worker_proc");
    }
    assert!(cc.update(&me_workerproc_conf, 1) == 0);

    ipc_cb = mln_ipc_handler_register(255, master_ipc_handler, worker_ipc_handler, ptr::null_mut(), ptr::null_mut());
    assert!(!ipc_cb.is_null());
    0
}

unsafe extern "C" fn child_iterator(ev: *mut mln_event_t, child: *mut mln_fork_t, data: *mut std::ffi::c_void) -> i32 {
    mln_ipc_master_send_prepare(ev, 255, "abc", 4, child)
}

unsafe extern "C" fn master_handler(ev: *mut mln_event_t) {
    assert!(mln_fork_iterate(ev, child_iterator, ptr::null_mut()) == 0);
}

fn main() {
    println!("NOTE: This test has memory leak because we don't release memory of log, conf and multiprocess-related stuffs.");
    println!("In fact, `mln_framework_init` should be the last function called in `main`, so we don't need to release anything.");

    let cattr = mln_framework_attr {
        argc: 0,
        argv: ptr::null_mut(),
        global_init: global_init,
        main_thread: None,
        master_process: master_handler,
        worker_process: None,
    };

    unsafe { mln_framework_init(&cattr) };
}
