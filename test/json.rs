#[macro_use]
extern crate c2rust_bitfields;

mod mln_string;
mod mln_json;

use std::ffi::CString;
use std::ptr;
use std::process::exit;
use std::io::Write;
use libc::{STDOUT_FILENO, write};

use crate::mln_string::{mln_string, mln_string_free};
use crate::mln_json::{
    mln_json_t, mln_json_number_init, mln_json_decode, mln_json_parse, mln_json_dump, 
    mln_json_generate, mln_json_encode, mln_json_init, mln_json_destroy
};

static mut ipc_cb: *mut mln_json_t = ptr::null_mut();

unsafe extern "C" fn callback(j: *mut mln_json_t, data: *mut std::ffi::c_void) -> i32 {
    let val = *(data as *mut i32);
    mln_json_number_init(j, val);
    0
}

unsafe extern "C" fn handler(j: *mut mln_json_t, data: *mut std::ffi::c_void) -> i32 {
    mln_json_dump(j, 0, CString::new("").unwrap().as_ptr());
    0
}

fn main() {
    let i: i32 = 1024;
    let mut j: mln_json_t;
    let mut k: mln_json_t;
    let mut ca: mln_json_call_attr;
    let mut res: *mut mln_string_t;
    let exp = mln_string("protocols.0");
    let tmp = mln_string("{\"paths\":[\"/mock\"],\"methods\":null,\"sources\":null,\"destinations\":null,\"name\":\"example_route\",\"headers\":null,\"hosts\":null,\"preserve_host\":false,\"regex_priority\":0,\"snis\":null,\"https_redirect_status_code\":426,\"tags\":null,\"protocols\":[\"http\",\"https\"],\"path_handling\":\"v0\",\"id\":\"52d58293-ae25-4c69-acc8-6dd729718a61\",\"updated_at\":1661345592,\"service\":{\"id\":\"c1e98b2b-6e77-476c-82ca-a5f1fb877e07\"},\"response_buffering\":true,\"strip_path\":true,\"request_buffering\":true,\"created_at\":1661345592}");

    // Decode JSON
    if unsafe { mln_json_decode(&tmp, &mut j, ptr::null_mut()) } < 0 {
        eprintln!("decode error");
        return;
    }

    unsafe {
        mln_json_parse(&mut j, &exp, handler, ptr::null_mut());
    }

    // Prepare callback attributes
    ca.callback = callback;
    ca.data = &i as *const i32 as *mut std::ffi::c_void;

    unsafe {
        mln_json_init(&mut k);
        if mln_json_generate(&mut k, "[{s:d,s:d,s:{s:d}},d,[],j,c]", 
                            "a", 1, "b", 3, "c", "d", 4, 5, &j, &ca) < 0 {
            eprintln!("generate failed");
            return;
        }

        mln_json_generate(&mut k, "[s,d]", "g", 99);

        // Encode JSON result
        res = mln_json_encode(&mut k);

        mln_json_destroy(&mut k);
    }

    if res.is_null() {
        eprintln!("encode failed");
        return;
    }

    unsafe {
        write(STDOUT_FILENO, (*res).data as *const u8, (*res).len);
        write(STDOUT_FILENO, "\n".as_ptr(), 1);
        mln_string_free(res);
    }
}
