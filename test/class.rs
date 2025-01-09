mod mln_class_bindings { 
    include!("mln_class_bindings.rs"); 
}

use mln_class_bindings::*;
use std::ffi::CStr;
use std::ptr;
use std::io::{self, Write};

// 定义 class 结构体及其方法
#[repr(C)]
pub struct F {
    a: i32,
    f: Option<unsafe extern "C" fn(*mut F)>,
}

impl F {
    fn new(a: i32, func: unsafe extern "C" fn(*mut F)) -> *mut F {
        unsafe {
            let mut obj = Box::new(F { a, f: Some(func) });
            let ptr = Box::into_raw(obj);
            _constructor(ptr, a, func);
            ptr
        }
    }

    fn delete(ptr: *mut F) {
        unsafe {
            _destructor(ptr);
            if !ptr.is_null() {
                Box::from_raw(ptr); // 自动释放内存
            }
        }
    }
}

// 构造函数
unsafe extern "C" fn _constructor(o: *mut F, a: i32, func: unsafe extern "C" fn(*mut F)) {
    println!("constructor");
    (*o).a = a;
    (*o).f = Some(func);
}

// 析构函数
unsafe extern "C" fn _destructor(o: *mut F) {
    println!("destructor");
}

// 测试调用函数
unsafe extern "C" fn fcall(o: *mut F) {
    println!("in function call");
}

fn main() {
    unsafe {
        // 创建对象并调用函数
        let f = F::new(1, fcall);
        println!("{}", (*f).a);
        (*f).f.unwrap()(f);
        F::delete(f);
    }
}
