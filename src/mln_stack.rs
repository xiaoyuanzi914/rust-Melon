use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type mln_uauto_t = libc::c_ulong;
pub type stack_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type stack_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
pub type stack_iterate_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_stack_node_s {
    pub data: *mut libc::c_void,
    pub prev: *mut mln_stack_node_s,
    pub next: *mut mln_stack_node_s,
}
pub type mln_stack_node_t = mln_stack_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_stack_t {
    pub bottom: *mut mln_stack_node_t,
    pub top: *mut mln_stack_node_t,
    pub free_head: *mut mln_stack_node_t,
    pub free_tail: *mut mln_stack_node_t,
    pub nr_node: mln_uauto_t,
    pub free_handler: stack_free,
    pub copy_handler: stack_copy,
}
unsafe extern "C" fn mln_stack_node_init(
    mut data: *mut libc::c_void,
) -> *mut mln_stack_node_t {
    let mut sn: *mut mln_stack_node_t = malloc(
        ::core::mem::size_of::<mln_stack_node_t>() as libc::c_ulong,
    ) as *mut mln_stack_node_t;
    if sn.is_null() {
        return 0 as *mut mln_stack_node_t;
    }
    (*sn).prev = 0 as *mut mln_stack_node_s;
    (*sn).next = 0 as *mut mln_stack_node_s;
    (*sn).data = data;
    return sn;
}
unsafe extern "C" fn mln_stack_node_destroy(
    mut st: *mut mln_stack_t,
    mut free_handler: stack_free,
    mut sn: *mut mln_stack_node_t,
) {
    if sn.is_null() {
        return;
    }
    if free_handler.is_some() {
        free_handler.expect("non-null function pointer")((*sn).data);
    }
    free(sn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_init(
    mut free_handler: stack_free,
    mut copy_handler: stack_copy,
) -> *mut mln_stack_t {
    let mut st: *mut mln_stack_t = malloc(
        ::core::mem::size_of::<mln_stack_t>() as libc::c_ulong,
    ) as *mut mln_stack_t;
    if st.is_null() {
        return 0 as *mut mln_stack_t;
    }
    (*st).bottom = 0 as *mut mln_stack_node_t;
    (*st).top = 0 as *mut mln_stack_node_t;
    (*st).nr_node = 0 as libc::c_int as mln_uauto_t;
    (*st).free_handler = free_handler;
    (*st).copy_handler = copy_handler;
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_destroy(mut st: *mut mln_stack_t) {
    if st.is_null() {
        return;
    }
    let mut sn: *mut mln_stack_node_t = 0 as *mut mln_stack_node_t;
    loop {
        sn = (*st).bottom;
        if sn.is_null() {
            break;
        }
        mln_stack_chain_del(&mut (*st).bottom, &mut (*st).top, sn);
        mln_stack_node_destroy(st, (*st).free_handler, sn);
    }
    free(st as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_stack_chain_add(
    mut head: *mut *mut mln_stack_node_t,
    mut tail: *mut *mut mln_stack_node_t,
    mut node: *mut mln_stack_node_t,
) {
    (*node).next = 0 as *mut mln_stack_node_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_stack_chain_del(
    mut head: *mut *mut mln_stack_node_t,
    mut tail: *mut *mut mln_stack_node_t,
    mut node: *mut mln_stack_node_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_stack_node_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_stack_node_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_stack_node_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_stack_node_s;
    (*node).prev = (*node).next;
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_push(
    mut st: *mut mln_stack_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut sn: *mut mln_stack_node_t = 0 as *mut mln_stack_node_t;
    sn = mln_stack_node_init(data);
    if sn.is_null() {
        return -(1 as libc::c_int);
    }
    mln_stack_chain_add(&mut (*st).bottom, &mut (*st).top, sn);
    (*st).nr_node = ((*st).nr_node).wrapping_add(1);
    (*st).nr_node;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_pop(mut st: *mut mln_stack_t) -> *mut libc::c_void {
    let mut sn: *mut mln_stack_node_t = (*st).top;
    if sn.is_null() {
        return 0 as *mut libc::c_void;
    }
    mln_stack_chain_del(&mut (*st).bottom, &mut (*st).top, sn);
    (*st).nr_node = ((*st).nr_node).wrapping_sub(1);
    (*st).nr_node;
    let mut ptr: *mut libc::c_void = (*sn).data;
    mln_stack_node_destroy(st, None, sn);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_dup(
    mut st: *mut mln_stack_t,
    mut udata: *mut libc::c_void,
) -> *mut mln_stack_t {
    let mut new_st: *mut mln_stack_t = mln_stack_init(
        (*st).free_handler,
        (*st).copy_handler,
    );
    if new_st.is_null() {
        return 0 as *mut mln_stack_t;
    }
    let mut scan: *mut mln_stack_node_t = 0 as *mut mln_stack_node_t;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    scan = (*st).bottom;
    while !scan.is_null() {
        if ((*new_st).copy_handler).is_none() {
            data = (*scan).data;
        } else {
            data = ((*new_st).copy_handler)
                .expect("non-null function pointer")((*scan).data, udata);
            if data.is_null() {
                mln_stack_destroy(new_st);
                return 0 as *mut mln_stack_t;
            }
        }
        if mln_stack_push(new_st, data) < 0 as libc::c_int {
            if ((*new_st).free_handler).is_some() {
                ((*new_st).free_handler).expect("non-null function pointer")(data);
            }
            mln_stack_destroy(new_st);
            return 0 as *mut mln_stack_t;
        }
        scan = (*scan).next;
    }
    return new_st;
}
#[no_mangle]
pub unsafe extern "C" fn mln_stack_iterate(
    mut st: *mut mln_stack_t,
    mut handler: stack_iterate_handler,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut sn: *mut mln_stack_node_t = 0 as *mut mln_stack_node_t;
    sn = (*st).top;
    while !sn.is_null() {
        if handler.expect("non-null function pointer")((*sn).data, data)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        sn = (*sn).prev;
    }
    return 0 as libc::c_int;
}
