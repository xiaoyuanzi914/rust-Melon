#[macro_use]
extern crate c2rust_bitfields;

mod mln_lang;
mod mln_event;
mod mln_string;

use std::ptr;
use std::ffi::CString;
use libc::{socketpair, AF_UNIX, SOCK_STREAM, close};

use crate::mln_lang::{mln_lang_t, mln_lang_new, mln_lang_job_new, mln_lang_free, mln_lang_event_get, mln_lang_launcher_get};
use crate::mln_event::{mln_event_t, mln_event_new, mln_event_dispatch, mln_event_fd_set, mln_event_break_set};
use crate::mln_string::{mln_string, mln_string_free};

static mut fds: [i32; 2] = [-1, -1];

unsafe extern "C" fn lang_signal(lang: *mut mln_lang_t) -> i32 {
    let event = mln_lang_event_get(lang);
    let launcher = mln_lang_launcher_get(lang);
    mln_event_fd_set(event, fds[0], M_EV_SEND | M_EV_ONESHOT, M_EV_UNLIMITED, lang, launcher)
}

unsafe extern "C" fn lang_clear(lang: *mut mln_lang_t) -> i32 {
    let event = mln_lang_event_get(lang);
    mln_event_fd_set(event, fds[0], M_EV_CLR, M_EV_UNLIMITED, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn return_handler(ctx: *mut mln_lang_ctx_t) {
    let event = mln_lang_event_get((*ctx).lang);
    mln_event_break_set(event);
}

fn main() {
    // Initialize code and event handler
    let code = mln_string("Dump('Hello');");
    let mut ev: *mut mln_event_t;
    let mut lang: *mut mln_lang_t;

    // Create socketpair
    if unsafe { socketpair(AF_UNIX, SOCK_STREAM, 0, fds.as_mut_ptr()) } != 0 {
        eprintln!("socketpair error");
        return;
    }

    // Create event and lang objects
    unsafe {
        ev = mln_event_new();
        if ev.is_null() {
            eprintln!("Failed to create event");
            return;
        }

        lang = mln_lang_new(ev, lang_signal, lang_clear);
        if lang.is_null() {
            eprintln!("Failed to create lang");
            return;
        }

        if mln_lang_job_new(lang, ptr::null_mut(), M_INPUT_T_BUF, &code, ptr::null_mut(), return_handler) != 0 {
            eprintln!("Failed to create lang job");
            return;
        }

        // Dispatch event
        mln_event_dispatch(ev);

        // Clean up
        mln_lang_free(lang);
        mln_event_free(ev);
        close(fds[0]);
        close(fds[1]);
    }
}
