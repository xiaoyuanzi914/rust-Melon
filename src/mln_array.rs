use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type array_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type array_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type array_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_array_t {
    pub elts: *mut libc::c_void,
    pub size: mln_size_t,
    pub nalloc: mln_size_t,
    pub nelts: mln_size_t,
    pub pool: *mut libc::c_void,
    pub pool_alloc: array_pool_alloc_handler,
    pub pool_free: array_pool_free_handler,
    pub free: array_free,
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_init(
    mut arr: *mut mln_array_t,
    mut free_0: array_free,
    mut size: mln_size_t,
    mut nalloc: mln_size_t,
) -> libc::c_int {
    (*arr).elts = 0 as *mut libc::c_void;
    (*arr).size = size;
    (*arr).nalloc = nalloc;
    (*arr).nelts = 0 as libc::c_int as mln_size_t;
    (*arr).pool = 0 as *mut libc::c_void;
    (*arr).pool_alloc = None;
    (*arr).pool_free = None;
    (*arr).free = free_0;
    return mln_array_alloc(arr, (*arr).nalloc);
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_pool_init(
    mut arr: *mut mln_array_t,
    mut free_0: array_free,
    mut size: mln_size_t,
    mut nalloc: mln_size_t,
    mut pool: *mut libc::c_void,
    mut pool_alloc: array_pool_alloc_handler,
    mut pool_free: array_pool_free_handler,
) -> libc::c_int {
    (*arr).elts = 0 as *mut libc::c_void;
    (*arr).size = size;
    (*arr).nalloc = nalloc;
    (*arr).nelts = 0 as libc::c_int as mln_size_t;
    (*arr).pool = pool;
    (*arr).pool_alloc = pool_alloc;
    (*arr).pool_free = pool_free;
    (*arr).free = free_0;
    return mln_array_alloc(arr, (*arr).nalloc);
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_new(
    mut free_0: array_free,
    mut size: mln_size_t,
    mut nalloc: mln_size_t,
) -> *mut mln_array_t {
    let mut arr: *mut mln_array_t = 0 as *mut mln_array_t;
    arr = malloc(::core::mem::size_of::<mln_array_t>() as libc::c_ulong)
        as *mut mln_array_t;
    if arr.is_null() {
        return 0 as *mut mln_array_t;
    }
    if mln_array_init(arr, free_0, size, nalloc) < 0 as libc::c_int {
        free_0.expect("non-null function pointer")(arr as *mut libc::c_void);
        return 0 as *mut mln_array_t;
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_pool_new(
    mut free_0: array_free,
    mut size: mln_size_t,
    mut nalloc: mln_size_t,
    mut pool: *mut libc::c_void,
    mut pool_alloc: array_pool_alloc_handler,
    mut pool_free: array_pool_free_handler,
) -> *mut mln_array_t {
    let mut arr: *mut mln_array_t = 0 as *mut mln_array_t;
    arr = pool_alloc
        .expect(
            "non-null function pointer",
        )(pool, ::core::mem::size_of::<mln_array_t>() as libc::c_ulong)
        as *mut mln_array_t;
    if arr.is_null() {
        return 0 as *mut mln_array_t;
    }
    if mln_array_pool_init(arr, free_0, size, nalloc, pool, pool_alloc, pool_free)
        < 0 as libc::c_int
    {
        pool_free.expect("non-null function pointer")(arr as *mut libc::c_void);
        return 0 as *mut mln_array_t;
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_destroy(mut arr: *mut mln_array_t) {
    if arr.is_null() {
        return;
    }
    if ((*arr).free).is_some() && (*arr).nelts != 0 {
        let mut p: mln_u8ptr_t = (*arr).elts as mln_u8ptr_t;
        let mut pend: mln_u8ptr_t = ((*arr).elts)
            .offset(((*arr).nelts).wrapping_mul((*arr).size) as isize) as mln_u8ptr_t;
        while p < pend {
            ((*arr).free).expect("non-null function pointer")(p as *mut libc::c_void);
            p = p.offset((*arr).size as isize);
        }
    }
    if !((*arr).pool).is_null() {
        ((*arr).pool_free).expect("non-null function pointer")((*arr).elts);
    } else {
        free((*arr).elts);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_free(mut arr: *mut mln_array_t) {
    if arr.is_null() {
        return;
    }
    if ((*arr).free).is_some() && (*arr).nelts != 0 {
        let mut p: mln_u8ptr_t = (*arr).elts as mln_u8ptr_t;
        let mut pend: mln_u8ptr_t = ((*arr).elts)
            .offset(((*arr).nelts).wrapping_mul((*arr).size) as isize) as mln_u8ptr_t;
        while p < pend {
            ((*arr).free).expect("non-null function pointer")(p as *mut libc::c_void);
            p = p.offset((*arr).size as isize);
        }
    }
    if !((*arr).pool).is_null() {
        ((*arr).pool_free).expect("non-null function pointer")((*arr).elts);
        ((*arr).pool_free).expect("non-null function pointer")(arr as *mut libc::c_void);
    } else {
        free((*arr).elts);
        free(arr as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_reset(mut arr: *mut mln_array_t) {
    if arr.is_null() {
        return;
    }
    if ((*arr).free).is_some() && (*arr).nelts != 0 {
        let mut p: mln_u8ptr_t = (*arr).elts as mln_u8ptr_t;
        let mut pend: mln_u8ptr_t = ((*arr).elts)
            .offset(((*arr).nelts).wrapping_mul((*arr).size) as isize) as mln_u8ptr_t;
        while p < pend {
            ((*arr).free).expect("non-null function pointer")(p as *mut libc::c_void);
            p = p.offset((*arr).size as isize);
        }
    }
    (*arr).nelts = 0 as libc::c_int as mln_size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_push(mut arr: *mut mln_array_t) -> *mut libc::c_void {
    if (*arr).nelts >= (*arr).nalloc {
        if mln_array_alloc(arr, 1 as libc::c_int as mln_size_t) < 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
    }
    let fresh0 = (*arr).nelts;
    (*arr).nelts = ((*arr).nelts).wrapping_add(1);
    return ((*arr).elts).offset(fresh0.wrapping_mul((*arr).size) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_pushn(
    mut arr: *mut mln_array_t,
    mut n: mln_size_t,
) -> *mut libc::c_void {
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    if ((*arr).nelts).wrapping_add(n) > (*arr).nalloc {
        if mln_array_alloc(arr, n) < 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
    }
    ptr = ((*arr).elts).offset(((*arr).nelts).wrapping_mul((*arr).size) as isize)
        as mln_u8ptr_t;
    (*arr)
        .nelts = ((*arr).nelts as libc::c_ulong).wrapping_add(n) as mln_size_t
        as mln_size_t;
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn mln_array_alloc(
    mut arr: *mut mln_array_t,
    mut n: mln_size_t,
) -> libc::c_int {
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut num: mln_size_t = (*arr).nalloc;
    while n.wrapping_add((*arr).nelts) > num {
        num <<= 1 as libc::c_int;
    }
    if !((*arr).pool).is_null() {
        ptr = ((*arr).pool_alloc)
            .expect(
                "non-null function pointer",
            )((*arr).pool, num.wrapping_mul((*arr).size)) as mln_u8ptr_t;
    } else {
        ptr = malloc(num.wrapping_mul((*arr).size)) as mln_u8ptr_t;
    }
    if ptr.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        ptr as *mut libc::c_void,
        (*arr).elts,
        ((*arr).nelts).wrapping_mul((*arr).size),
    );
    if !((*arr).pool).is_null() {
        ((*arr).pool_free).expect("non-null function pointer")((*arr).elts);
    } else {
        free((*arr).elts);
    }
    (*arr).elts = ptr as *mut libc::c_void;
    (*arr).nalloc = num;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_array_pop(mut arr: *mut mln_array_t) {
    if arr.is_null() || (*arr).nelts == 0 {
        return;
    }
    if ((*arr).free).is_some() {
        ((*arr).free)
            .expect(
                "non-null function pointer",
            )(
            ((*arr).elts)
                .offset(
                    ((*arr).nelts)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*arr).size) as isize,
                ),
        );
    }
    (*arr).nelts = ((*arr).nelts).wrapping_sub(1);
    (*arr).nelts;
}
