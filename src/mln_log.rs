use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn abort() -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_atfork(
        __prepare: Option::<unsafe extern "C" fn() -> ()>,
        __parent: Option::<unsafe extern "C" fn() -> ()>,
        __child: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn spin_lock(lock: *mut libc::c_void);
    fn spin_unlock(lock: *mut libc::c_void);
    fn mln_string_const_strcmp(
        s1: *mut mln_string_t,
        s2: *mut libc::c_char,
    ) -> libc::c_int;
    fn mln_conf() -> *mut mln_conf_t;
    fn mln_conf_arg_num(cc: *mut mln_conf_cmd_t) -> mln_u32_t;
    fn mln_path() -> *mut libc::c_char;
    fn mln_path_log() -> *mut libc::c_char;
    fn mln_time2utc(tm: time_t, uc: *mut utctime);
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
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub type mln_spin_t = libc::c_long;
pub type mln_u8_t = libc::c_uchar;
pub type mln_s8_t = libc::c_char;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s8ptr_t = *mut libc::c_char;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_sauto_t = libc::c_long;
pub type mln_uauto_t = libc::c_ulong;
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
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_log_t {
    pub thread_lock: mln_spin_t,
    pub fd: libc::c_int,
    #[bitfield(name = "in_daemon", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "init", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "2..=31")]
    pub in_daemon_init_padding: [u8; 4],
    pub level: mln_log_level_t,
    pub dir_path: [libc::c_char; 512],
    pub pid_path: [libc::c_char; 1024],
    pub log_path: [libc::c_char; 1024],
}
pub type mln_logger_t = Option::<
    unsafe extern "C" fn(
        *mut mln_log_t,
        mln_log_level_t,
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_char,
        *mut __va_list_tag,
    ) -> (),
>;
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
use core::ffi::VaList;
use libc::c_void;
//use crate::src::mln_log::__va_list_tag;

// 修改为正确的类型
static mut _logger: mln_logger_t = unsafe {
    Some(
        _mln_sys_log_process
            as unsafe extern "C" fn(
                *mut mln_log_t,
                mln_log_level_t,
                *const libc::c_char,
                *const libc::c_char,
                libc::c_int,
                *mut libc::c_char,
                VaList<'_, '_>,  // 使用 VaList 作为参数
            )
            
    )
};

#[no_mangle]
pub static mut log_err_level: [libc::c_char; 27] = unsafe {
    *::core::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"Log level permission deny.\0")
};
#[no_mangle]
pub static mut log_err_fmt: [libc::c_char; 26] = unsafe {
    *::core::mem::transmute::<
        &[u8; 26],
        &mut [libc::c_char; 26],
    >(b"Log message format error.\0")
};
#[no_mangle]
pub static mut log_path_cmd: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"log_path\0")
};
#[no_mangle]
pub static mut g_log: mln_log_t = mln_log_t {
    thread_lock: 0,
    fd: 0,
    in_daemon_init_padding: [0; 4],
    level: none,
    dir_path: [0; 512],
    pid_path: [0; 1024],
    log_path: [0; 1024],
};
#[inline]
unsafe extern "C" fn mln_file_lock(mut fd: libc::c_int) {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    memset(
        &mut fl as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<flock>() as libc::c_ulong,
    );
    fl.l_type = 1 as libc::c_int as libc::c_short;
    fl.l_start = 0 as libc::c_int as __off_t;
    fl.l_whence = 0 as libc::c_int as libc::c_short;
    fl.l_len = 0 as libc::c_int as __off_t;
    fcntl(fd, 7 as libc::c_int, &mut fl as *mut flock);
}
#[inline]
unsafe extern "C" fn mln_file_unlock(mut fd: libc::c_int) {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    memset(
        &mut fl as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<flock>() as libc::c_ulong,
    );
    fl.l_type = 2 as libc::c_int as libc::c_short;
    fl.l_start = 0 as libc::c_int as __off_t;
    fl.l_whence = 0 as libc::c_int as libc::c_short;
    fl.l_len = 0 as libc::c_int as __off_t;
    fcntl(fd, 7 as libc::c_int, &mut fl as *mut flock);
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_logger_set(mut logger: mln_logger_t) {
    _logger = logger;
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_logger_get() -> mln_logger_t {
    return _logger;
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_init(mut cf: *mut mln_conf_t) -> libc::c_int {
    let mut in_daemon: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut init: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    let mut cc: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    let mut log: *mut mln_log_t = &mut g_log;
    if (*log).init() != 0 {
        return 0 as libc::c_int;
    }
    if cf.is_null() {
        cf = mln_conf();
    }
    if cf.is_null()
        || (*(*cf).domain).nr_node <= 1 as libc::c_int as libc::c_ulong
            && (*(*((*(*(*cf).domain).root).data as *mut mln_conf_domain_t)).cmd).nr_node
                == 0 as libc::c_int as libc::c_ulong
    {
        fprintf(
            stdout,
            b"[WARN] Load configuration failed. Logger won't be initialized.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        cd = ((*cf).search)
            .expect(
                "non-null function pointer",
            )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if cd.is_null() {
            fprintf(
                stderr,
                b"No such domain named 'main'\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        cc = ((*cd).search)
            .expect(
                "non-null function pointer",
            )(cd, b"daemon\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !cc.is_null() {
            ci = ((*cc).search)
                .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
            if ci.is_null() {
                fprintf(
                    stderr,
                    b"Command 'daemon' need a parameter.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if (*ci).type_0 as libc::c_uint != CONF_BOOL as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"Parameter type of command 'daemon' error.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if (*ci).val.b != 0 {
                in_daemon = 1 as libc::c_int as mln_u32_t;
            }
        }
        init = 1 as libc::c_int as mln_u32_t;
    }
    (*log).set_in_daemon(in_daemon);
    (*log).set_init(init);
    (*log).level = none;
    let mut ret: libc::c_int = 0 as libc::c_int;
    (*log).thread_lock = 0 as libc::c_int as mln_spin_t;
    ret = (*log).thread_lock as libc::c_int;
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Init log's thread_lock failed. %s\n\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"mln_log_init\0"))
                .as_ptr(),
            strerror(ret),
        );
        return -(1 as libc::c_int);
    }
    ret = pthread_atfork(
        Some(mln_log_atfork_lock as unsafe extern "C" fn() -> ()),
        Some(mln_log_atfork_unlock as unsafe extern "C" fn() -> ()),
        Some(mln_log_atfork_unlock as unsafe extern "C" fn() -> ()),
    );
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): pthread_atfork failed. %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"mln_log_init\0"))
                .as_ptr(),
            strerror(ret),
        );
        (*log).thread_lock = 0 as libc::c_int as mln_spin_t;
        return -(1 as libc::c_int);
    }
    if mln_log_get_log(log, cf, 1 as libc::c_int) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Get log file failed.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"mln_log_init\0"))
                .as_ptr(),
        );
        mln_log_destroy();
        return -(1 as libc::c_int);
    }
    if mln_log_set_level(log, cf, 1 as libc::c_int) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Set log level failed.\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"mln_log_init\0"))
                .as_ptr(),
        );
        mln_log_destroy();
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_log_get_log(
    mut log: *mut mln_log_t,
    mut cf: *mut mln_conf_t,
    mut is_init: libc::c_int,
) -> libc::c_int {
    if cf.is_null()
        || (*(*cf).domain).nr_node <= 1 as libc::c_int as libc::c_ulong
            && (*(*((*(*(*cf).domain).root).data as *mut mln_conf_domain_t)).cmd).nr_node
                == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    let mut cc: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    let mut path_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_len: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut buf: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    cd = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    cc = ((*cd).search)
        .expect("non-null function pointer")(cd, log_path_cmd.as_mut_ptr());
    if cc.is_null() {
        path_len = snprintf(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%s\0" as *const u8 as *const libc::c_char,
            mln_path_log(),
        ) as mln_u32_t;
        path_str = buf.as_mut_ptr();
    } else {
        if mln_conf_arg_num(cc) != 1 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s(): Invalid command '%s' in domain 'main'.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"mln_log_get_log\0"))
                    .as_ptr(),
                log_path_cmd.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
        ci = ((*cc).search)
            .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
        if (*ci).type_0 as libc::c_uint != CONF_STR as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s(): Invalid command '%s' in domain 'main'.\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"mln_log_get_log\0"))
                    .as_ptr(),
                log_path_cmd.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
        if *((*(*ci).val.s).data).offset(0 as libc::c_int as isize) as libc::c_int
            != '/' as i32
            && *((*(*ci).val.s).data).offset(0 as libc::c_int as isize) as libc::c_int
                != '.' as i32
        {
            path_len = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                mln_path(),
                (*(*ci).val.s).data as *mut libc::c_char,
            ) as mln_u32_t;
            path_str = buf.as_mut_ptr();
        } else {
            path_len = (if (*(*ci).val.s).len
                > (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            {
                (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            } else {
                (*(*ci).val.s).len
            }) as mln_u32_t;
            path_str = (*(*ci).val.s).data as *mut libc::c_char;
        }
    }
    p = &mut *path_str
        .offset(path_len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut libc::c_char;
    while p >= path_str && *p as libc::c_int != '/' as i32 {
        p = p.offset(-1);
        p;
    }
    memcpy(
        ((*log).dir_path).as_mut_ptr() as *mut libc::c_void,
        path_str as *const libc::c_void,
        p.offset_from(path_str) as libc::c_long as libc::c_ulong,
    );
    (*log)
        .dir_path[p.offset_from(path_str) as libc::c_long
        as usize] = 0 as libc::c_int as libc::c_char;
    if mkdir(
        ((*log).dir_path).as_mut_ptr(),
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    ) < 0 as libc::c_int
    {
        if *__errno_location() != 17 as libc::c_int {
            fprintf(
                stderr,
                b"mkdir '%s' failed. %s\n\0" as *const u8 as *const libc::c_char,
                ((*log).dir_path).as_mut_ptr(),
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    fd = open(
        path_str,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): open '%s' failed. %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mln_log_get_log\0"))
                .as_ptr(),
            path_str,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if is_init == 0 && (*log).fd > 0 as libc::c_int && (*log).fd != 0 as libc::c_int
        && (*log).fd != 1 as libc::c_int && (*log).fd != 2 as libc::c_int
    {
        close((*log).fd);
    }
    (*log).fd = fd;
    memcpy(
        ((*log).log_path).as_mut_ptr() as *mut libc::c_void,
        path_str as *const libc::c_void,
        path_len as libc::c_ulong,
    );
    (*log).log_path[path_len as usize] = 0 as libc::c_int as libc::c_char;
    if is_init != 0 {
        memset(
            ((*log).pid_path).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            1024 as libc::c_int as libc::c_ulong,
        );
        snprintf(
            ((*log).pid_path).as_mut_ptr(),
            (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            b"%s/melon.pid\0" as *const u8 as *const libc::c_char,
            ((*log).dir_path).as_mut_ptr(),
        );
        fd = open(
            ((*log).pid_path).as_mut_ptr(),
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if fd < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s(): open pid file failed. %s\n\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"mln_log_get_log\0"))
                    .as_ptr(),
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        let mut pid_str: [libc::c_char; 64] = [
            0 as libc::c_int as libc::c_char,
        ];
        snprintf(
            pid_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%lu\0" as *const u8 as *const libc::c_char,
            getpid() as libc::c_ulong,
        );
        mln_file_lock(fd);
        let mut rc: libc::c_int = write(
            fd,
            pid_str.as_mut_ptr() as *const libc::c_void,
            strlen(pid_str.as_mut_ptr()),
        ) as libc::c_int;
        if rc <= 0 as libc::c_int {
            rc = 1 as libc::c_int;
        }
        mln_file_unlock(fd);
        close(fd);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_log_atfork_lock() {
    spin_lock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
}
unsafe extern "C" fn mln_log_atfork_unlock() {
    spin_unlock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_destroy() {
    let mut log: *mut mln_log_t = &mut g_log;
    if (*log).fd > 0 as libc::c_int && (*log).fd != 0 as libc::c_int
        && (*log).fd != 1 as libc::c_int && (*log).fd != 2 as libc::c_int
    {
        close((*log).fd);
    }
    (*log).thread_lock = 0 as libc::c_int as mln_spin_t;
}
unsafe extern "C" fn mln_log_set_level(
    mut log: *mut mln_log_t,
    mut cf: *mut mln_conf_t,
    mut is_init: libc::c_int,
) -> libc::c_int {
    if cf.is_null()
        || (*(*cf).domain).nr_node <= 1 as libc::c_int as libc::c_ulong
            && (*(*((*(*(*cf).domain).root).data as *mut mln_conf_domain_t)).cmd).nr_node
                == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cd.is_null() {
        if is_init != 0 {
            fprintf(
                stderr,
                b"No 'main' domain.\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            _mln_sys_log(
                error,
                b"src/mln_log.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"mln_log_set_level\0"))
                    .as_ptr(),
                322 as libc::c_int,
                b"No 'main' domain.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        abort();
    }
    let mut cc: *mut mln_conf_cmd_t = ((*cd).search)
        .expect(
            "non-null function pointer",
        )(cd, b"log_level\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cc.is_null() {
        return 0 as libc::c_int;
    }
    let mut ci: *mut mln_conf_item_t = ((*cc).search)
        .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
    if ci.is_null() {
        if is_init != 0 {
            fprintf(
                stderr,
                b"Command 'log_level' need a parameter.\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            _mln_sys_log(
                error,
                b"src/mln_log.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"mln_log_set_level\0"))
                    .as_ptr(),
                332 as libc::c_int,
                b"Command 'log_level' need a parameter.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*ci).type_0 as libc::c_uint != CONF_STR as libc::c_int as libc::c_uint {
        if is_init != 0 {
            fprintf(
                stderr,
                b"Parameter type of command 'log_level' error.\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            _mln_sys_log(
                error,
                b"src/mln_log.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"mln_log_set_level\0"))
                    .as_ptr(),
                339 as libc::c_int,
                b"Parameter type of command 'log_level' error.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if mln_string_const_strcmp(
        (*ci).val.s,
        b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        (*log).level = none;
    } else if mln_string_const_strcmp(
        (*ci).val.s,
        b"report\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        (*log).level = report;
    } else if mln_string_const_strcmp(
        (*ci).val.s,
        b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        (*log).level = debug;
    } else if mln_string_const_strcmp(
        (*ci).val.s,
        b"warn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        (*log).level = warn;
    } else if mln_string_const_strcmp(
        (*ci).val.s,
        b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        (*log).level = error;
    } else {
        if is_init != 0 {
            fprintf(
                stderr,
                b"Parameter value of command [log_level] error.\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            _mln_sys_log(
                error,
                b"src/mln_log.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"mln_log_set_level\0"))
                    .as_ptr(),
                356 as libc::c_int,
                b"Parameter value of command [log_level] error.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_reload(mut data: *mut libc::c_void) -> libc::c_int {
    spin_lock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
    mln_log_get_log(&mut g_log, mln_conf(), 0 as libc::c_int);
    mln_file_lock(g_log.fd);
    let mut ret: libc::c_int = mln_log_set_level(
        &mut g_log,
        mln_conf(),
        0 as libc::c_int,
    );
    mln_file_unlock(g_log.fd);
    spin_unlock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _mln_sys_log(
    mut level: mln_log_level_t,
    mut file: *const libc::c_char,
    mut func: *const libc::c_char,
    mut line: libc::c_int,
    mut msg: *mut libc::c_char,
    mut args: ...
) {
    spin_lock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
    mln_file_lock(g_log.fd);
    let va_list_ptr: *mut __va_list_tag = arg.as_va_list() as *mut __va_list_tag;
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    if _logger.is_some() {
        _logger
            .expect("non-null function pointer")(
                &mut g_log,
                level,
                file,
                func,
                line,
                msg,
                va_list_ptr,);
    }
    mln_file_unlock(g_log.fd);
    spin_unlock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_log_write(
    mut log: *mut mln_log_t,
    mut buf: *mut libc::c_void,
    mut size: mln_size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = write((*log).fd, buf, size) as libc::c_int;
    if (*log).init() as libc::c_int != 0 && (*log).in_daemon() == 0 {
        ret = write(2 as libc::c_int, buf, size) as libc::c_int;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn mln_log_level_write(
    mut log: *mut mln_log_t,
    mut level: mln_log_level_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    match level as libc::c_uint {
        1 => {
            ret = write(
                (*log).fd,
                b"REPORT: \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                8 as libc::c_int as size_t,
            ) as libc::c_int;
            if (*log).init() as libc::c_int != 0 && (*log).in_daemon() == 0 {
                ret = write(
                    2 as libc::c_int,
                    b"\x1B[34mREPORT\x1B[0m: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_void,
                    17 as libc::c_int as size_t,
                ) as libc::c_int;
            }
        }
        2 => {
            ret = write(
                (*log).fd,
                b"DEBUG: \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                7 as libc::c_int as size_t,
            ) as libc::c_int;
            if (*log).init() as libc::c_int != 0 && (*log).in_daemon() == 0 {
                ret = write(
                    2 as libc::c_int,
                    b"\x1B[32mDEBUG\x1B[0m: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_void,
                    16 as libc::c_int as size_t,
                ) as libc::c_int;
            }
        }
        3 => {
            ret = write(
                (*log).fd,
                b"WARN: \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                6 as libc::c_int as size_t,
            ) as libc::c_int;
            if (*log).init() as libc::c_int != 0 && (*log).in_daemon() == 0 {
                ret = write(
                    2 as libc::c_int,
                    b"\x1B[33mWARN\x1B[0m: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_void,
                    15 as libc::c_int as size_t,
                ) as libc::c_int;
            }
        }
        4 => {
            ret = write(
                (*log).fd,
                b"ERROR: \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                7 as libc::c_int as size_t,
            ) as libc::c_int;
            if (*log).init() as libc::c_int != 0 && (*log).in_daemon() == 0 {
                ret = write(
                    2 as libc::c_int,
                    b"\x1B[31mERROR\x1B[0m: \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_void,
                    16 as libc::c_int as size_t,
                ) as libc::c_int;
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_writen(
    mut buf: *mut libc::c_void,
    mut size: mln_size_t,
) -> libc::c_int {
    spin_lock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
    mln_file_lock(g_log.fd);
    let mut n: libc::c_int = mln_log_write(&mut g_log, buf, size);
    mln_file_unlock(g_log.fd);
    spin_unlock(&mut g_log.thread_lock as *mut mln_spin_t as *mut libc::c_void);
    return n;
}
unsafe extern "C" fn _mln_sys_log_process(
    mut log: *mut mln_log_t,
    mut level: mln_log_level_t,
    mut file: *const libc::c_char,
    mut func: *const libc::c_char,
    mut line: libc::c_int,
    mut msg: *mut libc::c_char,
    mut arg: ::core::ffi::VaList,
) {
    if (level as libc::c_uint) < (*log).level as libc::c_uint {
        return;
    }
    let mut n: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut uc: utctime = utctime {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        week: 0,
    };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    mln_time2utc(tv.tv_sec, &mut uc);
    let mut line_str: [libc::c_char; 256] = [
        0 as libc::c_int as libc::c_char,
    ];
    if level as libc::c_uint > none as libc::c_int as libc::c_uint {
        n = snprintf(
            line_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%02ld/%02ld/%ld %02ld:%02ld:%02ld UTC \0" as *const u8
                as *const libc::c_char,
            uc.month,
            uc.day,
            uc.year,
            uc.hour,
            uc.minute,
            uc.second,
        );
        mln_log_write(log, line_str.as_mut_ptr() as *mut libc::c_void, n as mln_size_t);
    }
    mln_log_level_write(log, level);
    if level as libc::c_uint >= debug as libc::c_int as libc::c_uint {
        memset(
            line_str.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        mln_log_write(log, file as *mut libc::c_void, strlen(file));
        mln_log_write(
            log,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            1 as libc::c_int as mln_size_t,
        );
        mln_log_write(log, func as *mut libc::c_void, strlen(func));
        mln_log_write(
            log,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            1 as libc::c_int as mln_size_t,
        );
        n = snprintf(
            line_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%d\0" as *const u8 as *const libc::c_char,
            line,
        );
        mln_log_write(log, line_str.as_mut_ptr() as *mut libc::c_void, n as mln_size_t);
        mln_log_write(
            log,
            b": \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            2 as libc::c_int as mln_size_t,
        );
    }
    if level as libc::c_uint > none as libc::c_int as libc::c_uint {
        memset(
            line_str.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        n = snprintf(
            line_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"PID:%d \0" as *const u8 as *const libc::c_char,
            getpid(),
        );
        mln_log_write(log, line_str.as_mut_ptr() as *mut libc::c_void, n as mln_size_t);
    }
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = msg;
    while *msg as libc::c_int != 0 as libc::c_int {
        if *msg as libc::c_int != '%' as i32 {
            cnt += 1;
            cnt;
            msg = msg.offset(1);
            msg;
        } else {
            mln_log_write(log, p as *mut libc::c_void, cnt as mln_size_t);
            cnt = 0 as libc::c_int;
            msg = msg.offset(1);
            msg;
            p = msg.offset(1 as libc::c_int as isize);
            match *msg as libc::c_int {
                115 => {
                    let mut s: *mut libc::c_char = arg.arg::<*mut libc::c_char>();
                    mln_log_write(log, s as *mut libc::c_void, strlen(s));
                }
                83 => {
                    let mut s_0: *mut mln_string_t = arg.arg::<*mut mln_string_t>();
                    mln_log_write(log, (*s_0).data as *mut libc::c_void, (*s_0).len);
                }
                108 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num: mln_sauto_t = arg.arg::<libc::c_long>();
                    let mut n_0: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%ld\0" as *const u8 as *const libc::c_char,
                        num,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_0 as mln_size_t,
                    );
                }
                100 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_0: libc::c_int = arg.arg::<libc::c_int>();
                    let mut n_1: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        num_0,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_1 as mln_size_t,
                    );
                }
                99 => {
                    let mut ch: libc::c_int = arg.arg::<libc::c_int>();
                    mln_log_write(
                        log,
                        &mut ch as *mut libc::c_int as *mut libc::c_void,
                        1 as libc::c_int as mln_size_t,
                    );
                }
                102 => {
                    let mut f: libc::c_double = arg.arg::<libc::c_double>();
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut n_2: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%f\0" as *const u8 as *const libc::c_char,
                        f,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_2 as mln_size_t,
                    );
                }
                120 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_1: libc::c_int = arg.arg::<libc::c_int>();
                    let mut n_3: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%x\0" as *const u8 as *const libc::c_char,
                        num_1,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_3 as mln_size_t,
                    );
                }
                88 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_2: libc::c_long = arg.arg::<libc::c_long>();
                    let mut n_4: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%lx\0" as *const u8 as *const libc::c_char,
                        num_2,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_4 as mln_size_t,
                    );
                }
                117 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_3: libc::c_uint = arg.arg::<libc::c_uint>();
                    let mut n_5: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%u\0" as *const u8 as *const libc::c_char,
                        num_3,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_5 as mln_size_t,
                    );
                }
                85 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_4: libc::c_ulong = arg.arg::<libc::c_ulong>();
                    let mut n_6: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%lu\0" as *const u8 as *const libc::c_char,
                        num_4,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_6 as mln_size_t,
                    );
                }
                105 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_5: libc::c_long = arg.arg::<libc::c_long>();
                    let mut n_7: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%ld\0" as *const u8 as *const libc::c_char,
                        num_5,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_7 as mln_size_t,
                    );
                }
                73 => {
                    memset(
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    let mut num_6: libc::c_ulong = arg.arg::<libc::c_ulong>();
                    let mut n_8: libc::c_int = snprintf(
                        line_str.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%lu\0" as *const u8 as *const libc::c_char,
                        num_6,
                    );
                    mln_log_write(
                        log,
                        line_str.as_mut_ptr() as *mut libc::c_void,
                        n_8 as mln_size_t,
                    );
                }
                _ => {
                    mln_log_write(
                        log,
                        log_err_fmt.as_mut_ptr() as *mut libc::c_void,
                        (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    mln_log_write(
                        log,
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                        1 as libc::c_int as mln_size_t,
                    );
                    return;
                }
            }
            msg = msg.offset(1);
            msg;
        }
    }
    if cnt != 0 {
        mln_log_write(log, p as *mut libc::c_void, cnt as mln_size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_fd() -> libc::c_int {
    return g_log.fd;
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_dir_path() -> *mut libc::c_char {
    return (g_log.dir_path).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_logfile_path() -> *mut libc::c_char {
    return (g_log.log_path).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn mln_log_pid_path() -> *mut libc::c_char {
    return (g_log.pid_path).as_mut_ptr();
}
unsafe extern "C" fn run_static_initializers() {
    g_log = {
        let mut init = mln_log_t {
            in_daemon_init_padding: [0; 4],
            thread_lock: 0 as libc::c_int as mln_spin_t,
            fd: 2 as libc::c_int,
            level: none,
            dir_path: [
                0 as libc::c_int as libc::c_char,
            ],
            pid_path: [
                0 as libc::c_int as libc::c_char,
            ],
            log_path: [
                0 as libc::c_int as libc::c_char,
            ],
        };
        init.set_in_daemon(0 as libc::c_int as mln_u32_t);
        init.set_init(0 as libc::c_int as mln_u32_t);
        init.set_padding(0 as libc::c_int as mln_u32_t);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
