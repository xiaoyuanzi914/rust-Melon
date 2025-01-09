use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn mln_string_const_strcmp(
        s1: *mut mln_string_t,
        s2: *mut libc::c_char,
    ) -> libc::c_int;
    fn mln_conf() -> *mut mln_conf_t;
    static mut g_log: mln_log_t;
    fn mln_log_init(cf: *mut mln_conf_t) -> libc::c_int;
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
    fn mln_log_dir_path() -> *mut libc::c_char;
    fn mln_log_logfile_path() -> *mut libc::c_char;
    fn mln_log_pid_path() -> *mut libc::c_char;
    fn mln_path_null() -> *mut libc::c_char;
    fn mln_path_pid() -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
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
pub type ssize_t = __ssize_t;
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
pub type boot_param = Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_boot_t {
    pub boot_str: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub handler: boot_param,
    pub cnt: mln_size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: *mut mln_string_t,
    pub c: mln_s8_t,
    pub b: mln_u8_t,
    pub i: mln_sauto_t,
    pub f: libc::c_float,
}
pub type mln_conf_item_t = mln_conf_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_conf_item_s {
    pub type_0: mln_conf_item_type_t,
    pub val: C2RustUnnamed,
}
pub type mln_conf_item_type_t = libc::c_uint;
pub const CONF_FLOAT: mln_conf_item_type_t = 5;
pub const CONF_INT: mln_conf_item_type_t = 4;
pub const CONF_BOOL: mln_conf_item_type_t = 3;
pub const CONF_CHAR: mln_conf_item_type_t = 2;
pub const CONF_STR: mln_conf_item_type_t = 1;
pub const CONF_NONE: mln_conf_item_type_t = 0;
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
pub type mln_rbtree_t = rbtree_s;
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
pub type mln_rbtree_node_t = mln_rbtree_node_s;
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
pub type rbtree_free_data = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type rbtree_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type rbtree_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type rbtree_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type mln_conf_cmd_cb_t = Option::<
    unsafe extern "C" fn(
        *mut mln_conf_domain_t,
        *mut libc::c_char,
    ) -> *mut mln_conf_cmd_t,
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
pub type mln_event_t = mln_event_s;
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
pub type fheap_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type fheap_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type fheap_key_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type fheap_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type fheap_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type mln_fheap_node_t = mln_fheap_node_s;
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
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type mln_conf_domain_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_conf_t, *mut libc::c_char) -> *mut mln_conf_domain_t,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut mon_days: [[libc::c_long; 12]; 2] = [
    [
        31 as libc::c_int as libc::c_long,
        28 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
    ],
    [
        31 as libc::c_int as libc::c_long,
        29 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
        30 as libc::c_int as libc::c_long,
        31 as libc::c_int as libc::c_long,
    ],
];
#[no_mangle]
pub static mut boot_params: [mln_boot_t; 3] = unsafe {
    [
        {
            let mut init = mln_boot_t {
                boot_str: b"--help\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                alias: b"-h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                handler: Some(
                    mln_boot_help
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                ),
                cnt: 0 as libc::c_int as mln_size_t,
            };
            init
        },
        {
            let mut init = mln_boot_t {
                boot_str: b"--reload\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                alias: b"-r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                handler: Some(
                    mln_boot_reload
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                ),
                cnt: 0 as libc::c_int as mln_size_t,
            };
            init
        },
        {
            let mut init = mln_boot_t {
                boot_str: b"--stop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                alias: b"-s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                handler: Some(
                    mln_boot_stop
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                ),
                cnt: 0 as libc::c_int as mln_size_t,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut mln_core_file_cmd: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"core_file_size\0")
};
#[no_mangle]
pub static mut mln_nofile_cmd: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(b"max_nofile\0")
};
#[no_mangle]
pub static mut mln_limit_unlimited: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"unlimited\0")
};
#[no_mangle]
pub unsafe extern "C" fn mln_sys_limit_modify() -> libc::c_int {
    if mln_sys_core_modify() < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return mln_sys_nofile_modify();
}
unsafe extern "C" fn mln_sys_core_modify() -> libc::c_int {
    let mut core_file_size: rlim_t = 0 as libc::c_int as rlim_t;
    let mut cf: *mut mln_conf_t = mln_conf();
    if cf.is_null() {
        fprintf(
            stderr,
            b"Configuration messed up.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cd.is_null() {
        fprintf(
            stderr,
            b"Configuration messed up.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cc: *mut mln_conf_cmd_t = ((*cd).search)
        .expect("non-null function pointer")(cd, mln_core_file_cmd.as_mut_ptr());
    if cc.is_null() {
        return 0 as libc::c_int;
    }
    let mut ci: *mut mln_conf_item_t = ((*cc).search)
        .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
    if (*ci).type_0 as libc::c_uint == CONF_INT as libc::c_int as libc::c_uint {
        core_file_size = (*ci).val.i as rlim_t;
    } else if (*ci).type_0 as libc::c_uint == CONF_STR as libc::c_int as libc::c_uint {
        if mln_string_const_strcmp((*ci).val.s, mln_limit_unlimited.as_mut_ptr()) != 0 {
            fprintf(
                stderr,
                b"Invalid argument of %s.\n\0" as *const u8 as *const libc::c_char,
                mln_core_file_cmd.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
        core_file_size = -(1 as libc::c_int) as __rlim_t;
    } else {
        fprintf(
            stderr,
            b"Invalid argument of %s.\n\0" as *const u8 as *const libc::c_char,
            mln_core_file_cmd.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    let mut rl: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    memset(
        &mut rl as *mut rlimit as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<rlimit>() as libc::c_ulong,
    );
    rl.rlim_max = core_file_size;
    rl.rlim_cur = rl.rlim_max;
    if setrlimit(RLIMIT_CORE as libc::c_int, &mut rl) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"setrlimit core failed, %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_sys_nofile_modify() -> libc::c_int {
    let mut nofile: rlim_t = 0 as libc::c_int as rlim_t;
    let mut cf: *mut mln_conf_t = mln_conf();
    if cf.is_null() {
        fprintf(
            stderr,
            b"Configuration messed up.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cd: *mut mln_conf_domain_t = ((*cf).search)
        .expect(
            "non-null function pointer",
        )(cf, b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cd.is_null() {
        fprintf(
            stderr,
            b"Configuration messed up.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cc: *mut mln_conf_cmd_t = ((*cd).search)
        .expect("non-null function pointer")(cd, mln_nofile_cmd.as_mut_ptr());
    if cc.is_null() {
        return 0 as libc::c_int;
    }
    let mut ci: *mut mln_conf_item_t = ((*cc).search)
        .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
    if (*ci).type_0 as libc::c_uint == CONF_INT as libc::c_int as libc::c_uint {
        nofile = (*ci).val.i as rlim_t;
    } else if (*ci).type_0 as libc::c_uint == CONF_STR as libc::c_int as libc::c_uint {
        if mln_string_const_strcmp((*ci).val.s, mln_limit_unlimited.as_mut_ptr()) != 0 {
            fprintf(
                stderr,
                b"Invalid argument of %s.\n\0" as *const u8 as *const libc::c_char,
                mln_nofile_cmd.as_mut_ptr(),
            );
            return -(1 as libc::c_int);
        }
        nofile = -(1 as libc::c_int) as __rlim_t;
    } else {
        fprintf(
            stderr,
            b"Invalid argument of %s.\n\0" as *const u8 as *const libc::c_char,
            mln_nofile_cmd.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    let mut rl: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    memset(
        &mut rl as *mut rlimit as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<rlimit>() as libc::c_ulong,
    );
    rl.rlim_max = nofile;
    rl.rlim_cur = rl.rlim_max;
    if setrlimit(RLIMIT_NOFILE as libc::c_int, &mut rl) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"setrlimit fd failed, %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_daemon() -> libc::c_int {
    let mut ret: libc::c_int = mln_log_init(0 as *mut mln_conf_t);
    if ret < 0 as libc::c_int {
        return ret;
    }
    if g_log.in_daemon() == 0 {
        return mln_set_id();
    }
    let mut pid: pid_t = 0;
    pid = fork();
    if pid < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_daemon\0"))
                .as_ptr(),
            151 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    } else if pid != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    setsid();
    let mut fd0: libc::c_int = 0 as libc::c_int;
    let mut fd1: libc::c_int = 1 as libc::c_int;
    let mut fd2: libc::c_int = 2 as libc::c_int;
    close(fd0);
    close(fd1);
    close(fd2);
    fd0 = open(mln_path_null(), 0o2 as libc::c_int);
    fd1 = dup(fd0);
    fd2 = dup(fd0);
    if fd0 != 0 as libc::c_int || fd1 != 1 as libc::c_int || fd2 != 2 as libc::c_int {
        fprintf(
            stderr,
            b"Unexpected file descriptors %d %d %d\n\0" as *const u8
                as *const libc::c_char,
            fd0,
            fd1,
            fd2,
        );
    }
    return mln_set_id();
}
unsafe extern "C" fn mln_set_id() -> libc::c_int {
    let mut name: [libc::c_char; 256] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut len: libc::c_int = 0;
    let mut uid: uid_t = 0;
    let mut gid: gid_t = 0;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut cf: *mut mln_conf_t = mln_conf();
    if cf.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            186 as libc::c_int,
            b"No configuration.\n\0" as *const u8 as *const libc::c_char
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
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            191 as libc::c_int,
            b"No 'main' domain.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        abort();
    }
    let mut cc: *mut mln_conf_cmd_t = ((*cd).search)
        .expect(
            "non-null function pointer",
        )(cd, b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if cc.is_null() {
        uid = getuid();
        gid = getgid();
    } else {
        let mut ci: *mut mln_conf_item_t = ((*cc).search)
            .expect("non-null function pointer")(cc, 1 as libc::c_int as mln_u32_t);
        if ci.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"mln_set_id\0"))
                    .as_ptr(),
                201 as libc::c_int,
                b"Command 'user' need a parameter.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if (*ci).type_0 as libc::c_uint != CONF_STR as libc::c_int as libc::c_uint {
            _mln_sys_log(
                error,
                b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"mln_set_id\0"))
                    .as_ptr(),
                205 as libc::c_int,
                b"Parameter type of command 'user' error.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        len = (*(*ci).val.s).len as libc::c_int;
        if len > 255 as libc::c_int {
            len = 255 as libc::c_int;
        }
        memcpy(
            name.as_mut_ptr() as *mut libc::c_void,
            (*(*ci).val.s).data as *const libc::c_void,
            len as libc::c_ulong,
        );
        pwd = getpwnam(name.as_mut_ptr());
        if pwd.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"mln_set_id\0"))
                    .as_ptr(),
                212 as libc::c_int,
                b"getpwnam failed. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        if (*pwd).pw_uid != getuid() {
            uid = (*pwd).pw_uid;
            gid = (*pwd).pw_gid;
        } else {
            uid = getuid();
            gid = getgid();
        }
    }
    let mut rc: libc::c_int = 1 as libc::c_int;
    let mut path: *mut libc::c_char = mln_log_dir_path();
    if access(path, 0 as libc::c_int) == 0 {
        rc = chown(path, uid, gid);
    }
    path = mln_log_logfile_path();
    if access(path, 0 as libc::c_int) == 0 {
        rc = chown(path, uid, gid);
    }
    path = mln_log_pid_path();
    if access(path, 0 as libc::c_int) == 0 {
        rc = chown(path, uid, gid);
    }
    if rc < 0 as libc::c_int {
        rc = 1 as libc::c_int;
    }
    if setgid(gid) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            239 as libc::c_int,
            b"Set GID failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if setuid(uid) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            243 as libc::c_int,
            b"Set UID failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if setegid(gid) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            246 as libc::c_int,
            b"Set eGID failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if seteuid(uid) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_tools.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mln_set_id\0"))
                .as_ptr(),
            249 as libc::c_int,
            b"Set eUID failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_boot_params(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut mln_boot_t = 0 as *mut mln_boot_t;
    let mut bend: *mut mln_boot_t = boot_params
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[mln_boot_t; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<mln_boot_t>() as libc::c_ulong)
                as isize,
        );
    i = 1 as libc::c_int;
    while i < argc {
        p = *argv.offset(i as isize);
        b = boot_params.as_mut_ptr();
        while b < bend {
            if (*b).cnt == 0
                && (strcmp(p, (*b).boot_str) == 0 || strcmp(p, (*b).alias) == 0)
            {
                (*b).cnt = ((*b).cnt).wrapping_add(1);
                (*b).cnt;
                ret = ((*b).handler)
                    .expect("non-null function pointer")((*b).boot_str, (*b).alias);
                if ret < 0 as libc::c_int {
                    return ret;
                }
            }
            b = b.offset(1);
            b;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_boot_help(
    mut boot_str: *const libc::c_char,
    mut alias: *const libc::c_char,
) -> libc::c_int {
    printf(b"Boot parameters:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\t--reload  -r\t\t\treload configuration\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--stop    -s\t\t\tstop melon service.\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
unsafe extern "C" fn mln_boot_reload(
    mut boot_str: *const libc::c_char,
    mut alias: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut fd: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    fd = open(mln_log_pid_path(), 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"'melon.pid' not existent.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    n = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if n <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"Invalid file 'melon.pid'.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    buf[n as usize] = 0 as libc::c_int as libc::c_char;
    pid = atoi(buf.as_mut_ptr());
    kill(pid, 12 as libc::c_int);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn mln_boot_stop(
    mut boot_str: *const libc::c_char,
    mut alias: *const libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [
        0 as libc::c_int as libc::c_char,
    ];
    let mut fd: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    snprintf(
        buf.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"%s\0" as *const u8 as *const libc::c_char,
        mln_path_pid(),
    );
    fd = open(buf.as_mut_ptr(), 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"'melon.pid' not existent.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    n = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if n <= 0 as libc::c_int {
        fprintf(
            stderr,
            b"Invalid file 'melon.pid'.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    buf[n as usize] = 0 as libc::c_int as libc::c_char;
    pid = atoi(buf.as_mut_ptr());
    kill(pid, 9 as libc::c_int);
    exit(0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_is_leap(mut year: libc::c_long) -> libc::c_int {
    if year % 4 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long
        && year % 100 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long
        || year % 400 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_time2utc(mut tm: time_t, mut uc: *mut utctime) {
    let mut days: libc::c_long = tm / 86400 as libc::c_int as libc::c_long;
    let mut subsec: libc::c_long = tm % 86400 as libc::c_int as libc::c_long;
    let mut month: libc::c_long = 0;
    let mut year: libc::c_long = 0;
    let mut cnt: libc::c_long = 0 as libc::c_int as libc::c_long;
    (*uc).month = 0 as libc::c_int as libc::c_long;
    (*uc).year = (*uc).month;
    while (if mln_is_leap(1970 as libc::c_int as libc::c_long + (*uc).year) != 0 {
        cnt + 366 as libc::c_int as libc::c_long
    } else {
        cnt + 365 as libc::c_int as libc::c_long
    }) <= days
    {
        if mln_is_leap(1970 as libc::c_int as libc::c_long + (*uc).year) != 0 {
            cnt += 366 as libc::c_int as libc::c_long;
        } else {
            cnt += 365 as libc::c_int as libc::c_long;
        }
        (*uc).year += 1;
        (*uc).year;
    }
    (*uc).year += 1970 as libc::c_int as libc::c_long;
    let mut is_leap_year: libc::c_int = mln_is_leap((*uc).year);
    let mut subdays: libc::c_long = days - cnt;
    cnt = 0 as libc::c_int as libc::c_long;
    while cnt + mon_days[is_leap_year as usize][(*uc).month as usize] <= subdays {
        cnt += mon_days[is_leap_year as usize][(*uc).month as usize];
        (*uc).month += 1;
        (*uc).month;
    }
    (*uc).month += 1;
    (*uc).month;
    (*uc).day = subdays - cnt + 1 as libc::c_int as libc::c_long;
    (*uc).hour = subsec / 3600 as libc::c_int as libc::c_long;
    (*uc)
        .minute = subsec % 3600 as libc::c_int as libc::c_long
        / 60 as libc::c_int as libc::c_long;
    (*uc)
        .second = subsec % 3600 as libc::c_int as libc::c_long
        % 60 as libc::c_int as libc::c_long;
    month = if (*uc).month < 3 as libc::c_int as libc::c_long {
        (*uc).month + 12 as libc::c_int as libc::c_long
    } else {
        (*uc).month
    };
    year = if (*uc).month < 3 as libc::c_int as libc::c_long {
        (*uc).year - 1 as libc::c_int as libc::c_long
    } else {
        (*uc).year
    };
    (*uc)
        .week = ((*uc).day + 1 as libc::c_int as libc::c_long
        + 2 as libc::c_int as libc::c_long * month
        + 3 as libc::c_int as libc::c_long * (month + 1 as libc::c_int as libc::c_long)
            / 5 as libc::c_int as libc::c_long + year + (year >> 2 as libc::c_int)
        - year / 100 as libc::c_int as libc::c_long
        + year / 400 as libc::c_int as libc::c_long) % 7 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn mln_utc2time(mut uc: *mut utctime) -> time_t {
    let mut ret: time_t = 0 as libc::c_int as time_t;
    let mut year: libc::c_long = (*uc).year - 1 as libc::c_int as libc::c_long;
    let mut month: libc::c_long = (*uc).month - 2 as libc::c_int as libc::c_long;
    let mut is_leap_year: libc::c_int = mln_is_leap((*uc).year);
    while year >= 1970 as libc::c_int as libc::c_long {
        ret
            += (if mln_is_leap(year) != 0 {
                366 as libc::c_int
            } else {
                365 as libc::c_int
            }) as libc::c_long;
        year -= 1;
        year;
    }
    while month >= 0 as libc::c_int as libc::c_long {
        ret += mon_days[is_leap_year as usize][month as usize];
        month -= 1;
        month;
    }
    ret += (*uc).day - 1 as libc::c_int as libc::c_long;
    ret *= 86400 as libc::c_int as libc::c_long;
    ret
        += (*uc).hour * 3600 as libc::c_int as libc::c_long
            + (*uc).minute * 60 as libc::c_int as libc::c_long + (*uc).second;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_utc_adjust(mut uc: *mut utctime) {
    let mut adj: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut month: libc::c_long = 0;
    let mut year: libc::c_long = 0;
    if (*uc).second >= 60 as libc::c_int as libc::c_long {
        adj = (*uc).second / 60 as libc::c_int as libc::c_long;
        (*uc).second %= 60 as libc::c_int as libc::c_long;
    }
    if adj != 0 {
        (*uc).minute += adj;
        adj = 0 as libc::c_int as libc::c_long;
    }
    if (*uc).minute >= 60 as libc::c_int as libc::c_long {
        adj = (*uc).minute / 60 as libc::c_int as libc::c_long;
        (*uc).minute %= 60 as libc::c_int as libc::c_long;
    }
    if adj != 0 {
        (*uc).hour += adj;
        adj = 0 as libc::c_int as libc::c_long;
    }
    if (*uc).hour >= 24 as libc::c_int as libc::c_long {
        adj = (*uc).hour / 24 as libc::c_int as libc::c_long;
        (*uc).hour %= 24 as libc::c_int as libc::c_long;
    }
    if adj != 0 {
        (*uc).day += adj;
        adj = 0 as libc::c_int as libc::c_long;
    }
    year = (*uc).year;
    month = (*uc).month;
    while (*uc).day
        > mon_days[mln_is_leap(year)
            as usize][(month - 1 as libc::c_int as libc::c_long) as usize]
    {
        adj = 1 as libc::c_int as libc::c_long;
        (*uc).day
            -= mon_days[mln_is_leap(year)
                as usize][(month - 1 as libc::c_int as libc::c_long) as usize];
        month += 1;
        if month > 12 as libc::c_int as libc::c_long {
            month = 1 as libc::c_int as libc::c_long;
            year += 1;
            year;
        }
    }
    if adj != 0 {
        (*uc).month = month;
        (*uc).year = year;
        adj = 0 as libc::c_int as libc::c_long;
    }
    if (*uc).month - 1 as libc::c_int as libc::c_long
        >= 12 as libc::c_int as libc::c_long
    {
        adj = ((*uc).month - 1 as libc::c_int as libc::c_long)
            / 12 as libc::c_int as libc::c_long;
        (*uc)
            .month = ((*uc).month - 1 as libc::c_int as libc::c_long)
            % 12 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long;
    }
    if adj != 0 {
        (*uc).year += adj;
    }
    month = if (*uc).month < 3 as libc::c_int as libc::c_long {
        (*uc).month + 12 as libc::c_int as libc::c_long
    } else {
        (*uc).month
    };
    year = if (*uc).month < 3 as libc::c_int as libc::c_long {
        (*uc).year - 1 as libc::c_int as libc::c_long
    } else {
        (*uc).year
    };
    (*uc)
        .week = ((*uc).day + 1 as libc::c_int as libc::c_long
        + 2 as libc::c_int as libc::c_long * month
        + 3 as libc::c_int as libc::c_long * (month + 1 as libc::c_int as libc::c_long)
            / 5 as libc::c_int as libc::c_long + year + (year >> 2 as libc::c_int)
        - year / 100 as libc::c_int as libc::c_long
        + year / 400 as libc::c_int as libc::c_long) % 7 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn mln_month_days(
    mut year: libc::c_long,
    mut month: libc::c_long,
) -> libc::c_long {
    return mon_days[mln_is_leap(year)
        as usize][(month - 1 as libc::c_int as libc::c_long) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn mln_s2time(
    mut tm: *mut time_t,
    mut s: *mut mln_string_t,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut year: time_t = 0 as libc::c_int as time_t;
    let mut month: time_t = 0 as libc::c_int as time_t;
    let mut day: time_t = 0 as libc::c_int as time_t;
    let mut hour: time_t = 0 as libc::c_int as time_t;
    let mut minute: time_t = 0 as libc::c_int as time_t;
    let mut second: time_t = 0 as libc::c_int as time_t;
    let mut tmp: time_t = 0;
    match type_0 {
        0 => {
            p = (*s).data;
            end = ((*s).data)
                .offset((*s).len as isize)
                .offset(-(1 as libc::c_int as isize));
            if (*s).len != 13 as libc::c_int as libc::c_ulong
                || *end as libc::c_int != 'Z' as i32 && *end as libc::c_int != 'z' as i32
            {
                return -(1 as libc::c_int);
            }
            while p < end {
                if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32)
                {
                    return -(1 as libc::c_int);
                }
                p = p.offset(1);
                p;
            }
            p = (*s).data;
            let fresh0 = p;
            p = p.offset(1);
            year = ((*fresh0 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
            let fresh1 = p;
            p = p.offset(1);
            year += (*fresh1 as libc::c_int - '0' as i32) as libc::c_long;
            if year >= 50 as libc::c_int as libc::c_long {
                year += 1900 as libc::c_int as libc::c_long;
            } else {
                year += 2000 as libc::c_int as libc::c_long;
            }
        }
        1 => {
            p = (*s).data;
            end = ((*s).data)
                .offset((*s).len as isize)
                .offset(-(1 as libc::c_int as isize));
            if (*s).len != 15 as libc::c_int as libc::c_ulong
                || *end as libc::c_int != 'Z' as i32 && *end as libc::c_int != 'z' as i32
            {
                return -(1 as libc::c_int);
            }
            while p < end {
                if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32)
                {
                    return -(1 as libc::c_int);
                }
                p = p.offset(1);
                p;
            }
            p = (*s).data;
            let fresh2 = p;
            p = p.offset(1);
            year = ((*fresh2 as libc::c_int - '0' as i32) * 1000 as libc::c_int)
                as time_t;
            let fresh3 = p;
            p = p.offset(1);
            year
                += ((*fresh3 as libc::c_int - '0' as i32) * 100 as libc::c_int)
                    as libc::c_long;
            let fresh4 = p;
            p = p.offset(1);
            year
                += ((*fresh4 as libc::c_int - '0' as i32) * 10 as libc::c_int)
                    as libc::c_long;
            let fresh5 = p;
            p = p.offset(1);
            year += (*fresh5 as libc::c_int - '0' as i32) as libc::c_long;
        }
        _ => return -(1 as libc::c_int),
    }
    let fresh6 = p;
    p = p.offset(1);
    month = ((*fresh6 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
    let fresh7 = p;
    p = p.offset(1);
    month += (*fresh7 as libc::c_int - '0' as i32) as libc::c_long;
    let fresh8 = p;
    p = p.offset(1);
    day = ((*fresh8 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
    let fresh9 = p;
    p = p.offset(1);
    day += (*fresh9 as libc::c_int - '0' as i32) as libc::c_long;
    let fresh10 = p;
    p = p.offset(1);
    hour = ((*fresh10 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
    let fresh11 = p;
    p = p.offset(1);
    hour += (*fresh11 as libc::c_int - '0' as i32) as libc::c_long;
    let fresh12 = p;
    p = p.offset(1);
    minute = ((*fresh12 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
    let fresh13 = p;
    p = p.offset(1);
    minute += (*fresh13 as libc::c_int - '0' as i32) as libc::c_long;
    let fresh14 = p;
    p = p.offset(1);
    second = ((*fresh14 as libc::c_int - '0' as i32) * 10 as libc::c_int) as time_t;
    let fresh15 = p;
    p = p.offset(1);
    second += (*fresh15 as libc::c_int - '0' as i32) as libc::c_long;
    if year < 1970 as libc::c_int as libc::c_long
        || month > 12 as libc::c_int as libc::c_long
        || day
            > mon_days[mln_is_leap(year)
                as usize][(month - 1 as libc::c_int as libc::c_long) as usize]
        || hour >= 24 as libc::c_int as libc::c_long
        || minute >= 60 as libc::c_int as libc::c_long
        || second >= 60 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    tmp = 1970 as libc::c_int as time_t;
    while tmp < year {
        day
            += (if mln_is_leap(tmp) != 0 {
                366 as libc::c_int
            } else {
                365 as libc::c_int
            }) as libc::c_long;
        tmp += 1;
        tmp;
    }
    month -= 1;
    month;
    tmp = 0 as libc::c_int as time_t;
    while tmp < month {
        day += mon_days[mln_is_leap(year) as usize][tmp as usize];
        tmp += 1;
        tmp;
    }
    day -= 1;
    day;
    *tm = day * 86400 as libc::c_int as libc::c_long;
    if hour != 0 || minute != 0 || second != 0 {
        *tm
            += hour * 3600 as libc::c_int as libc::c_long
                + minute * 60 as libc::c_int as libc::c_long + second;
    }
    return 0 as libc::c_int;
}
