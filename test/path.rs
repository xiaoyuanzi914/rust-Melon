#[macro_use]
extern crate c2rust_bitfields;

mod mln_path;

use std::ffi::CStr;
use std::ptr;
use std::assert;

static CONF_PATH: &str = "/tmp/a.conf";

// 模拟 C 中的静态函数 return_conf_path
fn return_conf_path() -> *const i8 {
    std::ffi::CString::new(CONF_PATH).unwrap().into_raw()
}

fn main() {
    unsafe {
        // 设置路径钩子
        mln_path::mln_path_hook_set("m_p_conf", return_conf_path);

        // 校验路径
        let path = mln_path::mln_path_conf();
        assert!(!CStr::from_ptr(path).to_str().unwrap().is_empty());
        assert_eq!(CStr::from_ptr(path).to_str().unwrap(), CONF_PATH);
    }
}
