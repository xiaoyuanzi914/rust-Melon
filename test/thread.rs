#[macro_use]
extern crate c2rust_bitfields;

mod mln_framework;
mod mln_log;
mod mln_thread;

use std::ptr;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::RawFd;
use std::thread::sleep;
use std::time::Duration;

static ME_FRAMEWORK_MODE: &str = "multithread";
static ME_THREAD_HELLO_STR: &str = "hello";
static ME_THREAD_HAHA_STR: &str = "haha";
static ME_WORKERPROC_CONF: i32 = 1;

fn global_init() -> i32 {
    unsafe {
        let cf = mln_framework::mln_conf();
        let cd = cf.search(cf, "main");
        let mut cc = cd.search(cd, "framework");

        if cc.is_null() {
            cc = cd.insert(cd, "framework");
        }
        assert!(!cc.is_null());
        assert!(cc.update(cc, &ME_FRAMEWORK_MODE, 1) == 0);

        cc = cd.search(cd, "worker_proc");
        if cc.is_null() {
            cc = cd.insert(cd, "worker_proc");
        }
        assert!(!cc.is_null());
        assert!(cc.update(cc, &ME_WORKERPROC_CONF, 1) == 0);

        let cd = cf.search(cf, "thread_exec");
        if cd.is_null() {
            cd.insert(cd, "thread_exec");
        }
        assert!(!cd.is_null());

        let cc = cd.insert(cd, "default");
        assert!(!cc.is_null());
        assert!(cc.update(cc, &ME_THREAD_HELLO_STR, 1) == 0);

        let cc = cd.insert(cd, "default");
        assert!(!cc.is_null());
        assert!(cc.update(cc, &ME_THREAD_HAHA_STR, 1) == 0);

        0
    }
}

fn haha(argc: i32, argv: Vec<String>) -> i32 {
    let fd = argv[argc as usize - 1].parse::<i32>().unwrap();
    let mut msg: mln_thread::mln_thread_msg_t = unsafe { std::mem::zeroed() };
    let mut rdset = unsafe { std::mem::zeroed::<libc::fd_set>() };
    unsafe { libc::FD_ZERO(&mut rdset) };
    unsafe { libc::FD_SET(fd, &mut rdset) };

    let nfds = unsafe { libc::select(fd + 1, &mut rdset, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) };
    if nfds < 0 {
        mln_log::mln_log_error("select error", std::ffi::CStr::from_ptr(libc::strerror(libc::errno())).to_str().unwrap());
        return -1;
    }

    let n = unsafe { libc::recv(fd, &mut msg as *mut _ as *mut libc::c_void, std::mem::size_of::<mln_thread::mln_thread_msg_t>(), 0) };
    if n != std::mem::size_of::<mln_thread::mln_thread_msg_t>() as i32 {
        mln_log::mln_log_debug("recv error", format!("n={}: {}", n, std::ffi::CStr::from_ptr(libc::strerror(libc::errno())).to_str().unwrap()).as_str());
        return -1;
    }
    mln_log::mln_log_debug("src:", &format!("{} auto:{} char:{}", msg.src, msg.sauto, msg.c));
    mln_thread::mln_thread_clear_msg(&msg);

    0
}

fn hello_cleanup(_data: *mut libc::c_void) {
    mln_log::mln_log_debug("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@", "");
}

fn hello(argc: i32, argv: Vec<String>) -> i32 {
    mln_thread::mln_thread_cleanup_set(hello_cleanup, ptr::null_mut());
    for _i in 0..1 {
        let fd = argv[argc as usize - 1].parse::<i32>().unwrap();
        let mut msg: mln_thread::mln_thread_msg_t = unsafe { std::mem::zeroed() };
        msg.dest = mln_thread::mln_string_new("haha");
        assert!(!msg.dest.is_null());
        msg.sauto = 9736;
        msg.c = 'N' as i8;
        msg.type_ = mln_thread::ITC_REQUEST;
        msg.need_clear = 1;
        let n = unsafe { libc::send(fd, &msg as *const _ as *const libc::c_void, std::mem::size_of::<mln_thread::mln_thread_msg_t>(), 0) };
        if n != std::mem::size_of::<mln_thread::mln_thread_msg_t>() as i32 {
            mln_log::mln_log_debug("send error", format!("n={}: {}", n, std::ffi::CStr::from_ptr(libc::strerror(libc::errno())).to_str().unwrap()).as_str());
            mln_thread::mln_string_free(msg.dest);
            return -1;
        }
    }
    sleep(Duration::new(1, 0));
    std::process::exit(0);
    0
}

fn master_handler(_ev: *mut mln_framework::mln_event_t) {
    sleep(Duration::new(0, 100000));
    std::process::exit(0);
}

fn main(argc: i32, argv: Vec<String>) -> i32 {
    println!("NOTE: This test has memory leak because we don't release memory of log, conf, multiprocess-related and multithread-related stuffs.");
    println!("In fact, mln_framework_init should be the last function called in main, so we don't need to release anything.");

    let modules = vec![
        mln_framework::mln_thread_module_t {
            name: CString::new("haha").unwrap(),
            func: haha,
        },
        mln_framework::mln_thread_module_t {
            name: CString::new("hello").unwrap(),
            func: hello,
        },
    ];

    let cattr = mln_framework::mln_framework_attr {
        argc,
        argv: argv.as_ptr(),
        global_init: global_init,
        main_thread: ptr::null_mut(),
        worker_process: ptr::null_mut(),
        master_process: master_handler,
    };

    mln_framework::mln_thread_module_set(modules.as_ptr(), 2);

    if mln_framework::mln_framework_init(&cattr) < 0 {
        eprintln!("Melon init failed.");
        return -1;
    }

    0
}
