use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_rbtree_insert(t: *mut mln_rbtree_t, node: *mut mln_rbtree_node_t);
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn mln_rbtree_iterate(
        t: *mut mln_rbtree_t,
        handler: rbtree_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mln_stack_pop(st: *mut mln_stack_t) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn mln_ipc_handler_unregister(cb: *mut mln_ipc_cb_t);
    fn mln_ipc_handler_register(
        type_0: mln_u32_t,
        master_handler: ipc_handler,
        worker_handler: ipc_handler,
        master_data: *mut libc::c_void,
        worker_data: *mut libc::c_void,
    ) -> *mut mln_ipc_cb_t;
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn mln_string_pool_dup(
        pool: *mut mln_alloc_t,
        str: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn mln_lex_init(attr: *mut mln_lex_attr) -> *mut mln_lex_t;
    fn mln_lex_destroy(lex: *mut mln_lex_t);
    fn mln_lex_strerror(lex: *mut mln_lex_t) -> *mut libc::c_char;
    fn mln_lex_push_input_file_stream(
        lex: *mut mln_lex_t,
        path: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_push_input_buf_stream(
        lex: *mut mln_lex_t,
        buf: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_check_file_loop(
        lex: *mut mln_lex_t,
        path: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_macro_new(
        pool: *mut mln_alloc_t,
        key: *mut mln_string_t,
        val: *mut mln_string_t,
    ) -> *mut mln_lex_macro_t;
    fn mln_lex_preprocess_data_new(
        pool: *mut mln_alloc_t,
    ) -> *mut mln_lex_preprocess_data_t;
    fn mln_lex_preprocess_data_free(lpd: *mut mln_lex_preprocess_data_t);
    fn mln_lex_condition_test(lex: *mut mln_lex_t) -> libc::c_int;
    fn mln_lex_input_free(in_0: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
    fn mln_path_conf() -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub type rbtree_iterate_handler = Option::<
    unsafe extern "C" fn(*mut mln_rbtree_node_t, *mut libc::c_void) -> libc::c_int,
>;
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
pub type stack_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type stack_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_stack_node_s {
    pub data: *mut libc::c_void,
    pub prev: *mut mln_stack_node_s,
    pub next: *mut mln_stack_node_s,
}
pub type mln_stack_node_t = mln_stack_node_s;
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
pub type mln_lex_t = mln_lex_s;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lex_attr {
    pub pool: *mut mln_alloc_t,
    pub keywords: *mut mln_string_t,
    pub hooks: *mut mln_lex_hooks_t,
    #[bitfield(name = "preprocess", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "1..=31")]
    pub preprocess_padding: [u8; 4],
    pub type_0: mln_u32_t,
    pub env: *mut mln_string_t,
    pub data: *mut mln_string_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_keyword_t {
    pub keyword: *mut mln_string_t,
    pub val: mln_uauto_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_spechar_t {
    pub sc: libc::c_char,
    pub handler: lex_hook,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_preprocess_handler_t {
    pub command: mln_string_t,
    pub handler: lex_hook,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_macro_t {
    pub key: *mut mln_string_t,
    pub val: *mut mln_string_t,
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
pub type mln_ipc_cb_t = mln_ipc_cb_s;
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
pub type mln_conf_item_t = mln_conf_item_s;
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
pub type mln_conf_domain_t = mln_conf_domain_s;
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
pub type mln_conf_domain_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_conf_t, *mut libc::c_char) -> *mut mln_conf_domain_t,
>;
pub type mln_conf_t = mln_conf_s;
pub type reload_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_hook_s {
    pub reload: reload_handler,
    pub data: *mut libc::c_void,
    pub prev: *mut mln_conf_hook_s,
    pub next: *mut mln_conf_hook_s,
}
pub type mln_conf_hook_t = mln_conf_hook_s;
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_lex_struct_t {
    pub text: *mut mln_string_t,
    pub line: mln_u32_t,
    pub type_0: mln_conf_lex_enum,
    pub file: *mut mln_string_t,
}
pub type mln_conf_lex_enum = libc::c_uint;
pub const CONF_TK_STRING: mln_conf_lex_enum = 44;
pub const CONF_TK_CHAR: mln_conf_lex_enum = 43;
pub const CONF_TK_COMMENT: mln_conf_lex_enum = 42;
pub const CONF_TK_OFF: mln_conf_lex_enum = 41;
pub const CONF_TK_ON: mln_conf_lex_enum = 40;
pub const CONF_TK_KEYWORD_BEGIN: mln_conf_lex_enum = 39;
pub const CONF_TK_DASH: mln_conf_lex_enum = 38;
pub const CONF_TK_RBRACE: mln_conf_lex_enum = 37;
pub const CONF_TK_VERTL: mln_conf_lex_enum = 36;
pub const CONF_TK_LBRACE: mln_conf_lex_enum = 35;
pub const CONF_TK_FULSTP: mln_conf_lex_enum = 34;
pub const CONF_TK_UNDER: mln_conf_lex_enum = 33;
pub const CONF_TK_XOR: mln_conf_lex_enum = 32;
pub const CONF_TK_RSQUAR: mln_conf_lex_enum = 31;
pub const CONF_TK_BSLASH: mln_conf_lex_enum = 30;
pub const CONF_TK_LSQUAR: mln_conf_lex_enum = 29;
pub const CONF_TK_AT: mln_conf_lex_enum = 28;
pub const CONF_TK_QUES: mln_conf_lex_enum = 27;
pub const CONF_TK_RAGL: mln_conf_lex_enum = 26;
pub const CONF_TK_EQUAL: mln_conf_lex_enum = 25;
pub const CONF_TK_LAGL: mln_conf_lex_enum = 24;
pub const CONF_TK_SEMIC: mln_conf_lex_enum = 23;
pub const CONF_TK_COLON: mln_conf_lex_enum = 22;
pub const CONF_TK_SLASH: mln_conf_lex_enum = 21;
pub const CONF_TK_PERIOD: mln_conf_lex_enum = 20;
pub const CONF_TK_SUB: mln_conf_lex_enum = 19;
pub const CONF_TK_COMMA: mln_conf_lex_enum = 18;
pub const CONF_TK_PLUS: mln_conf_lex_enum = 17;
pub const CONF_TK_AST: mln_conf_lex_enum = 16;
pub const CONF_TK_RPAR: mln_conf_lex_enum = 15;
pub const CONF_TK_LPAR: mln_conf_lex_enum = 14;
pub const CONF_TK_SGLQ: mln_conf_lex_enum = 13;
pub const CONF_TK_AMP: mln_conf_lex_enum = 12;
pub const CONF_TK_PERC: mln_conf_lex_enum = 11;
pub const CONF_TK_DOLL: mln_conf_lex_enum = 10;
pub const CONF_TK_NUMS: mln_conf_lex_enum = 9;
pub const CONF_TK_DBLQ: mln_conf_lex_enum = 8;
pub const CONF_TK_EXCL: mln_conf_lex_enum = 7;
pub const CONF_TK_SPACE: mln_conf_lex_enum = 6;
pub const CONF_TK_ID: mln_conf_lex_enum = 5;
pub const CONF_TK_REAL: mln_conf_lex_enum = 4;
pub const CONF_TK_HEX: mln_conf_lex_enum = 3;
pub const CONF_TK_DEC: mln_conf_lex_enum = 2;
pub const CONF_TK_OCT: mln_conf_lex_enum = 1;
pub const CONF_TK_EOF: mln_conf_lex_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_cmds_scan_s {
    pub cc: *mut *mut mln_conf_cmd_t,
    pub pos: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_lex_type_t {
    pub type_0: mln_conf_lex_enum,
    pub type_str: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn mln_lex_stepback(mut lex: *mut mln_lex_t, mut c: libc::c_char) {
    if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
        return;
    }
    if ((*lex).cur).is_null() || (*(*lex).cur).pos <= (*(*lex).cur).buf {
        abort();
    }
    (*(*lex).cur).pos = ((*(*lex).cur).pos).offset(-1);
    (*(*lex).cur).pos;
}
#[inline]
unsafe extern "C" fn mln_lex_putchar(
    mut lex: *mut mln_lex_t,
    mut c: libc::c_char,
) -> libc::c_int {
    if ((*lex).result_buf).is_null() {
        (*lex)
            .result_buf = mln_alloc_m((*lex).pool, (*lex).result_buf_len) as mln_u8ptr_t;
        if ((*lex).result_buf).is_null() {
            (*lex).error = 1 as libc::c_int;
            return -(2 as libc::c_int) as libc::c_char as libc::c_int;
        }
        (*lex).result_pos = (*lex).result_buf;
    }
    if (*lex).result_pos >= ((*lex).result_buf).offset((*lex).result_buf_len as isize) {
        let mut len: mln_u64_t = ((*lex).result_buf_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        (*lex)
            .result_buf_len = ((*lex).result_buf_len as libc::c_ulong)
            .wrapping_add(len >> 1 as libc::c_int) as mln_u64_t as mln_u64_t;
        let mut tmp: mln_u8ptr_t = (*lex).result_buf;
        (*lex)
            .result_buf = mln_alloc_re(
            (*lex).pool,
            tmp as *mut libc::c_void,
            (*lex).result_buf_len,
        ) as mln_u8ptr_t;
        if ((*lex).result_buf).is_null() {
            (*lex).result_buf = tmp;
            (*lex).result_buf_len = len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            (*lex).error = 1 as libc::c_int;
            return -(2 as libc::c_int) as libc::c_char as libc::c_int;
        }
        (*lex)
            .result_pos = ((*lex).result_buf)
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize));
    }
    let fresh0 = (*lex).result_pos;
    (*lex).result_pos = ((*lex).result_pos).offset(1);
    *fresh0 = c as mln_u8_t;
    return 0 as libc::c_int;
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
                    current_block = 17809101626333725014;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                17809101626333725014 => {}
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
        current_block_8 = 9124099917951443736;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 9124099917951443736;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        9124099917951443736 => {
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
            let fresh1 = blks;
            blks = blks.offset(1);
            mln_blk_chain_del(&mut (*am).free_head, &mut (*am).free_tail, *fresh1);
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
#[inline]
unsafe extern "C" fn mln_lex_getchar(mut lex: *mut mln_lex_t) -> libc::c_char {
    let mut current_block: u64;
    let mut n: libc::c_int = 0;
    let mut in_0: *mut mln_lex_input_t = 0 as *mut mln_lex_input_t;
    loop {
        if ((*lex).cur).is_null()
            && {
                (*lex).cur = mln_stack_pop((*lex).stack) as *mut mln_lex_input_t;
                ((*lex).cur).is_null()
            }
        {
            return -(1 as libc::c_int) as libc::c_char;
        }
        in_0 = (*lex).cur;
        if (*in_0).type_0 == 0 as libc::c_int as libc::c_uint {
            if ((*in_0).buf).is_null() {
                (*in_0).buf = (*(*in_0).data).data;
                (*in_0).pos = (*in_0).buf;
            }
            if !((*in_0).pos >= ((*in_0).buf).offset((*in_0).buf_len as isize)) {
                break;
            }
            (*lex).line = (*in_0).line;
            mln_lex_input_free(in_0 as *mut libc::c_void);
            (*lex).cur = 0 as *mut mln_lex_input_t;
        } else {
            if ((*in_0).buf).is_null() {
                (*in_0)
                    .buf = mln_alloc_m((*lex).pool, 4095 as libc::c_int as mln_size_t)
                    as mln_u8ptr_t;
                if ((*in_0).buf).is_null() {
                    (*lex).error = 1 as libc::c_int;
                    return -(2 as libc::c_int) as libc::c_char;
                }
                (*in_0).pos = ((*in_0).buf).offset((*in_0).buf_len as isize);
            }
            if !((*in_0).pos >= ((*in_0).buf).offset((*in_0).buf_len as isize)) {
                break;
            }
            loop {
                n = read(
                    (*in_0).fd,
                    (*in_0).buf as *mut libc::c_void,
                    4095 as libc::c_int as size_t,
                ) as libc::c_int;
                if n < 0 as libc::c_int {
                    if *__errno_location() == 4 as libc::c_int {
                        continue;
                    }
                    (*lex).error = 7 as libc::c_int;
                    return -(2 as libc::c_int) as libc::c_char;
                } else if n == 0 as libc::c_int {
                    current_block = 2838571290723028321;
                    break;
                } else {
                    current_block = 5634871135123216486;
                    break;
                }
            }
            match current_block {
                5634871135123216486 => {
                    (*in_0).pos = (*in_0).buf;
                    (*in_0).buf_len = n as mln_u64_t;
                    break;
                }
                _ => {
                    (*lex).line = (*in_0).line;
                    mln_lex_input_free(in_0 as *mut libc::c_void);
                    (*lex).cur = 0 as *mut mln_lex_input_t;
                }
            }
        }
    }
    let fresh2 = (*in_0).pos;
    (*in_0).pos = ((*in_0).pos).offset(1);
    return *fresh2 as libc::c_char;
}
#[inline]
unsafe extern "C" fn mln_lex_is_letter(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int == '_' as i32
        || (c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
            || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_lex_is_oct(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && (c as libc::c_int) < '8' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_lex_is_hex(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
        || c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32
        || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut g_conf_hook_head: *mut mln_conf_hook_t = 0 as *const mln_conf_hook_t
    as *mut mln_conf_hook_t;
static mut g_conf_hook_tail: *mut mln_conf_hook_t = 0 as *const mln_conf_hook_t
    as *mut mln_conf_hook_t;
static mut g_conf: *mut mln_conf_t = 0 as *const mln_conf_t as *mut mln_conf_t;
static mut default_domain: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"main\0")
};
static mut conf_keywords: [mln_string_t; 3] = [mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
}; 3];
static mut mln_conf_env: mln_string_t = mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
};
#[no_mangle]
pub unsafe extern "C" fn mln_conf_lex_lex_dup(
    mut pool: *mut mln_alloc_t,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut src: *mut mln_conf_lex_struct_t = ptr as *mut mln_conf_lex_struct_t;
    let mut dest: *mut mln_conf_lex_struct_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_conf_lex_struct_t>() as libc::c_ulong,
    ) as *mut mln_conf_lex_struct_t;
    if dest.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*dest).text = mln_string_pool_dup(pool, (*src).text);
    if ((*dest).text).is_null() {
        mln_alloc_free(dest as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    (*dest).line = (*src).line;
    (*dest).type_0 = (*src).type_0;
    if ((*src).file).is_null() {
        (*dest).file = 0 as *mut mln_string_t;
    } else {
        (*dest)
            .file = ({
            let mut __s: *mut mln_string_t = (*src).file;
            (*__s).set_ref_0((*__s).ref_0() + 1);
            (*__s).ref_0();
            __s
        });
    }
    return dest as *mut libc::c_void;
}
#[no_mangle]
pub static mut mln_conf_lex_token_type_array: [mln_conf_lex_type_t; 45] = [
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_EOF,
            type_str: b"CONF_TK_EOF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_OCT,
            type_str: b"CONF_TK_OCT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_DEC,
            type_str: b"CONF_TK_DEC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_HEX,
            type_str: b"CONF_TK_HEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_REAL,
            type_str: b"CONF_TK_REAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_ID,
            type_str: b"CONF_TK_ID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_SPACE,
            type_str: b"CONF_TK_SPACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_EXCL,
            type_str: b"CONF_TK_EXCL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_DBLQ,
            type_str: b"CONF_TK_DBLQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_NUMS,
            type_str: b"CONF_TK_NUMS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_DOLL,
            type_str: b"CONF_TK_DOLL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_PERC,
            type_str: b"CONF_TK_PERC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_AMP,
            type_str: b"CONF_TK_AMP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_SGLQ,
            type_str: b"CONF_TK_SGLQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_LPAR,
            type_str: b"CONF_TK_LPAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_RPAR,
            type_str: b"CONF_TK_RPAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_AST,
            type_str: b"CONF_TK_AST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_PLUS,
            type_str: b"CONF_TK_PLUS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_COMMA,
            type_str: b"CONF_TK_COMMA\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_SUB,
            type_str: b"CONF_TK_SUB\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_PERIOD,
            type_str: b"CONF_TK_PERIOD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_SLASH,
            type_str: b"CONF_TK_SLASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_COLON,
            type_str: b"CONF_TK_COLON\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_SEMIC,
            type_str: b"CONF_TK_SEMIC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_LAGL,
            type_str: b"CONF_TK_LAGL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_EQUAL,
            type_str: b"CONF_TK_EQUAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_RAGL,
            type_str: b"CONF_TK_RAGL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_QUES,
            type_str: b"CONF_TK_QUES\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_AT,
            type_str: b"CONF_TK_AT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_LSQUAR,
            type_str: b"CONF_TK_LSQUAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_BSLASH,
            type_str: b"CONF_TK_BSLASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_RSQUAR,
            type_str: b"CONF_TK_RSQUAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_XOR,
            type_str: b"CONF_TK_XOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_UNDER,
            type_str: b"CONF_TK_UNDER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_FULSTP,
            type_str: b"CONF_TK_FULSTP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_LBRACE,
            type_str: b"CONF_TK_LBRACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_VERTL,
            type_str: b"CONF_TK_VERTL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_RBRACE,
            type_str: b"CONF_TK_RBRACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_DASH,
            type_str: b"CONF_TK_DASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_KEYWORD_BEGIN,
            type_str: b"CONF_TK_KEYWORD_BEGIN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_ON,
            type_str: b"CONF_TK_ON\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_OFF,
            type_str: b"CONF_TK_OFF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_COMMENT,
            type_str: b"CONF_TK_COMMENT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_CHAR,
            type_str: b"CONF_TK_CHAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_conf_lex_type_t {
            type_0: CONF_TK_STRING,
            type_str: b"CONF_TK_STRING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn mln_conf_lex_process_spec_char(
    mut lex: *mut mln_lex_t,
    mut c: libc::c_char,
) -> *mut mln_conf_lex_struct_t {
    let mut i: mln_s32_t = 0;
    let mut end: mln_s32_t = (::core::mem::size_of::<[mln_spechar_t; 32]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<mln_spechar_t>() as libc::c_ulong)
        as mln_s32_t;
    i = 0 as libc::c_int;
    while i < end {
        if c as libc::c_int == mln_conf_lex_handlers[i as usize].sc as libc::c_int {
            return (mln_conf_lex_handlers[i as usize].handler)
                .expect(
                    "non-null function pointer",
                )(lex, mln_conf_lex_handlers[i as usize].data)
                as *mut mln_conf_lex_struct_t;
        }
        i += 1;
        i;
    }
    (*lex).error = 2 as libc::c_int;
    return 0 as *mut mln_conf_lex_struct_t;
}
static mut mln_conf_lex_handlers: [mln_spechar_t; 32] = unsafe {
    [
        {
            let mut init = mln_spechar_t {
                sc: '!' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_excl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '"' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_dblq_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '#' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_nums_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '$' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_doll_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '%' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_perc_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '&' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_amp_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '\'' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_sglq_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '(' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lpar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: ')' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_rpar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '*' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_ast_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '+' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_plus_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: ',' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_comma_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '-' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_sub_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '.' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_period_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '/' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_slash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: ':' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_colon_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: ';' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_semic_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '<' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lagl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '=' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_equal_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '>' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_ragl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '?' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_ques_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '@' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_at_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '[' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lsquar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '\\' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_bslash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: ']' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_rsquar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '^' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_xor_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '_' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_under_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '`' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_fulstp_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '{' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lbrace_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '|' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_vertl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '}' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_rbrace_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = mln_spechar_t {
                sc: '~' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_dash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C" fn mln_conf_lex_process_keywords(
    mut lex: *mut mln_lex_t,
) -> *mut mln_conf_lex_struct_t {
    if ((*lex).keywords).is_null() {
        return mln_conf_lex_new(lex, CONF_TK_ID);
    }
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut diff: mln_u32_t = ((*lex).result_pos).offset_from((*lex).result_buf)
        as libc::c_long as mln_u32_t;
    let mut p: mln_u8ptr_t = (*lex).result_buf;
    let mut lk: mln_lex_keyword_t = mln_lex_keyword_t {
        keyword: 0 as *mut mln_string_t,
        val: 0,
    };
    let mut plk: *mut mln_lex_keyword_t = 0 as *mut mln_lex_keyword_t;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    ({
        tmp.data = p;
        tmp.len = diff as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    lk.keyword = &mut tmp;
    rn = mln_rbtree_search(
        (*lex).keywords,
        &mut lk as *mut mln_lex_keyword_t as *mut libc::c_void,
    );
    if !(rn == &mut (*(*lex).keywords).nil as *mut mln_rbtree_node_t) {
        plk = (*rn).data as *mut mln_lex_keyword_t;
        return mln_conf_lex_new(
            lex,
            (CONF_TK_KEYWORD_BEGIN as libc::c_int as libc::c_ulong)
                .wrapping_add((*plk).val)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as mln_conf_lex_enum,
        );
    }
    return mln_conf_lex_new(lex, CONF_TK_ID);
}
static mut mln_conf_lex_preprocess_handlers: [mln_preprocess_handler_t; 6] = [mln_preprocess_handler_t {
    command: mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    },
    handler: None,
}; 6];
unsafe extern "C" fn mln_conf_lex_new(
    mut lex: *mut mln_lex_t,
    mut type_0: mln_conf_lex_enum,
) -> *mut mln_conf_lex_struct_t {
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut ptr: *mut mln_conf_lex_struct_t = 0 as *mut mln_conf_lex_struct_t;
    ptr = mln_alloc_m(
        (*lex).pool,
        ::core::mem::size_of::<mln_conf_lex_struct_t>() as libc::c_ulong,
    ) as *mut mln_conf_lex_struct_t;
    if ptr.is_null() {
        (*lex).error = 1 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if type_0 as libc::c_uint != CONF_TK_EOF as libc::c_int as libc::c_uint {
        ({
            tmp.data = (*lex).result_buf;
            tmp
                .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
                as mln_u64_t;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
    } else {
        ({
            tmp.data = b"##EOF##\0" as *const u8 as *const libc::c_char as mln_u8ptr_t;
            tmp
                .len = (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
    }
    (*ptr).text = mln_string_pool_dup((*lex).pool, &mut tmp);
    if ((*ptr).text).is_null() {
        mln_alloc_free(ptr as *mut libc::c_void);
        (*lex).error = 1 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    (*ptr).line = (*lex).line as mln_u32_t;
    (*ptr).type_0 = type_0;
    (*ptr).file = 0 as *mut mln_string_t;
    if !((*lex).cur).is_null()
        && (*(*lex).cur).type_0 == 1 as libc::c_int as libc::c_uint
    {
        (*ptr)
            .file = ({
            let mut __s: *mut mln_string_t = (*(*lex).cur).data;
            (*__s).set_ref_0((*__s).ref_0() + 1);
            (*__s).ref_0();
            __s
        });
    }
    (*lex).error = 0 as libc::c_int;
    (*lex).result_pos = (*lex).result_buf;
    return ptr;
}
unsafe extern "C" fn mln_conf_lex_rbrace_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_RBRACE);
}
unsafe extern "C" fn mln_conf_lex_excl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_EXCL);
}
unsafe extern "C" fn mln_conf_lex_amp_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_AMP);
}
unsafe extern "C" fn mln_conf_lex_perc_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_PERC);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_undef(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut c: libc::c_char = 0;
    let mut str: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut tmp: mln_lex_macro_t = mln_lex_macro_t {
        key: 0 as *mut mln_string_t,
        val: 0 as *mut mln_string_t,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if mln_lex_is_letter(c) == 0
            && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
        {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        mln_lex_stepback(lex, c);
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    ({
        str.data = (*lex).result_buf;
        str
            .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
            as mln_u64_t;
        str.set_data_ref(1 as libc::c_int as mln_uauto_t);
        str.set_pool(0 as libc::c_int as mln_uauto_t);
        str.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut str;
        &mut str
    });
    tmp.key = &mut str;
    tmp.val = 0 as *mut mln_string_t;
    rn = mln_rbtree_search(
        (*lex).macros,
        &mut tmp as *mut mln_lex_macro_t as *mut libc::c_void,
    );
    if !(rn == &mut (*(*lex).macros).nil as *mut mln_rbtree_node_t) {
        mln_rbtree_delete((*lex).macros, rn);
        mln_rbtree_node_free((*lex).macros, rn);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_endif(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    if (*lpd).if_level == 0 as libc::c_int as libc::c_ulong {
        (*lex).error = 15 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    let fresh3 = (*lpd).if_level;
    (*lpd).if_level = ((*lpd).if_level).wrapping_sub(1);
    if (*lpd).if_matched == fresh3 {
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_sub(1);
        (*lpd).if_matched;
    }
    (*lex)
        .set_ignore(!((*lpd).if_matched == (*lpd).if_level) as libc::c_int as mln_u32_t);
    (*lex).result_pos = (*lex).result_buf;
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_else(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    if (*lpd).if_level == 0 as libc::c_int as libc::c_ulong {
        (*lex).error = 15 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    (*lex).result_pos = (*lex).result_buf;
    if (*lpd).if_level <= (*lpd).if_matched {
        (*lex).set_ignore(1 as libc::c_int as mln_u32_t);
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_sub(1);
        (*lpd).if_matched;
    } else if ((*lpd).if_matched).wrapping_add(1 as libc::c_int as libc::c_ulong)
        == (*lpd).if_level
    {
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_add(1);
        (*lpd).if_matched;
        (*lex).set_ignore(0 as libc::c_int as mln_u32_t);
    }
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_if(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut ret: libc::c_int = 0;
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    (*lex).result_pos = (*lex).result_buf;
    (*lpd).if_level = ((*lpd).if_level).wrapping_add(1);
    (*lpd).if_level;
    if (*lex).ignore() != 0 {
        return mln_conf_lex_token(lex);
    }
    ret = mln_lex_condition_test(lex);
    if ret < 0 as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if ret != 0 {
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_add(1);
        (*lpd).if_matched;
        (*lex).set_ignore(0 as libc::c_int as mln_u32_t);
    } else {
        (*lex).set_ignore(1 as libc::c_int as mln_u32_t);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_include(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut c: libc::c_char = 0;
    let mut path: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int != '"' as i32 {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == '"' as i32 {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    ({
        path.data = (*lex).result_buf;
        path
            .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
            as mln_u64_t;
        path.set_data_ref(1 as libc::c_int as mln_uauto_t);
        path.set_pool(0 as libc::c_int as mln_uauto_t);
        path.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut path;
        &mut path
    });
    if path.len == 0 as libc::c_int as libc::c_ulong {
        (*lex).error = 11 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if mln_lex_check_file_loop(lex, &mut path) < 0 as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if mln_lex_push_input_file_stream(lex, &mut path) < 0 as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_lex_preprocess_define(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut c: libc::c_char = 0;
    let mut str: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut k: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut v: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut lm: *mut mln_lex_macro_t = 0 as *mut mln_lex_macro_t;
    let mut tmp: mln_lex_macro_t = mln_lex_macro_t {
        key: 0 as *mut mln_string_t,
        val: 0 as *mut mln_string_t,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if mln_lex_is_letter(c) == 0
            && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
        {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        mln_lex_stepback(lex, c);
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    ({
        str.data = (*lex).result_buf;
        str
            .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
            as mln_u64_t;
        str.set_data_ref(1 as libc::c_int as mln_uauto_t);
        str.set_pool(0 as libc::c_int as mln_uauto_t);
        str.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut str;
        &mut str
    });
    if (*lex).ignore() == 0 {
        let mut current_block_60: u64;
        k = mln_string_pool_dup((*lex).pool, &mut str);
        if k.is_null() {
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        tmp.key = k;
        tmp.val = 0 as *mut mln_string_t;
        rn = mln_rbtree_search(
            (*lex).macros,
            &mut tmp as *mut mln_lex_macro_t as *mut libc::c_void,
        );
        if !(rn == &mut (*(*lex).macros).nil as *mut mln_rbtree_node_t) {
            let mut __s: *mut mln_string_t = k;
            if !__s.is_null() {
                let ref mut fresh4 = (*__s).ref_0();
                let fresh5 = *fresh4;
                *fresh4 = (*fresh4).wrapping_sub(1);
                if fresh5 <= 1 as libc::c_int as libc::c_ulong {
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
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int != '\n' as i32 {
            (*lex).result_pos = (*lex).result_buf;
            loop {
                c = mln_lex_getchar(lex);
                if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return 0 as *mut mln_conf_lex_struct_t;
                }
                if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
                    continue;
                }
                if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int
                    || c as libc::c_int == '\n' as i32
                {
                    current_block_60 = 8256453181684711694;
                    break;
                }
                mln_lex_stepback(lex, c);
                current_block_60 = 11952910966027254843;
                break;
            }
            match current_block_60 {
                8256453181684711694 => {}
                _ => {
                    loop {
                        loop {
                            c = mln_lex_getchar(lex);
                            if c as libc::c_int
                                == -(2 as libc::c_int) as libc::c_char as libc::c_int
                            {
                                return 0 as *mut mln_conf_lex_struct_t;
                            }
                            if c as libc::c_int
                                == -(1 as libc::c_int) as libc::c_char as libc::c_int
                                || c as libc::c_int == '\n' as i32
                            {
                                break;
                            }
                            if mln_lex_putchar(lex, c)
                                == -(2 as libc::c_int) as libc::c_char as libc::c_int
                            {
                                return 0 as *mut mln_conf_lex_struct_t;
                            }
                        }
                        if !((*lex).result_pos > (*lex).result_buf) {
                            break;
                        }
                        if !((*lex).result_pos).is_null()
                            && *((*lex).result_pos).offset(-(1 as libc::c_int as isize))
                                as libc::c_int == '\\' as i32 as mln_u8_t as libc::c_int
                        {
                            (*lex).result_pos = ((*lex).result_pos).offset(-1);
                            (*lex).result_pos;
                        } else {
                            ({
                                str.data = (*lex).result_buf;
                                str
                                    .len = ((*lex).result_pos).offset_from((*lex).result_buf)
                                    as libc::c_long as mln_u64_t;
                                str.set_data_ref(1 as libc::c_int as mln_uauto_t);
                                str.set_pool(0 as libc::c_int as mln_uauto_t);
                                str.set_ref_0(1 as libc::c_int as mln_uauto_t);
                                &mut str;
                                &mut str
                            });
                            v = &mut str;
                            break;
                        }
                    }
                }
            }
        }
        lm = mln_lex_macro_new((*lex).pool, k, v);
        if lm.is_null() {
            let mut __s: *mut mln_string_t = k;
            if !__s.is_null() {
                let ref mut fresh6 = (*__s).ref_0();
                let fresh7 = *fresh6;
                *fresh6 = (*fresh6).wrapping_sub(1);
                if fresh7 <= 1 as libc::c_int as libc::c_ulong {
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
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        let mut __s: *mut mln_string_t = k;
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
        rn = mln_rbtree_node_new((*lex).macros, lm as *mut libc::c_void);
        if rn.is_null() {
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        mln_rbtree_insert((*lex).macros, rn);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_conf_lex_token(lex);
}
unsafe extern "C" fn mln_conf_lex_under_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_UNDER);
}
unsafe extern "C" fn mln_conf_lex_xor_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_XOR);
}
unsafe extern "C" fn mln_conf_lex_nums_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut ph: *mut mln_preprocess_handler_t = 0 as *mut mln_preprocess_handler_t;
    let mut phend: *mut mln_preprocess_handler_t = 0 as *mut mln_preprocess_handler_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut lm: mln_lex_macro_t = mln_lex_macro_t {
        key: 0 as *mut mln_string_t,
        val: 0 as *mut mln_string_t,
    };
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if mln_lex_is_letter(c) == 0
        && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
    {
        mln_lex_stepback(lex, c);
        return mln_conf_lex_new(lex, CONF_TK_AT);
    }
    (*lex).result_pos = (*lex).result_buf;
    while mln_lex_is_letter(c) != 0
        || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
    {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    mln_lex_stepback(lex, c);
    ({
        tmp.data = (*lex).result_buf;
        tmp
            .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
            as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    phend = mln_conf_lex_preprocess_handlers
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[mln_preprocess_handler_t; 6]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<mln_preprocess_handler_t>() as libc::c_ulong,
                ) as isize,
        );
    ph = mln_conf_lex_preprocess_handlers.as_mut_ptr();
    while ph < phend {
        if mln_string_strcmp(&mut (*ph).command, &mut tmp) == 0 {
            (*lex).result_pos = (*lex).result_buf;
            return ((*ph).handler).expect("non-null function pointer")(lex, data)
                as *mut mln_conf_lex_struct_t;
        }
        ph = ph.offset(1);
        ph;
    }
    ({
        tmp.data = (*lex).result_buf;
        tmp
            .len = ((*lex).result_pos).offset_from((*lex).result_buf) as libc::c_long
            as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    lm.key = &mut tmp;
    lm.val = 0 as *mut mln_string_t;
    rn = mln_rbtree_search(
        (*lex).macros,
        &mut lm as *mut mln_lex_macro_t as *mut libc::c_void,
    );
    if !(rn == &mut (*(*lex).macros).nil as *mut mln_rbtree_node_t) {
        let mut ltmp: *mut mln_lex_macro_t = (*rn).data as *mut mln_lex_macro_t;
        if !((*ltmp).val).is_null()
            && mln_lex_push_input_buf_stream(lex, (*ltmp).val) < 0 as libc::c_int
        {
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        (*lex).result_pos = (*lex).result_buf;
        return mln_conf_lex_token(lex);
    }
    (*lex).error = 13 as libc::c_int;
    return 0 as *mut mln_conf_lex_struct_t;
}
unsafe extern "C" fn mln_conf_lex_dblq_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_DBLQ);
}
unsafe extern "C" fn mln_conf_lex_doll_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_DOLL);
}
unsafe extern "C" fn mln_conf_lex_rpar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_RPAR);
}
unsafe extern "C" fn mln_conf_lex_lpar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_LPAR);
}
unsafe extern "C" fn mln_conf_lex_set_hooks(mut lex: *mut mln_lex_t) {
    let mut hooks: *mut mln_lex_hooks_t = &mut (*lex).hooks;
    if ((*hooks).excl_handler).is_some() {
        mln_conf_lex_handlers[0 as libc::c_int as usize].handler = (*hooks).excl_handler;
        mln_conf_lex_handlers[0 as libc::c_int as usize].data = (*hooks).excl_data;
    }
    if ((*hooks).dblq_handler).is_some() {
        mln_conf_lex_handlers[1 as libc::c_int as usize].handler = (*hooks).dblq_handler;
        mln_conf_lex_handlers[1 as libc::c_int as usize].data = (*hooks).dblq_data;
    }
    if ((*hooks).nums_handler).is_some() {
        mln_conf_lex_handlers[2 as libc::c_int as usize].handler = (*hooks).nums_handler;
        mln_conf_lex_handlers[2 as libc::c_int as usize].data = (*hooks).nums_data;
    }
    if ((*hooks).doll_handler).is_some() {
        mln_conf_lex_handlers[3 as libc::c_int as usize].handler = (*hooks).doll_handler;
        mln_conf_lex_handlers[3 as libc::c_int as usize].data = (*hooks).doll_data;
    }
    if ((*hooks).perc_handler).is_some() {
        mln_conf_lex_handlers[4 as libc::c_int as usize].handler = (*hooks).perc_handler;
        mln_conf_lex_handlers[4 as libc::c_int as usize].data = (*hooks).perc_data;
    }
    if ((*hooks).amp_handler).is_some() {
        mln_conf_lex_handlers[5 as libc::c_int as usize].handler = (*hooks).amp_handler;
        mln_conf_lex_handlers[5 as libc::c_int as usize].data = (*hooks).amp_data;
    }
    if ((*hooks).sglq_handler).is_some() {
        mln_conf_lex_handlers[6 as libc::c_int as usize].handler = (*hooks).sglq_handler;
        mln_conf_lex_handlers[6 as libc::c_int as usize].data = (*hooks).slgq_data;
    }
    if ((*hooks).lpar_handler).is_some() {
        mln_conf_lex_handlers[7 as libc::c_int as usize].handler = (*hooks).lpar_handler;
        mln_conf_lex_handlers[7 as libc::c_int as usize].data = (*hooks).lpar_data;
    }
    if ((*hooks).rpar_handler).is_some() {
        mln_conf_lex_handlers[8 as libc::c_int as usize].handler = (*hooks).rpar_handler;
        mln_conf_lex_handlers[8 as libc::c_int as usize].data = (*hooks).rpar_data;
    }
    if ((*hooks).ast_handler).is_some() {
        mln_conf_lex_handlers[9 as libc::c_int as usize].handler = (*hooks).ast_handler;
        mln_conf_lex_handlers[9 as libc::c_int as usize].data = (*hooks).ast_data;
    }
    if ((*hooks).plus_handler).is_some() {
        mln_conf_lex_handlers[10 as libc::c_int as usize]
            .handler = (*hooks).plus_handler;
        mln_conf_lex_handlers[10 as libc::c_int as usize].data = (*hooks).plus_data;
    }
    if ((*hooks).comma_handler).is_some() {
        mln_conf_lex_handlers[11 as libc::c_int as usize]
            .handler = (*hooks).comma_handler;
        mln_conf_lex_handlers[11 as libc::c_int as usize].data = (*hooks).comma_data;
    }
    if ((*hooks).sub_handler).is_some() {
        mln_conf_lex_handlers[12 as libc::c_int as usize].handler = (*hooks).sub_handler;
        mln_conf_lex_handlers[12 as libc::c_int as usize].data = (*hooks).sub_data;
    }
    if ((*hooks).period_handler).is_some() {
        mln_conf_lex_handlers[13 as libc::c_int as usize]
            .handler = (*hooks).period_handler;
        mln_conf_lex_handlers[13 as libc::c_int as usize].data = (*hooks).period_data;
    }
    if ((*hooks).slash_handler).is_some() {
        mln_conf_lex_handlers[14 as libc::c_int as usize]
            .handler = (*hooks).slash_handler;
        mln_conf_lex_handlers[14 as libc::c_int as usize].data = (*hooks).slash_data;
    }
    if ((*hooks).colon_handler).is_some() {
        mln_conf_lex_handlers[15 as libc::c_int as usize]
            .handler = (*hooks).colon_handler;
        mln_conf_lex_handlers[15 as libc::c_int as usize].data = (*hooks).colon_data;
    }
    if ((*hooks).semic_handler).is_some() {
        mln_conf_lex_handlers[16 as libc::c_int as usize]
            .handler = (*hooks).semic_handler;
        mln_conf_lex_handlers[16 as libc::c_int as usize].data = (*hooks).semic_data;
    }
    if ((*hooks).lagl_handler).is_some() {
        mln_conf_lex_handlers[17 as libc::c_int as usize]
            .handler = (*hooks).lagl_handler;
        mln_conf_lex_handlers[17 as libc::c_int as usize].data = (*hooks).lagl_data;
    }
    if ((*hooks).equal_handler).is_some() {
        mln_conf_lex_handlers[18 as libc::c_int as usize]
            .handler = (*hooks).equal_handler;
        mln_conf_lex_handlers[18 as libc::c_int as usize].data = (*hooks).equal_data;
    }
    if ((*hooks).ragl_handler).is_some() {
        mln_conf_lex_handlers[19 as libc::c_int as usize]
            .handler = (*hooks).ragl_handler;
        mln_conf_lex_handlers[19 as libc::c_int as usize].data = (*hooks).ragl_data;
    }
    if ((*hooks).ques_handler).is_some() {
        mln_conf_lex_handlers[20 as libc::c_int as usize]
            .handler = (*hooks).ques_handler;
        mln_conf_lex_handlers[20 as libc::c_int as usize].data = (*hooks).ques_data;
    }
    if ((*hooks).at_handler).is_some() {
        mln_conf_lex_handlers[21 as libc::c_int as usize].handler = (*hooks).at_handler;
        mln_conf_lex_handlers[21 as libc::c_int as usize].data = (*hooks).at_data;
    }
    if ((*hooks).lsquar_handler).is_some() {
        mln_conf_lex_handlers[22 as libc::c_int as usize]
            .handler = (*hooks).lsquar_handler;
        mln_conf_lex_handlers[22 as libc::c_int as usize].data = (*hooks).lsquar_data;
    }
    if ((*hooks).bslash_handler).is_some() {
        mln_conf_lex_handlers[23 as libc::c_int as usize]
            .handler = (*hooks).bslash_handler;
        mln_conf_lex_handlers[23 as libc::c_int as usize].data = (*hooks).bslash_data;
    }
    if ((*hooks).rsquar_handler).is_some() {
        mln_conf_lex_handlers[24 as libc::c_int as usize]
            .handler = (*hooks).rsquar_handler;
        mln_conf_lex_handlers[24 as libc::c_int as usize].data = (*hooks).rsquar_data;
    }
    if ((*hooks).xor_handler).is_some() {
        mln_conf_lex_handlers[25 as libc::c_int as usize].handler = (*hooks).xor_handler;
        mln_conf_lex_handlers[25 as libc::c_int as usize].data = (*hooks).xor_data;
    }
    if ((*hooks).under_handler).is_some() {
        mln_conf_lex_handlers[26 as libc::c_int as usize]
            .handler = (*hooks).under_handler;
        mln_conf_lex_handlers[26 as libc::c_int as usize].data = (*hooks).under_data;
    }
    if ((*hooks).fulstp_handler).is_some() {
        mln_conf_lex_handlers[27 as libc::c_int as usize]
            .handler = (*hooks).fulstp_handler;
        mln_conf_lex_handlers[27 as libc::c_int as usize].data = (*hooks).fulstp_data;
    }
    if ((*hooks).lbrace_handler).is_some() {
        mln_conf_lex_handlers[28 as libc::c_int as usize]
            .handler = (*hooks).lbrace_handler;
        mln_conf_lex_handlers[28 as libc::c_int as usize].data = (*hooks).lbrace_data;
    }
    if ((*hooks).vertl_handler).is_some() {
        mln_conf_lex_handlers[29 as libc::c_int as usize]
            .handler = (*hooks).vertl_handler;
        mln_conf_lex_handlers[29 as libc::c_int as usize].data = (*hooks).vertl_data;
    }
    if ((*hooks).rbrace_handler).is_some() {
        mln_conf_lex_handlers[30 as libc::c_int as usize]
            .handler = (*hooks).rbrace_handler;
        mln_conf_lex_handlers[30 as libc::c_int as usize].data = (*hooks).rbrace_data;
    }
    if ((*hooks).dash_handler).is_some() {
        mln_conf_lex_handlers[31 as libc::c_int as usize]
            .handler = (*hooks).dash_handler;
        mln_conf_lex_handlers[31 as libc::c_int as usize].data = (*hooks).dash_data;
    }
}
unsafe extern "C" fn mln_conf_lex_free(mut ptr: *mut mln_conf_lex_struct_t) {
    if ptr.is_null() {
        return;
    }
    if !((*ptr).text).is_null() {
        let mut __s: *mut mln_string_t = (*ptr).text;
        if !__s.is_null() {
            let ref mut fresh10 = (*__s).ref_0();
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).wrapping_sub(1);
            if fresh11 <= 1 as libc::c_int as libc::c_ulong {
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
    if !((*ptr).file).is_null() {
        let mut __s: *mut mln_string_t = (*ptr).file;
        if !__s.is_null() {
            let ref mut fresh12 = (*__s).ref_0();
            let fresh13 = *fresh12;
            *fresh12 = (*fresh12).wrapping_sub(1);
            if fresh13 <= 1 as libc::c_int as libc::c_ulong {
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
    mln_alloc_free(ptr as *mut libc::c_void);
}
unsafe extern "C" fn mln_conf_lex_ast_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_AST);
}
unsafe extern "C" fn mln_conf_lex_token(
    mut lex: *mut mln_lex_t,
) -> *mut mln_conf_lex_struct_t {
    let mut dot_cnt: mln_s32_t = 0;
    let mut chk: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut sret: *mut mln_conf_lex_struct_t = 0 as *mut mln_conf_lex_struct_t;
    '_beg: loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        loop {
            match c as libc::c_int {
                -1 => return mln_conf_lex_new(lex, CONF_TK_EOF),
                10 => {
                    while c as libc::c_int == '\n' as i32 {
                        (*lex).line = ((*lex).line).wrapping_add(1);
                        (*lex).line;
                        c = mln_lex_getchar(lex);
                        if c as libc::c_int
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_conf_lex_struct_t;
                        }
                    }
                }
                9 | 32 => {
                    while c as libc::c_int == ' ' as i32
                        || c as libc::c_int == '\t' as i32
                    {
                        c = mln_lex_getchar(lex);
                        if c as libc::c_int
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_conf_lex_struct_t;
                        }
                    }
                }
                _ => {
                    if mln_lex_is_letter(c) != 0 {
                        current_block = 8831408221741692167;
                        break;
                    } else {
                        current_block = 1054647088692577877;
                        break;
                    }
                }
            }
        }
        match current_block {
            8831408221741692167 => {
                while mln_lex_is_letter(c) != 0
                    || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                {
                    if mln_lex_putchar(lex, c)
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_conf_lex_struct_t;
                    }
                    c = mln_lex_getchar(lex);
                    if c as libc::c_int
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_conf_lex_struct_t;
                    }
                }
                mln_lex_stepback(lex, c);
                if *((*lex).result_buf).offset(0 as libc::c_int as isize) as libc::c_int
                    == '_' as i32 as mln_u8_t as libc::c_int
                    && (*lex).result_pos
                        == ((*lex).result_buf).offset(1 as libc::c_int as isize)
                {
                    sret = mln_conf_lex_process_spec_char(
                        lex,
                        '_' as i32 as libc::c_char,
                    );
                    if sret.is_null() || (*lex).ignore() == 0 {
                        return sret;
                    }
                } else {
                    sret = mln_conf_lex_process_keywords(lex);
                    if sret.is_null() || (*lex).ignore() == 0 {
                        return sret;
                    }
                }
            }
            _ => {
                if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                    loop {
                        if mln_lex_putchar(lex, c)
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_conf_lex_struct_t;
                        }
                        c = mln_lex_getchar(lex);
                        if c as libc::c_int
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_conf_lex_struct_t;
                        }
                        if !(!(c as libc::c_int >= '0' as i32
                            && c as libc::c_int <= '9' as i32)
                            && !(c as libc::c_int >= 'a' as i32
                                && c as libc::c_int <= 'z' as i32
                                || c as libc::c_int >= 'A' as i32
                                    && c as libc::c_int <= 'Z' as i32)
                            && c as libc::c_int != '.' as i32)
                        {
                            continue;
                        }
                        mln_lex_stepback(lex, c);
                        break;
                    }
                    chk = (*lex).result_buf;
                    if *chk as libc::c_int == '0' as i32 as mln_u8_t as libc::c_int
                        && ((*lex).result_pos).offset_from((*lex).result_buf)
                            as libc::c_long > 1 as libc::c_int as libc::c_long
                    {
                        if *chk.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'x' as i32 as mln_u8_t as libc::c_int
                        {
                            if ((*lex).result_pos).offset_from((*lex).result_buf)
                                as libc::c_long == 2 as libc::c_int as libc::c_long
                            {
                                (*lex).error = 3 as libc::c_int;
                                return 0 as *mut mln_conf_lex_struct_t;
                            }
                            chk = chk.offset(2 as libc::c_int as isize);
                            while chk < (*lex).result_pos {
                                if mln_lex_is_hex(*chk as libc::c_char) == 0 {
                                    (*lex).error = 3 as libc::c_int;
                                    return 0 as *mut mln_conf_lex_struct_t;
                                }
                                chk = chk.offset(1);
                                chk;
                            }
                            sret = mln_conf_lex_new(lex, CONF_TK_HEX);
                            if sret.is_null() || (*lex).ignore() == 0 {
                                return sret;
                            }
                        } else {
                            chk = chk.offset(1);
                            chk;
                            loop {
                                if chk < (*lex).result_pos {
                                    if mln_lex_is_oct(*chk as libc::c_char) == 0 {
                                        dot_cnt = 0 as libc::c_int;
                                        while chk < (*lex).result_pos {
                                            if *chk as libc::c_int
                                                == '.' as i32 as mln_u8_t as libc::c_int
                                            {
                                                dot_cnt += 1;
                                                dot_cnt;
                                            }
                                            if !(*chk as libc::c_char as libc::c_int >= '0' as i32
                                                && *chk as libc::c_char as libc::c_int <= '9' as i32)
                                                && *chk as libc::c_int
                                                    != '.' as i32 as mln_u8_t as libc::c_int
                                            {
                                                (*lex).error = 5 as libc::c_int;
                                                return 0 as *mut mln_conf_lex_struct_t;
                                            }
                                            chk = chk.offset(1);
                                            chk;
                                        }
                                        if dot_cnt > 1 as libc::c_int {
                                            current_block = 9859671972921157070;
                                            break;
                                        } else {
                                            current_block = 6721012065216013753;
                                            break;
                                        }
                                    } else {
                                        chk = chk.offset(1);
                                        chk;
                                    }
                                } else {
                                    sret = mln_conf_lex_new(lex, CONF_TK_OCT);
                                    if sret.is_null() || (*lex).ignore() == 0 {
                                        return sret;
                                    }
                                    continue '_beg;
                                }
                            }
                            match current_block {
                                9859671972921157070 => {
                                    (*lex).error = 6 as libc::c_int;
                                    break;
                                }
                                _ => {
                                    if dot_cnt == 0 as libc::c_int {
                                        (*lex).error = 5 as libc::c_int;
                                        break;
                                    } else {
                                        sret = mln_conf_lex_new(lex, CONF_TK_REAL);
                                        if sret.is_null() || (*lex).ignore() == 0 {
                                            return sret;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        while chk < (*lex).result_pos {
                            if *chk as libc::c_char as libc::c_int >= '0' as i32
                                && *chk as libc::c_char as libc::c_int <= '9' as i32
                            {
                                chk = chk.offset(1);
                                chk;
                            } else if *chk as libc::c_int
                                == '.' as i32 as mln_u8_t as libc::c_int
                            {
                                chk = chk.offset(1);
                                chk;
                                while chk < (*lex).result_pos {
                                    if !(*chk as libc::c_char as libc::c_int >= '0' as i32
                                        && *chk as libc::c_char as libc::c_int <= '9' as i32)
                                    {
                                        (*lex).error = 6 as libc::c_int;
                                        return 0 as *mut mln_conf_lex_struct_t;
                                    }
                                    chk = chk.offset(1);
                                    chk;
                                }
                                sret = mln_conf_lex_new(lex, CONF_TK_REAL);
                                if sret.is_null() || (*lex).ignore() == 0 {
                                    return sret;
                                }
                                continue '_beg;
                            } else {
                                (*lex).error = 4 as libc::c_int;
                                return 0 as *mut mln_conf_lex_struct_t;
                            }
                        }
                        sret = mln_conf_lex_new(lex, CONF_TK_DEC);
                        if sret.is_null() || (*lex).ignore() == 0 {
                            return sret;
                        }
                    }
                } else {
                    if mln_lex_putchar(lex, c)
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_conf_lex_struct_t;
                    }
                    sret = mln_conf_lex_process_spec_char(lex, c);
                    if sret.is_null() {
                        return 0 as *mut mln_conf_lex_struct_t
                    } else if (*lex).ignore() == 0 {
                        return sret
                    }
                }
            }
        }
    }
    return 0 as *mut mln_conf_lex_struct_t;
}
unsafe extern "C" fn mln_conf_lex_nums_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_NUMS);
}
unsafe extern "C" fn mln_conf_lex_fulstp_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_FULSTP);
}
unsafe extern "C" fn mln_conf_lex_dash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_DASH);
}
unsafe extern "C" fn mln_conf_lex_lbrace_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_LBRACE);
}
unsafe extern "C" fn mln_conf_lex_sglq_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_SGLQ);
}
unsafe extern "C" fn mln_conf_lex_rsquar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_RSQUAR);
}
unsafe extern "C" fn mln_conf_lex_bslash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_BSLASH);
}
unsafe extern "C" fn mln_conf_lex_lsquar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_LSQUAR);
}
unsafe extern "C" fn mln_conf_lex_vertl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_VERTL);
}
unsafe extern "C" fn mln_conf_lex_at_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_AT);
}
unsafe extern "C" fn mln_conf_lex_ques_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_QUES);
}
unsafe extern "C" fn mln_conf_lex_ragl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_RAGL);
}
unsafe extern "C" fn mln_conf_lex_equal_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_EQUAL);
}
unsafe extern "C" fn mln_conf_lex_lagl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_LAGL);
}
unsafe extern "C" fn mln_conf_lex_semic_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_SEMIC);
}
unsafe extern "C" fn mln_conf_lex_colon_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_COLON);
}
unsafe extern "C" fn mln_conf_lex_slash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_SLASH);
}
unsafe extern "C" fn mln_conf_lex_period_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_PERIOD);
}
unsafe extern "C" fn mln_conf_lex_sub_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_SUB);
}
unsafe extern "C" fn mln_conf_lex_comma_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_COMMA);
}
unsafe extern "C" fn mln_conf_lex_plus_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    return mln_conf_lex_new(lex, CONF_TK_PLUS);
}
unsafe extern "C" fn mln_conf_lex_sglq_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    (*lex).result_pos = (*lex).result_buf;
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if !(c as libc::c_int >= 0 as libc::c_int && c as libc::c_int <= 127 as libc::c_int)
        || c as libc::c_int == '\'' as i32
    {
        (*lex).error = 2 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if mln_get_char(lex, c) < 0 as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    c = mln_lex_getchar(lex);
    if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if c as libc::c_int != '\'' as i32 {
        (*lex).error = 2 as libc::c_int;
        return 0 as *mut mln_conf_lex_struct_t;
    }
    return mln_conf_lex_new(lex, CONF_TK_CHAR);
}
#[inline]
unsafe extern "C" fn mln_get_char(
    mut lex: *mut mln_lex_t,
    mut c: libc::c_char,
) -> libc::c_int {
    if c as libc::c_int == '\\' as i32 {
        let mut n: libc::c_char = 0;
        n = mln_lex_getchar(lex);
        if n as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return -(1 as libc::c_int);
        }
        match n as libc::c_int {
            34 => {
                if mln_lex_putchar(lex, n)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            39 => {
                if mln_lex_putchar(lex, n)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            110 => {
                if mln_lex_putchar(lex, '\n' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            116 => {
                if mln_lex_putchar(lex, '\t' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            98 => {
                if mln_lex_putchar(lex, '\u{8}' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            97 => {
                if mln_lex_putchar(lex, '\u{7}' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            102 => {
                if mln_lex_putchar(lex, '\u{c}' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            114 => {
                if mln_lex_putchar(lex, '\r' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            101 => {
                if mln_lex_putchar(lex, '\u{1b}' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            118 => {
                if mln_lex_putchar(lex, '\u{b}' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            92 => {
                if mln_lex_putchar(lex, '\\' as i32 as libc::c_char)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            _ => {
                (*lex).error = 2 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    } else if mln_lex_putchar(lex, c)
        == -(2 as libc::c_int) as libc::c_char as libc::c_int
    {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_conf_lex_dblq_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    (*lex).result_pos = (*lex).result_buf;
    let mut c: libc::c_char = 0;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_conf_lex_struct_t;
        }
        if c as libc::c_int == '"' as i32 {
            break;
        }
        if mln_get_char(lex, c) < 0 as libc::c_int {
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    return mln_conf_lex_new(lex, CONF_TK_STRING);
}
unsafe extern "C" fn mln_conf_lex_slash_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_lex_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
        return 0 as *mut mln_conf_lex_struct_t;
    }
    if c as libc::c_int == '*' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        loop {
            c = mln_lex_getchar(lex);
            if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
                return 0 as *mut mln_conf_lex_struct_t;
            }
            if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
                mln_lex_stepback(lex, c);
                break;
            } else {
                if c as libc::c_int == '\n' as i32 {
                    (*lex).line = ((*lex).line).wrapping_add(1);
                    (*lex).line;
                }
                if c as libc::c_int == '*' as i32 {
                    if mln_lex_putchar(lex, c)
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_conf_lex_struct_t;
                    }
                    c = mln_lex_getchar(lex);
                    if c as libc::c_int
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_conf_lex_struct_t;
                    }
                    if c as libc::c_int
                        == -(1 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        mln_lex_stepback(lex, c);
                        break;
                    } else {
                        if c as libc::c_int == '\n' as i32 {
                            (*lex).line = ((*lex).line).wrapping_add(1);
                            (*lex).line;
                        }
                        if c as libc::c_int == '/' as i32 {
                            if mln_lex_putchar(lex, c)
                                == -(2 as libc::c_int) as libc::c_char as libc::c_int
                            {
                                return 0 as *mut mln_conf_lex_struct_t;
                            }
                            break;
                        }
                    }
                }
                if mln_lex_putchar(lex, c)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return 0 as *mut mln_conf_lex_struct_t;
                }
            }
        }
    } else if c as libc::c_int == '/' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_conf_lex_struct_t;
        }
        loop {
            c = mln_lex_getchar(lex);
            if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
                return 0 as *mut mln_conf_lex_struct_t;
            }
            if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
                mln_lex_stepback(lex, c);
                break;
            } else if c as libc::c_int == '\n' as i32 {
                mln_lex_stepback(lex, c);
                break;
            } else if mln_lex_putchar(lex, c)
                == -(2 as libc::c_int) as libc::c_char as libc::c_int
            {
                return 0 as *mut mln_conf_lex_struct_t
            }
        }
    } else {
        mln_lex_stepback(lex, c);
        return mln_conf_lex_new(lex, CONF_TK_SLASH);
    }
    return mln_conf_lex_new(lex, CONF_TK_COMMENT);
}
unsafe extern "C" fn mln_conf_token(
    mut lex: *mut mln_lex_t,
) -> *mut mln_conf_lex_struct_t {
    let mut clst: *mut mln_conf_lex_struct_t = 0 as *mut mln_conf_lex_struct_t;
    let mut sub_mark: libc::c_int = 0 as libc::c_int;
    loop {
        clst = mln_conf_lex_token(lex);
        if clst.is_null() {
            if sub_mark != 0 {
                (*lex).error = 2 as libc::c_int;
                return 0 as *mut mln_conf_lex_struct_t;
            }
            break;
        } else if (*clst).type_0 as libc::c_uint
            == CONF_TK_COMMENT as libc::c_int as libc::c_uint
        {
            mln_conf_lex_free(clst);
            if sub_mark != 0 {
                (*lex).error = 2 as libc::c_int;
                return 0 as *mut mln_conf_lex_struct_t;
            }
        } else if (*clst).type_0 as libc::c_uint
            == CONF_TK_SUB as libc::c_int as libc::c_uint
        {
            mln_conf_lex_free(clst);
            if sub_mark != 0 {
                (*lex).error = 2 as libc::c_int;
                return 0 as *mut mln_conf_lex_struct_t;
            }
            sub_mark = 1 as libc::c_int;
        } else if (*clst).type_0 as libc::c_uint
            == CONF_TK_DEC as libc::c_int as libc::c_uint
            || (*clst).type_0 as libc::c_uint
                == CONF_TK_REAL as libc::c_int as libc::c_uint
        {
            if sub_mark == 0 {
                break;
            }
            let mut len: mln_u64_t = ((*(*clst).text).len)
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut s: mln_u8ptr_t = mln_alloc_m((*lex).pool, len) as mln_u8ptr_t;
            if s.is_null() {
                (*lex).error = 1 as libc::c_int;
                mln_conf_lex_free(clst);
                return 0 as *mut mln_conf_lex_struct_t;
            }
            *s.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_uchar;
            memcpy(
                s.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                (*(*clst).text).data as *const libc::c_void,
                (*(*clst).text).len,
            );
            *s
                .offset(
                    len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
            mln_alloc_free((*(*clst).text).data as *mut libc::c_void);
            (*(*clst).text).data = s;
            break;
        } else {
            if sub_mark == 0 {
                break;
            }
            (*lex).error = 2 as libc::c_int;
            mln_conf_lex_free(clst);
            return 0 as *mut mln_conf_lex_struct_t;
        }
    }
    return clst;
}
#[inline]
unsafe extern "C" fn mln_conf_init() -> *mut mln_conf_t {
    let mut cf: *mut mln_conf_t = 0 as *mut mln_conf_t;
    cf = malloc(::core::mem::size_of::<mln_conf_t>() as libc::c_ulong)
        as *mut mln_conf_t;
    if cf.is_null() {
        fprintf(
            stderr,
            b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_init\0"))
                .as_ptr(),
            311 as libc::c_int,
        );
        return 0 as *mut mln_conf_t;
    }
    (*cf)
        .search = Some(
        mln_conf_domain_search
            as unsafe extern "C" fn(
                *mut mln_conf_t,
                *mut libc::c_char,
            ) -> *mut mln_conf_domain_t,
    );
    (*cf)
        .insert = Some(
        mln_conf_domain_insert
            as unsafe extern "C" fn(
                *mut mln_conf_t,
                *mut libc::c_char,
            ) -> *mut mln_conf_domain_t,
    );
    (*cf)
        .remove = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut mln_conf_t, *mut libc::c_char) -> ()>,
        mln_conf_domain_cb_t,
    >(
        Some(
            mln_conf_domain_remove
                as unsafe extern "C" fn(*mut mln_conf_t, *mut libc::c_char) -> (),
        ),
    );
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
        mln_conf_domain_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr
        .data_free = Some(
        mln_conf_domain_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*cf).domain = mln_rbtree_new(&mut rbattr);
    if ((*cf).domain).is_null() {
        fprintf(
            stderr,
            b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_init\0"))
                .as_ptr(),
            325 as libc::c_int,
        );
        free(cf as *mut libc::c_void);
        return 0 as *mut mln_conf_t;
    }
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut path: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut conf_file_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_len: mln_size_t = strlen(mln_path_conf());
    let mut hooks: mln_lex_hooks_t = mln_lex_hooks_t {
        excl_handler: None,
        excl_data: 0 as *mut libc::c_void,
        dblq_handler: None,
        dblq_data: 0 as *mut libc::c_void,
        nums_handler: None,
        nums_data: 0 as *mut libc::c_void,
        doll_handler: None,
        doll_data: 0 as *mut libc::c_void,
        perc_handler: None,
        perc_data: 0 as *mut libc::c_void,
        amp_handler: None,
        amp_data: 0 as *mut libc::c_void,
        sglq_handler: None,
        slgq_data: 0 as *mut libc::c_void,
        lpar_handler: None,
        lpar_data: 0 as *mut libc::c_void,
        rpar_handler: None,
        rpar_data: 0 as *mut libc::c_void,
        ast_handler: None,
        ast_data: 0 as *mut libc::c_void,
        plus_handler: None,
        plus_data: 0 as *mut libc::c_void,
        comma_handler: None,
        comma_data: 0 as *mut libc::c_void,
        sub_handler: None,
        sub_data: 0 as *mut libc::c_void,
        period_handler: None,
        period_data: 0 as *mut libc::c_void,
        slash_handler: None,
        slash_data: 0 as *mut libc::c_void,
        colon_handler: None,
        colon_data: 0 as *mut libc::c_void,
        semic_handler: None,
        semic_data: 0 as *mut libc::c_void,
        lagl_handler: None,
        lagl_data: 0 as *mut libc::c_void,
        equal_handler: None,
        equal_data: 0 as *mut libc::c_void,
        ragl_handler: None,
        ragl_data: 0 as *mut libc::c_void,
        ques_handler: None,
        ques_data: 0 as *mut libc::c_void,
        at_handler: None,
        at_data: 0 as *mut libc::c_void,
        lsquar_handler: None,
        lsquar_data: 0 as *mut libc::c_void,
        bslash_handler: None,
        bslash_data: 0 as *mut libc::c_void,
        rsquar_handler: None,
        rsquar_data: 0 as *mut libc::c_void,
        xor_handler: None,
        xor_data: 0 as *mut libc::c_void,
        under_handler: None,
        under_data: 0 as *mut libc::c_void,
        fulstp_handler: None,
        fulstp_data: 0 as *mut libc::c_void,
        lbrace_handler: None,
        lbrace_data: 0 as *mut libc::c_void,
        vertl_handler: None,
        vertl_data: 0 as *mut libc::c_void,
        rbrace_handler: None,
        rbrace_data: 0 as *mut libc::c_void,
        dash_handler: None,
        dash_data: 0 as *mut libc::c_void,
    };
    let mut lattr: mln_lex_attr = mln_lex_attr {
        pool: 0 as *mut mln_alloc_t,
        keywords: 0 as *mut mln_string_t,
        hooks: 0 as *mut mln_lex_hooks_t,
        preprocess_padding: [0; 4],
        type_0: 0,
        env: 0 as *mut mln_string_t,
        data: 0 as *mut mln_string_t,
    };
    pool = mln_alloc_init(0 as *mut mln_alloc_t);
    if pool.is_null() {
        fprintf(
            stderr,
            b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_init\0"))
                .as_ptr(),
            338 as libc::c_int,
        );
        mln_rbtree_free((*cf).domain);
        free(cf as *mut libc::c_void);
        return 0 as *mut mln_conf_t;
    }
    conf_file_path = mln_alloc_m(
        pool,
        path_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if conf_file_path.is_null() {
        fprintf(
            stderr,
            b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_init\0"))
                .as_ptr(),
            344 as libc::c_int,
        );
        mln_alloc_destroy(pool);
        mln_rbtree_free((*cf).domain);
        free(cf as *mut libc::c_void);
        return 0 as *mut mln_conf_t;
    }
    memcpy(
        conf_file_path as *mut libc::c_void,
        mln_path_conf() as *const libc::c_void,
        path_len,
    );
    *conf_file_path.offset(path_len as isize) = '\0' as i32 as libc::c_char;
    ({
        path.data = conf_file_path as mln_u8ptr_t;
        path.len = path_len;
        path.set_data_ref(1 as libc::c_int as mln_uauto_t);
        path.set_pool(0 as libc::c_int as mln_uauto_t);
        path.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut path;
        &mut path
    });
    if access(conf_file_path, 0 as libc::c_int) == 0 {
        lattr.pool = pool;
        lattr.keywords = conf_keywords.as_mut_ptr();
        memset(
            &mut hooks as *mut mln_lex_hooks_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_lex_hooks_t>() as libc::c_ulong,
        );
        hooks
            .slash_handler = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_conf_lex_struct_t,
            >,
            lex_hook,
        >(
            Some(
                mln_conf_lex_slash_handler
                    as unsafe extern "C" fn(
                        *mut mln_lex_t,
                        *mut libc::c_void,
                    ) -> *mut mln_conf_lex_struct_t,
            ),
        );
        hooks
            .sglq_handler = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_conf_lex_struct_t,
            >,
            lex_hook,
        >(
            Some(
                mln_conf_lex_sglq_handler
                    as unsafe extern "C" fn(
                        *mut mln_lex_t,
                        *mut libc::c_void,
                    ) -> *mut mln_conf_lex_struct_t,
            ),
        );
        hooks
            .dblq_handler = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_conf_lex_struct_t,
            >,
            lex_hook,
        >(
            Some(
                mln_conf_lex_dblq_handler
                    as unsafe extern "C" fn(
                        *mut mln_lex_t,
                        *mut libc::c_void,
                    ) -> *mut mln_conf_lex_struct_t,
            ),
        );
        lattr.hooks = &mut hooks;
        lattr.set_preprocess(1 as libc::c_int as mln_u32_t);
        lattr.type_0 = 1 as libc::c_int as mln_u32_t;
        lattr.data = &mut path;
        lattr.env = &mut mln_conf_env;
        if lattr.preprocess() != 0 {
            let mut lpd: *mut mln_lex_preprocess_data_t = mln_lex_preprocess_data_new(
                lattr.pool,
            );
            if lpd.is_null() {
                (*cf).lex = 0 as *mut mln_lex_t;
            } else {
                if (lattr.hooks).is_null() {
                    let mut __hooks: mln_lex_hooks_t = mln_lex_hooks_t {
                        excl_handler: None,
                        excl_data: 0 as *mut libc::c_void,
                        dblq_handler: None,
                        dblq_data: 0 as *mut libc::c_void,
                        nums_handler: None,
                        nums_data: 0 as *mut libc::c_void,
                        doll_handler: None,
                        doll_data: 0 as *mut libc::c_void,
                        perc_handler: None,
                        perc_data: 0 as *mut libc::c_void,
                        amp_handler: None,
                        amp_data: 0 as *mut libc::c_void,
                        sglq_handler: None,
                        slgq_data: 0 as *mut libc::c_void,
                        lpar_handler: None,
                        lpar_data: 0 as *mut libc::c_void,
                        rpar_handler: None,
                        rpar_data: 0 as *mut libc::c_void,
                        ast_handler: None,
                        ast_data: 0 as *mut libc::c_void,
                        plus_handler: None,
                        plus_data: 0 as *mut libc::c_void,
                        comma_handler: None,
                        comma_data: 0 as *mut libc::c_void,
                        sub_handler: None,
                        sub_data: 0 as *mut libc::c_void,
                        period_handler: None,
                        period_data: 0 as *mut libc::c_void,
                        slash_handler: None,
                        slash_data: 0 as *mut libc::c_void,
                        colon_handler: None,
                        colon_data: 0 as *mut libc::c_void,
                        semic_handler: None,
                        semic_data: 0 as *mut libc::c_void,
                        lagl_handler: None,
                        lagl_data: 0 as *mut libc::c_void,
                        equal_handler: None,
                        equal_data: 0 as *mut libc::c_void,
                        ragl_handler: None,
                        ragl_data: 0 as *mut libc::c_void,
                        ques_handler: None,
                        ques_data: 0 as *mut libc::c_void,
                        at_handler: None,
                        at_data: 0 as *mut libc::c_void,
                        lsquar_handler: None,
                        lsquar_data: 0 as *mut libc::c_void,
                        bslash_handler: None,
                        bslash_data: 0 as *mut libc::c_void,
                        rsquar_handler: None,
                        rsquar_data: 0 as *mut libc::c_void,
                        xor_handler: None,
                        xor_data: 0 as *mut libc::c_void,
                        under_handler: None,
                        under_data: 0 as *mut libc::c_void,
                        fulstp_handler: None,
                        fulstp_data: 0 as *mut libc::c_void,
                        lbrace_handler: None,
                        lbrace_data: 0 as *mut libc::c_void,
                        vertl_handler: None,
                        vertl_data: 0 as *mut libc::c_void,
                        rbrace_handler: None,
                        rbrace_data: 0 as *mut libc::c_void,
                        dash_handler: None,
                        dash_data: 0 as *mut libc::c_void,
                    };
                    memset(
                        &mut __hooks as *mut mln_lex_hooks_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<mln_lex_hooks_t>() as libc::c_ulong,
                    );
                    __hooks
                        .nums_handler = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                        >,
                        lex_hook,
                    >(
                        Some(
                            mln_conf_lex_nums_handler
                                as unsafe extern "C" fn(
                                    *mut mln_lex_t,
                                    *mut libc::c_void,
                                ) -> *mut mln_conf_lex_struct_t,
                        ),
                    );
                    __hooks.nums_data = lpd as *mut libc::c_void;
                    lattr.hooks = &mut __hooks;
                } else {
                    (*lattr.hooks)
                        .nums_handler = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                        >,
                        lex_hook,
                    >(
                        Some(
                            mln_conf_lex_nums_handler
                                as unsafe extern "C" fn(
                                    *mut mln_lex_t,
                                    *mut libc::c_void,
                                ) -> *mut mln_conf_lex_struct_t,
                        ),
                    );
                    (*lattr.hooks).nums_data = lpd as *mut libc::c_void;
                }
                (*cf).lex = mln_lex_init(&mut lattr);
                if !((*cf).lex).is_null() {
                    if !(lattr.hooks).is_null() {
                        mln_conf_lex_set_hooks((*cf).lex);
                    }
                    (*(*cf).lex).preprocess_data = lpd;
                } else {
                    mln_lex_preprocess_data_free(lpd);
                }
            }
        } else {
            (*cf).lex = mln_lex_init(&mut lattr);
            if !((*cf).lex).is_null() && !(lattr.hooks).is_null() {
                mln_conf_lex_set_hooks((*cf).lex);
            }
        }
        mln_alloc_free(conf_file_path as *mut libc::c_void);
        if ((*cf).lex).is_null() {
            fprintf(
                stderr,
                b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"mln_conf_init\0"))
                    .as_ptr(),
                373 as libc::c_int,
            );
            mln_alloc_destroy(pool);
            mln_rbtree_free((*cf).domain);
            free(cf as *mut libc::c_void);
            return 0 as *mut mln_conf_t;
        }
    } else {
        fprintf(
            stderr,
            b"[Warn] Configuration file [%s] not found, default configuration will be used.\n\0"
                as *const u8 as *const libc::c_char,
            conf_file_path,
        );
        (*cf).lex = 0 as *mut mln_lex_t;
    }
    if (((*cf).insert)
        .expect("non-null function pointer")(cf, default_domain.as_mut_ptr()))
        .is_null()
    {
        fprintf(
            stderr,
            b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_init\0"))
                .as_ptr(),
            385 as libc::c_int,
        );
        mln_conf_destroy(cf);
        return 0 as *mut mln_conf_t;
    }
    (*cf).cb = 0 as *mut mln_ipc_cb_t;
    return cf;
}
unsafe extern "C" fn mln_conf_destroy(mut cf: *mut mln_conf_t) {
    if cf.is_null() {
        return;
    }
    mln_conf_destroy_lex(cf);
    if !((*cf).domain).is_null() {
        mln_rbtree_free((*cf).domain);
        (*cf).domain = 0 as *mut mln_rbtree_t;
    }
    if !((*cf).cb).is_null() {
        mln_ipc_handler_unregister((*cf).cb);
    }
    free(cf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_conf_destroy_lex(mut cf: *mut mln_conf_t) {
    if cf.is_null() {
        return;
    }
    if !((*cf).lex).is_null() {
        let mut pool: *mut mln_alloc_t = (*(*cf).lex).pool;
        mln_lex_destroy((*cf).lex);
        mln_alloc_destroy(pool);
        (*cf).lex = 0 as *mut mln_lex_t;
    }
}
unsafe extern "C" fn mln_conf_domain_init(
    mut cf: *mut mln_conf_t,
    mut domain_name: *mut mln_string_t,
) -> *mut mln_conf_domain_t {
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    cd = malloc(::core::mem::size_of::<mln_conf_domain_t>() as libc::c_ulong)
        as *mut mln_conf_domain_t;
    if cd.is_null() {
        return 0 as *mut mln_conf_domain_t;
    }
    (*cd)
        .search = Some(
        mln_conf_cmd_search
            as unsafe extern "C" fn(
                *mut mln_conf_domain_t,
                *mut libc::c_char,
            ) -> *mut mln_conf_cmd_t,
    );
    (*cd)
        .insert = Some(
        mln_conf_cmd_insert
            as unsafe extern "C" fn(
                *mut mln_conf_domain_t,
                *mut libc::c_char,
            ) -> *mut mln_conf_cmd_t,
    );
    (*cd)
        .remove = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut mln_conf_domain_t, *mut libc::c_char) -> ()>,
        mln_conf_cmd_cb_t,
    >(
        Some(
            mln_conf_cmd_remove
                as unsafe extern "C" fn(*mut mln_conf_domain_t, *mut libc::c_char) -> (),
        ),
    );
    (*cd).domain_name = mln_string_dup(domain_name);
    if ((*cd).domain_name).is_null() {
        free(cd as *mut libc::c_void);
        return 0 as *mut mln_conf_domain_t;
    }
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
        mln_conf_cmd_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr
        .data_free = Some(
        mln_conf_cmd_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*cd).cmd = mln_rbtree_new(&mut rbattr);
    if ((*cd).cmd).is_null() {
        let mut __s: *mut mln_string_t = (*cd).domain_name;
        if !__s.is_null() {
            let ref mut fresh14 = (*__s).ref_0();
            let fresh15 = *fresh14;
            *fresh14 = (*fresh14).wrapping_sub(1);
            if fresh15 <= 1 as libc::c_int as libc::c_ulong {
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
        free(cd as *mut libc::c_void);
        return 0 as *mut mln_conf_domain_t;
    }
    return cd;
}
unsafe extern "C" fn mln_conf_domain_destroy(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut cd: *mut mln_conf_domain_t = data as *mut mln_conf_domain_t;
    if !((*cd).domain_name).is_null() {
        let mut __s: *mut mln_string_t = (*cd).domain_name;
        if !__s.is_null() {
            let ref mut fresh16 = (*__s).ref_0();
            let fresh17 = *fresh16;
            *fresh16 = (*fresh16).wrapping_sub(1);
            if fresh17 <= 1 as libc::c_int as libc::c_ulong {
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
        (*cd).domain_name = 0 as *mut mln_string_t;
    }
    if !((*cd).cmd).is_null() {
        mln_rbtree_free((*cd).cmd);
        (*cd).cmd = 0 as *mut mln_rbtree_t;
    }
    free(cd as *mut libc::c_void);
}
unsafe extern "C" fn mln_conf_domain_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut d1: *mut mln_conf_domain_t = data1 as *mut mln_conf_domain_t;
    let mut d2: *mut mln_conf_domain_t = data2 as *mut mln_conf_domain_t;
    return mln_string_strcmp((*d1).domain_name, (*d2).domain_name);
}
unsafe extern "C" fn mln_conf_domain_search(
    mut cf: *mut mln_conf_t,
    mut domain_name: *mut libc::c_char,
) -> *mut mln_conf_domain_t {
    let mut str: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut tmp: mln_conf_domain_t = mln_conf_domain_t {
        search: None,
        insert: None,
        remove: None,
        domain_name: 0 as *mut mln_string_t,
        cmd: 0 as *mut mln_rbtree_t,
    };
    ({
        str.data = domain_name as mln_u8ptr_t;
        str.len = strlen(domain_name);
        str.set_data_ref(1 as libc::c_int as mln_uauto_t);
        str.set_pool(0 as libc::c_int as mln_uauto_t);
        str.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut str;
        &mut str
    });
    tmp.domain_name = &mut str;
    rn = mln_rbtree_search(
        (*cf).domain,
        &mut tmp as *mut mln_conf_domain_t as *mut libc::c_void,
    );
    if rn == &mut (*(*cf).domain).nil as *mut mln_rbtree_node_t {
        return 0 as *mut mln_conf_domain_t;
    }
    return (*rn).data as *mut mln_conf_domain_t;
}
unsafe extern "C" fn mln_conf_domain_insert(
    mut cf: *mut mln_conf_t,
    mut domain_name: *mut libc::c_char,
) -> *mut mln_conf_domain_t {
    let mut name: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    ({
        name.data = domain_name as mln_u8ptr_t;
        name.len = strlen(domain_name);
        name.set_data_ref(1 as libc::c_int as mln_uauto_t);
        name.set_pool(0 as libc::c_int as mln_uauto_t);
        name.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut name;
        &mut name
    });
    let mut cd: *mut mln_conf_domain_t = mln_conf_domain_init(cf, &mut name);
    if cd.is_null() {
        return 0 as *mut mln_conf_domain_t;
    }
    rn = mln_rbtree_node_new((*cf).domain, cd as *mut libc::c_void);
    if rn.is_null() {
        mln_conf_domain_destroy(cd as *mut libc::c_void);
        return 0 as *mut mln_conf_domain_t;
    }
    mln_rbtree_insert((*cf).domain, rn);
    return cd;
}
unsafe extern "C" fn mln_conf_domain_remove(
    mut cf: *mut mln_conf_t,
    mut domain_name: *mut libc::c_char,
) {
    let mut dname: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut cd: mln_conf_domain_t = mln_conf_domain_t {
        search: None,
        insert: None,
        remove: None,
        domain_name: 0 as *mut mln_string_t,
        cmd: 0 as *mut mln_rbtree_t,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    ({
        dname.data = domain_name as mln_u8ptr_t;
        dname.len = strlen(domain_name);
        dname.set_data_ref(1 as libc::c_int as mln_uauto_t);
        dname.set_pool(0 as libc::c_int as mln_uauto_t);
        dname.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut dname;
        &mut dname
    });
    cd.domain_name = &mut dname;
    rn = mln_rbtree_search(
        (*cf).domain,
        &mut cd as *mut mln_conf_domain_t as *mut libc::c_void,
    );
    if !(rn == &mut (*(*cf).domain).nil as *mut mln_rbtree_node_t) {
        mln_rbtree_delete((*cf).domain, rn);
        mln_rbtree_node_free((*cf).domain, rn);
    }
}
unsafe extern "C" fn mln_conf_cmd_init(
    mut cmd_name: *mut mln_string_t,
) -> *mut mln_conf_cmd_t {
    let mut cc: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    cc = malloc(::core::mem::size_of::<mln_conf_cmd_t>() as libc::c_ulong)
        as *mut mln_conf_cmd_t;
    if cc.is_null() {
        return 0 as *mut mln_conf_cmd_t;
    }
    (*cc).cmd_name = mln_string_dup(cmd_name);
    if ((*cc).cmd_name).is_null() {
        free(cc as *mut libc::c_void);
        return 0 as *mut mln_conf_cmd_t;
    }
    (*cc)
        .search = Some(
        mln_conf_item_search
            as unsafe extern "C" fn(
                *mut mln_conf_cmd_t,
                mln_u32_t,
            ) -> *mut mln_conf_item_t,
    );
    (*cc)
        .update = Some(
        mln_conf_item_update
            as unsafe extern "C" fn(
                *mut mln_conf_cmd_t,
                *mut mln_conf_item_t,
                mln_u32_t,
            ) -> libc::c_int,
    );
    (*cc).arg_tbl = 0 as *mut mln_conf_item_t;
    (*cc).n_args = 0 as libc::c_int as mln_u32_t;
    return cc;
}
unsafe extern "C" fn mln_conf_cmd_destroy(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut cc: *mut mln_conf_cmd_t = data as *mut mln_conf_cmd_t;
    if !((*cc).cmd_name).is_null() {
        let mut __s: *mut mln_string_t = (*cc).cmd_name;
        if !__s.is_null() {
            let ref mut fresh18 = (*__s).ref_0();
            let fresh19 = *fresh18;
            *fresh18 = (*fresh18).wrapping_sub(1);
            if fresh19 <= 1 as libc::c_int as libc::c_ulong {
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
        (*cc).cmd_name = 0 as *mut mln_string_t;
    }
    if !((*cc).arg_tbl).is_null() {
        let mut i: mln_u32_t = 0;
        let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
        i = 0 as libc::c_int as mln_u32_t;
        while i < (*cc).n_args {
            ci = &mut *((*cc).arg_tbl).offset(i as isize) as *mut mln_conf_item_t;
            if !((*ci).type_0 as libc::c_uint
                == CONF_NONE as libc::c_int as libc::c_uint)
            {
                if (*ci).type_0 as libc::c_uint
                    == CONF_STR as libc::c_int as libc::c_uint
                {
                    let mut __s: *mut mln_string_t = (*ci).val.s;
                    if !__s.is_null() {
                        let ref mut fresh20 = (*__s).ref_0();
                        let fresh21 = *fresh20;
                        *fresh20 = (*fresh20).wrapping_sub(1);
                        if fresh21 <= 1 as libc::c_int as libc::c_ulong {
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
                    (*ci).val.s = 0 as *mut mln_string_t;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        free((*cc).arg_tbl as *mut libc::c_void);
        (*cc).arg_tbl = 0 as *mut mln_conf_item_t;
        (*cc).n_args = 0 as libc::c_int as mln_u32_t;
    }
    free(cc as *mut libc::c_void);
}
unsafe extern "C" fn mln_conf_cmd_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut c1: *mut mln_conf_cmd_t = data1 as *mut mln_conf_cmd_t;
    let mut c2: *mut mln_conf_cmd_t = data2 as *mut mln_conf_cmd_t;
    return mln_string_strcmp((*c1).cmd_name, (*c2).cmd_name);
}
unsafe extern "C" fn mln_conf_cmd_search(
    mut cd: *mut mln_conf_domain_t,
    mut cmd_name: *mut libc::c_char,
) -> *mut mln_conf_cmd_t {
    let mut str: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut cmd: mln_conf_cmd_t = mln_conf_cmd_t {
        cmd_name: 0 as *mut mln_string_t,
        search: None,
        update: None,
        arg_tbl: 0 as *mut mln_conf_item_t,
        n_args: 0,
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    cmd.cmd_name = &mut str;
    ({
        str.data = cmd_name as mln_u8ptr_t;
        str.len = strlen(cmd_name);
        str.set_data_ref(1 as libc::c_int as mln_uauto_t);
        str.set_pool(0 as libc::c_int as mln_uauto_t);
        str.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut str;
        &mut str
    });
    rn = mln_rbtree_search(
        (*cd).cmd,
        &mut cmd as *mut mln_conf_cmd_t as *mut libc::c_void,
    );
    if rn == &mut (*(*cd).cmd).nil as *mut mln_rbtree_node_t {
        return 0 as *mut mln_conf_cmd_t;
    }
    return (*rn).data as *mut mln_conf_cmd_t;
}
unsafe extern "C" fn mln_conf_cmd_insert(
    mut cd: *mut mln_conf_domain_t,
    mut cmd_name: *mut libc::c_char,
) -> *mut mln_conf_cmd_t {
    let mut cmd: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut cname: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        cname.data = cmd_name as mln_u8ptr_t;
        cname.len = strlen(cmd_name);
        cname.set_data_ref(1 as libc::c_int as mln_uauto_t);
        cname.set_pool(0 as libc::c_int as mln_uauto_t);
        cname.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut cname;
        &mut cname
    });
    cmd = mln_conf_cmd_init(&mut cname);
    if cmd.is_null() {
        return 0 as *mut mln_conf_cmd_t;
    }
    rn = mln_rbtree_node_new((*cd).cmd, cmd as *mut libc::c_void);
    if rn.is_null() {
        mln_conf_cmd_destroy(cmd as *mut libc::c_void);
        return 0 as *mut mln_conf_cmd_t;
    }
    mln_rbtree_insert((*cd).cmd, rn);
    return cmd;
}
unsafe extern "C" fn mln_conf_cmd_remove(
    mut cd: *mut mln_conf_domain_t,
    mut cmd_name: *mut libc::c_char,
) {
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut cname: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut cmd: mln_conf_cmd_t = mln_conf_cmd_t {
        cmd_name: 0 as *mut mln_string_t,
        search: None,
        update: None,
        arg_tbl: 0 as *mut mln_conf_item_t,
        n_args: 0,
    };
    ({
        cname.data = cmd_name as mln_u8ptr_t;
        cname.len = strlen(cmd_name);
        cname.set_data_ref(1 as libc::c_int as mln_uauto_t);
        cname.set_pool(0 as libc::c_int as mln_uauto_t);
        cname.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut cname;
        &mut cname
    });
    cmd.cmd_name = &mut cname;
    rn = mln_rbtree_search(
        (*cd).cmd,
        &mut cmd as *mut mln_conf_cmd_t as *mut libc::c_void,
    );
    if !(rn == &mut (*(*cd).cmd).nil as *mut mln_rbtree_node_t) {
        mln_rbtree_delete((*cd).cmd, rn);
        mln_rbtree_node_free((*cd).cmd, rn);
    }
}
unsafe extern "C" fn mln_isvalid_item(
    mut cls: *mut mln_conf_lex_struct_t,
) -> libc::c_int {
    match (*cls).type_0 as libc::c_uint {
        2 | 4 | 44 | 40 | 41 | 43 => {}
        _ => return 0 as libc::c_int,
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mln_conf_item_init(
    mut cf: *mut mln_conf_t,
    mut cls: *mut mln_conf_lex_struct_t,
    mut ci: *mut mln_conf_item_t,
) -> libc::c_int {
    if mln_isvalid_item(cls) == 0 {
        let mut path: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
            0 as *mut mln_string_t
        } else if (*(*(*cf).lex).cur).type_0 == 0 as libc::c_int as libc::c_uint {
            0 as *mut mln_string_t
        } else {
            (*(*(*cf).lex).cur).data
        };
        fprintf(stderr, b"Configuration error. \0" as *const u8 as *const libc::c_char);
        if !path.is_null() {
            fprintf(
                stderr,
                b"%s:\0" as *const u8 as *const libc::c_char,
                (*path).data as *mut libc::c_char,
            );
        }
        fprintf(
            stderr,
            b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
            (*cls).line,
            (*(*cls).text).data as *mut libc::c_char,
            b"Invalid type of item\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    match (*cls).type_0 as libc::c_uint {
        2 => {
            (*ci).type_0 = CONF_INT;
            (*ci).val.i = atol((*(*cls).text).data as *mut libc::c_char);
        }
        4 => {
            (*ci).type_0 = CONF_FLOAT;
            (*ci)
                .val
                .f = atof((*(*cls).text).data as *mut libc::c_char) as libc::c_float;
        }
        44 => {
            (*ci).type_0 = CONF_STR;
            (*ci).val.s = mln_string_dup((*cls).text);
            if ((*ci).val.s).is_null() {
                fprintf(
                    stderr,
                    b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"mln_conf_item_init\0"))
                        .as_ptr(),
                    644 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        40 => {
            (*ci).type_0 = CONF_BOOL;
            (*ci).val.b = 1 as libc::c_int as mln_u8_t;
        }
        41 => {
            (*ci).type_0 = CONF_BOOL;
            (*ci).val.b = 0 as libc::c_int as mln_u8_t;
        }
        43 => {
            (*ci).type_0 = CONF_CHAR;
            (*ci)
                .val
                .c = *((*(*cls).text).data).offset(0 as libc::c_int as isize)
                as mln_s8_t;
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: No such token type.\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"mln_conf_item_init\0"))
                    .as_ptr(),
                661 as libc::c_int,
            );
            abort();
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_conf_item_search(
    mut cmd: *mut mln_conf_cmd_t,
    mut index: mln_u32_t,
) -> *mut mln_conf_item_t {
    if index == 0 || index > (*cmd).n_args {
        return 0 as *mut mln_conf_item_t;
    }
    return &mut *((*cmd).arg_tbl)
        .offset(index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut mln_conf_item_t;
}
unsafe extern "C" fn mln_conf_item_update(
    mut cmd: *mut mln_conf_cmd_t,
    mut items: *mut mln_conf_item_t,
    mut nitems: mln_u32_t,
) -> libc::c_int {
    let mut i: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut args: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    args = malloc(
        (::core::mem::size_of::<mln_conf_item_t>() as libc::c_ulong)
            .wrapping_mul(nitems as libc::c_ulong),
    ) as *mut mln_conf_item_t;
    if args.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as mln_u32_t;
    while i < nitems {
        (*args.offset(i as isize)).type_0 = (*items.offset(i as isize)).type_0;
        if (*items.offset(i as isize)).type_0 as libc::c_uint
            == CONF_STR as libc::c_int as libc::c_uint
        {
            let ref mut fresh22 = (*args.offset(i as isize)).val.s;
            *fresh22 = mln_string_dup((*items.offset(i as isize)).val.s);
            if (*fresh22).is_null() {
                j = 0 as libc::c_int as mln_u32_t;
                while j < i {
                    if (*args.offset(j as isize)).type_0 as libc::c_uint
                        == CONF_STR as libc::c_int as libc::c_uint
                        && !((*args.offset(j as isize)).val.s).is_null()
                    {
                        let mut __s: *mut mln_string_t = (*args.offset(j as isize))
                            .val
                            .s;
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
                    j = j.wrapping_add(1);
                    j;
                }
                free(args as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        } else {
            (*args.offset(i as isize)).val = (*items.offset(i as isize)).val;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*cmd).arg_tbl).is_null() {
        i = 0 as libc::c_int as mln_u32_t;
        while i < (*cmd).n_args {
            ci = &mut *((*cmd).arg_tbl).offset(i as isize) as *mut mln_conf_item_t;
            if !((*ci).type_0 as libc::c_uint
                == CONF_NONE as libc::c_int as libc::c_uint)
            {
                if (*ci).type_0 as libc::c_uint
                    == CONF_STR as libc::c_int as libc::c_uint
                {
                    let mut __s: *mut mln_string_t = (*ci).val.s;
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
                    (*ci).val.s = 0 as *mut mln_string_t;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        free((*cmd).arg_tbl as *mut libc::c_void);
    }
    (*cmd).arg_tbl = args;
    (*cmd).n_args = nitems;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_conf_item_recursive(
    mut cf: *mut mln_conf_t,
    mut cc: *mut mln_conf_cmd_t,
    mut cls: *mut mln_conf_lex_struct_t,
    mut cnt: mln_u32_t,
) -> libc::c_int {
    if cls.is_null() {
        fprintf(
            stderr,
            b"%s:%d: Get token error. %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"mln_conf_item_recursive\0"))
                .as_ptr(),
            723 as libc::c_int,
            mln_lex_strerror((*cf).lex),
        );
        return -(1 as libc::c_int);
    }
    if (*cls).type_0 as libc::c_uint == CONF_TK_EOF as libc::c_int as libc::c_uint {
        let mut path: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
            0 as *mut mln_string_t
        } else if (*(*(*cf).lex).cur).type_0 == 0 as libc::c_int as libc::c_uint {
            0 as *mut mln_string_t
        } else {
            (*(*(*cf).lex).cur).data
        };
        fprintf(stderr, b"Configuration error. \0" as *const u8 as *const libc::c_char);
        if !path.is_null() {
            fprintf(
                stderr,
                b"%s:\0" as *const u8 as *const libc::c_char,
                (*path).data as *mut libc::c_char,
            );
        }
        fprintf(
            stderr,
            b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
            (*cls).line,
            (*(*cls).text).data as *mut libc::c_char,
            b"Invalid end of file\0" as *const u8 as *const libc::c_char,
        );
        mln_conf_lex_free(cls);
        return -(1 as libc::c_int);
    }
    if (*cls).type_0 as libc::c_uint == CONF_TK_SEMIC as libc::c_int as libc::c_uint {
        if cnt == 0 {
            mln_conf_lex_free(cls);
            return 0 as libc::c_int;
        }
        (*cc)
            .arg_tbl = calloc(
            cnt as libc::c_ulong,
            ::core::mem::size_of::<mln_conf_item_t>() as libc::c_ulong,
        ) as *mut mln_conf_item_t;
        if ((*cc).arg_tbl).is_null() {
            fprintf(
                stderr,
                b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"mln_conf_item_recursive\0"))
                    .as_ptr(),
                738 as libc::c_int,
            );
            mln_conf_lex_free(cls);
            return -(1 as libc::c_int);
        }
        (*cc).n_args = cnt;
        mln_conf_lex_free(cls);
        return 0 as libc::c_int;
    }
    let mut next: *mut mln_conf_lex_struct_t = mln_conf_token((*cf).lex);
    if mln_conf_item_recursive(
        cf,
        cc,
        next,
        cnt.wrapping_add(1 as libc::c_int as libc::c_uint),
    ) < 0 as libc::c_int
    {
        mln_conf_lex_free(cls);
        return -(1 as libc::c_int);
    }
    let mut ci: *mut mln_conf_item_t = &mut *((*cc).arg_tbl).offset(cnt as isize)
        as *mut mln_conf_item_t;
    let mut ret: libc::c_int = mln_conf_item_init(cf, cls, ci);
    mln_conf_lex_free(cls);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _mln_conf_load(
    mut cf: *mut mln_conf_t,
    mut current: *mut mln_conf_domain_t,
) -> libc::c_int {
    let mut fir: *mut mln_conf_lex_struct_t = 0 as *mut mln_conf_lex_struct_t;
    let mut next: *mut mln_conf_lex_struct_t = 0 as *mut mln_conf_lex_struct_t;
    let mut cmd: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    let mut dname: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        dname.data = default_domain.as_mut_ptr() as mln_u8ptr_t;
        dname.len = strlen(default_domain.as_mut_ptr());
        dname.set_data_ref(1 as libc::c_int as mln_uauto_t);
        dname.set_pool(0 as libc::c_int as mln_uauto_t);
        dname.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut dname;
        &mut dname
    });
    loop {
        fir = mln_conf_token((*cf).lex);
        if fir.is_null() {
            fprintf(
                stderr,
                b"%s:%d: Get token error. %s\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"_mln_conf_load\0"))
                    .as_ptr(),
                769 as libc::c_int,
                mln_lex_strerror((*cf).lex),
            );
            return -(1 as libc::c_int);
        } else if (*fir).type_0 as libc::c_uint
            == CONF_TK_EOF as libc::c_int as libc::c_uint
        {
            mln_conf_lex_free(fir);
            break;
        } else {
            if (*fir).type_0 as libc::c_uint
                == CONF_TK_RBRACE as libc::c_int as libc::c_uint
            {
                if mln_string_strcmp((*current).domain_name, &mut dname) != 0 {
                    mln_conf_lex_free(fir);
                    return 0 as libc::c_int;
                }
                let mut path: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
                    0 as *mut mln_string_t
                } else if (*(*(*cf).lex).cur).type_0 == 0 as libc::c_int as libc::c_uint
                {
                    0 as *mut mln_string_t
                } else {
                    (*(*(*cf).lex).cur).data
                };
                fprintf(
                    stderr,
                    b"Configuration error. \0" as *const u8 as *const libc::c_char,
                );
                if !path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:\0" as *const u8 as *const libc::c_char,
                        (*path).data as *mut libc::c_char,
                    );
                }
                fprintf(
                    stderr,
                    b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
                    (*fir).line,
                    (*(*fir).text).data as *mut libc::c_char,
                    b"Invalid right brace\0" as *const u8 as *const libc::c_char,
                );
                mln_conf_lex_free(fir);
                return -(1 as libc::c_int);
            } else if (*fir).type_0 as libc::c_uint
                != CONF_TK_ID as libc::c_int as libc::c_uint
            {
                let mut path_0: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
                    0 as *mut mln_string_t
                } else if (*(*(*cf).lex).cur).type_0 == 0 as libc::c_int as libc::c_uint
                {
                    0 as *mut mln_string_t
                } else {
                    (*(*(*cf).lex).cur).data
                };
                fprintf(
                    stderr,
                    b"Configuration error. \0" as *const u8 as *const libc::c_char,
                );
                if !path_0.is_null() {
                    fprintf(
                        stderr,
                        b"%s:\0" as *const u8 as *const libc::c_char,
                        (*path_0).data as *mut libc::c_char,
                    );
                }
                fprintf(
                    stderr,
                    b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
                    (*fir).line,
                    (*(*fir).text).data as *mut libc::c_char,
                    b"Unexpected token\0" as *const u8 as *const libc::c_char,
                );
                mln_conf_lex_free(fir);
                return -(1 as libc::c_int);
            }
            next = mln_conf_token((*cf).lex);
            if next.is_null() {
                mln_conf_lex_free(fir);
                fprintf(
                    stderr,
                    b"%s:%d: Get token error. %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"_mln_conf_load\0"))
                        .as_ptr(),
                    790 as libc::c_int,
                    mln_lex_strerror((*cf).lex),
                );
                return -(1 as libc::c_int);
            } else if (*next).type_0 as libc::c_uint
                == CONF_TK_EOF as libc::c_int as libc::c_uint
            {
                let mut path_1: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
                    0 as *mut mln_string_t
                } else if (*(*(*cf).lex).cur).type_0 == 0 as libc::c_int as libc::c_uint
                {
                    0 as *mut mln_string_t
                } else {
                    (*(*(*cf).lex).cur).data
                };
                fprintf(
                    stderr,
                    b"Configuration error. \0" as *const u8 as *const libc::c_char,
                );
                if !path_1.is_null() {
                    fprintf(
                        stderr,
                        b"%s:\0" as *const u8 as *const libc::c_char,
                        (*path_1).data as *mut libc::c_char,
                    );
                }
                fprintf(
                    stderr,
                    b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
                    (*next).line,
                    (*(*next).text).data as *mut libc::c_char,
                    b"Invalid end of file\0" as *const u8 as *const libc::c_char,
                );
                mln_conf_lex_free(fir);
                mln_conf_lex_free(next);
                return -(1 as libc::c_int);
            }
            if (*next).type_0 as libc::c_uint
                == CONF_TK_LBRACE as libc::c_int as libc::c_uint
            {
                mln_conf_lex_free(next);
                if mln_string_strcmp((*current).domain_name, &mut dname) != 0 {
                    let mut path_2: *mut mln_string_t = if ((*(*cf).lex).cur).is_null() {
                        0 as *mut mln_string_t
                    } else if (*(*(*cf).lex).cur).type_0
                        == 0 as libc::c_int as libc::c_uint
                    {
                        0 as *mut mln_string_t
                    } else {
                        (*(*(*cf).lex).cur).data
                    };
                    fprintf(
                        stderr,
                        b"Configuration error. \0" as *const u8 as *const libc::c_char,
                    );
                    if !path_2.is_null() {
                        fprintf(
                            stderr,
                            b"%s:\0" as *const u8 as *const libc::c_char,
                            (*path_2).data as *mut libc::c_char,
                        );
                    }
                    fprintf(
                        stderr,
                        b"%d: \"%s\" %s.\n\0" as *const u8 as *const libc::c_char,
                        (*fir).line,
                        (*(*fir).text).data as *mut libc::c_char,
                        b"Illegal domain\0" as *const u8 as *const libc::c_char,
                    );
                    mln_conf_lex_free(fir);
                    return -(1 as libc::c_int);
                }
                cd = ((*cf).insert)
                    .expect(
                        "non-null function pointer",
                    )(cf, (*(*fir).text).data as *mut libc::c_char);
                mln_conf_lex_free(fir);
                if cd.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"_mln_conf_load\0"))
                            .as_ptr(),
                        811 as libc::c_int,
                    );
                    return -(1 as libc::c_int);
                }
                if _mln_conf_load(cf, cd) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
            } else {
                cmd = ((*current).insert)
                    .expect(
                        "non-null function pointer",
                    )(current, (*(*fir).text).data as *mut libc::c_char);
                mln_conf_lex_free(fir);
                if cmd.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%d: No memory.\n\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"_mln_conf_load\0"))
                            .as_ptr(),
                        821 as libc::c_int,
                    );
                    mln_conf_lex_free(next);
                    return -(1 as libc::c_int);
                }
                if mln_conf_item_recursive(cf, cmd, next, 0 as libc::c_int as mln_u32_t)
                    < 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_load() -> libc::c_int {
    let mut dname: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    let mut tmp: mln_conf_domain_t = mln_conf_domain_t {
        search: None,
        insert: None,
        remove: None,
        domain_name: 0 as *mut mln_string_t,
        cmd: 0 as *mut mln_rbtree_t,
    };
    if !g_conf.is_null() {
        return 0 as libc::c_int;
    }
    g_conf = mln_conf_init();
    if g_conf.is_null() {
        return -(1 as libc::c_int);
    }
    ({
        dname.data = default_domain.as_mut_ptr() as mln_u8ptr_t;
        dname.len = strlen(default_domain.as_mut_ptr());
        dname.set_data_ref(1 as libc::c_int as mln_uauto_t);
        dname.set_pool(0 as libc::c_int as mln_uauto_t);
        dname.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut dname;
        &mut dname
    });
    tmp.domain_name = &mut dname;
    rn = mln_rbtree_search(
        (*g_conf).domain,
        &mut tmp as *mut mln_conf_domain_t as *mut libc::c_void,
    );
    cd = (*rn).data as *mut mln_conf_domain_t;
    if !((*g_conf).lex).is_null() {
        let mut ret: mln_s32_t = _mln_conf_load(g_conf, cd);
        mln_conf_destroy_lex(g_conf);
        if ret < 0 as libc::c_int {
            mln_conf_destroy(g_conf);
            g_conf = 0 as *mut mln_conf_t;
            return -(1 as libc::c_int);
        }
    }
    (*g_conf)
        .cb = mln_ipc_handler_register(
        1 as libc::c_int as mln_u32_t,
        Some(
            mln_conf_reload_master_handler
                as unsafe extern "C" fn(
                    *mut mln_event_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    mln_u32_t,
                    *mut *mut libc::c_void,
                ) -> (),
        ),
        Some(
            mln_conf_reload_worker_handler
                as unsafe extern "C" fn(
                    *mut mln_event_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                    mln_u32_t,
                    *mut *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    if ((*g_conf).cb).is_null() {
        mln_conf_destroy(g_conf);
        g_conf = 0 as *mut mln_conf_t;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_free() {
    if g_conf.is_null() {
        return;
    }
    mln_conf_destroy(g_conf);
    g_conf = 0 as *mut mln_conf_t;
}
unsafe extern "C" fn mln_conf_hook_init() -> *mut mln_conf_hook_t {
    let mut ch: *mut mln_conf_hook_t = malloc(
        ::core::mem::size_of::<mln_conf_hook_t>() as libc::c_ulong,
    ) as *mut mln_conf_hook_t;
    if ch.is_null() {
        return 0 as *mut mln_conf_hook_t;
    }
    (*ch).reload = None;
    (*ch).data = 0 as *mut libc::c_void;
    (*ch).prev = 0 as *mut mln_conf_hook_s;
    (*ch).next = 0 as *mut mln_conf_hook_s;
    return ch;
}
unsafe extern "C" fn mln_conf_hook_destroy(mut ch: *mut mln_conf_hook_t) {
    if ch.is_null() {
        return;
    }
    free(ch as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_hook_set(
    mut reload: reload_handler,
    mut data: *mut libc::c_void,
) -> *mut mln_conf_hook_t {
    let mut ch: *mut mln_conf_hook_t = mln_conf_hook_init();
    if ch.is_null() {
        return 0 as *mut mln_conf_hook_t;
    }
    (*ch).reload = reload;
    (*ch).data = data;
    conf_hook_chain_add(&mut g_conf_hook_head, &mut g_conf_hook_tail, ch);
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_hook_unset(mut hook: *mut mln_conf_hook_t) {
    if hook.is_null() {
        return;
    }
    conf_hook_chain_del(&mut g_conf_hook_head, &mut g_conf_hook_tail, hook);
    mln_conf_hook_destroy(hook);
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_hook_free() {
    let mut ch: *mut mln_conf_hook_t = 0 as *mut mln_conf_hook_t;
    loop {
        ch = g_conf_hook_head;
        if ch.is_null() {
            break;
        }
        conf_hook_chain_del(&mut g_conf_hook_head, &mut g_conf_hook_tail, ch);
        mln_conf_hook_destroy(ch);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_reload() -> libc::c_int {
    mln_conf_free();
    mln_conf_load();
    let mut ch: *mut mln_conf_hook_t = 0 as *mut mln_conf_hook_t;
    ch = g_conf_hook_head;
    while !ch.is_null() {
        if ((*ch).reload).is_some()
            && ((*ch).reload).expect("non-null function pointer")((*ch).data)
                < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        ch = (*ch).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf() -> *mut mln_conf_t {
    return g_conf;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_cmd_num(
    mut cf: *mut mln_conf_t,
    mut domain: *mut libc::c_char,
) -> mln_u32_t {
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect("non-null function pointer")(cf, domain);
    if cd.is_null() {
        return 0 as libc::c_int as mln_u32_t;
    }
    return (*(*cd).cmd).nr_node as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_cmds(
    mut cf: *mut mln_conf_t,
    mut domain: *mut libc::c_char,
    mut v: *mut *mut mln_conf_cmd_t,
) {
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect("non-null function pointer")(cf, domain);
    if cd.is_null() {
        return;
    }
    let mut ccs: conf_cmds_scan_s = conf_cmds_scan_s {
        cc: 0 as *mut *mut mln_conf_cmd_t,
        pos: 0,
    };
    ccs.cc = v;
    ccs.pos = 0 as libc::c_int as mln_u32_t;
    if mln_rbtree_iterate(
        (*cd).cmd,
        Some(
            mln_conf_cmds_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut ccs as *mut conf_cmds_scan_s as *mut libc::c_void,
    ) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_conf.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mln_conf_cmds\0"))
                .as_ptr(),
            957 as libc::c_int,
            b"Shouldn't be here.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
}
unsafe extern "C" fn mln_conf_cmds_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut ccs: *mut conf_cmds_scan_s = udata as *mut conf_cmds_scan_s;
    let fresh27 = (*ccs).pos;
    (*ccs).pos = ((*ccs).pos).wrapping_add(1);
    let ref mut fresh28 = *((*ccs).cc).offset(fresh27 as isize);
    *fresh28 = (*node).data as *mut mln_conf_cmd_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_arg_num(mut cc: *mut mln_conf_cmd_t) -> mln_u32_t {
    return (*cc).n_args;
}
#[no_mangle]
pub unsafe extern "C" fn mln_conf_dumpi() {
    printf(b"CONFIGURATIONS:\n\0" as *const u8 as *const libc::c_char);
    mln_rbtree_iterate(
        (*g_conf).domain,
        Some(
            mln_conf_dump_conf_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn mln_conf_dump_conf_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut cd: *mut mln_conf_domain_t = (*node).data as *mut mln_conf_domain_t;
    printf(
        b"\tDOMAIN [%s]:\n\0" as *const u8 as *const libc::c_char,
        (*(*cd).domain_name).data as *mut libc::c_char,
    );
    mln_rbtree_iterate(
        (*cd).cmd,
        Some(
            mln_conf_dump_domain_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_conf_dump_domain_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut cc: *mut mln_conf_cmd_t = (*node).data as *mut mln_conf_cmd_t;
    printf(
        b"\t\tCOMMAND [%s]:\n\0" as *const u8 as *const libc::c_char,
        (*(*cc).cmd_name).data as *mut libc::c_char,
    );
    let mut i: mln_s32_t = 0;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*cc).n_args {
        ci = &mut *((*cc).arg_tbl).offset(i as isize) as *mut mln_conf_item_t;
        printf(b"\t\t\t\0" as *const u8 as *const libc::c_char);
        match (*ci).type_0 as libc::c_uint {
            1 => {
                printf(
                    b"STRING [%s]\n\0" as *const u8 as *const libc::c_char,
                    (*(*ci).val.s).data as *mut libc::c_char,
                );
            }
            2 => {
                printf(
                    b"CHAR [%c]\n\0" as *const u8 as *const libc::c_char,
                    (*ci).val.c as libc::c_int,
                );
            }
            3 => {
                printf(
                    b"BOOL [%u]\n\0" as *const u8 as *const libc::c_char,
                    (*ci).val.b as libc::c_int,
                );
            }
            4 => {
                printf(
                    b"INT [%ld]\n\0" as *const u8 as *const libc::c_char,
                    (*ci).val.i,
                );
            }
            5 => {
                printf(
                    b"FLOAT [%f]\n\0" as *const u8 as *const libc::c_char,
                    (*ci).val.f as libc::c_double,
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn conf_hook_chain_add(
    mut head: *mut *mut mln_conf_hook_t,
    mut tail: *mut *mut mln_conf_hook_t,
    mut node: *mut mln_conf_hook_t,
) {
    (*node).next = 0 as *mut mln_conf_hook_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn conf_hook_chain_del(
    mut head: *mut *mut mln_conf_hook_t,
    mut tail: *mut *mut mln_conf_hook_t,
    mut node: *mut mln_conf_hook_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_conf_hook_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_conf_hook_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_conf_hook_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_conf_hook_s;
    (*node).prev = (*node).next;
}
unsafe extern "C" fn mln_conf_reload_master_handler(
    mut ev: *mut mln_event_t,
    mut f_ptr: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: mln_u32_t,
    mut udata_ptr: *mut *mut libc::c_void,
) {}
unsafe extern "C" fn mln_conf_reload_worker_handler(
    mut ev: *mut mln_event_t,
    mut f_ptr: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: mln_u32_t,
    mut udata_ptr: *mut *mut libc::c_void,
) {
    if mln_conf_reload() < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_conf.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"mln_conf_reload_worker_handler\0"))
                .as_ptr(),
            1044 as libc::c_int,
            b"mln_conf_reload() failed.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn run_static_initializers() {
    conf_keywords = [
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"on\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
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
                data: b"off\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: 0 as *mut libc::c_void as mln_u8ptr_t,
                len: (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
    ];
    mln_conf_env = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"MELON_CONF_PATH\0" as *const u8 as *const libc::c_char
                as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
    mln_conf_lex_preprocess_handlers = [
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"define\0" as *const u8 as *const libc::c_char
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
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_define
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
            };
            init
        },
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"include\0" as *const u8 as *const libc::c_char
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
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_include
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
            };
            init
        },
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"if\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 3]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_if
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
            };
            init
        },
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"else\0" as *const u8 as *const libc::c_char
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
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_else
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
            };
            init
        },
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"endif\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 6]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_endif
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
            };
            init
        },
        {
            let mut init = mln_preprocess_handler_t {
                command: {
                    let mut init = mln_string_t {
                        data_ref_pool_ref_0: [0; 4],
                        c2rust_padding: [0; 4],
                        data: b"undef\0" as *const u8 as *const libc::c_char
                            as mln_u8ptr_t,
                        len: (::core::mem::size_of::<[libc::c_char; 6]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    };
                    init.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    init.set_pool(0 as libc::c_int as mln_uauto_t);
                    init.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    init
                },
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_conf_lex_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_conf_lex_lex_preprocess_undef
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_conf_lex_struct_t,
                    ),
                ),
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
