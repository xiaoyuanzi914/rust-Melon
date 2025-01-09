use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_list_s {
    pub next: *mut mln_list_s,
    pub prev: *mut mln_list_s,
}
pub type mln_list_t = mln_list_s;
#[inline]
unsafe extern "C" fn mln_list_chain_del(
    mut head: *mut *mut mln_list_t,
    mut tail: *mut *mut mln_list_t,
    mut node: *mut mln_list_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_list_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_list_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_list_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_list_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_list_chain_add(
    mut head: *mut *mut mln_list_t,
    mut tail: *mut *mut mln_list_t,
    mut node: *mut mln_list_t,
) {
    (*node).next = 0 as *mut mln_list_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[no_mangle]
pub unsafe extern "C" fn mln_list_add(
    mut sentinel: *mut mln_list_t,
    mut node: *mut mln_list_t,
) {
    mln_list_chain_add(&mut (*sentinel).prev, &mut (*sentinel).next, node);
}
#[no_mangle]
pub unsafe extern "C" fn mln_list_remove(
    mut sentinel: *mut mln_list_t,
    mut node: *mut mln_list_t,
) {
    mln_list_chain_del(&mut (*sentinel).prev, &mut (*sentinel).next, node);
}
