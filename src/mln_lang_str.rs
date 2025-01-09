use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn mln_string_pool_dup(
        pool: *mut mln_alloc_t,
        str: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_string_strseqcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn mln_string_pool_strcat(
        pool: *mut mln_alloc_t,
        s1: *mut mln_string_t,
        s2: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_lang_errmsg(ctx: *mut mln_lang_ctx_t, msg: *mut libc::c_char);
    fn mln_lang_var_create_true(
        ctx: *mut mln_lang_ctx_t,
        name: *mut mln_string_t,
    ) -> *mut mln_lang_var_t;
    fn mln_lang_var_create_false(
        ctx: *mut mln_lang_ctx_t,
        name: *mut mln_string_t,
    ) -> *mut mln_lang_var_t;
    fn mln_lang_var_create_string(
        ctx: *mut mln_lang_ctx_t,
        s: *mut mln_string_t,
        name: *mut mln_string_t,
    ) -> *mut mln_lang_var_t;
    fn mln_lang_var_new(
        ctx: *mut mln_lang_ctx_t,
        name: *mut mln_string_t,
        type_0: mln_lang_var_type_t,
        val: *mut mln_lang_val_t,
        in_set: *mut mln_lang_set_detail_t,
    ) -> *mut mln_lang_var_t;
    fn mln_lang_var_set_string(var: *mut mln_lang_var_t, s: *mut mln_string_t);
    fn mln_lang_var_value_set_string_ref(
        ctx: *mut mln_lang_ctx_t,
        dest: *mut mln_lang_var_t,
        src: *mut mln_lang_var_t,
    ) -> libc::c_int;
    fn mln_lang_val_new(
        ctx: *mut mln_lang_ctx_t,
        type_0: mln_s32_t,
        data: *mut libc::c_void,
    ) -> *mut mln_lang_val_t;
    fn mln_lang_val_free(val: *mut mln_lang_val_t);
    fn mln_lang_condition_is_true(var: *mut mln_lang_var_t) -> libc::c_int;
    fn mln_lang_funccall_val_operator(
        ctx: *mut mln_lang_ctx_t,
        name: *mut mln_string_t,
        ret: *mut *mut mln_lang_var_t,
        op1: *mut mln_lang_var_t,
        op2: *mut mln_lang_var_t,
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type off_t = __off_t;
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
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s64_t = libc::c_long;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_off_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_stm_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_stm_type_t,
    pub data: C2RustUnnamed_0,
    pub next: *mut mln_lang_stm_s,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub data: C2RustUnnamed_1,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub right: C2RustUnnamed_2,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub right: C2RustUnnamed_3,
    pub next: *mut mln_lang_locate_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub data: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub data: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub data: C2RustUnnamed_6,
    pub next: *mut mln_lang_setstm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub data: C2RustUnnamed_10,
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
    pub data: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub data: C2RustUnnamed_8,
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
pub union C2RustUnnamed_8 {
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
    pub data: C2RustUnnamed_9,
    pub ret_var: *mut mln_lang_var_t,
    pub ret_var2: *mut mln_lang_var_t,
    pub pos: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
pub union C2RustUnnamed_10 {
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
    pub data: C2RustUnnamed_11,
    pub gc_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_methods_s {
    pub assign_handler: mln_lang_op,
    pub pluseq_handler: mln_lang_op,
    pub subeq_handler: mln_lang_op,
    pub lmoveq_handler: mln_lang_op,
    pub rmoveq_handler: mln_lang_op,
    pub muleq_handler: mln_lang_op,
    pub diveq_handler: mln_lang_op,
    pub oreq_handler: mln_lang_op,
    pub andeq_handler: mln_lang_op,
    pub xoreq_handler: mln_lang_op,
    pub modeq_handler: mln_lang_op,
    pub cor_handler: mln_lang_op,
    pub cand_handler: mln_lang_op,
    pub cxor_handler: mln_lang_op,
    pub equal_handler: mln_lang_op,
    pub nonequal_handler: mln_lang_op,
    pub less_handler: mln_lang_op,
    pub lesseq_handler: mln_lang_op,
    pub grea_handler: mln_lang_op,
    pub greale_handler: mln_lang_op,
    pub lmov_handler: mln_lang_op,
    pub rmov_handler: mln_lang_op,
    pub plus_handler: mln_lang_op,
    pub sub_handler: mln_lang_op,
    pub mul_handler: mln_lang_op,
    pub div_handler: mln_lang_op,
    pub mod_handler: mln_lang_op,
    pub sdec_handler: mln_lang_op,
    pub sinc_handler: mln_lang_op,
    pub index_handler: mln_lang_op,
    pub property_handler: mln_lang_op,
    pub negative_handler: mln_lang_op,
    pub reverse_handler: mln_lang_op,
    pub not_handler: mln_lang_op,
    pub pinc_handler: mln_lang_op,
    pub pdec_handler: mln_lang_op,
}
pub type mln_lang_op = Option::<
    unsafe extern "C" fn(
        *mut mln_lang_ctx_t,
        *mut *mut mln_lang_var_t,
        *mut mln_lang_var_t,
        *mut mln_lang_var_t,
    ) -> libc::c_int,
>;
pub type mln_lang_method_t = mln_lang_methods_s;
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
pub static mut mln_lang_str_oprs: mln_lang_method_t = unsafe {
    {
        let mut init = mln_lang_methods_s {
            assign_handler: Some(
                mln_lang_str_assign
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            pluseq_handler: Some(
                mln_lang_str_pluseq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            subeq_handler: Some(
                mln_lang_str_subeq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            lmoveq_handler: Some(
                mln_lang_str_lmoveq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            rmoveq_handler: Some(
                mln_lang_str_rmoveq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            muleq_handler: Some(
                mln_lang_str_muleq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            diveq_handler: Some(
                mln_lang_str_diveq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            oreq_handler: Some(
                mln_lang_str_oreq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            andeq_handler: Some(
                mln_lang_str_andeq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            xoreq_handler: Some(
                mln_lang_str_xoreq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            modeq_handler: Some(
                mln_lang_str_modeq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            cor_handler: Some(
                mln_lang_str_cor
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            cand_handler: Some(
                mln_lang_str_cand
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            cxor_handler: Some(
                mln_lang_str_cxor
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            equal_handler: Some(
                mln_lang_str_equal
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            nonequal_handler: Some(
                mln_lang_str_nonequal
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            less_handler: Some(
                mln_lang_str_less
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            lesseq_handler: Some(
                mln_lang_str_lesseq
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            grea_handler: Some(
                mln_lang_str_grea
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            greale_handler: Some(
                mln_lang_str_greale
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            lmov_handler: Some(
                mln_lang_str_lmov
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            rmov_handler: Some(
                mln_lang_str_rmov
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            plus_handler: Some(
                mln_lang_str_plus
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            sub_handler: Some(
                mln_lang_str_sub
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            mul_handler: Some(
                mln_lang_str_mul
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            div_handler: Some(
                mln_lang_str_div
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            mod_handler: Some(
                mln_lang_str_mod
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            sdec_handler: Some(
                mln_lang_str_sdec
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            sinc_handler: Some(
                mln_lang_str_sinc
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            index_handler: Some(
                mln_lang_str_index
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            property_handler: Some(
                mln_lang_str_property
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            negative_handler: Some(
                mln_lang_str_negative
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            reverse_handler: Some(
                mln_lang_str_reverse
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            not_handler: Some(
                mln_lang_str_not
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            pinc_handler: Some(
                mln_lang_str_pinc
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
            pdec_handler: Some(
                mln_lang_str_pdec
                    as unsafe extern "C" fn(
                        *mut mln_lang_ctx_t,
                        *mut *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                        *mut mln_lang_var_t,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
static mut mln_lang_str_opr_names: [mln_string_t; 36] = [mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
}; 36];
#[no_mangle]
pub unsafe extern "C" fn mln_lang_str_var_tostring(
    mut pool: *mut mln_alloc_t,
    mut var: *mut mln_lang_var_t,
) -> *mut mln_string_t {
    return __mln_lang_str_var_tostring(pool, var);
}
#[inline]
unsafe extern "C" fn __mln_lang_str_var_tostring(
    mut pool: *mut mln_alloc_t,
    mut var: *mut mln_lang_var_t,
) -> *mut mln_string_t {
    let mut buf: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut val: *mut mln_lang_val_t = (*var).val;
    let mut n: libc::c_int = 0 as libc::c_int;
    match (*val).type_0 {
        0 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"nil\0" as *const u8 as *const libc::c_char,
            );
        }
        5 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"Object\0" as *const u8 as *const libc::c_char,
            );
        }
        6 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"Function\0" as *const u8 as *const libc::c_char,
            );
        }
        7 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"Array\0" as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%ld\0" as *const u8 as *const libc::c_char,
                (*val).data.i,
            );
        }
        2 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%s\0" as *const u8 as *const libc::c_char,
                if (*val).data.b as libc::c_int != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        3 => {
            n = snprintf(
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%lf\0" as *const u8 as *const libc::c_char,
                (*val).data.f,
            );
        }
        4 => {
            let mut s: *mut mln_string_t = ({
                let mut __s: *mut mln_string_t = (*val).data.s;
                (*__s).set_ref_0((*__s).ref_0() + 1);
                (*__s).ref_0();
                __s
            });
            return s;
        }
        _ => {}
    }
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    ({
        tmp.data = buf.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = n as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    return mln_string_pool_dup(pool, &mut tmp);
}
unsafe extern "C" fn mln_lang_str_assign(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(0 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if mln_lang_var_value_set_string_ref(ctx, op1, op2) < 0 as libc::c_int {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*op1).ref_0 = ((*op1).ref_0).wrapping_add(1);
    (*op1).ref_0;
    *ret = op1;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_pluseq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(1 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    s = mln_string_pool_strcat((*ctx).pool, tmp1, tmp2);
    if s.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        let mut __s: *mut mln_string_t = tmp2;
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
        return -(1 as libc::c_int);
    }
    let mut __s: *mut mln_string_t = tmp1;
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
    let mut __s: *mut mln_string_t = tmp2;
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
    mln_lang_var_set_string(op1, s);
    (*op1).ref_0 = ((*op1).ref_0).wrapping_add(1);
    (*op1).ref_0;
    *ret = op1;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_subeq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(2 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_lmoveq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(3 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_rmoveq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(4 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_muleq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(5 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_diveq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(6 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_oreq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(7 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_andeq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(8 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_xoreq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(9 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_modeq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(10 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_cor(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(11 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_cand(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(12 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_cxor(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(13 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_equal(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(14 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    tmp = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rv = mln_string_strcmp((*(*op1).val).data.s, tmp);
    let mut __s: *mut mln_string_t = tmp;
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
    if rv != 0 {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_nonequal(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(15 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    tmp = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rv = mln_string_strcmp((*(*op1).val).data.s, tmp);
    let mut __s: *mut mln_string_t = tmp;
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
    if rv != 0 {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_less(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(16 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        mln_lang_errmsg(
            ctx,
            b"Operation NOT support.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    rv = mln_string_strseqcmp(tmp1, tmp2);
    let mut __s: *mut mln_string_t = tmp1;
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
    let mut __s: *mut mln_string_t = tmp2;
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
    if rv < 0 as libc::c_int {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_lesseq(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(17 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        mln_lang_errmsg(
            ctx,
            b"Operation NOT support.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    rv = mln_string_strseqcmp(tmp1, tmp2);
    let mut __s: *mut mln_string_t = tmp1;
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
    let mut __s: *mut mln_string_t = tmp2;
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
    if rv <= 0 as libc::c_int {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_grea(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(18 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        mln_lang_errmsg(
            ctx,
            b"Operation NOT support.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    rv = mln_string_strseqcmp(tmp1, tmp2);
    let mut __s: *mut mln_string_t = tmp1;
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
    let mut __s: *mut mln_string_t = tmp2;
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
    if rv > 0 as libc::c_int {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_greale(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(19 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut rv: libc::c_int = 0;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut type_0: mln_s32_t = (*(*op2).val).type_0;
    if type_0 != 1 as libc::c_int && type_0 != 3 as libc::c_int
        && type_0 != 4 as libc::c_int
    {
        mln_lang_errmsg(
            ctx,
            b"Operation NOT support.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    rv = mln_string_strseqcmp(tmp1, tmp2);
    let mut __s: *mut mln_string_t = tmp1;
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
    let mut __s: *mut mln_string_t = tmp2;
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
    if rv >= 0 as libc::c_int {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_lmov(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(20 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_rmov(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(21 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_plus(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(22 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp1: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp2: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut val: *mut mln_lang_val_t = 0 as *mut mln_lang_val_t;
    tmp1 = __mln_lang_str_var_tostring((*ctx).pool, op1);
    if tmp1.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tmp2 = __mln_lang_str_var_tostring((*ctx).pool, op2);
    if tmp2.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
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
        return -(1 as libc::c_int);
    }
    s = mln_string_pool_strcat((*ctx).pool, tmp1, tmp2);
    if s.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = tmp1;
        if !__s.is_null() {
            let ref mut fresh41 = (*__s).ref_0();
            let fresh42 = *fresh41;
            *fresh41 = (*fresh41).wrapping_sub(1);
            if fresh42 <= 1 as libc::c_int as libc::c_ulong {
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
        let mut __s: *mut mln_string_t = tmp2;
        if !__s.is_null() {
            let ref mut fresh43 = (*__s).ref_0();
            let fresh44 = *fresh43;
            *fresh43 = (*fresh43).wrapping_sub(1);
            if fresh44 <= 1 as libc::c_int as libc::c_ulong {
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
        return -(1 as libc::c_int);
    }
    let mut __s: *mut mln_string_t = tmp1;
    if !__s.is_null() {
        let ref mut fresh45 = (*__s).ref_0();
        let fresh46 = *fresh45;
        *fresh45 = (*fresh45).wrapping_sub(1);
        if fresh46 <= 1 as libc::c_int as libc::c_ulong {
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
    let mut __s: *mut mln_string_t = tmp2;
    if !__s.is_null() {
        let ref mut fresh47 = (*__s).ref_0();
        let fresh48 = *fresh47;
        *fresh47 = (*fresh47).wrapping_sub(1);
        if fresh48 <= 1 as libc::c_int as libc::c_ulong {
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
    val = mln_lang_val_new(ctx, 4 as libc::c_int, s as *mut libc::c_void);
    if val.is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut __s: *mut mln_string_t = s;
        if !__s.is_null() {
            let ref mut fresh49 = (*__s).ref_0();
            let fresh50 = *fresh49;
            *fresh49 = (*fresh49).wrapping_sub(1);
            if fresh50 <= 1 as libc::c_int as libc::c_ulong {
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
        return -(1 as libc::c_int);
    }
    *ret = mln_lang_var_new(
        ctx,
        0 as *mut mln_string_t,
        M_LANG_VAR_NORMAL,
        val,
        0 as *mut mln_lang_set_detail_t,
    );
    if (*ret).is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_lang_val_free(val);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_sub(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(23 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_mul(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(24 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_div(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(25 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_mod(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(26 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_sdec(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(27 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_sinc(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(28 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_index(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(29 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut offset: mln_s64_t = 0;
    let mut c: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut s: *mut mln_string_t = (*(*op1).val).data.s;
    if (*(*op2).val).type_0 != 1 as libc::c_int {
        mln_lang_errmsg(
            ctx,
            b"Offset must be an integer.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    offset = (*(*op2).val).data.i;
    if offset < 0 as libc::c_int as libc::c_long {
        offset = (*s).len as mln_s64_t + offset;
    }
    if offset < 0 as libc::c_int as libc::c_long || offset as libc::c_ulong >= (*s).len {
        mln_lang_errmsg(
            ctx,
            b"Invalid offset.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ({
        c.data = &mut *((*s).data).offset(offset as isize) as *mut libc::c_uchar;
        c.len = 1 as libc::c_int as mln_u64_t;
        c.set_data_ref(1 as libc::c_int as mln_uauto_t);
        c.set_pool(0 as libc::c_int as mln_uauto_t);
        c.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut c;
        &mut c
    });
    *ret = mln_lang_var_create_string(ctx, &mut c, 0 as *mut mln_string_t);
    if (*ret).is_null() {
        mln_lang_errmsg(
            ctx,
            b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_property(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(30 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_negative(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(31 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_reverse(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(32 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_not(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(33 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if mln_lang_condition_is_true(op1) != 0 {
        *ret = mln_lang_var_create_false(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        *ret = mln_lang_var_create_true(ctx, 0 as *mut mln_string_t);
        if (*ret).is_null() {
            mln_lang_errmsg(
                ctx,
                b"No memory.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_str_pinc(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(34 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_str_pdec(
    mut ctx: *mut mln_lang_ctx_t,
    mut ret: *mut *mut mln_lang_var_t,
    mut op1: *mut mln_lang_var_t,
    mut op2: *mut mln_lang_var_t,
) -> libc::c_int {
    if (*ctx).op_str_flag() != 0 {
        let mut rc: libc::c_int = mln_lang_funccall_val_operator(
            ctx,
            &mut *mln_lang_str_opr_names.as_mut_ptr().offset(35 as libc::c_int as isize),
            ret,
            op1,
            op2,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
        if rc > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    mln_lang_errmsg(
        ctx,
        b"Operation NOT support.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    mln_lang_str_opr_names = [
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"__str_assign_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
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
                data: b"__str_pluseq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
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
                data: b"__str_subeq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_lmoveq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
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
                data: b"__str_rmoveq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
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
                data: b"__str_muleq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_diveq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_oreq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_andeq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_xoreq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_modeq_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_cor_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_cand_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_cxor_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_equal_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_nonequal_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
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
                data: b"__str_lt_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
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
                data: b"__str_le_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
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
                data: b"__str_gt_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
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
                data: b"__str_ge_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
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
                data: b"__str_lmov_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_rmov_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_plus_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_sub_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_mul_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_div_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_mod_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_sdec_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_sinc_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_index_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
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
                data: b"__str_property_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
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
                data: b"__str_negative_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
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
                data: b"__str_reverse_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
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
                data: b"__str_not_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
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
                data: b"__str_pinc_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
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
                data: b"__str_pdec_operator__\0" as *const u8 as *const libc::c_char
                    as mln_u8ptr_t,
                len: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init.set_data_ref(1 as libc::c_int as mln_uauto_t);
            init.set_pool(0 as libc::c_int as mln_uauto_t);
            init.set_ref_0(1 as libc::c_int as mln_uauto_t);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
