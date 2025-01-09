use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mln_file_tmp_open(pool: *mut mln_alloc_t) -> *mut mln_file_t;
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_chain_pool_release_all(c: *mut mln_chain_t);
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut off_t,
        __count: size_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_tcp_conn_t {
    pub pool: *mut mln_alloc_t,
    pub rcv_head: *mut mln_chain_t,
    pub rcv_tail: *mut mln_chain_t,
    pub snd_head: *mut mln_chain_t,
    pub snd_tail: *mut mln_chain_t,
    pub sent_head: *mut mln_chain_t,
    pub sent_tail: *mut mln_chain_t,
    pub sockfd: libc::c_int,
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
                    current_block = 3782519129184425543;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                3782519129184425543 => {}
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
        current_block_8 = 2384836304670827819;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 2384836304670827819;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        2384836304670827819 => {
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
#[inline]
unsafe extern "C" fn mln_fd_is_nonblock(mut fd: libc::c_int) -> libc::c_int {
    let mut flg: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
    return flg & 0o4000 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_init(
    mut tc: *mut mln_tcp_conn_t,
    mut sockfd: libc::c_int,
) -> libc::c_int {
    (*tc).pool = mln_alloc_init(0 as *mut mln_alloc_t);
    if ((*tc).pool).is_null() {
        return -(1 as libc::c_int);
    }
    (*tc).rcv_tail = 0 as *mut mln_chain_t;
    (*tc).rcv_head = (*tc).rcv_tail;
    (*tc).snd_tail = 0 as *mut mln_chain_t;
    (*tc).snd_head = (*tc).snd_tail;
    (*tc).sent_tail = 0 as *mut mln_chain_t;
    (*tc).sent_head = (*tc).sent_tail;
    (*tc).sockfd = sockfd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_destroy(mut tc: *mut mln_tcp_conn_t) {
    if tc.is_null() {
        return;
    }
    mln_chain_pool_release_all(mln_tcp_conn_remove(tc, 1 as libc::c_int));
    mln_chain_pool_release_all(mln_tcp_conn_remove(tc, 2 as libc::c_int));
    mln_chain_pool_release_all(mln_tcp_conn_remove(tc, 3 as libc::c_int));
    mln_alloc_destroy((*tc).pool);
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_append_chain(
    mut tc: *mut mln_tcp_conn_t,
    mut c_head: *mut mln_chain_t,
    mut c_tail: *mut mln_chain_t,
    mut type_0: libc::c_int,
) {
    if c_head.is_null() {
        return;
    }
    let mut head: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    let mut tail: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        head = &mut (*tc).snd_head;
        tail = &mut (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        head = &mut (*tc).rcv_head;
        tail = &mut (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        head = &mut (*tc).sent_head;
        tail = &mut (*tc).sent_tail;
    }
    if c_tail.is_null() {
        c_tail = c_head;
        while !((*c_tail).next).is_null() {
            c_tail = (*c_tail).next;
        }
    }
    if (*head).is_null() {
        *head = c_head;
        *tail = c_tail;
    } else {
        (**tail).next = c_head;
        *tail = c_tail;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_append(
    mut tc: *mut mln_tcp_conn_t,
    mut c: *mut mln_chain_t,
    mut type_0: libc::c_int,
) {
    let mut head: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    let mut tail: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        head = &mut (*tc).snd_head;
        tail = &mut (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        head = &mut (*tc).rcv_head;
        tail = &mut (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        head = &mut (*tc).sent_head;
        tail = &mut (*tc).sent_tail;
    }
    if (*head).is_null() {
        *tail = c;
        *head = *tail;
    } else {
        (**tail).next = c;
        *tail = c;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_head(
    mut tc: *mut mln_tcp_conn_t,
    mut type_0: libc::c_int,
) -> *mut mln_chain_t {
    let mut rc: *mut mln_chain_t = 0 as *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        rc = (*tc).snd_head;
    } else if type_0 == 2 as libc::c_int {
        rc = (*tc).rcv_head;
    } else if type_0 == 3 as libc::c_int {
        rc = (*tc).sent_head;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_remove(
    mut tc: *mut mln_tcp_conn_t,
    mut type_0: libc::c_int,
) -> *mut mln_chain_t {
    let mut rc: *mut mln_chain_t = 0 as *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        rc = (*tc).snd_head;
        (*tc).snd_tail = 0 as *mut mln_chain_t;
        (*tc).snd_head = (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        rc = (*tc).rcv_head;
        (*tc).rcv_tail = 0 as *mut mln_chain_t;
        (*tc).rcv_head = (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        rc = (*tc).sent_head;
        (*tc).sent_tail = 0 as *mut mln_chain_t;
        (*tc).sent_head = (*tc).sent_tail;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_pop(
    mut tc: *mut mln_tcp_conn_t,
    mut type_0: libc::c_int,
) -> *mut mln_chain_t {
    let mut head: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    let mut tail: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        head = &mut (*tc).snd_head;
        tail = &mut (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        head = &mut (*tc).rcv_head;
        tail = &mut (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        head = &mut (*tc).sent_head;
        tail = &mut (*tc).sent_tail;
    }
    let mut rc: *mut mln_chain_t = *head;
    if rc == *tail {
        *tail = 0 as *mut mln_chain_t;
        *head = *tail;
        return rc;
    }
    *head = (*rc).next;
    (*rc).next = 0 as *mut mln_chain_s;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_tail(
    mut tc: *mut mln_tcp_conn_t,
    mut type_0: libc::c_int,
) -> *mut mln_chain_t {
    let mut rc: *mut mln_chain_t = 0 as *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        rc = (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        rc = (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        rc = (*tc).sent_tail;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_send(mut tc: *mut mln_tcp_conn_t) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if ((*tc).snd_head).is_null() {
        return 2 as libc::c_int;
    }
    loop {
        n = mln_tcp_conn_send_chain_memory(tc);
        if n == 0 as libc::c_int && !((*tc).snd_head).is_null()
            && !((*(*tc).snd_head).buf).is_null()
            && (*(*(*tc).snd_head).buf).in_file() as libc::c_int != 0
        {
            n = mln_tcp_conn_send_chain_file(tc);
            if n == 0 as libc::c_int && !((*tc).snd_head).is_null()
                && !((*(*tc).snd_head).buf).is_null()
                && (*(*(*tc).snd_head).buf).in_memory() as libc::c_int != 0
            {
                continue;
            }
            if n == 0 as libc::c_int {
                return 2 as libc::c_int;
            }
            if n > 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            return 3 as libc::c_int;
        } else {
            if n == 0 as libc::c_int {
                return 2 as libc::c_int;
            }
            if n > 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            return 3 as libc::c_int;
        }
    };
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_send_chain_memory(
    mut tc: *mut mln_tcp_conn_t,
) -> libc::c_int {
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut n: libc::c_int = 0;
    let mut is_done: libc::c_int = 0 as libc::c_int;
    let mut buf_left_size: mln_size_t = 0;
    let mut proc_vec: libc::c_int = 0;
    let mut nvec: libc::c_int = 256 as libc::c_int;
    let mut vector: [iovec; 256] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 256];
    if mln_fd_is_nonblock((*tc).sockfd) != 0 {
        's_19: loop {
            proc_vec = 0 as libc::c_int;
            c = (*tc).snd_head;
            while !c.is_null() {
                if proc_vec >= nvec {
                    break;
                }
                b = (*c).buf;
                if !b.is_null() {
                    if (*b).in_memory() == 0 {
                        break;
                    }
                    buf_left_size = (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*b).in_file() as libc::c_int != 0 {
                        (*b).file_last - (*b).file_left_pos
                    } else {
                        ((*b).last).offset_from((*b).left_pos) as libc::c_long
                    }) as mln_size_t;
                    if buf_left_size != 0 {
                        vector[proc_vec as usize]
                            .iov_base = (*b).left_pos as *mut libc::c_void;
                        vector[proc_vec as usize].iov_len = buf_left_size;
                        proc_vec += 1;
                        proc_vec;
                    }
                    if (*b).last_in_chain() != 0 {
                        break;
                    }
                }
                c = (*c).next;
            }
            if proc_vec == 0 {
                if !((*tc).snd_head).is_null() {
                    mln_chain_pool_release_all((*tc).snd_head);
                }
                (*tc).snd_tail = 0 as *mut mln_chain_t;
                (*tc).snd_head = (*tc).snd_tail;
                return 0 as libc::c_int;
            }
            loop {
                n = writev((*tc).sockfd, vector.as_mut_ptr(), proc_vec) as libc::c_int;
                if n <= 0 as libc::c_int {
                    if *__errno_location() == 4 as libc::c_int {
                        continue;
                    }
                    if *__errno_location() == 11 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    return -(1 as libc::c_int);
                } else {
                    loop {
                        c = (*tc).snd_head;
                        if c.is_null() {
                            break;
                        }
                        b = (*c).buf;
                        if b.is_null() {
                            c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                            mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                        } else {
                            if (*b).in_memory() == 0 {
                                break;
                            }
                            if (*b).last_in_chain() != 0 {
                                is_done = 1 as libc::c_int;
                            }
                            buf_left_size = (if b.is_null() {
                                0 as libc::c_int as libc::c_long
                            } else if (*b).in_file() as libc::c_int != 0 {
                                (*b).file_last - (*b).file_left_pos
                            } else {
                                ((*b).last).offset_from((*b).left_pos) as libc::c_long
                            }) as mln_size_t;
                            if n as libc::c_ulong >= buf_left_size {
                                (*b)
                                    .left_pos = ((*b).left_pos).offset(buf_left_size as isize);
                                n = (n as libc::c_ulong).wrapping_sub(buf_left_size)
                                    as libc::c_int as libc::c_int;
                                c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                                mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                            } else {
                                (*b).left_pos = ((*b).left_pos).offset(n as isize);
                                n = 0 as libc::c_int;
                            }
                            if is_done != 0 || n == 0 as libc::c_int {
                                break;
                            }
                        }
                    }
                    if is_done != 0 {
                        break 's_19;
                    } else {
                        break;
                    }
                }
            }
        }
        return 1 as libc::c_int;
    }
    proc_vec = 0 as libc::c_int;
    c = (*tc).snd_head;
    while !c.is_null() {
        if proc_vec >= nvec {
            break;
        }
        b = (*c).buf;
        if !b.is_null() {
            if (*b).in_memory() == 0 {
                break;
            }
            buf_left_size = (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else if (*b).in_file() as libc::c_int != 0 {
                (*b).file_last - (*b).file_left_pos
            } else {
                ((*b).last).offset_from((*b).left_pos) as libc::c_long
            }) as mln_size_t;
            if buf_left_size != 0 {
                vector[proc_vec as usize].iov_base = (*b).left_pos as *mut libc::c_void;
                vector[proc_vec as usize].iov_len = buf_left_size;
                proc_vec += 1;
                proc_vec;
            }
            if (*b).last_in_chain() != 0 {
                break;
            }
        }
        c = (*c).next;
    }
    if proc_vec == 0 {
        if !((*tc).snd_head).is_null() {
            mln_chain_pool_release_all((*tc).snd_head);
        }
        (*tc).snd_tail = 0 as *mut mln_chain_t;
        (*tc).snd_head = (*tc).snd_tail;
        return 0 as libc::c_int;
    }
    loop {
        n = writev((*tc).sockfd, vector.as_mut_ptr(), proc_vec) as libc::c_int;
        if n <= 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            return -(1 as libc::c_int);
        } else {
            loop {
                c = (*tc).snd_head;
                if c.is_null() {
                    break;
                }
                b = (*c).buf;
                if b.is_null() {
                    c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                    mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                } else {
                    if (*b).in_memory() == 0 {
                        break;
                    }
                    if (*b).last_in_chain() != 0 {
                        is_done = 1 as libc::c_int;
                    }
                    buf_left_size = (if b.is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*b).in_file() as libc::c_int != 0 {
                        (*b).file_last - (*b).file_left_pos
                    } else {
                        ((*b).last).offset_from((*b).left_pos) as libc::c_long
                    }) as mln_size_t;
                    if n as libc::c_ulong >= buf_left_size {
                        (*b).left_pos = ((*b).left_pos).offset(buf_left_size as isize);
                        n = (n as libc::c_ulong).wrapping_sub(buf_left_size)
                            as libc::c_int as libc::c_int;
                        c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                        mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                    } else {
                        (*b).left_pos = ((*b).left_pos).offset(n as isize);
                        n = 0 as libc::c_int;
                    }
                    if is_done != 0 || n == 0 as libc::c_int {
                        break;
                    }
                }
            }
            return is_done;
        }
    };
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_send_chain_file(
    mut tc: *mut mln_tcp_conn_t,
) -> libc::c_int {
    let mut sockfd: libc::c_int = (*tc).sockfd;
    let mut n: libc::c_int = 0;
    let mut is_done: libc::c_int = 0 as libc::c_int;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut buf_left_size: mln_size_t = 0;
    if mln_fd_is_nonblock(sockfd) != 0 {
        loop {
            c = (*tc).snd_head;
            if c.is_null() {
                break;
            }
            b = (*c).buf;
            if b.is_null() {
                c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                mln_tcp_conn_append(tc, c, 3 as libc::c_int);
            } else {
                if (*b).in_file() == 0 {
                    break;
                }
                buf_left_size = (if b.is_null() {
                    0 as libc::c_int as libc::c_long
                } else if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                }) as mln_size_t;
                if (*b).last_in_chain() != 0 {
                    is_done = 1 as libc::c_int;
                }
                if buf_left_size != 0 {
                    loop {
                        n = sendfile(
                            sockfd,
                            (*(*b).file).fd,
                            &mut (*b).file_left_pos,
                            buf_left_size,
                        ) as libc::c_int;
                        if n <= 0 as libc::c_int {
                            if *__errno_location() == 4 as libc::c_int {
                                continue;
                            }
                            if *__errno_location() == 11 as libc::c_int {
                                return 0 as libc::c_int;
                            }
                            return -(1 as libc::c_int);
                        } else if !(if b.is_null() {
                            0 as libc::c_int as libc::c_long
                        } else if (*b).in_file() as libc::c_int != 0 {
                            (*b).file_last - (*b).file_left_pos
                        } else {
                            ((*b).last).offset_from((*b).left_pos) as libc::c_long
                        } != 0)
                        {
                            break;
                        }
                    }
                }
                c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                if is_done != 0 {
                    break;
                }
            }
        }
        return 1 as libc::c_int;
    } else {
        loop {
            c = (*tc).snd_head;
            if c.is_null() {
                break;
            }
            b = (*c).buf;
            if !b.is_null() {
                if if b.is_null() {
                    0 as libc::c_int as libc::c_long
                } else if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                } != 0
                {
                    break;
                }
                if (*b).last_in_chain() != 0 {
                    is_done = 1 as libc::c_int;
                }
            }
            c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
            mln_tcp_conn_append(tc, c, 3 as libc::c_int);
            if is_done != 0 {
                return 1 as libc::c_int;
            }
        }
        if ((*tc).snd_head).is_null() {
            return 0 as libc::c_int;
        }
        if (*b).in_file() == 0 {
            return 0 as libc::c_int;
        }
        loop {
            n = sendfile(
                sockfd,
                (*(*b).file).fd,
                &mut (*b).file_left_pos,
                (if b.is_null() {
                    0 as libc::c_int as libc::c_long
                } else if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                }) as size_t,
            ) as libc::c_int;
            if n <= 0 as libc::c_int {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                return -(1 as libc::c_int);
            } else {
                if if b.is_null() {
                    0 as libc::c_int as libc::c_long
                } else if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                } != 0
                {
                    continue;
                }
                if (*b).last_in_chain() != 0 {
                    is_done = 1 as libc::c_int;
                }
                c = mln_tcp_conn_pop_inline(tc, 1 as libc::c_int);
                mln_tcp_conn_append(tc, c, 3 as libc::c_int);
                return is_done;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_pop_inline(
    mut tc: *mut mln_tcp_conn_t,
    mut type_0: libc::c_int,
) -> *mut mln_chain_t {
    let mut head: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    let mut tail: *mut *mut mln_chain_t = 0 as *mut *mut mln_chain_t;
    if type_0 == 1 as libc::c_int {
        head = &mut (*tc).snd_head;
        tail = &mut (*tc).snd_tail;
    } else if type_0 == 2 as libc::c_int {
        head = &mut (*tc).rcv_head;
        tail = &mut (*tc).rcv_tail;
    } else if type_0 == 3 as libc::c_int {
        head = &mut (*tc).sent_head;
        tail = &mut (*tc).sent_tail;
    }
    let mut rc: *mut mln_chain_t = *head;
    if rc == *tail {
        *tail = 0 as *mut mln_chain_t;
        *head = *tail;
        return rc;
    }
    *head = (*rc).next;
    (*rc).next = 0 as *mut mln_chain_s;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mln_tcp_conn_recv(
    mut tc: *mut mln_tcp_conn_t,
    mut flag: mln_u32_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut n: libc::c_int = 0;
    if mln_fd_is_nonblock((*tc).sockfd) != 0 {
        current_block = 12732413134667473024;
    } else {
        current_block = 9666564013501412840;
    }
    loop {
        match current_block {
            9666564013501412840 => {
                n = mln_tcp_conn_recv_chain(tc, flag);
                if n > 0 as libc::c_int {
                    return 2 as libc::c_int;
                }
            }
            _ => {
                loop {
                    n = mln_tcp_conn_recv_chain(tc, flag);
                    if !(n > 0 as libc::c_int) {
                        break;
                    }
                }
            }
        }
        if n == 0 as libc::c_int {
            return 4 as libc::c_int;
        }
        if !(*__errno_location() == 4 as libc::c_int) {
            break;
        }
        if mln_fd_is_nonblock((*tc).sockfd) != 0 {
            current_block = 12732413134667473024;
        } else {
            current_block = 9666564013501412840;
        }
    }
    if *__errno_location() == 11 as libc::c_int {
        return 2 as libc::c_int;
    }
    return 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_recv_chain(
    mut tc: *mut mln_tcp_conn_t,
    mut flag: mln_u32_t,
) -> libc::c_int {
    let mut last: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut pool: *mut mln_alloc_t = (*tc).pool;
    c = mln_chain_new(pool);
    b = mln_buf_new(pool);
    if c.is_null() || b.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*c).buf = b;
    if flag & 0x2 as libc::c_int as libc::c_uint != 0 {
        if flag & 0x8 as libc::c_int as libc::c_uint != 0 && !((*tc).rcv_tail).is_null()
            && !((*(*tc).rcv_tail).buf).is_null()
        {
            last = (*(*tc).rcv_tail).buf;
            if (*last).in_file() == 0 {
                last = 0 as *mut mln_buf_t;
            }
        }
        n = mln_tcp_conn_recv_chain_file((*tc).sockfd, pool, b, last);
    } else if flag & 0x1 as libc::c_int as libc::c_uint != 0 {
        n = mln_tcp_conn_recv_chain_mem((*tc).sockfd, pool, b);
    }
    if n <= 0 as libc::c_int {
        mln_chain_pool_release(c);
    } else {
        mln_tcp_conn_append(tc, c, 2 as libc::c_int);
    }
    return n;
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_recv_chain_file(
    mut sockfd: libc::c_int,
    mut pool: *mut mln_alloc_t,
    mut b: *mut mln_buf_t,
    mut last: *mut mln_buf_t,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut buf: [mln_u8_t; 1024] = [0; 1024];
    n = recv(
        sockfd,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[mln_u8_t; 1024]>() as libc::c_ulong,
        0 as libc::c_int,
    ) as libc::c_int;
    if n <= 0 as libc::c_int {
        return n;
    }
    if last.is_null() {
        (*b).file = mln_file_tmp_open(pool);
        if ((*b).file).is_null() {
            return -(1 as libc::c_int);
        }
        (*b).file_pos = 0 as libc::c_int as mln_off_t;
        (*b).file_left_pos = (*b).file_pos;
    } else {
        (*b).file_pos = (*last).file_last;
        (*b).file_left_pos = (*b).file_pos;
        (*b).file = (*last).file;
        (*last).shadow = b;
    }
    (*b).file_last = (*b).file_pos + n as libc::c_long;
    (*b).set_in_file(1 as libc::c_int as mln_u32_t);
    (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
    if write((*(*b).file).fd, buf.as_mut_ptr() as *const libc::c_void, n as size_t)
        < 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    return n;
}
#[inline]
unsafe extern "C" fn mln_tcp_conn_recv_chain_mem(
    mut sockfd: libc::c_int,
    mut pool: *mut mln_alloc_t,
    mut b: *mut mln_buf_t,
) -> libc::c_int {
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut n: libc::c_int = 0;
    buf = mln_alloc_m(pool, 1024 as libc::c_int as mln_size_t) as mln_u8ptr_t;
    if buf.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    n = recv(
        sockfd,
        buf as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if n <= 0 as libc::c_int {
        mln_alloc_free(buf as *mut libc::c_void);
        return n;
    }
    (*b).start = buf;
    (*b).pos = (*b).start;
    (*b).left_pos = (*b).pos;
    (*b).end = buf.offset(n as isize);
    (*b).last = (*b).end;
    (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
    (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
    return n;
}
