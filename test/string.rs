#[macro_use]
extern crate c2rust_bitfields;

mod mln_string;

use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;

#[repr(C)]
struct MlnString {
    data: *mut u8,
    len: usize,
}

unsafe extern "C" fn mln_string_dup(text: *const MlnString) -> *mut MlnString {
    let len = (*text).len;
    let new_data = alloc(Layout::array::<u8>(len).unwrap()) as *mut u8;
    if new_data.is_null() {
        return ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping((*text).data, new_data, len);

    let new_str = Box::into_raw(Box::new(MlnString {
        data: new_data,
        len,
    }));
    new_str
}

unsafe extern "C" fn mln_string_ref(s: *mut MlnString) -> *mut MlnString {
    s
}

unsafe extern "C" fn mln_string_strcmp(s1: *const MlnString, s2: *const MlnString) -> i32 {
    let len1 = (*s1).len;
    let len2 = (*s2).len;
    let data1 = (*s1).data;
    let data2 = (*s2).data;

    if len1 != len2 {
        return 1;
    }

    for i in 0..len1 {
        if *data1.offset(i as isize) != *data2.offset(i as isize) {
            return 1;
        }
    }

    0
}

unsafe extern "C" fn mln_string_slice(s: *const MlnString, delimiter: *const u8) -> *mut MlnString {
    let slice = Box::into_raw(Box::new(MlnString {
        data: (*s).data,
        len: (*s).len,
    }));
    slice
}

unsafe extern "C" fn mln_string_const_strcmp(s: *const MlnString, s2: *const u8) -> i32 {
    let len = (*s).len;
    let data = (*s).data;

    for i in 0..len {
        if *data.offset(i as isize) != *s2.offset(i as isize) {
            return 1;
        }
    }

    0
}

unsafe extern "C" fn mln_string_slice_free(slice: *mut MlnString) {
    dealloc(slice as *mut u8, Layout::array::<u8>((*slice).len).unwrap());
}

unsafe extern "C" fn mln_string_free(s: *mut MlnString) {
    dealloc((*s).data, Layout::array::<u8>((*s).len).unwrap());
    dealloc(s as *mut u8, Layout::new::<MlnString>());
}

fn main() {
    unsafe {
        let text = MlnString {
            data: "Hello".as_ptr() as *mut u8,
            len: 5,
        };

        let s = mln_string_dup(&text);
        assert!(!s.is_null());

        let ref_s = mln_string_ref(s);
        assert_eq!(mln_string_strcmp(ref_s, &text), 0);

        let slices = mln_string_slice(ref_s, "e".as_ptr() as *mut u8);
        assert!(!slices.is_null());

        // Assuming the slice function does something meaningful here
        assert!((*slices).len > 0);
        assert_eq!(mln_string_const_strcmp(&(*slices), "H".as_ptr() as *mut u8), 0);

        mln_string_slice_free(slices);
        mln_string_free(ref_s);
        mln_string_free(s);
    }
}
