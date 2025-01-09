#[macro_use]
extern crate c2rust_bitfields;

mod mln_event;  // 导入 mln_event.rs 文件

use crate::mln_event::{mln_event_new, mln_event_free, mln_event_timer_set, mln_event_fd_set, mln_event_dispatch, mln_event_break_set, mln_event_fd_set as mln_event_fd_clear};
use std::os::unix::io::RawFd;
use std::io::Write;
use std::ptr;

fn timer_handler(ev: *mut mln_event_t, data: *mut std::ffi::c_void) {
    println!("timer");
    unsafe { mln_event_break_set(ev) };
}

fn mln_fd_write(ev: *mut mln_event_t, fd: RawFd, data: *mut std::ffi::c_void) {
    println!("write handler");
    let _ = unsafe { libc::write(fd, b"hello\n" as *const u8 as *const libc::c_void, 6) };
    unsafe { mln_event_fd_clear(ev, fd, M_EV_CLR, M_EV_UNLIMITED, ptr::null_mut(), ptr::null_mut()) };
}

fn main() {
    let ev = unsafe { mln_event_new() };
    if ev.is_null() {
        eprintln!("event init failed.");
        return;
    }

    // Set timer event
    if unsafe { mln_event_timer_set(ev, 1000, ptr::null_mut(), Some(timer_handler)) } < 0 {
        eprintln!("timer set failed.");
        return;
    }

    // Set file descriptor event
    if unsafe { mln_event_fd_set(ev, libc::STDOUT_FILENO, M_EV_SEND, M_EV_UNLIMITED, ptr::null_mut(), Some(mln_fd_write)) } < 0 {
        eprintln!("fd handler set failed.");
        return;
    }

    // Dispatch the events
    unsafe { mln_event_dispatch(ev) };

    // Free the event object
    unsafe { mln_event_free(ev) };
}
