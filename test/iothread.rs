#[macro_use]
extern crate c2rust_bitfields;

mod mln_iothread;
mod mln_iothread_msg;
mod mln_iothread_entry;

use std::thread;
use std::time::Duration;
use libc::sleep;
use crate::mln_iothread::{mln_iothread_t, mln_iothread_init, mln_iothread_recv, mln_iothread_send, mln_iothread_destroy};
use crate::mln_iothread_msg::{mln_iothread_msg_type, mln_iothread_msg_t};
use crate::mln_iothread_entry::{mln_iothread_entry_t, mln_iothread_msg_process_t};

static USER_THREAD: i32 = 1;
static IO_THREAD: i32 = 0;

unsafe extern "C" fn msg_handler(t: *mut mln_iothread_t, from: i32, msg: *mut mln_iothread_msg_t) {
    let type_ = mln_iothread_msg_type(msg);
    println!("msg type: {}", type_);
}

unsafe extern "C" fn entry(args: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let t = args as *mut mln_iothread_t;
    loop {
        let n = mln_iothread_recv(t, USER_THREAD);
        println!("recv {} message(s)", n);
    }
}

fn main() {
    unsafe {
        let mut t: mln_iothread_t = std::mem::zeroed();

        if mln_iothread_init(&mut t, 1, entry, &mut t as *mut _, msg_handler) < 0 {
            eprintln!("iothread init failed");
            return;
        }

        for i in 0..1000 {
            let rc = mln_iothread_send(&mut t, i, ptr::null_mut(), IO_THREAD, 1);
            if rc < 0 {
                eprintln!("send failed");
                return;
            } else if rc > 0 {
                continue;
            }
        }

        sleep(1);
        mln_iothread_destroy(&mut t);
        sleep(3);
        println!("DONE");
    }
}
