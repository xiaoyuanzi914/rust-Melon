#[macro_use]
extern crate c2rust_bitfields;

mod mln_hash;

use std::ptr;
use std::ffi::CString;
use libc::free;

use crate::mln_hash::{mln_hash_new, mln_hash_insert, mln_hash_search, mln_hash_free, M_HASH_F_VAL, mln_hash_t, mln_hash_attr};

#[repr(C)]
struct mln_hash_test_t {
    key: i32,
    val: i32,
}

unsafe extern "C" fn calc_handler(h: *mut mln_hash_t, key: *mut libc::c_void) -> u64 {
    let key_ref = &*(key as *mut i32);
    (*h).len as u64 % (*key_ref) as u64
}

unsafe extern "C" fn cmp_handler(h: *mut mln_hash_t, key1: *mut libc::c_void, key2: *mut libc::c_void) -> i32 {
    let key1_ref = &*(key1 as *mut i32);
    let key2_ref = &*(key2 as *mut i32);
    if key1_ref == key2_ref {
        0
    } else {
        1
    }
}

unsafe extern "C" fn free_handler(val: *mut libc::c_void) {
    free(val);
}

fn main() {
    let mut hattr = mln_hash_attr {
        pool: ptr::null_mut(),
        pool_alloc: None,
        pool_free: None,
        hash: Some(calc_handler),
        cmp: Some(cmp_handler),
        key_freer: None,
        val_freer: Some(free_handler),
        len_base: 97,
        expandable: 0,
        calc_prime: 0,
    };

    let h = unsafe { mln_hash_new(&mut hattr) };
    if h.is_null() {
        eprintln!("Hash init failed.");
        return;
    }

    // Allocate and initialize a test item
    let item = Box::into_raw(Box::new(mln_hash_test_t {
        key: 1,
        val: 10,
    }));

    // Insert item into hash
    if unsafe { mln_hash_insert(h, &(*item).key as *const i32 as *mut i32, item as *mut libc::c_void) } < 0 {
        eprintln!("insert failed.");
        return;
    }

    // Search for the item
    let ret = unsafe { mln_hash_search(h, &(*item).key as *const i32 as *mut i32) };
    if !ret.is_null() {
        println!("Item found at pointer: {:?}, expected pointer: {:?}", ret, item);
    }

    // Free the hash table
    unsafe { mln_hash_free(h, M_HASH_F_VAL) };
}
