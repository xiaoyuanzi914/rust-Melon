use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn mln_string_const_ndup(
        str: *mut libc::c_char,
        size: mln_s32_t,
    ) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn mln_string_slice(
        s: *mut mln_string_t,
        sep_array: *const libc::c_char,
    ) -> *mut mln_string_t;
    fn mln_string_slice_free(array: *mut mln_string_t);
    fn mln_array_new(
        free_0: array_free,
        size: mln_size_t,
        nalloc: mln_size_t,
    ) -> *mut mln_array_t;
    fn mln_array_free(arr: *mut mln_array_t);
    fn mln_array_push(arr: *mut mln_array_t) -> *mut libc::c_void;
    fn mln_array_pop(arr: *mut mln_array_t);
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_rbtree_delete(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_iterate(
        t: *mut mln_rbtree_t,
        handler: rbtree_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s64_t = libc::c_long;
pub type mln_s8ptr_t = *mut libc::c_char;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_off_t = off_t;
pub type mln_uauto_t = libc::c_ulong;
pub type mln_alloc_shm_lock_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_alloc_s {
    pub mem: *mut libc::c_void,
    pub shm_size: mln_size_t,
    pub locker: *mut libc::c_void,
    pub lock: mln_alloc_shm_lock_cb_t,
    pub unlock: mln_alloc_shm_lock_cb_t,
    pub parent: *mut mln_alloc_s,
    pub mgr_tbl: [mln_alloc_mgr_t; 35],
    pub large_used_head: *mut mln_alloc_chunk_t,
    pub large_used_tail: *mut mln_alloc_chunk_t,
    pub shm_head: *mut mln_alloc_shm_t,
    pub shm_tail: *mut mln_alloc_shm_t,
}
pub type mln_alloc_shm_t = mln_alloc_shm_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_alloc_shm_s {
    pub prev: *mut mln_alloc_shm_s,
    pub next: *mut mln_alloc_shm_s,
    pub pool: *mut mln_alloc_t,
    pub addr: *mut libc::c_void,
    pub size: mln_size_t,
    pub nfree: mln_u32_t,
    #[bitfield(name = "base", ty = "mln_u32_t", bits = "0..=30")]
    #[bitfield(name = "large", ty = "mln_u32_t", bits = "31..=31")]
    pub base_large: [u8; 4],
    pub bitmap: [mln_u8_t; 4096],
}
pub type mln_alloc_t = mln_alloc_s;
pub type mln_alloc_chunk_t = mln_alloc_chunk_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_alloc_chunk_s {
    pub prev: *mut mln_alloc_chunk_s,
    pub next: *mut mln_alloc_chunk_s,
    pub refer: mln_size_t,
    pub count: mln_size_t,
    pub mgr: *mut mln_alloc_mgr_t,
    pub blks: [*mut mln_alloc_blk_t; 5],
}
pub type mln_alloc_blk_t = mln_alloc_blk_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_alloc_blk_s {
    pub prev: *mut mln_alloc_blk_s,
    pub next: *mut mln_alloc_blk_s,
    pub pool: *mut mln_alloc_t,
    pub data: *mut libc::c_void,
    pub chunk: *mut mln_alloc_chunk_t,
    pub blk_size: mln_size_t,
    #[bitfield(name = "is_large", ty = "mln_size_t", bits = "0..=0")]
    #[bitfield(name = "in_used", ty = "mln_size_t", bits = "1..=1")]
    #[bitfield(name = "padding", ty = "mln_size_t", bits = "2..=31")]
    pub is_large_in_used_padding: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_alloc_mgr_t = mln_alloc_mgr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_alloc_mgr_s {
    pub blk_size: mln_size_t,
    pub free_head: *mut mln_alloc_blk_t,
    pub free_tail: *mut mln_alloc_blk_t,
    pub used_head: *mut mln_alloc_blk_t,
    pub used_tail: *mut mln_alloc_blk_t,
    pub chunk_head: *mut mln_alloc_chunk_t,
    pub chunk_tail: *mut mln_alloc_chunk_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_string_t {
    pub data: mln_u8ptr_t,
    pub len: mln_u64_t,
    #[bitfield(name = "data_ref", ty = "mln_uauto_t", bits = "0..=0")]
    #[bitfield(name = "pool", ty = "mln_uauto_t", bits = "1..=1")]
    #[bitfield(name = "ref_0", ty = "mln_uauto_t", bits = "2..=31")]
    pub data_ref_pool_ref_0: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_json_s {
    pub type_0: json_type,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub m_j_obj: *mut mln_rbtree_t,
    pub m_j_array: *mut mln_array_t,
    pub m_j_string: *mut mln_string_t,
    pub m_j_number: libc::c_double,
    pub m_j_true: mln_u8_t,
    pub m_j_false: mln_u8_t,
    pub m_j_null: mln_u8ptr_t,
}
pub type json_type = libc::c_uint;
pub const M_JSON_NULL: json_type = 7;
pub const M_JSON_FALSE: json_type = 6;
pub const M_JSON_TRUE: json_type = 5;
pub const M_JSON_NUM: json_type = 4;
pub const M_JSON_STRING: json_type = 3;
pub const M_JSON_ARRAY: json_type = 2;
pub const M_JSON_OBJECT: json_type = 1;
pub const M_JSON_NONE: json_type = 0;
pub type mln_json_t = mln_json_s;
pub type mln_json_iterator_t = Option::<
    unsafe extern "C" fn(*mut mln_json_t, *mut libc::c_void) -> libc::c_int,
>;
pub type mln_json_object_iterator_t = Option::<
    unsafe extern "C" fn(
        *mut mln_json_t,
        *mut mln_json_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type mln_json_array_iterator_t = Option::<
    unsafe extern "C" fn(*mut mln_json_t, *mut libc::c_void) -> libc::c_int,
>;
pub type mln_json_call_func_t = mln_json_array_iterator_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_json_kv_t {
    pub key: mln_json_t,
    pub val: mln_json_t,
    pub node: mln_rbtree_node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_json_call_attr {
    pub callback: mln_json_call_func_t,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_json_policy_t {
    pub depth: mln_size_t,
    pub key_len: mln_size_t,
    pub str_len: mln_size_t,
    pub arr_elem_num: mln_size_t,
    pub obj_kv_num: mln_size_t,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_json_tmp_s {
    pub ptr1: *mut libc::c_void,
    pub ptr2: *mut libc::c_void,
    pub ptr3: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn mln_blk_chain_add(
    mut head: *mut *mut mln_alloc_blk_t,
    mut tail: *mut *mut mln_alloc_blk_t,
    mut node: *mut mln_alloc_blk_t,
) {
    (*node).next = 0 as *mut mln_alloc_blk_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_blk_chain_del(
    mut head: *mut *mut mln_alloc_blk_t,
    mut tail: *mut *mut mln_alloc_blk_t,
    mut node: *mut mln_alloc_blk_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_alloc_blk_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_alloc_blk_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_alloc_blk_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_alloc_blk_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_alloc_free(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut ch: *mut mln_alloc_chunk_t = 0 as *mut mln_alloc_chunk_t;
    let mut am: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
    let mut blk: *mut mln_alloc_blk_t = 0 as *mut mln_alloc_blk_t;
    blk = (ptr as mln_u8ptr_t)
        .offset(-(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize))
        as *mut mln_alloc_blk_t;
    pool = (*blk).pool;
    if !((*pool).mem).is_null() {
        mln_alloc_free_shm(ptr);
        return;
    }
    if (*blk).is_large() != 0 {
        mln_chunk_chain_del(
            &mut (*pool).large_used_head,
            &mut (*pool).large_used_tail,
            (*blk).chunk,
        );
        if !((*pool).parent).is_null() {
            if !((*(*pool).parent).mem).is_null() {
                if ((*(*pool).parent).lock)
                    .expect("non-null function pointer")((*(*pool).parent).locker)
                    != 0 as libc::c_int
                {
                    return;
                }
            }
            mln_alloc_free((*blk).chunk as *mut libc::c_void);
            if !((*(*pool).parent).mem).is_null() {
                ((*(*pool).parent).unlock)
                    .expect("non-null function pointer")((*(*pool).parent).locker);
            }
        } else {
            free((*blk).chunk as *mut libc::c_void);
        }
        return;
    }
    ch = (*blk).chunk;
    am = (*ch).mgr;
    (*blk).set_in_used(0 as libc::c_int as mln_size_t);
    mln_blk_chain_del(&mut (*am).used_head, &mut (*am).used_tail, blk);
    mln_blk_chain_add(&mut (*am).free_head, &mut (*am).free_tail, blk);
    (*ch).refer = ((*ch).refer).wrapping_sub(1);
    if (*ch).refer == 0
        && {
            (*ch).count = ((*ch).count).wrapping_add(1);
            (*ch).count > 1023 as libc::c_int as libc::c_ulong
        }
    {
        let mut blks: *mut *mut mln_alloc_blk_t = ((*ch).blks).as_mut_ptr();
        while !(*blks).is_null() {
            let fresh0 = blks;
            blks = blks.offset(1);
            mln_blk_chain_del(&mut (*am).free_head, &mut (*am).free_tail, *fresh0);
        }
        mln_chunk_chain_del(&mut (*am).chunk_head, &mut (*am).chunk_tail, ch);
        if !((*pool).parent).is_null() {
            if !((*(*pool).parent).mem).is_null() {
                if ((*(*pool).parent).lock)
                    .expect("non-null function pointer")((*(*pool).parent).locker)
                    != 0 as libc::c_int
                {
                    return;
                }
            }
            mln_alloc_free(ch as *mut libc::c_void);
            if !((*(*pool).parent).mem).is_null() {
                ((*(*pool).parent).unlock)
                    .expect("non-null function pointer")((*(*pool).parent).locker);
            }
        } else {
            free(ch as *mut libc::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn mln_chunk_chain_del(
    mut head: *mut *mut mln_alloc_chunk_t,
    mut tail: *mut *mut mln_alloc_chunk_t,
    mut node: *mut mln_alloc_chunk_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_alloc_chunk_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_alloc_chunk_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_alloc_chunk_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_alloc_chunk_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_alloc_free_shm(mut ptr: *mut libc::c_void) {
    let mut blk: *mut mln_alloc_blk_t = 0 as *mut mln_alloc_blk_t;
    let mut as_0: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    let mut Boff: mln_off_t = 0;
    let mut boff: mln_off_t = 0;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pend: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    blk = (ptr as mln_u8ptr_t)
        .offset(-(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize))
        as *mut mln_alloc_blk_t;
    as_0 = (*blk).chunk as *mut mln_alloc_shm_t;
    if (*as_0).large() == 0 {
        Boff = ((*blk).padding() as libc::c_int >> 8 as libc::c_int
            & 0xffff as libc::c_int) as mln_off_t;
        boff = ((*blk).padding() as libc::c_int & 0xff as libc::c_int) as mln_off_t;
        (*blk).set_in_used(0 as libc::c_int as mln_size_t);
        p = ((*as_0).bitmap).as_mut_ptr().offset(Boff as isize);
        n = ((*blk).blk_size)
            .wrapping_add(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong)
            .wrapping_add(64 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(64 as libc::c_int as libc::c_ulong) as libc::c_int;
        i = boff as libc::c_int;
        pend = ((*as_0).bitmap).as_mut_ptr().offset(4096 as libc::c_int as isize);
        while p < pend {
            *p = (*p as libc::c_int
                & !((1 as libc::c_int as mln_u8_t as libc::c_int) << i))
                as libc::c_uchar;
            (*as_0).nfree = ((*as_0).nfree).wrapping_add(1);
            (*as_0).nfree;
            n -= 1;
            if n <= 0 as libc::c_int {
                break;
            }
            i -= 1;
            if i < 0 as libc::c_int {
                i = 7 as libc::c_int;
                p = p.offset(1);
                p;
            }
        }
    }
    if (*as_0).large() as libc::c_int != 0 || (*as_0).nfree == (*as_0).base() {
        mln_alloc_shm_chain_del(
            &mut (*(*as_0).pool).shm_head,
            &mut (*(*as_0).pool).shm_tail,
            as_0,
        );
    }
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_chain_del(
    mut head: *mut *mut mln_alloc_shm_t,
    mut tail: *mut *mut mln_alloc_shm_t,
    mut node: *mut mln_alloc_shm_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_alloc_shm_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_alloc_shm_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_alloc_shm_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_alloc_shm_s;
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
unsafe extern "C" fn mln_json_kv_cmp(
    mut kv1: *const mln_json_kv_t,
    mut kv2: *const mln_json_kv_t,
) -> libc::c_int {
    return mln_string_strcmp((*kv1).key.data.m_j_string, (*kv2).key.data.m_j_string);
}
#[inline]
unsafe extern "C" fn mln_json_kv_free(mut kv: *mut mln_json_kv_t) {
    if kv.is_null() {
        return;
    }
    mln_json_destroy(&mut (*kv).key);
    mln_json_destroy(&mut (*kv).val);
    free(kv as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn __mln_json_obj_init(mut j: *mut mln_json_t) -> libc::c_int {
    (*j).type_0 = M_JSON_OBJECT;
    (*j).data.m_j_obj = mln_rbtree_new(0 as *mut mln_rbtree_attr);
    if ((*j).data.m_j_obj).is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_obj_init(mut j: *mut mln_json_t) -> libc::c_int {
    return __mln_json_obj_init(j);
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_obj_update(
    mut j: *mut mln_json_t,
    mut key: *mut mln_json_t,
    mut val: *mut mln_json_t,
) -> libc::c_int {
    return __mln_json_obj_update(j, key, val);
}
#[inline]
unsafe extern "C" fn __mln_json_obj_update(
    mut j: *mut mln_json_t,
    mut key: *mut mln_json_t,
    mut val: *mut mln_json_t,
) -> libc::c_int {
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut kv: mln_json_kv_t = mln_json_kv_t {
        key: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        val: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        node: mln_rbtree_node_t {
            data: 0 as *mut libc::c_void,
            prev: 0 as *mut mln_rbtree_node_s,
            next: 0 as *mut mln_rbtree_node_s,
            parent: 0 as *mut mln_rbtree_node_s,
            left: 0 as *mut mln_rbtree_node_s,
            right: 0 as *mut mln_rbtree_node_s,
            nofree_color: [0; 4],
            c2rust_padding: [0; 4],
        },
    };
    let mut pkv: *mut mln_json_kv_t = 0 as *mut mln_json_kv_t;
    if !((*key).type_0 as libc::c_uint == M_JSON_STRING as libc::c_int as libc::c_uint)
        || !((*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    kv.key = *key;
    rn = ({
        let mut tree: *mut mln_rbtree_t = (*j).data.m_j_obj;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_json_kv_cmp(&mut kv, (*ret_node).data as *const mln_json_kv_t);
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    if rn == &mut (*(*j).data.m_j_obj).nil as *mut mln_rbtree_node_t {
        pkv = malloc(::core::mem::size_of::<mln_json_kv_t>() as libc::c_ulong)
            as *mut mln_json_kv_t;
        if pkv.is_null() {
            return -(1 as libc::c_int);
        }
        (*pkv).key = *key;
        (*pkv).val = *val;
        rn = ({
            (*pkv).node.data = pkv as *mut libc::c_void;
            ((*pkv).node).set_nofree(1 as libc::c_int as mln_u32_t);
            &mut (*pkv).node
        });
        ({
            let mut tree: *mut mln_rbtree_t = (*j).data.m_j_obj;
            let mut y: *mut mln_rbtree_node_t = &mut (*tree).nil;
            let mut x: *mut mln_rbtree_node_t = (*tree).root;
            let mut nil: *mut mln_rbtree_node_t = &mut (*tree).nil;
            while x != nil {
                y = x;
                if mln_json_kv_cmp(
                    (*rn).data as *const mln_json_kv_t,
                    (*x).data as *const mln_json_kv_t,
                ) < 0 as libc::c_int
                {
                    x = (*x).left;
                } else {
                    x = (*x).right;
                }
            }
            (*rn).parent = y;
            if y == nil {
                (*tree).root = rn;
            } else if mln_json_kv_cmp(
                (*rn).data as *const mln_json_kv_t,
                (*y).data as *const mln_json_kv_t,
            ) < 0 as libc::c_int
            {
                (*y).left = rn;
            } else {
                (*y).right = rn;
            }
            (*rn).right = nil;
            (*rn).left = (*rn).right;
            (*rn).set_color(M_RB_RED as libc::c_int as mln_u32_t);
            rbtree_insert_fixup(tree, rn);
            if (*tree).min == nil {
                (*tree).min = rn;
            } else if mln_json_kv_cmp(
                (*rn).data as *const mln_json_kv_t,
                (*(*tree).min).data as *const mln_json_kv_t,
            ) < 0 as libc::c_int
            {
                (*tree).min = rn;
            }
            (*tree).nr_node = ((*tree).nr_node).wrapping_add(1);
            (*tree).nr_node;
            mln_rbtree_chain_add(&mut (*tree).head, &mut (*tree).tail, rn);
            //compile_error!("Function call expression is not supposed to be used")
        });
    } else {
        pkv = (*rn).data as *mut mln_json_kv_t;
        mln_json_destroy(&mut (*pkv).key);
        mln_json_destroy(&mut (*pkv).val);
        (*pkv).key = *key;
        (*pkv).val = *val;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_obj_search(
    mut j: *mut mln_json_t,
    mut key: *mut mln_string_t,
) -> *mut mln_json_t {
    if !((*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint) {
        return 0 as *mut mln_json_t;
    }
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut kv: mln_json_kv_t = mln_json_kv_t {
        key: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        val: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        node: mln_rbtree_node_t {
            data: 0 as *mut libc::c_void,
            prev: 0 as *mut mln_rbtree_node_s,
            next: 0 as *mut mln_rbtree_node_s,
            parent: 0 as *mut mln_rbtree_node_s,
            left: 0 as *mut mln_rbtree_node_s,
            right: 0 as *mut mln_rbtree_node_s,
            nofree_color: [0; 4],
            c2rust_padding: [0; 4],
        },
    };
    ({
        let mut json: *mut mln_json_t = &mut kv.key;
        (*json).type_0 = M_JSON_STRING;
        (*json).data.m_j_string = key;
        (*json).data.m_j_string
    });
    rn = ({
        let mut tree: *mut mln_rbtree_t = (*j).data.m_j_obj;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_json_kv_cmp(&mut kv, (*ret_node).data as *const mln_json_kv_t);
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    if rn == &mut (*(*j).data.m_j_obj).nil as *mut mln_rbtree_node_t {
        return 0 as *mut mln_json_t;
    }
    return &mut (*((*rn).data as *mut mln_json_kv_t)).val;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_obj_remove(
    mut j: *mut mln_json_t,
    mut key: *mut mln_string_t,
) {
    if !((*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint) {
        return;
    }
    let mut kv: mln_json_kv_t = mln_json_kv_t {
        key: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        val: mln_json_t {
            type_0: M_JSON_NONE,
            data: C2RustUnnamed {
                m_j_obj: 0 as *mut mln_rbtree_t,
            },
        },
        node: mln_rbtree_node_t {
            data: 0 as *mut libc::c_void,
            prev: 0 as *mut mln_rbtree_node_s,
            next: 0 as *mut mln_rbtree_node_s,
            parent: 0 as *mut mln_rbtree_node_s,
            left: 0 as *mut mln_rbtree_node_s,
            right: 0 as *mut mln_rbtree_node_s,
            nofree_color: [0; 4],
            c2rust_padding: [0; 4],
        },
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    ({
        let mut json: *mut mln_json_t = &mut kv.key;
        (*json).type_0 = M_JSON_STRING;
        (*json).data.m_j_string = key;
        (*json).data.m_j_string
    });
    rn = ({
        let mut tree: *mut mln_rbtree_t = (*j).data.m_j_obj;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_json_kv_cmp(&mut kv, (*ret_node).data as *const mln_json_kv_t);
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    if rn == &mut (*(*j).data.m_j_obj).nil as *mut mln_rbtree_node_t {
        return;
    }
    mln_rbtree_delete((*j).data.m_j_obj, rn);
    let mut nofree: mln_u32_t = (*rn).nofree();
    if !((*rn).data).is_null() {
        mln_json_kv_free((*rn).data as *mut mln_json_kv_t);
    }
    if nofree == 0 {
        if !((*(*j).data.m_j_obj).pool).is_null() {
            ((*(*j).data.m_j_obj).pool_free)
                .expect("non-null function pointer")(rn as *mut libc::c_void);
        } else {
            free(rn as *mut libc::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn __mln_json_array_init(mut j: *mut mln_json_t) -> libc::c_int {
    (*j)
        .data
        .m_j_array = mln_array_new(
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut mln_json_t) -> ()>,
            array_free,
        >(Some(mln_json_destroy as unsafe extern "C" fn(*mut mln_json_t) -> ())),
        ::core::mem::size_of::<mln_json_t>() as libc::c_ulong,
        31 as libc::c_int as mln_size_t,
    );
    if ((*j).data.m_j_array).is_null() {
        return -(1 as libc::c_int);
    }
    (*j).type_0 = M_JSON_ARRAY;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_init(mut j: *mut mln_json_t) -> libc::c_int {
    return __mln_json_array_init(j);
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_search(
    mut j: *mut mln_json_t,
    mut index: mln_uauto_t,
) -> *mut mln_json_t {
    if !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint) {
        return 0 as *mut mln_json_t;
    }
    if index >= (*(*j).data.m_j_array).nelts {
        return 0 as *mut mln_json_t;
    }
    return &mut *((*(*j).data.m_j_array).elts as *mut mln_json_t).offset(index as isize)
        as *mut mln_json_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_length(mut j: *mut mln_json_t) -> mln_uauto_t {
    if !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int as mln_uauto_t;
    }
    return (*(*j).data.m_j_array).nelts;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_append(
    mut j: *mut mln_json_t,
    mut value: *mut mln_json_t,
) -> libc::c_int {
    return __mln_json_array_append(j, value);
}
#[inline]
unsafe extern "C" fn __mln_json_array_append(
    mut j: *mut mln_json_t,
    mut value: *mut mln_json_t,
) -> libc::c_int {
    let mut elem: *mut mln_json_t = 0 as *mut mln_json_t;
    if !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int);
    }
    elem = mln_array_push((*j).data.m_j_array) as *mut mln_json_t;
    if elem.is_null() {
        return -(1 as libc::c_int);
    }
    *elem = *value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_update(
    mut j: *mut mln_json_t,
    mut value: *mut mln_json_t,
    mut index: mln_uauto_t,
) -> libc::c_int {
    if !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int);
    }
    let mut arr: *mut mln_array_t = (*j).data.m_j_array;
    if index >= (*arr).nelts {
        return -(1 as libc::c_int);
    }
    mln_json_destroy(&mut *((*arr).elts as *mut mln_json_t).offset(index as isize));
    *((*arr).elts as *mut mln_json_t).offset(index as isize) = *value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_array_remove(
    mut j: *mut mln_json_t,
    mut index: mln_uauto_t,
) {
    if j.is_null()
        || !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        return;
    }
    mln_array_pop((*j).data.m_j_array);
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_destroy(mut j: *mut mln_json_t) {
    if j.is_null() {
        return;
    }
    match (*j).type_0 as libc::c_uint {
        1 => {
            let mut tree: *mut mln_rbtree_t = (*j).data.m_j_obj;
            if !tree.is_null() {
                let mut fr: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
                loop {
                    fr = (*tree).tail;
                    if fr.is_null() {
                        break;
                    }
                    mln_rbtree_chain_del(&mut (*tree).head, &mut (*tree).tail, fr);
                    let mut nofree: mln_u32_t = (*fr).nofree();
                    if !((*fr).data).is_null() {
                        mln_json_kv_free((*fr).data as *mut mln_json_kv_t);
                    }
                    if nofree == 0 {
                        if !((*tree).pool).is_null() {
                            ((*tree).pool_free)
                                .expect(
                                    "non-null function pointer",
                                )(fr as *mut libc::c_void);
                        } else {
                            free(fr as *mut libc::c_void);
                        }
                    }
                }
                if !((*tree).pool).is_null() {
                    ((*tree).pool_free)
                        .expect("non-null function pointer")(tree as *mut libc::c_void);
                } else {
                    free(tree as *mut libc::c_void);
                }
            }
        }
        2 => {
            mln_array_free((*j).data.m_j_array);
        }
        3 => {
            if !((*j).data.m_j_string).is_null() {
                let mut __s: *mut mln_string_t = (*j).data.m_j_string;
                if !__s.is_null() {
                    let ref mut fresh1 = (*__s).ref_0();
                    let fresh2 = *fresh1;
                    *fresh1 = (*fresh1).wrapping_sub(1);
                    if fresh2 <= 1 as libc::c_int as libc::c_ulong {
                        if (*__s).data_ref() == 0 && !((*__s).data).is_null() {
                            if (*__s).pool() != 0 {
                                mln_alloc_free((*__s).data as *mut libc::c_void);
                            } else {
                                free((*__s).data as *mut libc::c_void);
                            }
                        }
                        if (*__s).pool() != 0 {
                            mln_alloc_free(__s as *mut libc::c_void);
                        } else {
                            free(__s as *mut libc::c_void);
                        }
                    }
                }
            }
        }
        0 | 4 | 5 | 6 | 7 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_dump(
    mut j: *mut mln_json_t,
    mut n_space: libc::c_int,
    mut prefix: *mut libc::c_char,
) {
    if j.is_null() {
        return;
    }
    let mut i: libc::c_int = 0;
    let mut space: libc::c_int = n_space + 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n_space {
        printf(b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    if !prefix.is_null() {
        printf(b"%s \0" as *const u8 as *const libc::c_char, prefix);
    }
    match (*j).type_0 as libc::c_uint {
        1 => {
            printf(b"type:object\n\0" as *const u8 as *const libc::c_char);
            mln_rbtree_iterate(
                (*j).data.m_j_obj,
                Some(
                    mln_json_dump_obj_iterate_handler
                        as unsafe extern "C" fn(
                            *mut mln_rbtree_node_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                &mut space as *mut libc::c_int as *mut libc::c_void,
            );
        }
        2 => {
            printf(b"type:array\n\0" as *const u8 as *const libc::c_char);
            let mut elem: *mut mln_json_t = (*(*j).data.m_j_array).elts
                as *mut mln_json_t;
            let mut end: *mut mln_json_t = elem
                .offset((*(*j).data.m_j_array).nelts as isize);
            while elem < end {
                mln_json_dump(
                    elem,
                    space,
                    b"Array member:\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                elem = elem.offset(1);
                elem;
            }
        }
        3 => {
            if !((*j).data.m_j_string).is_null()
                && !((*(*j).data.m_j_string).data).is_null()
            {
                printf(
                    b"type:string val:[%s]\n\0" as *const u8 as *const libc::c_char,
                    (*(*j).data.m_j_string).data as *mut libc::c_char,
                );
            }
        }
        4 => {
            printf(
                b"type:number val:[%f]\n\0" as *const u8 as *const libc::c_char,
                (*j).data.m_j_number,
            );
        }
        5 => {
            printf(b"type:true val:[true]\n\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            printf(b"type:false val:[false]\n\0" as *const u8 as *const libc::c_char);
        }
        7 => {
            printf(b"type:NULL val:[NULL]\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"type:none\n\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn mln_json_dump_obj_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut space: *mut libc::c_int = data as *mut libc::c_int;
    let mut kv: *mut mln_json_kv_t = (*node).data as *mut mln_json_kv_t;
    mln_json_dump(
        &mut (*kv).key,
        *space,
        b"Object key:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mln_json_dump(
        &mut (*kv).val,
        *space,
        b"Object value:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_decode(
    mut jstr: *mut mln_string_t,
    mut out: *mut mln_json_t,
    mut policy: *mut mln_json_policy_t,
) -> libc::c_int {
    if jstr.is_null() || out.is_null() {
        return -(1 as libc::c_int);
    }
    (*out).type_0 = M_JSON_NONE;
    if mln_json_parse_json(
        out,
        (*jstr).data as *mut libc::c_char,
        (*jstr).len as libc::c_int,
        0 as libc::c_int as mln_uauto_t,
        policy,
        0 as libc::c_int,
        0 as libc::c_int as mln_size_t,
    ) != 0 as libc::c_int
    {
        mln_json_destroy(out);
        return -(1 as libc::c_int);
    }
    if !((*out).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint)
        && !((*out).type_0 as libc::c_uint
            == M_JSON_ARRAY as libc::c_int as libc::c_uint)
    {
        mln_json_destroy(out);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_parse_json(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
    mut policy: *mut mln_json_policy_t,
    mut obj_key: libc::c_int,
    mut depth: mln_size_t,
) -> libc::c_int {
    if jstr.is_null() {
        return -(1 as libc::c_int);
    }
    let mut left: libc::c_int = 0;
    mln_json_jumpoff_blank(&mut jstr, &mut len);
    if len <= 0 as libc::c_int {
        return len;
    }
    match *jstr.offset(0 as libc::c_int as isize) as libc::c_int {
        123 => return mln_json_parse_obj(j, jstr, len, index, policy, depth),
        91 => return mln_json_parse_array(j, jstr, len, index, policy, depth),
        34 => return mln_json_parse_string(j, jstr, len, index, policy, obj_key),
        _ => {
            if *jstr.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *jstr.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
                || *jstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            {
                return mln_json_parse_digit(j, jstr, len, index);
            }
            left = mln_json_parse_true(j, jstr, len, index);
            if left >= 0 as libc::c_int {
                return left;
            }
            left = mln_json_parse_false(j, jstr, len, index);
            if left >= 0 as libc::c_int {
                return left;
            }
        }
    }
    return mln_json_parse_null(j, jstr, len, index);
}
#[inline]
unsafe extern "C" fn mln_json_parse_obj(
    mut val: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
    mut policy: *mut mln_json_policy_t,
    mut depth: mln_size_t,
) -> libc::c_int {
    let mut left: libc::c_int = 0;
    let mut key: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    let mut v: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    jstr = jstr.offset(1);
    jstr;
    len -= 1;
    if len <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if __mln_json_obj_init(val) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !policy.is_null() && (*policy).depth != 0
        && depth.wrapping_add(1 as libc::c_int as libc::c_ulong) > (*policy).depth
    {
        (*policy).error = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        key.type_0 = M_JSON_NONE;
        v.type_0 = M_JSON_NONE;
        mln_json_jumpoff_blank(&mut jstr, &mut len);
        if *jstr.offset(0 as libc::c_int as isize) as libc::c_int != '}' as i32 {
            left = mln_json_parse_json(
                &mut key,
                jstr,
                len,
                0 as libc::c_int as mln_uauto_t,
                policy,
                1 as libc::c_int,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if left <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            if !(key.type_0 as libc::c_uint
                == M_JSON_STRING as libc::c_int as libc::c_uint)
            {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            jstr = jstr.offset((len - left) as isize);
            len = left;
            if len <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            mln_json_jumpoff_blank(&mut jstr, &mut len);
            if len <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            if *jstr.offset(0 as libc::c_int as isize) as libc::c_int != ':' as i32 {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            jstr = jstr.offset(1);
            jstr;
            len -= 1;
            if len <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            left = mln_json_parse_json(
                &mut v,
                jstr,
                len,
                0 as libc::c_int as mln_uauto_t,
                policy,
                0 as libc::c_int,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if left <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            jstr = jstr.offset((len - left) as isize);
            len = left;
            mln_json_jumpoff_blank(&mut jstr, &mut len);
            if len <= 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            if __mln_json_obj_update(val, &mut key, &mut v) < 0 as libc::c_int {
                mln_json_destroy(&mut key);
                mln_json_destroy(&mut v);
                return -(1 as libc::c_int);
            }
            if !policy.is_null() && (*policy).obj_kv_num != 0
                && (*(*val).data.m_j_obj).nr_node > (*policy).obj_kv_num
            {
                (*policy).error = 5 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        if !(*jstr.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32) {
            break;
        }
        jstr = jstr.offset(1);
        jstr;
        len -= 1;
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if *jstr.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32 {
        jstr = jstr.offset(1);
        jstr;
        len -= 1;
        len;
        mln_json_jumpoff_blank(&mut jstr, &mut len);
        return len;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_json_parse_array(
    mut val: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
    mut policy: *mut mln_json_policy_t,
    mut depth: mln_size_t,
) -> libc::c_int {
    let mut left: libc::c_int = 0;
    let mut j: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    let mut cnt: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    if __mln_json_array_init(val) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    jstr = jstr.offset(1);
    jstr;
    len -= 1;
    if len <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !policy.is_null() && (*policy).depth != 0
        && depth.wrapping_add(1 as libc::c_int as libc::c_ulong) > (*policy).depth
    {
        (*policy).error = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        j.type_0 = M_JSON_NONE;
        if *jstr.offset(0 as libc::c_int as isize) as libc::c_int != ']' as i32 {
            cnt = cnt.wrapping_add(1);
            left = mln_json_parse_json(
                &mut j,
                jstr,
                len,
                cnt,
                policy,
                0 as libc::c_int,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if left <= 0 as libc::c_int {
                mln_json_destroy(&mut j);
                return -(1 as libc::c_int);
            }
            jstr = jstr.offset((len - left) as isize);
            len = left;
            if len <= 0 as libc::c_int {
                mln_json_destroy(&mut j);
                return -(1 as libc::c_int);
            }
        }
        if __mln_json_array_append(val, &mut j) < 0 as libc::c_int {
            mln_json_destroy(&mut j);
            return -(1 as libc::c_int);
        }
        if !policy.is_null() && (*policy).arr_elem_num != 0
            && (*(*val).data.m_j_array).nelts > (*policy).arr_elem_num
        {
            (*policy).error = 4 as libc::c_int;
            return -(1 as libc::c_int);
        }
        mln_json_jumpoff_blank(&mut jstr, &mut len);
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if !(*jstr.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32) {
            break;
        }
        jstr = jstr.offset(1);
        jstr;
        len -= 1;
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if *jstr.offset(0 as libc::c_int as isize) as libc::c_int == ']' as i32 {
        jstr = jstr.offset(1);
        jstr;
        len -= 1;
        len;
        mln_json_jumpoff_blank(&mut jstr, &mut len);
        return len;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_json_parse_string(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
    mut policy: *mut mln_json_policy_t,
    mut obj_key: libc::c_int,
) -> libc::c_int {
    let mut p: *mut mln_u8_t = 0 as *mut mln_u8_t;
    let mut flag: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    let mut plen: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut str: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    jstr = jstr.offset(1);
    jstr;
    len -= 1;
    if len <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    p = jstr as mln_u8ptr_t;
    plen = len;
    while plen > 0 as libc::c_int {
        if flag == 0 && *p as libc::c_int == '\\' as i32 as mln_u8_t as libc::c_int {
            flag = 1 as libc::c_int as mln_u8_t;
        } else {
            if *p as libc::c_int == '"' as i32 as mln_u8_t as libc::c_int
                && (p == jstr as mln_u8ptr_t || flag == 0)
            {
                break;
            }
            if flag != 0 {
                flag = 0 as libc::c_int as mln_u8_t;
            }
        }
        p = p.offset(1);
        p;
        count += 1;
        count;
        plen -= 1;
        plen;
    }
    if plen <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    buf = mln_json_parse_string_fetch(jstr as mln_u8ptr_t, &mut count);
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    if !policy.is_null() {
        if obj_key != 0 {
            if (*policy).key_len != 0 && count as libc::c_ulong > (*policy).key_len {
                (*policy).error = 2 as libc::c_int;
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        } else if (*policy).str_len != 0 && count as libc::c_ulong > (*policy).str_len {
            (*policy).error = 3 as libc::c_int;
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    }
    str = mln_string_const_ndup(buf as *mut libc::c_char, count);
    free(buf as *mut libc::c_void);
    if str.is_null() {
        return -(1 as libc::c_int);
    }
    ({
        let mut json: *mut mln_json_t = j;
        (*json).type_0 = M_JSON_STRING;
        (*json).data.m_j_string = str;
        (*json).data.m_j_string
    });
    plen -= 1;
    return plen;
}
unsafe extern "C" fn mln_json_parse_string_fetch(
    mut jstr: mln_u8ptr_t,
    mut len: *mut libc::c_int,
) -> mln_u8ptr_t {
    let mut l: libc::c_int = *len;
    let mut c: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut hex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: mln_u8ptr_t = jstr;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut q: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    buf = malloc(l as libc::c_ulong) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as mln_u8ptr_t;
    }
    q = buf;
    while l > 0 as libc::c_int {
        c = mln_json_get_char(&mut p, &mut l, &mut hex);
        if c < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as mln_u8ptr_t;
        } else if c == 0 as libc::c_int {
            mln_json_encode_utf8(hex, &mut q, &mut count);
        } else {
            let fresh3 = q;
            q = q.offset(1);
            *fresh3 = c as mln_u8_t;
            count += 1;
            count;
        }
    }
    *len = count;
    return buf;
}
unsafe extern "C" fn mln_json_encode_utf8(
    mut u: libc::c_uint,
    mut b: *mut mln_u8ptr_t,
    mut count: *mut libc::c_int,
) {
    let mut buf: mln_u8ptr_t = *b;
    if u <= 0x7f as libc::c_int as libc::c_uint {
        let fresh4 = buf;
        buf = buf.offset(1);
        *fresh4 = (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *count += 1;
        *count;
    } else if u <= 0x7ff as libc::c_int as libc::c_uint {
        let fresh5 = buf;
        buf = buf.offset(1);
        *fresh5 = (0xc0 as libc::c_int as libc::c_uint
            | u >> 6 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh6 = buf;
        buf = buf.offset(1);
        *fresh6 = (0x80 as libc::c_int as libc::c_uint
            | u & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar;
        *count += 2 as libc::c_int;
    } else if u <= 0xffff as libc::c_int as libc::c_uint {
        let fresh7 = buf;
        buf = buf.offset(1);
        *fresh7 = (0xe0 as libc::c_int as libc::c_uint
            | u >> 12 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh8 = buf;
        buf = buf.offset(1);
        *fresh8 = (0x80 as libc::c_int as libc::c_uint
            | u >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh9 = buf;
        buf = buf.offset(1);
        *fresh9 = (0x80 as libc::c_int as libc::c_uint
            | u & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar;
        *count += 3 as libc::c_int;
    } else {
        let fresh10 = buf;
        buf = buf.offset(1);
        *fresh10 = (0xf0 as libc::c_int as libc::c_uint
            | u >> 18 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh11 = buf;
        buf = buf.offset(1);
        *fresh11 = (0x80 as libc::c_int as libc::c_uint
            | u >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh12 = buf;
        buf = buf.offset(1);
        *fresh12 = (0x80 as libc::c_int as libc::c_uint
            | u >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh13 = buf;
        buf = buf.offset(1);
        *fresh13 = (0x80 as libc::c_int as libc::c_uint
            | u & 0x3f as libc::c_int as libc::c_uint) as libc::c_uchar;
        *count += 4 as libc::c_int;
    }
    *b = buf;
}
#[inline]
unsafe extern "C" fn mln_json_char2int(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
    }
    return c as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_get_char(
    mut s: *mut mln_u8ptr_t,
    mut len: *mut libc::c_int,
    mut hex: *mut libc::c_uint,
) -> libc::c_int {
    if *len <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *(*s).offset(0 as libc::c_int as isize) as libc::c_int
        == '\\' as i32 as mln_u8_t as libc::c_int && *len > 1 as libc::c_int
    {
        match *(*s).offset(1 as libc::c_int as isize) as libc::c_int {
            34 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '"' as i32;
            }
            92 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\\' as i32;
            }
            47 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '/' as i32;
            }
            98 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\u{8}' as i32;
            }
            102 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\u{c}' as i32;
            }
            110 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\n' as i32;
            }
            114 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\r' as i32;
            }
            116 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                return '\t' as i32;
            }
            117 => {
                *s = (*s).offset(2 as libc::c_int as isize);
                *len -= 2 as libc::c_int;
                if *len < 4 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let fresh14 = *s;
                *s = (*s).offset(1);
                h = mln_json_char2int(*fresh14 as libc::c_char) as libc::c_uint;
                h <<= 4 as libc::c_int;
                let fresh15 = *s;
                *s = (*s).offset(1);
                h |= mln_json_char2int(*fresh15 as libc::c_char) as libc::c_uint;
                h <<= 4 as libc::c_int;
                let fresh16 = *s;
                *s = (*s).offset(1);
                h |= mln_json_char2int(*fresh16 as libc::c_char) as libc::c_uint;
                h <<= 4 as libc::c_int;
                let fresh17 = *s;
                *s = (*s).offset(1);
                h |= mln_json_char2int(*fresh17 as libc::c_char) as libc::c_uint;
                *len -= 4 as libc::c_int;
                *hex = h;
                return 0 as libc::c_int;
            }
            _ => return -(1 as libc::c_int),
        }
    }
    match *(*s).offset(0 as libc::c_int as isize) as libc::c_int {
        92 => return -(1 as libc::c_int),
        _ => {}
    }
    *len -= 1 as libc::c_int;
    let fresh18 = *s;
    *s = (*s).offset(1);
    return *fresh18 as libc::c_int;
}
unsafe extern "C" fn mln_json_parse_digit(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
) -> libc::c_int {
    let mut sub_flag: libc::c_int = 0 as libc::c_int;
    let mut left: libc::c_int = 0;
    let mut val: libc::c_double = 0 as libc::c_int as libc::c_double;
    if *jstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        sub_flag = 1 as libc::c_int;
        jstr = jstr.offset(1);
        jstr;
        len -= 1;
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    left = mln_json_digit_process(&mut val, jstr, len);
    if left < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ({
        let mut json: *mut mln_json_t = j;
        (*json).type_0 = M_JSON_NUM;
        (*json).data.m_j_number = if sub_flag != 0 { -val } else { val };
        //compile_error!("Volatile value is not supposed to be read")
    });
    return left;
}
#[inline]
unsafe extern "C" fn mln_json_digit_process(
    mut val: *mut libc::c_double,
    mut s: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut f: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dir: libc::c_int = 1 as libc::c_int;
    if !(*s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32) {
        return -(1 as libc::c_int);
    }
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        s = s.offset(1);
        s;
        len -= 1;
        if len <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            return -(1 as libc::c_int);
        }
    } else {
        while len > 0 as libc::c_int {
            if !(*s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32) {
                break;
            }
            *val *= 10 as libc::c_int as libc::c_double;
            *val += (*s as libc::c_int - '0' as i32) as libc::c_double;
            len -= 1;
            len;
            s = s.offset(1);
            s;
        }
        if len <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if *s as libc::c_int == '.' as i32 {
        s = s.offset(1);
        s;
        len -= 1;
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i = 1 as libc::c_int;
        while len > 0 as libc::c_int {
            if !(*s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32) {
                break;
            }
            f = (*s as libc::c_int - '0' as i32) as libc::c_double;
            j = 0 as libc::c_int;
            while j < i {
                f /= 10 as libc::c_int as libc::c_double;
                j += 1;
                j;
            }
            *val += f;
            len -= 1;
            len;
            s = s.offset(1);
            s;
            i += 1;
            i;
        }
        if len <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if *s as libc::c_int == 'e' as i32 || *s as libc::c_int == 'E' as i32 {
        s = s.offset(1);
        s;
        len -= 1;
        if len <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if *s as libc::c_int == '+' as i32 {
            s = s.offset(1);
            s;
            len -= 1;
            if len <= 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        } else if *s as libc::c_int == '-' as i32 {
            dir = 0 as libc::c_int;
            s = s.offset(1);
            s;
            len -= 1;
            if len <= 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        i = 0 as libc::c_int;
        while len > 0 as libc::c_int {
            if !(*s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32) {
                break;
            }
            i *= 10 as libc::c_int;
            i += *s as libc::c_int - '0' as i32;
            len -= 1;
            len;
            s = s.offset(1);
            s;
        }
        if i == 0 as libc::c_int {
            return len;
        }
        f = 1 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < i {
            if dir != 0 {
                f *= 10 as libc::c_int as libc::c_double;
            } else {
                f /= 10 as libc::c_int as libc::c_double;
            }
            j += 1;
            j;
        }
        *val *= f;
    }
    return len;
}
#[inline]
unsafe extern "C" fn mln_json_parse_true(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
) -> libc::c_int {
    if len < 4 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if strncasecmp(
        jstr,
        b"true\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    ({
        let mut json: *mut mln_json_t = j;
        (*json).type_0 = M_JSON_TRUE;
        (*json).data.m_j_true = 1 as libc::c_int as mln_u8_t;
        //compile_error!("Volatile value is not supposed to be read")
    });
    return len - 4 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_parse_false(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
) -> libc::c_int {
    if len < 5 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if strncasecmp(
        jstr,
        b"false\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    ({
        let mut json: *mut mln_json_t = j;
        (*json).type_0 = M_JSON_FALSE;
        (*json).data.m_j_false = 1 as libc::c_int as mln_u8_t;
        //compile_error!("Volatile value is not supposed to be read")
    });
    return len - 5 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_parse_null(
    mut j: *mut mln_json_t,
    mut jstr: *mut libc::c_char,
    mut len: libc::c_int,
    mut index: mln_uauto_t,
) -> libc::c_int {
    if len < 4 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if strncasecmp(
        jstr,
        b"null\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    ({
        let mut json: *mut mln_json_t = j;
        (*json).type_0 = M_JSON_NULL;
        (*json).data.m_j_null = 0 as mln_u8ptr_t;
        (*json).data.m_j_null
    });
    return len - 4 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_jumpoff_blank(
    mut jstr: *mut *mut libc::c_char,
    mut len: *mut libc::c_int,
) {
    while *len > 0 as libc::c_int
        && (**jstr.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            || **jstr.offset(0 as libc::c_int as isize) as libc::c_int == '\t' as i32
            || **jstr.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            || **jstr.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32)
    {
        *jstr = (*jstr).offset(1);
        *jstr;
        *len -= 1;
        *len;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_encode(mut j: *mut mln_json_t) -> *mut mln_string_t {
    let mut buf: mln_s8ptr_t = 0 as *mut libc::c_char;
    let mut size: mln_size_t = 512 as libc::c_int as mln_size_t;
    let mut pos: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    buf = malloc(size) as mln_s8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    if mln_json_write_content(j, &mut buf, &mut size, &mut pos) < 0 as libc::c_int {
        free(buf as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    s = mln_string_const_ndup(buf, pos as mln_s32_t);
    free(buf as *mut libc::c_void);
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    return s;
}
#[inline]
unsafe extern "C" fn mln_json_write_byte(
    mut buf: *mut mln_s8ptr_t,
    mut size: *mut mln_size_t,
    mut off: *mut mln_size_t,
    mut s: mln_u8ptr_t,
    mut n: mln_size_t,
) -> libc::c_int {
    if *size < (*off).wrapping_add(n) {
        let mut tmp: mln_s8ptr_t = realloc(
            *buf as *mut libc::c_void,
            *size << 1 as libc::c_int,
        ) as mln_s8ptr_t;
        if tmp.is_null() {
            return -(1 as libc::c_int);
        }
        *buf = tmp;
        *size <<= 1 as libc::c_int;
    }
    memcpy(
        (*buf).offset(*off as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        n,
    );
    *off = (*off as libc::c_ulong).wrapping_add(n) as mln_size_t as mln_size_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_write_content(
    mut j: *mut mln_json_t,
    mut buf: *mut mln_s8ptr_t,
    mut size: *mut mln_size_t,
    mut off: *mut mln_size_t,
) -> libc::c_int {
    if j.is_null() {
        return 0 as libc::c_int;
    }
    let mut save: mln_size_t = 0;
    match (*j).type_0 as libc::c_uint {
        1 => {
            let mut tmp: mln_json_tmp_s = mln_json_tmp_s {
                ptr1: 0 as *mut libc::c_void,
                ptr2: 0 as *mut libc::c_void,
                ptr3: 0 as *mut libc::c_void,
            };
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"{\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            tmp.ptr1 = buf as *mut libc::c_void;
            tmp.ptr2 = size as *mut libc::c_void;
            tmp.ptr3 = off as *mut libc::c_void;
            save = *off;
            mln_rbtree_iterate(
                (*j).data.m_j_obj,
                Some(
                    mln_json_write_content_obj_iterate_handler
                        as unsafe extern "C" fn(
                            *mut mln_rbtree_node_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                &mut tmp as *mut mln_json_tmp_s as *mut libc::c_void,
            );
            if save < *off {
                ({
                    let mut o: mln_size_t = *off;
                    let mut n: mln_size_t = 1 as libc::c_int as mln_size_t;
                    *off = if o < n {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        o.wrapping_sub(n)
                    };
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"}\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        2 => {
            let mut el: *mut mln_json_t = (*(*j).data.m_j_array).elts as *mut mln_json_t;
            let mut elend: *mut mln_json_t = el
                .offset((*(*j).data.m_j_array).nelts as isize);
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"[\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            save = *off;
            while el < elend {
                if mln_json_write_content(el, buf, size, off) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if mln_json_write_byte(
                    buf,
                    size,
                    off,
                    b",\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                    1 as libc::c_int as mln_size_t,
                ) < 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                el = el.offset(1);
                el;
            }
            if save < *off {
                ({
                    let mut o: mln_size_t = *off;
                    let mut n: mln_size_t = 1 as libc::c_int as mln_size_t;
                    *off = if o < n {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        o.wrapping_sub(n)
                    };
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"]\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        3 => {
            let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
            let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
            let mut end: mln_u8ptr_t = 0 as *mut libc::c_uchar;
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"\"\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            s = (*j).data.m_j_string;
            if !s.is_null() {
                p = (*(*j).data.m_j_string).data;
                end = p.offset((*(*j).data.m_j_string).len as isize);
                while p < end {
                    if *p as libc::c_int == '"' as i32
                        || *p as libc::c_int == '\\' as i32
                    {
                        if mln_json_write_byte(
                            buf,
                            size,
                            off,
                            b"\\\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                            1 as libc::c_int as mln_size_t,
                        ) < 0 as libc::c_int
                        {
                            return -(1 as libc::c_int);
                        }
                    }
                    let fresh19 = p;
                    p = p.offset(1);
                    if mln_json_write_byte(
                        buf,
                        size,
                        off,
                        fresh19,
                        1 as libc::c_int as mln_size_t,
                    ) < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                }
            }
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"\"\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                1 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        4 => {
            let mut n: libc::c_int = 0;
            let mut tmp_0: [libc::c_char; 512] = [0; 512];
            let mut i: mln_s64_t = (*j).data.m_j_number as mln_s64_t;
            if i as libc::c_double == (*j).data.m_j_number {
                n = snprintf(
                    tmp_0.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    i,
                );
            } else {
                n = snprintf(
                    tmp_0.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    b"%f\0" as *const u8 as *const libc::c_char,
                    (*j).data.m_j_number,
                );
            }
            if mln_json_write_byte(
                buf,
                size,
                off,
                tmp_0.as_mut_ptr() as mln_u8ptr_t,
                n as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        5 => {
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"true\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                4 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        6 => {
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"false\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                5 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        7 => {
            if mln_json_write_byte(
                buf,
                size,
                off,
                b"null\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                4 as libc::c_int as mln_size_t,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_json_write_content_obj_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut kv: *mut mln_json_kv_t = (*node).data as *mut mln_json_kv_t;
    let mut tmp: *mut mln_json_tmp_s = data as *mut mln_json_tmp_s;
    let mut buf: *mut mln_s8ptr_t = (*tmp).ptr1 as *mut mln_s8ptr_t;
    let mut size: *mut mln_size_t = (*tmp).ptr2 as *mut mln_size_t;
    let mut off: *mut mln_size_t = (*tmp).ptr3 as *mut mln_size_t;
    if kv.is_null() {
        return 0 as libc::c_int;
    }
    if mln_json_write_content(&mut (*kv).key, buf, size, off) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mln_json_write_byte(
        buf,
        size,
        off,
        b":\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
        1 as libc::c_int as mln_size_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mln_json_write_content(&mut (*kv).val, buf, size, off) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mln_json_write_byte(
        buf,
        size,
        off,
        b",\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
        1 as libc::c_int as mln_size_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_parse(
    mut j: *mut mln_json_t,
    mut exp: *mut mln_string_t,
    mut iterator: mln_json_iterator_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut idx: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut p: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut arr: *mut mln_string_t = mln_string_slice(
        exp,
        b".\0" as *const u8 as *const libc::c_char,
    );
    if arr.is_null() {
        return -(1 as libc::c_int);
    }
    p = arr;
    while (*p).len != 0 as libc::c_int as libc::c_ulong {
        if (*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint {
            j = mln_json_obj_search(j, p);
            if j.is_null() {
                current_block = 3891982576112998699;
            } else {
                current_block = 14155750587950065367;
            }
        } else if (*j).type_0 as libc::c_uint
            == M_JSON_ARRAY as libc::c_int as libc::c_uint
        {
            if mln_json_parse_is_index(p, &mut idx) == 0 {
                current_block = 3891982576112998699;
            } else {
                j = mln_json_array_search(j, idx);
                if j.is_null() {
                    current_block = 3891982576112998699;
                } else {
                    current_block = 14155750587950065367;
                }
            }
        } else {
            current_block = 3891982576112998699;
        }
        match current_block {
            14155750587950065367 => {
                p = p.offset(1);
                p;
            }
            _ => {
                mln_string_slice_free(arr);
                return -(1 as libc::c_int);
            }
        }
    }
    mln_string_slice_free(arr);
    if iterator.is_some() {
        return iterator.expect("non-null function pointer")(j, data);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_json_parse_is_index(
    mut s: *mut mln_string_t,
    mut idx: *mut mln_size_t,
) -> libc::c_int {
    let mut p: mln_u8ptr_t = (*s).data;
    let mut pend: mln_u8ptr_t = ((*s).data)
        .offset((*s).len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut sum: mln_size_t = 0 as libc::c_int as mln_size_t;
    while pend >= p {
        if (*pend as libc::c_int) < '0' as i32 as mln_u8_t as libc::c_int
            || *pend as libc::c_int > '9' as i32 as mln_u8_t as libc::c_int
        {
            return 0 as libc::c_int;
        }
        sum = sum
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (*pend as libc::c_int - '0' as i32 as mln_u8_t as libc::c_int)
                    as libc::c_ulong,
            );
        pend = pend.offset(-1);
        pend;
    }
    *idx = sum;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_generate(
    mut j: *mut mln_json_t,
    mut fmt: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    if *fmt as libc::c_int == '{' as i32 {
        rc = mln_json_obj_generate(j, &mut fmt, &mut arg.as_va_list());
    } else if *fmt as libc::c_int == '[' as i32 {
        rc = mln_json_array_generate(j, &mut fmt, &mut arg.as_va_list());
    } else {
        rc = -(1 as libc::c_int);
    }
    return rc;
}
#[inline]
unsafe extern "C" fn mln_json_obj_generate(
    mut j: *mut mln_json_t,
    mut fmt: *mut *mut libc::c_char,
    mut arg: *mut ::core::ffi::VaList,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut k: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    let mut v: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    let mut json: *mut mln_json_t = 0 as *mut mln_json_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = *fmt;
    let mut str: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut s32: mln_s32_t = 0;
    let mut u32: mln_u32_t = 0;
    let mut s64: mln_s64_t = 0;
    let mut u64: mln_u64_t = 0;
    let mut d: libc::c_double = 0.;
    let mut ca: *mut mln_json_call_attr = 0 as *mut mln_json_call_attr;
    if (*j).type_0 as libc::c_uint == M_JSON_NONE as libc::c_int as libc::c_uint
        && __mln_json_obj_init(j) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if !((*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int);
    }
    f = f.offset(1);
    f;
    loop {
        k.type_0 = M_JSON_NONE;
        v.type_0 = M_JSON_NONE;
        let fresh20 = f;
        f = f.offset(1);
        match *fresh20 as libc::c_int {
            115 => {
                s = (*arg).arg::<*mut libc::c_char>();
                ({
                    tmp.data = s as mln_u8ptr_t;
                    tmp.len = strlen(s);
                    tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                    tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    &mut tmp;
                    &mut tmp
                });
                str = mln_string_dup(&mut tmp);
                if str.is_null() {
                    current_block = 12549308426115558357;
                    break;
                }
                ({
                    let mut json_0: *mut mln_json_t = &mut k;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0).data.m_j_string = str;
                    (*json_0).data.m_j_string
                });
            }
            83 => {
                str = (*arg).arg::<*mut mln_string_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut k;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0)
                        .data
                        .m_j_string = ({
                        let mut __s: *mut mln_string_t = str;
                        (*__s).set_ref_0((*__s).ref_0() + 1);
                        (*__s).ref_0();
                        __s
                    });
                    (*json_0).data.m_j_string
                });
            }
            99 => {
                ca = (*arg).arg::<*mut mln_json_call_attr>();
                if ((*ca).callback)
                    .expect("non-null function pointer")(&mut k, (*ca).data)
                    < 0 as libc::c_int
                {
                    current_block = 12549308426115558357;
                    break;
                }
            }
            125 => {
                current_block = 13724565774631848959;
                break;
            }
            _ => {
                current_block = 12549308426115558357;
                break;
            }
        }
        let fresh21 = f;
        f = f.offset(1);
        if *fresh21 as libc::c_int != ':' as i32 {
            current_block = 12549308426115558357;
            break;
        }
        let fresh22 = f;
        f = f.offset(1);
        match *fresh22 as libc::c_int {
            106 => {
                json = (*arg).arg::<*mut mln_json_t>();
                v = *json;
            }
            100 => {
                s32 = (*arg).arg::<mln_s32_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = s32 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            68 => {
                s64 = (*arg).arg::<mln_s64_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = s64 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            117 => {
                u32 = (*arg).arg::<mln_u32_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = u32 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            85 => {
                u64 = (*arg).arg::<mln_u64_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = u64 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            70 => {
                d = (*arg).arg::<libc::c_double>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = d;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            116 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_TRUE;
                    (*json_0).data.m_j_true = 1 as libc::c_int as mln_u8_t;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            102 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_FALSE;
                    (*json_0).data.m_j_false = 1 as libc::c_int as mln_u8_t;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            110 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NULL;
                    (*json_0).data.m_j_null = 0 as mln_u8ptr_t;
                    (*json_0).data.m_j_null
                });
            }
            115 => {
                s = (*arg).arg::<*mut libc::c_char>();
                ({
                    tmp.data = s as mln_u8ptr_t;
                    tmp.len = strlen(s);
                    tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                    tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    &mut tmp;
                    &mut tmp
                });
                str = mln_string_dup(&mut tmp);
                if str.is_null() {
                    current_block = 12549308426115558357;
                    break;
                }
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0).data.m_j_string = str;
                    (*json_0).data.m_j_string
                });
            }
            83 => {
                str = (*arg).arg::<*mut mln_string_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0)
                        .data
                        .m_j_string = ({
                        let mut __s: *mut mln_string_t = str;
                        (*__s).set_ref_0((*__s).ref_0() + 1);
                        (*__s).ref_0();
                        __s
                    });
                    (*json_0).data.m_j_string
                });
            }
            99 => {
                ca = (*arg).arg::<*mut mln_json_call_attr>();
                if ((*ca).callback)
                    .expect("non-null function pointer")(&mut v, (*ca).data)
                    < 0 as libc::c_int
                {
                    current_block = 12549308426115558357;
                    break;
                }
            }
            123 => {
                f = f.offset(-1);
                f;
                if mln_json_obj_generate(&mut v, &mut f, arg) < 0 as libc::c_int {
                    current_block = 12549308426115558357;
                    break;
                }
            }
            91 => {
                f = f.offset(-1);
                f;
                if mln_json_array_generate(&mut v, &mut f, arg) < 0 as libc::c_int {
                    current_block = 12549308426115558357;
                    break;
                }
            }
            _ => {
                current_block = 12549308426115558357;
                break;
            }
        }
        if __mln_json_obj_update(j, &mut k, &mut v) < 0 as libc::c_int {
            current_block = 12549308426115558357;
            break;
        }
        if *f as libc::c_int == '}' as i32 {
            f = f.offset(1);
            f;
            current_block = 13724565774631848959;
            break;
        } else {
            let fresh23 = f;
            f = f.offset(1);
            if *fresh23 as libc::c_int != ',' as i32 {
                current_block = 4699894777566340892;
                break;
            }
        }
    }
    match current_block {
        12549308426115558357 => {
            mln_json_destroy(&mut k);
            mln_json_destroy(&mut v);
            current_block = 4699894777566340892;
        }
        _ => {}
    }
    match current_block {
        4699894777566340892 => {
            rc = -(1 as libc::c_int);
            ({
                let mut json_0: *mut mln_json_t = j;
                mln_json_destroy(json_0);
                (*json_0).type_0 = M_JSON_NONE;
                //compile_error!("Volatile value is not supposed to be read")
            });
        }
        _ => {}
    }
    *fmt = f;
    return rc;
}
#[inline]
unsafe extern "C" fn mln_json_array_generate(
    mut j: *mut mln_json_t,
    mut fmt: *mut *mut libc::c_char,
    mut arg: *mut ::core::ffi::VaList,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut v: mln_json_t = mln_json_t {
        type_0: M_JSON_NONE,
        data: C2RustUnnamed {
            m_j_obj: 0 as *mut mln_rbtree_t,
        },
    };
    let mut json: *mut mln_json_t = 0 as *mut mln_json_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = *fmt;
    let mut str: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut s32: mln_s32_t = 0;
    let mut u32: mln_u32_t = 0;
    let mut s64: mln_s64_t = 0;
    let mut u64: mln_u64_t = 0;
    let mut d: libc::c_double = 0.;
    let mut ca: *mut mln_json_call_attr = 0 as *mut mln_json_call_attr;
    if (*j).type_0 as libc::c_uint == M_JSON_NONE as libc::c_int as libc::c_uint
        && __mln_json_array_init(j) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if !((*j).type_0 as libc::c_uint == M_JSON_ARRAY as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int);
    }
    f = f.offset(1);
    f;
    loop {
        v.type_0 = M_JSON_NONE;
        let fresh24 = f;
        f = f.offset(1);
        match *fresh24 as libc::c_int {
            106 => {
                json = (*arg).arg::<*mut mln_json_t>();
                v = *json;
            }
            100 => {
                s32 = (*arg).arg::<mln_s32_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = s32 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            68 => {
                s64 = (*arg).arg::<mln_s64_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = s64 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            117 => {
                u32 = (*arg).arg::<mln_u32_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = u32 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            85 => {
                u64 = (*arg).arg::<mln_u64_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = u64 as libc::c_double;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            70 => {
                d = (*arg).arg::<libc::c_double>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NUM;
                    (*json_0).data.m_j_number = d;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            116 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_TRUE;
                    (*json_0).data.m_j_true = 1 as libc::c_int as mln_u8_t;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            102 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_FALSE;
                    (*json_0).data.m_j_false = 1 as libc::c_int as mln_u8_t;
                    //compile_error!("Volatile value is not supposed to be read")
                });
            }
            110 => {
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_NULL;
                    (*json_0).data.m_j_null = 0 as mln_u8ptr_t;
                    (*json_0).data.m_j_null
                });
            }
            115 => {
                s = (*arg).arg::<*mut libc::c_char>();
                ({
                    tmp.data = s as mln_u8ptr_t;
                    tmp.len = strlen(s);
                    tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                    tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    &mut tmp;
                    &mut tmp
                });
                str = mln_string_dup(&mut tmp);
                if str.is_null() {
                    current_block = 8801246572580356462;
                    break;
                }
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0).data.m_j_string = str;
                    (*json_0).data.m_j_string
                });
            }
            83 => {
                str = (*arg).arg::<*mut mln_string_t>();
                ({
                    let mut json_0: *mut mln_json_t = &mut v;
                    (*json_0).type_0 = M_JSON_STRING;
                    (*json_0)
                        .data
                        .m_j_string = ({
                        let mut __s: *mut mln_string_t = str;
                        (*__s).set_ref_0((*__s).ref_0() + 1);
                        (*__s).ref_0();
                        __s
                    });
                    (*json_0).data.m_j_string
                });
            }
            99 => {
                ca = (*arg).arg::<*mut mln_json_call_attr>();
                if ((*ca).callback)
                    .expect("non-null function pointer")(&mut v, (*ca).data)
                    < 0 as libc::c_int
                {
                    current_block = 8801246572580356462;
                    break;
                }
            }
            93 => {
                current_block = 9320956757400089605;
                break;
            }
            123 => {
                f = f.offset(-1);
                f;
                if mln_json_obj_generate(&mut v, &mut f, arg) < 0 as libc::c_int {
                    current_block = 8801246572580356462;
                    break;
                }
            }
            91 => {
                f = f.offset(-1);
                f;
                if mln_json_array_generate(&mut v, &mut f, arg) < 0 as libc::c_int {
                    current_block = 8801246572580356462;
                    break;
                }
            }
            _ => {
                current_block = 8801246572580356462;
                break;
            }
        }
        if __mln_json_array_append(j, &mut v) < 0 as libc::c_int {
            current_block = 8801246572580356462;
            break;
        }
        if *f as libc::c_int == ']' as i32 {
            f = f.offset(1);
            f;
            current_block = 9320956757400089605;
            break;
        } else {
            let fresh25 = f;
            f = f.offset(1);
            if *fresh25 as libc::c_int != ',' as i32 {
                current_block = 1028577634761322290;
                break;
            }
        }
    }
    match current_block {
        8801246572580356462 => {
            mln_json_destroy(&mut v);
            current_block = 1028577634761322290;
        }
        _ => {}
    }
    match current_block {
        1028577634761322290 => {
            rc = -(1 as libc::c_int);
            ({
                let mut json_0: *mut mln_json_t = j;
                mln_json_destroy(json_0);
                (*json_0).type_0 = M_JSON_NONE;
                //compile_error!("Volatile value is not supposed to be read")
            });
        }
        _ => {}
    }
    *fmt = f;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_json_object_iterate(
    mut j: *mut mln_json_t,
    mut it: mln_json_object_iterator_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if !((*j).type_0 as libc::c_uint == M_JSON_OBJECT as libc::c_int as libc::c_uint) {
        return -(1 as libc::c_int);
    }
    let mut tmp: mln_json_tmp_s = mln_json_tmp_s {
        ptr1: 0 as *mut libc::c_void,
        ptr2: 0 as *mut libc::c_void,
        ptr3: 0 as *mut libc::c_void,
    };
    tmp
        .ptr1 = ::core::mem::transmute::<
        mln_json_object_iterator_t,
        *mut libc::c_void,
    >(it);
    tmp.ptr2 = data;
    return mln_rbtree_iterate(
        (*j).data.m_j_obj,
        Some(
            mln_json_object_iterator
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut tmp as *mut mln_json_tmp_s as *mut libc::c_void,
    );
}
#[inline]
unsafe extern "C" fn mln_json_object_iterator(
    mut node: *mut mln_rbtree_node_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: *mut mln_json_tmp_s = data as *mut mln_json_tmp_s;
    let mut kv: *mut mln_json_kv_t = (*node).data as *mut mln_json_kv_t;
    let mut it: mln_json_object_iterator_t = ::core::mem::transmute::<
        *mut libc::c_void,
        mln_json_object_iterator_t,
    >((*tmp).ptr1);
    return it
        .expect(
            "non-null function pointer",
        )(&mut (*kv).key, &mut (*kv).val, (*tmp).ptr2);
}
