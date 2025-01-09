use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn mln_lang_ctx_pipe_recv_handler_set(
        ctx: *mut mln_lang_ctx_t,
        recv_handler: mln_lang_ctx_pipe_recv_cb_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn mln_event_fd_set(
        event: *mut mln_event_t,
        fd: libc::c_int,
        flag: mln_u32_t,
        timeout_ms: libc::c_int,
        data: *mut libc::c_void,
        fd_handler: ev_fd_handler,
    ) -> libc::c_int;
    fn mln_event_timer_set(
        event: *mut mln_event_t,
        msec: mln_u32_t,
        data: *mut libc::c_void,
        tm_handler: ev_tm_handler,
    ) -> *mut mln_event_timer_t;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn mln_conf() -> *mut mln_conf_t;
    fn mln_conf_arg_num(cc: *mut mln_conf_cmd_t) -> mln_u32_t;
    fn mln_lang_new(
        ev: *mut mln_event_t,
        signal: mln_lang_run_ctl_t,
        clear: mln_lang_run_ctl_t,
    ) -> *mut mln_lang_t;
    fn mln_lang_free(lang: *mut mln_lang_t);
    fn mln_lang_job_new(
        lang: *mut mln_lang_t,
        alias: *mut mln_string_t,
        type_0: mln_u32_t,
        data: *mut mln_string_t,
        udata: *mut libc::c_void,
        handler: mln_lang_return_handler,
    ) -> *mut mln_lang_ctx_t;
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type mln_u8_t = libc::c_uchar;
pub type mln_s8_t = libc::c_char;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s64_t = libc::c_long;
pub type mln_s8ptr_t = *mut libc::c_char;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
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
pub type mln_event_timer_t = mln_fheap_node_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_stm_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_stm_type_t,
    pub data: C2RustUnnamed_1,
    pub next: *mut mln_lang_stm_s,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub block: *mut mln_lang_block_t,
    pub func: *mut mln_lang_funcdef_t,
    pub setdef: *mut mln_lang_set_t,
    pub sw: *mut mln_lang_switch_t,
    pub w: *mut mln_lang_while_t,
    pub f: *mut mln_lang_for_t,
    pub pos: *mut mln_string_t,
}
pub type mln_lang_for_t = mln_lang_for_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_for_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub init_exp: *mut mln_lang_exp_t,
    pub condition: *mut mln_lang_exp_t,
    pub mod_exp: *mut mln_lang_exp_t,
    pub blockstm: *mut mln_lang_block_t,
}
pub type mln_lang_block_t = mln_lang_block_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_block_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_block_type_t,
    pub data: C2RustUnnamed_2,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub exp: *mut mln_lang_exp_t,
    pub stm: *mut mln_lang_stm_t,
    pub pos: *mut mln_string_t,
    pub i: *mut mln_lang_if_t,
}
pub type mln_lang_if_t = mln_lang_if_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_if_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub condition: *mut mln_lang_exp_t,
    pub blockstm: *mut mln_lang_block_t,
    pub elsestm: *mut mln_lang_block_t,
}
pub type mln_lang_exp_t = mln_lang_exp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_exp_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub assign: *mut mln_lang_assign_t,
    pub next: *mut mln_lang_exp_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_assign_t = mln_lang_assign_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_assign_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_logiclow_t,
    pub op: mln_lang_assign_op_t,
    pub right: *mut mln_lang_assign_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_assign_op_t = mln_lang_assign_op_e;
pub type mln_lang_assign_op_e = libc::c_uint;
pub const M_ASSIGN_MODEQ: mln_lang_assign_op_e = 11;
pub const M_ASSIGN_XOREQ: mln_lang_assign_op_e = 10;
pub const M_ASSIGN_ANDEQ: mln_lang_assign_op_e = 9;
pub const M_ASSIGN_OREQ: mln_lang_assign_op_e = 8;
pub const M_ASSIGN_DIVEQ: mln_lang_assign_op_e = 7;
pub const M_ASSIGN_MULEQ: mln_lang_assign_op_e = 6;
pub const M_ASSIGN_RMOVEQ: mln_lang_assign_op_e = 5;
pub const M_ASSIGN_LMOVEQ: mln_lang_assign_op_e = 4;
pub const M_ASSIGN_SUBEQ: mln_lang_assign_op_e = 3;
pub const M_ASSIGN_PLUSEQ: mln_lang_assign_op_e = 2;
pub const M_ASSIGN_EQUAL: mln_lang_assign_op_e = 1;
pub const M_ASSIGN_NONE: mln_lang_assign_op_e = 0;
pub type mln_lang_logiclow_t = mln_lang_logiclow_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_logiclow_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_logichigh_t,
    pub op: mln_lang_logiclow_op_t,
    pub right: *mut mln_lang_logiclow_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_logiclow_op_t = mln_lang_logiclow_op_e;
pub type mln_lang_logiclow_op_e = libc::c_uint;
pub const M_LOGICLOW_AND: mln_lang_logiclow_op_e = 2;
pub const M_LOGICLOW_OR: mln_lang_logiclow_op_e = 1;
pub const M_LOGICLOW_NONE: mln_lang_logiclow_op_e = 0;
pub type mln_lang_logichigh_t = mln_lang_logichigh_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_logichigh_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_relativelow_t,
    pub op: mln_lang_logichigh_op_t,
    pub right: *mut mln_lang_logichigh_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_logichigh_op_t = mln_lang_logichigh_op_e;
pub type mln_lang_logichigh_op_e = libc::c_uint;
pub const M_LOGICHIGH_XOR: mln_lang_logichigh_op_e = 3;
pub const M_LOGICHIGH_AND: mln_lang_logichigh_op_e = 2;
pub const M_LOGICHIGH_OR: mln_lang_logichigh_op_e = 1;
pub const M_LOGICHIGH_NONE: mln_lang_logichigh_op_e = 0;
pub type mln_lang_relativelow_t = mln_lang_relativelow_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_relativelow_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_relativehigh_t,
    pub op: mln_lang_relativelow_op_t,
    pub right: *mut mln_lang_relativelow_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_relativelow_op_t = mln_lang_relativelow_op_e;
pub type mln_lang_relativelow_op_e = libc::c_uint;
pub const M_RELATIVELOW_NEQUAL: mln_lang_relativelow_op_e = 2;
pub const M_RELATIVELOW_EQUAL: mln_lang_relativelow_op_e = 1;
pub const M_RELATIVELOW_NONE: mln_lang_relativelow_op_e = 0;
pub type mln_lang_relativehigh_t = mln_lang_relativehigh_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_relativehigh_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_move_t,
    pub op: mln_lang_relativehigh_op_t,
    pub right: *mut mln_lang_relativehigh_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_relativehigh_op_t = mln_lang_relativehigh_op_e;
pub type mln_lang_relativehigh_op_e = libc::c_uint;
pub const M_RELATIVEHIGH_GREATEREQ: mln_lang_relativehigh_op_e = 4;
pub const M_RELATIVEHIGH_GREATER: mln_lang_relativehigh_op_e = 3;
pub const M_RELATIVEHIGH_LESSEQ: mln_lang_relativehigh_op_e = 2;
pub const M_RELATIVEHIGH_LESS: mln_lang_relativehigh_op_e = 1;
pub const M_RELATIVEHIGH_NONE: mln_lang_relativehigh_op_e = 0;
pub type mln_lang_move_t = mln_lang_move_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_move_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_addsub_t,
    pub op: mln_lang_move_op_t,
    pub right: *mut mln_lang_move_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_move_op_t = mln_lang_move_op_e;
pub type mln_lang_move_op_e = libc::c_uint;
pub const M_MOVE_RMOVE: mln_lang_move_op_e = 2;
pub const M_MOVE_LMOVE: mln_lang_move_op_e = 1;
pub const M_MOVE_NONE: mln_lang_move_op_e = 0;
pub type mln_lang_addsub_t = mln_lang_addsub_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_addsub_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_muldiv_t,
    pub op: mln_lang_addsub_op_t,
    pub right: *mut mln_lang_addsub_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_addsub_op_t = mln_lang_addsub_op_e;
pub type mln_lang_addsub_op_e = libc::c_uint;
pub const M_ADDSUB_SUB: mln_lang_addsub_op_e = 2;
pub const M_ADDSUB_PLUS: mln_lang_addsub_op_e = 1;
pub const M_ADDSUB_NONE: mln_lang_addsub_op_e = 0;
pub type mln_lang_muldiv_t = mln_lang_muldiv_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_muldiv_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_not_t,
    pub op: mln_lang_muldiv_op_t,
    pub right: *mut mln_lang_muldiv_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_muldiv_op_t = mln_lang_muldiv_op_e;
pub type mln_lang_muldiv_op_e = libc::c_uint;
pub const M_MULDIV_MOD: mln_lang_muldiv_op_e = 3;
pub const M_MULDIV_DIV: mln_lang_muldiv_op_e = 2;
pub const M_MULDIV_MUL: mln_lang_muldiv_op_e = 1;
pub const M_MULDIV_NONE: mln_lang_muldiv_op_e = 0;
pub type mln_lang_not_t = mln_lang_not_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_not_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub op: mln_lang_not_op_t,
    pub right: C2RustUnnamed_3,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub not: *mut mln_lang_not_t,
    pub suffix: *mut mln_lang_suffix_t,
}
pub type mln_lang_suffix_t = mln_lang_suffix_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_suffix_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_locate_t,
    pub op: mln_lang_suffix_op_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
pub type mln_lang_suffix_op_t = mln_lang_suffix_op_e;
pub type mln_lang_suffix_op_e = libc::c_uint;
pub const M_SUFFIX_DEC: mln_lang_suffix_op_e = 2;
pub const M_SUFFIX_INC: mln_lang_suffix_op_e = 1;
pub const M_SUFFIX_NONE: mln_lang_suffix_op_e = 0;
pub type mln_lang_locate_t = mln_lang_locate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_locate_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub left: *mut mln_lang_spec_t,
    pub op: mln_lang_locate_op_t,
    pub right: C2RustUnnamed_4,
    pub next: *mut mln_lang_locate_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub exp: *mut mln_lang_exp_t,
    pub id: *mut mln_string_t,
}
pub type mln_lang_locate_op_t = mln_lang_locate_op_e;
pub type mln_lang_locate_op_e = libc::c_uint;
pub const M_LOCATE_FUNC: mln_lang_locate_op_e = 3;
pub const M_LOCATE_PROPERTY: mln_lang_locate_op_e = 2;
pub const M_LOCATE_INDEX: mln_lang_locate_op_e = 1;
pub const M_LOCATE_NONE: mln_lang_locate_op_e = 0;
pub type mln_lang_spec_t = mln_lang_spec_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_spec_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub op: mln_lang_spec_op_t,
    pub data: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub exp: *mut mln_lang_exp_t,
    pub factor: *mut mln_lang_factor_t,
    pub spec: *mut mln_lang_spec_t,
    pub set_name: *mut mln_string_t,
}
pub type mln_lang_factor_t = mln_lang_factor_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_factor_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_factor_type_t,
    pub data: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub b: mln_u8_t,
    pub ptr: *mut mln_u8ptr_t,
    pub s_id: *mut mln_string_t,
    pub i: mln_s64_t,
    pub f: libc::c_double,
    pub array: *mut mln_lang_elemlist_t,
}
pub type mln_lang_elemlist_t = mln_lang_elemlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_elemlist_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub key: *mut mln_lang_assign_t,
    pub val: *mut mln_lang_assign_t,
    pub next: *mut mln_lang_elemlist_t,
}
pub type mln_lang_factor_type_t = libc::c_uint;
pub const M_FACTOR_NIL: mln_lang_factor_type_t = 6;
pub const M_FACTOR_ID: mln_lang_factor_type_t = 5;
pub const M_FACTOR_ARRAY: mln_lang_factor_type_t = 4;
pub const M_FACTOR_REAL: mln_lang_factor_type_t = 3;
pub const M_FACTOR_INT: mln_lang_factor_type_t = 2;
pub const M_FACTOR_STRING: mln_lang_factor_type_t = 1;
pub const M_FACTOR_BOOL: mln_lang_factor_type_t = 0;
pub type mln_lang_spec_op_t = mln_lang_spec_op_e;
pub type mln_lang_spec_op_e = libc::c_uint;
pub const M_SPEC_FACTOR: mln_lang_spec_op_e = 7;
pub const M_SPEC_PARENTH: mln_lang_spec_op_e = 6;
pub const M_SPEC_NEW: mln_lang_spec_op_e = 5;
pub const M_SPEC_DEC: mln_lang_spec_op_e = 4;
pub const M_SPEC_INC: mln_lang_spec_op_e = 3;
pub const M_SPEC_REFER: mln_lang_spec_op_e = 2;
pub const M_SPEC_REVERSE: mln_lang_spec_op_e = 1;
pub const M_SPEC_NEGATIVE: mln_lang_spec_op_e = 0;
pub type mln_lang_not_op_t = mln_lang_not_op_e;
pub type mln_lang_not_op_e = libc::c_uint;
pub const M_NOT_NOT: mln_lang_not_op_e = 1;
pub const M_NOT_NONE: mln_lang_not_op_e = 0;
pub type mln_lang_stm_t = mln_lang_stm_s;
pub type mln_lang_block_type_t = libc::c_uint;
pub const M_BLOCK_IF: mln_lang_block_type_t = 6;
pub const M_BLOCK_GOTO: mln_lang_block_type_t = 5;
pub const M_BLOCK_RETURN: mln_lang_block_type_t = 4;
pub const M_BLOCK_BREAK: mln_lang_block_type_t = 3;
pub const M_BLOCK_CONTINUE: mln_lang_block_type_t = 2;
pub const M_BLOCK_STM: mln_lang_block_type_t = 1;
pub const M_BLOCK_EXP: mln_lang_block_type_t = 0;
pub type mln_lang_while_t = mln_lang_while_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_while_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub condition: *mut mln_lang_exp_t,
    pub blockstm: *mut mln_lang_block_t,
}
pub type mln_lang_switch_t = mln_lang_switch_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_switch_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub condition: *mut mln_lang_exp_t,
    pub switchstm: *mut mln_lang_switchstm_t,
}
pub type mln_lang_switchstm_t = mln_lang_switchstm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_switchstm_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub factor: *mut mln_lang_factor_t,
    pub stm: *mut mln_lang_stm_t,
    pub next: *mut mln_lang_switchstm_t,
}
pub type mln_lang_set_t = mln_lang_set_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_set_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub name: *mut mln_string_t,
    pub stm: *mut mln_lang_setstm_t,
}
pub type mln_lang_setstm_t = mln_lang_setstm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_setstm_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_setstm_type_t,
    pub data: C2RustUnnamed_7,
    pub next: *mut mln_lang_setstm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub var: *mut mln_string_t,
    pub func: *mut mln_lang_funcdef_t,
}
pub type mln_lang_funcdef_t = mln_lang_funcdef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_funcdef_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub name: *mut mln_string_t,
    pub args: *mut mln_lang_exp_t,
    pub stm: *mut mln_lang_stm_t,
    pub closure: *mut mln_lang_exp_t,
}
pub type mln_lang_setstm_type_t = libc::c_uint;
pub const M_SETSTM_FUNC: mln_lang_setstm_type_t = 1;
pub const M_SETSTM_VAR: mln_lang_setstm_type_t = 0;
pub type mln_lang_stm_type_t = libc::c_uint;
pub const M_STM_FOR: mln_lang_stm_type_t = 6;
pub const M_STM_WHILE: mln_lang_stm_type_t = 5;
pub const M_STM_SWITCH: mln_lang_stm_type_t = 4;
pub const M_STM_LABEL: mln_lang_stm_type_t = 3;
pub const M_STM_SET: mln_lang_stm_type_t = 2;
pub const M_STM_FUNC: mln_lang_stm_type_t = 1;
pub const M_STM_BLOCK: mln_lang_stm_type_t = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_gc_s {
    pub pool: *mut mln_alloc_t,
    pub item_head: *mut mln_gc_item_t,
    pub item_tail: *mut mln_gc_item_t,
    pub proc_head: *mut mln_gc_item_t,
    pub proc_tail: *mut mln_gc_item_t,
    pub iter: *mut mln_gc_item_t,
    pub item_getter: gc_item_getter,
    pub item_setter: gc_item_setter,
    pub item_freer: gc_item_freer,
    pub member_setter: gc_member_setter,
    pub move_handler: gc_move_handler,
    pub root_setter: gc_root_setter,
    pub clean_searcher: gc_clean_searcher,
    pub free_handler: gc_free_handler,
    #[bitfield(name = "del", ty = "mln_u32_t", bits = "0..=0")]
    pub del: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type gc_free_handler = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type gc_clean_searcher = Option::<
    unsafe extern "C" fn(*mut mln_gc_t, *mut libc::c_void) -> (),
>;
pub type mln_gc_t = mln_gc_s;
pub type gc_root_setter = Option::<
    unsafe extern "C" fn(*mut mln_gc_t, *mut libc::c_void) -> (),
>;
pub type gc_move_handler = Option::<
    unsafe extern "C" fn(*mut mln_gc_t, *mut libc::c_void) -> (),
>;
pub type gc_member_setter = Option::<
    unsafe extern "C" fn(*mut mln_gc_t, *mut libc::c_void) -> (),
>;
pub type gc_item_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type gc_item_setter = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type gc_item_getter = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type mln_gc_item_t = mln_gc_item_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_gc_item_s {
    pub gc: *mut mln_gc_t,
    pub data: *mut libc::c_void,
    pub prev: *mut mln_gc_item_s,
    pub next: *mut mln_gc_item_s,
    pub proc_prev: *mut mln_gc_item_s,
    pub proc_next: *mut mln_gc_item_s,
    #[bitfield(name = "suspected", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "credit", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "inc", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "visited", ty = "mln_u32_t", bits = "3..=3")]
    pub suspected_credit_inc_visited: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_funccall_val_s {
    pub name: *mut mln_string_t,
    pub prototype: *mut mln_lang_func_detail_t,
    pub object: *mut mln_lang_val_t,
    pub args: mln_array_t,
}
pub type mln_lang_val_t = mln_lang_val_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lang_val_s {
    pub prev: *mut mln_lang_val_s,
    pub next: *mut mln_lang_val_s,
    pub data: C2RustUnnamed_11,
    pub type_0: mln_s32_t,
    pub ref_0: mln_u32_t,
    pub udata: *mut mln_lang_val_t,
    pub func: *mut mln_lang_func_detail_t,
    #[bitfield(name = "not_modify", ty = "mln_u32_t", bits = "0..=0")]
    pub not_modify: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mln_lang_func_detail_t = mln_lang_func_detail_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_func_detail_s {
    pub exp: *mut mln_lang_exp_t,
    pub args: mln_array_t,
    pub closure: mln_array_t,
    pub type_0: mln_lang_func_type_t,
    pub data: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub process: mln_lang_internal,
    pub stm: *mut mln_lang_stm_t,
}
pub type mln_lang_internal = Option::<
    unsafe extern "C" fn(*mut mln_lang_ctx_t) -> *mut mln_lang_var_t,
>;
pub type mln_lang_ctx_t = mln_lang_ctx_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lang_ctx_s {
    pub lang: *mut mln_lang_t,
    pub pool: *mut mln_alloc_t,
    pub fset: *mut mln_fileset_t,
    pub data: *mut libc::c_void,
    pub stm: *mut mln_lang_stm_t,
    pub run_stack: [mln_lang_stack_node_t; 1025],
    pub run_stack_top: *mut mln_lang_stack_node_t,
    pub scopes: [mln_lang_scope_t; 1025],
    pub scope_top: *mut mln_lang_scope_t,
    pub ref_0: mln_u64_t,
    pub filename: *mut mln_string_t,
    pub resource_set: *mut mln_rbtree_t,
    pub ret_var: *mut mln_lang_var_t,
    pub return_handler: mln_lang_return_handler,
    pub cache: *mut mln_lang_ast_cache_t,
    pub gc: *mut mln_gc_t,
    pub symbols: *mut mln_lang_hash_t,
    pub prev: *mut mln_lang_ctx_s,
    pub next: *mut mln_lang_ctx_s,
    pub sym_head: *mut mln_lang_symbol_node_t,
    pub sym_tail: *mut mln_lang_symbol_node_t,
    pub owner: pthread_t,
    pub alias: *mut mln_string_t,
    #[bitfield(name = "sym_count", ty = "mln_u32_t", bits = "0..=15")]
    #[bitfield(name = "ret_flag", ty = "mln_u32_t", bits = "16..=16")]
    #[bitfield(name = "op_array_flag", ty = "mln_u32_t", bits = "17..=17")]
    #[bitfield(name = "op_bool_flag", ty = "mln_u32_t", bits = "18..=18")]
    #[bitfield(name = "op_func_flag", ty = "mln_u32_t", bits = "19..=19")]
    #[bitfield(name = "op_int_flag", ty = "mln_u32_t", bits = "20..=20")]
    #[bitfield(name = "op_nil_flag", ty = "mln_u32_t", bits = "21..=21")]
    #[bitfield(name = "op_obj_flag", ty = "mln_u32_t", bits = "22..=22")]
    #[bitfield(name = "op_real_flag", ty = "mln_u32_t", bits = "23..=23")]
    #[bitfield(name = "op_str_flag", ty = "mln_u32_t", bits = "24..=24")]
    #[bitfield(name = "quit", ty = "mln_u32_t", bits = "25..=25")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "26..=31")]
    pub sym_count_ret_flag_op_array_flag_op_bool_flag_op_func_flag_op_int_flag_op_nil_flag_op_obj_flag_op_real_flag_op_str_flag_quit_padding: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_lang_symbol_node_t = mln_lang_symbol_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_symbol_node_s {
    pub symbol: *mut mln_string_t,
    pub ctx: *mut mln_lang_ctx_t,
    pub type_0: mln_lang_symbol_type_t,
    pub data: C2RustUnnamed_9,
    pub layer: mln_uauto_t,
    pub bucket: *mut mln_lang_hash_bucket_t,
    pub prev: *mut mln_lang_symbol_node_s,
    pub next: *mut mln_lang_symbol_node_s,
    pub scope_prev: *mut mln_lang_symbol_node_s,
    pub scope_next: *mut mln_lang_symbol_node_s,
}
pub type mln_lang_hash_bucket_t = mln_lang_hash_bucket_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_hash_bucket_s {
    pub head: *mut mln_lang_symbol_node_t,
    pub tail: *mut mln_lang_symbol_node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub var: *mut mln_lang_var_t,
    pub set: *mut mln_lang_set_detail_t,
}
pub type mln_lang_set_detail_t = mln_lang_set_detail_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_set_detail_s {
    pub name: *mut mln_string_t,
    pub members: *mut mln_rbtree_t,
    pub ref_0: mln_u64_t,
}
pub type mln_lang_var_t = mln_lang_var_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_var_s {
    pub type_0: mln_lang_var_type_t,
    pub name: *mut mln_string_t,
    pub val: *mut mln_lang_val_t,
    pub in_set: *mut mln_lang_set_detail_t,
    pub prev: *mut mln_lang_var_t,
    pub next: *mut mln_lang_var_t,
    pub ref_0: mln_uauto_t,
}
pub type mln_lang_var_type_t = libc::c_uint;
pub const M_LANG_VAR_REFER: mln_lang_var_type_t = 1;
pub const M_LANG_VAR_NORMAL: mln_lang_var_type_t = 0;
pub type mln_lang_symbol_type_t = libc::c_uint;
pub const M_LANG_SYMBOL_SET: mln_lang_symbol_type_t = 1;
pub const M_LANG_SYMBOL_VAR: mln_lang_symbol_type_t = 0;
pub type mln_lang_hash_t = mln_lang_hash_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_hash_s {
    pub bucket: *mut mln_lang_hash_bucket_t,
    pub len: mln_size_t,
}
pub type mln_lang_ast_cache_t = mln_lang_ast_cache_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lang_ast_cache_s {
    pub stm: *mut mln_lang_stm_t,
    pub code: *mut mln_string_t,
    #[bitfield(name = "ref_0", ty = "mln_u64_t", bits = "0..=62")]
    #[bitfield(name = "expire", ty = "mln_u64_t", bits = "63..=63")]
    pub ref_0_expire: [u8; 8],
    pub timestamp: mln_u64_t,
    pub prev: *mut mln_lang_ast_cache_s,
    pub next: *mut mln_lang_ast_cache_s,
}
pub type mln_lang_return_handler = Option::<
    unsafe extern "C" fn(*mut mln_lang_ctx_t) -> (),
>;
pub type mln_lang_scope_t = mln_lang_scope_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_scope_s {
    pub type_0: mln_lang_scope_type_t,
    pub name: *mut mln_string_t,
    pub ctx: *mut mln_lang_ctx_t,
    pub cur_stack: *mut mln_lang_stack_node_t,
    pub entry: *mut mln_lang_stm_t,
    pub layer: mln_uauto_t,
    pub sym_head: *mut mln_lang_symbol_node_t,
    pub sym_tail: *mut mln_lang_symbol_node_t,
}
pub type mln_lang_stack_node_t = mln_lang_stack_node_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lang_stack_node_s {
    pub ctx: *mut mln_lang_ctx_t,
    pub type_0: mln_lang_stack_node_type_t,
    #[bitfield(name = "step", ty = "mln_u32_t", bits = "0..=30")]
    #[bitfield(name = "call", ty = "mln_u32_t", bits = "31..=31")]
    pub step_call: [u8; 4],
    pub data: C2RustUnnamed_10,
    pub ret_var: *mut mln_lang_var_t,
    pub ret_var2: *mut mln_lang_var_t,
    pub pos: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub stm: *mut mln_lang_stm_t,
    pub funcdef: *mut mln_lang_funcdef_t,
    pub set: *mut mln_lang_set_t,
    pub set_stm: *mut mln_lang_setstm_t,
    pub block: *mut mln_lang_block_t,
    pub w: *mut mln_lang_while_t,
    pub sw: *mut mln_lang_switch_t,
    pub sw_stm: *mut mln_lang_switchstm_t,
    pub f: *mut mln_lang_for_t,
    pub i: *mut mln_lang_if_t,
    pub exp: *mut mln_lang_exp_t,
    pub assign: *mut mln_lang_assign_t,
    pub logiclow: *mut mln_lang_logiclow_t,
    pub logichigh: *mut mln_lang_logichigh_t,
    pub relativelow: *mut mln_lang_relativelow_t,
    pub relativehigh: *mut mln_lang_relativehigh_t,
    pub move_0: *mut mln_lang_move_t,
    pub addsub: *mut mln_lang_addsub_t,
    pub muldiv: *mut mln_lang_muldiv_t,
    pub not: *mut mln_lang_not_t,
    pub suffix: *mut mln_lang_suffix_t,
    pub locate: *mut mln_lang_locate_t,
    pub spec: *mut mln_lang_spec_t,
    pub factor: *mut mln_lang_factor_t,
    pub elemlist: *mut mln_lang_elemlist_t,
}
pub type mln_lang_stack_node_type_t = libc::c_uint;
pub const M_LSNT_ELEMLIST: mln_lang_stack_node_type_t = 24;
pub const M_LSNT_FACTOR: mln_lang_stack_node_type_t = 23;
pub const M_LSNT_SPEC: mln_lang_stack_node_type_t = 22;
pub const M_LSNT_LOCATE: mln_lang_stack_node_type_t = 21;
pub const M_LSNT_SUFFIX: mln_lang_stack_node_type_t = 20;
pub const M_LSNT_NOT: mln_lang_stack_node_type_t = 19;
pub const M_LSNT_MULDIV: mln_lang_stack_node_type_t = 18;
pub const M_LSNT_ADDSUB: mln_lang_stack_node_type_t = 17;
pub const M_LSNT_MOVE: mln_lang_stack_node_type_t = 16;
pub const M_LSNT_RELATIVEHIGH: mln_lang_stack_node_type_t = 15;
pub const M_LSNT_RELATIVELOW: mln_lang_stack_node_type_t = 14;
pub const M_LSNT_LOGICHIGH: mln_lang_stack_node_type_t = 13;
pub const M_LSNT_LOGICLOW: mln_lang_stack_node_type_t = 12;
pub const M_LSNT_ASSIGN: mln_lang_stack_node_type_t = 11;
pub const M_LSNT_EXP: mln_lang_stack_node_type_t = 10;
pub const M_LSNT_IF: mln_lang_stack_node_type_t = 9;
pub const M_LSNT_FOR: mln_lang_stack_node_type_t = 8;
pub const M_LSNT_SWITCHSTM: mln_lang_stack_node_type_t = 7;
pub const M_LSNT_SWITCH: mln_lang_stack_node_type_t = 6;
pub const M_LSNT_WHILE: mln_lang_stack_node_type_t = 5;
pub const M_LSNT_BLOCK: mln_lang_stack_node_type_t = 4;
pub const M_LSNT_SETSTM: mln_lang_stack_node_type_t = 3;
pub const M_LSNT_SET: mln_lang_stack_node_type_t = 2;
pub const M_LSNT_FUNCDEF: mln_lang_stack_node_type_t = 1;
pub const M_LSNT_STM: mln_lang_stack_node_type_t = 0;
pub type mln_lang_scope_type_t = libc::c_uint;
pub const M_LANG_SCOPE_TYPE_FUNC: mln_lang_scope_type_t = 1;
pub const M_LANG_SCOPE_TYPE_SET: mln_lang_scope_type_t = 0;
pub type mln_lang_t = mln_lang_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_lang_s {
    pub ev: *mut mln_event_t,
    pub pool: *mut mln_alloc_t,
    pub run_head: *mut mln_lang_ctx_t,
    pub run_tail: *mut mln_lang_ctx_t,
    pub wait_head: *mut mln_lang_ctx_t,
    pub wait_tail: *mut mln_lang_ctx_t,
    pub resource_set: *mut mln_rbtree_t,
    #[bitfield(name = "wait", ty = "mln_u64_t", bits = "0..=61")]
    #[bitfield(name = "quit", ty = "mln_u64_t", bits = "62..=62")]
    #[bitfield(name = "cache", ty = "mln_u64_t", bits = "63..=63")]
    pub wait_quit_cache: [u8; 8],
    pub shift_table: *mut libc::c_void,
    pub cache_head: *mut mln_lang_ast_cache_t,
    pub cache_tail: *mut mln_lang_ast_cache_t,
    pub signal: mln_lang_run_ctl_t,
    pub clear: mln_lang_run_ctl_t,
    pub launcher: ev_fd_handler,
    pub alias_set: *mut mln_rbtree_t,
    pub lock: pthread_mutex_t,
}
pub type mln_lang_run_ctl_t = Option::<
    unsafe extern "C" fn(*mut mln_lang_t) -> libc::c_int,
>;
pub type mln_lang_func_type_t = libc::c_uint;
pub const M_FUNC_EXTERNAL: mln_lang_func_type_t = 1;
pub const M_FUNC_INTERNAL: mln_lang_func_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub i: mln_s64_t,
    pub b: mln_u8_t,
    pub f: libc::c_double,
    pub s: *mut mln_string_t,
    pub obj: *mut mln_lang_object_t,
    pub func: *mut mln_lang_func_detail_t,
    pub array: *mut mln_lang_array_t,
    pub call: *mut mln_lang_funccall_val_t,
}
pub type mln_lang_funccall_val_t = mln_lang_funccall_val_s;
pub type mln_lang_array_t = mln_lang_array_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_array_s {
    pub elems_index: *mut mln_rbtree_t,
    pub elems_key: *mut mln_rbtree_t,
    pub index: mln_u64_t,
    pub ref_0: mln_u64_t,
    pub gc_item: *mut mln_lang_gc_item_t,
    pub ctx: *mut mln_lang_ctx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_gc_item_t {
    pub gc: *mut mln_gc_t,
    pub type_0: mln_lang_gc_type_t,
    pub data: C2RustUnnamed_12,
    pub gc_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub obj: *mut mln_lang_object_t,
    pub array: *mut mln_lang_array_t,
}
pub type mln_lang_object_t = mln_lang_object_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_object_s {
    pub in_set: *mut mln_lang_set_detail_t,
    pub members: *mut mln_rbtree_t,
    pub ref_0: mln_u64_t,
    pub gc_item: *mut mln_lang_gc_item_t,
    pub ctx: *mut mln_lang_ctx_t,
}
pub type mln_lang_gc_type_t = libc::c_uint;
pub const M_GC_ARRAY: mln_lang_gc_type_t = 1;
pub const M_GC_OBJ: mln_lang_gc_type_t = 0;
pub type mln_lang_ctx_pipe_recv_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_lang_ctx_t, *mut mln_lang_val_t) -> libc::c_int,
>;
pub type mln_trace_init_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_lang_ctx_t) -> libc::c_int,
>;
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
static mut trace_lang: *mut mln_lang_t = 0 as *const mln_lang_t as *mut mln_lang_t;
static mut trace_ctx: *mut mln_lang_ctx_t = 0 as *const mln_lang_ctx_t
    as *mut mln_lang_ctx_t;
static mut trace_signal_fds: [libc::c_int; 2] = [0; 2];
static mut trace_init_cb: mln_trace_init_cb_t = None;
#[no_mangle]
pub unsafe extern "C" fn mln_trace_path() -> *mut mln_string_t {
    let mut cf: *mut mln_conf_t = 0 as *mut mln_conf_t;
    let mut cd: *mut mln_conf_domain_t = 0 as *mut mln_conf_domain_t;
    let mut cc: *mut mln_conf_cmd_t = 0 as *mut mln_conf_cmd_t;
    let mut ci: *mut mln_conf_item_t = 0 as *mut mln_conf_item_t;
    cf = mln_conf();
    if cf.is_null() {
        return 0 as *mut mln_string_t;
    }
    cd = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cd.is_null() {
        return 0 as *mut mln_string_t;
    }
    cc = ((*cd).search)
        .expect(
            "non-null function pointer",
        )(cd, b"trace_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cc.is_null() {
        return 0 as *mut mln_string_t;
    }
    ci = ((*cc).search)
        .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
    if mln_conf_arg_num(cc) != 1 as libc::c_int as libc::c_uint || ci.is_null() {
        return 0 as *mut mln_string_t;
    }
    if (*ci).type_0 as libc::c_uint == CONF_STR as libc::c_int as libc::c_uint {
        return (*ci).val.s;
    }
    return 0 as *mut mln_string_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_trace_init(
    mut ev: *mut mln_event_t,
    mut path: *mut mln_string_t,
) -> libc::c_int {
    if !(path.is_null() || !trace_lang.is_null() || !trace_ctx.is_null()) {
        if !(socketpair(
            1 as libc::c_int,
            SOCK_STREAM as libc::c_int,
            0 as libc::c_int,
            trace_signal_fds.as_mut_ptr(),
        ) < 0 as libc::c_int)
        {
            trace_lang = mln_lang_new(
                ev,
                Some(
                    mln_trace_lang_signal
                        as unsafe extern "C" fn(*mut mln_lang_t) -> libc::c_int,
                ),
                Some(
                    mln_trace_lang_clear
                        as unsafe extern "C" fn(*mut mln_lang_t) -> libc::c_int,
                ),
            );
            if !trace_lang.is_null() {
                (*trace_lang).set_cache(1 as libc::c_int as mln_u64_t);
                trace_ctx = mln_lang_job_new(
                    trace_lang,
                    0 as *mut mln_string_t,
                    1 as libc::c_int as mln_u32_t,
                    path,
                    0 as *mut libc::c_void,
                    Some(
                        mln_trace_lang_ctx_return_handler
                            as unsafe extern "C" fn(*mut mln_lang_ctx_t) -> (),
                    ),
                );
                if !trace_ctx.is_null() {
                    if !(mln_event_timer_set(
                        ev,
                        10 as libc::c_int as mln_u32_t,
                        0 as *mut libc::c_void,
                        Some(
                            mln_trace_lang_check
                                as unsafe extern "C" fn(
                                    *mut mln_event_t,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                    ))
                        .is_null()
                    {
                        if !(trace_init_cb.is_some()
                            && trace_init_cb
                                .expect("non-null function pointer")(trace_ctx)
                                < 0 as libc::c_int)
                        {
                            return 0 as libc::c_int;
                        }
                    }
                }
                mln_lang_free(trace_lang);
            }
            close(trace_signal_fds[0 as libc::c_int as usize]);
            close(trace_signal_fds[1 as libc::c_int as usize]);
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mln_trace_task_get() -> *mut mln_lang_ctx_t {
    return trace_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn mln_trace_finalize() {
    mln_lang_free(trace_lang);
    trace_lang = 0 as *mut mln_lang_t;
    trace_ctx = 0 as *mut mln_lang_ctx_t;
}
unsafe extern "C" fn mln_trace_lang_check(
    mut ev: *mut mln_event_t,
    mut data: *mut libc::c_void,
) {
    if !trace_lang.is_null() && ((*trace_lang).run_head).is_null()
        && ((*trace_lang).wait_head).is_null()
    {
        mln_lang_free(trace_lang);
        trace_lang = 0 as *mut mln_lang_t;
        trace_ctx = 0 as *mut mln_lang_ctx_t;
    } else {
        mln_event_timer_set(
            ev,
            10 as libc::c_int as mln_u32_t,
            0 as *mut libc::c_void,
            Some(
                mln_trace_lang_check
                    as unsafe extern "C" fn(*mut mln_event_t, *mut libc::c_void) -> (),
            ),
        );
    };
}
unsafe extern "C" fn mln_trace_lang_signal(mut lang: *mut mln_lang_t) -> libc::c_int {
    return mln_event_fd_set(
        (*lang).ev,
        trace_signal_fds[0 as libc::c_int as usize],
        0x2 as libc::c_int as mln_u32_t | 0x8 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        lang as *mut libc::c_void,
        (*lang).launcher,
    );
}
unsafe extern "C" fn mln_trace_lang_clear(mut lang: *mut mln_lang_t) -> libc::c_int {
    return mln_event_fd_set(
        (*lang).ev,
        trace_signal_fds[0 as libc::c_int as usize],
        0x80 as libc::c_int as mln_u32_t,
        -(1 as libc::c_int),
        0 as *mut libc::c_void,
        None,
    );
}
unsafe extern "C" fn mln_trace_lang_ctx_return_handler(mut ctx: *mut mln_lang_ctx_t) {
    trace_ctx = 0 as *mut mln_lang_ctx_t;
    if (*ctx).quit() != 0 {
        _mln_sys_log(
            warn,
            b"src/mln_trace.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"mln_trace_lang_ctx_return_handler\0"))
                .as_ptr(),
            113 as libc::c_int,
            b"trace script exit\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_trace_init_callback_set(mut cb: mln_trace_init_cb_t) {
    trace_init_cb = cb;
}
#[no_mangle]
pub unsafe extern "C" fn mln_trace_recv_handler_set(
    mut recv_handler: mln_lang_ctx_pipe_recv_cb_t,
) -> libc::c_int {
    if trace_ctx.is_null() {
        return -(1 as libc::c_int);
    }
    return mln_lang_ctx_pipe_recv_handler_set(trace_ctx, recv_handler);
}
