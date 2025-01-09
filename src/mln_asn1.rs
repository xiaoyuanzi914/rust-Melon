use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_time2utc(tm: time_t, uc: *mut utctime);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
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
pub type rbtree_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type rbtree_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
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
pub struct mln_fileset_s {
    pub pool: *mut mln_alloc_t,
    pub reg_file_tree: *mut mln_rbtree_t,
    pub reg_free_head: *mut mln_file_t,
    pub reg_free_tail: *mut mln_file_t,
    pub max_file: mln_size_t,
}
pub type mln_file_t = mln_file_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_file_s {
    pub prev: *mut mln_file_s,
    pub next: *mut mln_file_s,
    pub fd: libc::c_int,
    #[bitfield(name = "is_tmp", ty = "mln_u32_t", bits = "0..=0")]
    pub is_tmp: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub file_path: *mut mln_string_t,
    pub fset: *mut mln_fileset_t,
    pub node: *mut mln_rbtree_node_t,
    pub refer_cnt: size_t,
    pub size: size_t,
    pub mtime: time_t,
    pub ctime: time_t,
    pub atime: time_t,
}
pub type mln_fileset_t = mln_fileset_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_buf_s {
    pub left_pos: mln_u8ptr_t,
    pub pos: mln_u8ptr_t,
    pub last: mln_u8ptr_t,
    pub start: mln_u8ptr_t,
    pub end: mln_u8ptr_t,
    pub shadow: *mut mln_buf_s,
    pub file_left_pos: mln_off_t,
    pub file_pos: mln_off_t,
    pub file_last: mln_off_t,
    pub file: *mut mln_file_t,
    #[bitfield(name = "temporary", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "mmap", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "in_memory", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "in_file", ty = "mln_u32_t", bits = "3..=3")]
    #[bitfield(name = "flush", ty = "mln_u32_t", bits = "4..=4")]
    #[bitfield(name = "sync", ty = "mln_u32_t", bits = "5..=5")]
    #[bitfield(name = "last_buf", ty = "mln_u32_t", bits = "6..=6")]
    #[bitfield(name = "last_in_chain", ty = "mln_u32_t", bits = "7..=7")]
    pub temporary_mmap_in_memory_in_file_flush_sync_last_buf_last_in_chain: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mln_buf_t = mln_buf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_chain_s {
    pub buf: *mut mln_buf_t,
    pub next: *mut mln_chain_s,
}
pub type mln_chain_t = mln_chain_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_asn1_deresult_s {
    pub pool: *mut mln_alloc_t,
    pub code_buf: mln_u8ptr_t,
    pub code_len: mln_u64_t,
    pub contents: *mut *mut mln_asn1_deresult_t,
    pub max: mln_u32_t,
    pub pos: mln_u32_t,
    #[bitfield(name = "_class", ty = "mln_u32_t", bits = "0..=1")]
    #[bitfield(name = "is_struct", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "ident", ty = "mln_u32_t", bits = "3..=30")]
    #[bitfield(name = "free", ty = "mln_u32_t", bits = "31..=31")]
    pub _class_is_struct_ident_free: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_asn1_deresult_t = mln_asn1_deresult_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_asn1_enresult_t {
    pub pool: *mut mln_alloc_t,
    pub size: mln_u64_t,
    pub contents: *mut mln_string_t,
    pub max: mln_u32_t,
    pub pos: mln_u32_t,
    #[bitfield(name = "_class", ty = "mln_u32_t", bits = "0..=1")]
    #[bitfield(name = "is_struct", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "ident", ty = "mln_u32_t", bits = "3..=31")]
    pub _class_is_struct_ident: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utctime {
    pub year: libc::c_long,
    pub month: libc::c_long,
    pub day: libc::c_long,
    pub hour: libc::c_long,
    pub minute: libc::c_long,
    pub second: libc::c_long,
    pub week: libc::c_long,
}
#[inline]
unsafe extern "C" fn mln_alloc_m(
    mut pool: *mut mln_alloc_t,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut blk: *mut mln_alloc_blk_t = 0 as *mut mln_alloc_blk_t;
    let mut am: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
    let mut ch: *mut mln_alloc_chunk_t = 0 as *mut mln_alloc_chunk_t;
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut n: mln_size_t = 0;
    if !((*pool).mem).is_null() {
        return mln_alloc_shm_m(pool, size);
    }
    am = mln_alloc_get_mgr_by_size(((*pool).mgr_tbl).as_mut_ptr(), size);
    if am.is_null() {
        n = size
            .wrapping_add(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong) >> 2 as libc::c_int;
        size = n << 2 as libc::c_int;
        if !((*pool).parent).is_null() {
            if !((*(*pool).parent).mem).is_null() {
                if ((*(*pool).parent).lock)
                    .expect("non-null function pointer")((*(*pool).parent).locker)
                    != 0 as libc::c_int
                {
                    return 0 as *mut libc::c_void;
                }
            }
            ptr = mln_alloc_c((*pool).parent, size) as mln_u8ptr_t;
            if !((*(*pool).parent).mem).is_null() {
                ((*(*pool).parent).unlock)
                    .expect("non-null function pointer")((*(*pool).parent).locker);
            }
        } else {
            ptr = calloc(1 as libc::c_int as libc::c_ulong, size) as mln_u8ptr_t;
        }
        if ptr.is_null() {
            return 0 as *mut libc::c_void;
        }
        ch = ptr as *mut mln_alloc_chunk_t;
        (*ch).refer = 1 as libc::c_int as mln_size_t;
        mln_chunk_chain_add(
            &mut (*pool).large_used_head,
            &mut (*pool).large_used_tail,
            ch,
        );
        blk = ptr
            .offset(
                ::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong as isize,
            ) as *mut mln_alloc_blk_t;
        (*blk)
            .data = ptr
            .offset(
                ::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong as isize,
            )
            .offset(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize)
            as *mut libc::c_void;
        (*blk).chunk = ch;
        (*blk).pool = pool;
        (*blk)
            .blk_size = size
            .wrapping_sub(
                (::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong,
                    ),
            );
        (*blk).set_is_large(1 as libc::c_int as mln_size_t);
        (*blk).set_in_used(1 as libc::c_int as mln_size_t);
        (*ch).blks[0 as libc::c_int as usize] = blk;
        return (*blk).data;
    }
    if ((*am).free_head).is_null() {
        n = (::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong)
            .wrapping_add((*am).blk_size)
            .wrapping_add(3 as libc::c_int as libc::c_ulong) >> 2 as libc::c_int;
        size = n << 2 as libc::c_int;
        n = (::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong)
            .wrapping_add((4 as libc::c_int as libc::c_ulong).wrapping_mul(size))
            .wrapping_add(3 as libc::c_int as libc::c_ulong) >> 2 as libc::c_int;
        if !((*pool).parent).is_null() {
            if !((*(*pool).parent).mem).is_null() {
                if ((*(*pool).parent).lock)
                    .expect("non-null function pointer")((*(*pool).parent).locker)
                    != 0 as libc::c_int
                {
                    return 0 as *mut libc::c_void;
                }
            }
            ptr = mln_alloc_m((*pool).parent, n << 2 as libc::c_int) as mln_u8ptr_t;
            if !((*(*pool).parent).mem).is_null() {
                ((*(*pool).parent).unlock)
                    .expect("non-null function pointer")((*(*pool).parent).locker);
            }
        } else {
            ptr = malloc(n << 2 as libc::c_int) as mln_u8ptr_t;
        }
        if ptr.is_null() {
            loop {
                if !(am
                    < ((*pool).mgr_tbl)
                        .as_mut_ptr()
                        .offset((18 as libc::c_int * 2 as libc::c_int) as isize)
                        .offset(-((2 as libc::c_int - 1 as libc::c_int) as isize)))
                {
                    current_block = 7427571413727699167;
                    break;
                }
                if !((*am).free_head).is_null() {
                    current_block = 10752631921686271448;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                10752631921686271448 => {}
                _ => return 0 as *mut libc::c_void,
            }
        } else {
            ch = ptr as *mut mln_alloc_chunk_t;
            mln_chunk_chain_add(&mut (*am).chunk_head, &mut (*am).chunk_tail, ch);
            (*ch).count = 0 as libc::c_int as mln_size_t;
            (*ch).refer = (*ch).count;
            (*ch).mgr = am;
            ptr = ptr
                .offset(
                    ::core::mem::size_of::<mln_alloc_chunk_t>() as libc::c_ulong as isize,
                );
            n = 0 as libc::c_int as mln_size_t;
            while n < 4 as libc::c_int as libc::c_ulong {
                blk = ptr as *mut mln_alloc_blk_t;
                mln_blk_chain_add(&mut (*am).free_head, &mut (*am).free_tail, blk);
                (*blk).pool = pool;
                (*blk)
                    .data = ptr
                    .offset(
                        ::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong
                            as isize,
                    ) as *mut libc::c_void;
                (*blk).chunk = ch;
                (*blk).blk_size = (*am).blk_size;
                (*blk).set_in_used(0 as libc::c_int as mln_size_t);
                (*blk).set_is_large((*blk).in_used());
                (*ch).blks[n as usize] = blk;
                ptr = ptr.offset(size as isize);
                n = n.wrapping_add(1);
                n;
            }
            (*ch).blks[n as usize] = 0 as *mut mln_alloc_blk_t;
        }
    }
    blk = (*am).free_tail;
    mln_blk_chain_del(&mut (*am).free_head, &mut (*am).free_tail, blk);
    mln_blk_chain_add(&mut (*am).used_head, &mut (*am).used_tail, blk);
    (*blk).set_in_used(1 as libc::c_int as mln_size_t);
    (*(*blk).chunk).refer = ((*(*blk).chunk).refer).wrapping_add(1);
    (*(*blk).chunk).refer;
    return (*blk).data;
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
unsafe extern "C" fn mln_chunk_chain_add(
    mut head: *mut *mut mln_alloc_chunk_t,
    mut tail: *mut *mut mln_alloc_chunk_t,
    mut node: *mut mln_alloc_chunk_t,
) {
    (*node).next = 0 as *mut mln_alloc_chunk_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_alloc_c(
    mut pool: *mut mln_alloc_t,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    let mut ptr: mln_u8ptr_t = mln_alloc_m(pool, size) as mln_u8ptr_t;
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    memset(ptr as *mut libc::c_void, 0 as libc::c_int, size);
    return ptr as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn mln_alloc_get_mgr_by_size(
    mut tbl: *mut mln_alloc_mgr_t,
    mut size: mln_size_t,
) -> *mut mln_alloc_mgr_t {
    if size
        > (*tbl
            .offset(
                (18 as libc::c_int * 2 as libc::c_int
                    - (2 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int) as isize,
            ))
            .blk_size
    {
        return 0 as *mut mln_alloc_mgr_t;
    }
    if size <= (*tbl.offset(0 as libc::c_int as isize)).blk_size {
        return &mut *tbl.offset(0 as libc::c_int as isize) as *mut mln_alloc_mgr_t;
    }
    let mut am: *mut mln_alloc_mgr_t = tbl;
    let mut off: mln_size_t = 0 as libc::c_int as mln_size_t;
    asm!(
        "bsr {0}, {1}", lateout(reg) off, in(reg) &size, options(preserves_flags)
    );
    off = off
        .wrapping_sub(4 as libc::c_int as mln_size_t)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if (*am.offset(off as isize)).blk_size >= size {
        return &mut *am.offset(off as isize) as *mut mln_alloc_mgr_t;
    }
    if (*am.offset(off.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
        .blk_size >= size
    {
        return &mut *am
            .offset(off.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut mln_alloc_mgr_t;
    }
    return &mut *am.offset(off.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
        as *mut mln_alloc_mgr_t;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_m(
    mut pool: *mut mln_alloc_t,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    let mut as_0: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    let mut Boff: mln_off_t = -(1 as libc::c_int) as mln_off_t;
    let mut boff: mln_off_t = -(1 as libc::c_int) as mln_off_t;
    if size
        > ((1 as libc::c_int * 1024 as libc::c_int + 512 as libc::c_int)
            * 1024 as libc::c_int) as libc::c_ulong
    {
        return mln_alloc_shm_large_m(pool, size);
    }
    let mut current_block_8: u64;
    if ((*pool).shm_head).is_null() {
        current_block_8 = 18278753667732557088;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 18278753667732557088;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        18278753667732557088 => {
            as_0 = mln_alloc_shm_new_block(pool, &mut Boff, &mut boff, size);
            if as_0.is_null() {
                return 0 as *mut libc::c_void;
            }
        }
        _ => {}
    }
    return mln_alloc_shm_set_bitmap(as_0, Boff, boff, size);
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_set_bitmap(
    mut as_0: *mut mln_alloc_shm_t,
    mut Boff: mln_off_t,
    mut boff: mln_off_t,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = size
        .wrapping_add(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong)
        .wrapping_add(64 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(64 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pend: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut addr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut blk: *mut mln_alloc_blk_t = 0 as *mut mln_alloc_blk_t;
    addr = ((*as_0).addr)
        .offset(
            ((Boff * 8 as libc::c_int as libc::c_long
                + (7 as libc::c_int as libc::c_long - boff))
                * 64 as libc::c_int as libc::c_long) as isize,
        ) as mln_u8ptr_t;
    blk = addr as *mut mln_alloc_blk_t;
    memset(
        blk as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong,
    );
    (*blk).pool = (*as_0).pool;
    (*blk)
        .data = addr
        .offset(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*blk).chunk = as_0 as *mut mln_alloc_chunk_t;
    (*blk).blk_size = size;
    (*blk)
        .set_padding(
            ((Boff & 0xffff as libc::c_int as libc::c_long) << 8 as libc::c_int
                | boff & 0xff as libc::c_int as libc::c_long) as mln_size_t,
        );
    (*blk).set_is_large(0 as libc::c_int as mln_size_t);
    (*blk).set_in_used(1 as libc::c_int as mln_size_t);
    p = ((*as_0).bitmap).as_mut_ptr().offset(Boff as isize);
    pend = p.offset(4096 as libc::c_int as isize);
    i = boff as libc::c_int;
    while p < pend {
        *p = (*p as libc::c_int | (1 as libc::c_int as mln_u8_t as libc::c_int) << i)
            as libc::c_uchar;
        (*as_0).nfree = ((*as_0).nfree).wrapping_sub(1);
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
    return (*blk).data;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_new_block(
    mut pool: *mut mln_alloc_t,
    mut Boff: *mut mln_off_t,
    mut boff: *mut mln_off_t,
    mut size: mln_size_t,
) -> *mut mln_alloc_shm_t {
    let mut ret: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    ret = mln_alloc_shm_new(
        pool,
        (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as mln_size_t,
        0 as libc::c_int,
    );
    if ret.is_null() {
        return 0 as *mut mln_alloc_shm_t;
    }
    mln_alloc_shm_allowed(ret, Boff, boff, size);
    return ret;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_allowed(
    mut as_0: *mut mln_alloc_shm_t,
    mut Boff: *mut mln_off_t,
    mut boff: *mut mln_off_t,
    mut size: mln_size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = -(1 as libc::c_int);
    let mut s: libc::c_int = -(1 as libc::c_int);
    let mut n: libc::c_int = size
        .wrapping_add(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong)
        .wrapping_add(64 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(64 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pend: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut save: mln_u8ptr_t = 0 as mln_u8ptr_t;
    if n as libc::c_uint > (*as_0).nfree {
        return 0 as libc::c_int;
    }
    p = ((*as_0).bitmap).as_mut_ptr();
    pend = p.offset(4096 as libc::c_int as isize);
    while p < pend {
        if *p as libc::c_int & 0xff as libc::c_int == 0xff as libc::c_int {
            if !save.is_null() {
                j = -(1 as libc::c_int);
                s = -(1 as libc::c_int);
                save = 0 as mln_u8ptr_t;
            }
        } else {
            i = 7 as libc::c_int;
            while i >= 0 as libc::c_int {
                if *p as libc::c_int & (1 as libc::c_int as mln_u8_t as libc::c_int) << i
                    == 0
                {
                    if save.is_null() {
                        j = n;
                        s = i;
                        save = p;
                    }
                    j -= 1;
                    if j <= 0 as libc::c_int {
                        break;
                    }
                } else if !save.is_null() {
                    j = -(1 as libc::c_int);
                    s = -(1 as libc::c_int);
                    save = 0 as mln_u8ptr_t;
                }
                i -= 1;
                i;
            }
            if !save.is_null() && j == 0 {
                *Boff = save.offset_from(((*as_0).bitmap).as_mut_ptr()) as libc::c_long;
                *boff = s as mln_off_t;
                return 1 as libc::c_int;
            }
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_new(
    mut pool: *mut mln_alloc_t,
    mut size: mln_size_t,
    mut is_large: libc::c_int,
) -> *mut mln_alloc_shm_t {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut shm: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    let mut tmp: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    let mut p: mln_u8ptr_t = ((*pool).mem)
        .offset(::core::mem::size_of::<mln_alloc_t>() as libc::c_ulong as isize)
        as mln_u8ptr_t;
    tmp = (*pool).shm_head;
    while !tmp.is_null() {
        if ((*tmp).addr as mln_u8ptr_t).offset_from(p) as libc::c_long as libc::c_ulong
            >= size
        {
            break;
        }
        p = ((*tmp).addr).offset((*tmp).size as isize) as mln_u8ptr_t;
        tmp = (*tmp).next;
    }
    if tmp.is_null()
        && ((((*pool).mem).offset((*pool).shm_size as isize) as mln_u8ptr_t)
            .offset_from(p) as libc::c_long as libc::c_ulong) < size
    {
        return 0 as *mut mln_alloc_shm_t;
    }
    shm = p as *mut mln_alloc_shm_t;
    (*shm).pool = pool;
    (*shm).addr = p as *mut libc::c_void;
    (*shm).size = size;
    (*shm)
        .nfree = (if is_large != 0 {
        1 as libc::c_int as libc::c_ulong
    } else {
        size.wrapping_div(64 as libc::c_int as libc::c_ulong)
    }) as mln_u32_t;
    (*shm).set_base((*shm).nfree);
    (*shm).set_large(is_large as mln_u32_t);
    (*shm).next = 0 as *mut mln_alloc_shm_s;
    (*shm).prev = (*shm).next;
    if tmp.is_null() {
        mln_alloc_shm_chain_add(&mut (*pool).shm_head, &mut (*pool).shm_tail, shm);
    } else if tmp == (*pool).shm_head {
        (*shm).next = tmp;
        (*shm).prev = 0 as *mut mln_alloc_shm_s;
        (*tmp).prev = shm;
        (*pool).shm_head = shm;
    } else {
        (*shm).next = tmp;
        (*shm).prev = (*tmp).prev;
        (*(*tmp).prev).next = shm;
        (*tmp).prev = shm;
    }
    if is_large == 0 {
        memset(
            ((*shm).bitmap).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            4096 as libc::c_int as libc::c_ulong,
        );
        n = (::core::mem::size_of::<mln_alloc_shm_t>() as libc::c_ulong)
            .wrapping_add(64 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(64 as libc::c_int as libc::c_ulong) as libc::c_int;
        (*shm)
            .nfree = ((*shm).nfree as libc::c_uint).wrapping_sub(n as libc::c_uint)
            as mln_u32_t as mln_u32_t;
        (*shm).set_base((*shm).base() - n as mln_u32_t);
        i = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while n > 0 as libc::c_int {
            (*shm)
                .bitmap[i
                as usize] = ((*shm).bitmap[i as usize] as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int - j) as mln_u8_t;
            j += 1;
            if j >= 8 as libc::c_int {
                j = 0 as libc::c_int;
                i += 1;
                i;
            }
            n -= 1;
            n;
        }
    }
    return shm;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_chain_add(
    mut head: *mut *mut mln_alloc_shm_t,
    mut tail: *mut *mut mln_alloc_shm_t,
    mut node: *mut mln_alloc_shm_t,
) {
    (*node).next = 0 as *mut mln_alloc_shm_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_alloc_shm_large_m(
    mut pool: *mut mln_alloc_t,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    let mut as_0: *mut mln_alloc_shm_t = 0 as *mut mln_alloc_shm_t;
    let mut blk: *mut mln_alloc_blk_t = 0 as *mut mln_alloc_blk_t;
    as_0 = mln_alloc_shm_new(
        pool,
        size
            .wrapping_add(::core::mem::size_of::<mln_alloc_shm_t>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong),
        1 as libc::c_int,
    );
    if as_0.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*as_0).nfree = 0 as libc::c_int as mln_u32_t;
    blk = ((*as_0).addr)
        .offset(::core::mem::size_of::<mln_alloc_shm_t>() as libc::c_ulong as isize)
        as *mut mln_alloc_blk_t;
    memset(
        blk as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong,
    );
    (*blk).pool = pool;
    (*blk).blk_size = size;
    (*blk)
        .data = ((*as_0).addr)
        .offset(::core::mem::size_of::<mln_alloc_shm_t>() as libc::c_ulong as isize)
        .offset(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize);
    (*blk).chunk = as_0 as *mut mln_alloc_chunk_t;
    (*blk).set_is_large(1 as libc::c_int as mln_size_t);
    (*blk).set_in_used(1 as libc::c_int as mln_size_t);
    return (*blk).data;
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
unsafe extern "C" fn mln_alloc_re(
    mut pool: *mut mln_alloc_t,
    mut ptr: *mut libc::c_void,
    mut size: mln_size_t,
) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong {
        mln_alloc_free(ptr);
        return 0 as *mut libc::c_void;
    }
    let mut old_blk: *mut mln_alloc_blk_t = (ptr as mln_u8ptr_t)
        .offset(-(::core::mem::size_of::<mln_alloc_blk_t>() as libc::c_ulong as isize))
        as *mut mln_alloc_blk_t;
    if (*old_blk).pool == pool && (*old_blk).blk_size >= size {
        return ptr;
    }
    let mut new_ptr: mln_u8ptr_t = mln_alloc_m(pool, size) as mln_u8ptr_t;
    if new_ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    memcpy(new_ptr as *mut libc::c_void, ptr, (*old_blk).blk_size);
    mln_alloc_free(ptr);
    return new_ptr as *mut libc::c_void;
}
unsafe extern "C" fn mln_asn1_deresult_new(
    mut pool: *mut mln_alloc_t,
    mut _class: mln_u32_t,
    mut is_struct: mln_u32_t,
    mut ident: mln_u32_t,
    mut code: mln_u8ptr_t,
    mut code_len: mln_size_t,
    mut free_0: mln_u32_t,
) -> *mut mln_asn1_deresult_t {
    let mut res: *mut mln_asn1_deresult_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_asn1_deresult_t>() as libc::c_ulong,
    ) as *mut mln_asn1_deresult_t;
    if res.is_null() {
        return 0 as *mut mln_asn1_deresult_t;
    }
    (*res).pool = pool;
    (*res).code_buf = code;
    (*res).code_len = code_len;
    (*res)
        .contents = mln_alloc_m(
        pool,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut mln_asn1_deresult_t>() as libc::c_ulong,
            ),
    ) as *mut *mut mln_asn1_deresult_t;
    if ((*res).contents).is_null() {
        mln_alloc_free(res as *mut libc::c_void);
        return 0 as *mut mln_asn1_deresult_t;
    }
    (*res).max = 32 as libc::c_int as mln_u32_t;
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    (*res).set__class(_class);
    (*res).set_is_struct(is_struct);
    (*res).set_ident(ident);
    (*res).set_free(free_0);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_deresult_free(mut res: *mut mln_asn1_deresult_t) {
    if res.is_null() {
        return;
    }
    if !((*res).contents).is_null() {
        let mut p: *mut *mut mln_asn1_deresult_t = 0 as *mut *mut mln_asn1_deresult_t;
        let mut pend: *mut *mut mln_asn1_deresult_t = 0 as *mut *mut mln_asn1_deresult_t;
        pend = ((*res).contents).offset((*res).pos as isize);
        p = (*res).contents;
        while p < pend {
            mln_asn1_deresult_free(*p);
            p = p.offset(1);
            p;
        }
        mln_alloc_free((*res).contents as *mut libc::c_void);
    }
    if (*res).free() != 0 {
        mln_alloc_free((*res).code_buf as *mut libc::c_void);
    }
    mln_alloc_free(res as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_decode_chain(
    mut in_0: *mut mln_chain_t,
    mut err: *mut libc::c_int,
    mut pool: *mut mln_alloc_t,
) -> *mut mln_asn1_deresult_t {
    let mut code: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut code_len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    let mut n: libc::c_int = 0;
    c = in_0;
    while !c.is_null() {
        if !((*c).buf).is_null() {
            code_len = (code_len as libc::c_ulong)
                .wrapping_add(
                    (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*(*c).buf).in_file() as libc::c_int != 0 {
                        (*(*c).buf).file_last - (*(*c).buf).file_pos
                    } else {
                        ((*(*c).buf).last).offset_from((*(*c).buf).pos) as libc::c_long
                    }) as libc::c_ulong,
                ) as mln_u64_t as mln_u64_t;
        }
        c = (*c).next;
    }
    code = mln_alloc_m(pool, code_len) as mln_u8ptr_t;
    if code.is_null() {
        *err = 2 as libc::c_int;
        return 0 as *mut mln_asn1_deresult_t;
    }
    p = code;
    c = in_0;
    while !c.is_null() {
        b = (*c).buf;
        if !b.is_null() {
            if (*b).in_memory() != 0 {
                memcpy(
                    p as *mut libc::c_void,
                    (*b).pos as *const libc::c_void,
                    (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*b).in_file() as libc::c_int != 0 {
                        (*b).file_last - (*b).file_pos
                    } else {
                        ((*b).last).offset_from((*b).pos) as libc::c_long
                    }) as libc::c_ulong,
                );
            } else {
                if ((*b).file).is_null() {
                    mln_alloc_free(code as *mut libc::c_void);
                    return 0 as *mut mln_asn1_deresult_t;
                }
                lseek((*(*b).file).fd, (*b).file_pos, 0 as libc::c_int);
                n = read(
                    (*(*b).file).fd,
                    p as *mut libc::c_void,
                    (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*b).in_file() as libc::c_int != 0 {
                            (*b).file_last - (*b).file_pos
                        } else {
                            ((*b).last).offset_from((*b).pos) as libc::c_long
                        })
                    }) as size_t,
                ) as libc::c_int;
                if n as libc::c_long
                    != (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*b).in_file() as libc::c_int != 0 {
                            (*b).file_last - (*b).file_pos
                        } else {
                            ((*b).last).offset_from((*b).pos) as libc::c_long
                        })
                    })
                {
                    mln_alloc_free(code as *mut libc::c_void);
                    return 0 as *mut mln_asn1_deresult_t;
                }
            }
            p = p
                .offset(
                    (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*b).in_file() as libc::c_int != 0 {
                        (*b).file_last - (*b).file_pos
                    } else {
                        ((*b).last).offset_from((*b).pos) as libc::c_long
                    }) as isize,
                );
        }
        c = (*c).next;
    }
    p = code;
    res = mln_asn1_decode_recursive(
        &mut p,
        &mut code_len,
        err,
        pool,
        1 as libc::c_int as mln_u32_t,
    );
    if res.is_null() {
        mln_alloc_free(code as *mut libc::c_void);
        return 0 as *mut mln_asn1_deresult_t;
    }
    *err = 0 as libc::c_int;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_decode(
    mut data: *mut libc::c_void,
    mut len: mln_u64_t,
    mut err: *mut libc::c_int,
    mut pool: *mut mln_alloc_t,
) -> *mut mln_asn1_deresult_t {
    let mut buf: mln_u8ptr_t = data as mln_u8ptr_t;
    return mln_asn1_decode_recursive(
        &mut buf,
        &mut len,
        err,
        pool,
        1 as libc::c_int as mln_u32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_decode_ref(
    mut data: *mut libc::c_void,
    mut len: mln_u64_t,
    mut err: *mut libc::c_int,
    mut pool: *mut mln_alloc_t,
) -> *mut mln_asn1_deresult_t {
    let mut buf: mln_u8ptr_t = data as mln_u8ptr_t;
    return mln_asn1_decode_recursive(
        &mut buf,
        &mut len,
        err,
        pool,
        0 as libc::c_int as mln_u32_t,
    );
}
unsafe extern "C" fn mln_asn1_decode_recursive(
    mut code: *mut mln_u8ptr_t,
    mut code_len: *mut mln_u64_t,
    mut err: *mut libc::c_int,
    mut pool: *mut mln_alloc_t,
    mut free_0: mln_u32_t,
) -> *mut mln_asn1_deresult_t {
    let mut current_block: u64;
    let mut p: mln_u8ptr_t = *code;
    let mut end: mln_u8ptr_t = (*code).offset(*code_len as isize);
    let mut _class: mln_u32_t = 0;
    let mut struc: mln_u32_t = 0;
    let mut ident: mln_u32_t = 0;
    let mut ch: mln_u8_t = 0;
    let mut length: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    let mut sub_res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    if !(*code_len < 2 as libc::c_int as libc::c_ulong) {
        let fresh1 = p;
        p = p.offset(1);
        ch = *fresh1;
        _class = (ch as libc::c_int >> 6 as libc::c_int & 0x3 as libc::c_int)
            as mln_u32_t;
        struc = (ch as libc::c_int >> 5 as libc::c_int & 0x1 as libc::c_int)
            as mln_u32_t;
        ident = (ch as libc::c_int & 0x1f as libc::c_int) as mln_u32_t;
        let fresh2 = p;
        p = p.offset(1);
        ch = *fresh2;
        if ch as libc::c_int & 0x80 as libc::c_int != 0 {
            if (ch as libc::c_int & 0x7f as libc::c_int) as libc::c_ulong
                > ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong
            {
                *err = 2 as libc::c_int;
                return 0 as *mut mln_asn1_deresult_t;
            }
            let mut tmp: [mln_u8_t; 8] = [
                0 as libc::c_int as mln_u8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            let mut q: *mut mln_u8_t = 0 as *mut mln_u8_t;
            let mut qend: *mut mln_u8_t = 0 as *mut mln_u8_t;
            if p.offset((ch as libc::c_int & 0x7f as libc::c_int) as isize) >= end {
                current_block = 16757219340313469753;
            } else {
                memcpy(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    p as *const libc::c_void,
                    (ch as libc::c_int & 0x7f as libc::c_int) as libc::c_ulong,
                );
                p = p.offset((ch as libc::c_int & 0x7f as libc::c_int) as isize);
                q = tmp.as_mut_ptr();
                qend = tmp
                    .as_mut_ptr()
                    .offset((ch as libc::c_int & 0x7f as libc::c_int) as isize);
                while q < qend {
                    length
                        |= ((*q as libc::c_int)
                            << ((qend.offset_from(q) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) << 3 as libc::c_int))
                            as libc::c_ulong;
                    q = q.offset(1);
                    q;
                }
                current_block = 13242334135786603907;
            }
        } else {
            length = ch as mln_u64_t;
            current_block = 13242334135786603907;
        }
        match current_block {
            16757219340313469753 => {}
            _ => {
                if !((end.offset_from(p) as libc::c_long as libc::c_ulong) < length) {
                    res = mln_asn1_deresult_new(
                        pool,
                        _class,
                        struc,
                        ident,
                        *code,
                        (p.offset_from(*code) as libc::c_long as libc::c_ulong)
                            .wrapping_add(length),
                        free_0,
                    );
                    if res.is_null() {
                        *err = 2 as libc::c_int;
                        return 0 as *mut mln_asn1_deresult_t;
                    }
                    if struc != 0 {
                        while length > 0 as libc::c_int as libc::c_ulong {
                            sub_res = mln_asn1_decode_recursive(
                                &mut p,
                                &mut length,
                                err,
                                pool,
                                0 as libc::c_int as mln_u32_t,
                            );
                            if sub_res.is_null() {
                                (*res).set_free(0 as libc::c_int as mln_u32_t);
                                mln_asn1_deresult_free(res);
                                return 0 as *mut mln_asn1_deresult_t;
                            }
                            if (*res).pos >= (*res).max {
                                (*res)
                                    .max = ((*res).max as libc::c_uint)
                                    .wrapping_add((*res).max >> 1 as libc::c_int) as mln_u32_t
                                    as mln_u32_t;
                                let mut ptr: *mut *mut mln_asn1_deresult_t = 0
                                    as *mut *mut mln_asn1_deresult_t;
                                ptr = mln_alloc_re(
                                    pool,
                                    (*res).contents as *mut libc::c_void,
                                    ((*res).max as libc::c_ulong)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut mln_asn1_deresult_t>()
                                                as libc::c_ulong,
                                        ),
                                ) as *mut *mut mln_asn1_deresult_t;
                                if ptr.is_null() {
                                    (*res).set_free(0 as libc::c_int as mln_u32_t);
                                    mln_asn1_deresult_free(res);
                                    *err = 2 as libc::c_int;
                                    return 0 as *mut mln_asn1_deresult_t;
                                }
                                (*res).contents = ptr;
                            }
                            let fresh3 = (*res).pos;
                            (*res).pos = ((*res).pos).wrapping_add(1);
                            let ref mut fresh4 = *((*res).contents)
                                .offset(fresh3 as isize);
                            *fresh4 = sub_res;
                        }
                    } else {
                        sub_res = mln_asn1_deresult_new(
                            pool,
                            0 as libc::c_int as mln_u32_t,
                            0 as libc::c_int as mln_u32_t,
                            0xfffffff as libc::c_int as mln_u32_t,
                            p,
                            length,
                            0 as libc::c_int as mln_u32_t,
                        );
                        if sub_res.is_null() {
                            (*res).set_free(0 as libc::c_int as mln_u32_t);
                            mln_asn1_deresult_free(res);
                            *err = 2 as libc::c_int;
                            return 0 as *mut mln_asn1_deresult_t;
                        }
                        let fresh5 = (*res).pos;
                        (*res).pos = ((*res).pos).wrapping_add(1);
                        let ref mut fresh6 = *((*res).contents).offset(fresh5 as isize);
                        *fresh6 = sub_res;
                        p = p.offset(length as isize);
                    }
                    *code = p;
                    *code_len = end.offset_from(p) as libc::c_long as mln_u64_t;
                    return res;
                }
            }
        }
    }
    *err = 1 as libc::c_int;
    return 0 as *mut mln_asn1_deresult_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_deresult_content_get(
    mut res: *mut mln_asn1_deresult_t,
    mut index: mln_u32_t,
) -> *mut mln_asn1_deresult_t {
    if ((*res).contents).is_null() {
        return 0 as *mut mln_asn1_deresult_t;
    }
    if (*res).pos <= index {
        return 0 as *mut mln_asn1_deresult_t;
    }
    return *((*res).contents).offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_deresult_dump(mut res: *mut mln_asn1_deresult_t) {
    if res.is_null() {
        return;
    }
    mln_asn1_deresult_dump_recursive(res, 0 as libc::c_int as mln_u32_t);
}
unsafe extern "C" fn mln_asn1_deresult_dump_recursive(
    mut res: *mut mln_asn1_deresult_t,
    mut nblank: mln_u32_t,
) {
    let mut p: *mut *mut mln_asn1_deresult_t = 0 as *mut *mut mln_asn1_deresult_t;
    let mut end: *mut *mut mln_asn1_deresult_t = 0 as *mut *mut mln_asn1_deresult_t;
    let mut i: mln_u32_t = 0;
    let mut classes: [*mut libc::c_char; 4] = [
        b"universal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"application\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"context specific\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"private\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut idents: [*mut libc::c_char; 15] = [
        b"None\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Boolean\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Integer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Bit string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Octet string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"NULL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Object identifier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"UTF8String\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Sequence/Sequence Of\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"Set/Set Of\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"PrintableString\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"T61String\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"IA5String\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"UTCTime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"GeneralizedTime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut unknown: *mut libc::c_char = b"Unknown\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut pclass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pident: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as mln_u32_t;
    while i < nblank {
        printf(b" \0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
    match (*res)._class() as libc::c_int {
        0 => {
            pclass = classes[0 as libc::c_int as usize];
        }
        1 => {
            pclass = classes[1 as libc::c_int as usize];
        }
        2 => {
            pclass = classes[2 as libc::c_int as usize];
        }
        3 => {
            pclass = classes[3 as libc::c_int as usize];
        }
        _ => {
            pclass = unknown;
        }
    }
    match (*res).ident() as libc::c_int {
        268435455 => {
            pident = idents[0 as libc::c_int as usize];
        }
        1 => {
            pident = idents[1 as libc::c_int as usize];
        }
        2 => {
            pident = idents[2 as libc::c_int as usize];
        }
        3 => {
            pident = idents[3 as libc::c_int as usize];
        }
        4 => {
            pident = idents[4 as libc::c_int as usize];
        }
        5 => {
            pident = idents[5 as libc::c_int as usize];
        }
        6 => {
            pident = idents[6 as libc::c_int as usize];
        }
        12 => {
            pident = idents[7 as libc::c_int as usize];
        }
        16 => {
            pident = idents[8 as libc::c_int as usize];
        }
        17 => {
            pident = idents[9 as libc::c_int as usize];
        }
        19 => {
            pident = idents[10 as libc::c_int as usize];
        }
        20 => {
            pident = idents[11 as libc::c_int as usize];
        }
        22 => {
            pident = idents[12 as libc::c_int as usize];
        }
        23 => {
            pident = idents[13 as libc::c_int as usize];
        }
        24 => {
            pident = idents[14 as libc::c_int as usize];
        }
        _ => {
            pident = unknown;
        }
    }
    printf(
        b"class:[%s][0x%x] is_struct:[%u] ident:[%s][0x%x] free:[%u] nContent:[%u]\n\0"
            as *const u8 as *const libc::c_char,
        pclass,
        (*res)._class() as libc::c_int,
        (*res).is_struct() as libc::c_int,
        pident,
        (*res).ident() as libc::c_int,
        (*res).free() as libc::c_int,
        (*res).pos,
    );
    i = 0 as libc::c_int as mln_u32_t;
    while i < nblank {
        printf(b" \0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
    printf(b"[\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < (*res).code_len {
        printf(
            b"%02x \0" as *const u8 as *const libc::c_char,
            *((*res).code_buf).offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"]\n\0" as *const u8 as *const libc::c_char);
    end = ((*res).contents).offset((*res).pos as isize);
    p = (*res).contents;
    while p < end {
        mln_asn1_deresult_dump_recursive(
            *p,
            nblank.wrapping_add(2 as libc::c_int as libc::c_uint),
        );
        p = p.offset(1);
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_enresult_init(
    mut res: *mut mln_asn1_enresult_t,
    mut pool: *mut mln_alloc_t,
) -> libc::c_int {
    (*res).pool = pool;
    (*res).size = 0 as libc::c_int as mln_u64_t;
    (*res)
        .contents = mln_alloc_m(
        pool,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<mln_string_t>() as libc::c_ulong),
    ) as *mut mln_string_t;
    if ((*res).contents).is_null() {
        return 2 as libc::c_int;
    }
    (*res).max = 32 as libc::c_int as mln_u32_t;
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    (*res).set__class(0 as libc::c_int as mln_u32_t);
    (*res).set_is_struct(0 as libc::c_int as mln_u32_t);
    (*res).set_ident(0xfffffff as libc::c_int as mln_u32_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_enresult_destroy(mut res: *mut mln_asn1_enresult_t) {
    if res.is_null() {
        return;
    }
    if !((*res).contents).is_null() {
        let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
        let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
        send = ((*res).contents).offset((*res).pos as isize);
        s = (*res).contents;
        while s < send {
            mln_alloc_free((*s).data as *mut libc::c_void);
            s = s.offset(1);
            s;
        }
        mln_alloc_free((*res).contents as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_enresult_new(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_asn1_enresult_t {
    let mut res: *mut mln_asn1_enresult_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_asn1_enresult_t>() as libc::c_ulong,
    ) as *mut mln_asn1_enresult_t;
    if res.is_null() {
        return 0 as *mut mln_asn1_enresult_t;
    }
    if mln_asn1_enresult_init(res, pool) != 0 as libc::c_int {
        mln_alloc_free(res as *mut libc::c_void);
        return 0 as *mut mln_asn1_enresult_t;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_enresult_free(mut res: *mut mln_asn1_enresult_t) {
    if res.is_null() {
        return;
    }
    mln_asn1_enresult_destroy(res);
    mln_alloc_free(res as *mut libc::c_void);
}
unsafe extern "C" fn mln_asn1_encode_add_content(
    mut res: *mut mln_asn1_enresult_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_u64_t,
) -> libc::c_int {
    if (*res).pos == (*res).max {
        (*res)
            .max = ((*res).max as libc::c_uint)
            .wrapping_add((*res).max >> 1 as libc::c_int) as mln_u32_t as mln_u32_t;
        let mut ptr: *mut mln_string_t = mln_alloc_re(
            (*res).pool,
            (*res).contents as *mut libc::c_void,
            ((*res).max as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<mln_string_t>() as libc::c_ulong),
        ) as *mut mln_string_t;
        if ptr.is_null() {
            return -(1 as libc::c_int);
        }
        (*res).contents = ptr;
    }
    let fresh7 = (*res).pos;
    (*res).pos = ((*res).pos).wrapping_add(1);
    let mut s: *mut mln_string_t = &mut *((*res).contents).offset(fresh7 as isize)
        as *mut mln_string_t;
    (*s).data = buf;
    (*s).len = len;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(1 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_sequence(
    mut res: *mut mln_asn1_enresult_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    len = ((*res).size).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            16 as libc::c_int
        })) as libc::c_uchar;
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh8 = p;
                                    p = p.offset(1);
                                    *fresh8 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh9 = p;
                                    p = p.offset(1);
                                    *fresh9 = ((*res).size >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh10 = p;
                                    p = p.offset(1);
                                    *fresh10 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh11 = p;
                                    p = p.offset(1);
                                    *fresh11 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh12 = p;
                                    p = p.offset(1);
                                    *fresh12 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh13 = p;
                                    p = p.offset(1);
                                    *fresh13 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh14 = p;
                                    p = p.offset(1);
                                    *fresh14 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh15 = p;
                                    p = p.offset(1);
                                    *fresh15 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh16 = p;
                                    p = p.offset(1);
                                    *fresh16 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh17 = p;
                                    p = p.offset(1);
                                    *fresh17 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh18 = p;
                                    p = p.offset(1);
                                    *fresh18 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh19 = p;
                                    p = p.offset(1);
                                    *fresh19 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh20 = p;
                                    p = p.offset(1);
                                    *fresh20 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    *fresh21 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh22 = p;
                                    p = p.offset(1);
                                    *fresh22 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh23 = p;
                                    p = p.offset(1);
                                    *fresh23 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh24 = p;
                                    p = p.offset(1);
                                    *fresh24 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh25 = p;
                                p = p.offset(1);
                                *fresh25 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh26 = p;
                                p = p.offset(1);
                                *fresh26 = ((*res).size >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh27 = p;
                                p = p.offset(1);
                                *fresh27 = ((*res).size >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh28 = p;
                                p = p.offset(1);
                                *fresh28 = ((*res).size >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh29 = p;
                                p = p.offset(1);
                                *fresh29 = ((*res).size >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh30 = p;
                                p = p.offset(1);
                                *fresh30 = ((*res).size >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh31 = p;
                                p = p.offset(1);
                                *fresh31 = ((*res).size
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh32 = p;
                            p = p.offset(1);
                            *fresh32 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh33 = p;
                            p = p.offset(1);
                            *fresh33 = ((*res).size >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh34 = p;
                            p = p.offset(1);
                            *fresh34 = ((*res).size >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh35 = p;
                            p = p.offset(1);
                            *fresh35 = ((*res).size >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh36 = p;
                            p = p.offset(1);
                            *fresh36 = ((*res).size >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh37 = p;
                            p = p.offset(1);
                            *fresh37 = ((*res).size
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh38 = p;
                        p = p.offset(1);
                        *fresh38 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh39 = p;
                        p = p.offset(1);
                        *fresh39 = ((*res).size >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh40 = p;
                        p = p.offset(1);
                        *fresh40 = ((*res).size >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh41 = p;
                        p = p.offset(1);
                        *fresh41 = ((*res).size >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh42 = p;
                        p = p.offset(1);
                        *fresh42 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh43 = p;
                    p = p.offset(1);
                    *fresh43 = (0x80 as libc::c_int | 3 as libc::c_int) as libc::c_uchar;
                    let fresh44 = p;
                    p = p.offset(1);
                    *fresh44 = ((*res).size >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh45 = p;
                    p = p.offset(1);
                    *fresh45 = ((*res).size >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh46 = p;
                    p = p.offset(1);
                    *fresh46 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh47 = p;
                p = p.offset(1);
                *fresh47 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh48 = p;
                p = p.offset(1);
                *fresh48 = ((*res).size >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh49 = p;
                p = p.offset(1);
                *fresh49 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh50 = p;
            p = p.offset(1);
            *fresh50 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh51 = p;
            p = p.offset(1);
            *fresh51 = (*res).size as libc::c_uchar;
        }
    } else {
        let fresh52 = p;
        p = p.offset(1);
        *fresh52 = (*res).size as libc::c_uchar;
    }
    send = ((*res).contents).offset((*res).pos as isize);
    s = (*res).contents;
    while s < send {
        memcpy(p as *mut libc::c_void, (*s).data as *const libc::c_void, (*s).len);
        p = p.offset((*s).len as isize);
        mln_alloc_free((*s).data as *mut libc::c_void);
        s = s.offset(1);
        s;
    }
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res).size = len;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_boolean(
    mut res: *mut mln_asn1_enresult_t,
    mut val: mln_u8_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    len = (1 as libc::c_int + 1 as libc::c_int) as mln_u64_t;
    if 1 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 1 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 1 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 1 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 1 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            1 as libc::c_int
        })) as libc::c_uchar;
    if 1 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 1 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 1 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 1 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 1 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 1 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh53 = p;
                                    p = p.offset(1);
                                    *fresh53 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh54 = p;
                                    p = p.offset(1);
                                    *fresh54 = (1 as libc::c_int as mln_u64_t
                                        >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh55 = p;
                                    p = p.offset(1);
                                    *fresh55 = (1 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh56 = p;
                                    p = p.offset(1);
                                    *fresh56 = (1 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh57 = p;
                                    p = p.offset(1);
                                    *fresh57 = (1 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh58 = p;
                                    p = p.offset(1);
                                    *fresh58 = (1 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh59 = p;
                                    p = p.offset(1);
                                    *fresh59 = (1 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh60 = p;
                                    p = p.offset(1);
                                    *fresh60 = (1 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh61 = p;
                                    p = p.offset(1);
                                    *fresh61 = (1 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh62 = p;
                                    p = p.offset(1);
                                    *fresh62 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh63 = p;
                                    p = p.offset(1);
                                    *fresh63 = (1 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh64 = p;
                                    p = p.offset(1);
                                    *fresh64 = (1 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh65 = p;
                                    p = p.offset(1);
                                    *fresh65 = (1 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh66 = p;
                                    p = p.offset(1);
                                    *fresh66 = (1 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh67 = p;
                                    p = p.offset(1);
                                    *fresh67 = (1 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh68 = p;
                                    p = p.offset(1);
                                    *fresh68 = (1 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh69 = p;
                                    p = p.offset(1);
                                    *fresh69 = (1 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh70 = p;
                                p = p.offset(1);
                                *fresh70 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh71 = p;
                                p = p.offset(1);
                                *fresh71 = (1 as libc::c_int as mln_u64_t
                                    >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh72 = p;
                                p = p.offset(1);
                                *fresh72 = (1 as libc::c_int as mln_u64_t
                                    >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh73 = p;
                                p = p.offset(1);
                                *fresh73 = (1 as libc::c_int as mln_u64_t
                                    >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh74 = p;
                                p = p.offset(1);
                                *fresh74 = (1 as libc::c_int as mln_u64_t
                                    >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh75 = p;
                                p = p.offset(1);
                                *fresh75 = (1 as libc::c_int as mln_u64_t
                                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh76 = p;
                                p = p.offset(1);
                                *fresh76 = (1 as libc::c_int as mln_u64_t
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh77 = p;
                            p = p.offset(1);
                            *fresh77 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh78 = p;
                            p = p.offset(1);
                            *fresh78 = (1 as libc::c_int as mln_u64_t
                                >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh79 = p;
                            p = p.offset(1);
                            *fresh79 = (1 as libc::c_int as mln_u64_t
                                >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh80 = p;
                            p = p.offset(1);
                            *fresh80 = (1 as libc::c_int as mln_u64_t
                                >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh81 = p;
                            p = p.offset(1);
                            *fresh81 = (1 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh82 = p;
                            p = p.offset(1);
                            *fresh82 = (1 as libc::c_int as mln_u64_t
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh83 = p;
                        p = p.offset(1);
                        *fresh83 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh84 = p;
                        p = p.offset(1);
                        *fresh84 = (1 as libc::c_int as mln_u64_t >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh85 = p;
                        p = p.offset(1);
                        *fresh85 = (1 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh86 = p;
                        p = p.offset(1);
                        *fresh86 = (1 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh87 = p;
                        p = p.offset(1);
                        *fresh87 = (1 as libc::c_int as mln_u64_t
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    }
                } else {
                    let fresh88 = p;
                    p = p.offset(1);
                    *fresh88 = (0x80 as libc::c_int | 3 as libc::c_int) as libc::c_uchar;
                    let fresh89 = p;
                    p = p.offset(1);
                    *fresh89 = (1 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh90 = p;
                    p = p.offset(1);
                    *fresh90 = (1 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh91 = p;
                    p = p.offset(1);
                    *fresh91 = (1 as libc::c_int as mln_u64_t
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                }
            } else {
                let fresh92 = p;
                p = p.offset(1);
                *fresh92 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh93 = p;
                p = p.offset(1);
                *fresh93 = (1 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh94 = p;
                p = p.offset(1);
                *fresh94 = (1 as libc::c_int as mln_u64_t
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            }
        } else {
            let fresh95 = p;
            p = p.offset(1);
            *fresh95 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh96 = p;
            p = p.offset(1);
            *fresh96 = 1 as libc::c_int as libc::c_uchar;
        }
    } else {
        let fresh97 = p;
        p = p.offset(1);
        *fresh97 = 1 as libc::c_int as libc::c_uchar;
    }
    *p = val;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_integer(
    mut res: *mut mln_asn1_enresult_t,
    mut ints: mln_u8ptr_t,
    mut nints: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    len = nints.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if nints > 127 as libc::c_int as libc::c_ulong {
        if nints > 0xff as libc::c_int as libc::c_ulong {
            if nints > 0xffff as libc::c_int as libc::c_ulong {
                if nints > 0xffffff as libc::c_int as libc::c_ulong {
                    if nints > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if nints as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if nints as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if nints as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            2 as libc::c_int
        })) as libc::c_uchar;
    if nints > 127 as libc::c_int as libc::c_ulong {
        if nints > 0xff as libc::c_int as libc::c_ulong {
            if nints > 0xffff as libc::c_int as libc::c_ulong {
                if nints > 0xffffff as libc::c_int as libc::c_ulong {
                    if nints > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if nints as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if nints as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if nints as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh98 = p;
                                    p = p.offset(1);
                                    *fresh98 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh99 = p;
                                    p = p.offset(1);
                                    *fresh99 = (nints >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh100 = p;
                                    p = p.offset(1);
                                    *fresh100 = (nints >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh101 = p;
                                    p = p.offset(1);
                                    *fresh101 = (nints >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh102 = p;
                                    p = p.offset(1);
                                    *fresh102 = (nints >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh103 = p;
                                    p = p.offset(1);
                                    *fresh103 = (nints >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh104 = p;
                                    p = p.offset(1);
                                    *fresh104 = (nints >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh105 = p;
                                    p = p.offset(1);
                                    *fresh105 = (nints >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh106 = p;
                                    p = p.offset(1);
                                    *fresh106 = (nints & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh107 = p;
                                    p = p.offset(1);
                                    *fresh107 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh108 = p;
                                    p = p.offset(1);
                                    *fresh108 = (nints >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh109 = p;
                                    p = p.offset(1);
                                    *fresh109 = (nints >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh110 = p;
                                    p = p.offset(1);
                                    *fresh110 = (nints >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh111 = p;
                                    p = p.offset(1);
                                    *fresh111 = (nints >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh112 = p;
                                    p = p.offset(1);
                                    *fresh112 = (nints >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh113 = p;
                                    p = p.offset(1);
                                    *fresh113 = (nints >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh114 = p;
                                    p = p.offset(1);
                                    *fresh114 = (nints & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh115 = p;
                                p = p.offset(1);
                                *fresh115 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh116 = p;
                                p = p.offset(1);
                                *fresh116 = (nints >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh117 = p;
                                p = p.offset(1);
                                *fresh117 = (nints >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh118 = p;
                                p = p.offset(1);
                                *fresh118 = (nints >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh119 = p;
                                p = p.offset(1);
                                *fresh119 = (nints >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh120 = p;
                                p = p.offset(1);
                                *fresh120 = (nints >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh121 = p;
                                p = p.offset(1);
                                *fresh121 = (nints & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh122 = p;
                            p = p.offset(1);
                            *fresh122 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh123 = p;
                            p = p.offset(1);
                            *fresh123 = (nints >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh124 = p;
                            p = p.offset(1);
                            *fresh124 = (nints >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh125 = p;
                            p = p.offset(1);
                            *fresh125 = (nints >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh126 = p;
                            p = p.offset(1);
                            *fresh126 = (nints >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh127 = p;
                            p = p.offset(1);
                            *fresh127 = (nints & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh128 = p;
                        p = p.offset(1);
                        *fresh128 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh129 = p;
                        p = p.offset(1);
                        *fresh129 = (nints >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh130 = p;
                        p = p.offset(1);
                        *fresh130 = (nints >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh131 = p;
                        p = p.offset(1);
                        *fresh131 = (nints >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh132 = p;
                        p = p.offset(1);
                        *fresh132 = (nints & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh133 = p;
                    p = p.offset(1);
                    *fresh133 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh134 = p;
                    p = p.offset(1);
                    *fresh134 = (nints >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh135 = p;
                    p = p.offset(1);
                    *fresh135 = (nints >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh136 = p;
                    p = p.offset(1);
                    *fresh136 = (nints & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh137 = p;
                p = p.offset(1);
                *fresh137 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh138 = p;
                p = p.offset(1);
                *fresh138 = (nints >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh139 = p;
                p = p.offset(1);
                *fresh139 = (nints & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh140 = p;
            p = p.offset(1);
            *fresh140 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh141 = p;
            p = p.offset(1);
            *fresh141 = nints as libc::c_uchar;
        }
    } else {
        let fresh142 = p;
        p = p.offset(1);
        *fresh142 = nints as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, ints as *const libc::c_void, nints);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_bitstring(
    mut res: *mut mln_asn1_enresult_t,
    mut bits: mln_u8ptr_t,
    mut nbits: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut bytes: mln_u64_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (if nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong) != 0 {
                (nbits >> 3 as libc::c_int)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                nbits >> 3 as libc::c_int
            }),
        );
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = bytes.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if bytes > 127 as libc::c_int as libc::c_ulong {
        if bytes > 0xff as libc::c_int as libc::c_ulong {
            if bytes > 0xffff as libc::c_int as libc::c_ulong {
                if bytes > 0xffffff as libc::c_int as libc::c_ulong {
                    if bytes > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if bytes as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if bytes as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if bytes as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            3 as libc::c_int
        })) as libc::c_uchar;
    if bytes > 127 as libc::c_int as libc::c_ulong {
        if bytes > 0xff as libc::c_int as libc::c_ulong {
            if bytes > 0xffff as libc::c_int as libc::c_ulong {
                if bytes > 0xffffff as libc::c_int as libc::c_ulong {
                    if bytes > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if bytes as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if bytes as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if bytes as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh143 = p;
                                    p = p.offset(1);
                                    *fresh143 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh144 = p;
                                    p = p.offset(1);
                                    *fresh144 = (bytes >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh145 = p;
                                    p = p.offset(1);
                                    *fresh145 = (bytes >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh146 = p;
                                    p = p.offset(1);
                                    *fresh146 = (bytes >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh147 = p;
                                    p = p.offset(1);
                                    *fresh147 = (bytes >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh148 = p;
                                    p = p.offset(1);
                                    *fresh148 = (bytes >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh149 = p;
                                    p = p.offset(1);
                                    *fresh149 = (bytes >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh150 = p;
                                    p = p.offset(1);
                                    *fresh150 = (bytes >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh151 = p;
                                    p = p.offset(1);
                                    *fresh151 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh152 = p;
                                    p = p.offset(1);
                                    *fresh152 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh153 = p;
                                    p = p.offset(1);
                                    *fresh153 = (bytes >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh154 = p;
                                    p = p.offset(1);
                                    *fresh154 = (bytes >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh155 = p;
                                    p = p.offset(1);
                                    *fresh155 = (bytes >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh156 = p;
                                    p = p.offset(1);
                                    *fresh156 = (bytes >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh157 = p;
                                    p = p.offset(1);
                                    *fresh157 = (bytes >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh158 = p;
                                    p = p.offset(1);
                                    *fresh158 = (bytes >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh159 = p;
                                    p = p.offset(1);
                                    *fresh159 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh160 = p;
                                p = p.offset(1);
                                *fresh160 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh161 = p;
                                p = p.offset(1);
                                *fresh161 = (bytes >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh162 = p;
                                p = p.offset(1);
                                *fresh162 = (bytes >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh163 = p;
                                p = p.offset(1);
                                *fresh163 = (bytes >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh164 = p;
                                p = p.offset(1);
                                *fresh164 = (bytes >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh165 = p;
                                p = p.offset(1);
                                *fresh165 = (bytes >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh166 = p;
                                p = p.offset(1);
                                *fresh166 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh167 = p;
                            p = p.offset(1);
                            *fresh167 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh168 = p;
                            p = p.offset(1);
                            *fresh168 = (bytes >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh169 = p;
                            p = p.offset(1);
                            *fresh169 = (bytes >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh170 = p;
                            p = p.offset(1);
                            *fresh170 = (bytes >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh171 = p;
                            p = p.offset(1);
                            *fresh171 = (bytes >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh172 = p;
                            p = p.offset(1);
                            *fresh172 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh173 = p;
                        p = p.offset(1);
                        *fresh173 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh174 = p;
                        p = p.offset(1);
                        *fresh174 = (bytes >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh175 = p;
                        p = p.offset(1);
                        *fresh175 = (bytes >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh176 = p;
                        p = p.offset(1);
                        *fresh176 = (bytes >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh177 = p;
                        p = p.offset(1);
                        *fresh177 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh178 = p;
                    p = p.offset(1);
                    *fresh178 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh179 = p;
                    p = p.offset(1);
                    *fresh179 = (bytes >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh180 = p;
                    p = p.offset(1);
                    *fresh180 = (bytes >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh181 = p;
                    p = p.offset(1);
                    *fresh181 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh182 = p;
                p = p.offset(1);
                *fresh182 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh183 = p;
                p = p.offset(1);
                *fresh183 = (bytes >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh184 = p;
                p = p.offset(1);
                *fresh184 = (bytes & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh185 = p;
            p = p.offset(1);
            *fresh185 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh186 = p;
            p = p.offset(1);
            *fresh186 = bytes as libc::c_uchar;
        }
    } else {
        let fresh187 = p;
        p = p.offset(1);
        *fresh187 = bytes as libc::c_uchar;
    }
    let fresh188 = p;
    p = p.offset(1);
    *fresh188 = (if nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong) != 0 {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong))
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uchar;
    if bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0 {
        memcpy(
            p as *mut libc::c_void,
            bits as *const libc::c_void,
            bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    p = p.offset(bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    if nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong) != 0 {
        p = p.offset(-1);
        p;
        *p = ((*p as libc::c_int
            >> (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong))
            & 0xff as libc::c_int)
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(nbits.wrapping_rem(8 as libc::c_int as libc::c_ulong)))
            as libc::c_uchar;
    }
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_octetstring(
    mut res: *mut mln_asn1_enresult_t,
    mut octets: mln_u8ptr_t,
    mut n: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = n.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if n > 127 as libc::c_int as libc::c_ulong {
        if n > 0xff as libc::c_int as libc::c_ulong {
            if n > 0xffff as libc::c_int as libc::c_ulong {
                if n > 0xffffff as libc::c_int as libc::c_ulong {
                    if n > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if n as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong {
                            if n as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if n as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            4 as libc::c_int
        })) as libc::c_uchar;
    if n > 127 as libc::c_int as libc::c_ulong {
        if n > 0xff as libc::c_int as libc::c_ulong {
            if n > 0xffff as libc::c_int as libc::c_ulong {
                if n > 0xffffff as libc::c_int as libc::c_ulong {
                    if n > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if n as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong {
                            if n as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if n as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh189 = p;
                                    p = p.offset(1);
                                    *fresh189 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh190 = p;
                                    p = p.offset(1);
                                    *fresh190 = (n >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh191 = p;
                                    p = p.offset(1);
                                    *fresh191 = (n >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh192 = p;
                                    p = p.offset(1);
                                    *fresh192 = (n >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh193 = p;
                                    p = p.offset(1);
                                    *fresh193 = (n >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh194 = p;
                                    p = p.offset(1);
                                    *fresh194 = (n >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh195 = p;
                                    p = p.offset(1);
                                    *fresh195 = (n >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh196 = p;
                                    p = p.offset(1);
                                    *fresh196 = (n >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh197 = p;
                                    p = p.offset(1);
                                    *fresh197 = (n & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh198 = p;
                                    p = p.offset(1);
                                    *fresh198 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh199 = p;
                                    p = p.offset(1);
                                    *fresh199 = (n >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh200 = p;
                                    p = p.offset(1);
                                    *fresh200 = (n >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh201 = p;
                                    p = p.offset(1);
                                    *fresh201 = (n >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh202 = p;
                                    p = p.offset(1);
                                    *fresh202 = (n >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh203 = p;
                                    p = p.offset(1);
                                    *fresh203 = (n >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh204 = p;
                                    p = p.offset(1);
                                    *fresh204 = (n >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh205 = p;
                                    p = p.offset(1);
                                    *fresh205 = (n & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh206 = p;
                                p = p.offset(1);
                                *fresh206 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh207 = p;
                                p = p.offset(1);
                                *fresh207 = (n >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh208 = p;
                                p = p.offset(1);
                                *fresh208 = (n >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh209 = p;
                                p = p.offset(1);
                                *fresh209 = (n >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh210 = p;
                                p = p.offset(1);
                                *fresh210 = (n >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh211 = p;
                                p = p.offset(1);
                                *fresh211 = (n >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh212 = p;
                                p = p.offset(1);
                                *fresh212 = (n & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh213 = p;
                            p = p.offset(1);
                            *fresh213 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh214 = p;
                            p = p.offset(1);
                            *fresh214 = (n >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh215 = p;
                            p = p.offset(1);
                            *fresh215 = (n >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh216 = p;
                            p = p.offset(1);
                            *fresh216 = (n >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh217 = p;
                            p = p.offset(1);
                            *fresh217 = (n >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh218 = p;
                            p = p.offset(1);
                            *fresh218 = (n & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh219 = p;
                        p = p.offset(1);
                        *fresh219 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh220 = p;
                        p = p.offset(1);
                        *fresh220 = (n >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh221 = p;
                        p = p.offset(1);
                        *fresh221 = (n >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh222 = p;
                        p = p.offset(1);
                        *fresh222 = (n >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh223 = p;
                        p = p.offset(1);
                        *fresh223 = (n & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh224 = p;
                    p = p.offset(1);
                    *fresh224 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh225 = p;
                    p = p.offset(1);
                    *fresh225 = (n >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh226 = p;
                    p = p.offset(1);
                    *fresh226 = (n >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh227 = p;
                    p = p.offset(1);
                    *fresh227 = (n & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh228 = p;
                p = p.offset(1);
                *fresh228 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh229 = p;
                p = p.offset(1);
                *fresh229 = (n >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh230 = p;
                p = p.offset(1);
                *fresh230 = (n & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            }
        } else {
            let fresh231 = p;
            p = p.offset(1);
            *fresh231 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh232 = p;
            p = p.offset(1);
            *fresh232 = n as libc::c_uchar;
        }
    } else {
        let fresh233 = p;
        p = p.offset(1);
        *fresh233 = n as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, octets as *const libc::c_void, n);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_null(
    mut res: *mut mln_asn1_enresult_t,
) -> libc::c_int {
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = (*res).pool;
    buf = mln_alloc_m(pool, 2 as libc::c_int as mln_size_t) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            5 as libc::c_int
        })) as libc::c_uchar;
    *buf.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    if mln_asn1_encode_add_content(res, buf, 2 as libc::c_int as mln_u64_t)
        < 0 as libc::c_int
    {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as mln_u64_t as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_object_identifier(
    mut res: *mut mln_asn1_enresult_t,
    mut oid: mln_u8ptr_t,
    mut n: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = n.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if n > 127 as libc::c_int as libc::c_ulong {
        if n > 0xff as libc::c_int as libc::c_ulong {
            if n > 0xffff as libc::c_int as libc::c_ulong {
                if n > 0xffffff as libc::c_int as libc::c_ulong {
                    if n > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if n as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong {
                            if n as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if n as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            6 as libc::c_int
        })) as libc::c_uchar;
    if n > 127 as libc::c_int as libc::c_ulong {
        if n > 0xff as libc::c_int as libc::c_ulong {
            if n > 0xffff as libc::c_int as libc::c_ulong {
                if n > 0xffffff as libc::c_int as libc::c_ulong {
                    if n > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if n as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong {
                            if n as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if n as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh234 = p;
                                    p = p.offset(1);
                                    *fresh234 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh235 = p;
                                    p = p.offset(1);
                                    *fresh235 = (n >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh236 = p;
                                    p = p.offset(1);
                                    *fresh236 = (n >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh237 = p;
                                    p = p.offset(1);
                                    *fresh237 = (n >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh238 = p;
                                    p = p.offset(1);
                                    *fresh238 = (n >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh239 = p;
                                    p = p.offset(1);
                                    *fresh239 = (n >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh240 = p;
                                    p = p.offset(1);
                                    *fresh240 = (n >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh241 = p;
                                    p = p.offset(1);
                                    *fresh241 = (n >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh242 = p;
                                    p = p.offset(1);
                                    *fresh242 = (n & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh243 = p;
                                    p = p.offset(1);
                                    *fresh243 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh244 = p;
                                    p = p.offset(1);
                                    *fresh244 = (n >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh245 = p;
                                    p = p.offset(1);
                                    *fresh245 = (n >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh246 = p;
                                    p = p.offset(1);
                                    *fresh246 = (n >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh247 = p;
                                    p = p.offset(1);
                                    *fresh247 = (n >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh248 = p;
                                    p = p.offset(1);
                                    *fresh248 = (n >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh249 = p;
                                    p = p.offset(1);
                                    *fresh249 = (n >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh250 = p;
                                    p = p.offset(1);
                                    *fresh250 = (n & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh251 = p;
                                p = p.offset(1);
                                *fresh251 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh252 = p;
                                p = p.offset(1);
                                *fresh252 = (n >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh253 = p;
                                p = p.offset(1);
                                *fresh253 = (n >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh254 = p;
                                p = p.offset(1);
                                *fresh254 = (n >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh255 = p;
                                p = p.offset(1);
                                *fresh255 = (n >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh256 = p;
                                p = p.offset(1);
                                *fresh256 = (n >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh257 = p;
                                p = p.offset(1);
                                *fresh257 = (n & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh258 = p;
                            p = p.offset(1);
                            *fresh258 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh259 = p;
                            p = p.offset(1);
                            *fresh259 = (n >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh260 = p;
                            p = p.offset(1);
                            *fresh260 = (n >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh261 = p;
                            p = p.offset(1);
                            *fresh261 = (n >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh262 = p;
                            p = p.offset(1);
                            *fresh262 = (n >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh263 = p;
                            p = p.offset(1);
                            *fresh263 = (n & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh264 = p;
                        p = p.offset(1);
                        *fresh264 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh265 = p;
                        p = p.offset(1);
                        *fresh265 = (n >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh266 = p;
                        p = p.offset(1);
                        *fresh266 = (n >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh267 = p;
                        p = p.offset(1);
                        *fresh267 = (n >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh268 = p;
                        p = p.offset(1);
                        *fresh268 = (n & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh269 = p;
                    p = p.offset(1);
                    *fresh269 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh270 = p;
                    p = p.offset(1);
                    *fresh270 = (n >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh271 = p;
                    p = p.offset(1);
                    *fresh271 = (n >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh272 = p;
                    p = p.offset(1);
                    *fresh272 = (n & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh273 = p;
                p = p.offset(1);
                *fresh273 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh274 = p;
                p = p.offset(1);
                *fresh274 = (n >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh275 = p;
                p = p.offset(1);
                *fresh275 = (n & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            }
        } else {
            let fresh276 = p;
            p = p.offset(1);
            *fresh276 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh277 = p;
            p = p.offset(1);
            *fresh277 = n as libc::c_uchar;
        }
    } else {
        let fresh278 = p;
        p = p.offset(1);
        *fresh278 = n as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, oid as *const libc::c_void, n);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_utf8string(
    mut res: *mut mln_asn1_enresult_t,
    mut s: mln_u8ptr_t,
    mut slen: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = slen.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            12 as libc::c_int
        })) as libc::c_uchar;
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh279 = p;
                                    p = p.offset(1);
                                    *fresh279 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh280 = p;
                                    p = p.offset(1);
                                    *fresh280 = (slen >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh281 = p;
                                    p = p.offset(1);
                                    *fresh281 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh282 = p;
                                    p = p.offset(1);
                                    *fresh282 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh283 = p;
                                    p = p.offset(1);
                                    *fresh283 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh284 = p;
                                    p = p.offset(1);
                                    *fresh284 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh285 = p;
                                    p = p.offset(1);
                                    *fresh285 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh286 = p;
                                    p = p.offset(1);
                                    *fresh286 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh287 = p;
                                    p = p.offset(1);
                                    *fresh287 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh288 = p;
                                    p = p.offset(1);
                                    *fresh288 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh289 = p;
                                    p = p.offset(1);
                                    *fresh289 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh290 = p;
                                    p = p.offset(1);
                                    *fresh290 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh291 = p;
                                    p = p.offset(1);
                                    *fresh291 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh292 = p;
                                    p = p.offset(1);
                                    *fresh292 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh293 = p;
                                    p = p.offset(1);
                                    *fresh293 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh294 = p;
                                    p = p.offset(1);
                                    *fresh294 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh295 = p;
                                    p = p.offset(1);
                                    *fresh295 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh296 = p;
                                p = p.offset(1);
                                *fresh296 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh297 = p;
                                p = p.offset(1);
                                *fresh297 = (slen >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh298 = p;
                                p = p.offset(1);
                                *fresh298 = (slen >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh299 = p;
                                p = p.offset(1);
                                *fresh299 = (slen >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh300 = p;
                                p = p.offset(1);
                                *fresh300 = (slen >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh301 = p;
                                p = p.offset(1);
                                *fresh301 = (slen >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh302 = p;
                                p = p.offset(1);
                                *fresh302 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh303 = p;
                            p = p.offset(1);
                            *fresh303 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh304 = p;
                            p = p.offset(1);
                            *fresh304 = (slen >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh305 = p;
                            p = p.offset(1);
                            *fresh305 = (slen >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh306 = p;
                            p = p.offset(1);
                            *fresh306 = (slen >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh307 = p;
                            p = p.offset(1);
                            *fresh307 = (slen >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh308 = p;
                            p = p.offset(1);
                            *fresh308 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh309 = p;
                        p = p.offset(1);
                        *fresh309 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh310 = p;
                        p = p.offset(1);
                        *fresh310 = (slen >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh311 = p;
                        p = p.offset(1);
                        *fresh311 = (slen >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh312 = p;
                        p = p.offset(1);
                        *fresh312 = (slen >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh313 = p;
                        p = p.offset(1);
                        *fresh313 = (slen & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh314 = p;
                    p = p.offset(1);
                    *fresh314 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh315 = p;
                    p = p.offset(1);
                    *fresh315 = (slen >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh316 = p;
                    p = p.offset(1);
                    *fresh316 = (slen >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh317 = p;
                    p = p.offset(1);
                    *fresh317 = (slen & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh318 = p;
                p = p.offset(1);
                *fresh318 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh319 = p;
                p = p.offset(1);
                *fresh319 = (slen >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh320 = p;
                p = p.offset(1);
                *fresh320 = (slen & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh321 = p;
            p = p.offset(1);
            *fresh321 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh322 = p;
            p = p.offset(1);
            *fresh322 = slen as libc::c_uchar;
        }
    } else {
        let fresh323 = p;
        p = p.offset(1);
        *fresh323 = slen as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_printablestring(
    mut res: *mut mln_asn1_enresult_t,
    mut s: mln_s8ptr_t,
    mut slen: mln_u64_t,
) -> libc::c_int {
    let mut scan: mln_s8ptr_t = 0 as *mut libc::c_char;
    let mut end: mln_s8ptr_t = 0 as *mut libc::c_char;
    scan = s;
    end = s.offset(slen as isize);
    while scan < end {
        if !(*scan as libc::c_int >= 32 as libc::c_int
            && *scan as libc::c_int <= 126 as libc::c_int)
        {
            return 2 as libc::c_int;
        }
        scan = scan.offset(1);
        scan;
    }
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = slen.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            19 as libc::c_int
        })) as libc::c_uchar;
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh324 = p;
                                    p = p.offset(1);
                                    *fresh324 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh325 = p;
                                    p = p.offset(1);
                                    *fresh325 = (slen >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh326 = p;
                                    p = p.offset(1);
                                    *fresh326 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh327 = p;
                                    p = p.offset(1);
                                    *fresh327 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh328 = p;
                                    p = p.offset(1);
                                    *fresh328 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh329 = p;
                                    p = p.offset(1);
                                    *fresh329 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh330 = p;
                                    p = p.offset(1);
                                    *fresh330 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh331 = p;
                                    p = p.offset(1);
                                    *fresh331 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh332 = p;
                                    p = p.offset(1);
                                    *fresh332 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh333 = p;
                                    p = p.offset(1);
                                    *fresh333 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh334 = p;
                                    p = p.offset(1);
                                    *fresh334 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh335 = p;
                                    p = p.offset(1);
                                    *fresh335 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh336 = p;
                                    p = p.offset(1);
                                    *fresh336 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh337 = p;
                                    p = p.offset(1);
                                    *fresh337 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh338 = p;
                                    p = p.offset(1);
                                    *fresh338 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh339 = p;
                                    p = p.offset(1);
                                    *fresh339 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh340 = p;
                                    p = p.offset(1);
                                    *fresh340 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh341 = p;
                                p = p.offset(1);
                                *fresh341 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh342 = p;
                                p = p.offset(1);
                                *fresh342 = (slen >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh343 = p;
                                p = p.offset(1);
                                *fresh343 = (slen >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh344 = p;
                                p = p.offset(1);
                                *fresh344 = (slen >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh345 = p;
                                p = p.offset(1);
                                *fresh345 = (slen >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh346 = p;
                                p = p.offset(1);
                                *fresh346 = (slen >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh347 = p;
                                p = p.offset(1);
                                *fresh347 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh348 = p;
                            p = p.offset(1);
                            *fresh348 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh349 = p;
                            p = p.offset(1);
                            *fresh349 = (slen >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh350 = p;
                            p = p.offset(1);
                            *fresh350 = (slen >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh351 = p;
                            p = p.offset(1);
                            *fresh351 = (slen >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh352 = p;
                            p = p.offset(1);
                            *fresh352 = (slen >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh353 = p;
                            p = p.offset(1);
                            *fresh353 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh354 = p;
                        p = p.offset(1);
                        *fresh354 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh355 = p;
                        p = p.offset(1);
                        *fresh355 = (slen >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh356 = p;
                        p = p.offset(1);
                        *fresh356 = (slen >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh357 = p;
                        p = p.offset(1);
                        *fresh357 = (slen >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh358 = p;
                        p = p.offset(1);
                        *fresh358 = (slen & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh359 = p;
                    p = p.offset(1);
                    *fresh359 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh360 = p;
                    p = p.offset(1);
                    *fresh360 = (slen >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh361 = p;
                    p = p.offset(1);
                    *fresh361 = (slen >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh362 = p;
                    p = p.offset(1);
                    *fresh362 = (slen & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh363 = p;
                p = p.offset(1);
                *fresh363 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh364 = p;
                p = p.offset(1);
                *fresh364 = (slen >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh365 = p;
                p = p.offset(1);
                *fresh365 = (slen & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh366 = p;
            p = p.offset(1);
            *fresh366 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh367 = p;
            p = p.offset(1);
            *fresh367 = slen as libc::c_uchar;
        }
    } else {
        let fresh368 = p;
        p = p.offset(1);
        *fresh368 = slen as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_t61string(
    mut res: *mut mln_asn1_enresult_t,
    mut s: mln_u8ptr_t,
    mut slen: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = slen.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            20 as libc::c_int
        })) as libc::c_uchar;
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh369 = p;
                                    p = p.offset(1);
                                    *fresh369 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh370 = p;
                                    p = p.offset(1);
                                    *fresh370 = (slen >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh371 = p;
                                    p = p.offset(1);
                                    *fresh371 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh372 = p;
                                    p = p.offset(1);
                                    *fresh372 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh373 = p;
                                    p = p.offset(1);
                                    *fresh373 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh374 = p;
                                    p = p.offset(1);
                                    *fresh374 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh375 = p;
                                    p = p.offset(1);
                                    *fresh375 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh376 = p;
                                    p = p.offset(1);
                                    *fresh376 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh377 = p;
                                    p = p.offset(1);
                                    *fresh377 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh378 = p;
                                    p = p.offset(1);
                                    *fresh378 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh379 = p;
                                    p = p.offset(1);
                                    *fresh379 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh380 = p;
                                    p = p.offset(1);
                                    *fresh380 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh381 = p;
                                    p = p.offset(1);
                                    *fresh381 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh382 = p;
                                    p = p.offset(1);
                                    *fresh382 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh383 = p;
                                    p = p.offset(1);
                                    *fresh383 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh384 = p;
                                    p = p.offset(1);
                                    *fresh384 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh385 = p;
                                    p = p.offset(1);
                                    *fresh385 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh386 = p;
                                p = p.offset(1);
                                *fresh386 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh387 = p;
                                p = p.offset(1);
                                *fresh387 = (slen >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh388 = p;
                                p = p.offset(1);
                                *fresh388 = (slen >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh389 = p;
                                p = p.offset(1);
                                *fresh389 = (slen >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh390 = p;
                                p = p.offset(1);
                                *fresh390 = (slen >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh391 = p;
                                p = p.offset(1);
                                *fresh391 = (slen >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh392 = p;
                                p = p.offset(1);
                                *fresh392 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh393 = p;
                            p = p.offset(1);
                            *fresh393 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh394 = p;
                            p = p.offset(1);
                            *fresh394 = (slen >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh395 = p;
                            p = p.offset(1);
                            *fresh395 = (slen >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh396 = p;
                            p = p.offset(1);
                            *fresh396 = (slen >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh397 = p;
                            p = p.offset(1);
                            *fresh397 = (slen >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh398 = p;
                            p = p.offset(1);
                            *fresh398 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh399 = p;
                        p = p.offset(1);
                        *fresh399 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh400 = p;
                        p = p.offset(1);
                        *fresh400 = (slen >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh401 = p;
                        p = p.offset(1);
                        *fresh401 = (slen >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh402 = p;
                        p = p.offset(1);
                        *fresh402 = (slen >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh403 = p;
                        p = p.offset(1);
                        *fresh403 = (slen & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh404 = p;
                    p = p.offset(1);
                    *fresh404 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh405 = p;
                    p = p.offset(1);
                    *fresh405 = (slen >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh406 = p;
                    p = p.offset(1);
                    *fresh406 = (slen >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh407 = p;
                    p = p.offset(1);
                    *fresh407 = (slen & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh408 = p;
                p = p.offset(1);
                *fresh408 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh409 = p;
                p = p.offset(1);
                *fresh409 = (slen >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh410 = p;
                p = p.offset(1);
                *fresh410 = (slen & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh411 = p;
            p = p.offset(1);
            *fresh411 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh412 = p;
            p = p.offset(1);
            *fresh412 = slen as libc::c_uchar;
        }
    } else {
        let fresh413 = p;
        p = p.offset(1);
        *fresh413 = slen as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_ia5string(
    mut res: *mut mln_asn1_enresult_t,
    mut s: mln_u8ptr_t,
    mut slen: mln_u64_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    len = slen.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            22 as libc::c_int
        })) as libc::c_uchar;
    if slen > 127 as libc::c_int as libc::c_ulong {
        if slen > 0xff as libc::c_int as libc::c_ulong {
            if slen > 0xffff as libc::c_int as libc::c_ulong {
                if slen > 0xffffff as libc::c_int as libc::c_ulong {
                    if slen > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if slen as libc::c_ulonglong > 0xffffffffff as libc::c_ulonglong
                        {
                            if slen as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if slen as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh414 = p;
                                    p = p.offset(1);
                                    *fresh414 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh415 = p;
                                    p = p.offset(1);
                                    *fresh415 = (slen >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh416 = p;
                                    p = p.offset(1);
                                    *fresh416 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh417 = p;
                                    p = p.offset(1);
                                    *fresh417 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh418 = p;
                                    p = p.offset(1);
                                    *fresh418 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh419 = p;
                                    p = p.offset(1);
                                    *fresh419 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh420 = p;
                                    p = p.offset(1);
                                    *fresh420 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh421 = p;
                                    p = p.offset(1);
                                    *fresh421 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh422 = p;
                                    p = p.offset(1);
                                    *fresh422 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                } else {
                                    let fresh423 = p;
                                    p = p.offset(1);
                                    *fresh423 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh424 = p;
                                    p = p.offset(1);
                                    *fresh424 = (slen >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh425 = p;
                                    p = p.offset(1);
                                    *fresh425 = (slen >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh426 = p;
                                    p = p.offset(1);
                                    *fresh426 = (slen >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh427 = p;
                                    p = p.offset(1);
                                    *fresh427 = (slen >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh428 = p;
                                    p = p.offset(1);
                                    *fresh428 = (slen >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh429 = p;
                                    p = p.offset(1);
                                    *fresh429 = (slen >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh430 = p;
                                    p = p.offset(1);
                                    *fresh430 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                }
                            } else {
                                let fresh431 = p;
                                p = p.offset(1);
                                *fresh431 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh432 = p;
                                p = p.offset(1);
                                *fresh432 = (slen >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh433 = p;
                                p = p.offset(1);
                                *fresh433 = (slen >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh434 = p;
                                p = p.offset(1);
                                *fresh434 = (slen >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh435 = p;
                                p = p.offset(1);
                                *fresh435 = (slen >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh436 = p;
                                p = p.offset(1);
                                *fresh436 = (slen >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh437 = p;
                                p = p.offset(1);
                                *fresh437 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                        } else {
                            let fresh438 = p;
                            p = p.offset(1);
                            *fresh438 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh439 = p;
                            p = p.offset(1);
                            *fresh439 = (slen >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh440 = p;
                            p = p.offset(1);
                            *fresh440 = (slen >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh441 = p;
                            p = p.offset(1);
                            *fresh441 = (slen >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh442 = p;
                            p = p.offset(1);
                            *fresh442 = (slen >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh443 = p;
                            p = p.offset(1);
                            *fresh443 = (slen & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        }
                    } else {
                        let fresh444 = p;
                        p = p.offset(1);
                        *fresh444 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh445 = p;
                        p = p.offset(1);
                        *fresh445 = (slen >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh446 = p;
                        p = p.offset(1);
                        *fresh446 = (slen >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh447 = p;
                        p = p.offset(1);
                        *fresh447 = (slen >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh448 = p;
                        p = p.offset(1);
                        *fresh448 = (slen & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh449 = p;
                    p = p.offset(1);
                    *fresh449 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh450 = p;
                    p = p.offset(1);
                    *fresh450 = (slen >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh451 = p;
                    p = p.offset(1);
                    *fresh451 = (slen >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh452 = p;
                    p = p.offset(1);
                    *fresh452 = (slen & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh453 = p;
                p = p.offset(1);
                *fresh453 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh454 = p;
                p = p.offset(1);
                *fresh454 = (slen >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh455 = p;
                p = p.offset(1);
                *fresh455 = (slen & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh456 = p;
            p = p.offset(1);
            *fresh456 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh457 = p;
            p = p.offset(1);
            *fresh457 = slen as libc::c_uchar;
        }
    } else {
        let fresh458 = p;
        p = p.offset(1);
        *fresh458 = slen as libc::c_uchar;
    }
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_utctime(
    mut res: *mut mln_asn1_enresult_t,
    mut time: time_t,
) -> libc::c_int {
    let mut uc: utctime = utctime {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        week: 0,
    };
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    mln_time2utc(time, &mut uc);
    if uc.year > 2000 as libc::c_int as libc::c_long {
        uc.year -= 2000 as libc::c_int as libc::c_long;
    } else {
        uc.year -= 1900 as libc::c_int as libc::c_long;
    }
    len = (13 as libc::c_int + 1 as libc::c_int) as mln_u64_t;
    if 13 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 13 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 13 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 13 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 13 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            23 as libc::c_int
        })) as libc::c_uchar;
    if 13 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 13 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 13 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 13 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 13 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 13 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh459 = p;
                                    p = p.offset(1);
                                    *fresh459 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh460 = p;
                                    p = p.offset(1);
                                    *fresh460 = (13 as libc::c_int as mln_u64_t
                                        >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh461 = p;
                                    p = p.offset(1);
                                    *fresh461 = (13 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh462 = p;
                                    p = p.offset(1);
                                    *fresh462 = (13 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh463 = p;
                                    p = p.offset(1);
                                    *fresh463 = (13 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh464 = p;
                                    p = p.offset(1);
                                    *fresh464 = (13 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh465 = p;
                                    p = p.offset(1);
                                    *fresh465 = (13 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh466 = p;
                                    p = p.offset(1);
                                    *fresh466 = (13 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh467 = p;
                                    p = p.offset(1);
                                    *fresh467 = (13 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh468 = p;
                                    p = p.offset(1);
                                    *fresh468 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh469 = p;
                                    p = p.offset(1);
                                    *fresh469 = (13 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh470 = p;
                                    p = p.offset(1);
                                    *fresh470 = (13 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh471 = p;
                                    p = p.offset(1);
                                    *fresh471 = (13 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh472 = p;
                                    p = p.offset(1);
                                    *fresh472 = (13 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh473 = p;
                                    p = p.offset(1);
                                    *fresh473 = (13 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh474 = p;
                                    p = p.offset(1);
                                    *fresh474 = (13 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh475 = p;
                                    p = p.offset(1);
                                    *fresh475 = (13 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh476 = p;
                                p = p.offset(1);
                                *fresh476 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh477 = p;
                                p = p.offset(1);
                                *fresh477 = (13 as libc::c_int as mln_u64_t
                                    >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh478 = p;
                                p = p.offset(1);
                                *fresh478 = (13 as libc::c_int as mln_u64_t
                                    >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh479 = p;
                                p = p.offset(1);
                                *fresh479 = (13 as libc::c_int as mln_u64_t
                                    >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh480 = p;
                                p = p.offset(1);
                                *fresh480 = (13 as libc::c_int as mln_u64_t
                                    >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh481 = p;
                                p = p.offset(1);
                                *fresh481 = (13 as libc::c_int as mln_u64_t
                                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh482 = p;
                                p = p.offset(1);
                                *fresh482 = (13 as libc::c_int as mln_u64_t
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh483 = p;
                            p = p.offset(1);
                            *fresh483 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh484 = p;
                            p = p.offset(1);
                            *fresh484 = (13 as libc::c_int as mln_u64_t
                                >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh485 = p;
                            p = p.offset(1);
                            *fresh485 = (13 as libc::c_int as mln_u64_t
                                >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh486 = p;
                            p = p.offset(1);
                            *fresh486 = (13 as libc::c_int as mln_u64_t
                                >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh487 = p;
                            p = p.offset(1);
                            *fresh487 = (13 as libc::c_int as mln_u64_t
                                >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh488 = p;
                            p = p.offset(1);
                            *fresh488 = (13 as libc::c_int as mln_u64_t
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh489 = p;
                        p = p.offset(1);
                        *fresh489 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh490 = p;
                        p = p.offset(1);
                        *fresh490 = (13 as libc::c_int as mln_u64_t >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh491 = p;
                        p = p.offset(1);
                        *fresh491 = (13 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh492 = p;
                        p = p.offset(1);
                        *fresh492 = (13 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh493 = p;
                        p = p.offset(1);
                        *fresh493 = (13 as libc::c_int as mln_u64_t
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    }
                } else {
                    let fresh494 = p;
                    p = p.offset(1);
                    *fresh494 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh495 = p;
                    p = p.offset(1);
                    *fresh495 = (13 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh496 = p;
                    p = p.offset(1);
                    *fresh496 = (13 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh497 = p;
                    p = p.offset(1);
                    *fresh497 = (13 as libc::c_int as mln_u64_t
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                }
            } else {
                let fresh498 = p;
                p = p.offset(1);
                *fresh498 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh499 = p;
                p = p.offset(1);
                *fresh499 = (13 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh500 = p;
                p = p.offset(1);
                *fresh500 = (13 as libc::c_int as mln_u64_t
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            }
        } else {
            let fresh501 = p;
            p = p.offset(1);
            *fresh501 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh502 = p;
            p = p.offset(1);
            *fresh502 = 13 as libc::c_int as libc::c_uchar;
        }
    } else {
        let fresh503 = p;
        p = p.offset(1);
        *fresh503 = 13 as libc::c_int as libc::c_uchar;
    }
    let fresh504 = p;
    p = p.offset(1);
    *fresh504 = (uc.year / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh505 = p;
    p = p.offset(1);
    *fresh505 = (uc.year % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh506 = p;
    p = p.offset(1);
    *fresh506 = (uc.month / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh507 = p;
    p = p.offset(1);
    *fresh507 = (uc.month % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh508 = p;
    p = p.offset(1);
    *fresh508 = (uc.day / 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh509 = p;
    p = p.offset(1);
    *fresh509 = (uc.day % 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh510 = p;
    p = p.offset(1);
    *fresh510 = (uc.hour / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh511 = p;
    p = p.offset(1);
    *fresh511 = (uc.hour % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh512 = p;
    p = p.offset(1);
    *fresh512 = (uc.minute / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh513 = p;
    p = p.offset(1);
    *fresh513 = (uc.minute % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh514 = p;
    p = p.offset(1);
    *fresh514 = (uc.second / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh515 = p;
    p = p.offset(1);
    *fresh515 = (uc.second % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    *p = 'Z' as i32 as libc::c_uchar;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_generalized_time(
    mut res: *mut mln_asn1_enresult_t,
    mut time: time_t,
) -> libc::c_int {
    let mut uc: utctime = utctime {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        week: 0,
    };
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    mln_time2utc(time, &mut uc);
    len = (15 as libc::c_int + 1 as libc::c_int) as mln_u64_t;
    if 15 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 15 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 15 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 15 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 15 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    pool = (*res).pool;
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    p = buf.offset(1 as libc::c_int as isize);
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (if (*res).is_struct() as libc::c_int != 0 {
            (1 as libc::c_int) << 5 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            24 as libc::c_int
        })) as libc::c_uchar;
    if 15 as libc::c_int as mln_u64_t > 127 as libc::c_int as libc::c_ulong {
        if 15 as libc::c_int as mln_u64_t > 0xff as libc::c_int as libc::c_ulong {
            if 15 as libc::c_int as mln_u64_t > 0xffff as libc::c_int as libc::c_ulong {
                if 15 as libc::c_int as mln_u64_t
                    > 0xffffff as libc::c_int as libc::c_ulong
                {
                    if 15 as libc::c_int as mln_u64_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if 15 as libc::c_int as mln_u64_t as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh516 = p;
                                    p = p.offset(1);
                                    *fresh516 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh517 = p;
                                    p = p.offset(1);
                                    *fresh517 = (15 as libc::c_int as mln_u64_t
                                        >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh518 = p;
                                    p = p.offset(1);
                                    *fresh518 = (15 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh519 = p;
                                    p = p.offset(1);
                                    *fresh519 = (15 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh520 = p;
                                    p = p.offset(1);
                                    *fresh520 = (15 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh521 = p;
                                    p = p.offset(1);
                                    *fresh521 = (15 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh522 = p;
                                    p = p.offset(1);
                                    *fresh522 = (15 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh523 = p;
                                    p = p.offset(1);
                                    *fresh523 = (15 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh524 = p;
                                    p = p.offset(1);
                                    *fresh524 = (15 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh525 = p;
                                    p = p.offset(1);
                                    *fresh525 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh526 = p;
                                    p = p.offset(1);
                                    *fresh526 = (15 as libc::c_int as mln_u64_t
                                        >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh527 = p;
                                    p = p.offset(1);
                                    *fresh527 = (15 as libc::c_int as mln_u64_t
                                        >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh528 = p;
                                    p = p.offset(1);
                                    *fresh528 = (15 as libc::c_int as mln_u64_t
                                        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh529 = p;
                                    p = p.offset(1);
                                    *fresh529 = (15 as libc::c_int as mln_u64_t
                                        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh530 = p;
                                    p = p.offset(1);
                                    *fresh530 = (15 as libc::c_int as mln_u64_t
                                        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh531 = p;
                                    p = p.offset(1);
                                    *fresh531 = (15 as libc::c_int as mln_u64_t
                                        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                        as libc::c_uchar;
                                    let fresh532 = p;
                                    p = p.offset(1);
                                    *fresh532 = (15 as libc::c_int as mln_u64_t
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh533 = p;
                                p = p.offset(1);
                                *fresh533 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh534 = p;
                                p = p.offset(1);
                                *fresh534 = (15 as libc::c_int as mln_u64_t
                                    >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh535 = p;
                                p = p.offset(1);
                                *fresh535 = (15 as libc::c_int as mln_u64_t
                                    >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh536 = p;
                                p = p.offset(1);
                                *fresh536 = (15 as libc::c_int as mln_u64_t
                                    >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh537 = p;
                                p = p.offset(1);
                                *fresh537 = (15 as libc::c_int as mln_u64_t
                                    >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh538 = p;
                                p = p.offset(1);
                                *fresh538 = (15 as libc::c_int as mln_u64_t
                                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                                let fresh539 = p;
                                p = p.offset(1);
                                *fresh539 = (15 as libc::c_int as mln_u64_t
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh540 = p;
                            p = p.offset(1);
                            *fresh540 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh541 = p;
                            p = p.offset(1);
                            *fresh541 = (15 as libc::c_int as mln_u64_t
                                >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh542 = p;
                            p = p.offset(1);
                            *fresh542 = (15 as libc::c_int as mln_u64_t
                                >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh543 = p;
                            p = p.offset(1);
                            *fresh543 = (15 as libc::c_int as mln_u64_t
                                >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh544 = p;
                            p = p.offset(1);
                            *fresh544 = (15 as libc::c_int as mln_u64_t
                                >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            let fresh545 = p;
                            p = p.offset(1);
                            *fresh545 = (15 as libc::c_int as mln_u64_t
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh546 = p;
                        p = p.offset(1);
                        *fresh546 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh547 = p;
                        p = p.offset(1);
                        *fresh547 = (15 as libc::c_int as mln_u64_t >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh548 = p;
                        p = p.offset(1);
                        *fresh548 = (15 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh549 = p;
                        p = p.offset(1);
                        *fresh549 = (15 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh550 = p;
                        p = p.offset(1);
                        *fresh550 = (15 as libc::c_int as mln_u64_t
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    }
                } else {
                    let fresh551 = p;
                    p = p.offset(1);
                    *fresh551 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh552 = p;
                    p = p.offset(1);
                    *fresh552 = (15 as libc::c_int as mln_u64_t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh553 = p;
                    p = p.offset(1);
                    *fresh553 = (15 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh554 = p;
                    p = p.offset(1);
                    *fresh554 = (15 as libc::c_int as mln_u64_t
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                }
            } else {
                let fresh555 = p;
                p = p.offset(1);
                *fresh555 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh556 = p;
                p = p.offset(1);
                *fresh556 = (15 as libc::c_int as mln_u64_t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh557 = p;
                p = p.offset(1);
                *fresh557 = (15 as libc::c_int as mln_u64_t
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            }
        } else {
            let fresh558 = p;
            p = p.offset(1);
            *fresh558 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh559 = p;
            p = p.offset(1);
            *fresh559 = 15 as libc::c_int as libc::c_uchar;
        }
    } else {
        let fresh560 = p;
        p = p.offset(1);
        *fresh560 = 15 as libc::c_int as libc::c_uchar;
    }
    let fresh561 = p;
    p = p.offset(1);
    *fresh561 = (uc.year / 1000 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh562 = p;
    p = p.offset(1);
    *fresh562 = (uc.year % 1000 as libc::c_int as libc::c_long
        / 100 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh563 = p;
    p = p.offset(1);
    *fresh563 = (uc.year % 100 as libc::c_int as libc::c_long
        / 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh564 = p;
    p = p.offset(1);
    *fresh564 = (uc.year % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh565 = p;
    p = p.offset(1);
    *fresh565 = (uc.month / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh566 = p;
    p = p.offset(1);
    *fresh566 = (uc.month % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh567 = p;
    p = p.offset(1);
    *fresh567 = (uc.day / 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh568 = p;
    p = p.offset(1);
    *fresh568 = (uc.day % 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
        as libc::c_uchar;
    let fresh569 = p;
    p = p.offset(1);
    *fresh569 = (uc.hour / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh570 = p;
    p = p.offset(1);
    *fresh570 = (uc.hour % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh571 = p;
    p = p.offset(1);
    *fresh571 = (uc.minute / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh572 = p;
    p = p.offset(1);
    *fresh572 = (uc.minute % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh573 = p;
    p = p.offset(1);
    *fresh573 = (uc.second / 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    let fresh574 = p;
    p = p.offset(1);
    *fresh574 = (uc.second % 10 as libc::c_int as libc::c_long
        + '0' as i32 as libc::c_long) as libc::c_uchar;
    *p = 'Z' as i32 as libc::c_uchar;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res)
        .size = ((*res).size as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_set(
    mut res: *mut mln_asn1_enresult_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = (*res).pool;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    len = ((*res).size).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            17 as libc::c_int
        })) as libc::c_uchar;
    p = buf.offset(1 as libc::c_int as isize);
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh575 = p;
                                    p = p.offset(1);
                                    *fresh575 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh576 = p;
                                    p = p.offset(1);
                                    *fresh576 = ((*res).size >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh577 = p;
                                    p = p.offset(1);
                                    *fresh577 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh578 = p;
                                    p = p.offset(1);
                                    *fresh578 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh579 = p;
                                    p = p.offset(1);
                                    *fresh579 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh580 = p;
                                    p = p.offset(1);
                                    *fresh580 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh581 = p;
                                    p = p.offset(1);
                                    *fresh581 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh582 = p;
                                    p = p.offset(1);
                                    *fresh582 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh583 = p;
                                    p = p.offset(1);
                                    *fresh583 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh584 = p;
                                    p = p.offset(1);
                                    *fresh584 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh585 = p;
                                    p = p.offset(1);
                                    *fresh585 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh586 = p;
                                    p = p.offset(1);
                                    *fresh586 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh587 = p;
                                    p = p.offset(1);
                                    *fresh587 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh588 = p;
                                    p = p.offset(1);
                                    *fresh588 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh589 = p;
                                    p = p.offset(1);
                                    *fresh589 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh590 = p;
                                    p = p.offset(1);
                                    *fresh590 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh591 = p;
                                    p = p.offset(1);
                                    *fresh591 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh592 = p;
                                p = p.offset(1);
                                *fresh592 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh593 = p;
                                p = p.offset(1);
                                *fresh593 = ((*res).size >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh594 = p;
                                p = p.offset(1);
                                *fresh594 = ((*res).size >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh595 = p;
                                p = p.offset(1);
                                *fresh595 = ((*res).size >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh596 = p;
                                p = p.offset(1);
                                *fresh596 = ((*res).size >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh597 = p;
                                p = p.offset(1);
                                *fresh597 = ((*res).size >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh598 = p;
                                p = p.offset(1);
                                *fresh598 = ((*res).size
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh599 = p;
                            p = p.offset(1);
                            *fresh599 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh600 = p;
                            p = p.offset(1);
                            *fresh600 = ((*res).size >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh601 = p;
                            p = p.offset(1);
                            *fresh601 = ((*res).size >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh602 = p;
                            p = p.offset(1);
                            *fresh602 = ((*res).size >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh603 = p;
                            p = p.offset(1);
                            *fresh603 = ((*res).size >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh604 = p;
                            p = p.offset(1);
                            *fresh604 = ((*res).size
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh605 = p;
                        p = p.offset(1);
                        *fresh605 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh606 = p;
                        p = p.offset(1);
                        *fresh606 = ((*res).size >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh607 = p;
                        p = p.offset(1);
                        *fresh607 = ((*res).size >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh608 = p;
                        p = p.offset(1);
                        *fresh608 = ((*res).size >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh609 = p;
                        p = p.offset(1);
                        *fresh609 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh610 = p;
                    p = p.offset(1);
                    *fresh610 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh611 = p;
                    p = p.offset(1);
                    *fresh611 = ((*res).size >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh612 = p;
                    p = p.offset(1);
                    *fresh612 = ((*res).size >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh613 = p;
                    p = p.offset(1);
                    *fresh613 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh614 = p;
                p = p.offset(1);
                *fresh614 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh615 = p;
                p = p.offset(1);
                *fresh615 = ((*res).size >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh616 = p;
                p = p.offset(1);
                *fresh616 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh617 = p;
            p = p.offset(1);
            *fresh617 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh618 = p;
            p = p.offset(1);
            *fresh618 = (*res).size as libc::c_uchar;
        }
    } else {
        let fresh619 = p;
        p = p.offset(1);
        *fresh619 = (*res).size as libc::c_uchar;
    }
    qsort(
        (*res).contents as *mut libc::c_void,
        (*res).pos as size_t,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
        Some(
            mln_encode_set_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    send = ((*res).contents).offset((*res).pos as isize);
    s = (*res).contents;
    while s < send {
        memcpy(p as *mut libc::c_void, (*s).data as *const libc::c_void, (*s).len);
        p = p.offset((*s).len as isize);
        mln_alloc_free((*s).data as *mut libc::c_void);
        s = s.offset(1);
        s;
    }
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res).size = len;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_encode_set_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *mut mln_string_t = data1 as *mut mln_string_t;
    let mut s2: *mut mln_string_t = data2 as *mut mln_string_t;
    if *((*s1).data).offset(0 as libc::c_int as isize) as libc::c_int
        > *((*s2).data).offset(0 as libc::c_int as isize) as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *((*s1).data).offset(0 as libc::c_int as isize) as libc::c_int
        == *((*s2).data).offset(0 as libc::c_int as isize) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_setof(
    mut res: *mut mln_asn1_enresult_t,
) -> libc::c_int {
    let mut len: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut buf: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = (*res).pool;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    len = ((*res).size).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(9 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                } else {
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                        as mln_u64_t as mln_u64_t;
                                }
                            } else {
                                len = (len as libc::c_ulong)
                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                    as mln_u64_t as mln_u64_t;
                            }
                        } else {
                            len = (len as libc::c_ulong)
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as mln_u64_t as mln_u64_t;
                        }
                    } else {
                        len = (len as libc::c_ulong)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong) as mln_u64_t
                            as mln_u64_t;
                    }
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong) as mln_u64_t
                        as mln_u64_t;
                }
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as mln_u64_t
                    as mln_u64_t;
            }
        } else {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        }
    } else {
        len = len.wrapping_add(1);
        len;
    }
    buf = mln_alloc_m(pool, len) as mln_u8ptr_t;
    if buf.is_null() {
        return 2 as libc::c_int;
    }
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (((*res)._class() as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int
        | (if (*res).ident() as libc::c_int != 0xfffffff as libc::c_int {
            (*res).ident() as libc::c_int & 0x1f as libc::c_int
        } else {
            17 as libc::c_int
        })) as libc::c_uchar;
    p = buf.offset(1 as libc::c_int as isize);
    if (*res).size > 127 as libc::c_int as libc::c_ulong {
        if (*res).size > 0xff as libc::c_int as libc::c_ulong {
            if (*res).size > 0xffff as libc::c_int as libc::c_ulong {
                if (*res).size > 0xffffff as libc::c_int as libc::c_ulong {
                    if (*res).size > 0xffffffff as libc::c_uint as libc::c_ulong {
                        if (*res).size as libc::c_ulonglong
                            > 0xffffffffff as libc::c_ulonglong
                        {
                            if (*res).size as libc::c_ulonglong
                                > 0xffffffffffff as libc::c_ulonglong
                            {
                                if (*res).size as libc::c_ulonglong
                                    > 0xffffffffffffff as libc::c_ulonglong
                                {
                                    let fresh620 = p;
                                    p = p.offset(1);
                                    *fresh620 = (0x80 as libc::c_int | 8 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh621 = p;
                                    p = p.offset(1);
                                    *fresh621 = ((*res).size >> 56 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh622 = p;
                                    p = p.offset(1);
                                    *fresh622 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh623 = p;
                                    p = p.offset(1);
                                    *fresh623 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh624 = p;
                                    p = p.offset(1);
                                    *fresh624 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh625 = p;
                                    p = p.offset(1);
                                    *fresh625 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh626 = p;
                                    p = p.offset(1);
                                    *fresh626 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh627 = p;
                                    p = p.offset(1);
                                    *fresh627 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh628 = p;
                                    p = p.offset(1);
                                    *fresh628 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                } else {
                                    let fresh629 = p;
                                    p = p.offset(1);
                                    *fresh629 = (0x80 as libc::c_int | 7 as libc::c_int)
                                        as libc::c_uchar;
                                    let fresh630 = p;
                                    p = p.offset(1);
                                    *fresh630 = ((*res).size >> 48 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh631 = p;
                                    p = p.offset(1);
                                    *fresh631 = ((*res).size >> 40 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh632 = p;
                                    p = p.offset(1);
                                    *fresh632 = ((*res).size >> 32 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh633 = p;
                                    p = p.offset(1);
                                    *fresh633 = ((*res).size >> 24 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh634 = p;
                                    p = p.offset(1);
                                    *fresh634 = ((*res).size >> 16 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh635 = p;
                                    p = p.offset(1);
                                    *fresh635 = ((*res).size >> 8 as libc::c_int
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                    let fresh636 = p;
                                    p = p.offset(1);
                                    *fresh636 = ((*res).size
                                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                }
                            } else {
                                let fresh637 = p;
                                p = p.offset(1);
                                *fresh637 = (0x80 as libc::c_int | 6 as libc::c_int)
                                    as libc::c_uchar;
                                let fresh638 = p;
                                p = p.offset(1);
                                *fresh638 = ((*res).size >> 40 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh639 = p;
                                p = p.offset(1);
                                *fresh639 = ((*res).size >> 32 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh640 = p;
                                p = p.offset(1);
                                *fresh640 = ((*res).size >> 24 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh641 = p;
                                p = p.offset(1);
                                *fresh641 = ((*res).size >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh642 = p;
                                p = p.offset(1);
                                *fresh642 = ((*res).size >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                let fresh643 = p;
                                p = p.offset(1);
                                *fresh643 = ((*res).size
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            }
                        } else {
                            let fresh644 = p;
                            p = p.offset(1);
                            *fresh644 = (0x80 as libc::c_int | 5 as libc::c_int)
                                as libc::c_uchar;
                            let fresh645 = p;
                            p = p.offset(1);
                            *fresh645 = ((*res).size >> 32 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh646 = p;
                            p = p.offset(1);
                            *fresh646 = ((*res).size >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh647 = p;
                            p = p.offset(1);
                            *fresh647 = ((*res).size >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh648 = p;
                            p = p.offset(1);
                            *fresh648 = ((*res).size >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            let fresh649 = p;
                            p = p.offset(1);
                            *fresh649 = ((*res).size
                                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        }
                    } else {
                        let fresh650 = p;
                        p = p.offset(1);
                        *fresh650 = (0x80 as libc::c_int | 4 as libc::c_int)
                            as libc::c_uchar;
                        let fresh651 = p;
                        p = p.offset(1);
                        *fresh651 = ((*res).size >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh652 = p;
                        p = p.offset(1);
                        *fresh652 = ((*res).size >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh653 = p;
                        p = p.offset(1);
                        *fresh653 = ((*res).size >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                        let fresh654 = p;
                        p = p.offset(1);
                        *fresh654 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                            as libc::c_uchar;
                    }
                } else {
                    let fresh655 = p;
                    p = p.offset(1);
                    *fresh655 = (0x80 as libc::c_int | 3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh656 = p;
                    p = p.offset(1);
                    *fresh656 = ((*res).size >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh657 = p;
                    p = p.offset(1);
                    *fresh657 = ((*res).size >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                    let fresh658 = p;
                    p = p.offset(1);
                    *fresh658 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_uchar;
                }
            } else {
                let fresh659 = p;
                p = p.offset(1);
                *fresh659 = (0x80 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                let fresh660 = p;
                p = p.offset(1);
                *fresh660 = ((*res).size >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                let fresh661 = p;
                p = p.offset(1);
                *fresh661 = ((*res).size & 0xff as libc::c_int as libc::c_ulong)
                    as libc::c_uchar;
            }
        } else {
            let fresh662 = p;
            p = p.offset(1);
            *fresh662 = (0x80 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            let fresh663 = p;
            p = p.offset(1);
            *fresh663 = (*res).size as libc::c_uchar;
        }
    } else {
        let fresh664 = p;
        p = p.offset(1);
        *fresh664 = (*res).size as libc::c_uchar;
    }
    qsort(
        (*res).contents as *mut libc::c_void,
        (*res).pos as size_t,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
        Some(
            mln_encode_setof_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    send = ((*res).contents).offset((*res).pos as isize);
    s = (*res).contents;
    while s < send {
        memcpy(p as *mut libc::c_void, (*s).data as *const libc::c_void, (*s).len);
        p = p.offset((*s).len as isize);
        mln_alloc_free((*s).data as *mut libc::c_void);
        s = s.offset(1);
        s;
    }
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    if mln_asn1_encode_add_content(res, buf, len) < 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return 2 as libc::c_int;
    }
    (*res).size = len;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_encode_setof_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *mut mln_string_t = data1 as *mut mln_string_t;
    let mut s2: *mut mln_string_t = data2 as *mut mln_string_t;
    let mut len: mln_size_t = if (*s1).len > (*s2).len { (*s2).len } else { (*s1).len };
    let mut ret: libc::c_int = memcmp(
        (*s1).data as *const libc::c_void,
        (*s2).data as *const libc::c_void,
        len,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    if (*s1).len > (*s2).len {
        return 1 as libc::c_int;
    }
    if (*s1).len < (*s2).len {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_merge(
    mut dest: *mut mln_asn1_enresult_t,
    mut src: *mut mln_asn1_enresult_t,
) -> libc::c_int {
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut pool: *mut mln_alloc_t = (*dest).pool;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    send = ((*src).contents).offset((*src).pos as isize);
    s = (*src).contents;
    while s < send {
        buf = mln_alloc_m(pool, (*s).len) as mln_u8ptr_t;
        if buf.is_null() {
            return 2 as libc::c_int;
        }
        memcpy(buf as *mut libc::c_void, (*s).data as *const libc::c_void, (*s).len);
        if mln_asn1_encode_add_content(dest, buf, (*s).len) < 0 as libc::c_int {
            return 2 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    (*dest)
        .size = ((*dest).size as libc::c_ulong).wrapping_add((*src).size) as mln_u64_t
        as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_trans_chain_once(
    mut res: *mut mln_asn1_enresult_t,
    mut head: *mut *mut mln_chain_t,
    mut tail: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut pool: *mut mln_alloc_t = (*res).pool;
    send = ((*res).contents).offset((*res).pos as isize);
    s = (*res).contents;
    while s < send {
        c = mln_chain_new(pool);
        if c.is_null() {
            return 2 as libc::c_int;
        }
        (*c).buf = mln_buf_new(pool);
        b = (*c).buf;
        if b.is_null() {
            mln_chain_pool_release(c);
            return 2 as libc::c_int;
        }
        (*b).start = (*s).data;
        (*b).pos = (*b).start;
        (*b).left_pos = (*b).pos;
        (*b).last = ((*s).data).offset((*s).len as isize);
        (*b).end = (*b).last;
        (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
        (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
        if (*head).is_null() {
            *tail = c;
            *head = *tail;
        } else {
            (**tail).next = c;
            *tail = c;
        }
        s = s.offset(1);
        s;
    }
    (*res).pos = 0 as libc::c_int as mln_u32_t;
    (*res).size = 0 as libc::c_int as mln_u64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_enresult_get_content(
    mut res: *mut mln_asn1_enresult_t,
    mut index: mln_u32_t,
    mut buf: *mut mln_u8ptr_t,
    mut len: *mut mln_u64_t,
) -> libc::c_int {
    if index >= (*res).pos {
        return 2 as libc::c_int;
    }
    let mut s: *mut mln_string_t = &mut *((*res).contents).offset(index as isize)
        as *mut mln_string_t;
    *buf = (*s).data;
    *len = (*s).len;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_asn1_encode_implicit(
    mut res: *mut mln_asn1_enresult_t,
    mut ident: mln_u32_t,
    mut index: mln_u32_t,
) -> libc::c_int {
    if index >= (*res).pos {
        return 2 as libc::c_int;
    }
    let mut s: *mut mln_string_t = &mut *((*res).contents).offset(index as isize)
        as *mut mln_string_t;
    let ref mut fresh665 = *((*s).data).offset(0 as libc::c_int as isize);
    *fresh665 = (*fresh665 as libc::c_int & 0x20 as libc::c_int) as libc::c_uchar;
    let ref mut fresh666 = *((*s).data).offset(0 as libc::c_int as isize);
    *fresh666 = (*fresh666 as libc::c_uint
        | (((2 as libc::c_int) << 6 as libc::c_int) as libc::c_uint | ident))
        as libc::c_uchar;
    return 0 as libc::c_int;
}
