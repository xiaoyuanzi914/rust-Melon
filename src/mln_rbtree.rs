use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type mln_u32_t = libc::c_uint;
pub type mln_size_t = size_t;
pub type mln_uauto_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_rbtree_node_s {
    pub data: *mut libc::c_void,
    pub prev: *mut mln_rbtree_node_s,
    pub next: *mut mln_rbtree_node_s,
    pub parent: *mut mln_rbtree_node_s,
    pub left: *mut mln_rbtree_node_s,
    pub right: *mut mln_rbtree_node_s,
    #[bitfield(name = "nofree", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "color", ty = "mln_u32_t", bits = "1..=31")]
    pub nofree_color: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_rbtree_node_t = mln_rbtree_node_s;
pub type rbtree_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type rbtree_free_data = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type rbtree_iterate_handler = Option::<
    unsafe extern "C" fn(*mut mln_rbtree_node_t, *mut libc::c_void) -> libc::c_int,
>;
pub type rbtree_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type rbtree_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type rbtree_color = libc::c_uint;
pub const M_RB_BLACK: rbtree_color = 1;
pub const M_RB_RED: rbtree_color = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_rbtree_attr {
    pub pool: *mut libc::c_void,
    pub pool_alloc: rbtree_pool_alloc_handler,
    pub pool_free: rbtree_pool_free_handler,
    pub cmp: rbtree_cmp,
    pub data_free: rbtree_free_data,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct rbtree_s {
    pub pool: *mut libc::c_void,
    pub pool_alloc: rbtree_pool_alloc_handler,
    pub pool_free: rbtree_pool_free_handler,
    pub cmp: rbtree_cmp,
    pub data_free: rbtree_free_data,
    pub nil: mln_rbtree_node_t,
    pub root: *mut mln_rbtree_node_t,
    pub min: *mut mln_rbtree_node_t,
    pub head: *mut mln_rbtree_node_t,
    pub tail: *mut mln_rbtree_node_t,
    pub iter: *mut mln_rbtree_node_t,
    pub nr_node: mln_uauto_t,
    #[bitfield(name = "del", ty = "mln_u32_t", bits = "0..=0")]
    pub del: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mln_rbtree_t = rbtree_s;
#[inline]
unsafe extern "C" fn rbtree_insert_fixup(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    let mut tmp: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    while (*(*n).parent).color() as libc::c_int == M_RB_RED as libc::c_int {
        if (*n).parent == (*(*(*n).parent).parent).left {
            tmp = (*(*(*n).parent).parent).right;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*(*n).parent).parent;
            } else {
                if n == (*(*n).parent).right {
                    n = (*n).parent;
                    mln_rbtree_left_rotate(t, n);
                }
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_right_rotate(t, (*(*n).parent).parent);
            }
        } else {
            tmp = (*(*(*n).parent).parent).left;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*(*n).parent).parent;
            } else {
                if n == (*(*n).parent).left {
                    n = (*n).parent;
                    mln_rbtree_right_rotate(t, n);
                }
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_left_rotate(t, (*(*n).parent).parent);
            }
        }
    }
    (*(*t).root).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_rbtree_right_rotate(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    if (*n).left == &mut (*t).nil as *mut mln_rbtree_node_t {
        return;
    }
    let mut tmp: *mut mln_rbtree_node_t = (*n).left;
    (*n).left = (*tmp).right;
    if (*tmp).right != &mut (*t).nil as *mut mln_rbtree_node_t {
        (*(*tmp).right).parent = n;
    }
    (*tmp).parent = (*n).parent;
    if (*n).parent == &mut (*t).nil as *mut mln_rbtree_node_t {
        (*t).root = tmp;
    } else if n == (*(*n).parent).right {
        (*(*n).parent).right = tmp;
    } else {
        (*(*n).parent).left = tmp;
    }
    (*tmp).right = n;
    (*n).parent = tmp;
}
#[inline]
unsafe extern "C" fn mln_rbtree_left_rotate(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    if (*n).right == &mut (*t).nil as *mut mln_rbtree_node_t {
        return;
    }
    let mut tmp: *mut mln_rbtree_node_t = (*n).right;
    (*n).right = (*tmp).left;
    if (*tmp).left != &mut (*t).nil as *mut mln_rbtree_node_t {
        (*(*tmp).left).parent = n;
    }
    (*tmp).parent = (*n).parent;
    if (*n).parent == &mut (*t).nil as *mut mln_rbtree_node_t {
        (*t).root = tmp;
    } else if n == (*(*n).parent).left {
        (*(*n).parent).left = tmp;
    } else {
        (*(*n).parent).right = tmp;
    }
    (*tmp).left = n;
    (*n).parent = tmp;
}
#[inline]
unsafe extern "C" fn mln_rbtree_chain_del(
    mut head: *mut *mut mln_rbtree_node_t,
    mut tail: *mut *mut mln_rbtree_node_t,
    mut node: *mut mln_rbtree_node_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_rbtree_node_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_rbtree_node_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_rbtree_node_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_rbtree_node_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_rbtree_chain_add(
    mut head: *mut *mut mln_rbtree_node_t,
    mut tail: *mut *mut mln_rbtree_node_t,
    mut node: *mut mln_rbtree_node_t,
) {
    (*node).next = 0 as *mut mln_rbtree_node_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_new(
    mut attr: *mut mln_rbtree_attr,
) -> *mut mln_rbtree_t {
    let mut t: *mut mln_rbtree_t = 0 as *mut mln_rbtree_t;
    if attr.is_null() || ((*attr).pool).is_null() {
        t = malloc(::core::mem::size_of::<mln_rbtree_t>() as libc::c_ulong)
            as *mut mln_rbtree_t;
    } else {
        t = ((*attr).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*attr).pool, ::core::mem::size_of::<mln_rbtree_t>() as libc::c_ulong)
            as *mut mln_rbtree_t;
    }
    if t.is_null() {
        return 0 as *mut mln_rbtree_t;
    }
    if attr.is_null() {
        (*t).pool = 0 as *mut libc::c_void;
        (*t).pool_alloc = None;
        (*t).pool_free = None;
        (*t).cmp = None;
        (*t).data_free = None;
    } else {
        (*t).pool = (*attr).pool;
        (*t).pool_alloc = (*attr).pool_alloc;
        (*t).pool_free = (*attr).pool_free;
        (*t).cmp = (*attr).cmp;
        (*t).data_free = (*attr).data_free;
    }
    (*t).nil.data = 0 as *mut libc::c_void;
    (*t).nil.parent = &mut (*t).nil;
    (*t).nil.left = &mut (*t).nil;
    (*t).nil.right = &mut (*t).nil;
    ((*t).nil).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
    (*t).root = &mut (*t).nil;
    (*t).min = &mut (*t).nil;
    (*t).tail = 0 as *mut mln_rbtree_node_t;
    (*t).head = (*t).tail;
    (*t).iter = 0 as *mut mln_rbtree_node_t;
    (*t).nr_node = 0 as libc::c_int as mln_uauto_t;
    (*t).set_del(0 as libc::c_int as mln_u32_t);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_free(mut t: *mut mln_rbtree_t) {
    if t.is_null() {
        return;
    }
    let mut fr: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    loop {
        fr = (*t).tail;
        if fr.is_null() {
            break;
        }
        mln_rbtree_chain_del(&mut (*t).head, &mut (*t).tail, fr);
        if ((*t).data_free).is_some() {
            let mut nofree: mln_u32_t = (*fr).nofree();
            if !((*fr).data).is_null() {
                ((*t).data_free).expect("non-null function pointer")((*fr).data);
            }
            if nofree == 0 {
                if !((*t).pool).is_null() {
                    ((*t).pool_free)
                        .expect("non-null function pointer")(fr as *mut libc::c_void);
                } else {
                    free(fr as *mut libc::c_void);
                }
            }
        } else {
            mln_rbtree_node_free(t, fr);
        }
    }
    if !((*t).pool).is_null() {
        ((*t).pool_free).expect("non-null function pointer")(t as *mut libc::c_void);
    } else {
        free(t as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_reset(mut t: *mut mln_rbtree_t) {
    let mut fr: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    loop {
        fr = (*t).tail;
        if fr.is_null() {
            break;
        }
        mln_rbtree_chain_del(&mut (*t).head, &mut (*t).tail, fr);
        if ((*t).data_free).is_some() {
            let mut nofree: mln_u32_t = (*fr).nofree();
            if !((*fr).data).is_null() {
                ((*t).data_free).expect("non-null function pointer")((*fr).data);
            }
            if nofree == 0 {
                if !((*t).pool).is_null() {
                    ((*t).pool_free)
                        .expect("non-null function pointer")(fr as *mut libc::c_void);
                } else {
                    free(fr as *mut libc::c_void);
                }
            }
        } else {
            mln_rbtree_node_free(t, fr);
        }
    }
    (*t).root = &mut (*t).nil;
    (*t).min = &mut (*t).nil;
    (*t).iter = 0 as *mut mln_rbtree_node_t;
    (*t).nr_node = 0 as libc::c_int as mln_uauto_t;
    (*t).set_del(0 as libc::c_int as mln_u32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_insert(
    mut t: *mut mln_rbtree_t,
    mut node: *mut mln_rbtree_node_t,
) {
    let mut y: *mut mln_rbtree_node_t = &mut (*t).nil;
    let mut x: *mut mln_rbtree_node_t = (*t).root;
    let mut nil: *mut mln_rbtree_node_t = &mut (*t).nil;
    while x != nil {
        y = x;
        if ((*t).cmp).expect("non-null function pointer")((*node).data, (*x).data)
            < 0 as libc::c_int
        {
            x = (*x).left;
        } else {
            x = (*x).right;
        }
    }
    (*node).parent = y;
    if y == nil {
        (*t).root = node;
    } else if ((*t).cmp).expect("non-null function pointer")((*node).data, (*y).data)
        < 0 as libc::c_int
    {
        (*y).left = node;
    } else {
        (*y).right = node;
    }
    (*node).right = nil;
    (*node).left = (*node).right;
    (*node).set_color(M_RB_RED as libc::c_int as mln_u32_t);
    rbtree_insert_fixup(t, node);
    if (*t).min == nil {
        (*t).min = node;
    } else if ((*t).cmp)
        .expect("non-null function pointer")((*node).data, (*(*t).min).data)
        < 0 as libc::c_int
    {
        (*t).min = node;
    }
    (*t).nr_node = ((*t).nr_node).wrapping_add(1);
    (*t).nr_node;
    mln_rbtree_chain_add(&mut (*t).head, &mut (*t).tail, node);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_search(
    mut t: *mut mln_rbtree_t,
    mut key: *mut libc::c_void,
) -> *mut mln_rbtree_node_t {
    let mut ret_node: *mut mln_rbtree_node_t = (*t).root;
    let mut ret: libc::c_int = 0;
    while ret_node != &mut (*t).nil as *mut mln_rbtree_node_t
        && {
            ret = ((*t).cmp).expect("non-null function pointer")(key, (*ret_node).data);
            ret != 0 as libc::c_int
        }
    {
        if ret < 0 as libc::c_int {
            ret_node = (*ret_node).left;
        } else {
            ret_node = (*ret_node).right;
        }
    }
    return ret_node;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_successor(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) -> *mut mln_rbtree_node_t {
    if n != &mut (*t).nil as *mut mln_rbtree_node_t
        && (*n).right != &mut (*t).nil as *mut mln_rbtree_node_t
    {
        return rbtree_minimum(t, (*n).right);
    }
    let mut tmp: *mut mln_rbtree_node_t = (*n).parent;
    while tmp != &mut (*t).nil as *mut mln_rbtree_node_t && n == (*tmp).right {
        n = tmp;
        tmp = (*tmp).parent;
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_node_new(
    mut t: *mut mln_rbtree_t,
    mut data: *mut libc::c_void,
) -> *mut mln_rbtree_node_t {
    let mut n: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    if ((*t).pool).is_null() {
        n = malloc(::core::mem::size_of::<mln_rbtree_node_t>() as libc::c_ulong)
            as *mut mln_rbtree_node_t;
    } else {
        n = ((*t).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*t).pool, ::core::mem::size_of::<mln_rbtree_node_t>() as libc::c_ulong)
            as *mut mln_rbtree_node_t;
    }
    if n.is_null() {
        return 0 as *mut mln_rbtree_node_t;
    }
    (*n).data = data;
    (*n).set_nofree(0 as libc::c_int as mln_u32_t);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_node_free(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    let mut nofree: mln_u32_t = (*n).nofree();
    if !((*n).data).is_null() && ((*t).data_free).is_some() {
        ((*t).data_free).expect("non-null function pointer")((*n).data);
    }
    if nofree == 0 {
        if !((*t).pool).is_null() {
            ((*t).pool_free).expect("non-null function pointer")(n as *mut libc::c_void);
        } else {
            free(n as *mut libc::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn rbtree_minimum(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) -> *mut mln_rbtree_node_t {
    while (*n).left != &mut (*t).nil as *mut mln_rbtree_node_t {
        n = (*n).left;
    }
    return n;
}
#[inline]
unsafe extern "C" fn rbtree_transplant(
    mut t: *mut mln_rbtree_t,
    mut u: *mut mln_rbtree_node_t,
    mut v: *mut mln_rbtree_node_t,
) {
    if (*u).parent == &mut (*t).nil as *mut mln_rbtree_node_t {
        (*t).root = v;
    } else if u == (*(*u).parent).left {
        (*(*u).parent).left = v;
    } else {
        (*(*u).parent).right = v;
    }
    (*v).parent = (*u).parent;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_delete(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    if n == (*t).min {
        (*t).min = mln_rbtree_successor(t, n);
    }
    let mut y_original_color: rbtree_color = M_RB_RED;
    let mut x: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut y: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    y = n;
    y_original_color = (*y).color() as rbtree_color;
    if (*n).left == &mut (*t).nil as *mut mln_rbtree_node_t {
        x = (*n).right;
        rbtree_transplant(t, n, (*n).right);
    } else if (*n).right == &mut (*t).nil as *mut mln_rbtree_node_t {
        x = (*n).left;
        rbtree_transplant(t, n, (*n).left);
    } else {
        y = rbtree_minimum(t, (*n).right);
        y_original_color = (*y).color() as rbtree_color;
        x = (*y).right;
        if (*y).parent == n {
            (*x).parent = y;
        } else {
            rbtree_transplant(t, y, (*y).right);
            (*y).right = (*n).right;
            (*(*y).right).parent = y;
        }
        rbtree_transplant(t, n, y);
        (*y).left = (*n).left;
        (*(*y).left).parent = y;
        (*y).set_color((*n).color());
    }
    if y_original_color as libc::c_uint == M_RB_BLACK as libc::c_int as libc::c_uint {
        rbtree_delete_fixup(t, x);
    }
    (*n).right = &mut (*t).nil;
    (*n).left = (*n).right;
    (*n).parent = (*n).left;
    (*t).nr_node = ((*t).nr_node).wrapping_sub(1);
    (*t).nr_node;
    if !((*t).iter).is_null() && (*t).iter == n {
        (*t).iter = (*n).next;
        (*t).set_del(1 as libc::c_int as mln_u32_t);
    }
    mln_rbtree_chain_del(&mut (*t).head, &mut (*t).tail, n);
}
#[inline]
unsafe extern "C" fn rbtree_delete_fixup(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    let mut tmp: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    while n != (*t).root && (*n).color() as libc::c_int == M_RB_BLACK as libc::c_int {
        if n == (*(*n).parent).left {
            tmp = (*(*n).parent).right;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*n).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_left_rotate(t, (*n).parent);
                tmp = (*(*n).parent).right;
            }
            if (*(*tmp).left).color() as libc::c_int == M_RB_BLACK as libc::c_int
                && (*(*tmp).right).color() as libc::c_int == M_RB_BLACK as libc::c_int
            {
                (*tmp).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*n).parent;
            } else {
                if (*(*tmp).right).color() as libc::c_int == M_RB_BLACK as libc::c_int {
                    (*(*tmp).left).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                    (*tmp).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                    mln_rbtree_right_rotate(t, tmp);
                    tmp = (*(*n).parent).right;
                }
                (*tmp).set_color((*(*n).parent).color());
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*tmp).right).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                mln_rbtree_left_rotate(t, (*n).parent);
                n = (*t).root;
            }
        } else {
            tmp = (*(*n).parent).left;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*n).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_right_rotate(t, (*n).parent);
                tmp = (*(*n).parent).left;
            }
            if (*(*tmp).right).color() as libc::c_int == M_RB_BLACK as libc::c_int
                && (*(*tmp).left).color() as libc::c_int == M_RB_BLACK as libc::c_int
            {
                (*tmp).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*n).parent;
            } else {
                if (*(*tmp).left).color() as libc::c_int == M_RB_BLACK as libc::c_int {
                    (*(*tmp).right).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                    (*tmp).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                    mln_rbtree_left_rotate(t, tmp);
                    tmp = (*(*n).parent).left;
                }
                (*tmp).set_color((*(*n).parent).color());
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*tmp).left).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                mln_rbtree_right_rotate(t, (*n).parent);
                n = (*t).root;
            }
        }
    }
    (*n).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_min(
    mut t: *mut mln_rbtree_t,
) -> *mut mln_rbtree_node_t {
    return (*t).min;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rbtree_iterate(
    mut t: *mut mln_rbtree_t,
    mut handler: rbtree_iterate_handler,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    (*t).iter = (*t).head;
    while !((*t).iter).is_null() {
        if handler.expect("non-null function pointer")((*t).iter, udata)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (*t).del() != 0 {
            (*t).set_del(0 as libc::c_int as mln_u32_t);
        } else {
            (*t).iter = (*(*t).iter).next;
        }
    }
    return 0 as libc::c_int;
}
