#[macro_use]
extern crate c2rust_bitfields;

mod mln_thread_pool;

use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::alloc::{alloc, dealloc, Layout};
use std::{thread, time};
use std::sync::{Arc, Mutex};

#[repr(C)]
struct MlnThreadPoolAttr {
    main_data: *mut c_void,
    child_process_handler: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    main_process_handler: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    free_handler: Option<unsafe extern "C" fn(*mut c_void)>,
    cond_timeout: i32,
    max: i32,
    concurrency: i32,
}

unsafe extern "C" fn child_process_handler(data: *mut c_void) -> i32 {
    let text = data as *mut i8;
    let c_str = CStr::from_ptr(text);
    println!("{}", c_str.to_str().unwrap());
    dealloc(text as *mut u8, Layout::new::<i8>());
    0
}

unsafe extern "C" fn main_process_handler(data: *mut c_void) -> i32 {
    let mut i = 0;
    loop {
        let text = alloc(Layout::new::<i8>()) as *mut i8;
        if text.is_null() {
            return -1;
        }
        let n = snprintf(text, 15, "hello world");
        *text.offset(n as isize) = 0;
        mln_thread_pool::mln_thread_pool_resource_add(text as *mut c_void);
        thread::sleep(time::Duration::from_millis(1));

        if i >= 20 {
            break;
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn free_handler(data: *mut c_void) {
    dealloc(data as *mut u8, Layout::new::<i8>());
}

fn main() {
    unsafe {
        let tpattr = MlnThreadPoolAttr {
            main_data: ptr::null_mut(),
            child_process_handler: Some(child_process_handler),
            main_process_handler: Some(main_process_handler),
            free_handler: Some(free_handler),
            cond_timeout: 1,
            max: 1,
            concurrency: 1,
        };
        mln_thread_pool::mln_thread_pool_run(&tpattr);
    }
}
