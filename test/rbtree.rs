#[macro_use]
extern crate c2rust_bitfields;

mod mln_rbtree;

use std::ffi::CStr;
use std::ptr;
use std::assert;

static mut CONF_PATH: *const libc::c_char = ptr::null_mut();

unsafe fn cmp_handler(data1: *const libc::c_void, data2: *const libc::c_void) -> libc::c_int {
    let d1 = *(data1 as *const i32);
    let d2 = *(data2 as *const i32);
    d1 - d2
}

fn test1() -> i32 {
    let n = 10;
    let t: *mut mln_rbtree::mln_rbtree_t;
    let mut rn: *mut mln_rbtree::mln_rbtree_node_t;

    unsafe {
        t = mln_rbtree::mln_rbtree_new(ptr::null_mut());
        assert!(!t.is_null());

        rn = mln_rbtree::mln_rbtree_node_new(t, &n as *const i32 as *const libc::c_void);
        assert!(!rn.is_null());

        mln_rbtree::mln_rbtree_inline_insert(t, rn, Some(cmp_handler));

        rn = mln_rbtree::mln_rbtree_inline_search(t, &n as *const i32 as *const libc::c_void, Some(cmp_handler));
        assert!(!mln_rbtree::mln_rbtree_null(rn, t));

        println!("{}", *(mln_rbtree::mln_rbtree_node_data_get(rn) as *const i32));

        mln_rbtree::mln_rbtree_delete(t, rn);
        mln_rbtree::mln_rbtree_node_free(t, rn);
        mln_rbtree::mln_rbtree_free(t);
    }

    0
}

#[repr(C)]
struct Ud {
    val: i32,
    node: mln_rbtree::mln_rbtree_node_t,
}

unsafe fn cmp2_handler(data1: *const libc::c_void, data2: *const libc::c_void) -> libc::c_int {
    let d1 = *(data1 as *const Ud);
    let d2 = *(data2 as *const Ud);
    d1.val - d2.val
}

fn test2() -> i32 {
    let t: *mut mln_rbtree::mln_rbtree_t;
    let mut data1 = Ud { val: 1, node: mln_rbtree::mln_rbtree_node_t::default() };
    let mut data2 = Ud { val: 2, node: mln_rbtree::mln_rbtree_node_t::default() };
    let mut rn: *mut mln_rbtree::mln_rbtree_node_t;
    let rbattr = mln_rbtree::mln_rbtree_attr_t {
        pool: ptr::null_mut(),
        pool_alloc: None,
        pool_free: None,
        cmp: Some(cmp2_handler),
        data_free: None,
    };

    unsafe {
        t = mln_rbtree::mln_rbtree_new(&rbattr);
        assert!(!t.is_null());

        mln_rbtree::mln_rbtree_node_init(&mut data1.node, &data1 as *const Ud as *const libc::c_void);
        mln_rbtree::mln_rbtree_node_init(&mut data2.node, &data2 as *const Ud as *const libc::c_void);

        mln_rbtree::mln_rbtree_insert(t, &mut data1.node);
        mln_rbtree::mln_rbtree_insert(t, &mut data2.node);

        rn = mln_rbtree::mln_rbtree_search(t, &data1);
        assert!(!mln_rbtree::mln_rbtree_null(rn, t));

        let rndata = mln_rbtree::mln_rbtree_node_data_get(rn) as *const Ud;
        println!("{}", (*rndata).val);

        let container = mln_rbtree::mln_container_of(rn, Ud, node);
        println!("{}", (*container).val);

        mln_rbtree::mln_rbtree_delete(t, &data1.node);
        mln_rbtree::mln_rbtree_node_free(t, &data1.node);
        mln_rbtree::mln_rbtree_free(t);
    }

    0
}

fn main() {
    assert_eq!(test1(), 0);
    assert_eq!(test2(), 0);
}
