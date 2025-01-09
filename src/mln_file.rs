use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mln_rbtree_node_free(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_node_new(
        t: *mut mln_rbtree_t,
        data: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_delete(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_search(
        t: *mut mln_rbtree_t,
        key: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_insert(t: *mut mln_rbtree_t, node: *mut mln_rbtree_node_t);
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mln_string_pool_new(
        pool: *mut mln_alloc_t,
        s: *const libc::c_char,
    ) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mln_path_tmpfile() -> *mut libc::c_char;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[inline]
unsafe extern "C" fn mln_alloc_init(mut parent: *mut mln_alloc_t) -> *mut mln_alloc_t {
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    if !parent.is_null() {
        if !((*parent).mem).is_null() {
            if ((*parent).lock).expect("non-null function pointer")((*parent).locker)
                != 0 as libc::c_int
            {
                return 0 as *mut mln_alloc_t;
            }
        }
        pool = mln_alloc_m(
            parent,
            ::core::mem::size_of::<mln_alloc_t>() as libc::c_ulong,
        ) as *mut mln_alloc_t;
        if !((*parent).mem).is_null() {
            ((*parent).unlock).expect("non-null function pointer")((*parent).locker);
        }
    } else {
        pool = malloc(::core::mem::size_of::<mln_alloc_t>() as libc::c_ulong)
            as *mut mln_alloc_t;
    }
    if pool.is_null() {
        return pool;
    }
    mln_alloc_mgr_table_init(((*pool).mgr_tbl).as_mut_ptr());
    (*pool).parent = parent;
    (*pool).large_used_tail = 0 as *mut mln_alloc_chunk_t;
    (*pool).large_used_head = (*pool).large_used_tail;
    (*pool).shm_tail = 0 as *mut mln_alloc_shm_t;
    (*pool).shm_head = (*pool).shm_tail;
    (*pool).mem = 0 as *mut libc::c_void;
    (*pool).shm_size = 0 as libc::c_int as mln_size_t;
    (*pool).locker = 0 as *mut libc::c_void;
    (*pool).lock = None;
    (*pool).unlock = None;
    return pool;
}
#[inline]
unsafe extern "C" fn mln_alloc_mgr_table_init(mut tbl: *mut mln_alloc_mgr_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut blk_size: mln_size_t = 0;
    let mut am: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
    let mut amprev: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
    i = 0 as libc::c_int;
    while i
        < 18 as libc::c_int * 2 as libc::c_int - (2 as libc::c_int - 1 as libc::c_int)
    {
        blk_size = 0 as libc::c_int as mln_size_t;
        j = 0 as libc::c_int;
        while (j as libc::c_ulong)
            < ((i / 2 as libc::c_int) as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as mln_size_t)
        {
            blk_size |= (1 as libc::c_int as mln_size_t) << j;
            j += 1;
            j;
        }
        am = &mut *tbl.offset(i as isize) as *mut mln_alloc_mgr_t;
        (*am).free_tail = 0 as *mut mln_alloc_blk_t;
        (*am).free_head = (*am).free_tail;
        (*am).used_tail = 0 as *mut mln_alloc_blk_t;
        (*am).used_head = (*am).used_tail;
        (*am).chunk_tail = 0 as *mut mln_alloc_chunk_t;
        (*am).chunk_head = (*am).chunk_tail;
        (*am).blk_size = blk_size.wrapping_add(1 as libc::c_int as libc::c_ulong);
        if i != 0 as libc::c_int {
            amprev = &mut *tbl.offset((i - 1 as libc::c_int) as isize)
                as *mut mln_alloc_mgr_t;
            (*amprev).free_tail = 0 as *mut mln_alloc_blk_t;
            (*amprev).free_head = (*amprev).free_tail;
            (*amprev).used_tail = 0 as *mut mln_alloc_blk_t;
            (*amprev).used_head = (*amprev).used_tail;
            (*amprev).chunk_tail = 0 as *mut mln_alloc_chunk_t;
            (*amprev).chunk_head = (*amprev).chunk_tail;
            (*amprev)
                .blk_size = ((*am).blk_size)
                .wrapping_add((*tbl.offset((i - 2 as libc::c_int) as isize)).blk_size)
                >> 1 as libc::c_int;
        }
        i += 2 as libc::c_int;
    }
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
                    current_block = 17999661984222547796;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                17999661984222547796 => {}
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
        current_block_8 = 16379945107832692948;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 16379945107832692948;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        16379945107832692948 => {
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
unsafe extern "C" fn mln_alloc_destroy(mut pool: *mut mln_alloc_t) {
    if pool.is_null() {
        return;
    }
    let mut parent: *mut mln_alloc_t = (*pool).parent;
    if !parent.is_null() && !((*parent).mem).is_null() {
        if ((*parent).lock).expect("non-null function pointer")((*parent).locker)
            != 0 as libc::c_int
        {
            return;
        }
    }
    if ((*pool).mem).is_null() {
        let mut am: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
        let mut amend: *mut mln_alloc_mgr_t = 0 as *mut mln_alloc_mgr_t;
        amend = ((*pool).mgr_tbl)
            .as_mut_ptr()
            .offset((18 as libc::c_int * 2 as libc::c_int) as isize)
            .offset(-((2 as libc::c_int - 1 as libc::c_int) as isize));
        let mut ch: *mut mln_alloc_chunk_t = 0 as *mut mln_alloc_chunk_t;
        am = ((*pool).mgr_tbl).as_mut_ptr();
        while am < amend {
            loop {
                ch = (*am).chunk_head;
                if ch.is_null() {
                    break;
                }
                mln_chunk_chain_del(&mut (*am).chunk_head, &mut (*am).chunk_tail, ch);
                if !parent.is_null() {
                    mln_alloc_free(ch as *mut libc::c_void);
                } else {
                    free(ch as *mut libc::c_void);
                }
            }
            am = am.offset(1);
            am;
        }
        loop {
            ch = (*pool).large_used_head;
            if ch.is_null() {
                break;
            }
            mln_chunk_chain_del(
                &mut (*pool).large_used_head,
                &mut (*pool).large_used_tail,
                ch,
            );
            if !parent.is_null() {
                mln_alloc_free(ch as *mut libc::c_void);
            } else {
                free(ch as *mut libc::c_void);
            }
        }
        if !parent.is_null() {
            mln_alloc_free(pool as *mut libc::c_void);
        } else {
            free(pool as *mut libc::c_void);
        }
    } else {
        munmap((*pool).mem, (*pool).shm_size);
    }
    if !parent.is_null() && !((*parent).mem).is_null() {
        ((*parent).unlock).expect("non-null function pointer")((*parent).locker);
    }
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
#[no_mangle]
pub unsafe extern "C" fn mln_fileset_init(
    mut max_file: mln_size_t,
) -> *mut mln_fileset_t {
    let mut attr: mln_rbtree_attr = mln_rbtree_attr {
        pool: 0 as *mut libc::c_void,
        pool_alloc: None,
        pool_free: None,
        cmp: None,
        data_free: None,
    };
    let mut fs: *mut mln_fileset_t = 0 as *mut mln_fileset_t;
    fs = malloc(::core::mem::size_of::<mln_fileset_t>() as libc::c_ulong)
        as *mut mln_fileset_t;
    if fs.is_null() {
        return 0 as *mut mln_fileset_t;
    }
    (*fs).pool = mln_alloc_init(0 as *mut mln_alloc_t);
    if ((*fs).pool).is_null() {
        free(fs as *mut libc::c_void);
        return 0 as *mut mln_fileset_t;
    }
    attr.pool = 0 as *mut libc::c_void;
    attr.pool_alloc = None;
    attr.pool_free = None;
    attr
        .cmp = Some(
        mln_file_set_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    attr
        .data_free = Some(
        mln_file_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*fs).reg_file_tree = mln_rbtree_new(&mut attr);
    if ((*fs).reg_file_tree).is_null() {
        mln_alloc_destroy((*fs).pool);
        free(fs as *mut libc::c_void);
        return 0 as *mut mln_fileset_t;
    }
    (*fs).reg_free_tail = 0 as *mut mln_file_t;
    (*fs).reg_free_head = (*fs).reg_free_tail;
    (*fs).max_file = max_file;
    return fs;
}
unsafe extern "C" fn mln_file_set_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut f1: *mut mln_file_t = data1 as *mut mln_file_t;
    let mut f2: *mut mln_file_t = data2 as *mut mln_file_t;
    return mln_string_strcmp((*f1).file_path, (*f2).file_path);
}
#[no_mangle]
pub unsafe extern "C" fn mln_fileset_destroy(mut fs: *mut mln_fileset_t) {
    if fs.is_null() {
        return;
    }
    if !((*fs).reg_file_tree).is_null() {
        mln_rbtree_free((*fs).reg_file_tree);
    }
    if !((*fs).pool).is_null() {
        mln_alloc_destroy((*fs).pool);
    }
    free(fs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_file_open(
    mut fs: *mut mln_fileset_t,
    mut filepath: *mut libc::c_char,
) -> *mut mln_file_t {
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut f: *mut mln_file_t = 0 as *mut mln_file_t;
    let mut tmpf: mln_file_t = mln_file_t {
        prev: 0 as *mut mln_file_s,
        next: 0 as *mut mln_file_s,
        fd: 0,
        is_tmp: [0; 1],
        c2rust_padding: [0; 3],
        file_path: 0 as *mut mln_string_t,
        fset: 0 as *mut mln_fileset_t,
        node: 0 as *mut mln_rbtree_node_t,
        refer_cnt: 0,
        size: 0,
        mtime: 0,
        ctime: 0,
        atime: 0,
    };
    let mut path: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        path.data = filepath as mln_u8ptr_t;
        path.len = strlen(filepath);
        path.set_data_ref(1 as libc::c_int as mln_uauto_t);
        path.set_pool(0 as libc::c_int as mln_uauto_t);
        path.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut path;
        &mut path
    });
    tmpf.file_path = &mut path;
    rn = mln_rbtree_search(
        (*fs).reg_file_tree,
        &mut tmpf as *mut mln_file_t as *mut libc::c_void,
    );
    if rn == &mut (*(*fs).reg_file_tree).nil as *mut mln_rbtree_node_t {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        f = mln_alloc_m(
            (*fs).pool,
            ::core::mem::size_of::<mln_file_t>() as libc::c_ulong,
        ) as *mut mln_file_t;
        if f.is_null() {
            return 0 as *mut mln_file_t;
        }
        (*f).file_path = mln_string_pool_new((*fs).pool, filepath);
        if ((*f).file_path).is_null() {
            mln_alloc_free(f as *mut libc::c_void);
            return 0 as *mut mln_file_t;
        }
        (*f).fd = open(filepath, 0 as libc::c_int);
        if (*f).fd < 0 as libc::c_int {
            let mut __s: *mut mln_string_t = (*f).file_path;
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
            mln_alloc_free(f as *mut libc::c_void);
            return 0 as *mut mln_file_t;
        }
        (*f).set_is_tmp(0 as libc::c_int as mln_u32_t);
        if fstat((*f).fd, &mut st) < 0 as libc::c_int {
            close((*f).fd);
            let mut __s: *mut mln_string_t = (*f).file_path;
            if !__s.is_null() {
                let ref mut fresh3 = (*__s).ref_0();
                let fresh4 = *fresh3;
                *fresh3 = (*fresh3).wrapping_sub(1);
                if fresh4 <= 1 as libc::c_int as libc::c_ulong {
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
            mln_alloc_free(f as *mut libc::c_void);
            return 0 as *mut mln_file_t;
        }
        (*f).mtime = st.st_mtim.tv_sec;
        (*f).ctime = st.st_ctim.tv_sec;
        (*f).atime = st.st_atim.tv_sec;
        (*f).size = st.st_size as size_t;
        (*f).refer_cnt = 0 as libc::c_int as size_t;
        reg_file_chain_add(&mut (*fs).reg_free_head, &mut (*fs).reg_free_tail, f);
        (*f).fset = fs;
        rn = mln_rbtree_node_new((*fs).reg_file_tree, f as *mut libc::c_void);
        if rn.is_null() {
            close((*f).fd);
            let mut __s: *mut mln_string_t = (*f).file_path;
            if !__s.is_null() {
                let ref mut fresh5 = (*__s).ref_0();
                let fresh6 = *fresh5;
                *fresh5 = (*fresh5).wrapping_sub(1);
                if fresh6 <= 1 as libc::c_int as libc::c_ulong {
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
            mln_alloc_free(f as *mut libc::c_void);
            return 0 as *mut mln_file_t;
        }
        mln_rbtree_insert((*fs).reg_file_tree, rn);
        (*f).node = rn;
    } else {
        f = (*rn).data as *mut mln_file_t;
    }
    let fresh7 = (*f).refer_cnt;
    (*f).refer_cnt = ((*f).refer_cnt).wrapping_add(1);
    if fresh7 == 0 as libc::c_int as libc::c_ulong {
        reg_file_chain_del(&mut (*fs).reg_free_head, &mut (*fs).reg_free_tail, f);
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn mln_file_close(mut pfile: *mut mln_file_t) {
    if pfile.is_null() {
        return;
    }
    let mut fs: *mut mln_fileset_t = 0 as *mut mln_fileset_t;
    if (*pfile).is_tmp() != 0 {
        if (*pfile).fd >= 0 as libc::c_int {
            close((*pfile).fd);
        }
        mln_alloc_free(pfile as *mut libc::c_void);
        return;
    }
    (*pfile).refer_cnt = ((*pfile).refer_cnt).wrapping_sub(1);
    if (*pfile).refer_cnt > 0 as libc::c_int as libc::c_ulong {
        return;
    }
    fs = (*pfile).fset;
    reg_file_chain_add(&mut (*fs).reg_free_head, &mut (*fs).reg_free_tail, pfile);
    if (*(*fs).reg_file_tree).nr_node > (*fs).max_file {
        pfile = (*fs).reg_free_head;
        reg_file_chain_del(&mut (*fs).reg_free_head, &mut (*fs).reg_free_tail, pfile);
        mln_rbtree_delete((*fs).reg_file_tree, (*pfile).node);
        mln_rbtree_node_free((*fs).reg_file_tree, (*pfile).node);
    }
}
unsafe extern "C" fn mln_file_free(mut pfile: *mut libc::c_void) {
    if pfile.is_null() {
        return;
    }
    let mut f: *mut mln_file_t = pfile as *mut mln_file_t;
    if !((*f).file_path).is_null() {
        let mut __s: *mut mln_string_t = (*f).file_path;
        if !__s.is_null() {
            let ref mut fresh8 = (*__s).ref_0();
            let fresh9 = *fresh8;
            *fresh8 = (*fresh8).wrapping_sub(1);
            if fresh9 <= 1 as libc::c_int as libc::c_ulong {
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
    if (*f).fd >= 0 as libc::c_int {
        close((*f).fd);
    }
    mln_alloc_free(f as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_file_tmp_open(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_file_t {
    let mut dir_path: [libc::c_char; 512] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut tmp_path: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut suffix: libc::c_ulong = 0;
    let mut f: *mut mln_file_t = 0 as *mut mln_file_t;
    snprintf(
        dir_path.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"%s\0" as *const u8 as *const libc::c_char,
        mln_path_tmpfile(),
    );
    if mkdir(
        dir_path.as_mut_ptr(),
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) as __mode_t,
    ) < 0 as libc::c_int
    {
        if *__errno_location() != 17 as libc::c_int {
            return 0 as *mut mln_file_t;
        }
    }
    f = mln_alloc_m(pool, ::core::mem::size_of::<mln_file_t>() as libc::c_ulong)
        as *mut mln_file_t;
    if f.is_null() {
        return 0 as *mut mln_file_t;
    }
    (*f).file_path = 0 as *mut mln_string_t;
    (*f).fd = -(1 as libc::c_int);
    (*f).set_is_tmp(1 as libc::c_int as mln_u32_t);
    (*f).atime = 0 as libc::c_int as time_t;
    (*f).ctime = (*f).atime;
    (*f).mtime = (*f).ctime;
    (*f).size = 0 as libc::c_int as size_t;
    (*f).refer_cnt = 0 as libc::c_int as size_t;
    (*f).next = 0 as *mut mln_file_s;
    (*f).prev = (*f).next;
    (*f).fset = 0 as *mut mln_fileset_t;
    (*f).node = 0 as *mut mln_rbtree_node_t;
    loop {
        gettimeofday(&mut now, 0 as *mut libc::c_void);
        suffix = (now.tv_sec * 1000000 as libc::c_int as libc::c_long + now.tv_usec)
            as libc::c_ulong;
        snprintf(
            tmp_path.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%s/mln_tmp_%lu.tmp\0" as *const u8 as *const libc::c_char,
            dir_path.as_mut_ptr(),
            suffix,
        );
        (*f)
            .fd = open(
            tmp_path.as_mut_ptr(),
            0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int,
        );
        if (*f).fd < 0 as libc::c_int {
            if *__errno_location() == 17 as libc::c_int {
                memset(
                    tmp_path.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            } else {
                mln_alloc_free(f as *mut libc::c_void);
                return 0 as *mut mln_file_t;
            }
        } else {
            unlink(tmp_path.as_mut_ptr());
            (*f).atime = now.tv_sec;
            (*f).ctime = (*f).atime;
            (*f).mtime = (*f).ctime;
            return f;
        }
    };
}
#[inline]
unsafe extern "C" fn reg_file_chain_del(
    mut head: *mut *mut mln_file_t,
    mut tail: *mut *mut mln_file_t,
    mut node: *mut mln_file_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_file_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_file_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_file_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_file_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn reg_file_chain_add(
    mut head: *mut *mut mln_file_t,
    mut tail: *mut *mut mln_file_t,
    mut node: *mut mln_file_t,
) {
    (*node).next = 0 as *mut mln_file_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
