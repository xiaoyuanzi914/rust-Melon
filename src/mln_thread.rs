use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn mln_string_new(s: *const libc::c_char) -> *mut mln_string_t;
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn mln_string_const_strcmp(
        s1: *mut mln_string_t,
        s2: *mut libc::c_char,
    ) -> libc::c_int;
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_rbtree_insert(t: *mut mln_rbtree_t, node: *mut mln_rbtree_node_t);
    fn mln_rbtree_search(
        t: *mut mln_rbtree_t,
        key: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_delete(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_node_new(
        t: *mut mln_rbtree_t,
        data: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_node_free(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_chain_pool_release_all(c: *mut mln_chain_t);
    fn mln_tcp_conn_init(tc: *mut mln_tcp_conn_t, sockfd: libc::c_int) -> libc::c_int;
    fn mln_tcp_conn_destroy(tc: *mut mln_tcp_conn_t);
    fn mln_tcp_conn_append(
        tc: *mut mln_tcp_conn_t,
        c: *mut mln_chain_t,
        type_0: libc::c_int,
    );
    fn mln_tcp_conn_head(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_remove(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_pop(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_send(tc: *mut mln_tcp_conn_t) -> libc::c_int;
    fn mln_tcp_conn_recv(tc: *mut mln_tcp_conn_t, flag: mln_u32_t) -> libc::c_int;
    fn mln_event_fd_set(
        event: *mut mln_event_t,
        fd: libc::c_int,
        flag: mln_u32_t,
        timeout_ms: libc::c_int,
        data: *mut libc::c_void,
        fd_handler: ev_fd_handler,
    ) -> libc::c_int;
    fn mln_conf() -> *mut mln_conf_t;
    fn mln_conf_cmd_num(cf: *mut mln_conf_t, domain: *mut libc::c_char) -> mln_u32_t;
    fn mln_conf_arg_num(cc: *mut mln_conf_cmd_t) -> mln_u32_t;
    fn mln_conf_cmds(
        cf: *mut mln_conf_t,
        domain: *mut libc::c_char,
        vector: *mut *mut mln_conf_cmd_t,
    );
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __intptr_t = libc::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type intptr_t = __intptr_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_s8_t = libc::c_char;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s8ptr_t = *mut libc::c_char;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_off_t = off_t;
pub type mln_sauto_t = libc::c_long;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_event_s {
    pub fd_lock: pthread_mutex_t,
    pub timer_lock: pthread_mutex_t,
    pub cb_lock: pthread_mutex_t,
    pub callback: dispatch_callback,
    pub callback_data: *mut libc::c_void,
    #[bitfield(name = "is_break", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "1..=31")]
    pub is_break_padding: [u8; 4],
    pub epollfd: libc::c_int,
    pub unusedfd: libc::c_int,
    pub ev_fd_tree: *mut mln_rbtree_t,
    pub ev_fd_wait_head: *mut mln_event_desc_t,
    pub ev_fd_wait_tail: *mut mln_event_desc_t,
    pub ev_fd_active_head: *mut mln_event_desc_t,
    pub ev_fd_active_tail: *mut mln_event_desc_t,
    pub ev_fd_timeout_heap: *mut mln_fheap_t,
    pub ev_timer_heap: *mut mln_fheap_t,
}
pub type mln_event_desc_t = mln_event_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_event_desc_s {
    pub prev: *mut mln_event_desc_s,
    pub next: *mut mln_event_desc_s,
    pub act_prev: *mut mln_event_desc_s,
    pub act_next: *mut mln_event_desc_s,
    pub type_0: mln_event_type,
    pub flag: mln_u32_t,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tm: mln_event_tm_t,
    pub fd: mln_event_fd_t,
}
pub type mln_event_fd_t = mln_event_fd_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_event_fd_s {
    pub fd: libc::c_int,
    pub active_flag: mln_u32_t,
    #[bitfield(name = "in_process", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "is_clear", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "in_active", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "rd_oneshot", ty = "mln_u32_t", bits = "3..=3")]
    #[bitfield(name = "wr_oneshot", ty = "mln_u32_t", bits = "4..=4")]
    #[bitfield(name = "err_oneshot", ty = "mln_u32_t", bits = "5..=5")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "6..=31")]
    pub in_process_is_clear_in_active_rd_oneshot_wr_oneshot_err_oneshot_padding: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub rcv_data: *mut libc::c_void,
    pub rcv_handler: ev_fd_handler,
    pub snd_data: *mut libc::c_void,
    pub snd_handler: ev_fd_handler,
    pub err_data: *mut libc::c_void,
    pub err_handler: ev_fd_handler,
    pub timeout_data: *mut libc::c_void,
    pub timeout_handler: ev_fd_handler,
    pub timeout_node: *mut mln_fheap_node_t,
    pub end_us: mln_u64_t,
}
pub type ev_fd_handler = Option::<
    unsafe extern "C" fn(*mut mln_event_t, libc::c_int, *mut libc::c_void) -> (),
>;
pub type mln_event_t = mln_event_s;
pub type mln_event_tm_t = mln_event_tm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_event_tm_s {
    pub data: *mut libc::c_void,
    pub handler: ev_tm_handler,
    pub end_tm: mln_uauto_t,
}
pub type ev_tm_handler = Option::<
    unsafe extern "C" fn(*mut mln_event_t, *mut libc::c_void) -> (),
>;
pub type mln_event_type = libc::c_uint;
pub const M_EV_TM: mln_event_type = 1;
pub const M_EV_FD: mln_event_type = 0;
pub type dispatch_callback = Option::<
    unsafe extern "C" fn(*mut mln_event_t, *mut libc::c_void) -> (),
>;
pub type mln_thread_entrance_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_s {
    pub ev: *mut mln_event_t,
    pub thread_main: mln_thread_entrance_t,
    pub alias: *mut mln_string_t,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub peerfd: libc::c_int,
    pub is_created: libc::c_int,
    pub stype: mln_thread_stype_t,
    pub tid: pthread_t,
    pub conn: mln_tcp_conn_t,
    pub local_head: *mut mln_thread_msgq_t,
    pub local_tail: *mut mln_thread_msgq_t,
    pub dest_head: *mut mln_thread_msgq_t,
    pub dest_tail: *mut mln_thread_msgq_t,
    pub node: *mut mln_rbtree_node_t,
}
pub type mln_thread_msgq_t = mln_thread_msgq_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_msgq_s {
    pub sender: *mut mln_thread_t,
    pub dest_prev: *mut mln_thread_msgq_s,
    pub dest_next: *mut mln_thread_msgq_s,
    pub local_prev: *mut mln_thread_msgq_s,
    pub local_next: *mut mln_thread_msgq_s,
    pub msg: mln_thread_msg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_msg_t {
    pub dest: *mut mln_string_t,
    pub src: *mut mln_string_t,
    pub f: libc::c_double,
    pub pfunc: *mut libc::c_void,
    pub pdata: *mut libc::c_void,
    pub sauto: mln_sauto_t,
    pub uauto: mln_uauto_t,
    pub c: mln_s8_t,
    pub padding: [mln_s8_t; 7],
    pub type_0: C2RustUnnamed_0,
    pub need_clear: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ITC_RESPONSE: C2RustUnnamed_0 = 1;
pub const ITC_REQUEST: C2RustUnnamed_0 = 0;
pub type mln_thread_t = mln_thread_s;
pub type mln_thread_stype_t = libc::c_uint;
pub const THREAD_DEFAULT: mln_thread_stype_t = 1;
pub const THREAD_RESTART: mln_thread_stype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_module_t {
    pub alias: *mut libc::c_char,
    pub thread_main: mln_thread_entrance_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_attr {
    pub ev: *mut mln_event_t,
    pub thread_main: mln_thread_entrance_t,
    pub alias: *mut libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub peerfd: libc::c_int,
    pub sockfd: libc::c_int,
    pub stype: mln_thread_stype_t,
}
pub type mln_conf_cmd_t = mln_conf_cmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_cmd_s {
    pub cmd_name: *mut mln_string_t,
    pub search: mln_conf_item_cb_t,
    pub update: mln_conf_item_update_cb_t,
    pub arg_tbl: *mut mln_conf_item_t,
    pub n_args: mln_u32_t,
}
pub type mln_conf_item_t = mln_conf_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_item_s {
    pub type_0: mln_conf_item_type_t,
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: *mut mln_string_t,
    pub c: mln_s8_t,
    pub b: mln_u8_t,
    pub i: mln_sauto_t,
    pub f: libc::c_float,
}
pub type mln_conf_item_type_t = libc::c_uint;
pub const CONF_FLOAT: mln_conf_item_type_t = 5;
pub const CONF_INT: mln_conf_item_type_t = 4;
pub const CONF_BOOL: mln_conf_item_type_t = 3;
pub const CONF_CHAR: mln_conf_item_type_t = 2;
pub const CONF_STR: mln_conf_item_type_t = 1;
pub const CONF_NONE: mln_conf_item_type_t = 0;
pub type mln_conf_item_update_cb_t = Option::<
    unsafe extern "C" fn(
        *mut mln_conf_cmd_t,
        *mut mln_conf_item_t,
        mln_u32_t,
    ) -> libc::c_int,
>;
pub type mln_conf_item_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_conf_cmd_t, mln_u32_t) -> *mut mln_conf_item_t,
>;
pub type mln_conf_t = mln_conf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_s {
    pub lex: *mut mln_lex_t,
    pub domain: *mut mln_rbtree_t,
    pub search: mln_conf_domain_cb_t,
    pub insert: mln_conf_domain_cb_t,
    pub remove: mln_conf_domain_cb_t,
    pub cb: *mut mln_ipc_cb_t,
}
pub type mln_ipc_cb_t = mln_ipc_cb_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_ipc_cb_s {
    pub next: *mut mln_ipc_cb_s,
    pub prev: *mut mln_ipc_cb_s,
    pub master_handler: ipc_handler,
    pub worker_handler: ipc_handler,
    pub master_data: *mut libc::c_void,
    pub worker_data: *mut libc::c_void,
    pub type_0: mln_u32_t,
}
pub type ipc_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_event_t,
        *mut libc::c_void,
        *mut libc::c_void,
        mln_u32_t,
        *mut *mut libc::c_void,
    ) -> (),
>;
pub type mln_conf_domain_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_conf_t, *mut libc::c_char) -> *mut mln_conf_domain_t,
>;
pub type mln_conf_domain_t = mln_conf_domain_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_domain_s {
    pub search: mln_conf_cmd_cb_t,
    pub insert: mln_conf_cmd_cb_t,
    pub remove: mln_conf_cmd_cb_t,
    pub domain_name: *mut mln_string_t,
    pub cmd: *mut mln_rbtree_t,
}
pub type mln_conf_cmd_cb_t = Option::<
    unsafe extern "C" fn(
        *mut mln_conf_domain_t,
        *mut libc::c_char,
    ) -> *mut mln_conf_cmd_t,
>;
pub type mln_lex_t = mln_lex_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lex_s {
    pub pool: *mut mln_alloc_t,
    pub macros: *mut mln_rbtree_t,
    pub cur: *mut mln_lex_input_t,
    pub stack: *mut mln_stack_t,
    pub hooks: mln_lex_hooks_t,
    pub keywords: *mut mln_rbtree_t,
    pub err_msg: mln_s8ptr_t,
    pub result_buf: mln_u8ptr_t,
    pub result_pos: mln_u8ptr_t,
    pub result_buf_len: mln_u64_t,
    pub line: mln_u64_t,
    pub error: mln_s32_t,
    #[bitfield(name = "preprocess", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "ignore", ty = "mln_u32_t", bits = "1..=1")]
    pub preprocess_ignore: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub env: *mut mln_string_t,
    pub preprocess_data: *mut mln_lex_preprocess_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_preprocess_data_t {
    pub if_level: mln_u64_t,
    pub if_matched: mln_u64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_hooks_t {
    pub excl_handler: lex_hook,
    pub excl_data: *mut libc::c_void,
    pub dblq_handler: lex_hook,
    pub dblq_data: *mut libc::c_void,
    pub nums_handler: lex_hook,
    pub nums_data: *mut libc::c_void,
    pub doll_handler: lex_hook,
    pub doll_data: *mut libc::c_void,
    pub perc_handler: lex_hook,
    pub perc_data: *mut libc::c_void,
    pub amp_handler: lex_hook,
    pub amp_data: *mut libc::c_void,
    pub sglq_handler: lex_hook,
    pub slgq_data: *mut libc::c_void,
    pub lpar_handler: lex_hook,
    pub lpar_data: *mut libc::c_void,
    pub rpar_handler: lex_hook,
    pub rpar_data: *mut libc::c_void,
    pub ast_handler: lex_hook,
    pub ast_data: *mut libc::c_void,
    pub plus_handler: lex_hook,
    pub plus_data: *mut libc::c_void,
    pub comma_handler: lex_hook,
    pub comma_data: *mut libc::c_void,
    pub sub_handler: lex_hook,
    pub sub_data: *mut libc::c_void,
    pub period_handler: lex_hook,
    pub period_data: *mut libc::c_void,
    pub slash_handler: lex_hook,
    pub slash_data: *mut libc::c_void,
    pub colon_handler: lex_hook,
    pub colon_data: *mut libc::c_void,
    pub semic_handler: lex_hook,
    pub semic_data: *mut libc::c_void,
    pub lagl_handler: lex_hook,
    pub lagl_data: *mut libc::c_void,
    pub equal_handler: lex_hook,
    pub equal_data: *mut libc::c_void,
    pub ragl_handler: lex_hook,
    pub ragl_data: *mut libc::c_void,
    pub ques_handler: lex_hook,
    pub ques_data: *mut libc::c_void,
    pub at_handler: lex_hook,
    pub at_data: *mut libc::c_void,
    pub lsquar_handler: lex_hook,
    pub lsquar_data: *mut libc::c_void,
    pub bslash_handler: lex_hook,
    pub bslash_data: *mut libc::c_void,
    pub rsquar_handler: lex_hook,
    pub rsquar_data: *mut libc::c_void,
    pub xor_handler: lex_hook,
    pub xor_data: *mut libc::c_void,
    pub under_handler: lex_hook,
    pub under_data: *mut libc::c_void,
    pub fulstp_handler: lex_hook,
    pub fulstp_data: *mut libc::c_void,
    pub lbrace_handler: lex_hook,
    pub lbrace_data: *mut libc::c_void,
    pub vertl_handler: lex_hook,
    pub vertl_data: *mut libc::c_void,
    pub rbrace_handler: lex_hook,
    pub rbrace_data: *mut libc::c_void,
    pub dash_handler: lex_hook,
    pub dash_data: *mut libc::c_void,
}
pub type lex_hook = Option::<
    unsafe extern "C" fn(*mut mln_lex_t, *mut libc::c_void) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_stack_t {
    pub bottom: *mut mln_stack_node_t,
    pub top: *mut mln_stack_node_t,
    pub free_head: *mut mln_stack_node_t,
    pub free_tail: *mut mln_stack_node_t,
    pub nr_node: mln_uauto_t,
    pub free_handler: stack_free,
    pub copy_handler: stack_copy,
}
pub type stack_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
pub type stack_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type mln_stack_node_t = mln_stack_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_stack_node_s {
    pub data: *mut libc::c_void,
    pub prev: *mut mln_stack_node_s,
    pub next: *mut mln_stack_node_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_input_t {
    pub type_0: mln_u32_t,
    pub fd: libc::c_int,
    pub data: *mut mln_string_t,
    pub dir: *mut mln_string_t,
    pub buf: mln_u8ptr_t,
    pub pos: mln_u8ptr_t,
    pub buf_len: mln_u64_t,
    pub line: mln_u64_t,
}
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
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
                    current_block = 9708792784534785756;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                9708792784534785756 => {}
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
        current_block_8 = 7128820075091651679;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 7128820075091651679;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        7128820075091651679 => {
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
#[thread_local]
pub static mut thread_cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
#[no_mangle]
#[thread_local]
pub static mut thread_data: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
#[thread_local]
pub static mut m_thread: *mut mln_thread_t = 0 as *const mln_thread_t
    as *mut mln_thread_t;
#[no_mangle]
pub static mut thread_tree: *mut mln_rbtree_t = 0 as *const mln_rbtree_t
    as *mut mln_rbtree_t;
#[no_mangle]
pub static mut thread_domain: [libc::c_char; 12] = unsafe {
    *::core::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"thread_exec\0")
};
#[no_mangle]
pub static mut thread_s_restart: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"restart\0")
};
#[no_mangle]
pub static mut thread_s_default: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"default\0")
};
#[no_mangle]
pub static mut thread_start_func: [libc::c_char; 12] = unsafe {
    *::core::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"thread_main\0")
};
static mut module_array: *mut mln_thread_module_t = 0 as *const mln_thread_module_t
    as *mut mln_thread_module_t;
static mut module_array_num: mln_size_t = 0 as libc::c_int as mln_size_t;
unsafe extern "C" fn mln_thread_init(
    mut attr: *mut mln_thread_attr,
) -> *mut mln_thread_t {
    let mut t: *mut mln_thread_t = malloc(
        ::core::mem::size_of::<mln_thread_t>() as libc::c_ulong,
    ) as *mut mln_thread_t;
    if t.is_null() {
        return 0 as *mut mln_thread_t;
    }
    (*t).ev = (*attr).ev;
    (*t).thread_main = (*attr).thread_main;
    (*t).alias = mln_string_new((*attr).alias);
    if ((*t).alias).is_null() {
        free(t as *mut libc::c_void);
        return 0 as *mut mln_thread_t;
    }
    (*t).argv = (*attr).argv;
    (*t).argc = (*attr).argc;
    (*t).peerfd = (*attr).peerfd;
    (*t).is_created = 0 as libc::c_int;
    (*t).stype = (*attr).stype;
    if mln_tcp_conn_init(&mut (*t).conn, (*attr).sockfd) < 0 as libc::c_int {
        let mut __s: *mut mln_string_t = (*t).alias;
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
        free(t as *mut libc::c_void);
        return 0 as *mut mln_thread_t;
    }
    (*t).local_head = 0 as *mut mln_thread_msgq_t;
    (*t).local_tail = 0 as *mut mln_thread_msgq_t;
    (*t).dest_head = 0 as *mut mln_thread_msgq_t;
    (*t).dest_tail = 0 as *mut mln_thread_msgq_t;
    return t;
}
unsafe extern "C" fn mln_thread_destroy(mut t: *mut mln_thread_t) {
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    if t.is_null() {
        return;
    }
    if !((*t).alias).is_null() {
        let mut __s: *mut mln_string_t = (*t).alias;
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
    if !((*t).argv).is_null() {
        if !(*((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)).is_null() {
            free(
                *((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)
                    as *mut libc::c_void,
            );
        }
        free((*t).argv as *mut libc::c_void);
    }
    if (*t).peerfd >= 0 as libc::c_int {
        close((*t).peerfd);
    }
    if (*t).is_created != 0 {
        let mut tret: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut err: libc::c_int = pthread_join((*t).tid, &mut tret);
        if err != 0 as libc::c_int {
            _mln_sys_log(
                error,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"mln_thread_destroy\0"))
                    .as_ptr(),
                131 as libc::c_int,
                b"pthread_join error. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            abort();
        }
        _mln_sys_log(
            report,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"mln_thread_destroy\0"))
                .as_ptr(),
            134 as libc::c_int,
            b"child thread pthread_join's exit code: %l\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            tret as intptr_t,
        );
    }
    if (*t).conn.sockfd >= 0 as libc::c_int {
        close((*t).conn.sockfd);
    }
    c = mln_tcp_conn_head(&mut (*t).conn, 1 as libc::c_int);
    mln_thread_itc_chain_release_msg(c);
    c = mln_tcp_conn_head(&mut (*t).conn, 2 as libc::c_int);
    mln_thread_itc_chain_release_msg(c);
    mln_tcp_conn_destroy(&mut (*t).conn);
    mln_thread_clear_msg_queue((*t).ev, t);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn mln_thread_clear_msg_queue(
    mut ev: *mut mln_event_t,
    mut t: *mut mln_thread_t,
) {
    let mut tmq: *mut mln_thread_msgq_t = 0 as *mut mln_thread_msgq_t;
    loop {
        tmq = (*t).local_head;
        if tmq.is_null() {
            break;
        }
        msg_local_chain_del(&mut (*t).local_head, &mut (*t).local_tail, tmq);
        (*tmq).sender = 0 as *mut mln_thread_t;
    }
    let mut sender: *mut mln_thread_t = 0 as *mut mln_thread_t;
    loop {
        tmq = (*t).dest_head;
        if tmq.is_null() {
            break;
        }
        sender = (*tmq).sender;
        if !sender.is_null() {
            msg_local_chain_del(
                &mut (*sender).local_head,
                &mut (*sender).local_tail,
                tmq,
            );
        }
        msg_dest_chain_del(&mut (*t).dest_head, &mut (*t).dest_tail, tmq);
        mln_thread_clear_msg(&mut (*tmq).msg);
        mln_thread_msgq_destroy(tmq);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_clear_msg(mut msg: *mut mln_thread_msg_t) {
    if msg.is_null() {
        return;
    }
    if !((*msg).dest).is_null() {
        let mut __s: *mut mln_string_t = (*msg).dest;
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
        (*msg).dest = 0 as *mut mln_string_t;
    }
    if !((*msg).src).is_null() {
        let mut __s: *mut mln_string_t = (*msg).src;
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
        (*msg).src = 0 as *mut mln_string_t;
    }
    (*msg).pfunc = 0 as *mut libc::c_void;
    if (*msg).need_clear != 0 && !((*msg).pdata).is_null() {
        free((*msg).pdata);
    } else {
        (*msg).pdata = 0 as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_load_thread(mut ev: *mut mln_event_t) -> libc::c_int {
    if mln_thread_rbtree_init() < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mln_load_thread\0"))
                .as_ptr(),
            192 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cf: *mut mln_conf_t = mln_conf();
    if cf.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mln_load_thread\0"))
                .as_ptr(),
            197 as libc::c_int,
            b"configuration messed up!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
    let mut nr_cmds: mln_u32_t = mln_conf_cmd_num(cf, thread_domain.as_mut_ptr());
    if nr_cmds == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    let mut v: *mut *mut mln_conf_cmd_t = calloc(
        nr_cmds as libc::c_ulong,
        ::core::mem::size_of::<*mut mln_conf_cmd_t>() as libc::c_ulong,
    ) as *mut *mut mln_conf_cmd_t;
    if v.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mln_load_thread\0"))
                .as_ptr(),
            206 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_rbtree_free(thread_tree);
        thread_tree = 0 as *mut mln_rbtree_t;
        return -(1 as libc::c_int);
    }
    mln_conf_cmds(cf, thread_domain.as_mut_ptr(), v);
    let mut i: mln_u32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < nr_cmds {
        mln_loada_thread(ev, *v.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free(v as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_loada_thread(
    mut ev: *mut mln_event_t,
    mut cc: *mut mln_conf_cmd_t,
) {
    let mut t: *mut mln_thread_t = 0 as *mut mln_thread_t;
    let mut i: mln_u32_t = 0;
    let mut nr_args: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut thattr: mln_thread_attr = mln_thread_attr {
        ev: 0 as *mut mln_event_t,
        thread_main: None,
        alias: 0 as *mut libc::c_char,
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        peerfd: 0,
        sockfd: 0,
        stype: THREAD_RESTART,
    };
    if mln_string_const_strcmp((*cc).cmd_name, thread_s_restart.as_mut_ptr()) == 0 {
        thattr.stype = THREAD_RESTART;
    } else if mln_string_const_strcmp((*cc).cmd_name, thread_s_default.as_mut_ptr()) == 0
    {
        thattr.stype = THREAD_DEFAULT;
    } else {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_loada_thread\0"))
                .as_ptr(),
            233 as libc::c_int,
            b"No such command '%s' in domain '%s'.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*cc).cmd_name,
            thread_domain.as_mut_ptr(),
        );
        return;
    }
    nr_args = mln_conf_arg_num(cc);
    if nr_args < 1 as libc::c_int as libc::c_uint {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_loada_thread\0"))
                .as_ptr(),
            238 as libc::c_int,
            b"Invalid arguments in domain '%s'.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            thread_domain.as_mut_ptr(),
        );
        return;
    }
    thattr.ev = ev;
    thattr
        .argv = calloc(
        nr_args.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if (thattr.argv).is_null() {
        return;
    }
    thattr.argc = nr_args.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 1 as libc::c_int as mln_u32_t;
    while i <= nr_args {
        ci = ((*cc).search).expect("non-null function pointer")(cc, i);
        if (*ci).type_0 as libc::c_uint != CONF_STR as libc::c_int as libc::c_uint {
            _mln_sys_log(
                error,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"mln_loada_thread\0"))
                    .as_ptr(),
                249 as libc::c_int,
                b"Invalid argument type in domain '%s'.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                thread_domain.as_mut_ptr(),
            );
            free(thattr.argv as *mut libc::c_void);
            return;
        }
        let ref mut fresh9 = *(thattr.argv)
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        *fresh9 = (*(*ci).val.s).data as *mut libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    thattr.alias = *(thattr.argv).offset(0 as libc::c_int as isize);
    thattr
        .thread_main = ::core::mem::transmute::<
        *mut libc::c_void,
        mln_thread_entrance_t,
    >(mln_get_module_entrance(thattr.alias));
    if (thattr.thread_main).is_none() {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_loada_thread\0"))
                .as_ptr(),
            259 as libc::c_int,
            b"No such thread module named '%s'.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            thattr.alias,
        );
        free(thattr.argv as *mut libc::c_void);
        return;
    }
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_loada_thread\0"))
                .as_ptr(),
            265 as libc::c_int,
            b"socketpair error. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        free(thattr.argv as *mut libc::c_void);
        return;
    }
    thattr.sockfd = fds[0 as libc::c_int as usize];
    thattr.peerfd = fds[1 as libc::c_int as usize];
    t = mln_thread_init(&mut thattr);
    if t.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_loada_thread\0"))
                .as_ptr(),
            273 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        close(fds[0 as libc::c_int as usize]);
        close(fds[1 as libc::c_int as usize]);
        free(thattr.argv as *mut libc::c_void);
        return;
    }
    _mln_sys_log(
        none,
        b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"mln_loada_thread\0"))
            .as_ptr(),
        278 as libc::c_int,
        b"Start thread '%s'\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        *((*t).argv).offset(0 as libc::c_int as isize),
    );
    if __mln_thread_create(t) < 0 as libc::c_int {
        mln_thread_destroy(t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_create(
    mut ev: *mut mln_event_t,
    mut alias: *mut libc::c_char,
    mut stype: mln_thread_stype_t,
    mut entrance: mln_thread_entrance_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut t: *mut mln_thread_t = 0 as *mut mln_thread_t;
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut thattr: mln_thread_attr = mln_thread_attr {
        ev: 0 as *mut mln_event_t,
        thread_main: None,
        alias: 0 as *mut libc::c_char,
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        peerfd: 0,
        sockfd: 0,
        stype: THREAD_RESTART,
    };
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    thattr.ev = ev;
    thattr.stype = stype;
    thattr.argv = argv;
    thattr.argc = argc + 1 as libc::c_int;
    thattr.alias = alias;
    thattr.thread_main = entrance;
    thattr.sockfd = fds[0 as libc::c_int as usize];
    thattr.peerfd = fds[1 as libc::c_int as usize];
    t = mln_thread_init(&mut thattr);
    if t.is_null() {
        close(fds[0 as libc::c_int as usize]);
        close(fds[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    if __mln_thread_create(t) < 0 as libc::c_int {
        mln_thread_destroy(t);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_get_module_entrance(
    mut alias: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut tm: *mut mln_thread_module_t = module_array;
    let mut end: *mut mln_thread_module_t = module_array
        .offset(module_array_num as isize);
    while tm < end {
        if strcmp(alias, (*tm).alias) == 0 {
            return ::core::mem::transmute::<
                mln_thread_entrance_t,
                *mut libc::c_void,
            >((*tm).thread_main);
        }
        tm = tm.offset(1);
        tm;
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn __mln_thread_create(mut t: *mut mln_thread_t) -> libc::c_int {
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    if (*((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)).is_null() {
        let mut int_str: *mut libc::c_char = malloc(128 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        if int_str.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"__mln_thread_create\0"))
                    .as_ptr(),
                335 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        let ref mut fresh10 = *((*t).argv)
            .offset(((*t).argc - 1 as libc::c_int) as isize);
        *fresh10 = int_str;
    }
    memset(
        *((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)
            as *mut libc::c_void,
        0 as libc::c_int,
        128 as libc::c_int as libc::c_ulong,
    );
    snprintf(
        *((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize),
        (128 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*t).peerfd,
    );
    rn = mln_rbtree_node_new(thread_tree, t as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"__mln_thread_create\0"))
                .as_ptr(),
            343 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(thread_tree, rn);
    if mln_event_fd_set(
        (*t).ev,
        (*t).conn.sockfd,
        0x1 as libc::c_int as mln_u32_t | 0x10 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        t as *mut libc::c_void,
        Some(
            mln_main_thread_itc_recv_handler
                as unsafe extern "C" fn(
                    *mut mln_event_t,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
    ) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"__mln_thread_create\0"))
                .as_ptr(),
            354 as libc::c_int,
            b"mln_event_fd_set failed.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mln_rbtree_delete(thread_tree, rn);
        mln_rbtree_node_free(thread_tree, rn);
        (*t).node = 0 as *mut mln_rbtree_node_t;
        return -(1 as libc::c_int);
    }
    let mut err: libc::c_int = 0;
    err = pthread_create(
        &mut (*t).tid,
        0 as *const pthread_attr_t,
        Some(
            mln_thread_launcher
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        t as *mut libc::c_void,
    );
    if err != 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"__mln_thread_create\0"))
                .as_ptr(),
            362 as libc::c_int,
            b"pthread_create error. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(err),
        );
        mln_event_fd_set(
            (*t).ev,
            (*t).conn.sockfd,
            0x80 as libc::c_int as mln_u32_t,
            -(1 as libc::c_int),
            0 as *mut libc::c_void,
            None,
        );
        mln_rbtree_delete(thread_tree, rn);
        mln_rbtree_node_free(thread_tree, rn);
        (*t).node = 0 as *mut mln_rbtree_node_t;
        return -(1 as libc::c_int);
    }
    (*t).is_created = 1 as libc::c_int;
    (*t).node = rn;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_itc_get_buf_with_len(
    mut tc: *mut mln_tcp_conn_t,
    mut buf: *mut libc::c_void,
    mut len: mln_size_t,
) -> libc::c_int {
    let mut size: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut pos: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    c = mln_tcp_conn_head(tc, 2 as libc::c_int);
    while !c.is_null() {
        if !(((*c).buf).is_null() || ((*(*c).buf).pos).is_null()) {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (if ((*c).buf).is_null() {
                        0 as libc::c_int as libc::c_long
                    } else if (*(*c).buf).in_file() as libc::c_int != 0 {
                        (*(*c).buf).file_last - (*(*c).buf).file_left_pos
                    } else {
                        ((*(*c).buf).last).offset_from((*(*c).buf).left_pos)
                            as libc::c_long
                    }) as libc::c_ulong,
                ) as mln_size_t as mln_size_t;
            if size >= len {
                break;
            }
        }
        c = (*c).next;
    }
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    pos = buf as mln_u8ptr_t;
    loop {
        c = mln_tcp_conn_head(tc, 2 as libc::c_int);
        if c.is_null() {
            break;
        }
        b = (*c).buf;
        if b.is_null() || ((*b).pos).is_null() {
            mln_chain_pool_release(mln_tcp_conn_pop(tc, 2 as libc::c_int));
        } else {
            size = (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else if (*b).in_file() as libc::c_int != 0 {
                (*b).file_last - (*b).file_left_pos
            } else {
                ((*b).last).offset_from((*b).left_pos) as libc::c_long
            }) as mln_size_t;
            if size > len {
                memcpy(
                    pos as *mut libc::c_void,
                    (*b).left_pos as *const libc::c_void,
                    len,
                );
                (*b).left_pos = ((*b).left_pos).offset(len as isize);
                break;
            } else {
                memcpy(
                    pos as *mut libc::c_void,
                    (*b).left_pos as *const libc::c_void,
                    size,
                );
                pos = pos.offset(size as isize);
                len = (len as libc::c_ulong).wrapping_sub(size) as mln_size_t
                    as mln_size_t;
                (*b).left_pos = ((*b).left_pos).offset(size as isize);
                mln_chain_pool_release(mln_tcp_conn_pop(tc, 2 as libc::c_int));
                if len == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_thread_itc_chain_release_msg(mut c: *mut mln_chain_t) {
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    while !c.is_null() {
        b = (*c).buf;
        if !b.is_null() {
            mln_thread_clear_msg((*b).pos as *mut mln_thread_msg_t);
        }
        c = (*c).next;
    }
}
unsafe extern "C" fn mln_main_thread_itc_recv_handler(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut t: *mut mln_thread_t = data as *mut mln_thread_t;
    let mut conn: *mut mln_tcp_conn_t = &mut (*t).conn;
    let mut ret: libc::c_int = 0;
    loop {
        ret = mln_tcp_conn_recv(conn, 0x1 as libc::c_int as mln_u32_t);
        if ret == 1 as libc::c_int {
            continue;
        }
        if ret == 2 as libc::c_int {
            break;
        }
        if ret == 4 as libc::c_int {
            _mln_sys_log(
                report,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"mln_main_thread_itc_recv_handler\0"))
                    .as_ptr(),
                445 as libc::c_int,
                b"Child thread '%s' exit.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                *((*t).argv).offset(0 as libc::c_int as isize),
            );
            mln_thread_deal_child_exit(ev, t);
            return;
        } else {
            _mln_sys_log(
                error,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"mln_main_thread_itc_recv_handler\0"))
                    .as_ptr(),
                449 as libc::c_int,
                b"mln_tcp_conn_recv() error. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            mln_thread_deal_child_exit(ev, t);
            return;
        }
    }
    mln_main_thread_itc_recv_handler_process(ev, t);
}
unsafe extern "C" fn mln_main_thread_itc_recv_handler_process(
    mut ev: *mut mln_event_t,
    mut t: *mut mln_thread_t,
) {
    let mut conn: *mut mln_tcp_conn_t = &mut (*t).conn;
    let mut msg: mln_thread_msg_t = mln_thread_msg_t {
        dest: 0 as *mut mln_string_t,
        src: 0 as *mut mln_string_t,
        f: 0.,
        pfunc: 0 as *mut libc::c_void,
        pdata: 0 as *mut libc::c_void,
        sauto: 0,
        uauto: 0,
        c: 0,
        padding: [0; 7],
        type_0: ITC_REQUEST,
        need_clear: 0,
    };
    let mut m: *mut mln_thread_msg_t = 0 as *mut mln_thread_msg_t;
    let mut tmq: *mut mln_thread_msgq_t = 0 as *mut mln_thread_msgq_t;
    let mut target: *mut mln_thread_t = 0 as *mut mln_thread_t;
    let mut tmp: mln_thread_t = mln_thread_t {
        ev: 0 as *mut mln_event_t,
        thread_main: None,
        alias: 0 as *mut mln_string_t,
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        peerfd: 0,
        is_created: 0,
        stype: THREAD_RESTART,
        tid: 0,
        conn: mln_tcp_conn_t {
            pool: 0 as *mut mln_alloc_t,
            rcv_head: 0 as *mut mln_chain_t,
            rcv_tail: 0 as *mut mln_chain_t,
            snd_head: 0 as *mut mln_chain_t,
            snd_tail: 0 as *mut mln_chain_t,
            sent_head: 0 as *mut mln_chain_t,
            sent_tail: 0 as *mut mln_chain_t,
            sockfd: 0,
        },
        local_head: 0 as *mut mln_thread_msgq_t,
        local_tail: 0 as *mut mln_thread_msgq_t,
        dest_head: 0 as *mut mln_thread_msgq_t,
        dest_tail: 0 as *mut mln_thread_msgq_t,
        node: 0 as *mut mln_rbtree_node_t,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    loop {
        if mln_itc_get_buf_with_len(
            conn,
            &mut msg as *mut mln_thread_msg_t as *mut libc::c_void,
            ::core::mem::size_of::<mln_thread_msg_t>() as libc::c_ulong,
        ) < 0 as libc::c_int
        {
            return;
        }
        tmq = mln_thread_msgq_init(t, &mut msg);
        if tmq.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"mln_main_thread_itc_recv_handler_process\0"))
                    .as_ptr(),
                474 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            m = &mut (*tmq).msg;
            (*m).src = mln_string_dup((*t).alias);
            if ((*m).src).is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"mln_main_thread_itc_recv_handler_process\0"))
                        .as_ptr(),
                    481 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mln_thread_clear_msg(&mut (*tmq).msg);
                mln_thread_msgq_destroy(tmq);
            } else {
                tmp.alias = (*m).dest;
                rn = mln_rbtree_search(
                    thread_tree,
                    &mut tmp as *mut mln_thread_t as *mut libc::c_void,
                );
                if rn == &mut (*thread_tree).nil as *mut mln_rbtree_node_t {
                    _mln_sys_log(
                        report,
                        b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"mln_main_thread_itc_recv_handler_process\0"))
                            .as_ptr(),
                        490 as libc::c_int,
                        b"No such thread named '%s'.\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*(*m).dest).data as *mut libc::c_char,
                    );
                    mln_thread_clear_msg(&mut (*tmq).msg);
                    mln_thread_msgq_destroy(tmq);
                } else {
                    target = (*rn).data as *mut mln_thread_t;
                    if ((*target).dest_head).is_null() {
                        mln_event_fd_set(
                            ev,
                            (*target).conn.sockfd,
                            0x2 as libc::c_int as mln_u32_t
                                | 0x8 as libc::c_int as mln_u32_t
                                | 0x10 as libc::c_int as mln_u32_t
                                | 0x40 as libc::c_int as mln_u32_t,
                            -(1 as libc::c_int),
                            target as *mut libc::c_void,
                            Some(
                                mln_main_thread_itc_send_handler
                                    as unsafe extern "C" fn(
                                        *mut mln_event_t,
                                        libc::c_int,
                                        *mut libc::c_void,
                                    ) -> (),
                            ),
                        );
                    }
                    msg_local_chain_add(&mut (*t).local_head, &mut (*t).local_tail, tmq);
                    msg_dest_chain_add(
                        &mut (*target).dest_head,
                        &mut (*target).dest_tail,
                        tmq,
                    );
                }
            }
        }
    };
}
unsafe extern "C" fn mln_main_thread_itc_send_handler(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut t: *mut mln_thread_t = data as *mut mln_thread_t;
    let mut tmq: *mut mln_thread_msgq_t = 0 as *mut mln_thread_msgq_t;
    let mut conn: *mut mln_tcp_conn_t = &mut (*t).conn;
    let mut pool: *mut mln_alloc_t = (*conn).pool;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    '_again: loop {
        loop {
            c = mln_tcp_conn_head(conn, 1 as libc::c_int);
            if c.is_null() {
                break;
            }
            ret = mln_tcp_conn_send(conn);
            if ret == 1 as libc::c_int {
                continue;
            }
            if ret == 2 as libc::c_int {
                mln_chain_pool_release_all(mln_tcp_conn_remove(conn, 3 as libc::c_int));
                mln_event_fd_set(
                    ev,
                    (*t).conn.sockfd,
                    0x2 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t
                        | 0x10 as libc::c_int as mln_u32_t
                        | 0x40 as libc::c_int as mln_u32_t,
                    -(1 as libc::c_int),
                    t as *mut libc::c_void,
                    Some(
                        mln_main_thread_itc_send_handler
                            as unsafe extern "C" fn(
                                *mut mln_event_t,
                                libc::c_int,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                );
                return;
            } else if ret == 3 as libc::c_int {
                _mln_sys_log(
                    error,
                    b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"mln_main_thread_itc_send_handler\0"))
                        .as_ptr(),
                    538 as libc::c_int,
                    b"mln_tcp_conn_send() error. %s\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    strerror(*__errno_location()),
                );
                mln_thread_deal_child_exit(ev, t);
                return;
            } else {
                _mln_sys_log(
                    error,
                    b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"mln_main_thread_itc_send_handler\0"))
                        .as_ptr(),
                    542 as libc::c_int,
                    b"Shouldn't be here.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
        }
        mln_chain_pool_release_all(mln_tcp_conn_remove(conn, 3 as libc::c_int));
        loop {
            tmq = (*t).dest_head;
            if tmq.is_null() {
                break '_again;
            }
            msg_dest_chain_del(&mut (*t).dest_head, &mut (*t).dest_tail, tmq);
            if !((*tmq).sender).is_null() {
                msg_local_chain_del(
                    &mut (*(*tmq).sender).local_head,
                    &mut (*(*tmq).sender).local_tail,
                    tmq,
                );
            }
            c = mln_chain_new(pool);
            if c.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"mln_main_thread_itc_send_handler\0"))
                        .as_ptr(),
                    556 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mln_thread_clear_msg(&mut (*tmq).msg);
                mln_thread_msgq_destroy(tmq);
            } else {
                b = mln_buf_new(pool);
                if b.is_null() {
                    _mln_sys_log(
                        error,
                        b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 33],
                            &[libc::c_char; 33],
                        >(b"mln_main_thread_itc_send_handler\0"))
                            .as_ptr(),
                        563 as libc::c_int,
                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    mln_thread_clear_msg(&mut (*tmq).msg);
                    mln_thread_msgq_destroy(tmq);
                    mln_chain_pool_release(c);
                } else {
                    (*c).buf = b;
                    buf = mln_alloc_m(
                        pool,
                        ::core::mem::size_of::<mln_thread_msg_t>() as libc::c_ulong,
                    ) as mln_u8ptr_t;
                    if buf.is_null() {
                        _mln_sys_log(
                            error,
                            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 33],
                                &[libc::c_char; 33],
                            >(b"mln_main_thread_itc_send_handler\0"))
                                .as_ptr(),
                            572 as libc::c_int,
                            b"No memory.\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        mln_thread_clear_msg(&mut (*tmq).msg);
                        mln_thread_msgq_destroy(tmq);
                        mln_chain_pool_release(c);
                    } else {
                        memcpy(
                            buf as *mut libc::c_void,
                            &mut (*tmq).msg as *mut mln_thread_msg_t
                                as *const libc::c_void,
                            ::core::mem::size_of::<mln_thread_msg_t>() as libc::c_ulong,
                        );
                        mln_thread_msgq_destroy(tmq);
                        (*b).start = buf;
                        (*b).pos = (*b).start;
                        (*b).left_pos = (*b).pos;
                        (*b)
                            .end = buf
                            .offset(
                                ::core::mem::size_of::<mln_thread_msg_t>() as libc::c_ulong
                                    as isize,
                            );
                        (*b).last = (*b).end;
                        (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
                        (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
                        (*b).set_last_in_chain(1 as libc::c_int as mln_u32_t);
                        mln_tcp_conn_append(conn, c, 1 as libc::c_int);
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn mln_thread_deal_child_exit(
    mut ev: *mut mln_event_t,
    mut t: *mut mln_thread_t,
) -> libc::c_int {
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    mln_rbtree_delete(thread_tree, (*t).node);
    mln_rbtree_node_free(thread_tree, (*t).node);
    (*t).node = 0 as *mut mln_rbtree_node_t;
    mln_event_fd_set(
        ev,
        (*t).conn.sockfd,
        0x80 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        0 as *mut libc::c_void,
        None,
    );
    if (*t).stype as libc::c_uint == THREAD_DEFAULT as libc::c_int as libc::c_uint {
        mln_thread_destroy(t);
        return 0 as libc::c_int;
    }
    let mut tret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut err: libc::c_int = pthread_join((*t).tid, &mut tret);
    if err != 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"mln_thread_deal_child_exit\0"))
                .as_ptr(),
            616 as libc::c_int,
            b"pthread_join error. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        abort();
    }
    _mln_sys_log(
        report,
        b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"mln_thread_deal_child_exit\0"))
            .as_ptr(),
        619 as libc::c_int,
        b"child thread pthread_join's exit code: %l\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        tret as intptr_t,
    );
    (*t).is_created = 0 as libc::c_int;
    if !((*t).argv).is_null()
        && !(*((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)).is_null()
    {
        free(
            *((*t).argv).offset(((*t).argc - 1 as libc::c_int) as isize)
                as *mut libc::c_void,
        );
        let ref mut fresh11 = *((*t).argv)
            .offset(((*t).argc - 1 as libc::c_int) as isize);
        *fresh11 = 0 as *mut libc::c_char;
    }
    close((*t).conn.sockfd);
    c = mln_tcp_conn_remove(&mut (*t).conn, 1 as libc::c_int);
    mln_thread_itc_chain_release_msg(c);
    mln_chain_pool_release_all(c);
    c = mln_tcp_conn_remove(&mut (*t).conn, 2 as libc::c_int);
    mln_thread_itc_chain_release_msg(c);
    mln_chain_pool_release_all(c);
    mln_chain_pool_release_all(mln_tcp_conn_remove(&mut (*t).conn, 3 as libc::c_int));
    mln_thread_clear_msg_queue(ev, t);
    let mut fds: [libc::c_int; 2] = [0; 2];
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"mln_thread_deal_child_exit\0"))
                .as_ptr(),
            641 as libc::c_int,
            b"socketpair error. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        mln_thread_destroy(t);
        return -(1 as libc::c_int);
    }
    (*t).conn.sockfd = fds[0 as libc::c_int as usize];
    (*t).peerfd = fds[1 as libc::c_int as usize];
    if __mln_thread_create(t) < 0 as libc::c_int {
        mln_thread_destroy(t);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_thread_launcher(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut t: *mut mln_thread_t = args as *mut mln_thread_t;
    m_thread = t;
    let mut ret: libc::c_int = ((*t).thread_main)
        .expect("non-null function pointer")((*t).argc, (*t).argv);
    if thread_cleanup.is_some() {
        thread_cleanup.expect("non-null function pointer")(thread_data);
    }
    _mln_sys_log(
        report,
        b"src/mln_thread.c\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"mln_thread_launcher\0"))
            .as_ptr(),
        663 as libc::c_int,
        b"Thread '%s' return %d.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        *((*t).argv).offset(0 as libc::c_int as isize),
        ret,
    );
    close((*t).peerfd);
    (*t).peerfd = -(1 as libc::c_int);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn mln_thread_rbtree_init() -> libc::c_int {
    let mut rbattr: mln_rbtree_attr = mln_rbtree_attr {
        pool: 0 as *mut libc::c_void,
        pool_alloc: None,
        pool_free: None,
        cmp: None,
        data_free: None,
    };
    rbattr.pool = 0 as *mut libc::c_void;
    rbattr.pool_alloc = None;
    rbattr.pool_free = None;
    rbattr
        .cmp = Some(
        mln_thread_rbtree_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr.data_free = None;
    thread_tree = mln_rbtree_new(&mut rbattr);
    if thread_tree.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_thread_rbtree_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *mut mln_thread_t = data1 as *mut mln_thread_t;
    let mut t2: *mut mln_thread_t = data2 as *mut mln_thread_t;
    return mln_string_strcmp((*t1).alias, (*t2).alias);
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_module_set(
    mut modules: *mut mln_thread_module_t,
    mut num: mln_size_t,
) {
    module_array = modules;
    module_array_num = num;
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_exit(mut exit_code: libc::c_int) {
    close((*m_thread).peerfd);
    (*m_thread).peerfd = -(1 as libc::c_int);
    let mut ec: intptr_t = exit_code as intptr_t;
    pthread_exit(ec as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_kill(mut alias: *mut mln_string_t) {
    let mut t: *mut mln_thread_t = 0 as *mut mln_thread_t;
    let mut tmp: mln_thread_t = mln_thread_t {
        ev: 0 as *mut mln_event_t,
        thread_main: None,
        alias: 0 as *mut mln_string_t,
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        peerfd: 0,
        is_created: 0,
        stype: THREAD_RESTART,
        tid: 0,
        conn: mln_tcp_conn_t {
            pool: 0 as *mut mln_alloc_t,
            rcv_head: 0 as *mut mln_chain_t,
            rcv_tail: 0 as *mut mln_chain_t,
            snd_head: 0 as *mut mln_chain_t,
            snd_tail: 0 as *mut mln_chain_t,
            sent_head: 0 as *mut mln_chain_t,
            sent_tail: 0 as *mut mln_chain_t,
            sockfd: 0,
        },
        local_head: 0 as *mut mln_thread_msgq_t,
        local_tail: 0 as *mut mln_thread_msgq_t,
        dest_head: 0 as *mut mln_thread_msgq_t,
        dest_tail: 0 as *mut mln_thread_msgq_t,
        node: 0 as *mut mln_rbtree_node_t,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    tmp.alias = alias;
    rn = mln_rbtree_search(
        thread_tree,
        &mut tmp as *mut mln_thread_t as *mut libc::c_void,
    );
    if rn == &mut (*thread_tree).nil as *mut mln_rbtree_node_t {
        return;
    }
    t = (*rn).data as *mut mln_thread_t;
    close((*t).peerfd);
    (*t).peerfd = -(1 as libc::c_int);
    pthread_cancel((*t).tid);
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_cleanup_set(
    mut tcleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    thread_cleanup = tcleanup;
    thread_data = data;
}
#[inline]
unsafe extern "C" fn msg_dest_chain_del(
    mut head: *mut *mut mln_thread_msgq_t,
    mut tail: *mut *mut mln_thread_msgq_t,
    mut node: *mut mln_thread_msgq_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_thread_msgq_t;
            *head = *tail;
        } else {
            *head = (*node).dest_next;
            (**head).dest_prev = 0 as *mut mln_thread_msgq_s;
        }
    } else if *tail == node {
        *tail = (*node).dest_prev;
        (**tail).dest_next = 0 as *mut mln_thread_msgq_s;
    } else {
        (*(*node).dest_prev).dest_next = (*node).dest_next;
        (*(*node).dest_next).dest_prev = (*node).dest_prev;
    }
    (*node).dest_next = 0 as *mut mln_thread_msgq_s;
    (*node).dest_prev = (*node).dest_next;
}
#[inline]
unsafe extern "C" fn msg_dest_chain_add(
    mut head: *mut *mut mln_thread_msgq_t,
    mut tail: *mut *mut mln_thread_msgq_t,
    mut node: *mut mln_thread_msgq_t,
) {
    (*node).dest_next = 0 as *mut mln_thread_msgq_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).dest_next = node;
    }
    (*node).dest_prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn msg_local_chain_add(
    mut head: *mut *mut mln_thread_msgq_t,
    mut tail: *mut *mut mln_thread_msgq_t,
    mut node: *mut mln_thread_msgq_t,
) {
    (*node).local_next = 0 as *mut mln_thread_msgq_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).local_next = node;
    }
    (*node).local_prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn msg_local_chain_del(
    mut head: *mut *mut mln_thread_msgq_t,
    mut tail: *mut *mut mln_thread_msgq_t,
    mut node: *mut mln_thread_msgq_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_thread_msgq_t;
            *head = *tail;
        } else {
            *head = (*node).local_next;
            (**head).local_prev = 0 as *mut mln_thread_msgq_s;
        }
    } else if *tail == node {
        *tail = (*node).local_prev;
        (**tail).local_next = 0 as *mut mln_thread_msgq_s;
    } else {
        (*(*node).local_prev).local_next = (*node).local_next;
        (*(*node).local_next).local_prev = (*node).local_prev;
    }
    (*node).local_next = 0 as *mut mln_thread_msgq_s;
    (*node).local_prev = (*node).local_next;
}
unsafe extern "C" fn mln_thread_msgq_init(
    mut sender: *mut mln_thread_t,
    mut msg: *mut mln_thread_msg_t,
) -> *mut mln_thread_msgq_t {
    let mut tmq: *mut mln_thread_msgq_t = malloc(
        ::core::mem::size_of::<mln_thread_msgq_t>() as libc::c_ulong,
    ) as *mut mln_thread_msgq_t;
    if tmq.is_null() {
        return 0 as *mut mln_thread_msgq_t;
    }
    (*tmq).sender = sender;
    memcpy(
        &mut (*tmq).msg as *mut mln_thread_msg_t as *mut libc::c_void,
        msg as *const libc::c_void,
        ::core::mem::size_of::<mln_thread_msg_t>() as libc::c_ulong,
    );
    return tmq;
}
unsafe extern "C" fn mln_thread_msgq_destroy(mut tmq: *mut mln_thread_msgq_t) {
    free(tmq as *mut libc::c_void);
}
