#[macro_use]
extern crate c2rust_bitfields;

mod mln_expr;
mod mln_log;

use std::ffi::CString;
use std::ptr;

use crate::mln_expr::{mln_expr_val_t, mln_expr_val_new, mln_expr_val_free, mln_expr_run};
use crate::mln_log::mln_log;
use crate::mln_expr::{mln_string_t, mln_string};

static ANON: mln_string_t = mln_string_t { data: b"anonymous namespace".as_ptr() as *const i8, len: 23 };

unsafe fn var_expr_handler(namespace: *mut mln_string_t, name: *mut mln_string_t, is_func: i32, args: *mut mln_array_t, data: *mut libc::c_void) -> *mut mln_expr_val_t {
    let mut s: *mut mln_string_t = ptr::null_mut();
    let mut ret: *mut mln_expr_val_t = ptr::null_mut();
    let anon = &ANON;
    
    println!("{:?} {:?} {:?} {:?}", namespace, name, args, data);

    if is_func != 0 {
        mln_log(0, "%S %S %d %U %X", 
                if !namespace.is_null() { namespace } else { anon },
                name, 
                is_func, 
                (*args).nelts, 
                data);
    } else {
        mln_log(0, "%S %S %d %X", 
                if !namespace.is_null() { namespace } else { anon }, 
                name, 
                is_func, 
                data);
    }

    if !name.is_null() {
        s = mln_string_dup(name);
    }
    if s.is_null() {
        return ptr::null_mut();
    }

    ret = mln_expr_val_new(mln_expr_type_string, s, ptr::null_mut());
    mln_string_free(s);
    return ret;
}

unsafe fn func_expr_handler(namespace: *mut mln_string_t, name: *mut mln_string_t, is_func: i32, args: *mut mln_array_t, data: *mut libc::c_void) -> *mut mln_expr_val_t {
    let mut v: *mut mln_expr_val_t = ptr::null_mut();
    let mut p: *mut mln_expr_val_t = ptr::null_mut();
    let mut i: i32 = 0;
    let mut s1: *mut mln_string_t = ptr::null_mut();
    let mut s2: *mut mln_string_t = ptr::null_mut();
    let mut s3: *mut mln_string_t = ptr::null_mut();
    let anon = &ANON;

    if is_func != 0 {
        mln_log(0, "%S %S %d %U %X", 
                if !namespace.is_null() { namespace } else { anon }, 
                name, 
                is_func, 
                (*args).nelts, 
                data);
    } else {
        mln_log(0, "%S %S %d %X", 
                if !namespace.is_null() { namespace } else { anon }, 
                name, 
                is_func, 
                data);
        return mln_expr_val_new(mln_expr_type_string, name, ptr::null_mut());
    }

    for i in 0..(*args).nelts {
        v = (*args).elts.offset(i as isize);
        if s1.is_null() {
            s1 = mln_string_ref((*v).data.s);
            continue;
        }
        s2 = (*v).data.s;
        s3 = mln_string_strcat(s1, s2);
        mln_string_free(s1);
        s1 = s3;
    }

    v = mln_expr_val_new(mln_expr_type_string, s1, ptr::null_mut());
    mln_string_free(s1);

    return v;
}

fn main() {
    let var_exp = mln_string(":aaa (a:b:c:bbb)");
    let func_exp = mln_string("abc:def:concat('abc', concat(aaa, 'bbb')) ccc concat('eee', concat(bbb, 'fff'))");

    let mut v: *mut mln_expr_val_t;

    unsafe {
        v = mln_expr_run(&var_exp, var_expr_handler, ptr::null_mut());
        if v.is_null() {
            mln_log(1, "run failed");
            return;
        }
        mln_log(2, "%d %S", (*v).type_, (*v).data.s);
        mln_expr_val_free(v);

        v = mln_expr_run(&func_exp, func_expr_handler, ptr::null_mut());
        if v.is_null() {
            mln_log(1, "run failed");
            return;
        }
        mln_log(2, "%d %S", (*v).type_, (*v).data.s);
        mln_expr_val_free(v);
    }
}
