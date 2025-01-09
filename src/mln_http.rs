use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mln_string_pool_dup(
        pool: *mut mln_alloc_t,
        str: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_string_strcasecmp(
        s1: *mut mln_string_t,
        s2: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_chain_pool_release_all(c: *mut mln_chain_t);
    fn mln_hash_new(attr: *mut mln_hash_attr) -> *mut mln_hash_t;
    fn mln_hash_free(h: *mut mln_hash_t, flg: mln_hash_flag_t);
    fn mln_hash_search(h: *mut mln_hash_t, key: *mut libc::c_void) -> *mut libc::c_void;
    fn mln_hash_search_iterator(
        h: *mut mln_hash_t,
        key: *mut libc::c_void,
        ctx: *mut *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn mln_hash_update(
        h: *mut mln_hash_t,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_insert(
        h: *mut mln_hash_t,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_remove(h: *mut mln_hash_t, key: *mut libc::c_void, flg: mln_hash_flag_t);
    fn mln_hash_iterate(
        h: *mut mln_hash_t,
        handler: hash_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_reset(h: *mut mln_hash_t, flg: mln_hash_flag_t);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_http_s {
    pub connection: *mut mln_tcp_conn_t,
    pub pool: *mut mln_alloc_t,
    pub header_fields: *mut mln_hash_t,
    pub body_head: *mut mln_chain_t,
    pub body_tail: *mut mln_chain_t,
    pub body_handler: mln_http_handler,
    pub data: *mut libc::c_void,
    pub uri: *mut mln_string_t,
    pub args: *mut mln_string_t,
    pub response_msg: *mut mln_string_t,
    pub error: mln_u32_t,
    pub status: mln_u32_t,
    pub method: mln_u32_t,
    pub version: mln_u32_t,
    #[bitfield(name = "type_0", ty = "mln_u32_t", bits = "0..=1")]
    #[bitfield(name = "done", ty = "mln_u32_t", bits = "2..=2")]
    pub type_0_done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mln_http_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_http_t,
        *mut *mut mln_chain_t,
        *mut *mut mln_chain_t,
    ) -> libc::c_int,
>;
pub type mln_http_t = mln_http_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_http_map_t {
    pub msg_str: mln_string_t,
    pub code_str: mln_string_t,
    pub code: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_http_chain_s {
    pub http: *mut mln_http_t,
    pub head: *mut mln_chain_t,
    pub tail: *mut mln_chain_t,
    pub pos: mln_u8ptr_t,
    pub left_size: mln_size_t,
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
pub static mut http_version: [mln_string_t; 2] = [mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
}; 2];
#[no_mangle]
pub static mut http_method: [mln_string_t; 8] = [mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
}; 8];
#[no_mangle]
pub static mut mln_http_status: [mln_http_map_t; 55] = [mln_http_map_t {
    msg_str: mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    },
    code_str: mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    },
    code: 0,
}; 55];
#[no_mangle]
pub unsafe extern "C" fn mln_http_parse(
    mut http: *mut mln_http_t,
    mut in_0: *mut *mut mln_chain_t,
) -> libc::c_int {
    if http.is_null() {
        return 2 as libc::c_int;
    }
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut len: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut handler: mln_http_handler = (*http).body_handler;
    while (*http).done() == 0
        && {
            ret = mln_http_line_length(http, *in_0, &mut len);
            ret == 1 as libc::c_int
        }
    {
        rc = (mln_http_process_line(http, in_0, len) == 2 as libc::c_int) as libc::c_int;
        if rc != 0 {
            return rc;
        }
    }
    if ret == 0 as libc::c_int || ret == 2 as libc::c_int {
        return ret;
    }
    if handler.is_some() {
        ret = handler
            .expect("non-null function pointer")(http, in_0, 0 as *mut *mut mln_chain_t);
    }
    if ret == 1 as libc::c_int {
        (*http).set_done(0 as libc::c_int as mln_u32_t);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn mln_http_line_length(
    mut http: *mut mln_http_t,
    mut in_0: *mut mln_chain_t,
    mut len: *mut mln_size_t,
) -> libc::c_int {
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut length: mln_size_t = 0 as libc::c_int as mln_size_t;
    while !in_0.is_null() {
        b = (*in_0).buf;
        if b.is_null() || (*b).in_file() as libc::c_int != 0
            || (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                })
            }) <= 0 as libc::c_int as libc::c_long
        {
            in_0 = (*in_0).next;
        } else {
            p = (*b).left_pos;
            end = (*b).last;
            while p < end {
                if *p as libc::c_int == '\n' as i32 as mln_u8_t as libc::c_int {
                    break;
                }
                length = length.wrapping_add(1);
                length;
                p = p.offset(1);
                p;
            }
            if !(p >= end) {
                break;
            }
            in_0 = (*in_0).next;
        }
    }
    if in_0.is_null() {
        return 0 as libc::c_int;
    }
    *len = length;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_process_line(
    mut http: *mut mln_http_t,
    mut in_0: *mut *mut mln_chain_t,
    mut len: mln_size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut last: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut type_0: mln_u32_t = (*http).type_0();
    buf = mln_alloc_m(pool, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    if buf.is_null() {
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    *buf.offset(len as isize) = 0 as libc::c_int as libc::c_uchar;
    p = buf;
    loop {
        c = *in_0;
        if c.is_null() {
            break;
        }
        b = (*c).buf;
        if b.is_null() || (*b).in_file() as libc::c_int != 0
            || (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                })
            }) <= 0 as libc::c_int as libc::c_long
        {
            *in_0 = (**in_0).next;
            mln_chain_pool_release(c);
        } else {
            while (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                })
            }) > 0 as libc::c_int as libc::c_long
            {
                if *(*b).left_pos as libc::c_int == '\n' as i32 {
                    break;
                }
                let fresh1 = (*b).left_pos;
                (*b).left_pos = ((*b).left_pos).offset(1);
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = *fresh1;
            }
            if !((if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*b).in_file() as libc::c_int != 0 {
                    (*b).file_last - (*b).file_left_pos
                } else {
                    ((*b).last).offset_from((*b).left_pos) as libc::c_long
                })
            }) > 0 as libc::c_int as libc::c_long)
            {
                continue;
            }
            (*b).left_pos = ((*b).left_pos).offset(1);
            (*b).left_pos;
            break;
        }
    }
    last = buf.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while last > buf {
        if *last as libc::c_int != 0 as libc::c_int {
            break;
        }
        last = last.offset(-1);
        last;
        len = len.wrapping_sub(1);
        len;
    }
    if len == 0 as libc::c_int as libc::c_ulong
        || len == 1 as libc::c_int as libc::c_ulong
            && *buf.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
    {
        mln_alloc_free(buf as *mut libc::c_void);
        (*http).set_done(1 as libc::c_int as mln_u32_t);
        return 0 as libc::c_int;
    }
    if *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\r' as i32
    {
        *buf
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_uchar;
        len = len.wrapping_sub(1);
        len;
    }
    if type_0 == 0 as libc::c_int as libc::c_uint {
        ret = mln_http_parse_headline(http, buf, len);
    } else {
        ret = mln_http_parse_field(http, buf, len);
    }
    mln_alloc_free(buf as *mut libc::c_void);
    return ret;
}
#[inline]
unsafe extern "C" fn mln_http_parse_headline(
    mut http: *mut mln_http_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_size_t,
) -> libc::c_int {
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = buf.offset(len as isize);
    let mut ques: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut scan: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut send: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_u32_t = 0;
    let mut status: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        (*http).set_done(1 as libc::c_int as mln_u32_t);
        return 0 as libc::c_int;
    }
    p = buf;
    while p < end {
        if *p as libc::c_int == ' ' as i32 as mln_u8_t as libc::c_int
            || *p as libc::c_int == '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        p = p.offset(1);
        p;
    }
    ({
        tmp.data = buf;
        tmp.len = p.offset_from(buf) as libc::c_long as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    send = http_version
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[mln_string_t; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<mln_string_t>() as libc::c_ulong)
                as isize,
        );
    scan = http_version.as_mut_ptr();
    while scan < send {
        if mln_string_strcasecmp(&mut tmp, scan) == 0 {
            break;
        }
        scan = scan.offset(1);
        scan;
    }
    if scan < send {
        type_0 = 2 as libc::c_int as mln_u32_t;
        (*http).set_type_0(2 as libc::c_int as mln_u32_t);
        (*http)
            .version = scan.offset_from(http_version.as_mut_ptr()) as libc::c_long
            as mln_u32_t;
    } else {
        send = http_method
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<[mln_string_t; 8]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
                    ) as isize,
            );
        scan = http_method.as_mut_ptr();
        while scan < send {
            if mln_string_strcasecmp(&mut tmp, scan) == 0 {
                break;
            }
            scan = scan.offset(1);
            scan;
        }
        if scan < send {
            type_0 = 1 as libc::c_int as mln_u32_t;
            (*http).set_type_0(1 as libc::c_int as mln_u32_t);
            (*http)
                .method = scan.offset_from(http_method.as_mut_ptr()) as libc::c_long
                as mln_u32_t;
        } else {
            (*http).error = 400 as libc::c_int as mln_u32_t;
            return 2 as libc::c_int;
        }
    }
    buf = p;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        if type_0 == 1 as libc::c_int as libc::c_uint {
            (*http).error = 400 as libc::c_int as mln_u32_t;
        } else {
            (*http).error = 600 as libc::c_int as mln_u32_t;
        }
        return 2 as libc::c_int;
    }
    if type_0 == 1 as libc::c_int as libc::c_uint {
        ques = 0 as mln_u8ptr_t;
        p = buf;
        while p < end {
            if ques.is_null()
                && *p as libc::c_int == '?' as i32 as mln_u8_t as libc::c_int
            {
                ques = p;
            }
            if *p as libc::c_int == ' ' as i32 as mln_u8_t as libc::c_int
                || *p as libc::c_int == '\t' as i32 as mln_u8_t as libc::c_int
            {
                break;
            }
            p = p.offset(1);
            p;
        }
        if ques.is_null() || ques.offset(1 as libc::c_int as isize) >= p {
            ({
                tmp.data = buf;
                tmp
                    .len = (if ques.is_null() {
                    p.offset_from(buf) as libc::c_long
                } else {
                    ques.offset_from(buf) as libc::c_long
                }) as mln_u64_t;
                tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut tmp;
                &mut tmp
            });
            s = mln_string_pool_dup(pool, &mut tmp);
            if s.is_null() {
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            (*http).uri = s;
            (*http).args = 0 as *mut mln_string_t;
        } else {
            ({
                tmp.data = buf;
                tmp.len = ques.offset_from(buf) as libc::c_long as mln_u64_t;
                tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut tmp;
                &mut tmp
            });
            s = mln_string_pool_dup(pool, &mut tmp);
            if s.is_null() {
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            (*http).uri = s;
            ({
                ques = ques.offset(1);
                tmp.data = ques;
                tmp.len = p.offset_from(ques) as libc::c_long as mln_u64_t;
                tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut tmp;
                &mut tmp
            });
            s = mln_string_pool_dup(pool, &mut tmp);
            if s.is_null() {
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            (*http).args = s;
        }
    } else {
        p = buf;
        while p < end {
            if *p as libc::c_int == ' ' as i32 as mln_u8_t as libc::c_int
                || *p as libc::c_int == '\t' as i32 as mln_u8_t as libc::c_int
            {
                break;
            }
            p = p.offset(1);
            p;
        }
        ({
            tmp.data = buf;
            tmp.len = p.offset_from(buf) as libc::c_long as mln_u64_t;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
        if mln_http_atou(&mut tmp, &mut status) == 2 as libc::c_int {
            (*http).error = 600 as libc::c_int as mln_u32_t;
            return 2 as libc::c_int;
        }
        (*http).status = status;
    }
    buf = p;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        if type_0 == 1 as libc::c_int as libc::c_uint {
            (*http).version = 0 as libc::c_int as mln_u32_t;
        } else {
            (*http).response_msg = 0 as *mut mln_string_t;
        }
        return 0 as libc::c_int;
    }
    if type_0 == 1 as libc::c_int as libc::c_uint {
        ({
            tmp.data = buf;
            tmp.len = end.offset_from(buf) as libc::c_long as mln_u64_t;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
        send = http_version
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<[mln_string_t; 2]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
                    ) as isize,
            );
        scan = http_version.as_mut_ptr();
        while scan < send {
            if mln_string_strcasecmp(&mut tmp, scan) == 0 {
                break;
            }
            scan = scan.offset(1);
            scan;
        }
        if scan >= send {
            (*http).error = 400 as libc::c_int as mln_u32_t;
            return 2 as libc::c_int;
        }
        (*http)
            .version = scan.offset_from(http_version.as_mut_ptr()) as libc::c_long
            as mln_u32_t;
        return 0 as libc::c_int;
    }
    ({
        tmp.data = buf;
        tmp.len = end.offset_from(buf) as libc::c_long as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    s = mln_string_pool_dup(pool, &mut tmp);
    if s.is_null() {
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    (*http).response_msg = s;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_parse_field(
    mut http: *mut mln_http_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_size_t,
) -> libc::c_int {
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = buf.offset(len as isize);
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut v: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut type_0: mln_u32_t = (*http).type_0();
    let mut header_fields: *mut mln_hash_t = (*http).header_fields;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        (*http).set_done(1 as libc::c_int as mln_u32_t);
        return 0 as libc::c_int;
    }
    p = buf;
    while p < end {
        if *p as libc::c_int == ' ' as i32 as mln_u8_t as libc::c_int
            || *p as libc::c_int == '\t' as i32 as mln_u8_t as libc::c_int
            || *p as libc::c_int == ':' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        p = p.offset(1);
        p;
    }
    if p.offset_from(buf) as libc::c_long <= 0 as libc::c_int as libc::c_long {
        if type_0 == 1 as libc::c_int as libc::c_uint {
            (*http).error = 400 as libc::c_int as mln_u32_t;
        } else {
            (*http).error = 600 as libc::c_int as mln_u32_t;
        }
        return 2 as libc::c_int;
    }
    ({
        tmp.data = buf;
        tmp.len = p.offset_from(buf) as libc::c_long as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    s = mln_string_pool_dup(pool, &mut tmp);
    if s.is_null() {
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    buf = p;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        if mln_hash_insert(header_fields, s as *mut libc::c_void, 0 as *mut libc::c_void)
            < 0 as libc::c_int
        {
            let mut __s: *mut mln_string_t = s;
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
            (*http).error = 500 as libc::c_int as mln_u32_t;
            return 2 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int
        != ':' as i32 as mln_u8_t as libc::c_int
    {
        let mut __s: *mut mln_string_t = s;
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
        if type_0 == 1 as libc::c_int as libc::c_uint {
            (*http).error = 400 as libc::c_int as mln_u32_t;
        } else {
            (*http).error = 600 as libc::c_int as mln_u32_t;
        }
        return 2 as libc::c_int;
    }
    buf = buf.offset(1);
    buf;
    while buf < end {
        if *buf as libc::c_int != ' ' as i32 as mln_u8_t as libc::c_int
            && *buf as libc::c_int != '\t' as i32 as mln_u8_t as libc::c_int
        {
            break;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf >= end {
        if mln_hash_insert(header_fields, s as *mut libc::c_void, 0 as *mut libc::c_void)
            < 0 as libc::c_int
        {
            let mut __s: *mut mln_string_t = s;
            if !__s.is_null() {
                let ref mut fresh7 = (*__s).ref_0();
                let fresh8 = *fresh7;
                *fresh7 = (*fresh7).wrapping_sub(1);
                if fresh8 <= 1 as libc::c_int as libc::c_ulong {
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
            (*http).error = 500 as libc::c_int as mln_u32_t;
            return 2 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    ({
        tmp.data = buf;
        tmp.len = end.offset_from(buf) as libc::c_long as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    v = mln_string_pool_dup(pool, &mut tmp);
    if v.is_null() {
        let mut __s: *mut mln_string_t = s;
        if !__s.is_null() {
            let ref mut fresh9 = (*__s).ref_0();
            let fresh10 = *fresh9;
            *fresh9 = (*fresh9).wrapping_sub(1);
            if fresh10 <= 1 as libc::c_int as libc::c_ulong {
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
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    if mln_hash_insert(header_fields, s as *mut libc::c_void, v as *mut libc::c_void)
        < 0 as libc::c_int
    {
        let mut __s: *mut mln_string_t = v;
        if !__s.is_null() {
            let ref mut fresh11 = (*__s).ref_0();
            let fresh12 = *fresh11;
            *fresh11 = (*fresh11).wrapping_sub(1);
            if fresh12 <= 1 as libc::c_int as libc::c_ulong {
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
        let mut __s: *mut mln_string_t = s;
        if !__s.is_null() {
            let ref mut fresh13 = (*__s).ref_0();
            let fresh14 = *fresh13;
            *fresh13 = (*fresh13).wrapping_sub(1);
            if fresh14 <= 1 as libc::c_int as libc::c_ulong {
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
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_generate(
    mut http: *mut mln_http_t,
    mut out_head: *mut *mut mln_chain_t,
    mut out_tail: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut current_block: u64;
    if http.is_null() || out_head.is_null() || out_tail.is_null() {
        return 2 as libc::c_int;
    }
    let mut type_0: mln_u32_t = (*http).type_0();
    let mut header_fields: *mut mln_hash_t = (*http).header_fields;
    let mut handler: mln_http_handler = (*http).body_handler;
    let mut hc: mln_http_chain_s = mln_http_chain_s {
        http: 0 as *mut mln_http_t,
        head: 0 as *mut mln_chain_t,
        tail: 0 as *mut mln_chain_t,
        pos: 0 as *mut libc::c_uchar,
        left_size: 0,
    };
    let mut ret: libc::c_int = 0;
    if type_0 == 0 as libc::c_int as libc::c_uint {
        (*http).error = 500 as libc::c_int as mln_u32_t;
    } else {
        hc.http = http;
        if !(*out_head).is_null() {
            hc.head = *out_head;
            hc.tail = *out_tail;
        } else {
            hc.tail = 0 as *mut mln_chain_t;
            hc.head = hc.tail;
        }
        hc.pos = 0 as mln_u8ptr_t;
        hc.left_size = 0 as libc::c_int as mln_size_t;
        if (*http).done() == 0 {
            if handler.is_some() {
                ret = handler
                    .expect(
                        "non-null function pointer",
                    )(http, &mut (*http).body_head, &mut (*http).body_tail);
                if ret == 2 as libc::c_int {
                    current_block = 6913057124011898511;
                } else {
                    if ret == 0 as libc::c_int {
                        (*http).error = 200 as libc::c_int as mln_u32_t;
                        *out_head = hc.head;
                        *out_tail = hc.tail;
                        return 0 as libc::c_int;
                    }
                    current_block = 2370887241019905314;
                }
            } else {
                current_block = 2370887241019905314;
            }
            match current_block {
                6913057124011898511 => {}
                _ => {
                    (*http).set_done(1 as libc::c_int as mln_u32_t);
                    current_block = 26972500619410423;
                }
            }
        } else {
            current_block = 26972500619410423;
        }
        match current_block {
            6913057124011898511 => {}
            _ => {
                if type_0 == 2 as libc::c_int as libc::c_uint {
                    if mln_http_generate_version(&mut hc) == 2 as libc::c_int {
                        current_block = 6913057124011898511;
                    } else if mln_http_generate_write(
                        &mut hc,
                        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                        1 as libc::c_int as mln_size_t,
                    ) == 2 as libc::c_int
                    {
                        current_block = 6913057124011898511;
                    } else if mln_http_generate_status(&mut hc) == 2 as libc::c_int {
                        current_block = 6913057124011898511;
                    } else {
                        current_block = 8693738493027456495;
                    }
                } else if mln_http_generate_method(&mut hc) == 2 as libc::c_int {
                    current_block = 6913057124011898511;
                } else if mln_http_generate_write(
                    &mut hc,
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as mln_size_t,
                ) == 2 as libc::c_int
                {
                    current_block = 6913057124011898511;
                } else if mln_http_generate_uri(&mut hc) == 2 as libc::c_int {
                    current_block = 6913057124011898511;
                } else if mln_http_generate_write(
                    &mut hc,
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as mln_size_t,
                ) == 2 as libc::c_int
                {
                    current_block = 6913057124011898511;
                } else if mln_http_generate_version(&mut hc) == 2 as libc::c_int {
                    current_block = 6913057124011898511;
                } else {
                    current_block = 8693738493027456495;
                }
                match current_block {
                    6913057124011898511 => {}
                    _ => {
                        if !(mln_http_generate_write(
                            &mut hc,
                            b"\r\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_void,
                            2 as libc::c_int as mln_size_t,
                        ) == 2 as libc::c_int)
                        {
                            if !header_fields.is_null() {
                                if mln_hash_iterate(
                                    header_fields,
                                    Some(
                                        mln_http_generate_fields_hash_iterate_handler
                                            as unsafe extern "C" fn(
                                                *mut mln_hash_t,
                                                *mut libc::c_void,
                                                *mut libc::c_void,
                                                *mut libc::c_void,
                                            ) -> libc::c_int,
                                    ),
                                    &mut hc as *mut mln_http_chain_s as *mut libc::c_void,
                                ) < 0 as libc::c_int
                                {
                                    current_block = 6913057124011898511;
                                } else {
                                    current_block = 14434620278749266018;
                                }
                            } else {
                                current_block = 14434620278749266018;
                            }
                            match current_block {
                                6913057124011898511 => {}
                                _ => {
                                    if !(mln_http_generate_write(
                                        &mut hc,
                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_void,
                                        2 as libc::c_int as mln_size_t,
                                    ) == 2 as libc::c_int)
                                    {
                                        if ((*http).body_head).is_null() {
                                            if mln_http_generate_set_last_in_chain(&mut hc)
                                                == 2 as libc::c_int
                                            {
                                                current_block = 6913057124011898511;
                                            } else {
                                                current_block = 8845338526596852646;
                                            }
                                        } else {
                                            current_block = 8845338526596852646;
                                        }
                                        match current_block {
                                            6913057124011898511 => {}
                                            _ => {
                                                if (hc.head).is_null() {
                                                    hc.head = (*http).body_head;
                                                    hc.tail = (*http).body_tail;
                                                } else {
                                                    (*hc.tail).next = (*http).body_head;
                                                    hc.tail = (*http).body_tail;
                                                }
                                                (*http).body_tail = 0 as *mut mln_chain_t;
                                                (*http).body_head = (*http).body_tail;
                                                (*http).set_done(0 as libc::c_int as mln_u32_t);
                                                (*http).error = 200 as libc::c_int as mln_u32_t;
                                                *out_head = hc.head;
                                                *out_tail = hc.tail;
                                                return 1 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mln_chain_pool_release_all(hc.head);
    *out_tail = 0 as *mut mln_chain_t;
    *out_head = *out_tail;
    return 2 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_set_last_in_chain(
    mut hc: *mut mln_http_chain_s,
) -> libc::c_int {
    if !((*hc).tail).is_null() && !((*(*hc).tail).buf).is_null() {
        (*(*(*hc).tail).buf).set_last_in_chain(1 as libc::c_int as mln_u32_t);
        return 0 as libc::c_int;
    }
    let mut http: *mut mln_http_t = (*hc).http;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut c: *mut mln_chain_t = mln_chain_new(pool);
    if c.is_null() {
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    let mut b: *mut mln_buf_t = mln_buf_new(pool);
    if b.is_null() {
        mln_chain_pool_release(c);
        (*http).error = 500 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    (*c).buf = b;
    (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
    (*b).set_last_in_chain(1 as libc::c_int as mln_u32_t);
    if ((*hc).head).is_null() {
        (*hc).head = (*hc).tail;
    } else {
        (*(*hc).tail).next = c;
        (*hc).tail = c;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_write(
    mut hc: *mut mln_http_chain_s,
    mut buf: *mut libc::c_void,
    mut size: mln_size_t,
) -> libc::c_int {
    let mut cur: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut http: *mut mln_http_t = (*hc).http;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    while size > 0 as libc::c_int as libc::c_ulong {
        if (*hc).left_size == 0 as libc::c_int as libc::c_ulong {
            let mut c: *mut mln_chain_t = mln_chain_new(pool);
            if c.is_null() {
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            let mut b: *mut mln_buf_t = mln_buf_new(pool);
            if b.is_null() {
                mln_chain_pool_release(c);
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            (*c).buf = b;
            let mut buffer: mln_u8ptr_t = mln_alloc_m(
                pool,
                1024 as libc::c_int as mln_size_t,
            ) as mln_u8ptr_t;
            if buffer.is_null() {
                mln_chain_pool_release(c);
                (*http).error = 500 as libc::c_int as mln_u32_t;
                return 2 as libc::c_int;
            }
            (*b).start = buffer;
            (*b).pos = (*b).start;
            (*b).left_pos = (*b).pos;
            (*b).end = buffer;
            (*b).last = (*b).end;
            (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
            (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
            (*hc).pos = buffer;
            (*hc).left_size = 1024 as libc::c_int as mln_size_t;
            if ((*hc).head).is_null() {
                (*hc).tail = c;
                (*hc).head = (*hc).tail;
            } else {
                (*(*hc).tail).next = c;
                (*hc).tail = c;
            }
        }
        if ((*hc).tail).is_null() || ((*(*hc).tail).buf).is_null() {
            (*hc).left_size = 0 as libc::c_int as mln_size_t;
        } else {
            cur = (*(*hc).tail).buf;
            if size <= (*hc).left_size {
                memcpy((*hc).pos as *mut libc::c_void, buf, size);
                (*hc)
                    .left_size = ((*hc).left_size as libc::c_ulong).wrapping_sub(size)
                    as mln_size_t as mln_size_t;
                (*hc).pos = ((*hc).pos).offset(size as isize);
                buf = buf.offset(size as isize);
                size = 0 as libc::c_int as mln_size_t;
            } else {
                memcpy((*hc).pos as *mut libc::c_void, buf, (*hc).left_size);
                size = (size as libc::c_ulong).wrapping_sub((*hc).left_size)
                    as mln_size_t as mln_size_t;
                buf = buf.offset((*hc).left_size as isize);
                (*hc).pos = ((*hc).pos).offset((*hc).left_size as isize);
                (*hc).left_size = 0 as libc::c_int as mln_size_t;
            }
            (*cur).end = (*hc).pos;
            (*cur).last = (*cur).end;
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_version(
    mut hc: *mut mln_http_chain_s,
) -> libc::c_int {
    let mut version: mln_u32_t = (*(*hc).http).version;
    if version as libc::c_ulong
        >= (::core::mem::size_of::<[mln_string_t; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mln_string_t>() as libc::c_ulong)
    {
        if (*(*hc).http).type_0() as libc::c_int == 1 as libc::c_int {
            (*(*hc).http).error = 400 as libc::c_int as mln_u32_t;
        } else {
            (*(*hc).http).error = 600 as libc::c_int as mln_u32_t;
        }
        return 2 as libc::c_int;
    }
    let mut p: *mut mln_string_t = &mut *http_version
        .as_mut_ptr()
        .offset(version as isize) as *mut mln_string_t;
    if mln_http_generate_write(hc, (*p).data as *mut libc::c_void, (*p).len)
        == 2 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_status(
    mut hc: *mut mln_http_chain_s,
) -> libc::c_int {
    let mut status: mln_u32_t = (*(*hc).http).status;
    let mut map: *mut mln_http_map_t = mln_http_status.as_mut_ptr();
    let mut end: *mut mln_http_map_t = mln_http_status
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[mln_http_map_t; 55]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<mln_http_map_t>() as libc::c_ulong)
                as isize,
        );
    while map < end {
        if status == (*map).code {
            break;
        }
        map = map.offset(1);
        map;
    }
    if map >= end {
        (*(*hc).http).error = 600 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    if mln_http_generate_write(
        hc,
        (*map).code_str.data as *mut libc::c_void,
        (*map).code_str.len,
    ) == 2 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if mln_http_generate_write(
        hc,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        1 as libc::c_int as mln_size_t,
    ) == 2 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if mln_http_generate_write(
        hc,
        (*map).msg_str.data as *mut libc::c_void,
        (*map).msg_str.len,
    ) == 2 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_method(
    mut hc: *mut mln_http_chain_s,
) -> libc::c_int {
    let mut method: mln_u32_t = (*(*hc).http).method;
    if method as libc::c_ulong
        >= (::core::mem::size_of::<[mln_string_t; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mln_string_t>() as libc::c_ulong)
    {
        (*(*hc).http).error = 400 as libc::c_int as mln_u32_t;
        return 2 as libc::c_int;
    }
    let mut p: *mut mln_string_t = &mut *http_method.as_mut_ptr().offset(method as isize)
        as *mut mln_string_t;
    if mln_http_generate_write(hc, (*p).data as *mut libc::c_void, (*p).len)
        == 2 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_http_generate_uri(
    mut hc: *mut mln_http_chain_s,
) -> libc::c_int {
    let mut uri: *mut mln_string_t = (*(*hc).http).uri;
    if uri.is_null() {
        if mln_http_generate_write(
            hc,
            b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            1 as libc::c_int as mln_size_t,
        ) == 2 as libc::c_int
        {
            return 2 as libc::c_int;
        }
    } else if mln_http_generate_write(hc, (*uri).data as *mut libc::c_void, (*uri).len)
        == 2 as libc::c_int
    {
        return 2 as libc::c_int
    }
    let mut args: *mut mln_string_t = (*(*hc).http).args;
    if !args.is_null() {
        if mln_http_generate_write(
            hc,
            b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            1 as libc::c_int as mln_size_t,
        ) == 2 as libc::c_int
        {
            return 2 as libc::c_int;
        }
        if mln_http_generate_write(hc, (*args).data as *mut libc::c_void, (*args).len)
            == 2 as libc::c_int
        {
            return 2 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_http_generate_fields_hash_iterate_handler(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut k: *mut mln_string_t = key as *mut mln_string_t;
    let mut v: *mut mln_string_t = val as *mut mln_string_t;
    let mut hc: *mut mln_http_chain_s = data as *mut mln_http_chain_s;
    if mln_http_generate_write(hc, (*k).data as *mut libc::c_void, (*k).len)
        == 2 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mln_http_generate_write(
        hc,
        b": \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        2 as libc::c_int as mln_size_t,
    ) == 2 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if !val.is_null() {
        if mln_http_generate_write(hc, (*v).data as *mut libc::c_void, (*v).len)
            == 2 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    if mln_http_generate_write(
        hc,
        b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        2 as libc::c_int as mln_size_t,
    ) == 2 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_field_set(
    mut http: *mut mln_http_t,
    mut key: *mut mln_string_t,
    mut val: *mut mln_string_t,
) -> libc::c_int {
    if http.is_null() || key.is_null() {
        return 2 as libc::c_int;
    }
    let mut header_fields: *mut mln_hash_t = (*http).header_fields;
    if header_fields.is_null() {
        return 2 as libc::c_int;
    }
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut dup_key: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut dup_val: *mut mln_string_t = 0 as *mut mln_string_t;
    dup_key = mln_string_pool_dup(pool, key);
    if dup_key.is_null() {
        return 2 as libc::c_int;
    }
    dup_val = mln_string_pool_dup(pool, val);
    if dup_val.is_null() {
        let mut __s: *mut mln_string_t = dup_key;
        if !__s.is_null() {
            let ref mut fresh15 = (*__s).ref_0();
            let fresh16 = *fresh15;
            *fresh15 = (*fresh15).wrapping_sub(1);
            if fresh16 <= 1 as libc::c_int as libc::c_ulong {
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
        return 2 as libc::c_int;
    }
    let mut ret: libc::c_int = mln_hash_update(
        header_fields,
        &mut dup_key as *mut *mut mln_string_t as *mut libc::c_void,
        &mut dup_val as *mut *mut mln_string_t as *mut libc::c_void,
    );
    let mut __s: *mut mln_string_t = dup_key;
    if !__s.is_null() {
        let ref mut fresh17 = (*__s).ref_0();
        let fresh18 = *fresh17;
        *fresh17 = (*fresh17).wrapping_sub(1);
        if fresh18 <= 1 as libc::c_int as libc::c_ulong {
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
    let mut __s: *mut mln_string_t = dup_val;
    if !__s.is_null() {
        let ref mut fresh19 = (*__s).ref_0();
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_sub(1);
        if fresh20 <= 1 as libc::c_int as libc::c_ulong {
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
    if ret < 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_field_get(
    mut http: *mut mln_http_t,
    mut key: *mut mln_string_t,
) -> *mut mln_string_t {
    if http.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut header_fields: *mut mln_hash_t = (*http).header_fields;
    if header_fields.is_null() {
        return 0 as *mut mln_string_t;
    }
    return mln_hash_search(header_fields, key as *mut libc::c_void) as *mut mln_string_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_field_iterator(
    mut http: *mut mln_http_t,
    mut key: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut ctx: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut size: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut cnt: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut header: *mut mln_hash_t = (*http).header_fields;
    loop {
        val = mln_hash_search_iterator(header, key as *mut libc::c_void, &mut ctx)
            as *mut mln_string_t;
        if !val.is_null() {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    ((*val).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as mln_u32_t as mln_u32_t;
            cnt = cnt.wrapping_add(1);
            cnt;
        }
        if ctx.is_null() {
            break;
        }
    }
    if cnt < 1 as libc::c_int as libc::c_uint {
        return 0 as *mut mln_string_t;
    }
    buf = mln_alloc_m(
        pool,
        size.wrapping_add(1 as libc::c_int as libc::c_uint) as mln_size_t,
    ) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    size = 0 as libc::c_int as mln_u32_t;
    loop {
        val = mln_hash_search_iterator(header, key as *mut libc::c_void, &mut ctx)
            as *mut mln_string_t;
        if !val.is_null() {
            memcpy(
                buf.offset(size as isize) as *mut libc::c_void,
                (*val).data as *const libc::c_void,
                (*val).len,
            );
            size = (size as libc::c_ulong).wrapping_add((*val).len) as mln_u32_t
                as mln_u32_t;
            let fresh21 = cnt;
            cnt = cnt.wrapping_sub(1);
            if fresh21 > 1 as libc::c_int as libc::c_uint {
                let fresh22 = size;
                size = size.wrapping_add(1);
                *buf.offset(fresh22 as isize) = ',' as i32 as libc::c_uchar;
            }
        }
        if ctx.is_null() {
            break;
        }
    }
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        tmp.data = buf;
        tmp.len = size as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    val = mln_string_pool_dup(pool, &mut tmp);
    mln_alloc_free(buf as *mut libc::c_void);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_field_remove(
    mut http: *mut mln_http_t,
    mut key: *mut mln_string_t,
) {
    if http.is_null() || key.is_null() {
        return;
    }
    let mut val: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut header: *mut mln_hash_t = (*http).header_fields;
    loop {
        val = mln_hash_search(header, key as *mut libc::c_void) as *mut mln_string_t;
        if val.is_null() {
            break;
        }
        mln_hash_remove(header, key as *mut libc::c_void, M_HASH_F_KV);
    };
}
#[inline]
unsafe extern "C" fn mln_http_atou(
    mut s: *mut mln_string_t,
    mut status: *mut mln_u32_t,
) -> libc::c_int {
    let mut st: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = ((*s).data).offset((*s).len as isize);
    p = (*s).data;
    while p < end {
        if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32) {
            return 2 as libc::c_int;
        }
        st = (st as libc::c_uint).wrapping_mul(10 as libc::c_int as libc::c_uint)
            as mln_u32_t as mln_u32_t;
        st = (st as libc::c_uint)
            .wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_uint) as mln_u32_t
            as mln_u32_t;
        p = p.offset(1);
        p;
    }
    *status = st;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_init(
    mut connection: *mut mln_tcp_conn_t,
    mut data: *mut libc::c_void,
    mut body_handler: mln_http_handler,
) -> *mut mln_http_t {
    if connection.is_null() {
        return 0 as *mut mln_http_t;
    }
    let mut http: *mut mln_http_t = 0 as *mut mln_http_t;
    let mut hattr: mln_hash_attr = mln_hash_attr {
        hash: None,
        cmp: None,
        key_freer: None,
        val_freer: None,
        len_base: 0,
        expandable_calc_prime: [0; 1],
        c2rust_padding: [0; 7],
        pool: 0 as *mut libc::c_void,
        pool_alloc: None,
        pool_free: None,
    };
    let mut pool: *mut mln_alloc_t = (*connection).pool;
    if pool.is_null() {
        return 0 as *mut mln_http_t;
    }
    http = mln_alloc_m(pool, ::core::mem::size_of::<mln_http_t>() as libc::c_ulong)
        as *mut mln_http_t;
    if http.is_null() {
        return 0 as *mut mln_http_t;
    }
    (*http).connection = connection;
    (*http).pool = pool;
    hattr.pool = pool as *mut libc::c_void;
    hattr
        .pool_alloc = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut mln_alloc_t, mln_size_t) -> *mut libc::c_void,
        >,
        hash_pool_alloc_handler,
    >(
        Some(
            mln_alloc_m
                as unsafe extern "C" fn(
                    *mut mln_alloc_t,
                    mln_size_t,
                ) -> *mut libc::c_void,
        ),
    );
    hattr
        .pool_free = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        hash_pool_free_handler,
    >(Some(mln_alloc_free as unsafe extern "C" fn(*mut libc::c_void) -> ()));
    hattr
        .hash = Some(
        mln_http_hash_calc
            as unsafe extern "C" fn(*mut mln_hash_t, *mut libc::c_void) -> mln_u64_t,
    );
    hattr
        .cmp = Some(
        mln_http_hash_cmp
            as unsafe extern "C" fn(
                *mut mln_hash_t,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
    );
    hattr
        .key_freer = Some(
        mln_http_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    hattr
        .val_freer = Some(
        mln_http_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    hattr.len_base = 31 as libc::c_int as mln_u64_t;
    hattr.set_expandable(0 as libc::c_int as mln_u32_t);
    hattr.set_calc_prime(0 as libc::c_int as mln_u32_t);
    (*http).header_fields = mln_hash_new(&mut hattr);
    if ((*http).header_fields).is_null() {
        mln_alloc_free(http as *mut libc::c_void);
        return 0 as *mut mln_http_t;
    }
    (*http).body_tail = 0 as *mut mln_chain_t;
    (*http).body_head = (*http).body_tail;
    (*http).body_handler = body_handler;
    (*http).data = data;
    (*http).uri = 0 as *mut mln_string_t;
    (*http).args = 0 as *mut mln_string_t;
    (*http).response_msg = 0 as *mut mln_string_t;
    (*http).error = 200 as libc::c_int as mln_u32_t;
    (*http).status = 200 as libc::c_int as mln_u32_t;
    (*http).method = 0 as libc::c_int as mln_u32_t;
    (*http).version = 0 as libc::c_int as mln_u32_t;
    (*http).set_type_0(0 as libc::c_int as mln_u32_t);
    (*http).set_done(0 as libc::c_int as mln_u32_t);
    return http;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_destroy(mut http: *mut mln_http_t) {
    if http.is_null() {
        return;
    }
    if !((*http).header_fields).is_null() {
        mln_hash_free((*http).header_fields, M_HASH_F_KV);
    }
    if !((*http).body_head).is_null() {
        mln_chain_pool_release_all((*http).body_head);
    }
    if !((*http).uri).is_null() {
        let mut __s: *mut mln_string_t = (*http).uri;
        if !__s.is_null() {
            let ref mut fresh23 = (*__s).ref_0();
            let fresh24 = *fresh23;
            *fresh23 = (*fresh23).wrapping_sub(1);
            if fresh24 <= 1 as libc::c_int as libc::c_ulong {
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
    if !((*http).args).is_null() {
        let mut __s: *mut mln_string_t = (*http).args;
        if !__s.is_null() {
            let ref mut fresh25 = (*__s).ref_0();
            let fresh26 = *fresh25;
            *fresh25 = (*fresh25).wrapping_sub(1);
            if fresh26 <= 1 as libc::c_int as libc::c_ulong {
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
    if !((*http).response_msg).is_null() {
        let mut __s: *mut mln_string_t = (*http).response_msg;
        if !__s.is_null() {
            let ref mut fresh27 = (*__s).ref_0();
            let fresh28 = *fresh27;
            *fresh27 = (*fresh27).wrapping_sub(1);
            if fresh28 <= 1 as libc::c_int as libc::c_ulong {
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
    mln_alloc_free(http as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_reset(mut http: *mut mln_http_t) {
    if http.is_null() {
        return;
    }
    if !((*http).header_fields).is_null() {
        mln_hash_reset((*http).header_fields, M_HASH_F_KV);
    }
    if !((*http).body_head).is_null() {
        mln_chain_pool_release_all((*http).body_head);
        (*http).body_tail = 0 as *mut mln_chain_t;
        (*http).body_head = (*http).body_tail;
    }
    if !((*http).uri).is_null() {
        let mut __s: *mut mln_string_t = (*http).uri;
        if !__s.is_null() {
            let ref mut fresh29 = (*__s).ref_0();
            let fresh30 = *fresh29;
            *fresh29 = (*fresh29).wrapping_sub(1);
            if fresh30 <= 1 as libc::c_int as libc::c_ulong {
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
        (*http).uri = 0 as *mut mln_string_t;
    }
    if !((*http).args).is_null() {
        let mut __s: *mut mln_string_t = (*http).args;
        if !__s.is_null() {
            let ref mut fresh31 = (*__s).ref_0();
            let fresh32 = *fresh31;
            *fresh31 = (*fresh31).wrapping_sub(1);
            if fresh32 <= 1 as libc::c_int as libc::c_ulong {
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
        (*http).args = 0 as *mut mln_string_t;
    }
    if !((*http).response_msg).is_null() {
        let mut __s: *mut mln_string_t = (*http).response_msg;
        if !__s.is_null() {
            let ref mut fresh33 = (*__s).ref_0();
            let fresh34 = *fresh33;
            *fresh33 = (*fresh33).wrapping_sub(1);
            if fresh34 <= 1 as libc::c_int as libc::c_ulong {
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
        (*http).response_msg = 0 as *mut mln_string_t;
    }
    (*http).error = 200 as libc::c_int as mln_u32_t;
    (*http).status = 200 as libc::c_int as mln_u32_t;
    (*http).method = 0 as libc::c_int as mln_u32_t;
    (*http).version = 0 as libc::c_int as mln_u32_t;
    (*http).set_type_0(0 as libc::c_int as mln_u32_t);
    (*http).set_done(0 as libc::c_int as mln_u32_t);
}
unsafe extern "C" fn mln_http_hash_free(mut data: *mut libc::c_void) {
    let mut __s: *mut mln_string_t = data as *mut mln_string_t;
    if !__s.is_null() {
        let ref mut fresh35 = (*__s).ref_0();
        let fresh36 = *fresh35;
        *fresh35 = (*fresh35).wrapping_sub(1);
        if fresh36 <= 1 as libc::c_int as libc::c_ulong {
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
unsafe extern "C" fn mln_http_hash_calc(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
) -> mln_u64_t {
    let mut index: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut s: *mut mln_string_t = key as *mut mln_string_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = ((*s).data).offset((*s).len as isize);
    p = (*s).data;
    while p < end {
        index = (index as libc::c_ulong)
            .wrapping_add(
                (*p as mln_u64_t).wrapping_mul(3 as libc::c_int as libc::c_ulong),
            ) as mln_u64_t as mln_u64_t;
        p = p.offset(1);
        p;
    }
    return index.wrapping_rem((*h).len);
}
unsafe extern "C" fn mln_http_hash_cmp(
    mut h: *mut mln_hash_t,
    mut key1: *mut libc::c_void,
    mut key2: *mut libc::c_void,
) -> libc::c_int {
    return (mln_string_strcasecmp(key1 as *mut mln_string_t, key2 as *mut mln_string_t)
        == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_http_dump(mut http: *mut mln_http_t) {
    let mut rc: libc::c_int = 1 as libc::c_int;
    printf(b"HTTP Dump:\n\0" as *const u8 as *const libc::c_char);
    if http.is_null() {
        return;
    }
    if !((*http).uri).is_null() {
        printf(b"\tURI:[\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        rc = write(
            1 as libc::c_int,
            (*(*http).uri).data as *const libc::c_void,
            (*(*http).uri).len,
        ) as libc::c_int;
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
    }
    if !((*http).args).is_null() {
        printf(b"\tARGS:[\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        rc = write(
            1 as libc::c_int,
            (*(*http).args).data as *const libc::c_void,
            (*(*http).args).len,
        ) as libc::c_int;
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
    }
    if !((*http).response_msg).is_null() {
        printf(b"\tRESPONSE_MSG:[\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        rc = write(
            1 as libc::c_int,
            (*(*http).response_msg).data as *const libc::c_void,
            (*(*http).response_msg).len,
        ) as libc::c_int;
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\tstatus_code:%u\n\0" as *const u8 as *const libc::c_char, (*http).status);
    printf(b"\tMethod_code:%u\n\0" as *const u8 as *const libc::c_char, (*http).method);
    printf(
        b"\tversion_code:%u\n\0" as *const u8 as *const libc::c_char,
        (*http).version,
    );
    printf(
        b"\ttype_code:%u\n\0" as *const u8 as *const libc::c_char,
        (*http).type_0() as libc::c_int,
    );
    printf(b"\tfields:\n\0" as *const u8 as *const libc::c_char);
    if rc <= 0 as libc::c_int {
        rc = 1 as libc::c_int;
    }
    mln_hash_iterate(
        (*http).header_fields,
        Some(
            mln_http_dump_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_hash_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn mln_http_dump_iterate_handler(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    printf(
        b"\t\tkey:[%s] value:[%s]\n\0" as *const u8 as *const libc::c_char,
        (*(key as *mut mln_string_t)).data as *mut libc::c_char,
        if val.is_null() {
            b"NULL\0" as *const u8 as *const libc::c_char
        } else {
            (*(val as *mut mln_string_t)).data as *mut libc::c_char
                as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    http_version = [
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"HTTP/1.0\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"HTTP/1.1\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
    ];
    http_method = [
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"GET\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"POST\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"HEAD\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"PUT\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"DELETE\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"TRACE\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"CONNECT\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"OPTIONS\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
    ];
    mln_http_status = [
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Continue\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 9]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"100\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 100 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Switching Protocols\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 20]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"101\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 101 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Processing\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"102\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 102 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"OK\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 3]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"200\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 200 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Created\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 8]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"201\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 201 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Accepted\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 9]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"202\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 202 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Non-Authoritative Information\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 30]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"203\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 203 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"No Content\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"204\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 204 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Reset Content\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 14]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"205\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 205 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Partial Content\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"206\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 206 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Multi-Status\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 13]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"207\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 207 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Multiple Choices\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 17]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"300\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 300 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Moved Permanently\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 18]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"301\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 301 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Move temporarily\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 17]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"302\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 302 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"See Other\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"303\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 303 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Not Modified\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 13]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"304\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 304 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Use Proxy\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"305\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 305 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Switch Proxy\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 13]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"306\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 306 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Temporary Redirect\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 19]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"307\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 307 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Bad Request\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 12]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"400\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 400 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Unauthorized\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 13]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"401\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 401 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Payment Required\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 17]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"402\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 402 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Forbidden\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"403\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 403 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Not Found\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"404\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 404 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Method Not Allowed\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 19]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"405\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 405 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Not Acceptable\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 15]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"406\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 406 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Proxy Authentication Required\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 30]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"407\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 407 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Request Timeout\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"408\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 408 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Conflict\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 9]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"409\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 409 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Gone\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 5]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"410\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 410 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Length Required\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"411\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 411 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Precondition Failed\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 20]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"412\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 412 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Request Entity Too Large\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 25]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"413\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 413 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Request-URI Too Long\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 21]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"414\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 414 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Unsupported Media Type\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 23]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"415\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 415 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Requested Range Not Satisfiable\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 32]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"416\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 416 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Expectation Failed\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 19]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"417\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 417 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"There are too many connections from your internet address\0"
                            as *const u8 as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 58]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"421\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 421 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Unprocessable Entity\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 21]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"422\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 422 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Locked\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 7]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"423\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 423 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Failed Dependency\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 18]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"424\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 424 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Unordered Collection\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 21]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"425\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 425 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Upgrade Required\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 17]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"426\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 426 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Retry With\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"449\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 449 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Internal Server Error\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 22]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"500\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 500 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Not Implemented\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"501\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 501 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Bad Gateway\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 12]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"502\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 502 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Service Unavailable\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 20]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"503\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 503 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Gateway Timeout\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"504\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 504 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"HTTP Version Not Supported\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 27]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"505\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 505 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Variant Also Negotiates\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 24]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"506\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 506 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Insufficient Storage\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 21]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"507\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 507 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Bandwidth Limit Exceeded\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 25]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"509\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 509 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Not Extended\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 13]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"510\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 510 as libc::c_int as mln_u32_t,
            };
            init
        },
        {
            let mut init = mln_http_map_t {
                msg_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"Unparseable Response Headers\0" as *const u8
                            as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 29]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code_str: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"600\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                code: 600 as libc::c_int as mln_u32_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
