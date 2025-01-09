use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mln_string_pool_dup(
        pool: *mut mln_alloc_t,
        str: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_string_strcasecmp(
        s1: *mut mln_string_t,
        s2: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_string_slice(
        s: *mut mln_string_t,
        sep_array: *const libc::c_char,
    ) -> *mut mln_string_t;
    fn mln_string_slice_free(array: *mut mln_string_t);
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_chain_pool_release_all(c: *mut mln_chain_t);
    fn mln_hash_new(attr: *mut mln_hash_attr) -> *mut mln_hash_t;
    fn mln_hash_free(h: *mut mln_hash_t, flg: mln_hash_flag_t);
    fn mln_hash_search(h: *mut mln_hash_t, key: *mut libc::c_void) -> *mut libc::c_void;
    fn mln_hash_update(
        h: *mut mln_hash_t,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_iterate(
        h: *mut mln_hash_t,
        handler: hash_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_reset(h: *mut mln_hash_t, flg: mln_hash_flag_t);
    fn mln_http_reset(http: *mut mln_http_t);
    fn mln_http_generate(
        http: *mut mln_http_t,
        out_head: *mut *mut mln_chain_t,
        out_tail: *mut *mut mln_chain_t,
    ) -> libc::c_int;
    fn mln_http_field_set(
        http: *mut mln_http_t,
        key: *mut mln_string_t,
        val: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_http_field_get(
        http: *mut mln_http_t,
        key: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_http_field_iterator(
        http: *mut mln_http_t,
        key: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mln_array_new(
        free_0: array_free,
        size: mln_size_t,
        nalloc: mln_size_t,
    ) -> *mut mln_array_t;
    fn mln_reg_match(
        exp: *mut mln_string_t,
        text: *mut mln_string_t,
        matches: *mut mln_reg_match_result_t,
    ) -> libc::c_int;
    fn mln_array_free(arr: *mut mln_array_t);
    fn mln_sha1_init(s: *mut mln_sha1_t);
    fn mln_sha1_calc(
        s: *mut mln_sha1_t,
        input: mln_u8ptr_t,
        len: mln_uauto_t,
        is_last: mln_u32_t,
    );
    fn mln_sha1_tobytes(s: *mut mln_sha1_t, buf: mln_u8ptr_t, len: mln_u32_t);
    fn mln_base64_pool_encode(
        pool: *mut mln_alloc_t,
        in_0: mln_u8ptr_t,
        inlen: mln_uauto_t,
        out: *mut mln_u8ptr_t,
        outlen: *mut mln_uauto_t,
    ) -> libc::c_int;
    fn mln_base64_pool_free(data: mln_u8ptr_t);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type mln_u8_t = libc::c_uchar;
pub type mln_u16_t = libc::c_ushort;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_websocket_s {
    pub http: *mut mln_http_t,
    pub pool: *mut mln_alloc_t,
    pub connection: *mut mln_tcp_conn_t,
    pub fields: *mut mln_hash_t,
    pub uri: *mut mln_string_t,
    pub args: *mut mln_string_t,
    pub key: *mut mln_string_t,
    pub data: *mut libc::c_void,
    pub content: *mut libc::c_void,
    pub extension_handler: mln_ws_extension_handle,
    pub content_len: mln_u64_t,
    #[bitfield(name = "content_free", ty = "mln_u16_t", bits = "0..=0")]
    #[bitfield(name = "fin", ty = "mln_u16_t", bits = "1..=1")]
    #[bitfield(name = "rsv1", ty = "mln_u16_t", bits = "2..=2")]
    #[bitfield(name = "rsv2", ty = "mln_u16_t", bits = "3..=3")]
    #[bitfield(name = "rsv3", ty = "mln_u16_t", bits = "4..=4")]
    #[bitfield(name = "opcode", ty = "mln_u16_t", bits = "5..=8")]
    #[bitfield(name = "mask", ty = "mln_u16_t", bits = "9..=9")]
    #[bitfield(name = "padding", ty = "mln_u16_t", bits = "10..=15")]
    pub content_free_fin_rsv1_rsv2_rsv3_opcode_mask_padding: [u8; 2],
    pub status: mln_u16_t,
    pub masking_key: mln_u32_t,
}
pub type mln_ws_extension_handle = Option::<
    unsafe extern "C" fn(*mut mln_websocket_t) -> libc::c_int,
>;
pub type mln_websocket_t = mln_websocket_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_sha1_t {
    pub H0: mln_u32_t,
    pub H1: mln_u32_t,
    pub H2: mln_u32_t,
    pub H3: mln_u32_t,
    pub H4: mln_u32_t,
    pub length: mln_u64_t,
    pub pos: mln_u32_t,
    pub buf: [mln_u8_t; 64],
}
pub type mln_array_t = mln_reg_match_result_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_reg_match_result_t {
    pub elts: *mut libc::c_void,
    pub size: mln_size_t,
    pub nalloc: mln_size_t,
    pub nelts: mln_size_t,
    pub pool: *mut libc::c_void,
    pub pool_alloc: array_pool_alloc_handler,
    pub pool_free: array_pool_free_handler,
    pub free: array_free,
}
pub type array_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type array_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type array_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
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
                    current_block = 1374665086854173366;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                1374665086854173366 => {}
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
        current_block_8 = 3821153412726802155;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 3821153412726802155;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        3821153412726802155 => {
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
pub unsafe extern "C" fn mln_websocket_init(
    mut ws: *mut mln_websocket_t,
    mut http: *mut mln_http_t,
) -> libc::c_int {
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
    hattr.pool = (*http).pool as *mut libc::c_void;
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
        mln_websocket_hash_calc
            as unsafe extern "C" fn(*mut mln_hash_t, *mut libc::c_void) -> mln_u64_t,
    );
    hattr
        .cmp = Some(
        mln_websocket_hash_cmp
            as unsafe extern "C" fn(
                *mut mln_hash_t,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
    );
    hattr
        .key_freer = Some(
        mln_websocket_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    hattr
        .val_freer = Some(
        mln_websocket_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    hattr.len_base = 37 as libc::c_int as mln_u64_t;
    hattr.set_expandable(0 as libc::c_int as mln_u32_t);
    hattr.set_calc_prime(0 as libc::c_int as mln_u32_t);
    (*ws).http = http;
    (*ws).pool = (*http).pool;
    (*ws).connection = (*http).connection;
    (*ws).fields = mln_hash_new(&mut hattr);
    if ((*ws).fields).is_null() {
        return -(1 as libc::c_int);
    }
    (*ws).key = 0 as *mut mln_string_t;
    (*ws).args = (*ws).key;
    (*ws).uri = (*ws).args;
    (*ws).data = 0 as *mut libc::c_void;
    (*ws).content = 0 as *mut libc::c_void;
    (*ws).extension_handler = None;
    (*ws).content_len = 0 as libc::c_int as mln_u64_t;
    (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    (*ws).set_fin(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2((*ws).rsv3());
    (*ws).set_rsv1((*ws).rsv2());
    (*ws).set_opcode(0 as libc::c_int as mln_u16_t);
    (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    (*ws).status = 0 as libc::c_int as mln_u16_t;
    (*ws).masking_key = 0 as libc::c_int as mln_u32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_websocket_hash_calc(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
) -> mln_u64_t {
    let mut k: *mut mln_string_t = key as *mut mln_string_t;
    let mut index: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = ((*k).data).offset((*k).len as isize);
    p = (*k).data;
    while p < end {
        index = (index as libc::c_ulong)
            .wrapping_add((*p as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
            as mln_u64_t as mln_u64_t;
        p = p.offset(1);
        p;
    }
    return index.wrapping_rem((*h).len);
}
unsafe extern "C" fn mln_websocket_hash_cmp(
    mut h: *mut mln_hash_t,
    mut key1: *mut libc::c_void,
    mut key2: *mut libc::c_void,
) -> libc::c_int {
    return (mln_string_strcasecmp(key1 as *mut mln_string_t, key2 as *mut mln_string_t)
        == 0) as libc::c_int;
}
unsafe extern "C" fn mln_websocket_hash_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut __s: *mut mln_string_t = data as *mut mln_string_t;
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
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_new(
    mut http: *mut mln_http_t,
) -> *mut mln_websocket_t {
    let mut ws: *mut mln_websocket_t = mln_alloc_m(
        (*http).pool,
        ::core::mem::size_of::<mln_websocket_t>() as libc::c_ulong,
    ) as *mut mln_websocket_t;
    if ws.is_null() {
        return 0 as *mut mln_websocket_t;
    }
    if mln_websocket_init(ws, http) < 0 as libc::c_int {
        free(ws as *mut libc::c_void);
        return 0 as *mut mln_websocket_t;
    }
    return ws;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_destroy(mut ws: *mut mln_websocket_t) {
    if ws.is_null() {
        return;
    }
    if !((*ws).fields).is_null() {
        mln_hash_free((*ws).fields, M_HASH_F_KV);
    }
    if !((*ws).uri).is_null() {
        let mut __s: *mut mln_string_t = (*ws).uri;
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
    }
    if !((*ws).args).is_null() {
        let mut __s: *mut mln_string_t = (*ws).args;
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
    }
    if !((*ws).key).is_null() {
        let mut __s: *mut mln_string_t = (*ws).key;
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
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_free(mut ws: *mut mln_websocket_t) {
    if ws.is_null() {
        return;
    }
    mln_websocket_destroy(ws);
    mln_alloc_free(ws as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_reset(mut ws: *mut mln_websocket_t) {
    if !((*ws).fields).is_null() {
        mln_hash_reset((*ws).fields, M_HASH_F_KV);
    }
    if !((*ws).uri).is_null() {
        let mut __s: *mut mln_string_t = (*ws).uri;
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
        (*ws).uri = 0 as *mut mln_string_t;
    }
    if !((*ws).args).is_null() {
        let mut __s: *mut mln_string_t = (*ws).args;
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
        (*ws).args = 0 as *mut mln_string_t;
    }
    if !((*ws).key).is_null() {
        let mut __s: *mut mln_string_t = (*ws).key;
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
        (*ws).key = 0 as *mut mln_string_t;
    }
    (*ws).data = 0 as *mut libc::c_void;
    if (*ws).content_free() != 0 {
        (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
        mln_alloc_free((*ws).content);
        (*ws).content = 0 as *mut libc::c_void;
    } else {
        (*ws).content = 0 as *mut libc::c_void;
    }
    (*ws).extension_handler = None;
    (*ws).content_len = 0 as libc::c_int as mln_u64_t;
    (*ws).set_fin(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2((*ws).rsv3());
    (*ws).set_rsv1((*ws).rsv2());
    (*ws).set_opcode(0 as libc::c_int as mln_u16_t);
    (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    (*ws).status = 0 as libc::c_int as mln_u16_t;
    (*ws).masking_key = 0 as libc::c_int as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_is_websocket(
    mut http: *mut mln_http_t,
) -> libc::c_int {
    let mut key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Upgrade\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut val: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"websocket\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut tmp: *mut mln_string_t = mln_http_field_get(http, &mut key);
    if tmp.is_null() || mln_string_strcasecmp(&mut val, tmp) != 0 {
        return 2 as libc::c_int;
    }
    if (*http).type_0() as libc::c_int != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_validate(
    mut ws: *mut mln_websocket_t,
) -> libc::c_int {
    let mut http: *mut mln_http_t = (*ws).http;
    if (*http).status != 101 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int;
    }
    let mut upgrade_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Upgrade\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut upgrade_val: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"websocket\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut connection_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Connection\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut tmp: *mut mln_string_t = 0 as *mut mln_string_t;
    tmp = mln_http_field_get(http, &mut upgrade_key);
    if tmp.is_null() || mln_string_strcasecmp(tmp, &mut upgrade_val) != 0 {
        return 2 as libc::c_int;
    }
    tmp = mln_http_field_get(http, &mut connection_key);
    if tmp.is_null() || mln_string_strcasecmp(tmp, &mut upgrade_key) != 0 {
        return 2 as libc::c_int;
    }
    let mut ret: libc::c_int = mln_websocket_validate_accept(http, (*ws).key);
    if ret != 0 as libc::c_int {
        return ret;
    }
    if (*http).type_0() as libc::c_int != 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_websocket_validate_accept(
    mut http: *mut mln_http_t,
    mut wskey: *mut mln_string_t,
) -> libc::c_int {
    let mut s: mln_sha1_t = mln_sha1_t {
        H0: 0,
        H1: 0,
        H2: 0,
        H3: 0,
        H4: 0,
        length: 0,
        pos: 0,
        buf: [0; 64],
    };
    mln_sha1_init(&mut s);
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Accept\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut val: *mut mln_string_t = mln_http_field_get(http, &mut key);
    if val.is_null() {
        return 2 as libc::c_int;
    }
    let mut guid: [libc::c_char; 37] = *::core::mem::transmute::<
        &[u8; 37],
        &mut [libc::c_char; 37],
    >(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0");
    let mut buf: mln_u8ptr_t = mln_alloc_m(
        pool,
        ((*wskey).len)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong),
    ) as mln_u8ptr_t;
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(buf as *mut libc::c_void, (*wskey).data as *const libc::c_void, (*wskey).len);
    memcpy(
        buf.offset((*wskey).len as isize) as *mut libc::c_void,
        guid.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong,
    );
    mln_sha1_calc(
        &mut s,
        buf,
        ((*wskey).len)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as mln_u32_t,
    );
    mln_alloc_free(buf as *mut libc::c_void);
    buf = 0 as mln_u8ptr_t;
    let mut tmp: [mln_u8_t; 20] = [0; 20];
    let mut len: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    mln_sha1_tobytes(
        &mut s,
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong as mln_u32_t,
    );
    if mln_base64_pool_encode(
        pool,
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong,
        &mut buf,
        &mut len,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    ({
        key.data = buf;
        key.len = len;
        key.set_data_ref(1 as libc::c_int as mln_uauto_t);
        key.set_pool(0 as libc::c_int as mln_uauto_t);
        key.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut key;
        &mut key
    });
    let mut ret: libc::c_int = mln_string_strcasecmp(&mut key, val);
    mln_base64_pool_free(buf);
    return if ret != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_set_field(
    mut ws: *mut mln_websocket_t,
    mut key: *mut mln_string_t,
    mut val: *mut mln_string_t,
) -> libc::c_int {
    let mut dup_key: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut dup_val: *mut mln_string_t = 0 as *mut mln_string_t;
    dup_key = mln_string_pool_dup((*ws).pool, key);
    if dup_key.is_null() {
        return 1 as libc::c_int;
    }
    dup_val = mln_string_pool_dup((*ws).pool, val);
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
        return 1 as libc::c_int;
    }
    let mut ret: libc::c_int = mln_hash_update(
        (*ws).fields,
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
    return if ret < 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_get_field(
    mut ws: *mut mln_websocket_t,
    mut key: *mut mln_string_t,
) -> *mut mln_string_t {
    return mln_hash_search((*ws).fields, key as *mut libc::c_void) as *mut mln_string_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_match(
    mut ws: *mut mln_websocket_t,
) -> libc::c_int {
    if mln_hash_iterate(
        (*ws).fields,
        Some(
            mln_websocket_match_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_hash_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        (*ws).http as *mut libc::c_void,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_websocket_match_iterate_handler(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut res: *mut mln_reg_match_result_t = 0 as *mut mln_reg_match_result_t;
    let mut tmp: *mut mln_string_t = mln_http_field_get(
        data as *mut mln_http_t,
        key as *mut mln_string_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    res = mln_array_new(
        None,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
        1 as libc::c_int as mln_size_t,
    );
    if res.is_null() {
        return -(1 as libc::c_int);
    }
    if !val.is_null()
        && mln_reg_match(val as *mut mln_string_t, tmp, res) <= 0 as libc::c_int
    {
        mln_array_free(res);
        return -(1 as libc::c_int);
    }
    mln_array_free(res);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_handshake_response_generate(
    mut ws: *mut mln_websocket_t,
    mut chead: *mut *mut mln_chain_t,
    mut ctail: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut tmp: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut http: *mut mln_http_t = (*ws).http;
    let mut protocol_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Protocol\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut protocol_val: *mut mln_string_t = 0 as *mut mln_string_t;
    tmp = mln_http_field_iterator(http, &mut protocol_key);
    if !tmp.is_null() {
        protocol_val = mln_string_pool_dup((*ws).pool, tmp);
        if protocol_val.is_null() {
            return 1 as libc::c_int;
        }
    }
    let mut extension_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Extensions\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut extension_val: *mut mln_string_t = 0 as *mut mln_string_t;
    tmp = mln_http_field_iterator(http, &mut extension_key);
    if !tmp.is_null() {
        extension_val = mln_websocket_extension_tokens((*ws).pool, tmp);
    }
    let mut accept: *mut mln_string_t = mln_websocket_accept_field(http);
    if accept.is_null() {
        if !protocol_val.is_null() {
            let mut __s: *mut mln_string_t = protocol_val;
            if !__s.is_null() {
                let ref mut fresh21 = (*__s).ref_0();
                let fresh22 = *fresh21;
                *fresh21 = (*fresh21).wrapping_sub(1);
                if fresh22 <= 1 as libc::c_int as libc::c_ulong {
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
        if !extension_val.is_null() {
            let mut __s: *mut mln_string_t = extension_val;
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
        return 1 as libc::c_int;
    }
    mln_http_reset(http);
    (*http).status = 101 as libc::c_int as mln_u32_t;
    (*http).version = 1 as libc::c_int as mln_u32_t;
    (*http).set_type_0(2 as libc::c_int as mln_u32_t);
    (*http).body_handler = None;
    let mut upgrade_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Upgrade\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut upgrade_val: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"websocket\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut connection_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Connection\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut accept_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Accept\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    if !protocol_val.is_null() {
        if mln_http_field_set(http, &mut protocol_key, protocol_val) != 0 as libc::c_int
        {
            if !protocol_val.is_null() {
                let mut __s: *mut mln_string_t = protocol_val;
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
            if !extension_val.is_null() {
                let mut __s: *mut mln_string_t = extension_val;
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
            if !accept.is_null() {
                let mut __s: *mut mln_string_t = accept;
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
            }
            return 1 as libc::c_int;
        }
    }
    if !extension_val.is_null() {
        if mln_http_field_set(http, &mut extension_key, extension_val)
            != 0 as libc::c_int
        {
            if !extension_val.is_null() {
                let mut __s: *mut mln_string_t = extension_val;
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
            }
            if !accept.is_null() {
                let mut __s: *mut mln_string_t = accept;
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
            }
            return 1 as libc::c_int;
        }
    }
    if mln_http_field_set(http, &mut accept_key, accept) != 0 as libc::c_int {
        let mut __s: *mut mln_string_t = accept;
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
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut upgrade_key, &mut upgrade_val) != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut connection_key, &mut upgrade_key)
        != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if mln_hash_iterate(
        (*ws).fields,
        Some(
            mln_websocket_iterate_set_fields
                as unsafe extern "C" fn(
                    *mut mln_hash_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        http as *mut libc::c_void,
    ) < 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if mln_http_generate(http, chead, ctail) == 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_websocket_extension_tokens(
    mut pool: *mut mln_alloc_t,
    mut in_0: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut tmp: *mut mln_string_t = mln_string_pool_dup(pool, in_0);
    if tmp.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut array: *mut mln_string_t = mln_string_slice(
        tmp,
        b",\0" as *const u8 as *const libc::c_char,
    );
    if array.is_null() {
        let mut __s: *mut mln_string_t = tmp;
        if !__s.is_null() {
            let ref mut fresh37 = (*__s).ref_0();
            let fresh38 = *fresh37;
            *fresh37 = (*fresh37).wrapping_sub(1);
            if fresh38 <= 1 as libc::c_int as libc::c_ulong {
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
        return 0 as *mut mln_string_t;
    }
    let mut p: *mut mln_string_t = array;
    let mut pos: mln_s8ptr_t = 0 as *mut libc::c_char;
    let mut buf: mln_s8ptr_t = 0 as *mut libc::c_char;
    let mut size: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    while (*p).len != 0 {
        pos = strchr((*p).data as *mut libc::c_char, ';' as i32);
        if pos.is_null() {
            size = (size as libc::c_ulong)
                .wrapping_add(((*p).len).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as mln_uauto_t as mln_uauto_t;
        } else {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (pos.offset_from((*p).data as *mut libc::c_char) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                ) as mln_uauto_t as mln_uauto_t;
        }
        p = p.offset(1);
        p;
    }
    if size == 0 {
        return 0 as *mut mln_string_t;
    }
    size = size.wrapping_sub(1);
    size;
    buf = mln_alloc_m(pool, size) as mln_s8ptr_t;
    if buf.is_null() {
        mln_string_slice_free(array);
        let mut __s: *mut mln_string_t = tmp;
        if !__s.is_null() {
            let ref mut fresh39 = (*__s).ref_0();
            let fresh40 = *fresh39;
            *fresh39 = (*fresh39).wrapping_sub(1);
            if fresh40 <= 1 as libc::c_int as libc::c_ulong {
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
        return 0 as *mut mln_string_t;
    }
    size = 0 as libc::c_int as mln_uauto_t;
    p = array;
    while (*p).len != 0 {
        pos = strchr((*p).data as *mut libc::c_char, ';' as i32);
        if pos.is_null() {
            memcpy(
                buf.offset(size as isize) as *mut libc::c_void,
                (*p).data as *const libc::c_void,
                (*p).len,
            );
            size = (size as libc::c_ulong).wrapping_add((*p).len) as mln_uauto_t
                as mln_uauto_t;
        } else {
            memcpy(
                buf.offset(size as isize) as *mut libc::c_void,
                (*p).data as *const libc::c_void,
                pos.offset_from((*p).data as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            );
            size = (size as libc::c_ulong)
                .wrapping_add(
                    pos.offset_from((*p).data as *mut libc::c_char) as libc::c_long
                        as libc::c_ulong,
                ) as mln_uauto_t as mln_uauto_t;
        }
        let fresh41 = size;
        size = size.wrapping_add(1);
        *buf.offset(fresh41 as isize) = ',' as i32 as libc::c_char;
        p = p.offset(1);
        p;
    }
    size = size.wrapping_sub(1);
    *buf.offset(size as isize) = '0' as i32 as libc::c_char;
    mln_string_slice_free(array);
    let mut __s: *mut mln_string_t = tmp;
    if !__s.is_null() {
        let ref mut fresh42 = (*__s).ref_0();
        let fresh43 = *fresh42;
        *fresh42 = (*fresh42).wrapping_sub(1);
        if fresh43 <= 1 as libc::c_int as libc::c_ulong {
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
    let mut t: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        t.data = buf as mln_u8ptr_t;
        t.len = size;
        t.set_data_ref(1 as libc::c_int as mln_uauto_t);
        t.set_pool(0 as libc::c_int as mln_uauto_t);
        t.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut t;
        &mut t
    });
    tmp = mln_string_pool_dup(pool, &mut t);
    mln_alloc_free(buf as *mut libc::c_void);
    return tmp;
}
unsafe extern "C" fn mln_websocket_iterate_set_fields(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return if mln_http_field_set(
        data as *mut mln_http_t,
        key as *mut mln_string_t,
        val as *mut mln_string_t,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn mln_websocket_accept_field(
    mut http: *mut mln_http_t,
) -> *mut mln_string_t {
    let mut s: mln_sha1_t = mln_sha1_t {
        H0: 0,
        H1: 0,
        H2: 0,
        H3: 0,
        H4: 0,
        length: 0,
        pos: 0,
        buf: [0; 64],
    };
    mln_sha1_init(&mut s);
    let mut pool: *mut mln_alloc_t = (*http).pool;
    let mut key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Key\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut val: *mut mln_string_t = mln_http_field_get(http, &mut key);
    let mut guid: [libc::c_char; 37] = *::core::mem::transmute::<
        &[u8; 37],
        &mut [libc::c_char; 37],
    >(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0");
    let mut buf: mln_u8ptr_t = mln_alloc_m(
        pool,
        ((*val).len)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong),
    ) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    memcpy(buf as *mut libc::c_void, (*val).data as *const libc::c_void, (*val).len);
    memcpy(
        buf.offset((*val).len as isize) as *mut libc::c_void,
        guid.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong,
    );
    mln_sha1_calc(
        &mut s,
        buf,
        ((*val).len)
            .wrapping_add(::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as mln_u32_t,
    );
    mln_alloc_free(buf as *mut libc::c_void);
    buf = 0 as mln_u8ptr_t;
    let mut tmp: [mln_u8_t; 20] = [0; 20];
    let mut len: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    mln_sha1_tobytes(
        &mut s,
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong as mln_u32_t,
    );
    if mln_base64_pool_encode(
        pool,
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong,
        &mut buf,
        &mut len,
    ) < 0 as libc::c_int
    {
        return 0 as *mut mln_string_t;
    }
    ({
        key.data = buf;
        key.len = len;
        key.set_data_ref(1 as libc::c_int as mln_uauto_t);
        key.set_pool(0 as libc::c_int as mln_uauto_t);
        key.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut key;
        &mut key
    });
    val = mln_string_pool_dup(pool, &mut key);
    mln_base64_pool_free(buf);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_handshake_request_generate(
    mut ws: *mut mln_websocket_t,
    mut chead: *mut *mut mln_chain_t,
    mut ctail: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut http: *mut mln_http_t = (*ws).http;
    let mut pool: *mut mln_alloc_t = (*ws).pool;
    let mut dup_uri: *mut mln_string_t = 0 as *mut mln_string_t;
    if ((*ws).uri).is_null() {
        let mut tmp: mln_string_t = {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"/\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        };
        dup_uri = mln_string_pool_dup(pool, &mut tmp);
    } else {
        dup_uri = mln_string_pool_dup(pool, (*ws).uri);
    }
    if dup_uri.is_null() {
        return 1 as libc::c_int;
    }
    let mut dup_args: *mut mln_string_t = 0 as *mut mln_string_t;
    if !((*ws).args).is_null() {
        dup_args = mln_string_pool_dup(pool, (*ws).args);
        if dup_args.is_null() {
            let mut __s: *mut mln_string_t = dup_uri;
            if !__s.is_null() {
                let ref mut fresh44 = (*__s).ref_0();
                let fresh45 = *fresh44;
                *fresh44 = (*fresh44).wrapping_sub(1);
                if fresh45 <= 1 as libc::c_int as libc::c_ulong {
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
            return 1 as libc::c_int;
        }
    }
    mln_http_reset(http);
    (*http).method = 0 as libc::c_int as mln_u32_t;
    (*http).version = 1 as libc::c_int as mln_u32_t;
    (*http).set_type_0(1 as libc::c_int as mln_u32_t);
    (*http).uri = dup_uri;
    if !dup_args.is_null() {
        (*http).args = dup_args;
    }
    (*http).body_handler = None;
    let mut upgrade_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Upgrade\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut upgrade_val: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"websocket\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut connection_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Connection\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut version_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Version\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut version_val: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"13\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut key_key: mln_string_t = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"Sec-WebSocket-Key\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    let mut key_val: *mut mln_string_t = mln_websocket_client_handshake_key_generate(
        pool,
    );
    if key_val.is_null() {
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut key_key, key_val) < 0 as libc::c_int {
        let mut __s: *mut mln_string_t = key_val;
        if !__s.is_null() {
            let ref mut fresh46 = (*__s).ref_0();
            let fresh47 = *fresh46;
            *fresh46 = (*fresh46).wrapping_sub(1);
            if fresh47 <= 1 as libc::c_int as libc::c_ulong {
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
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut upgrade_key, &mut upgrade_val) < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut connection_key, &mut upgrade_key) < 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if mln_http_field_set(http, &mut version_key, &mut version_val) < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if mln_hash_iterate(
        (*ws).fields,
        Some(
            mln_websocket_iterate_set_fields
                as unsafe extern "C" fn(
                    *mut mln_hash_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        http as *mut libc::c_void,
    ) < 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if mln_http_generate(http, chead, ctail) == 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_websocket_client_handshake_key_generate(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_string_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut buf: [mln_u8_t; 16] = [0; 16];
    let mut i: mln_u32_t = 0;
    let mut tmp: mln_u32_t = 0;
    let mut out: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut outlen: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    let mut s: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut sdup: *mut mln_string_t = 0 as *mut mln_string_t;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    srand(
        (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec) as libc::c_uint,
    );
    i = 0 as libc::c_int as mln_u32_t;
    while i < 4 as libc::c_int as libc::c_uint {
        tmp = rand() as mln_u32_t;
        buf[i.wrapping_mul(4 as libc::c_int as libc::c_uint)
            as usize] = (tmp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        buf[i
            .wrapping_mul(4 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            as usize] = (tmp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        buf[i
            .wrapping_mul(4 as libc::c_int as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint)
            as usize] = (tmp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        buf[i
            .wrapping_mul(4 as libc::c_int as libc::c_uint)
            .wrapping_add(3 as libc::c_int as libc::c_uint)
            as usize] = (tmp & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        srand(tmp >> 16 as libc::c_int | tmp << 16 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    if mln_base64_pool_encode(
        pool,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 16]>() as libc::c_ulong,
        &mut out,
        &mut outlen,
    ) < 0 as libc::c_int
    {
        return 0 as *mut mln_string_t;
    }
    ({
        s.data = out;
        s.len = outlen;
        s.set_data_ref(1 as libc::c_int as mln_uauto_t);
        s.set_pool(0 as libc::c_int as mln_uauto_t);
        s.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut s;
        &mut s
    });
    sdup = mln_string_pool_dup(pool, &mut s);
    mln_base64_pool_free(out);
    return sdup;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_text_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_size_t,
    mut flags: mln_u32_t,
) -> libc::c_int {
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && flags & 0x8 as libc::c_int as libc::c_uint != 0
    {
        return -(1 as libc::c_int);
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
        (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    }
    (*ws).content = buf as *mut libc::c_void;
    (*ws).content_len = len;
    (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_fin(0 as libc::c_int as mln_u16_t);
    }
    (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_opcode(0x1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_opcode(0 as libc::c_int as mln_u16_t);
    }
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
        (*ws).masking_key = mln_websocket_masking_key_generate();
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    return mln_websocket_generate(ws, out_cnode);
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_binary_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
    mut buf: *mut libc::c_void,
    mut len: mln_size_t,
    mut flags: mln_u32_t,
) -> libc::c_int {
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && flags & 0x8 as libc::c_int as libc::c_uint != 0
    {
        return -(1 as libc::c_int);
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
        (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    }
    (*ws).content = buf;
    (*ws).content_len = len;
    (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_fin(0 as libc::c_int as mln_u16_t);
    }
    (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_opcode(0x2 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_opcode(0 as libc::c_int as mln_u16_t);
    }
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
        (*ws).masking_key = mln_websocket_masking_key_generate();
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    return mln_websocket_generate(ws, out_cnode);
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_close_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
    mut reason: *mut libc::c_char,
    mut status: mln_u16_t,
    mut flags: mln_u32_t,
) -> libc::c_int {
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && flags & 0x8 as libc::c_int as libc::c_uint != 0
    {
        return -(1 as libc::c_int);
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
        (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    }
    (*ws).content = reason as *mut libc::c_void;
    if reason.is_null() {
        (*ws).content_len = 0 as libc::c_int as mln_u64_t;
    } else {
        (*ws).content_len = strlen(reason);
    }
    (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    (*ws).set_opcode(0x8 as libc::c_int as mln_u16_t);
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
        (*ws).masking_key = mln_websocket_masking_key_generate();
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    return mln_websocket_generate(ws, out_cnode);
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_ping_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
    mut flags: mln_u32_t,
) -> libc::c_int {
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && flags & 0x8 as libc::c_int as libc::c_uint != 0
    {
        return -(1 as libc::c_int);
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
    }
    (*ws).content = 0 as *mut libc::c_void;
    (*ws).content_len = 0 as libc::c_int as mln_u64_t;
    (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    (*ws).set_opcode(0x9 as libc::c_int as mln_u16_t);
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
        (*ws).masking_key = mln_websocket_masking_key_generate();
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    return mln_websocket_generate(ws, out_cnode);
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_pong_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
    mut flags: mln_u32_t,
) -> libc::c_int {
    if flags & 0x4 as libc::c_int as libc::c_uint != 0
        && flags & 0x8 as libc::c_int as libc::c_uint != 0
    {
        return -(1 as libc::c_int);
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
    }
    (*ws).content = 0 as *mut libc::c_void;
    (*ws).content_len = 0 as libc::c_int as mln_u64_t;
    (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    (*ws).set_opcode(0xa as libc::c_int as mln_u16_t);
    if flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
        (*ws).masking_key = mln_websocket_masking_key_generate();
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    return mln_websocket_generate(ws, out_cnode);
}
unsafe extern "C" fn mln_websocket_masking_key_generate() -> mln_u32_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp: mln_uauto_t = &mut tv as *mut timeval as mln_uauto_t;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    srand(
        (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec) as libc::c_uint,
    );
    return tmp as mln_u32_t | rand() as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_generate(
    mut ws: *mut mln_websocket_t,
    mut out_cnode: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut size: mln_size_t = 2 as libc::c_int as mln_size_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut pool: *mut mln_alloc_t = (*ws).pool;
    let mut payload_length: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    let mut content: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut clen: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut opcode: mln_u32_t = (*ws).opcode() as mln_u32_t;
    if ((*ws).extension_handler).is_some() {
        let mut ret: libc::c_int = ((*ws).extension_handler)
            .expect("non-null function pointer")(ws);
        if ret != 0 as libc::c_int {
            return ret;
        }
    }
    content = (*ws).content as mln_u8ptr_t;
    clen = (*ws).content_len;
    if content.is_null() && clen != 0 {
        return -(1 as libc::c_int);
    }
    if opcode == 0x8 as libc::c_int as libc::c_uint {
        clen = (clen as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as mln_u64_t as mln_u64_t;
    }
    if (opcode == 0x8 as libc::c_int as libc::c_uint
        || opcode == 0x9 as libc::c_int as libc::c_uint
        || opcode == 0xa as libc::c_int as libc::c_uint)
        && clen > 125 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if clen > 125 as libc::c_int as libc::c_ulong {
        if clen >> 16 as libc::c_int != 0 {
            size = (size as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong) as mln_size_t
                as mln_size_t;
            payload_length = 127 as libc::c_int as mln_u8_t;
        } else {
            size = (size as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as mln_size_t
                as mln_size_t;
            payload_length = 126 as libc::c_int as mln_u8_t;
        }
    } else {
        payload_length = clen as mln_u8_t;
    }
    size = (size as libc::c_ulong).wrapping_add(clen) as mln_size_t as mln_size_t;
    if (*ws).mask() != 0 {
        size = (size as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as mln_size_t as mln_size_t;
    }
    c = mln_chain_new(pool);
    if c.is_null() {
        return 1 as libc::c_int;
    }
    b = mln_buf_new(pool);
    if b.is_null() {
        mln_chain_pool_release(c);
        return 1 as libc::c_int;
    }
    (*c).buf = b;
    buf = mln_alloc_m(pool, size) as mln_u8ptr_t;
    if buf.is_null() {
        mln_chain_pool_release(c);
        return 1 as libc::c_int;
    }
    (*b).start = buf;
    (*b).pos = (*b).start;
    (*b).left_pos = (*b).pos;
    (*b).last = buf.offset(size as isize);
    (*b).end = (*b).last;
    (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
    (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
    if (*ws).fin() != 0 {
        (*b).set_last_in_chain(1 as libc::c_int as mln_u32_t);
    }
    *out_cnode = c;
    p = buf;
    *p = 0 as libc::c_int as libc::c_uchar;
    if (*ws).fin() != 0 {
        *p = (*p as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar;
    }
    if (*ws).rsv1() != 0 {
        *p = (*p as libc::c_int | 0x40 as libc::c_int) as libc::c_uchar;
    }
    if (*ws).rsv2() != 0 {
        *p = (*p as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar;
    }
    if (*ws).rsv3() != 0 {
        *p = (*p as libc::c_int | 0x10 as libc::c_int) as libc::c_uchar;
    }
    let fresh48 = p;
    p = p.offset(1);
    *fresh48 = (*fresh48 as libc::c_uint | opcode & 0xf as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *p = 0 as libc::c_int as libc::c_uchar;
    if (*ws).mask() != 0 {
        *p = (*p as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar;
    }
    let fresh49 = p;
    p = p.offset(1);
    *fresh49 = (*fresh49 as libc::c_int
        | payload_length as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
    if payload_length as libc::c_int == 126 as libc::c_int {
        let fresh50 = p;
        p = p.offset(1);
        *fresh50 = (clen >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = (clen & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    } else if payload_length as libc::c_int == 127 as libc::c_int {
        let fresh52 = p;
        p = p.offset(1);
        *fresh52 = (clen >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh53 = p;
        p = p.offset(1);
        *fresh53 = (clen >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh54 = p;
        p = p.offset(1);
        *fresh54 = (clen >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh55 = p;
        p = p.offset(1);
        *fresh55 = (clen >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh56 = p;
        p = p.offset(1);
        *fresh56 = (clen >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = (clen >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh58 = p;
        p = p.offset(1);
        *fresh58 = (clen >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh59 = p;
        p = p.offset(1);
        *fresh59 = (clen & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    }
    if (*ws).mask() != 0 {
        let mut tmpkey: [mln_u8_t; 4] = [0; 4];
        let mut i: mln_u32_t = 0;
        let mut j: mln_u32_t = 0;
        let mut m: mln_u32_t = (*ws).masking_key;
        let mut pc: mln_u8ptr_t = content;
        tmpkey[0 as libc::c_int
            as usize] = (m >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        let fresh60 = p;
        p = p.offset(1);
        *fresh60 = tmpkey[0 as libc::c_int as usize];
        tmpkey[1 as libc::c_int
            as usize] = (m >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        let fresh61 = p;
        p = p.offset(1);
        *fresh61 = tmpkey[1 as libc::c_int as usize];
        tmpkey[2 as libc::c_int
            as usize] = (m >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as mln_u8_t;
        let fresh62 = p;
        p = p.offset(1);
        *fresh62 = tmpkey[2 as libc::c_int as usize];
        tmpkey[3 as libc::c_int
            as usize] = (m & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        let fresh63 = p;
        p = p.offset(1);
        *fresh63 = tmpkey[3 as libc::c_int as usize];
        i = 0 as libc::c_int as mln_u32_t;
        if opcode == 0x8 as libc::c_int as libc::c_uint {
            let fresh64 = i;
            i = i.wrapping_add(1);
            let fresh65 = p;
            p = p.offset(1);
            *fresh65 = ((*ws).status as libc::c_int >> 8 as libc::c_int
                & 0xff as libc::c_int
                ^ tmpkey[fresh64.wrapping_rem(4 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int) as libc::c_uchar;
            let fresh66 = i;
            i = i.wrapping_add(1);
            let fresh67 = p;
            p = p.offset(1);
            *fresh67 = ((*ws).status as libc::c_int & 0xff as libc::c_int
                ^ tmpkey[fresh66.wrapping_rem(4 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int) as libc::c_uchar;
        }
        j = 0 as libc::c_int as mln_u32_t;
        while (i as libc::c_ulong) < clen {
            let fresh68 = p;
            p = p.offset(1);
            *fresh68 = (*pc.offset(j as isize) as libc::c_int
                ^ tmpkey[i.wrapping_rem(4 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int) as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
    } else {
        if opcode == 0x8 as libc::c_int as libc::c_uint {
            let fresh69 = p;
            p = p.offset(1);
            *fresh69 = ((*ws).status as libc::c_int >> 8 as libc::c_int
                & 0xff as libc::c_int) as libc::c_uchar;
            let fresh70 = p;
            p = p.offset(1);
            *fresh70 = ((*ws).status as libc::c_int & 0xff as libc::c_int)
                as libc::c_uchar;
            clen = (clen as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as mln_u64_t
                as mln_u64_t;
        }
        if !content.is_null() {
            memcpy(p as *mut libc::c_void, content as *const libc::c_void, clen);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_websocket_parse(
    mut ws: *mut mln_websocket_t,
    mut in_0: *mut *mut mln_chain_t,
) -> libc::c_int {
    let mut c: *mut mln_chain_t = *in_0;
    let mut p: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut end: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut content: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut len: mln_u64_t = 0;
    let mut i: mln_u64_t = 0;
    let mut tmp: mln_u64_t = 0;
    let mut masking_key: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut b1: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    let mut b2: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    i = 0 as libc::c_int as mln_u64_t;
    while !c.is_null() {
        if !(((*c).buf).is_null()
            || (if ((*c).buf).is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*(*c).buf).in_file() as libc::c_int != 0 {
                    (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                } else {
                    ((*(*c).buf).last).offset_from((*(*c).buf).left_pos) as libc::c_long
                })
            }) == 0 as libc::c_int as libc::c_long)
        {
            p = (*(*c).buf).left_pos;
            end = (*(*c).buf).end;
            while p < end {
                if i == 0 as libc::c_int as libc::c_ulong {
                    b1 = *p;
                    i = i.wrapping_add(1);
                    i;
                    p = p.offset(1);
                    p;
                } else {
                    let fresh71 = p;
                    p = p.offset(1);
                    b2 = *fresh71;
                    i = i.wrapping_add(1);
                    i;
                    break;
                }
            }
            if i >= 2 as libc::c_int as libc::c_ulong {
                break;
            }
        }
        c = (*c).next;
    }
    if i < 2 as libc::c_int as libc::c_ulong {
        return 3 as libc::c_int;
    }
    len = (b2 as libc::c_int & 0x7f as libc::c_int) as mln_u64_t;
    if len == 127 as libc::c_int as libc::c_ulong {
        tmp = p.offset_from(end) as libc::c_long as mln_u64_t;
        if tmp > 8 as libc::c_int as libc::c_ulong {
            tmp = 8 as libc::c_int as mln_u64_t;
        }
        len = 0 as libc::c_int as mln_u64_t;
        i = 0 as libc::c_int as mln_u64_t;
        loop {
            while i < tmp {
                let fresh72 = p;
                p = p.offset(1);
                len
                    |= ((*fresh72 as libc::c_int)
                        << ((7 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                            << 3 as libc::c_int)) as libc::c_ulong;
                i = i.wrapping_add(1);
                i;
            }
            if !(tmp < 8 as libc::c_int as libc::c_ulong) {
                break;
            }
            c = (*c).next;
            while !c.is_null() {
                if ((*c).buf).is_null()
                    || (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*(*c).buf).in_file() as libc::c_int != 0 {
                            (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                        } else {
                            ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                                as libc::c_long
                        })
                    }) == 0 as libc::c_int as libc::c_long
                {
                    c = (*c).next;
                } else {
                    p = (*(*c).buf).left_pos;
                    end = (*(*c).buf).end;
                    break;
                }
            }
            if c.is_null() {
                return 3 as libc::c_int;
            }
            tmp = if (8 as libc::c_int as libc::c_ulong).wrapping_sub(tmp)
                > p.offset_from(end) as libc::c_long as libc::c_ulong
            {
                tmp.wrapping_add(
                    p.offset_from(end) as libc::c_long as mln_u32_t as libc::c_ulong,
                )
            } else {
                8 as libc::c_int as libc::c_ulong
            };
        }
    } else if len == 126 as libc::c_int as libc::c_ulong {
        tmp = p.offset_from(end) as libc::c_long as mln_u64_t;
        if tmp > 2 as libc::c_int as libc::c_ulong {
            tmp = 2 as libc::c_int as mln_u64_t;
        }
        len = 0 as libc::c_int as mln_u64_t;
        i = 0 as libc::c_int as mln_u64_t;
        loop {
            while i < tmp {
                let fresh73 = p;
                p = p.offset(1);
                len
                    |= ((*fresh73 as libc::c_int)
                        << ((1 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                            << 3 as libc::c_int)) as libc::c_ulong;
                i = i.wrapping_add(1);
                i;
            }
            if !(tmp < 2 as libc::c_int as libc::c_ulong) {
                break;
            }
            c = (*c).next;
            while !c.is_null() {
                if ((*c).buf).is_null()
                    || (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*(*c).buf).in_file() as libc::c_int != 0 {
                            (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                        } else {
                            ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                                as libc::c_long
                        })
                    }) == 0 as libc::c_int as libc::c_long
                {
                    c = (*c).next;
                } else {
                    p = (*(*c).buf).left_pos;
                    end = (*(*c).buf).end;
                    break;
                }
            }
            if c.is_null() {
                return 3 as libc::c_int;
            }
            tmp = if (2 as libc::c_int as libc::c_ulong).wrapping_sub(tmp)
                > p.offset_from(end) as libc::c_long as libc::c_ulong
            {
                tmp.wrapping_add(
                    p.offset_from(end) as libc::c_long as mln_u32_t as libc::c_ulong,
                )
            } else {
                2 as libc::c_int as libc::c_ulong
            };
        }
    }
    if b2 as libc::c_int & 0x80 as libc::c_int != 0 {
        tmp = p.offset_from(end) as libc::c_long as mln_u64_t;
        if tmp > 4 as libc::c_int as libc::c_ulong {
            tmp = 4 as libc::c_int as mln_u64_t;
        }
        i = 0 as libc::c_int as mln_u64_t;
        loop {
            while i < tmp {
                let fresh74 = p;
                p = p.offset(1);
                masking_key
                    |= ((*fresh74 as libc::c_int)
                        << ((3 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                            << 3 as libc::c_int)) as libc::c_uint;
                i = i.wrapping_add(1);
                i;
            }
            if !(tmp < 4 as libc::c_int as libc::c_ulong) {
                break;
            }
            c = (*c).next;
            while !c.is_null() {
                if ((*c).buf).is_null()
                    || (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*(*c).buf).in_file() as libc::c_int != 0 {
                            (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                        } else {
                            ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                                as libc::c_long
                        })
                    }) == 0 as libc::c_int as libc::c_long
                {
                    c = (*c).next;
                } else {
                    p = (*(*c).buf).left_pos;
                    end = (*(*c).buf).end;
                    break;
                }
            }
            if c.is_null() {
                return 3 as libc::c_int;
            }
            tmp = if (4 as libc::c_int as libc::c_ulong).wrapping_sub(tmp)
                > p.offset_from(end) as libc::c_long as libc::c_ulong
            {
                tmp.wrapping_add(
                    p.offset_from(end) as libc::c_long as mln_u32_t as libc::c_ulong,
                )
            } else {
                4 as libc::c_int as libc::c_ulong
            };
        }
    }
    if len != 0 {
        if b1 as libc::c_int & 0xf as libc::c_int == 0x8 as libc::c_int
            && len > 1 as libc::c_int as libc::c_ulong
        {
            let mut status: mln_u16_t = 0 as libc::c_int as mln_u16_t;
            tmp = p.offset_from(end) as libc::c_long as mln_u64_t;
            if tmp > 2 as libc::c_int as libc::c_ulong {
                tmp = 2 as libc::c_int as mln_u64_t;
            }
            i = 0 as libc::c_int as mln_u64_t;
            loop {
                while i < tmp {
                    let fresh75 = p;
                    p = p.offset(1);
                    status = (status as libc::c_int
                        | (*fresh75 as libc::c_int)
                            << ((1 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                                << 3 as libc::c_int)) as mln_u16_t;
                    i = i.wrapping_add(1);
                    i;
                }
                if !(tmp < 2 as libc::c_int as libc::c_ulong) {
                    break;
                }
                c = (*c).next;
                while !c.is_null() {
                    if ((*c).buf).is_null()
                        || (if ((*c).buf).is_null() {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if (*(*c).buf).in_file() as libc::c_int != 0 {
                                (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                            } else {
                                ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                                    as libc::c_long
                            })
                        }) == 0 as libc::c_int as libc::c_long
                    {
                        c = (*c).next;
                    } else {
                        p = (*(*c).buf).left_pos;
                        end = (*(*c).buf).end;
                        break;
                    }
                }
                if c.is_null() {
                    return 3 as libc::c_int;
                }
                tmp = if (2 as libc::c_int as libc::c_ulong).wrapping_sub(tmp)
                    > p.offset_from(end) as libc::c_long as libc::c_ulong
                {
                    tmp.wrapping_add(
                        p.offset_from(end) as libc::c_long as mln_u32_t as libc::c_ulong,
                    )
                } else {
                    2 as libc::c_int as libc::c_ulong
                };
            }
            (*ws).status = status;
            len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as mln_u64_t as mln_u64_t;
        } else {
            (*ws).status = 0 as libc::c_int as mln_u16_t;
        }
        content = mln_alloc_m((*ws).pool, len) as mln_u8ptr_t;
        if content.is_null() {
            return 1 as libc::c_int;
        }
        tmp = p.offset_from(end) as libc::c_long as mln_u64_t;
        if tmp > len {
            tmp = len;
        }
        i = 0 as libc::c_int as mln_u64_t;
        loop {
            if tmp > 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    content.offset(i as isize) as *mut libc::c_void,
                    p as *const libc::c_void,
                    tmp,
                );
            }
            i = (i as libc::c_ulong).wrapping_add(tmp) as mln_u64_t as mln_u64_t;
            p = p.offset(tmp as isize);
            if !(tmp < len) {
                break;
            }
            c = (*c).next;
            while !c.is_null() {
                if ((*c).buf).is_null()
                    || (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if (*(*c).buf).in_file() as libc::c_int != 0 {
                            (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                        } else {
                            ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                                as libc::c_long
                        })
                    }) == 0 as libc::c_int as libc::c_long
                {
                    c = (*c).next;
                } else {
                    p = (*(*c).buf).left_pos;
                    end = (*(*c).buf).end;
                    break;
                }
            }
            if c.is_null() {
                mln_alloc_free(content as *mut libc::c_void);
                return 3 as libc::c_int;
            }
            tmp = if len.wrapping_sub(tmp)
                > p.offset_from(end) as libc::c_long as libc::c_ulong
            {
                p.offset_from(end) as libc::c_long as libc::c_ulong
            } else {
                len.wrapping_sub(tmp)
            };
        }
    }
    if (*ws).content_free() != 0 {
        mln_alloc_free((*ws).content);
        (*ws).set_content_free(0 as libc::c_int as mln_u16_t);
    }
    (*ws).content = content as *mut libc::c_void;
    if !content.is_null() {
        (*ws).set_content_free(1 as libc::c_int as mln_u16_t);
    }
    (*ws).content_len = len;
    if b1 as libc::c_int & 0x80 as libc::c_int != 0 {
        (*ws).set_fin(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_fin(0 as libc::c_int as mln_u16_t);
    }
    if b1 as libc::c_int & 0x40 as libc::c_int != 0 {
        (*ws).set_rsv1(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_rsv1(0 as libc::c_int as mln_u16_t);
    }
    if b1 as libc::c_int & 0x20 as libc::c_int != 0 {
        (*ws).set_rsv2(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_rsv2(0 as libc::c_int as mln_u16_t);
    }
    if b1 as libc::c_int & 0x10 as libc::c_int != 0 {
        (*ws).set_rsv3(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_rsv3(0 as libc::c_int as mln_u16_t);
    }
    (*ws).set_opcode((b1 as libc::c_int & 0xf as libc::c_int) as mln_u16_t);
    if b2 as libc::c_int & 0x80 as libc::c_int != 0 {
        (*ws).set_mask(1 as libc::c_int as mln_u16_t);
    } else {
        (*ws).set_mask(0 as libc::c_int as mln_u16_t);
    }
    if b2 as libc::c_int & 0x80 as libc::c_int != 0 {
        (*ws).masking_key = masking_key;
    } else {
        (*ws).masking_key = 0 as libc::c_int as mln_u32_t;
    }
    if (*ws).mask() != 0 {
        let mut tmpkey: [mln_u8_t; 4] = [0; 4];
        let mut status_0: mln_u16_t = (*ws).status;
        tmpkey[0 as libc::c_int
            as usize] = (masking_key >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        tmpkey[1 as libc::c_int
            as usize] = (masking_key >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        tmpkey[2 as libc::c_int
            as usize] = (masking_key >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        tmpkey[3 as libc::c_int
            as usize] = (masking_key & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        i = 0 as libc::c_int as mln_u64_t;
        if (*ws).opcode() as libc::c_int == 0x8 as libc::c_int
            && status_0 as libc::c_int != 0 as libc::c_int
        {
            let fresh76 = i;
            i = i.wrapping_add(1);
            status_0 = ((status_0 as libc::c_int >> 8 as libc::c_int
                & 0xff as libc::c_int
                ^ tmpkey[fresh76.wrapping_rem(4 as libc::c_int as libc::c_ulong)
                    as usize] as libc::c_int) << 8 as libc::c_int
                | status_0 as libc::c_int & 0xff as libc::c_int) as mln_u16_t;
            let fresh77 = i;
            i = i.wrapping_add(1);
            status_0 = ((status_0 as libc::c_int >> 8 as libc::c_int
                & 0xff as libc::c_int) << 8 as libc::c_int
                | status_0 as libc::c_int & 0xff as libc::c_int
                    ^ tmpkey[fresh77.wrapping_rem(4 as libc::c_int as libc::c_ulong)
                        as usize] as libc::c_int) as mln_u16_t;
            (*ws).status = status_0;
        }
        if !content.is_null() {
            tmp = 0 as libc::c_int as mln_u64_t;
            while tmp < len {
                let fresh78 = i;
                i = i.wrapping_add(1);
                let ref mut fresh79 = *content.offset(tmp as isize);
                *fresh79 = (*fresh79 as libc::c_int
                    ^ tmpkey[fresh78.wrapping_rem(4 as libc::c_int as libc::c_ulong)
                        as usize] as libc::c_int) as libc::c_uchar;
                tmp = tmp.wrapping_add(1);
                tmp;
            }
        }
    }
    if ((*ws).extension_handler).is_some() {
        let mut ret: libc::c_int = ((*ws).extension_handler)
            .expect("non-null function pointer")(ws);
        if ret != 0 as libc::c_int {
            return ret;
        }
    }
    if !c.is_null() && !((*c).buf).is_null() {
        (*(*c).buf).left_pos = p;
    }
    while !c.is_null() {
        if !(((*c).buf).is_null()
            || (if ((*c).buf).is_null() {
                0 as libc::c_int as libc::c_long
            } else {
                (if (*(*c).buf).in_file() as libc::c_int != 0 {
                    (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                } else {
                    ((*(*c).buf).last).offset_from((*(*c).buf).left_pos) as libc::c_long
                })
            }) == 0 as libc::c_int as libc::c_long)
        {
            break;
        }
        c = (*c).next;
    }
    if c.is_null() {
        mln_chain_pool_release_all(*in_0);
        *in_0 = 0 as *mut mln_chain_t;
    } else if c != *in_0 {
        let mut tmpc: *mut mln_chain_t = *in_0;
        *in_0 = c;
        c = tmpc;
        while (*c).next != *in_0 {
            c = (*c).next;
        }
        (*c).next = 0 as *mut mln_chain_s;
        mln_chain_pool_release_all(tmpc);
    }
    return 0 as libc::c_int;
}
