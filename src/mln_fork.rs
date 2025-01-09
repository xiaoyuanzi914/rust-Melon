use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
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
    fn mln_event_free(ev: *mut mln_event_t);
    fn mln_event_fd_set(
        event: *mut mln_event_t,
        fd: libc::c_int,
        flag: mln_u32_t,
        timeout_ms: libc::c_int,
        data: *mut libc::c_void,
        fd_handler: ev_fd_handler,
    ) -> libc::c_int;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn mln_tcp_conn_recv(tc: *mut mln_tcp_conn_t, flag: mln_u32_t) -> libc::c_int;
    fn mln_tcp_conn_send(tc: *mut mln_tcp_conn_t) -> libc::c_int;
    fn mln_tcp_conn_pop(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_remove(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_head(
        tc: *mut mln_tcp_conn_t,
        type_0: libc::c_int,
    ) -> *mut mln_chain_t;
    fn mln_tcp_conn_append(
        tc: *mut mln_tcp_conn_t,
        c: *mut mln_chain_t,
        type_0: libc::c_int,
    );
    fn mln_tcp_conn_destroy(tc: *mut mln_tcp_conn_t);
    fn mln_tcp_conn_init(tc: *mut mln_tcp_conn_t, sockfd: libc::c_int) -> libc::c_int;
    fn mln_string_const_strcmp(
        s1: *mut mln_string_t,
        s2: *mut libc::c_char,
    ) -> libc::c_int;
    fn mln_buf_new(pool: *mut mln_alloc_t) -> *mut mln_buf_t;
    fn mln_chain_new(pool: *mut mln_alloc_t) -> *mut mln_chain_t;
    fn mln_chain_pool_release(c: *mut mln_chain_t);
    fn mln_chain_pool_release_all(c: *mut mln_chain_t);
    fn mln_conf_arg_num(cc: *mut mln_conf_cmd_t) -> mln_u32_t;
    fn mln_conf() -> *mut mln_conf_t;
    fn mln_conf_cmds(
        cf: *mut mln_conf_t,
        domain: *mut libc::c_char,
        vector: *mut *mut mln_conf_cmd_t,
    );
    fn mln_conf_cmd_num(cf: *mut mln_conf_t, domain: *mut libc::c_char) -> mln_u32_t;
    fn mln_ipc_set_process_handlers() -> libc::c_int;
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
    fn mln_log_destroy();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fork_s {
    pub prev: *mut mln_fork_s,
    pub next: *mut mln_fork_s,
    pub args: *mut mln_s8ptr_t,
    pub conn: mln_tcp_conn_t,
    pub pid: pid_t,
    pub n_args: mln_u32_t,
    pub state: mln_u32_t,
    pub msg_len: mln_u32_t,
    pub msg_type: mln_u32_t,
    pub error_bytes: mln_size_t,
    pub msg_content: *mut libc::c_void,
    pub etype: proc_exec_type,
    pub stype: proc_state_type,
}
pub type proc_state_type = libc::c_uint;
pub const M_PST_SUP: proc_state_type = 1;
pub const M_PST_DFL: proc_state_type = 0;
pub type proc_exec_type = libc::c_uint;
pub const M_PET_EXE: proc_exec_type = 1;
pub const M_PET_DFL: proc_exec_type = 0;
pub type mln_fork_t = mln_fork_s;
pub type clr_handler = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type fork_iterate_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_event_t,
        *mut mln_fork_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type ipc_handler = Option::<
    unsafe extern "C" fn(
        *mut mln_event_t,
        *mut libc::c_void,
        *mut libc::c_void,
        mln_u32_t,
        *mut *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_ipc_handler_t {
    pub handler: ipc_handler,
    pub data: *mut libc::c_void,
    pub type_0: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fork_attr {
    pub args: *mut mln_s8ptr_t,
    pub n_args: mln_u32_t,
    pub fd: libc::c_int,
    pub pid: pid_t,
    pub etype: proc_exec_type,
    pub stype: proc_state_type,
}
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
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
    pub val: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
        current_block_8 = 3440343322420171078;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 3440343322420171078;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        3440343322420171078 => {
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
                    current_block = 9856523195133516851;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                9856523195133516851 => {}
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
#[no_mangle]
pub static mut master_conn: mln_tcp_conn_t = mln_tcp_conn_t {
    pool: 0 as *const mln_alloc_t as *mut mln_alloc_t,
    rcv_head: 0 as *const mln_chain_t as *mut mln_chain_t,
    rcv_tail: 0 as *const mln_chain_t as *mut mln_chain_t,
    snd_head: 0 as *const mln_chain_t as *mut mln_chain_t,
    snd_tail: 0 as *const mln_chain_t as *mut mln_chain_t,
    sent_head: 0 as *const mln_chain_t as *mut mln_chain_t,
    sent_tail: 0 as *const mln_chain_t as *mut mln_chain_t,
    sockfd: 0,
};
#[no_mangle]
pub static mut child_error_bytes: mln_size_t = 0;
#[no_mangle]
pub static mut child_state: mln_u32_t = 0;
#[no_mangle]
pub static mut cur_msg_len: mln_u32_t = 0;
#[no_mangle]
pub static mut child_msg_content: mln_u8ptr_t = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut cur_msg_type: mln_u32_t = 0;
#[no_mangle]
pub static mut worker_list_head: *mut mln_fork_t = 0 as *const mln_fork_t
    as *mut mln_fork_t;
#[no_mangle]
pub static mut worker_list_tail: *mut mln_fork_t = 0 as *const mln_fork_t
    as *mut mln_fork_t;
#[no_mangle]
pub static mut master_ipc_tree: *mut mln_rbtree_t = 0 as *const mln_rbtree_t
    as *mut mln_rbtree_t;
#[no_mangle]
pub static mut worker_ipc_tree: *mut mln_rbtree_t = 0 as *const mln_rbtree_t
    as *mut mln_rbtree_t;
#[no_mangle]
pub static mut rs_clr_handler: clr_handler = None;
#[no_mangle]
pub static mut rs_clr_data: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn mln_fork_prepare() -> libc::c_int {
    if signal(
        17 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    )
        == ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            71 as libc::c_int,
            b"signal() to ignore SIGCHLD failed, %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if signal(
        13 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    )
        == ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            75 as libc::c_int,
            b"signal() to ignore SIGPIPE failed, %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if mln_tcp_conn_init(&mut master_conn, -(1 as libc::c_int)) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            79 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    child_state = 0 as libc::c_int as mln_u32_t;
    child_error_bytes = 0 as libc::c_int as mln_size_t;
    cur_msg_len = 0 as libc::c_int as mln_u32_t;
    child_msg_content = 0 as mln_u8ptr_t;
    cur_msg_type = 0 as libc::c_int as mln_u32_t;
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
        mln_fork_rbtree_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr
        .data_free = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut mln_ipc_handler_t) -> ()>,
        rbtree_free_data,
    >(Some(mln_ipc_handler_free as unsafe extern "C" fn(*mut mln_ipc_handler_t) -> ()));
    master_ipc_tree = mln_rbtree_new(&mut rbattr);
    if master_ipc_tree.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            94 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if master_conn.sockfd >= 0 as libc::c_int {
            close(master_conn.sockfd);
        }
        mln_tcp_conn_destroy(&mut master_conn);
        return -(1 as libc::c_int);
    }
    worker_ipc_tree = mln_rbtree_new(&mut rbattr);
    if worker_ipc_tree.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            101 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_rbtree_free(master_ipc_tree);
        master_ipc_tree = 0 as *mut mln_rbtree_t;
        if master_conn.sockfd >= 0 as libc::c_int {
            close(master_conn.sockfd);
        }
        mln_tcp_conn_destroy(&mut master_conn);
        return -(1 as libc::c_int);
    }
    if mln_ipc_set_process_handlers() < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_fork_prepare\0"))
                .as_ptr(),
            110 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_rbtree_free(worker_ipc_tree);
        worker_ipc_tree = 0 as *mut mln_rbtree_t;
        mln_rbtree_free(master_ipc_tree);
        master_ipc_tree = 0 as *mut mln_rbtree_t;
        if master_conn.sockfd >= 0 as libc::c_int {
            close(master_conn.sockfd);
        }
        mln_tcp_conn_destroy(&mut master_conn);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_fork_rbtree_cmp(
    mut k1: *const libc::c_void,
    mut k2: *const libc::c_void,
) -> libc::c_int {
    let mut ih1: *mut mln_ipc_handler_t = k1 as *mut mln_ipc_handler_t;
    let mut ih2: *mut mln_ipc_handler_t = k2 as *mut mln_ipc_handler_t;
    if (*ih1).type_0 > (*ih2).type_0 {
        return 1 as libc::c_int
    } else if (*ih1).type_0 == (*ih2).type_0 {
        return 0 as libc::c_int
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_fork_init(mut attr: *mut mln_fork_attr) -> *mut mln_fork_t {
    let mut f: *mut mln_fork_t = malloc(
        ::core::mem::size_of::<mln_fork_t>() as libc::c_ulong,
    ) as *mut mln_fork_t;
    if f.is_null() {
        return 0 as *mut mln_fork_t;
    }
    (*f).prev = 0 as *mut mln_fork_s;
    (*f).next = 0 as *mut mln_fork_s;
    (*f).args = (*attr).args;
    if mln_tcp_conn_init(&mut (*f).conn, (*attr).fd) < 0 as libc::c_int {
        free(f as *mut libc::c_void);
        return 0 as *mut mln_fork_t;
    }
    (*f).pid = (*attr).pid;
    (*f).n_args = (*attr).n_args;
    (*f).state = 0 as libc::c_int as mln_u32_t;
    (*f).msg_len = 0 as libc::c_int as mln_u32_t;
    (*f).msg_type = 0 as libc::c_int as mln_u32_t;
    (*f).error_bytes = 0 as libc::c_int as mln_size_t;
    (*f).msg_content = 0 as *mut libc::c_void;
    (*f).etype = (*attr).etype;
    (*f).stype = (*attr).stype;
    worker_list_chain_add(&mut worker_list_head, &mut worker_list_tail, f);
    return f;
}
unsafe extern "C" fn mln_fork_destroy(
    mut f: *mut mln_fork_t,
    mut free_args: libc::c_int,
) {
    if f.is_null() {
        return;
    }
    if !((*f).args).is_null() && free_args != 0 {
        free((*f).args as *mut libc::c_void);
        (*f).args = 0 as *mut mln_s8ptr_t;
    }
    if !((*f).msg_content).is_null() {
        free((*f).msg_content);
    }
    if (*f).conn.sockfd >= 0 as libc::c_int {
        close((*f).conn.sockfd);
    }
    mln_tcp_conn_destroy(&mut (*f).conn);
    worker_list_chain_del(&mut worker_list_head, &mut worker_list_tail, f);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn mln_fork_destroy_all() {
    let mut f: *mut mln_fork_t = 0 as *mut mln_fork_t;
    loop {
        f = worker_list_head;
        if f.is_null() {
            break;
        }
        mln_fork_destroy(f, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork() -> libc::c_int {
    let mut cf: *mut mln_conf_t = mln_conf();
    if cf.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                .as_ptr(),
            184 as libc::c_int,
            b"configuration crashed.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cd.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                .as_ptr(),
            189 as libc::c_int,
            b"Domain 'main' NOT existed.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
    let mut n_worker_proc: mln_sauto_t = 0 as libc::c_int as mln_sauto_t;
    let mut cmd: *mut mln_conf_cmd_t = ((*cd).search)
        .expect(
            "non-null function pointer",
        )(cd, b"worker_proc\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !cmd.is_null() {
        if mln_conf_arg_num(cmd) > 1 as libc::c_int as libc::c_uint {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                196 as libc::c_int,
                b"Too many arguments follow 'worker_proc'.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        let mut ci: *mut mln_conf_item_t = ((*cmd).search)
            .expect("non-null function pointer")(cmd, 1 as libc::c_int as mln_u32_t);
        if ci.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                201 as libc::c_int,
                b"'worker_proc' need an integer argument.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if (*ci).type_0 as libc::c_uint != CONF_INT as libc::c_int as libc::c_uint {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                205 as libc::c_int,
                b"'worker_proc' need an integer argument.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        n_worker_proc = (*ci).val.i;
        if n_worker_proc < 0 as libc::c_int as libc::c_long {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                210 as libc::c_int,
                b"Invalid value to 'worker_process'.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if do_fork_worker_process(n_worker_proc) == 0 {
        return 0 as libc::c_int;
    }
    let mut v: *mut *mut mln_conf_cmd_t = 0 as *mut *mut mln_conf_cmd_t;
    let mut cc: *mut *mut mln_conf_cmd_t = 0 as *mut *mut mln_conf_cmd_t;
    let mut i: mln_u32_t = 0;
    let mut n_args: mln_u32_t = 0;
    let mut arg_ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    let mut v_args: *mut mln_s8ptr_t = 0 as *mut mln_s8ptr_t;
    let mut n: mln_u32_t = mln_conf_cmd_num(
        cf,
        b"proc_exec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if n == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    v = calloc(
        n.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ::core::mem::size_of::<*mut mln_conf_cmd_t>() as libc::c_ulong,
    ) as *mut *mut mln_conf_cmd_t;
    if v.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                .as_ptr(),
            225 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_conf_cmds(
        cf,
        b"proc_exec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        v,
    );
    cc = v;
    while !(*cc).is_null() {
        n_args = mln_conf_arg_num(*cc);
        if n_args == 0 as libc::c_int as libc::c_uint {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                232 as libc::c_int,
                b"Demand arguments in 'proc_exec'.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        v_args = calloc(
            n_args.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ::core::mem::size_of::<mln_s8ptr_t>() as libc::c_ulong,
        ) as *mut mln_s8ptr_t;
        if v_args.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mln_fork\0"))
                    .as_ptr(),
                237 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            i = 0 as libc::c_int as mln_u32_t;
            while i < n_args {
                arg_ci = ((**cc).search)
                    .expect(
                        "non-null function pointer",
                    )(*cc, i.wrapping_add(1 as libc::c_int as libc::c_uint));
                if (*arg_ci).type_0 as libc::c_uint
                    != CONF_STR as libc::c_int as libc::c_uint
                {
                    _mln_sys_log(
                        error,
                        b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 9],
                            &[libc::c_char; 9],
                        >(b"mln_fork\0"))
                            .as_ptr(),
                        243 as libc::c_int,
                        b"Demand string arguments in 'proc_exec'.\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                let ref mut fresh0 = *v_args.offset(i as isize);
                *fresh0 = (*(*arg_ci).val.s).data as *mut libc::c_char;
                i = i.wrapping_add(1);
                i;
            }
            if mln_string_const_strcmp(
                (**cc).cmd_name,
                b"keepalive\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                mln_fork_spawn(M_PST_SUP, v_args, n_args, 0 as *mut mln_event_t);
            } else if mln_string_const_strcmp(
                (**cc).cmd_name,
                b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                mln_fork_spawn(M_PST_DFL, v_args, n_args, 0 as *mut mln_event_t);
            } else {
                _mln_sys_log(
                    error,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 9],
                        &[libc::c_char; 9],
                    >(b"mln_fork\0"))
                        .as_ptr(),
                    253 as libc::c_int,
                    b"Invalid command '%S' in 'proc_exec'.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (**cc).cmd_name,
                );
                exit(1 as libc::c_int);
            }
        }
        cc = cc.offset(1);
        cc;
    }
    free(v as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_spawn(
    mut stype: proc_state_type,
    mut args: *mut mln_s8ptr_t,
    mut n_args: mln_u32_t,
    mut master_ev: *mut mln_event_t,
) -> libc::c_int {
    _mln_sys_log(
        none,
        b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"mln_fork_spawn\0"))
            .as_ptr(),
        265 as libc::c_int,
        b"Start up process '%s'\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        *args.offset(0 as libc::c_int as isize),
    );
    let mut ret: libc::c_int = do_fork_core(M_PET_EXE, stype, args, n_args, master_ev);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else if ret == 0 as libc::c_int {
        let mut fd_str: [libc::c_char; 256] = [
            0 as libc::c_int as libc::c_char,
        ];
        snprintf(
            fd_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%d\0" as *const u8 as *const libc::c_char,
            master_conn.sockfd,
        );
        let ref mut fresh1 = *args.offset(n_args as isize);
        *fresh1 = fd_str.as_mut_ptr();
        if !master_ev.is_null() {
            mln_event_free(master_ev);
        }
        mln_log_destroy();
        if execv(
            *args.offset(0 as libc::c_int as isize) as *const libc::c_char,
            args as *const *mut libc::c_char,
        ) < 0 as libc::c_int
        {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"mln_fork_spawn\0"))
                    .as_ptr(),
                281 as libc::c_int,
                b"execv error, %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_restart(
    mut master_ev: *mut mln_event_t,
) -> libc::c_int {
    return do_fork_core(
        M_PET_DFL,
        M_PST_SUP,
        0 as *mut mln_s8ptr_t,
        0 as libc::c_int as mln_u32_t,
        master_ev,
    );
}
unsafe extern "C" fn do_fork_worker_process(
    mut n_worker_proc: mln_sauto_t,
) -> libc::c_int {
    let mut i: mln_sauto_t = 0;
    let mut ret: libc::c_int = 0;
    i = 0 as libc::c_int as mln_sauto_t;
    while i < n_worker_proc {
        _mln_sys_log(
            none,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"do_fork_worker_process\0"))
                .as_ptr(),
            300 as libc::c_int,
            b"Start up worker process No.%l\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            i + 1 as libc::c_int as libc::c_long,
        );
        ret = mln_fork_restart(0 as *mut mln_event_t);
        if !(ret < 0 as libc::c_int) {
            if ret == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_resource_clear_handler_set(
    mut handler: clr_handler,
    mut data: *mut libc::c_void,
) {
    rs_clr_handler = handler;
    rs_clr_data = data;
}
unsafe extern "C" fn do_fork_core(
    mut etype: proc_exec_type,
    mut stype: proc_state_type,
    mut args: *mut mln_s8ptr_t,
    mut n_args: mln_u32_t,
    mut master_ev: *mut mln_event_t,
) -> libc::c_int {
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut c: mln_u8_t = 0;
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"do_fork_core\0"))
                .as_ptr(),
            324 as libc::c_int,
            b"socketpair() error. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    let mut pid: pid_t = fork();
    if pid > 0 as libc::c_int {
        close(fds[1 as libc::c_int as usize]);
        while read(
            fds[0 as libc::c_int as usize],
            &mut c as *mut mln_u8_t as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) <= 0 as libc::c_int as libc::c_long
        {}
        let mut fattr: mln_fork_attr = mln_fork_attr {
            args: 0 as *mut mln_s8ptr_t,
            n_args: 0,
            fd: 0,
            pid: 0,
            etype: M_PET_DFL,
            stype: M_PST_DFL,
        };
        fattr.args = args;
        fattr.n_args = n_args;
        fattr.fd = fds[0 as libc::c_int as usize];
        fattr.pid = pid;
        fattr.etype = etype;
        fattr.stype = stype;
        let mut f: *mut mln_fork_t = mln_fork_init(&mut fattr);
        if f.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"do_fork_core\0"))
                    .as_ptr(),
                357 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            abort();
        }
        if !master_ev.is_null() {
            if mln_event_fd_set(
                master_ev,
                (*f).conn.sockfd,
                0x1 as libc::c_int as mln_u32_t,
                -(1 as libc::c_int),
                f as *mut libc::c_void,
                Some(
                    mln_ipc_fd_handler_master
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
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"do_fork_core\0"))
                        .as_ptr(),
                    368 as libc::c_int,
                    b"mln_event_fd_set() failed.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
        }
        return 1 as libc::c_int;
    } else if pid == 0 as libc::c_int {
        close(fds[0 as libc::c_int as usize]);
        mln_fork_destroy_all();
        mln_rbtree_free(master_ipc_tree);
        if rs_clr_handler.is_some() {
            rs_clr_handler.expect("non-null function pointer")(rs_clr_data);
        }
        master_ipc_tree = 0 as *mut mln_rbtree_t;
        master_conn.sockfd = fds[1 as libc::c_int as usize];
        signal(17 as libc::c_int, None);
        if write(
            fds[1 as libc::c_int as usize],
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) < 0 as libc::c_int as libc::c_long
        {
            exit(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    _mln_sys_log(
        error,
        b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"do_fork_core\0"))
            .as_ptr(),
        386 as libc::c_int,
        b"fork() error. %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strerror(*__errno_location()),
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_master_ipc_handler_set(
    mut type_0: mln_u32_t,
    mut handler: ipc_handler,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut ih: *mut mln_ipc_handler_t = mln_ipc_handler_new(type_0, handler, data);
    if ih.is_null() {
        return -(1 as libc::c_int);
    }
    let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
        master_ipc_tree,
        ih as *mut libc::c_void,
    );
    if !((*rn).data).is_null() {
        mln_rbtree_delete(master_ipc_tree, rn);
        mln_rbtree_node_free(master_ipc_tree, rn);
    }
    rn = mln_rbtree_node_new(master_ipc_tree, ih as *mut libc::c_void);
    if rn.is_null() {
        mln_ipc_handler_free(ih);
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(master_ipc_tree, rn);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_worker_ipc_handler_set(
    mut type_0: mln_u32_t,
    mut handler: ipc_handler,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut ih: *mut mln_ipc_handler_t = mln_ipc_handler_new(type_0, handler, data);
    if ih.is_null() {
        return -(1 as libc::c_int);
    }
    let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
        worker_ipc_tree,
        ih as *mut libc::c_void,
    );
    if !((*rn).data).is_null() {
        mln_rbtree_delete(worker_ipc_tree, rn);
        mln_rbtree_node_free(worker_ipc_tree, rn);
    }
    rn = mln_rbtree_node_new(worker_ipc_tree, ih as *mut libc::c_void);
    if rn.is_null() {
        mln_ipc_handler_free(ih);
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(worker_ipc_tree, rn);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_ipc_handler_new(
    mut type_0: mln_u32_t,
    mut handler: ipc_handler,
    mut data: *mut libc::c_void,
) -> *mut mln_ipc_handler_t {
    let mut ih: *mut mln_ipc_handler_t = 0 as *mut mln_ipc_handler_t;
    ih = malloc(::core::mem::size_of::<mln_ipc_handler_t>() as libc::c_ulong)
        as *mut mln_ipc_handler_t;
    if ih.is_null() {
        return 0 as *mut mln_ipc_handler_t;
    }
    (*ih).handler = handler;
    (*ih).data = data;
    (*ih).type_0 = type_0;
    return ih;
}
unsafe extern "C" fn mln_ipc_handler_free(mut ih: *mut mln_ipc_handler_t) {
    if ih.is_null() {
        return;
    }
    free(ih as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_master_events_set(mut ev: *mut mln_event_t) {
    let mut f: *mut mln_fork_t = 0 as *mut mln_fork_t;
    f = worker_list_head;
    while !f.is_null() {
        if mln_event_fd_set(
            ev,
            (*f).conn.sockfd,
            0x1 as libc::c_int as mln_u32_t | 0x10 as libc::c_int as mln_u32_t,
            -(1 as libc::c_int),
            f as *mut libc::c_void,
            Some(
                mln_ipc_fd_handler_master
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
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"mln_fork_master_events_set\0"))
                    .as_ptr(),
                464 as libc::c_int,
                b"mln_event_fd_set() failed.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            abort();
        }
        f = (*f).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_worker_events_set(mut ev: *mut mln_event_t) {
    if mln_event_fd_set(
        ev,
        master_conn.sockfd,
        0x1 as libc::c_int as mln_u32_t | 0x10 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        0 as *mut libc::c_void,
        Some(
            mln_ipc_fd_handler_worker
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
            b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"mln_fork_worker_events_set\0"))
                .as_ptr(),
            478 as libc::c_int,
            b"mln_event_fd_set() failed.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_iterate(
    mut ev: *mut mln_event_t,
    mut handler: fork_iterate_handler,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut f: *mut mln_fork_t = 0 as *mut mln_fork_t;
    f = worker_list_head;
    while !f.is_null() {
        if handler.is_some() {
            if handler.expect("non-null function pointer")(ev, f, data)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        f = (*f).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_master_connection_get() -> *mut mln_tcp_conn_t {
    return &mut master_conn;
}
#[no_mangle]
pub unsafe extern "C" fn mln_ipc_fd_handler_master(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut ret: libc::c_int = 0;
    let mut f: *mut mln_fork_t = data as *mut mln_fork_t;
    let mut conn: *mut mln_tcp_conn_t = &mut (*f).conn;
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
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"mln_ipc_fd_handler_master\0"))
                    .as_ptr(),
                523 as libc::c_int,
                b"Child process dead!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            mln_fork_socketpair_close_handler(ev, f, fd);
            return;
        } else {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"mln_ipc_fd_handler_master\0"))
                    .as_ptr(),
                527 as libc::c_int,
                b"recv msg error. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            mln_fork_socketpair_close_handler(ev, f, fd);
            return;
        }
    }
    mln_ipc_fd_handler_master_process(ev, f);
}
#[inline]
unsafe extern "C" fn mln_ipc_get_buf_with_len(
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
unsafe extern "C" fn mln_ipc_discard_bytes(
    mut tc: *mut mln_tcp_conn_t,
    mut size: mln_size_t,
) -> mln_size_t {
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut left_size: mln_size_t = 0;
    loop {
        c = mln_tcp_conn_head(tc, 2 as libc::c_int);
        if c.is_null() {
            break;
        }
        b = (*c).buf;
        if b.is_null() || ((*b).pos).is_null() {
            mln_chain_pool_release(mln_tcp_conn_pop(tc, 2 as libc::c_int));
        } else {
            left_size = (if b.is_null() {
                0 as libc::c_int as libc::c_long
            } else if (*b).in_file() as libc::c_int != 0 {
                (*b).file_last - (*b).file_left_pos
            } else {
                ((*b).last).offset_from((*b).left_pos) as libc::c_long
            }) as mln_size_t;
            if left_size > size {
                (*b).left_pos = ((*b).left_pos).offset(size as isize);
                size = 0 as libc::c_int as mln_size_t;
                break;
            } else {
                (*b).left_pos = ((*b).left_pos).offset(left_size as isize);
                size = (size as libc::c_ulong).wrapping_sub(left_size) as mln_size_t
                    as mln_size_t;
                mln_chain_pool_release(mln_tcp_conn_pop(tc, 2 as libc::c_int));
                if size == 0 as libc::c_int as libc::c_ulong {
                    return 0 as libc::c_int as mln_size_t;
                }
            }
        }
    }
    return size;
}
unsafe extern "C" fn mln_ipc_fd_handler_master_process(
    mut ev: *mut mln_event_t,
    mut f: *mut mln_fork_t,
) {
    let mut tc: *mut mln_tcp_conn_t = &mut (*f).conn;
    loop {
        while (*f).error_bytes != 0 {
            (*f).error_bytes = mln_ipc_discard_bytes(tc, (*f).error_bytes);
        }
        let mut current_block_24: u64;
        match (*f).state {
            0 => {
                if mln_ipc_get_buf_with_len(
                    tc,
                    &mut (*f).msg_len as *mut mln_u32_t as *mut libc::c_void,
                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    return;
                }
                (*f).state = 1 as libc::c_int as mln_u32_t;
            }
            1 => {}
            _ => {
                _mln_sys_log(
                    error,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"mln_ipc_fd_handler_master_process\0"))
                        .as_ptr(),
                    652 as libc::c_int,
                    b"No such state!\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
        }
        if ((*f).msg_content).is_null() {
            (*f).msg_content = malloc((*f).msg_len as libc::c_ulong);
            if ((*f).msg_content).is_null() {
                (*f).error_bytes = (*f).msg_len as mln_size_t;
                (*f).state = 0 as libc::c_int as mln_u32_t;
                current_block_24 = 15768484401365413375;
            } else {
                current_block_24 = 2979737022853876585;
            }
        } else {
            current_block_24 = 2979737022853876585;
        }
        match current_block_24 {
            2979737022853876585 => {
                if mln_ipc_get_buf_with_len(
                    tc,
                    (*f).msg_content,
                    (*f).msg_len as mln_size_t,
                ) < 0 as libc::c_int
                {
                    return;
                }
                memcpy(
                    &mut (*f).msg_type as *mut mln_u32_t as *mut libc::c_void,
                    (*f).msg_content,
                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                );
                (*f).state = 0 as libc::c_int as mln_u32_t;
                let mut ih: mln_ipc_handler_t = mln_ipc_handler_t {
                    handler: None,
                    data: 0 as *mut libc::c_void,
                    type_0: 0,
                };
                ih.type_0 = (*f).msg_type;
                let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
                    master_ipc_tree,
                    &mut ih as *mut mln_ipc_handler_t as *mut libc::c_void,
                );
                if !(rn == &mut (*master_ipc_tree).nil as *mut mln_rbtree_node_t) {
                    let mut ihp: *mut mln_ipc_handler_t = (*rn).data
                        as *mut mln_ipc_handler_t;
                    if ((*ihp).handler).is_some() {
                        ((*ihp).handler)
                            .expect(
                                "non-null function pointer",
                            )(
                            ev,
                            f as *mut libc::c_void,
                            ((*f).msg_content)
                                .offset(
                                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
                                        as isize,
                                ),
                            ((*f).msg_len as libc::c_ulong)
                                .wrapping_sub(
                                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                                ) as mln_u32_t,
                            &mut (*ihp).data,
                        );
                    }
                }
                free((*f).msg_content);
                (*f).msg_content = 0 as *mut libc::c_void;
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_fork_socketpair_close_handler(
    mut ev: *mut mln_event_t,
    mut f: *mut mln_fork_t,
    mut fd: libc::c_int,
) {
    mln_event_fd_set(
        ev,
        fd,
        0x80 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        0 as *mut libc::c_void,
        None,
    );
    let mut etype: proc_exec_type = (*f).etype;
    let mut stype: proc_state_type = (*f).stype;
    let mut args: *mut mln_s8ptr_t = (*f).args;
    let mut n_args: mln_u32_t = (*f).n_args;
    if stype as libc::c_uint == M_PST_SUP as libc::c_int as libc::c_uint {
        mln_fork_destroy(f, 0 as libc::c_int);
        if etype as libc::c_uint == M_PET_DFL as libc::c_int as libc::c_uint {
            let mut rv: libc::c_int = mln_fork_restart(ev);
            if rv < 0 as libc::c_int {
                _mln_sys_log(
                    error,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"mln_fork_socketpair_close_handler\0"))
                        .as_ptr(),
                    672 as libc::c_int,
                    b"mln_fork_restart() error.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            } else if rv == 0 as libc::c_int {
                (*ev).set_is_break(1 as libc::c_int as mln_u32_t);
            } else {
                _mln_sys_log(
                    report,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"mln_fork_socketpair_close_handler\0"))
                        .as_ptr(),
                    677 as libc::c_int,
                    b"Child process restart.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        } else {
            if mln_fork_spawn(stype, args, n_args, ev) < 0 as libc::c_int {
                _mln_sys_log(
                    error,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"mln_fork_socketpair_close_handler\0"))
                        .as_ptr(),
                    681 as libc::c_int,
                    b"mln_fork_spawn() error.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
            _mln_sys_log(
                report,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"mln_fork_socketpair_close_handler\0"))
                    .as_ptr(),
                684 as libc::c_int,
                b"Process %s restart.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                *args.offset(0 as libc::c_int as isize),
            );
        }
    } else {
        mln_fork_destroy(f, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_ipc_fd_handler_worker(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut ret: libc::c_int = 0;
    let mut conn: *mut mln_tcp_conn_t = &mut master_conn;
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
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"mln_ipc_fd_handler_worker\0"))
                    .as_ptr(),
                702 as libc::c_int,
                b"Master process dead!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            exit(127 as libc::c_int);
        } else {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"mln_ipc_fd_handler_worker\0"))
                    .as_ptr(),
                705 as libc::c_int,
                b"recv msg error. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            exit(127 as libc::c_int);
        }
    }
    mln_ipc_fd_handler_worker_process(ev, conn);
}
unsafe extern "C" fn mln_ipc_fd_handler_worker_process(
    mut ev: *mut mln_event_t,
    mut tc: *mut mln_tcp_conn_t,
) {
    loop {
        while child_error_bytes != 0 {
            child_error_bytes = mln_ipc_discard_bytes(tc, child_error_bytes);
        }
        let mut current_block_24: u64;
        match child_state {
            0 => {
                if mln_ipc_get_buf_with_len(
                    tc,
                    &mut cur_msg_len as *mut mln_u32_t as *mut libc::c_void,
                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    return;
                }
                child_state = 1 as libc::c_int as mln_u32_t;
            }
            1 => {}
            _ => {
                _mln_sys_log(
                    error,
                    b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"mln_ipc_fd_handler_worker_process\0"))
                        .as_ptr(),
                    760 as libc::c_int,
                    b"No such state!\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
        }
        if child_msg_content.is_null() {
            child_msg_content = malloc(cur_msg_len as libc::c_ulong) as mln_u8ptr_t;
            if child_msg_content.is_null() {
                child_error_bytes = cur_msg_len as mln_size_t;
                child_state = 0 as libc::c_int as mln_u32_t;
                current_block_24 = 17478428563724192186;
            } else {
                current_block_24 = 1841672684692190573;
            }
        } else {
            current_block_24 = 1841672684692190573;
        }
        match current_block_24 {
            1841672684692190573 => {
                if mln_ipc_get_buf_with_len(
                    tc,
                    child_msg_content as *mut libc::c_void,
                    cur_msg_len as mln_size_t,
                ) < 0 as libc::c_int
                {
                    return;
                }
                memcpy(
                    &mut cur_msg_type as *mut mln_u32_t as *mut libc::c_void,
                    child_msg_content as *const libc::c_void,
                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                );
                child_state = 0 as libc::c_int as mln_u32_t;
                let mut ih: mln_ipc_handler_t = mln_ipc_handler_t {
                    handler: None,
                    data: 0 as *mut libc::c_void,
                    type_0: 0,
                };
                ih.type_0 = cur_msg_type;
                let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
                    worker_ipc_tree,
                    &mut ih as *mut mln_ipc_handler_t as *mut libc::c_void,
                );
                if !(rn == &mut (*worker_ipc_tree).nil as *mut mln_rbtree_node_t) {
                    let mut ihp: *mut mln_ipc_handler_t = (*rn).data
                        as *mut mln_ipc_handler_t;
                    if ((*ihp).handler).is_some() {
                        ((*ihp).handler)
                            .expect(
                                "non-null function pointer",
                            )(
                            ev,
                            tc as *mut libc::c_void,
                            child_msg_content
                                .offset(
                                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
                                        as isize,
                                ) as *mut libc::c_void,
                            (cur_msg_len as libc::c_ulong)
                                .wrapping_sub(
                                    ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
                                ) as mln_u32_t,
                            &mut (*ihp).data,
                        );
                    }
                }
                free(child_msg_content as *mut libc::c_void);
                child_msg_content = 0 as mln_u8ptr_t;
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_ipc_master_send_prepare(
    mut ev: *mut mln_event_t,
    mut type_0: mln_u32_t,
    mut msg: *mut libc::c_void,
    mut len: mln_size_t,
    mut f_child: *mut mln_fork_t,
) -> libc::c_int {
    let mut length: mln_u32_t = (::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
        .wrapping_add(len) as mln_u32_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut buflen: mln_size_t = 0;
    let mut conn: *mut mln_tcp_conn_t = &mut (*f_child).conn;
    let mut pool: *mut mln_alloc_t = (*conn).pool;
    buflen = (length as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong);
    c = mln_chain_new(pool);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    b = mln_buf_new(pool);
    if b.is_null() {
        mln_chain_pool_release(c);
        return -(1 as libc::c_int);
    }
    (*c).buf = b;
    buf = mln_alloc_m(pool, buflen) as mln_u8ptr_t;
    if buf.is_null() {
        mln_chain_pool_release(c);
        return -(1 as libc::c_int);
    }
    (*b).start = buf;
    (*b).pos = (*b).start;
    (*b).left_pos = (*b).pos;
    (*b).end = buf.offset(buflen as isize);
    (*b).last = (*b).end;
    (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
    (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
    (*b).set_last_in_chain(1 as libc::c_int as mln_u32_t);
    memcpy(
        buf as *mut libc::c_void,
        &mut length as *mut mln_u32_t as *const libc::c_void,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    );
    memcpy(
        buf.offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        &mut type_0 as *mut mln_u32_t as *const libc::c_void,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    );
    memcpy(
        buf
            .offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            .offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        msg,
        len,
    );
    mln_tcp_conn_append(conn, c, 1 as libc::c_int);
    mln_event_fd_set(
        ev,
        (*conn).sockfd,
        0x2 as libc::c_int as mln_u32_t | 0x40 as libc::c_int as mln_u32_t
            | 0x10 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        f_child as *mut libc::c_void,
        Some(
            mln_ipc_fd_handler_master_send
                as unsafe extern "C" fn(
                    *mut mln_event_t,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_ipc_fd_handler_master_send(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut f: *mut mln_fork_t = data as *mut mln_fork_t;
    let mut conn: *mut mln_tcp_conn_t = &mut (*f).conn;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut ret: libc::c_int = 0;
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
                (*conn).sockfd,
                0x2 as libc::c_int as mln_u32_t | 0x40 as libc::c_int as mln_u32_t
                    | 0x10 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t,
                -(1 as libc::c_int),
                f as *mut libc::c_void,
                Some(
                    mln_ipc_fd_handler_master_send
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
                report,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"mln_ipc_fd_handler_master_send\0"))
                    .as_ptr(),
                840 as libc::c_int,
                b"Child process dead!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            mln_fork_socketpair_close_handler(ev, f, fd);
            return;
        } else {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"mln_ipc_fd_handler_master_send\0"))
                    .as_ptr(),
                844 as libc::c_int,
                b"Shouldn't be here.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            abort();
        }
    }
    mln_chain_pool_release_all(mln_tcp_conn_remove(conn, 3 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn mln_ipc_worker_send_prepare(
    mut ev: *mut mln_event_t,
    mut type_0: mln_u32_t,
    mut msg: *mut libc::c_void,
    mut len: mln_size_t,
) -> libc::c_int {
    let mut length: mln_u32_t = (::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
        .wrapping_add(len) as mln_u32_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut b: *mut mln_buf_t = 0 as *mut mln_buf_t;
    let mut buflen: mln_size_t = 0;
    let mut conn: *mut mln_tcp_conn_t = &mut master_conn;
    let mut pool: *mut mln_alloc_t = (*conn).pool;
    buflen = (length as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong);
    c = mln_chain_new(pool);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    b = mln_buf_new(pool);
    if b.is_null() {
        mln_chain_pool_release(c);
        return -(1 as libc::c_int);
    }
    (*c).buf = b;
    buf = mln_alloc_m(pool, buflen) as mln_u8ptr_t;
    if buf.is_null() {
        mln_chain_pool_release(c);
        return -(1 as libc::c_int);
    }
    (*b).start = buf;
    (*b).pos = (*b).start;
    (*b).left_pos = (*b).pos;
    (*b).end = buf.offset(buflen as isize);
    (*b).last = (*b).end;
    (*b).set_in_memory(1 as libc::c_int as mln_u32_t);
    (*b).set_last_buf(1 as libc::c_int as mln_u32_t);
    (*b).set_last_in_chain(1 as libc::c_int as mln_u32_t);
    memcpy(
        buf as *mut libc::c_void,
        &mut length as *mut mln_u32_t as *const libc::c_void,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    );
    memcpy(
        buf.offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        &mut type_0 as *mut mln_u32_t as *const libc::c_void,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    );
    memcpy(
        buf
            .offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            .offset(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        msg,
        len,
    );
    mln_tcp_conn_append(conn, c, 1 as libc::c_int);
    mln_event_fd_set(
        ev,
        (*conn).sockfd,
        0x2 as libc::c_int as mln_u32_t | 0x40 as libc::c_int as mln_u32_t
            | 0x10 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        0 as *mut libc::c_void,
        Some(
            mln_ipc_fd_handler_worker_send
                as unsafe extern "C" fn(
                    *mut mln_event_t,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_ipc_fd_handler_worker_send(
    mut ev: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut conn: *mut mln_tcp_conn_t = &mut master_conn;
    let mut c: *mut mln_chain_t = 0 as *mut mln_chain_t;
    let mut ret: libc::c_int = 0;
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
                (*conn).sockfd,
                0x2 as libc::c_int as mln_u32_t | 0x40 as libc::c_int as mln_u32_t
                    | 0x10 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t,
                -(1 as libc::c_int),
                0 as *mut libc::c_void,
                Some(
                    mln_ipc_fd_handler_worker_send
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
                report,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"mln_ipc_fd_handler_worker_send\0"))
                    .as_ptr(),
                925 as libc::c_int,
                b"master process dead!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            exit(127 as libc::c_int);
        } else {
            _mln_sys_log(
                error,
                b"src/mln_fork.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"mln_ipc_fd_handler_worker_send\0"))
                    .as_ptr(),
                928 as libc::c_int,
                b"Shouldn't be here.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            abort();
        }
    }
    mln_chain_pool_release_all(mln_tcp_conn_remove(conn, 3 as libc::c_int));
}
#[inline]
unsafe extern "C" fn worker_list_chain_del(
    mut head: *mut *mut mln_fork_t,
    mut tail: *mut *mut mln_fork_t,
    mut node: *mut mln_fork_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_fork_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_fork_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_fork_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_fork_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn worker_list_chain_add(
    mut head: *mut *mut mln_fork_t,
    mut tail: *mut *mut mln_fork_t,
    mut node: *mut mln_fork_t,
) {
    (*node).next = 0 as *mut mln_fork_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
