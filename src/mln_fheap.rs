use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type mln_u32_t = libc::c_uint;
pub type mln_size_t = size_t;
pub type mln_fheap_mark = libc::c_uint;
pub const FHEAP_TRUE: mln_fheap_mark = 1;
pub const FHEAP_FALSE: mln_fheap_mark = 0;
pub type fheap_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type fheap_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type fheap_key_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type fheap_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type fheap_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fheap_attr {
    pub pool: *mut libc::c_void,
    pub pool_alloc: fheap_pool_alloc_handler,
    pub pool_free: fheap_pool_free_handler,
    pub cmp: fheap_cmp,
    pub copy: fheap_copy,
    pub key_free: fheap_key_free,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_fheap_node_s {
    pub key: *mut libc::c_void,
    pub parent: *mut mln_fheap_node_s,
    pub child: *mut mln_fheap_node_s,
    pub left: *mut mln_fheap_node_s,
    pub right: *mut mln_fheap_node_s,
    pub degree: mln_size_t,
    #[bitfield(name = "nofree", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "mark", ty = "mln_u32_t", bits = "1..=31")]
    pub nofree_mark: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_fheap_node_t = mln_fheap_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fheap_t {
    pub root_list: *mut mln_fheap_node_t,
    pub min: *mut mln_fheap_node_t,
    pub cmp: fheap_cmp,
    pub copy: fheap_copy,
    pub key_free: fheap_key_free,
    pub num: mln_size_t,
    pub min_val: *mut libc::c_void,
    pub pool: *mut libc::c_void,
    pub pool_alloc: fheap_pool_alloc_handler,
    pub pool_free: fheap_pool_free_handler,
}
#[inline]
unsafe extern "C" fn mln_fheap_remove_child(
    mut root: *mut *mut mln_fheap_node_t,
) -> *mut mln_fheap_node_t {
    if (*root).is_null() {
        return 0 as *mut mln_fheap_node_t;
    }
    let mut ret: *mut mln_fheap_node_t = *root;
    if (*ret).right == ret {
        *root = 0 as *mut mln_fheap_node_t;
    } else {
        *root = (*ret).right;
        (*(*ret).right).left = (*ret).left;
        (*(*ret).left).right = (*ret).right;
    }
    (*ret).right = ret;
    (*ret).left = (*ret).right;
    return ret;
}
#[inline]
unsafe extern "C" fn mln_fheap_cascading_cut(
    mut fh: *mut mln_fheap_t,
    mut y: *mut mln_fheap_node_t,
) {
    let mut z: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    loop {
        z = (*y).parent;
        if z.is_null() {
            return;
        }
        if (*y).mark() as libc::c_int == FHEAP_FALSE as libc::c_int {
            break;
        }
        mln_fheap_cut(fh, y, z);
        y = z;
    }
    (*y).set_mark(FHEAP_TRUE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_cut(
    mut fh: *mut mln_fheap_t,
    mut x: *mut mln_fheap_node_t,
    mut y: *mut mln_fheap_node_t,
) {
    mln_fheap_del_child(&mut (*y).child, x);
    (*y).degree = ((*y).degree).wrapping_sub(1);
    (*y).degree;
    mln_fheap_add_child(&mut (*fh).root_list, x);
    (*x).parent = 0 as *mut mln_fheap_node_s;
    (*x).set_mark(FHEAP_FALSE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_consolidate(
    mut fh: *mut mln_fheap_t,
    mut cmp: fheap_cmp,
) {
    let mut array: [*mut mln_fheap_node_t; 65] = [0 as *mut mln_fheap_node_t; 65];
    memset(
        array.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<*mut mln_fheap_node_t>() as libc::c_ulong)
            .wrapping_mul(65 as libc::c_int as libc::c_ulong),
    );
    let mut x: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut y: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut w: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut tmp: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut d: mln_size_t = 0;
    let mut mark: mln_size_t = 0 as libc::c_int as mln_size_t;
    w = (*fh).root_list;
    while !w.is_null() && !(w == (*fh).root_list && mark != 0) {
        if w == (*fh).root_list {
            mark = mark.wrapping_add(1);
            mark;
        }
        x = w;
        w = (*w).right;
        d = (*x).degree;
        while !(array[d as usize]).is_null() {
            y = array[d as usize];
            if cmp.expect("non-null function pointer")((*y).key, (*x).key) == 0 {
                tmp = x;
                x = y;
                y = tmp;
            }
            if y == w {
                w = (*w).right;
            }
            mln_fheap_link(fh, y, x);
            array[d as usize] = 0 as *mut mln_fheap_node_t;
            d = d.wrapping_add(1);
            d;
        }
        array[d as usize] = x;
    }
    (*fh).min = 0 as *mut mln_fheap_node_t;
    let mut i: mln_size_t = 0;
    let mut root_list: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    i = 0 as libc::c_int as mln_size_t;
    while i < 65 as libc::c_int as libc::c_ulong {
        if !(array[i as usize]).is_null() {
            mln_fheap_del_child(&mut (*fh).root_list, array[i as usize]);
            mln_fheap_add_child(&mut root_list, array[i as usize]);
            (*array[i as usize]).parent = 0 as *mut mln_fheap_node_s;
            if ((*fh).min).is_null() {
                (*fh).min = array[i as usize];
            } else if cmp
                .expect(
                    "non-null function pointer",
                )((*array[i as usize]).key, (*(*fh).min).key) == 0
            {
                (*fh).min = array[i as usize];
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    (*fh).root_list = root_list;
}
#[inline]
unsafe extern "C" fn mln_fheap_link(
    mut fh: *mut mln_fheap_t,
    mut y: *mut mln_fheap_node_t,
    mut x: *mut mln_fheap_node_t,
) {
    mln_fheap_del_child(&mut (*fh).root_list, y);
    mln_fheap_add_child(&mut (*x).child, y);
    (*y).parent = x;
    (*x).degree = ((*x).degree).wrapping_add(1);
    (*x).degree;
    (*y).set_mark(FHEAP_FALSE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_del_child(
    mut root: *mut *mut mln_fheap_node_t,
    mut node: *mut mln_fheap_node_t,
) {
    if *root == node {
        if (*node).right == node {
            *root = 0 as *mut mln_fheap_node_t;
        } else {
            *root = (*node).right;
            (*(*node).right).left = (*node).left;
            (*(*node).left).right = (*node).right;
        }
    } else {
        (*(*node).right).left = (*node).left;
        (*(*node).left).right = (*node).right;
    }
    (*node).left = node;
    (*node).right = (*node).left;
}
#[inline]
unsafe extern "C" fn mln_fheap_add_child(
    mut root: *mut *mut mln_fheap_node_t,
    mut node: *mut mln_fheap_node_t,
) {
    if (*root).is_null() {
        *root = node;
        return;
    }
    (*node).right = *root;
    (*node).left = (**root).left;
    (**root).left = node;
    (*(*node).left).right = node;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_new(
    mut min_val: *mut libc::c_void,
    mut attr: *mut mln_fheap_attr,
) -> *mut mln_fheap_t {
    let mut fh: *mut mln_fheap_t = 0 as *mut mln_fheap_t;
    if !attr.is_null() && !((*attr).pool).is_null() {
        fh = ((*attr).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*attr).pool, ::core::mem::size_of::<mln_fheap_t>() as libc::c_ulong)
            as *mut mln_fheap_t;
    } else {
        fh = malloc(::core::mem::size_of::<mln_fheap_t>() as libc::c_ulong)
            as *mut mln_fheap_t;
    }
    if fh.is_null() {
        return 0 as *mut mln_fheap_t;
    }
    if !attr.is_null() {
        (*fh).pool = (*attr).pool;
        (*fh).pool_alloc = (*attr).pool_alloc;
        (*fh).pool_free = (*attr).pool_free;
        (*fh).cmp = (*attr).cmp;
        (*fh).copy = (*attr).copy;
        (*fh).key_free = (*attr).key_free;
    } else {
        (*fh).pool = 0 as *mut libc::c_void;
        (*fh).pool_alloc = None;
        (*fh).pool_free = None;
        (*fh).cmp = None;
        (*fh).copy = None;
        (*fh).key_free = None;
    }
    (*fh).min_val = min_val;
    (*fh).min = 0 as *mut mln_fheap_node_t;
    (*fh).root_list = 0 as *mut mln_fheap_node_t;
    (*fh).num = 0 as libc::c_int as mln_size_t;
    return fh;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_new_fast(
    mut min_val: *mut libc::c_void,
    mut cmp: fheap_cmp,
    mut copy: fheap_copy,
    mut key_free: fheap_key_free,
) -> *mut mln_fheap_t {
    let mut fh: *mut mln_fheap_t = 0 as *mut mln_fheap_t;
    fh = malloc(::core::mem::size_of::<mln_fheap_t>() as libc::c_ulong)
        as *mut mln_fheap_t;
    if fh.is_null() {
        return 0 as *mut mln_fheap_t;
    }
    (*fh).pool = 0 as *mut libc::c_void;
    (*fh).pool_alloc = None;
    (*fh).pool_free = None;
    (*fh).cmp = cmp;
    (*fh).copy = copy;
    (*fh).key_free = key_free;
    (*fh).min_val = min_val;
    (*fh).min = 0 as *mut mln_fheap_node_t;
    (*fh).root_list = 0 as *mut mln_fheap_node_t;
    (*fh).num = 0 as libc::c_int as mln_size_t;
    return fh;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_insert(
    mut fh: *mut mln_fheap_t,
    mut fn_0: *mut mln_fheap_node_t,
) {
    mln_fheap_add_child(&mut (*fh).root_list, fn_0);
    (*fn_0).parent = 0 as *mut mln_fheap_node_s;
    if ((*fh).min).is_null() {
        (*fh).min = fn_0;
    } else if ((*fh).cmp)
        .expect("non-null function pointer")((*fn_0).key, (*(*fh).min).key) == 0
    {
        (*fh).min = fn_0;
    }
    (*fh).num = ((*fh).num).wrapping_add(1);
    (*fh).num;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_extract_min(
    mut fh: *mut mln_fheap_t,
) -> *mut mln_fheap_node_t {
    let mut z: *mut mln_fheap_node_t = (*fh).min;
    if !z.is_null() {
        let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
        loop {
            child = mln_fheap_remove_child(&mut (*z).child);
            if child.is_null() {
                break;
            }
            mln_fheap_add_child(&mut (*fh).root_list, child);
            (*child).parent = 0 as *mut mln_fheap_node_s;
        }
        let mut right: *mut mln_fheap_node_t = (*z).right;
        mln_fheap_del_child(&mut (*fh).root_list, z);
        if z == right {
            (*fh).min = 0 as *mut mln_fheap_node_t;
        } else {
            (*fh).min = right;
            mln_fheap_consolidate(fh, (*fh).cmp);
        }
        (*fh).num = ((*fh).num).wrapping_sub(1);
        (*fh).num;
    }
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_decrease_key(
    mut fh: *mut mln_fheap_t,
    mut node: *mut mln_fheap_node_t,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    if ((*fh).cmp).expect("non-null function pointer")((*node).key, key) == 0 {
        return -(1 as libc::c_int);
    }
    ((*fh).copy).expect("non-null function pointer")((*node).key, key);
    let mut y: *mut mln_fheap_node_t = (*node).parent;
    if !y.is_null()
        && ((*fh).cmp).expect("non-null function pointer")((*node).key, (*y).key) == 0
    {
        mln_fheap_cut(fh, node, y);
        mln_fheap_cascading_cut(fh, y);
    }
    if node != (*fh).min
        && ((*fh).cmp).expect("non-null function pointer")((*node).key, (*(*fh).min).key)
            == 0
    {
        (*fh).min = node;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_delete(
    mut fh: *mut mln_fheap_t,
    mut node: *mut mln_fheap_node_t,
) {
    mln_fheap_decrease_key(fh, node, (*fh).min_val);
    mln_fheap_extract_min(fh);
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_free(mut fh: *mut mln_fheap_t) {
    if fh.is_null() {
        return;
    }
    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    loop {
        fn_0 = mln_fheap_extract_min(fh);
        if fn_0.is_null() {
            break;
        }
        mln_fheap_node_free(fh, fn_0);
    }
    if !((*fh).pool).is_null() {
        ((*fh).pool_free).expect("non-null function pointer")(fh as *mut libc::c_void);
    } else {
        free(fh as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_node_new(
    mut fh: *mut mln_fheap_t,
    mut key: *mut libc::c_void,
) -> *mut mln_fheap_node_t {
    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    if !((*fh).pool).is_null() {
        fn_0 = ((*fh).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*fh).pool, ::core::mem::size_of::<mln_fheap_node_t>() as libc::c_ulong)
            as *mut mln_fheap_node_t;
    } else {
        fn_0 = malloc(::core::mem::size_of::<mln_fheap_node_t>() as libc::c_ulong)
            as *mut mln_fheap_node_t;
    }
    if fn_0.is_null() {
        return 0 as *mut mln_fheap_node_t;
    }
    (*fn_0).key = key;
    (*fn_0).parent = 0 as *mut mln_fheap_node_s;
    (*fn_0).child = 0 as *mut mln_fheap_node_s;
    (*fn_0).left = fn_0;
    (*fn_0).right = fn_0;
    (*fn_0).degree = 0 as libc::c_int as mln_size_t;
    (*fn_0).set_nofree(0 as libc::c_int as mln_u32_t);
    (*fn_0).set_mark(FHEAP_FALSE as libc::c_int as mln_u32_t);
    return fn_0;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fheap_node_free(
    mut fh: *mut mln_fheap_t,
    mut fn_0: *mut mln_fheap_node_t,
) {
    if fn_0.is_null() {
        return;
    }
    if ((*fh).key_free).is_some() && !((*fn_0).key).is_null() {
        ((*fh).key_free).expect("non-null function pointer")((*fn_0).key);
    }
    if (*fn_0).nofree() == 0 {
        if !((*fh).pool).is_null() {
            ((*fh).pool_free)
                .expect("non-null function pointer")(fn_0 as *mut libc::c_void);
        } else {
            free(fn_0 as *mut libc::c_void);
        }
    }
}
