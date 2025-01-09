use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mln_prime_generate(n: mln_u32_t) -> mln_u32_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_hash_s {
    pub tbl: *mut mln_hash_mgr_t,
    pub len: mln_u64_t,
    pub hash: hash_calc_handler,
    pub cmp: hash_cmp_handler,
    pub key_freer: hash_free_handler,
    pub val_freer: hash_free_handler,
    pub nr_nodes: mln_u32_t,
    pub threshold: mln_u32_t,
    #[bitfield(name = "expandable", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "calc_prime", ty = "mln_u32_t", bits = "1..=1")]
    pub expandable_calc_prime: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub pool: *mut libc::c_void,
    pub pool_alloc: hash_pool_alloc_handler,
    pub pool_free: hash_pool_free_handler,
    pub iter_head: *mut mln_hash_entry_t,
    pub iter_tail: *mut mln_hash_entry_t,
    pub iter: *mut mln_hash_entry_t,
}
pub type mln_hash_entry_t = mln_hash_entry_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_hash_entry_s {
    pub prev: *mut mln_hash_entry_s,
    pub next: *mut mln_hash_entry_s,
    pub val: *mut libc::c_void,
    pub key: *mut libc::c_void,
    pub iter_prev: *mut mln_hash_entry_s,
    pub iter_next: *mut mln_hash_entry_s,
    pub mgr: *mut mln_hash_mgr_t,
    pub remove_flag: mln_hash_flag_t,
    #[bitfield(name = "removed", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "1..=31")]
    pub removed_padding: [u8; 4],
}
pub type mln_hash_flag_t = mln_hash_flag;
pub type mln_hash_flag = libc::c_uint;
pub const M_HASH_F_KV: mln_hash_flag = 3;
pub const M_HASH_F_KEY: mln_hash_flag = 2;
pub const M_HASH_F_VAL: mln_hash_flag = 1;
pub const M_HASH_F_NONE: mln_hash_flag = 0;
pub type mln_hash_mgr_t = mln_hash_mgr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_hash_mgr_s {
    pub head: *mut mln_hash_entry_t,
    pub tail: *mut mln_hash_entry_t,
}
pub type hash_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type hash_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type hash_free_handler = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type hash_cmp_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_hash_t,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type mln_hash_t = mln_hash_s;
pub type hash_calc_handler = Option::<
    unsafe extern "C" fn(*mut mln_hash_t, *mut libc::c_void) -> mln_u64_t,
>;
pub type hash_iterate_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_hash_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_hash_attr {
    pub hash: hash_calc_handler,
    pub cmp: hash_cmp_handler,
    pub key_freer: hash_free_handler,
    pub val_freer: hash_free_handler,
    pub len_base: mln_u64_t,
    #[bitfield(name = "expandable", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "calc_prime", ty = "mln_u32_t", bits = "1..=1")]
    pub expandable_calc_prime: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub pool: *mut libc::c_void,
    pub pool_alloc: hash_pool_alloc_handler,
    pub pool_free: hash_pool_free_handler,
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_init(
    mut h: *mut mln_hash_t,
    mut attr: *mut mln_hash_attr,
) -> libc::c_int {
    (*h).pool = (*attr).pool;
    (*h).pool_alloc = (*attr).pool_alloc;
    (*h).pool_free = (*attr).pool_free;
    (*h).hash = (*attr).hash;
    (*h).cmp = (*attr).cmp;
    (*h).key_freer = (*attr).key_freer;
    (*h).val_freer = (*attr).val_freer;
    (*h)
        .len = if (*attr).calc_prime() as libc::c_int != 0 {
        mln_prime_generate((*attr).len_base as mln_u32_t) as libc::c_ulong
    } else {
        (*attr).len_base
    };
    if !((*h).pool).is_null() {
        (*h)
            .tbl = ((*h).pool_alloc)
            .expect(
                "non-null function pointer",
            )(
            (*h).pool,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        ) as *mut mln_hash_mgr_t;
        memset(
            (*h).tbl as *mut libc::c_void,
            0 as libc::c_int,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        );
    } else {
        (*h)
            .tbl = calloc(
            (*h).len,
            ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
        ) as *mut mln_hash_mgr_t;
    }
    if ((*h).tbl).is_null() {
        return -(1 as libc::c_int);
    }
    (*h).nr_nodes = 0 as libc::c_int as mln_u32_t;
    (*h)
        .threshold = (if (*attr).calc_prime() as libc::c_int != 0 {
        mln_prime_generate(((*h).len << 1 as libc::c_int) as mln_u32_t) as libc::c_ulong
    } else {
        (*h).len << 1 as libc::c_int
    }) as mln_u32_t;
    (*h).set_expandable((*attr).expandable());
    (*h).set_calc_prime((*attr).calc_prime());
    if (*h).len == 0 as libc::c_int as libc::c_ulong || ((*h).hash).is_none()
        || ((*h).cmp).is_none()
    {
        if !((*h).pool).is_null() {
            ((*h).pool_free)
                .expect("non-null function pointer")((*h).tbl as *mut libc::c_void);
        } else {
            free((*h).tbl as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    (*h).iter_tail = 0 as *mut mln_hash_entry_t;
    (*h).iter_head = (*h).iter_tail;
    (*h).iter = (*h).iter_head;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_init_fast(
    mut h: *mut mln_hash_t,
    mut hash: hash_calc_handler,
    mut cmp: hash_cmp_handler,
    mut key_freer: hash_free_handler,
    mut val_freer: hash_free_handler,
    mut base_len: mln_u64_t,
    mut expandable: mln_u32_t,
    mut calc_prime: mln_u32_t,
) -> libc::c_int {
    (*h).pool = 0 as *mut libc::c_void;
    (*h).pool_alloc = None;
    (*h).pool_free = None;
    (*h).hash = hash;
    (*h).cmp = cmp;
    (*h).key_freer = key_freer;
    (*h).val_freer = val_freer;
    (*h)
        .len = if calc_prime != 0 {
        mln_prime_generate(base_len as mln_u32_t) as libc::c_ulong
    } else {
        base_len
    };
    (*h)
        .tbl = calloc(
        (*h).len,
        ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
    ) as *mut mln_hash_mgr_t;
    if ((*h).tbl).is_null() {
        return -(1 as libc::c_int);
    }
    (*h).nr_nodes = 0 as libc::c_int as mln_u32_t;
    (*h)
        .threshold = (if calc_prime != 0 {
        mln_prime_generate(((*h).len << 1 as libc::c_int) as mln_u32_t) as libc::c_ulong
    } else {
        (*h).len << 1 as libc::c_int
    }) as mln_u32_t;
    (*h).set_expandable(expandable);
    (*h).set_calc_prime(calc_prime);
    if (*h).len == 0 as libc::c_int as libc::c_ulong || ((*h).hash).is_none()
        || ((*h).cmp).is_none()
    {
        if !((*h).pool).is_null() {
            ((*h).pool_free)
                .expect("non-null function pointer")((*h).tbl as *mut libc::c_void);
        } else {
            free((*h).tbl as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    (*h).iter_tail = 0 as *mut mln_hash_entry_t;
    (*h).iter_head = (*h).iter_tail;
    (*h).iter = (*h).iter_head;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_new(mut attr: *mut mln_hash_attr) -> *mut mln_hash_t {
    let mut h: *mut mln_hash_t = 0 as *mut mln_hash_t;
    if !((*attr).pool).is_null() {
        h = ((*attr).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*attr).pool, ::core::mem::size_of::<mln_hash_t>() as libc::c_ulong)
            as *mut mln_hash_t;
    } else {
        h = malloc(::core::mem::size_of::<mln_hash_t>() as libc::c_ulong)
            as *mut mln_hash_t;
    }
    if h.is_null() {
        return 0 as *mut mln_hash_t;
    }
    (*h).pool = (*attr).pool;
    (*h).pool_alloc = (*attr).pool_alloc;
    (*h).pool_free = (*attr).pool_free;
    (*h).hash = (*attr).hash;
    (*h).cmp = (*attr).cmp;
    (*h).key_freer = (*attr).key_freer;
    (*h).val_freer = (*attr).val_freer;
    (*h)
        .len = if (*attr).calc_prime() as libc::c_int != 0 {
        mln_prime_generate((*attr).len_base as mln_u32_t) as libc::c_ulong
    } else {
        (*attr).len_base
    };
    if !((*h).pool).is_null() {
        (*h)
            .tbl = ((*h).pool_alloc)
            .expect(
                "non-null function pointer",
            )(
            (*h).pool,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        ) as *mut mln_hash_mgr_t;
        memset(
            (*h).tbl as *mut libc::c_void,
            0 as libc::c_int,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        );
    } else {
        (*h)
            .tbl = calloc(
            (*h).len,
            ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
        ) as *mut mln_hash_mgr_t;
    }
    if ((*h).tbl).is_null() {
        if !((*h).pool).is_null() {
            ((*h).pool_free).expect("non-null function pointer")(h as *mut libc::c_void);
        } else {
            free(h as *mut libc::c_void);
        }
        return 0 as *mut mln_hash_t;
    }
    (*h).nr_nodes = 0 as libc::c_int as mln_u32_t;
    (*h)
        .threshold = (if (*attr).calc_prime() as libc::c_int != 0 {
        mln_prime_generate(((*h).len << 1 as libc::c_int) as mln_u32_t) as libc::c_ulong
    } else {
        (*h).len << 1 as libc::c_int
    }) as mln_u32_t;
    (*h).set_expandable((*attr).expandable());
    (*h).set_calc_prime((*attr).calc_prime());
    if (*h).len == 0 as libc::c_int as libc::c_ulong || ((*h).hash).is_none()
        || ((*h).cmp).is_none()
    {
        if !((*h).pool).is_null() {
            ((*h).pool_free)
                .expect("non-null function pointer")((*h).tbl as *mut libc::c_void);
            ((*h).pool_free).expect("non-null function pointer")(h as *mut libc::c_void);
        } else {
            free((*h).tbl as *mut libc::c_void);
            free(h as *mut libc::c_void);
        }
        return 0 as *mut mln_hash_t;
    }
    (*h).iter_tail = 0 as *mut mln_hash_entry_t;
    (*h).iter_head = (*h).iter_tail;
    (*h).iter = (*h).iter_head;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_new_fast(
    mut hash: hash_calc_handler,
    mut cmp: hash_cmp_handler,
    mut key_freer: hash_free_handler,
    mut val_freer: hash_free_handler,
    mut base_len: mln_u64_t,
    mut expandable: mln_u32_t,
    mut calc_prime: mln_u32_t,
) -> *mut mln_hash_t {
    let mut h: *mut mln_hash_t = 0 as *mut mln_hash_t;
    h = malloc(::core::mem::size_of::<mln_hash_t>() as libc::c_ulong) as *mut mln_hash_t;
    if h.is_null() {
        return 0 as *mut mln_hash_t;
    }
    (*h).pool = 0 as *mut libc::c_void;
    (*h).pool_alloc = None;
    (*h).pool_free = None;
    (*h).hash = hash;
    (*h).cmp = cmp;
    (*h).key_freer = key_freer;
    (*h).val_freer = val_freer;
    (*h)
        .len = if calc_prime != 0 {
        mln_prime_generate(base_len as mln_u32_t) as libc::c_ulong
    } else {
        base_len
    };
    (*h)
        .tbl = calloc(
        (*h).len,
        ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
    ) as *mut mln_hash_mgr_t;
    if ((*h).tbl).is_null() {
        free(h as *mut libc::c_void);
        return 0 as *mut mln_hash_t;
    }
    (*h).nr_nodes = 0 as libc::c_int as mln_u32_t;
    (*h)
        .threshold = (if calc_prime != 0 {
        mln_prime_generate(((*h).len << 1 as libc::c_int) as mln_u32_t) as libc::c_ulong
    } else {
        (*h).len << 1 as libc::c_int
    }) as mln_u32_t;
    (*h).set_expandable(expandable);
    (*h).set_calc_prime(calc_prime);
    if (*h).len == 0 as libc::c_int as libc::c_ulong || ((*h).hash).is_none()
        || ((*h).cmp).is_none()
    {
        free((*h).tbl as *mut libc::c_void);
        free(h as *mut libc::c_void);
        return 0 as *mut mln_hash_t;
    }
    (*h).iter_tail = 0 as *mut mln_hash_entry_t;
    (*h).iter_head = (*h).iter_tail;
    (*h).iter = (*h).iter_head;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_destroy(
    mut h: *mut mln_hash_t,
    mut flg: mln_hash_flag_t,
) {
    if h.is_null() {
        return;
    }
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    let mut fr: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    let mut mgr: *mut mln_hash_mgr_t = 0 as *mut mln_hash_mgr_t;
    let mut mgr_end: *mut mln_hash_mgr_t = ((*h).tbl).offset((*h).len as isize);
    mgr = (*h).tbl;
    while mgr < mgr_end {
        he = (*mgr).head;
        while !he.is_null() {
            fr = he;
            he = (*he).next;
            mln_hash_entry_free(h, fr, flg);
        }
        mgr = mgr.offset(1);
        mgr;
    }
    if !((*h).pool).is_null() {
        ((*h).pool_free)
            .expect("non-null function pointer")((*h).tbl as *mut libc::c_void);
    } else {
        free((*h).tbl as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_free(
    mut h: *mut mln_hash_t,
    mut flg: mln_hash_flag_t,
) {
    if h.is_null() {
        return;
    }
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    let mut fr: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    let mut mgr: *mut mln_hash_mgr_t = 0 as *mut mln_hash_mgr_t;
    let mut mgr_end: *mut mln_hash_mgr_t = ((*h).tbl).offset((*h).len as isize);
    mgr = (*h).tbl;
    while mgr < mgr_end {
        he = (*mgr).head;
        while !he.is_null() {
            fr = he;
            he = (*he).next;
            mln_hash_entry_free(h, fr, flg);
        }
        mgr = mgr.offset(1);
        mgr;
    }
    if !((*h).pool).is_null() {
        ((*h).pool_free)
            .expect("non-null function pointer")((*h).tbl as *mut libc::c_void);
    } else {
        free((*h).tbl as *mut libc::c_void);
    }
    if !((*h).pool).is_null() {
        ((*h).pool_free).expect("non-null function pointer")(h as *mut libc::c_void);
    } else {
        free(h as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_update(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut k: *mut *mut libc::c_void = key as *mut *mut libc::c_void;
    let mut v: *mut *mut libc::c_void = val as *mut *mut libc::c_void;
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, *k)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he = (*mgr).head;
    while !he.is_null() {
        if ((*h).cmp).expect("non-null function pointer")(h, *k, (*he).key) != 0 {
            break;
        }
        he = (*he).next;
    }
    if !he.is_null() {
        (*he).set_removed(0 as libc::c_int as mln_u32_t);
        let mut save_key: *mut libc::c_void = (*he).key;
        let mut save_val: *mut libc::c_void = (*he).val;
        (*he).key = *k;
        (*he).val = *v;
        *k = save_key;
        *v = save_val;
        return 0 as libc::c_int;
    }
    if (*h).expandable() as libc::c_int != 0 && (*h).nr_nodes > (*h).threshold {
        mln_hash_expand(h);
    }
    if (*h).expandable() as libc::c_int != 0
        && (*h).nr_nodes <= (*h).threshold >> 3 as libc::c_int
    {
        mln_hash_reduce(h);
    }
    he = mln_hash_entry_new(h, mgr, *k, *v);
    if he.is_null() {
        return -(1 as libc::c_int);
    }
    mln_hash_entry_chain_add(&mut (*mgr).head, &mut (*mgr).tail, he);
    mln_hash_entry_iter_chain_add(&mut (*h).iter_head, &mut (*h).iter_tail, he);
    (*h).nr_nodes = ((*h).nr_nodes).wrapping_add(1);
    (*h).nr_nodes;
    *v = 0 as *mut libc::c_void;
    *k = *v;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_insert(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    if (*h).expandable() as libc::c_int != 0 && (*h).nr_nodes > (*h).threshold {
        mln_hash_expand(h);
    }
    if (*h).expandable() as libc::c_int != 0
        && (*h).nr_nodes <= (*h).threshold >> 3 as libc::c_int
    {
        mln_hash_reduce(h);
    }
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = mln_hash_entry_new(h, mgr, key, val);
    if he.is_null() {
        return -(1 as libc::c_int);
    }
    mln_hash_entry_chain_add(&mut (*mgr).head, &mut (*mgr).tail, he);
    mln_hash_entry_iter_chain_add(&mut (*h).iter_head, &mut (*h).iter_tail, he);
    (*h).nr_nodes = ((*h).nr_nodes).wrapping_add(1);
    (*h).nr_nodes;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_hash_reduce(mut h: *mut mln_hash_t) {
    let mut old_tbl: *mut mln_hash_mgr_t = (*h).tbl;
    let mut len: mln_u32_t = (*h).len as mln_u32_t;
    (*h)
        .len = (if (*h).calc_prime() as libc::c_int != 0 {
        mln_prime_generate((*h).threshold >> 2 as libc::c_int)
    } else {
        (*h).threshold >> 2 as libc::c_int
    }) as mln_u64_t;
    if (*h).len == 0 as libc::c_int as libc::c_ulong {
        (*h).len = 1 as libc::c_int as mln_u64_t;
    }
    if !((*h).pool).is_null() {
        (*h)
            .tbl = ((*h).pool_alloc)
            .expect(
                "non-null function pointer",
            )(
            (*h).pool,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        ) as *mut mln_hash_mgr_t;
        memset(
            (*h).tbl as *mut libc::c_void,
            0 as libc::c_int,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        );
    } else {
        (*h)
            .tbl = calloc(
            (*h).len,
            ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
        ) as *mut mln_hash_mgr_t;
    }
    if ((*h).tbl).is_null() {
        (*h).tbl = old_tbl;
        (*h).len = len as mln_u64_t;
        return;
    }
    (*h)
        .threshold = if (*h).calc_prime() as libc::c_int != 0 {
        mln_prime_generate((*h).threshold >> 1 as libc::c_int)
    } else {
        (*h).threshold >> 1 as libc::c_int
    };
    mln_move_hash_entry(h, old_tbl, len);
    if !((*h).pool).is_null() {
        ((*h).pool_free)
            .expect("non-null function pointer")(old_tbl as *mut libc::c_void);
    } else {
        free(old_tbl as *mut libc::c_void);
    };
}
#[inline]
unsafe extern "C" fn mln_hash_expand(mut h: *mut mln_hash_t) {
    let mut old_tbl: *mut mln_hash_mgr_t = (*h).tbl;
    let mut len: mln_u32_t = (*h).len as mln_u32_t;
    (*h)
        .len = (if (*h).calc_prime() as libc::c_int != 0 {
        mln_prime_generate(len << 1 as libc::c_int)
    } else {
        (len << 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
    }) as mln_u64_t;
    if !((*h).pool).is_null() {
        (*h)
            .tbl = ((*h).pool_alloc)
            .expect(
                "non-null function pointer",
            )(
            (*h).pool,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        ) as *mut mln_hash_mgr_t;
        memset(
            (*h).tbl as *mut libc::c_void,
            0 as libc::c_int,
            ((*h).len)
                .wrapping_mul(::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong),
        );
    } else {
        (*h)
            .tbl = calloc(
            (*h).len,
            ::core::mem::size_of::<mln_hash_mgr_t>() as libc::c_ulong,
        ) as *mut mln_hash_mgr_t;
    }
    if ((*h).tbl).is_null() {
        (*h).tbl = old_tbl;
        (*h).len = len as mln_u64_t;
        return;
    }
    (*h)
        .threshold = if (*h).calc_prime() as libc::c_int != 0 {
        mln_prime_generate((*h).threshold << 1 as libc::c_int)
    } else {
        ((*h).threshold << 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    };
    mln_move_hash_entry(h, old_tbl, len);
    if !((*h).pool).is_null() {
        ((*h).pool_free)
            .expect("non-null function pointer")(old_tbl as *mut libc::c_void);
    } else {
        free(old_tbl as *mut libc::c_void);
    };
}
#[inline]
unsafe extern "C" fn mln_move_hash_entry(
    mut h: *mut mln_hash_t,
    mut old_tbl: *mut mln_hash_mgr_t,
    mut old_len: mln_u32_t,
) {
    let mut old_end: *mut mln_hash_mgr_t = old_tbl.offset(old_len as isize);
    let mut new_mgr: *mut mln_hash_mgr_t = 0 as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    let mut index: mln_u32_t = 0;
    while old_tbl < old_end {
        loop {
            he = (*old_tbl).head;
            if he.is_null() {
                break;
            }
            mln_hash_entry_chain_del(&mut (*old_tbl).head, &mut (*old_tbl).tail, he);
            index = ((*h).hash).expect("non-null function pointer")(h, (*he).key)
                as mln_u32_t;
            new_mgr = &mut *((*h).tbl).offset(index as isize) as *mut mln_hash_mgr_t;
            mln_hash_entry_chain_add(&mut (*new_mgr).head, &mut (*new_mgr).tail, he);
        }
        old_tbl = old_tbl.offset(1);
        old_tbl;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_change_value(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut new_value: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he = (*mgr).head;
    while !he.is_null() {
        if ((*h).cmp).expect("non-null function pointer")(h, key, (*he).key) != 0 {
            break;
        }
        he = (*he).next;
    }
    if he.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut retval: mln_u8ptr_t = (*he).val as mln_u8ptr_t;
    (*he).val = new_value;
    return retval as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_search(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he = (*mgr).head;
    while !he.is_null() {
        if ((*h).cmp).expect("non-null function pointer")(h, key, (*he).key) != 0 {
            break;
        }
        he = (*he).next;
    }
    if he.is_null() || (*he).removed() as libc::c_int != 0 {
        return 0 as *mut libc::c_void;
    }
    return (*he).val;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_search_iterator(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut ctx: *mut *mut libc::c_int,
) -> *mut libc::c_void {
    if !(*ctx).is_null() {
        let mut he: *mut mln_hash_entry_t = *(ctx as *mut *mut mln_hash_entry_t);
        while !he.is_null() {
            if ((*h).cmp).expect("non-null function pointer")(h, key, (*he).key) != 0 {
                break;
            }
            he = (*he).next;
        }
        if he.is_null() || (*he).removed() as libc::c_int != 0 {
            *ctx = 0 as *mut libc::c_int;
            return 0 as *mut libc::c_void;
        }
        *ctx = (*he).next as *mut libc::c_int;
        return (*he).val;
    }
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he_0: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he_0 = (*mgr).head;
    while !he_0.is_null() {
        if ((*h).cmp).expect("non-null function pointer")(h, key, (*he_0).key) != 0 {
            break;
        }
        he_0 = (*he_0).next;
    }
    if he_0.is_null() || (*he_0).removed() as libc::c_int != 0 {
        return 0 as *mut libc::c_void;
    }
    *ctx = (*he_0).next as *mut libc::c_int;
    return (*he_0).val;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_remove(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut flg: mln_hash_flag_t,
) {
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he = (*mgr).head;
    while !he.is_null() {
        if ((*h).cmp).expect("non-null function pointer")(h, key, (*he).key) != 0 {
            break;
        }
        he = (*he).next;
    }
    if he.is_null() {
        return;
    }
    if (*h).iter == he {
        (*he).remove_flag = flg;
        (*he).set_removed(1 as libc::c_int as mln_u32_t);
        return;
    }
    mln_hash_entry_chain_del(&mut (*mgr).head, &mut (*mgr).tail, he);
    mln_hash_entry_iter_chain_del(&mut (*h).iter_head, &mut (*h).iter_tail, he);
    (*h).nr_nodes = ((*h).nr_nodes).wrapping_sub(1);
    (*h).nr_nodes;
    mln_hash_entry_free(h, he, flg);
}
#[inline]
unsafe extern "C" fn mln_hash_entry_new(
    mut h: *mut mln_hash_t,
    mut mgr: *mut mln_hash_mgr_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> *mut mln_hash_entry_t {
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    if !((*h).pool).is_null() {
        he = ((*h).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*h).pool, ::core::mem::size_of::<mln_hash_entry_t>() as libc::c_ulong)
            as *mut mln_hash_entry_t;
    } else {
        he = malloc(::core::mem::size_of::<mln_hash_entry_t>() as libc::c_ulong)
            as *mut mln_hash_entry_t;
    }
    if he.is_null() {
        return 0 as *mut mln_hash_entry_t;
    }
    (*he).val = val;
    (*he).key = key;
    (*he).next = 0 as *mut mln_hash_entry_s;
    (*he).prev = (*he).next;
    (*he).iter_next = 0 as *mut mln_hash_entry_s;
    (*he).iter_prev = (*he).iter_next;
    (*he).mgr = mgr;
    (*he).remove_flag = M_HASH_F_NONE;
    (*he).set_removed(0 as libc::c_int as mln_u32_t);
    return he;
}
#[inline]
unsafe extern "C" fn mln_hash_entry_free(
    mut h: *mut mln_hash_t,
    mut he: *mut mln_hash_entry_t,
    mut flg: mln_hash_flag_t,
) {
    if he.is_null() {
        return;
    }
    match flg as libc::c_uint {
        1 => {
            if ((*h).val_freer).is_some() {
                ((*h).val_freer).expect("non-null function pointer")((*he).val);
            }
        }
        2 => {
            if ((*h).key_freer).is_some() {
                ((*h).key_freer).expect("non-null function pointer")((*he).key);
            }
        }
        3 => {
            if ((*h).val_freer).is_some() {
                ((*h).val_freer).expect("non-null function pointer")((*he).val);
            }
            if ((*h).key_freer).is_some() {
                ((*h).key_freer).expect("non-null function pointer")((*he).key);
            }
        }
        _ => {}
    }
    if !((*h).pool).is_null() {
        ((*h).pool_free).expect("non-null function pointer")(he as *mut libc::c_void);
    } else {
        free(he as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_iterate(
    mut h: *mut mln_hash_t,
    mut handler: hash_iterate_handler,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut he: *mut mln_hash_entry_t = (*h).iter_head;
    let mut cur: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    while !he.is_null() {
        cur = he;
        (*h).iter = cur;
        he = (*he).iter_next;
        if (*cur).removed() != 0 {
            continue;
        }
        if handler.is_some()
            && handler
                .expect("non-null function pointer")(h, (*cur).key, (*cur).val, udata)
                < 0 as libc::c_int
        {
            (*h).iter = 0 as *mut mln_hash_entry_t;
            return -(1 as libc::c_int);
        }
        if (*cur).removed() != 0 {
            mln_hash_entry_chain_del(
                &mut (*(*cur).mgr).head,
                &mut (*(*cur).mgr).tail,
                cur,
            );
            mln_hash_entry_iter_chain_del(&mut (*h).iter_head, &mut (*h).iter_tail, cur);
            (*h).nr_nodes = ((*h).nr_nodes).wrapping_sub(1);
            (*h).nr_nodes;
            mln_hash_entry_free(h, cur, (*cur).remove_flag);
        }
    }
    (*h).iter = 0 as *mut mln_hash_entry_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_key_exist(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    let mut index: mln_u32_t = ((*h).hash).expect("non-null function pointer")(h, key)
        as mln_u32_t;
    let mut mgr: *mut mln_hash_mgr_t = &mut *((*h).tbl).offset(index as isize)
        as *mut mln_hash_mgr_t;
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    he = (*mgr).head;
    while !he.is_null() {
        if (*he).removed() == 0
            && ((*h).cmp).expect("non-null function pointer")(h, key, (*he).key) != 0
        {
            return 1 as libc::c_int;
        }
        he = (*he).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_hash_reset(
    mut h: *mut mln_hash_t,
    mut flg: mln_hash_flag_t,
) {
    let mut mgr: *mut mln_hash_mgr_t = 0 as *mut mln_hash_mgr_t;
    let mut end: *mut mln_hash_mgr_t = 0 as *mut mln_hash_mgr_t;
    mgr = (*h).tbl;
    end = ((*h).tbl).offset((*h).len as isize);
    let mut he: *mut mln_hash_entry_t = 0 as *mut mln_hash_entry_t;
    while mgr < end {
        loop {
            he = (*mgr).head;
            if he.is_null() {
                break;
            }
            mln_hash_entry_chain_del(&mut (*mgr).head, &mut (*mgr).tail, he);
            mln_hash_entry_free(h, he, flg);
        }
        mgr = mgr.offset(1);
        mgr;
    }
    (*h).nr_nodes = 0 as libc::c_int as mln_u32_t;
    (*h).iter_tail = 0 as *mut mln_hash_entry_t;
    (*h).iter_head = (*h).iter_tail;
    (*h).iter = (*h).iter_head;
}
#[inline]
unsafe extern "C" fn mln_hash_entry_chain_add(
    mut head: *mut *mut mln_hash_entry_t,
    mut tail: *mut *mut mln_hash_entry_t,
    mut node: *mut mln_hash_entry_t,
) {
    (*node).next = 0 as *mut mln_hash_entry_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_hash_entry_chain_del(
    mut head: *mut *mut mln_hash_entry_t,
    mut tail: *mut *mut mln_hash_entry_t,
    mut node: *mut mln_hash_entry_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_hash_entry_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_hash_entry_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_hash_entry_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_hash_entry_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_hash_entry_iter_chain_add(
    mut head: *mut *mut mln_hash_entry_t,
    mut tail: *mut *mut mln_hash_entry_t,
    mut node: *mut mln_hash_entry_t,
) {
    (*node).iter_next = 0 as *mut mln_hash_entry_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).iter_next = node;
    }
    (*node).iter_prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_hash_entry_iter_chain_del(
    mut head: *mut *mut mln_hash_entry_t,
    mut tail: *mut *mut mln_hash_entry_t,
    mut node: *mut mln_hash_entry_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_hash_entry_t;
            *head = *tail;
        } else {
            *head = (*node).iter_next;
            (**head).iter_prev = 0 as *mut mln_hash_entry_s;
        }
    } else if *tail == node {
        *tail = (*node).iter_prev;
        (**tail).iter_next = 0 as *mut mln_hash_entry_s;
    } else {
        (*(*node).iter_prev).iter_next = (*node).iter_next;
        (*(*node).iter_next).iter_prev = (*node).iter_prev;
    }
    (*node).iter_next = 0 as *mut mln_hash_entry_s;
    (*node).iter_prev = (*node).iter_next;
}
