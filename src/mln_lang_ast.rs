use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mln_string_pool_dup(
        pool: *mut mln_alloc_t,
        str: *mut mln_string_t,
    ) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn _mln_sys_log(
        level: mln_log_level_t,
        file: *const libc::c_char,
        func: *const libc::c_char,
        line: libc::c_int,
        msg: *mut libc::c_char,
        _: ...
    );
    fn mln_pg_token_rbtree_cmp(
        data1: *const libc::c_void,
        data2: *const libc::c_void,
    ) -> libc::c_int;
    fn mln_rbtree_iterate(
        t: *mut mln_rbtree_t,
        handler: rbtree_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_hash_free(h: *mut mln_hash_t, flg: mln_hash_flag_t);
    fn mln_pg_calc_info_destroy(pci: *mut mln_pg_calc_info_s);
    fn mln_pg_goto(pci: *mut mln_pg_calc_info_s) -> libc::c_int;
    fn mln_pg_calc_info_init(
        pci: *mut mln_pg_calc_info_s,
        first_input: *mut mln_pg_token_t,
        rule: *mut mln_pg_rule_t,
        nr_rule: mln_u32_t,
    ) -> libc::c_int;
    fn mln_pg_calc_follow(
        map: *mut libc::c_int,
        r: *mut mln_pg_rule_t,
        nr_rule: mln_u32_t,
    ) -> libc::c_int;
    fn mln_pg_calc_first(
        map: *mut libc::c_int,
        r: *mut mln_pg_rule_t,
        nr_rule: mln_u32_t,
    ) -> libc::c_int;
    fn mln_pg_calc_nullable(
        map: *mut libc::c_int,
        r: *mut mln_pg_rule_t,
        nr_rule: mln_u32_t,
    ) -> libc::c_int;
    fn mln_rbtree_search(
        t: *mut mln_rbtree_t,
        key: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_insert(t: *mut mln_rbtree_t, node: *mut mln_rbtree_node_t);
    fn mln_pg_token_free(token: *mut libc::c_void);
    fn mln_rbtree_node_new(
        t: *mut mln_rbtree_t,
        data: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_pg_token_new(
        token: *mut mln_string_t,
        nr_rule: mln_u32_t,
    ) -> *mut mln_pg_token_t;
    fn mln_hash_insert(
        h: *mut mln_hash_t,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_hash_search(h: *mut mln_hash_t, key: *mut libc::c_void) -> *mut libc::c_void;
    fn mln_stack_pop(st: *mut mln_stack_t) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn mln_rbtree_node_free(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_delete(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_pg_map_hash_free(data: *mut libc::c_void);
    fn mln_pg_map_hash_cmp(
        h: *mut mln_hash_t,
        key1: *mut libc::c_void,
        key2: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_pg_map_hash_calc(h: *mut mln_hash_t, key: *mut libc::c_void) -> mln_u64_t;
    fn mln_hash_new_fast(
        hash: hash_calc_handler,
        cmp: hash_cmp_handler,
        key_freer: hash_free_handler,
        val_freer: hash_free_handler,
        base_len: mln_u64_t,
        expandable: mln_u32_t,
        calc_prime: mln_u32_t,
    ) -> *mut mln_hash_t;
    fn mln_stack_destroy(st: *mut mln_stack_t);
    fn mln_queue_init(qlen: mln_uauto_t, free_handler: queue_free) -> *mut mln_queue_t;
    fn mln_stack_init(
        free_handler: stack_free,
        copy_handler: stack_copy,
    ) -> *mut mln_stack_t;
    fn mln_queue_destroy(q: *mut mln_queue_t);
    fn mln_queue_append(q: *mut mln_queue_t, data: *mut libc::c_void) -> libc::c_int;
    fn mln_queue_free_index(q: *mut mln_queue_t, index: mln_uauto_t);
    fn mln_queue_search(q: *mut mln_queue_t, index: mln_uauto_t) -> *mut libc::c_void;
    fn mln_stack_dup(st: *mut mln_stack_t, udata: *mut libc::c_void) -> *mut mln_stack_t;
    fn mln_queue_iterate(
        q: *mut mln_queue_t,
        handler: queue_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn mln_stack_push(st: *mut mln_stack_t, data: *mut libc::c_void) -> libc::c_int;
    fn mln_queue_remove(q: *mut mln_queue_t);
    fn mln_queue_get(q: *mut mln_queue_t) -> *mut libc::c_void;
    fn mln_lex_strerror(lex: *mut mln_lex_t) -> *mut libc::c_char;
    fn mln_lex_destroy(lex: *mut mln_lex_t);
    fn mln_lex_init(attr: *mut mln_lex_attr) -> *mut mln_lex_t;
    fn mln_lex_macro_new(
        pool: *mut mln_alloc_t,
        key: *mut mln_string_t,
        val: *mut mln_string_t,
    ) -> *mut mln_lex_macro_t;
    fn mln_lex_push_input_buf_stream(
        lex: *mut mln_lex_t,
        buf: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_check_file_loop(
        lex: *mut mln_lex_t,
        path: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_push_input_file_stream(
        lex: *mut mln_lex_t,
        path: *mut mln_string_t,
    ) -> libc::c_int;
    fn mln_lex_preprocess_data_new(
        pool: *mut mln_alloc_t,
    ) -> *mut mln_lex_preprocess_data_t;
    fn mln_lex_preprocess_data_free(lpd: *mut mln_lex_preprocess_data_t);
    fn mln_lex_condition_test(lex: *mut mln_lex_t) -> libc::c_int;
    fn mln_lex_input_free(in_0: *mut libc::c_void);
    fn mln_path_melang_lib() -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_stm_s {
    pub file: *mut mln_string_t,
    pub line: mln_u64_t,
    pub type_0: mln_lang_stm_type_t,
    pub data: C2RustUnnamed,
    pub next: *mut mln_lang_stm_s,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub data: C2RustUnnamed_0,
    pub jump: *mut libc::c_void,
    pub jump_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub right: C2RustUnnamed_1,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub right: C2RustUnnamed_2,
    pub next: *mut mln_lang_locate_t,
    pub jump: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub data: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub data: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub data: C2RustUnnamed_5,
    pub next: *mut mln_lang_setstm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_assign_tmp_s {
    pub op: mln_lang_assign_op_t,
    pub assign: *mut mln_lang_assign_t,
}
pub type mln_lang_assign_tmp_t = mln_lang_assign_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_logiclow_tmp_s {
    pub op: mln_lang_logiclow_op_t,
    pub logiclow: *mut mln_lang_logiclow_t,
}
pub type mln_lang_logiclow_tmp_t = mln_lang_logiclow_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_logichigh_tmp_s {
    pub op: mln_lang_logichigh_op_t,
    pub logichigh: *mut mln_lang_logichigh_t,
}
pub type mln_lang_logichigh_tmp_t = mln_lang_logichigh_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_relativelow_tmp_s {
    pub op: mln_lang_relativelow_op_t,
    pub relativelow: *mut mln_lang_relativelow_t,
}
pub type mln_lang_relativelow_tmp_t = mln_lang_relativelow_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_relativehigh_tmp_s {
    pub op: mln_lang_relativehigh_op_t,
    pub relativehigh: *mut mln_lang_relativehigh_t,
}
pub type mln_lang_relativehigh_tmp_t = mln_lang_relativehigh_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_move_tmp_s {
    pub op: mln_lang_move_op_t,
    pub move_0: *mut mln_lang_move_t,
}
pub type mln_lang_move_tmp_t = mln_lang_move_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_addsub_tmp_s {
    pub op: mln_lang_addsub_op_t,
    pub addsub: *mut mln_lang_addsub_t,
}
pub type mln_lang_addsub_tmp_t = mln_lang_addsub_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_muldiv_tmp_s {
    pub op: mln_lang_muldiv_op_t,
    pub muldiv: *mut mln_lang_muldiv_t,
}
pub type mln_lang_muldiv_tmp_t = mln_lang_muldiv_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_suffix_tmp_s {
    pub op: mln_lang_suffix_op_t,
}
pub type mln_lang_suffix_tmp_t = mln_lang_suffix_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_locate_tmp_s {
    pub op: mln_lang_locate_op_t,
    pub locate: C2RustUnnamed_6,
    pub next: *mut mln_lang_locate_tmp_t,
}
pub type mln_lang_locate_tmp_t = mln_lang_locate_tmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub exp: *mut mln_lang_exp_t,
    pub id: *mut mln_string_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_production_t {
    pub production: *mut libc::c_char,
    pub func: semantic_func,
}
pub type semantic_func = Option::<
    unsafe extern "C" fn(
        *mut mln_factor_t,
        *mut *mut mln_factor_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type mln_factor_t = mln_factor_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_factor_s {
    pub data: *mut libc::c_void,
    pub data_type: factor_data_type,
    pub nonterm_free_handler: nonterm_free,
    pub cur_state: mln_sauto_t,
    pub token_type: libc::c_int,
    pub line: mln_u32_t,
    pub file: *mut mln_string_t,
}
pub type nonterm_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type factor_data_type = libc::c_uint;
pub const M_P_NONTERM: factor_data_type = 1;
pub const M_P_TERM: factor_data_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_struct_t {
    pub text: *mut mln_string_t,
    pub line: mln_u32_t,
    pub type_0: mln_lang_enum,
    pub file: *mut mln_string_t,
}
pub type mln_lang_enum = libc::c_uint;
pub const LANG_TK_RMOVE: mln_lang_enum = 76;
pub const LANG_TK_LMOVE: mln_lang_enum = 75;
pub const LANG_TK_DECR: mln_lang_enum = 74;
pub const LANG_TK_INC: mln_lang_enum = 73;
pub const LANG_TK_GREATEREQ: mln_lang_enum = 72;
pub const LANG_TK_LESSEQ: mln_lang_enum = 71;
pub const LANG_TK_NONEQUAL: mln_lang_enum = 70;
pub const LANG_TK_DEQUAL: mln_lang_enum = 69;
pub const LANG_TK_LOWAND: mln_lang_enum = 68;
pub const LANG_TK_LOWOR: mln_lang_enum = 67;
pub const LANG_TK_MODEQ: mln_lang_enum = 66;
pub const LANG_TK_XOREQ: mln_lang_enum = 65;
pub const LANG_TK_ANTIEQ: mln_lang_enum = 64;
pub const LANG_TK_ANDEQ: mln_lang_enum = 63;
pub const LANG_TK_OREQ: mln_lang_enum = 62;
pub const LANG_TK_DIVEQ: mln_lang_enum = 61;
pub const LANG_TK_MULEQ: mln_lang_enum = 60;
pub const LANG_TK_RMOVEQ: mln_lang_enum = 59;
pub const LANG_TK_LMOVEQ: mln_lang_enum = 58;
pub const LANG_TK_SUBEQ: mln_lang_enum = 57;
pub const LANG_TK_PLUSEQ: mln_lang_enum = 56;
pub const LANG_TK_STRING: mln_lang_enum = 55;
pub const LANG_TK_FI: mln_lang_enum = 54;
pub const LANG_TK_DEFAULT: mln_lang_enum = 53;
pub const LANG_TK_CASE: mln_lang_enum = 52;
pub const LANG_TK_SWITCH: mln_lang_enum = 51;
pub const LANG_TK_FALSE: mln_lang_enum = 50;
pub const LANG_TK_TRUE: mln_lang_enum = 49;
pub const LANG_TK_NIL: mln_lang_enum = 48;
pub const LANG_TK_GOTO: mln_lang_enum = 47;
pub const LANG_TK_RETURN: mln_lang_enum = 46;
pub const LANG_TK_BREAK: mln_lang_enum = 45;
pub const LANG_TK_CONTINUE: mln_lang_enum = 44;
pub const LANG_TK_FOR: mln_lang_enum = 43;
pub const LANG_TK_WHILE: mln_lang_enum = 42;
pub const LANG_TK_ELSE: mln_lang_enum = 41;
pub const LANG_TK_IF: mln_lang_enum = 40;
pub const LANG_TK_KEYWORD_BEGIN: mln_lang_enum = 39;
pub const LANG_TK_DASH: mln_lang_enum = 38;
pub const LANG_TK_RBRACE: mln_lang_enum = 37;
pub const LANG_TK_VERTL: mln_lang_enum = 36;
pub const LANG_TK_LBRACE: mln_lang_enum = 35;
pub const LANG_TK_FULSTP: mln_lang_enum = 34;
pub const LANG_TK_UNDER: mln_lang_enum = 33;
pub const LANG_TK_XOR: mln_lang_enum = 32;
pub const LANG_TK_RSQUAR: mln_lang_enum = 31;
pub const LANG_TK_BSLASH: mln_lang_enum = 30;
pub const LANG_TK_LSQUAR: mln_lang_enum = 29;
pub const LANG_TK_AT: mln_lang_enum = 28;
pub const LANG_TK_QUES: mln_lang_enum = 27;
pub const LANG_TK_RAGL: mln_lang_enum = 26;
pub const LANG_TK_EQUAL: mln_lang_enum = 25;
pub const LANG_TK_LAGL: mln_lang_enum = 24;
pub const LANG_TK_SEMIC: mln_lang_enum = 23;
pub const LANG_TK_COLON: mln_lang_enum = 22;
pub const LANG_TK_SLASH: mln_lang_enum = 21;
pub const LANG_TK_PERIOD: mln_lang_enum = 20;
pub const LANG_TK_SUB: mln_lang_enum = 19;
pub const LANG_TK_COMMA: mln_lang_enum = 18;
pub const LANG_TK_PLUS: mln_lang_enum = 17;
pub const LANG_TK_AST: mln_lang_enum = 16;
pub const LANG_TK_RPAR: mln_lang_enum = 15;
pub const LANG_TK_LPAR: mln_lang_enum = 14;
pub const LANG_TK_SGLQ: mln_lang_enum = 13;
pub const LANG_TK_AMP: mln_lang_enum = 12;
pub const LANG_TK_PERC: mln_lang_enum = 11;
pub const LANG_TK_DOLL: mln_lang_enum = 10;
pub const LANG_TK_NUMS: mln_lang_enum = 9;
pub const LANG_TK_DBLQ: mln_lang_enum = 8;
pub const LANG_TK_EXCL: mln_lang_enum = 7;
pub const LANG_TK_SPACE: mln_lang_enum = 6;
pub const LANG_TK_ID: mln_lang_enum = 5;
pub const LANG_TK_REAL: mln_lang_enum = 4;
pub const LANG_TK_HEX: mln_lang_enum = 3;
pub const LANG_TK_DEC: mln_lang_enum = 2;
pub const LANG_TK_OCT: mln_lang_enum = 1;
pub const LANG_TK_EOF: mln_lang_enum = 0;
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_shift_tbl_t {
    pub tbl: *mut *mut mln_shift_t,
    pub nr_state: mln_sauto_t,
    pub type_val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_shift_t {
    pub index: mln_u64_t,
    pub type_0: mln_u32_t,
    pub rule_index: mln_u32_t,
    pub nr_args: mln_u32_t,
    pub left_type: mln_s32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_preprocess_attr {
    pub map_tbl: *mut mln_hash_t,
    pub token_tree: *mut mln_rbtree_t,
    pub prod_tbl: *mut mln_production_t,
    pub type_array: *mut mln_lang_type_t,
    pub rule_tbl: *mut mln_pg_rule_t,
    pub nr_prod: mln_u32_t,
    pub nr_type: mln_u32_t,
    pub type_val: libc::c_int,
    pub terminal_type_val: libc::c_int,
    pub env: *mut mln_string_t,
}
pub type mln_pg_rule_t = mln_pg_rule_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_rule_s {
    pub func: semantic_func,
    pub left: *mut mln_pg_token_t,
    pub rights: *mut *mut mln_pg_token_t,
    pub nr_right: mln_u32_t,
}
pub type mln_pg_token_t = mln_pg_token_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_pg_token_s {
    pub token: *mut mln_string_t,
    pub first_set: *mut mln_rbtree_t,
    pub follow_set: *mut mln_rbtree_t,
    pub right_rule_index: *mut mln_u32_t,
    pub left_rule_index: *mut mln_u32_t,
    pub type_0: libc::c_int,
    #[bitfield(name = "is_nonterminal", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "is_nullable", ty = "mln_u32_t", bits = "1..=1")]
    pub is_nonterminal_is_nullable: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_type_t {
    pub type_0: mln_lang_enum,
    pub type_str: *mut libc::c_char,
}
pub type mln_hash_t = mln_hash_s;
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
pub type hash_calc_handler = Option::<
    unsafe extern "C" fn(*mut mln_hash_t, *mut libc::c_void) -> mln_u64_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_calc_info_s {
    pub tree: *mut mln_rbtree_t,
    pub head: *mut mln_pg_state_t,
    pub tail: *mut mln_pg_state_t,
    pub id_counter: mln_sauto_t,
    pub first_input: *mut mln_pg_token_t,
    pub rule: *mut mln_pg_rule_t,
    pub nr_rule: mln_u32_t,
}
pub type mln_pg_state_t = mln_pg_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_state_s {
    pub id: mln_sauto_t,
    pub input: *mut mln_pg_token_t,
    pub head: *mut mln_pg_item_t,
    pub tail: *mut mln_pg_item_t,
    pub prev: *mut mln_pg_state_s,
    pub next: *mut mln_pg_state_s,
    pub q_prev: *mut mln_pg_state_s,
    pub q_next: *mut mln_pg_state_s,
    pub nr_item: mln_u64_t,
}
pub type mln_pg_item_t = mln_pg_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_item_s {
    pub prev: *mut mln_pg_item_s,
    pub next: *mut mln_pg_item_s,
    pub lookahead_set: *mut mln_rbtree_t,
    pub read: *mut mln_pg_token_t,
    pub goto_id: mln_sauto_t,
    pub rule: *mut mln_pg_rule_t,
    pub pos: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lang_reduce_info {
    pub sh: *mut mln_shift_t,
    pub item: *mut mln_pg_item_t,
    pub rule: *mut mln_pg_rule_t,
    pub state: *mut mln_pg_state_t,
    pub failed: *mut libc::c_int,
}
pub type rbtree_iterate_handler = Option::<
    unsafe extern "C" fn(*mut mln_rbtree_node_t, *mut libc::c_void) -> libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_spechar_t {
    pub sc: libc::c_char,
    pub handler: lex_hook,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_lex_keyword_t {
    pub keyword: *mut mln_string_t,
    pub val: mln_uauto_t,
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
pub struct mln_lex_macro_t {
    pub key: *mut mln_string_t,
    pub val: *mut mln_string_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_preprocess_handler_t {
    pub command: mln_string_t,
    pub handler: lex_hook,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_rbtree_attr {
    pub pool: *mut libc::c_void,
    pub pool_alloc: rbtree_pool_alloc_handler,
    pub pool_free: rbtree_pool_free_handler,
    pub cmp: rbtree_cmp,
    pub data_free: rbtree_free_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_parse_attr {
    pub pool: *mut mln_alloc_t,
    pub prod_tbl: *mut mln_production_t,
    pub lex: *mut mln_lex_t,
    pub pg_data: *mut libc::c_void,
    pub udata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_parser_t {
    pub cur_stack: *mut mln_stack_t,
    pub cur_la: *mut mln_factor_t,
    pub cur_state: mln_sauto_t,
    pub cur_reduce: mln_sauto_t,
    pub old_stack: *mut mln_stack_t,
    pub old_la: *mut mln_factor_t,
    pub old_state: mln_sauto_t,
    pub old_reduce: mln_sauto_t,
    pub err_stack: *mut mln_stack_t,
    pub err_la: *mut mln_factor_t,
    pub err_state: mln_sauto_t,
    pub err_reduce: mln_sauto_t,
    pub cur_queue: *mut mln_queue_t,
    pub err_queue: *mut mln_queue_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_queue_t {
    pub head: *mut *mut libc::c_void,
    pub tail: *mut *mut libc::c_void,
    pub queue: *mut *mut libc::c_void,
    pub qlen: mln_uauto_t,
    pub nr_element: mln_uauto_t,
    pub free_handler: queue_free,
}
pub type queue_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_sys_parse_attr {
    pub pool: *mut mln_alloc_t,
    pub p: *mut mln_parser_t,
    pub lex: *mut mln_lex_t,
    pub tbl: *mut mln_pg_shift_tbl_t,
    pub prod_tbl: *mut mln_production_t,
    pub udata: *mut libc::c_void,
    pub type_0: libc::c_int,
    pub done: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_err_queue_s {
    pub pool: *mut mln_alloc_t,
    pub q: *mut mln_queue_t,
    pub index: mln_uauto_t,
    pub pos: mln_uauto_t,
    pub opr: libc::c_int,
    pub ctype: libc::c_int,
}
pub type queue_iterate_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
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
                    current_block = 15983695695075294637;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                15983695695075294637 => {}
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
        current_block_8 = 1209868021705312628;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 1209868021705312628;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        1209868021705312628 => {
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
    let fresh1 = (*lex).result_pos;
    (*lex).result_pos = ((*lex).result_pos).offset(1);
    *fresh1 = c as mln_u8_t;
    return 0 as libc::c_int;
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
#[inline]
unsafe extern "C" fn mln_lang_process_spec_char(
    mut lex: *mut mln_lex_t,
    mut c: libc::c_char,
) -> *mut mln_lang_struct_t {
    let mut i: mln_s32_t = 0;
    let mut end: mln_s32_t = (::core::mem::size_of::<[mln_spechar_t; 32]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<mln_spechar_t>() as libc::c_ulong)
        as mln_s32_t;
    i = 0 as libc::c_int;
    while i < end {
        if c as libc::c_int == mln_lang_handlers[i as usize].sc as libc::c_int {
            return (mln_lang_handlers[i as usize].handler)
                .expect(
                    "non-null function pointer",
                )(lex, mln_lang_handlers[i as usize].data) as *mut mln_lang_struct_t;
        }
        i += 1;
        i;
    }
    (*lex).error = 2 as libc::c_int;
    return 0 as *mut mln_lang_struct_t;
}
#[inline]
unsafe extern "C" fn mln_lang_process_keywords(
    mut lex: *mut mln_lex_t,
) -> *mut mln_lang_struct_t {
    if ((*lex).keywords).is_null() {
        return mln_lang_new(lex, LANG_TK_ID);
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
        return mln_lang_new(
            lex,
            (LANG_TK_KEYWORD_BEGIN as libc::c_int as libc::c_ulong)
                .wrapping_add((*plk).val)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as mln_lang_enum,
        );
    }
    return mln_lang_new(lex, LANG_TK_ID);
}
#[no_mangle]
pub static mut mln_lang_token_type_array: [mln_lang_type_t; 77] = [
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_EOF,
            type_str: b"LANG_TK_EOF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_OCT,
            type_str: b"LANG_TK_OCT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DEC,
            type_str: b"LANG_TK_DEC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_HEX,
            type_str: b"LANG_TK_HEX\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_REAL,
            type_str: b"LANG_TK_REAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_ID,
            type_str: b"LANG_TK_ID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SPACE,
            type_str: b"LANG_TK_SPACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_EXCL,
            type_str: b"LANG_TK_EXCL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DBLQ,
            type_str: b"LANG_TK_DBLQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_NUMS,
            type_str: b"LANG_TK_NUMS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DOLL,
            type_str: b"LANG_TK_DOLL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_PERC,
            type_str: b"LANG_TK_PERC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_AMP,
            type_str: b"LANG_TK_AMP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SGLQ,
            type_str: b"LANG_TK_SGLQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LPAR,
            type_str: b"LANG_TK_LPAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RPAR,
            type_str: b"LANG_TK_RPAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_AST,
            type_str: b"LANG_TK_AST\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_PLUS,
            type_str: b"LANG_TK_PLUS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_COMMA,
            type_str: b"LANG_TK_COMMA\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SUB,
            type_str: b"LANG_TK_SUB\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_PERIOD,
            type_str: b"LANG_TK_PERIOD\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SLASH,
            type_str: b"LANG_TK_SLASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_COLON,
            type_str: b"LANG_TK_COLON\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SEMIC,
            type_str: b"LANG_TK_SEMIC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LAGL,
            type_str: b"LANG_TK_LAGL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_EQUAL,
            type_str: b"LANG_TK_EQUAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RAGL,
            type_str: b"LANG_TK_RAGL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_QUES,
            type_str: b"LANG_TK_QUES\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_AT,
            type_str: b"LANG_TK_AT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LSQUAR,
            type_str: b"LANG_TK_LSQUAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_BSLASH,
            type_str: b"LANG_TK_BSLASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RSQUAR,
            type_str: b"LANG_TK_RSQUAR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_XOR,
            type_str: b"LANG_TK_XOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_UNDER,
            type_str: b"LANG_TK_UNDER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_FULSTP,
            type_str: b"LANG_TK_FULSTP\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LBRACE,
            type_str: b"LANG_TK_LBRACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_VERTL,
            type_str: b"LANG_TK_VERTL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RBRACE,
            type_str: b"LANG_TK_RBRACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DASH,
            type_str: b"LANG_TK_DASH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_KEYWORD_BEGIN,
            type_str: b"LANG_TK_KEYWORD_BEGIN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_IF,
            type_str: b"LANG_TK_IF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_ELSE,
            type_str: b"LANG_TK_ELSE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_WHILE,
            type_str: b"LANG_TK_WHILE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_FOR,
            type_str: b"LANG_TK_FOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_CONTINUE,
            type_str: b"LANG_TK_CONTINUE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_BREAK,
            type_str: b"LANG_TK_BREAK\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RETURN,
            type_str: b"LANG_TK_RETURN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_GOTO,
            type_str: b"LANG_TK_GOTO\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_NIL,
            type_str: b"LANG_TK_NIL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_TRUE,
            type_str: b"LANG_TK_TRUE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_FALSE,
            type_str: b"LANG_TK_FALSE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SWITCH,
            type_str: b"LANG_TK_SWITCH\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_CASE,
            type_str: b"LANG_TK_CASE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DEFAULT,
            type_str: b"LANG_TK_DEFAULT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_FI,
            type_str: b"LANG_TK_FI\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_STRING,
            type_str: b"LANG_TK_STRING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_PLUSEQ,
            type_str: b"LANG_TK_PLUSEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_SUBEQ,
            type_str: b"LANG_TK_SUBEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LMOVEQ,
            type_str: b"LANG_TK_LMOVEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RMOVEQ,
            type_str: b"LANG_TK_RMOVEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_MULEQ,
            type_str: b"LANG_TK_MULEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DIVEQ,
            type_str: b"LANG_TK_DIVEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_OREQ,
            type_str: b"LANG_TK_OREQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_ANDEQ,
            type_str: b"LANG_TK_ANDEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_ANTIEQ,
            type_str: b"LANG_TK_ANTIEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_XOREQ,
            type_str: b"LANG_TK_XOREQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_MODEQ,
            type_str: b"LANG_TK_MODEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LOWOR,
            type_str: b"LANG_TK_LOWOR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LOWAND,
            type_str: b"LANG_TK_LOWAND\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DEQUAL,
            type_str: b"LANG_TK_DEQUAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_NONEQUAL,
            type_str: b"LANG_TK_NONEQUAL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LESSEQ,
            type_str: b"LANG_TK_LESSEQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_GREATEREQ,
            type_str: b"LANG_TK_GREATEREQ\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_INC,
            type_str: b"LANG_TK_INC\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_DECR,
            type_str: b"LANG_TK_DECR\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_LMOVE,
            type_str: b"LANG_TK_LMOVE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = mln_lang_type_t {
            type_0: LANG_TK_RMOVE,
            type_str: b"LANG_TK_RMOVE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
];
static mut mln_lang_preprocess_handlers: [mln_preprocess_handler_t; 6] = [mln_preprocess_handler_t {
    command: mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    },
    handler: None,
}; 6];
static mut mln_lang_handlers: [mln_spechar_t; 32] = unsafe {
    [
        {
            let mut init = mln_spechar_t {
                sc: '!' as i32 as libc::c_char,
                handler: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut mln_lex_t,
                            *mut libc::c_void,
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_excl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_dblq_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_nums_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_doll_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_perc_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_amp_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_sglq_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lpar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_rpar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_ast_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_plus_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_comma_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_sub_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_period_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_slash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_colon_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_semic_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lagl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_equal_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_ragl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_ques_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_at_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lsquar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_bslash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_rsquar_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_xor_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_under_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_fulstp_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lbrace_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_vertl_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_rbrace_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_dash_default_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
                    ),
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn mln_lang_lex_dup(
    mut pool: *mut mln_alloc_t,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut src: *mut mln_lang_struct_t = ptr as *mut mln_lang_struct_t;
    let mut dest: *mut mln_lang_struct_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_struct_t>() as libc::c_ulong,
    ) as *mut mln_lang_struct_t;
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
unsafe extern "C" fn mln_lang_under_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_UNDER);
}
unsafe extern "C" fn mln_lang_ragl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_RAGL);
}
unsafe extern "C" fn mln_lang_parser_generate(
    mut prod_tbl_0: *mut mln_production_t,
    mut nr_prod: mln_u32_t,
    mut env: *mut mln_string_t,
) -> *mut libc::c_void {
    let mut pattr: mln_lang_preprocess_attr = mln_lang_preprocess_attr {
        map_tbl: 0 as *mut mln_hash_t,
        token_tree: 0 as *mut mln_rbtree_t,
        prod_tbl: 0 as *mut mln_production_t,
        type_array: 0 as *mut mln_lang_type_t,
        rule_tbl: 0 as *mut mln_pg_rule_t,
        nr_prod: 0,
        nr_type: 0,
        type_val: 0,
        terminal_type_val: 0,
        env: 0 as *mut mln_string_t,
    };
    pattr.map_tbl = 0 as *mut mln_hash_t;
    pattr.token_tree = 0 as *mut mln_rbtree_t;
    pattr.prod_tbl = prod_tbl_0;
    pattr.type_array = mln_lang_token_type_array.as_mut_ptr();
    pattr
        .rule_tbl = calloc(
        nr_prod as libc::c_ulong,
        ::core::mem::size_of::<mln_pg_rule_t>() as libc::c_ulong,
    ) as *mut mln_pg_rule_t;
    if (pattr.rule_tbl).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_parser_generate\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    pattr.nr_prod = nr_prod;
    pattr
        .nr_type = (::core::mem::size_of::<[mln_lang_type_t; 77]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<mln_lang_type_t>() as libc::c_ulong)
        as mln_u32_t;
    pattr.type_val = -(1 as libc::c_int);
    pattr.terminal_type_val = -(1 as libc::c_int);
    pattr.env = env;
    if mln_lang_preprocess(&mut pattr) < 0 as libc::c_int {
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    let mut map: *mut libc::c_int = malloc(
        (nr_prod as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int as *mut libc::c_int;
    if map.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_parser_generate\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    if mln_pg_calc_nullable(map, pattr.rule_tbl, nr_prod) < 0 as libc::c_int {
        free(map as *mut libc::c_void);
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    if mln_pg_calc_first(map, pattr.rule_tbl, nr_prod) < 0 as libc::c_int {
        free(map as *mut libc::c_void);
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    if mln_pg_calc_follow(map, pattr.rule_tbl, nr_prod) < 0 as libc::c_int {
        free(map as *mut libc::c_void);
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    free(map as *mut libc::c_void);
    let mut first_input: mln_pg_token_t = {
        let mut init = mln_pg_token_s {
            is_nonterminal_is_nullable: [0; 1],
            c2rust_padding: [0; 3],
            token: 0 as *mut mln_string_t,
            first_set: 0 as *mut mln_rbtree_t,
            follow_set: 0 as *mut mln_rbtree_t,
            right_rule_index: 0 as *mut mln_u32_t,
            left_rule_index: 0 as *mut mln_u32_t,
            type_0: LANG_TK_EOF as libc::c_int,
        };
        init.set_is_nonterminal(0 as libc::c_int as mln_u32_t);
        init.set_is_nullable(1 as libc::c_int as mln_u32_t);
        init
    };
    let mut pci: mln_pg_calc_info_s = mln_pg_calc_info_s {
        tree: 0 as *mut mln_rbtree_t,
        head: 0 as *mut mln_pg_state_t,
        tail: 0 as *mut mln_pg_state_t,
        id_counter: 0,
        first_input: 0 as *mut mln_pg_token_t,
        rule: 0 as *mut mln_pg_rule_t,
        nr_rule: 0,
    };
    if mln_pg_calc_info_init(&mut pci, &mut first_input, pattr.rule_tbl, nr_prod)
        < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_parser_generate\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    if mln_pg_goto(&mut pci) < 0 as libc::c_int {
        mln_pg_calc_info_destroy(&mut pci);
        mln_lang_preprocess_attr_free(&mut pattr);
        return 0 as *mut libc::c_void;
    }
    let mut shift_tbl: *mut mln_pg_shift_tbl_t = mln_lang_build_shift_tbl(
        &mut pci,
        &mut pattr,
    );
    mln_pg_calc_info_destroy(&mut pci);
    mln_lang_preprocess_attr_free(&mut pattr);
    return shift_tbl as *mut libc::c_void;
}
unsafe extern "C" fn mln_lang_build_shift_tbl(
    mut pci: *mut mln_pg_calc_info_s,
    mut attr: *mut mln_lang_preprocess_attr,
) -> *mut mln_pg_shift_tbl_t {
    let mut stbl: *mut mln_pg_shift_tbl_t = malloc(
        ::core::mem::size_of::<mln_pg_shift_tbl_t>() as libc::c_ulong,
    ) as *mut mln_pg_shift_tbl_t;
    if stbl.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_build_shift_tbl\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut mln_pg_shift_tbl_t;
    }
    (*stbl).nr_state = (*pci).id_counter;
    (*stbl).type_val = (*attr).terminal_type_val;
    (*stbl)
        .tbl = calloc(
        (*stbl).nr_state as libc::c_ulong,
        ::core::mem::size_of::<*mut mln_shift_t>() as libc::c_ulong,
    ) as *mut *mut mln_shift_t;
    if ((*stbl).tbl).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_build_shift_tbl\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_lang_pg_data_free(stbl as *mut libc::c_void);
        return 0 as *mut mln_pg_shift_tbl_t;
    }
    let mut s: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    let mut sh: *mut mln_shift_t = 0 as *mut mln_shift_t;
    let mut it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut index: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut info: mln_lang_reduce_info = mln_lang_reduce_info {
        sh: 0 as *mut mln_shift_t,
        item: 0 as *mut mln_pg_item_t,
        rule: 0 as *mut mln_pg_rule_t,
        state: 0 as *mut mln_pg_state_t,
        failed: 0 as *mut libc::c_int,
    };
    s = (*pci).head;
    while !s.is_null() {
        let ref mut fresh3 = *((*stbl).tbl).offset((*s).id as isize);
        *fresh3 = calloc(
            ((*attr).type_val + 1 as libc::c_int) as libc::c_ulong,
            ::core::mem::size_of::<mln_shift_t>() as libc::c_ulong,
        ) as *mut mln_shift_t;
        if (*((*stbl).tbl).offset((*s).id as isize)).is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_build_shift_tbl\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            mln_lang_pg_data_free(stbl as *mut libc::c_void);
            return 0 as *mut mln_pg_shift_tbl_t;
        }
        sh = *((*stbl).tbl).offset((*s).id as isize);
        it = (*s).head;
        while !it.is_null() {
            if (*it).pos == (*(*it).rule).nr_right {
                info.sh = sh;
                info.item = it;
                info.rule = (*attr).rule_tbl;
                info.state = s;
                info.failed = &mut failed;
                if mln_rbtree_iterate(
                    (*it).lookahead_set,
                    Some(
                        mln_lang_reduce_iterate_handler
                            as unsafe extern "C" fn(
                                *mut mln_rbtree_node_t,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    &mut info as *mut mln_lang_reduce_info as *mut libc::c_void,
                ) < 0 as libc::c_int
                {
                    mln_lang_pg_data_free(stbl as *mut libc::c_void);
                    return 0 as *mut mln_pg_shift_tbl_t;
                }
            } else {
                index = (**((*(*it).rule).rights).offset((*it).pos as isize)).type_0;
                if index == LANG_TK_EOF as libc::c_int {
                    type_0 = 3 as libc::c_int;
                } else {
                    type_0 = 1 as libc::c_int;
                }
                if (*sh.offset(index as isize)).type_0
                    != 0 as libc::c_int as libc::c_uint
                    && ((*sh.offset(index as isize)).type_0 != type_0 as libc::c_uint
                        || (*sh.offset(index as isize)).index
                            != (*it).goto_id as libc::c_ulong)
                {
                    if (*sh.offset(index as isize)).type_0
                        == 3 as libc::c_int as libc::c_uint
                        || (*sh.offset(index as isize)).type_0
                            == 2 as libc::c_int as libc::c_uint
                    {
                        _mln_sys_log(
                            error,
                            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 25],
                                &[libc::c_char; 25],
                            >(b"mln_lang_build_shift_tbl\0"))
                                .as_ptr(),
                            90 as libc::c_int,
                            b"State:%l token:[%S] Shift-Reduce conflict.\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            (*s).id,
                            (**((*(*it).rule).rights).offset((*it).pos as isize)).token,
                        );
                    } else {
                        _mln_sys_log(
                            error,
                            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 25],
                                &[libc::c_char; 25],
                            >(b"mln_lang_build_shift_tbl\0"))
                                .as_ptr(),
                            90 as libc::c_int,
                            b"State:%l token:[%S] Shift-Shift conflict.\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*s).id,
                            (**((*(*it).rule).rights).offset((*it).pos as isize)).token,
                        );
                    }
                    failed = 1 as libc::c_int;
                }
                (*sh.offset(index as isize)).index = (*it).goto_id as mln_u64_t;
                (*sh.offset(index as isize)).type_0 = type_0 as mln_u32_t;
                (*sh.offset(index as isize))
                    .rule_index = ((*it).rule).offset_from((*attr).rule_tbl)
                    as libc::c_long as mln_u32_t;
                (*sh.offset(index as isize)).nr_args = (*(*it).rule).nr_right;
                (*sh.offset(index as isize)).left_type = (*(*(*it).rule).left).type_0;
            }
            it = (*it).next;
        }
        s = (*s).next;
    }
    if failed != 0 {
        mln_lang_pg_data_free(stbl as *mut libc::c_void);
        return 0 as *mut mln_pg_shift_tbl_t;
    }
    return stbl;
}
unsafe extern "C" fn mln_lang_pg_data_free(mut pg_data: *mut libc::c_void) {
    let mut tbl: *mut mln_pg_shift_tbl_t = pg_data as *mut mln_pg_shift_tbl_t;
    if tbl.is_null() {
        return;
    }
    if !((*tbl).tbl).is_null() {
        let mut i: mln_sauto_t = 0;
        i = 0 as libc::c_int as mln_sauto_t;
        while i < (*tbl).nr_state {
            if !(*((*tbl).tbl).offset(i as isize)).is_null() {
                free(*((*tbl).tbl).offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free((*tbl).tbl as *mut libc::c_void);
    }
    free(tbl as *mut libc::c_void);
}
unsafe extern "C" fn mln_lang_parser_destroy(mut p: *mut mln_parser_t) {
    if p.is_null() {
        return;
    }
    if !((*p).cur_stack).is_null() {
        mln_stack_destroy((*p).cur_stack);
    }
    if !((*p).cur_la).is_null() {
        mln_lang_factor_destroy((*p).cur_la as *mut libc::c_void);
    }
    if !((*p).old_stack).is_null() {
        mln_stack_destroy((*p).old_stack);
    }
    if !((*p).old_la).is_null() {
        mln_lang_factor_destroy((*p).old_la as *mut libc::c_void);
    }
    if !((*p).err_stack).is_null() {
        mln_stack_destroy((*p).err_stack);
    }
    if !((*p).err_la).is_null() {
        mln_lang_factor_destroy((*p).err_la as *mut libc::c_void);
    }
    if !((*p).cur_queue).is_null() {
        mln_queue_destroy((*p).cur_queue);
    }
    if !((*p).err_queue).is_null() {
        mln_queue_destroy((*p).err_queue);
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn mln_lang_reduce_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut info: *mut mln_lang_reduce_info = udata as *mut mln_lang_reduce_info;
    let mut sh: *mut mln_shift_t = (*info).sh;
    let mut index: libc::c_int = (*tk).type_0;
    if (*sh.offset(index as isize)).type_0 != 0 as libc::c_int as libc::c_uint
        && ((*sh.offset(index as isize)).type_0 != 2 as libc::c_int as libc::c_uint
            || (*sh.offset(index as isize)).index
                != ((*(*info).item).rule).offset_from((*info).rule) as libc::c_long
                    as libc::c_ulong)
    {
        if (*sh.offset(index as isize)).type_0 == 3 as libc::c_int as libc::c_uint
            || (*sh.offset(index as isize)).type_0 == 2 as libc::c_int as libc::c_uint
        {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"mln_lang_reduce_iterate_handler\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"State:%d token[%S] Reduce-Reduce conflict.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*(*info).state).id,
                (*tk).token,
            );
        } else {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"mln_lang_reduce_iterate_handler\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"State:%d token[%S] Shift-Reduce conflict.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*(*info).state).id,
                (*tk).token,
            );
        }
        *(*info).failed = 1 as libc::c_int;
    }
    (*sh.offset(index as isize))
        .index = ((*(*info).item).rule).offset_from((*info).rule) as libc::c_long
        as mln_u64_t;
    (*sh.offset(index as isize)).type_0 = 2 as libc::c_int as mln_u32_t;
    (*sh.offset(index as isize))
        .rule_index = ((*(*info).item).rule).offset_from((*info).rule) as libc::c_long
        as mln_u32_t;
    (*sh.offset(index as isize)).nr_args = (*(*(*info).item).rule).nr_right;
    (*sh.offset(index as isize)).left_type = (*(*(*(*info).item).rule).left).type_0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_preprocess_attr_free(
    mut attr: *mut mln_lang_preprocess_attr,
) {
    if !((*attr).map_tbl).is_null() {
        mln_hash_free((*attr).map_tbl, M_HASH_F_KV);
    }
    if !((*attr).token_tree).is_null() {
        mln_rbtree_free((*attr).token_tree);
    }
    if !((*attr).rule_tbl).is_null() {
        let mut i: mln_u32_t = 0;
        i = 0 as libc::c_int as mln_u32_t;
        while i < (*attr).nr_prod {
            if !((*((*attr).rule_tbl).offset(i as isize)).rights).is_null() {
                free(
                    (*((*attr).rule_tbl).offset(i as isize)).rights as *mut libc::c_void,
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        free((*attr).rule_tbl as *mut libc::c_void);
    }
}
unsafe extern "C" fn mln_lang_preprocess(
    mut attr: *mut mln_lang_preprocess_attr,
) -> libc::c_int {
    let mut rbattr: mln_rbtree_attr = mln_rbtree_attr {
        pool: 0 as *mut libc::c_void,
        pool_alloc: None,
        pool_free: None,
        cmp: None,
        data_free: None,
    };
    let mut ret: libc::c_int = 0;
    let mut lex: *mut mln_lex_t = 0 as *mut mln_lex_t;
    let mut pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut lattr: mln_lex_attr = mln_lex_attr {
        pool: 0 as *mut mln_alloc_t,
        keywords: 0 as *mut mln_string_t,
        hooks: 0 as *mut mln_lex_hooks_t,
        preprocess_padding: [0; 4],
        type_0: 0,
        env: 0 as *mut mln_string_t,
        data: 0 as *mut mln_string_t,
    };
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut prod: *mut mln_production_t = 0 as *mut mln_production_t;
    let mut prodend: *mut mln_production_t = 0 as *mut mln_production_t;
    let mut current_block: u64;
    (*attr)
        .map_tbl = mln_hash_new_fast(
        Some(
            mln_pg_map_hash_calc
                as unsafe extern "C" fn(*mut mln_hash_t, *mut libc::c_void) -> mln_u64_t,
        ),
        Some(
            mln_pg_map_hash_cmp
                as unsafe extern "C" fn(
                    *mut mln_hash_t,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        Some(mln_pg_map_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(mln_pg_map_hash_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        31 as libc::c_int as mln_u64_t,
        1 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
    );
    if ((*attr).map_tbl).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"mln_lang_preprocess\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut str_len: mln_size_t = 0;
    let mut new_str: mln_s8ptr_t = 0 as *mut libc::c_char;
    let mut new_val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tp: *mut mln_lang_type_t = (*attr).type_array;
    let mut tpend: *mut mln_lang_type_t = ((*attr).type_array)
        .offset((*attr).nr_type as isize);
    loop {
        if !(tp < tpend) {
            current_block = 11584701595673473500;
            break;
        }
        (*attr).type_val = (*tp).type_0 as libc::c_int;
        (*attr).terminal_type_val = (*tp).type_0 as libc::c_int;
        if (mln_hash_search((*attr).map_tbl, (*tp).type_str as *mut libc::c_void))
            .is_null()
        {
            str_len = strlen((*tp).type_str);
            new_str = malloc(str_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as mln_s8ptr_t;
            if new_str.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"mln_lang_preprocess\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block = 264271642989168924;
                break;
            } else {
                memcpy(
                    new_str as *mut libc::c_void,
                    (*tp).type_str as *const libc::c_void,
                    str_len,
                );
                *new_str.offset(str_len as isize) = 0 as libc::c_int as libc::c_char;
                new_val = malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as *mut libc::c_int;
                if new_val.is_null() {
                    _mln_sys_log(
                        error,
                        b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 20],
                            &[libc::c_char; 20],
                        >(b"mln_lang_preprocess\0"))
                            .as_ptr(),
                        90 as libc::c_int,
                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    free(new_str as *mut libc::c_void);
                    current_block = 264271642989168924;
                    break;
                } else {
                    *new_val = (*tp).type_0 as libc::c_int;
                    if mln_hash_insert(
                        (*attr).map_tbl,
                        new_str as *mut libc::c_void,
                        new_val as *mut libc::c_void,
                    ) < 0 as libc::c_int
                    {
                        _mln_sys_log(
                            error,
                            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 20],
                                &[libc::c_char; 20],
                            >(b"mln_lang_preprocess\0"))
                                .as_ptr(),
                            90 as libc::c_int,
                            b"No memory.\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        free(new_val as *mut libc::c_void);
                        free(new_str as *mut libc::c_void);
                        current_block = 264271642989168924;
                        break;
                    }
                }
            }
        }
        tp = tp.offset(1);
        tp;
    }
    match current_block {
        11584701595673473500 => {
            rbattr = mln_rbtree_attr {
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
                mln_pg_token_rbtree_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            rbattr
                .data_free = Some(
                mln_pg_token_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
            (*attr).token_tree = mln_rbtree_new(&mut rbattr);
            if ((*attr).token_tree).is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"mln_lang_preprocess\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                ret = 0;
                lex = 0 as *mut mln_lex_t;
                pool = 0 as *mut mln_alloc_t;
                lattr = mln_lex_attr {
                    pool: 0 as *mut mln_alloc_t,
                    keywords: 0 as *mut mln_string_t,
                    hooks: 0 as *mut mln_lex_hooks_t,
                    preprocess_padding: [0; 4],
                    type_0: 0,
                    env: 0 as *mut mln_string_t,
                    data: 0 as *mut mln_string_t,
                };
                tmp = mln_string_t {
                    data: 0 as *mut libc::c_uchar,
                    len: 0,
                    data_ref_pool_ref_0: [0; 4],
                    c2rust_padding: [0; 4],
                };
                prod = (*attr).prod_tbl;
                prodend = ((*attr).prod_tbl).offset((*attr).nr_prod as isize);
                pool = mln_alloc_init(0 as *mut mln_alloc_t);
                if pool.is_null() {
                    _mln_sys_log(
                        error,
                        b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 20],
                            &[libc::c_char; 20],
                        >(b"mln_lang_preprocess\0"))
                            .as_ptr(),
                        90 as libc::c_int,
                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    loop {
                        if !(prod < prodend) {
                            current_block = 1134115459065347084;
                            break;
                        }
                        ({
                            tmp.data = (*prod).production as mln_u8ptr_t;
                            tmp.len = strlen((*prod).production);
                            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                            &mut tmp;
                            &mut tmp
                        });
                        lattr.pool = pool;
                        lattr.keywords = 0 as *mut mln_string_t;
                        lattr.hooks = 0 as *mut mln_lex_hooks_t;
                        lattr.set_preprocess(0 as libc::c_int as mln_u32_t);
                        lattr.type_0 = 0 as libc::c_int as mln_u32_t;
                        lattr.data = &mut tmp;
                        lattr.env = (*attr).env;
                        if lattr.preprocess() != 0 {
                            let mut lpd: *mut mln_lex_preprocess_data_t = mln_lex_preprocess_data_new(
                                lattr.pool,
                            );
                            if lpd.is_null() {
                                lex = 0 as *mut mln_lex_t;
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
                                            ) -> *mut mln_lang_struct_t,
                                        >,
                                        lex_hook,
                                    >(
                                        Some(
                                            mln_lang_nums_handler
                                                as unsafe extern "C" fn(
                                                    *mut mln_lex_t,
                                                    *mut libc::c_void,
                                                ) -> *mut mln_lang_struct_t,
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
                                            ) -> *mut mln_lang_struct_t,
                                        >,
                                        lex_hook,
                                    >(
                                        Some(
                                            mln_lang_nums_handler
                                                as unsafe extern "C" fn(
                                                    *mut mln_lex_t,
                                                    *mut libc::c_void,
                                                ) -> *mut mln_lang_struct_t,
                                        ),
                                    );
                                    (*lattr.hooks).nums_data = lpd as *mut libc::c_void;
                                }
                                lex = mln_lex_init(&mut lattr);
                                if !lex.is_null() {
                                    if !(lattr.hooks).is_null() {
                                        mln_lang_set_hooks(lex);
                                    }
                                    (*lex).preprocess_data = lpd;
                                } else {
                                    mln_lex_preprocess_data_free(lpd);
                                }
                            }
                        } else {
                            lex = mln_lex_init(&mut lattr);
                            if !lex.is_null() && !(lattr.hooks).is_null() {
                                mln_lang_set_hooks(lex);
                            }
                        }
                        if lex.is_null() {
                            _mln_sys_log(
                                error,
                                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                                (*::core::mem::transmute::<
                                    &[u8; 20],
                                    &[libc::c_char; 20],
                                >(b"mln_lang_preprocess\0"))
                                    .as_ptr(),
                                90 as libc::c_int,
                                b"No memory.\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            current_block = 11045260486439920960;
                            break;
                        } else {
                            ret = mln_lang_pg_process_token(attr, lex, prod);
                            mln_lex_destroy(lex);
                            if ret < 0 as libc::c_int {
                                current_block = 18283366945912712396;
                                break;
                            }
                            prod = prod.offset(1);
                            prod;
                        }
                    }
                    match current_block {
                        18283366945912712396 => {}
                        _ => {
                            match current_block {
                                11045260486439920960 => {
                                    mln_alloc_destroy(pool);
                                }
                                _ => {
                                    mln_alloc_destroy(pool);
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
                mln_rbtree_free((*attr).token_tree);
            }
        }
        _ => {}
    }
    mln_hash_free((*attr).map_tbl, M_HASH_F_KV);
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_lang_pg_process_token(
    mut attr: *mut mln_lang_preprocess_attr,
    mut lex: *mut mln_lex_t,
    mut prod: *mut mln_production_t,
) -> libc::c_int {
    let mut index: libc::c_int = prod.offset_from((*attr).prod_tbl) as libc::c_long
        as libc::c_int;
    let mut pgs: *mut mln_lang_struct_t = 0 as *mut mln_lang_struct_t;
    pgs = mln_lang_token(lex);
    if pgs.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"mln_lang_pg_process_token\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Get token failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mln_lex_strerror(lex),
        );
        return -(1 as libc::c_int);
    }
    if (*pgs).type_0 as libc::c_uint != LANG_TK_ID as libc::c_int as libc::c_uint {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"mln_lang_pg_process_token\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Production '%u' error.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
        mln_lang_free(pgs);
        return -(1 as libc::c_int);
    }
    let mut tk: *mut mln_pg_token_t = mln_lang_pg_create_token(
        attr,
        pgs,
        -(1 as libc::c_int),
    );
    mln_lang_free(pgs);
    if tk.is_null() {
        return -(1 as libc::c_int);
    }
    *((*tk).left_rule_index).offset(index as isize) = 1 as libc::c_int as mln_u32_t;
    let mut r: *mut mln_pg_rule_t = ((*attr).rule_tbl).offset(index as isize);
    (*r).func = (*prod).func;
    (*r).left = tk;
    pgs = mln_lang_token(lex);
    if pgs.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"mln_lang_pg_process_token\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Get token failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mln_lex_strerror(lex),
        );
        return -(1 as libc::c_int);
    }
    if (*pgs).type_0 as libc::c_uint != LANG_TK_COLON as libc::c_int as libc::c_uint {
        mln_lang_free(pgs);
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"mln_lang_pg_process_token\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Production '%u' error.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
        return -(1 as libc::c_int);
    }
    mln_lang_free(pgs);
    if mln_lang_pg_process_right(attr, lex, index, 0 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_pg_process_right(
    mut attr: *mut mln_lang_preprocess_attr,
    mut lex: *mut mln_lex_t,
    mut index: libc::c_int,
    mut cnt: libc::c_int,
) -> libc::c_int {
    let mut pgs: *mut mln_lang_struct_t = 0 as *mut mln_lang_struct_t;
    pgs = mln_lang_token(lex);
    if pgs.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"mln_lang_pg_process_right\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Get token failed. %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mln_lex_strerror(lex),
        );
        return -(1 as libc::c_int);
    }
    if (*pgs).type_0 as libc::c_uint == LANG_TK_EOF as libc::c_int as libc::c_uint {
        mln_lang_free(pgs);
        let mut r: *mut mln_pg_rule_t = ((*attr).rule_tbl).offset(index as isize);
        if cnt != 0 {
            (*r)
                .rights = malloc(
                (cnt as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut mln_pg_token_t>() as libc::c_ulong,
                    ),
            ) as *mut *mut mln_pg_token_t;
            if ((*r).rights).is_null() {
                return -(1 as libc::c_int);
            }
        }
        (*r).nr_right = cnt as mln_u32_t;
        return 0 as libc::c_int;
    }
    if mln_lang_pg_process_right(attr, lex, index, cnt + 1 as libc::c_int)
        < 0 as libc::c_int
    {
        mln_lang_free(pgs);
        return -(1 as libc::c_int);
    }
    let mut r_0: *mut mln_pg_rule_t = ((*attr).rule_tbl).offset(index as isize);
    let mut tk: *mut mln_pg_token_t = mln_lang_pg_create_token(attr, pgs, index);
    mln_lang_free(pgs);
    if tk.is_null() {
        free((*r_0).rights as *mut libc::c_void);
        (*r_0).rights = 0 as *mut *mut mln_pg_token_t;
        return -(1 as libc::c_int);
    }
    let ref mut fresh4 = *((*r_0).rights).offset(cnt as isize);
    *fresh4 = tk;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_pg_create_token(
    mut attr: *mut mln_lang_preprocess_attr,
    mut pgs: *mut mln_lang_struct_t,
    mut index: libc::c_int,
) -> *mut mln_pg_token_t {
    let mut type_val: *mut libc::c_int = 0 as *mut libc::c_int;
    type_val = mln_hash_search((*attr).map_tbl, (*(*pgs).text).data as *mut libc::c_void)
        as *mut libc::c_int;
    if type_val.is_null() {
        let mut new_str: mln_s8ptr_t = malloc(
            ((*(*pgs).text).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as mln_s8ptr_t;
        if new_str.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_pg_create_token\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut mln_pg_token_t;
        }
        memcpy(
            new_str as *mut libc::c_void,
            (*(*pgs).text).data as *const libc::c_void,
            (*(*pgs).text).len,
        );
        *new_str.offset((*(*pgs).text).len as isize) = 0 as libc::c_int as libc::c_char;
        type_val = malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as *mut libc::c_int;
        if type_val.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_pg_create_token\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            free(new_str as *mut libc::c_void);
            return 0 as *mut mln_pg_token_t;
        }
        (*attr).type_val += 1;
        *type_val = (*attr).type_val;
        if mln_hash_insert(
            (*attr).map_tbl,
            new_str as *mut libc::c_void,
            type_val as *mut libc::c_void,
        ) < 0 as libc::c_int
        {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_pg_create_token\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            free(type_val as *mut libc::c_void);
            free(new_str as *mut libc::c_void);
            return 0 as *mut mln_pg_token_t;
        }
    }
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut token: mln_pg_token_t = mln_pg_token_t {
        token: 0 as *mut mln_string_t,
        first_set: 0 as *mut mln_rbtree_t,
        follow_set: 0 as *mut mln_rbtree_t,
        right_rule_index: 0 as *mut mln_u32_t,
        left_rule_index: 0 as *mut mln_u32_t,
        type_0: 0,
        is_nonterminal_is_nullable: [0; 1],
        c2rust_padding: [0; 3],
    };
    token.type_0 = *type_val;
    let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
        (*attr).token_tree,
        &mut token as *mut mln_pg_token_t as *mut libc::c_void,
    );
    if rn == &mut (*(*attr).token_tree).nil as *mut mln_rbtree_node_t {
        tk = mln_pg_token_new((*pgs).text, (*attr).nr_prod);
        if tk.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_pg_create_token\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut mln_pg_token_t;
        }
        (*tk).type_0 = *type_val;
        if index >= 0 as libc::c_int {
            *((*tk).right_rule_index)
                .offset(index as isize) = 1 as libc::c_int as mln_u32_t;
        } else {
            (*tk).set_is_nonterminal(1 as libc::c_int as mln_u32_t);
        }
        rn = mln_rbtree_node_new((*attr).token_tree, tk as *mut libc::c_void);
        if rn.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_pg_create_token\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            mln_pg_token_free(tk as *mut libc::c_void);
            return 0 as *mut mln_pg_token_t;
        }
        mln_rbtree_insert((*attr).token_tree, rn);
    } else {
        tk = (*rn).data as *mut mln_pg_token_t;
        if index >= 0 as libc::c_int {
            *((*tk).right_rule_index)
                .offset(index as isize) = 1 as libc::c_int as mln_u32_t;
        } else {
            (*tk).set_is_nonterminal(1 as libc::c_int as mln_u32_t);
        }
    }
    return tk;
}
unsafe extern "C" fn mln_lang_free(mut ptr: *mut mln_lang_struct_t) {
    if ptr.is_null() {
        return;
    }
    if !((*ptr).text).is_null() {
        let mut __s: *mut mln_string_t = (*ptr).text;
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
    if !((*ptr).file).is_null() {
        let mut __s: *mut mln_string_t = (*ptr).file;
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
    mln_alloc_free(ptr as *mut libc::c_void);
}
unsafe extern "C" fn mln_lang_token(mut lex: *mut mln_lex_t) -> *mut mln_lang_struct_t {
    let mut dot_cnt: mln_s32_t = 0;
    let mut chk: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut sret: *mut mln_lang_struct_t = 0 as *mut mln_lang_struct_t;
    '_beg: loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        loop {
            match c as libc::c_int {
                -1 => return mln_lang_new(lex, LANG_TK_EOF),
                10 => {
                    while c as libc::c_int == '\n' as i32 {
                        (*lex).line = ((*lex).line).wrapping_add(1);
                        (*lex).line;
                        c = mln_lex_getchar(lex);
                        if c as libc::c_int
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_lang_struct_t;
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
                            return 0 as *mut mln_lang_struct_t;
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
                        return 0 as *mut mln_lang_struct_t;
                    }
                    c = mln_lex_getchar(lex);
                    if c as libc::c_int
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_lang_struct_t;
                    }
                }
                mln_lex_stepback(lex, c);
                if *((*lex).result_buf).offset(0 as libc::c_int as isize) as libc::c_int
                    == '_' as i32 as mln_u8_t as libc::c_int
                    && (*lex).result_pos
                        == ((*lex).result_buf).offset(1 as libc::c_int as isize)
                {
                    sret = mln_lang_process_spec_char(lex, '_' as i32 as libc::c_char);
                    if sret.is_null() || (*lex).ignore() == 0 {
                        return sret;
                    }
                } else {
                    sret = mln_lang_process_keywords(lex);
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
                            return 0 as *mut mln_lang_struct_t;
                        }
                        c = mln_lex_getchar(lex);
                        if c as libc::c_int
                            == -(2 as libc::c_int) as libc::c_char as libc::c_int
                        {
                            return 0 as *mut mln_lang_struct_t;
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
                                return 0 as *mut mln_lang_struct_t;
                            }
                            chk = chk.offset(2 as libc::c_int as isize);
                            while chk < (*lex).result_pos {
                                if mln_lex_is_hex(*chk as libc::c_char) == 0 {
                                    (*lex).error = 3 as libc::c_int;
                                    return 0 as *mut mln_lang_struct_t;
                                }
                                chk = chk.offset(1);
                                chk;
                            }
                            sret = mln_lang_new(lex, LANG_TK_HEX);
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
                                                return 0 as *mut mln_lang_struct_t;
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
                                    sret = mln_lang_new(lex, LANG_TK_OCT);
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
                                        sret = mln_lang_new(lex, LANG_TK_REAL);
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
                                        return 0 as *mut mln_lang_struct_t;
                                    }
                                    chk = chk.offset(1);
                                    chk;
                                }
                                sret = mln_lang_new(lex, LANG_TK_REAL);
                                if sret.is_null() || (*lex).ignore() == 0 {
                                    return sret;
                                }
                                continue '_beg;
                            } else {
                                (*lex).error = 4 as libc::c_int;
                                return 0 as *mut mln_lang_struct_t;
                            }
                        }
                        sret = mln_lang_new(lex, LANG_TK_DEC);
                        if sret.is_null() || (*lex).ignore() == 0 {
                            return sret;
                        }
                    }
                } else {
                    if mln_lex_putchar(lex, c)
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_lang_struct_t;
                    }
                    sret = mln_lang_process_spec_char(lex, c);
                    if sret.is_null() {
                        return 0 as *mut mln_lang_struct_t
                    } else if (*lex).ignore() == 0 {
                        return sret
                    }
                }
            }
        }
    }
    return 0 as *mut mln_lang_struct_t;
}
unsafe extern "C" fn mln_lang_dash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_DASH);
}
unsafe extern "C" fn mln_lang_new(
    mut lex: *mut mln_lex_t,
    mut type_0: mln_lang_enum,
) -> *mut mln_lang_struct_t {
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut ptr: *mut mln_lang_struct_t = 0 as *mut mln_lang_struct_t;
    ptr = mln_alloc_m(
        (*lex).pool,
        ::core::mem::size_of::<mln_lang_struct_t>() as libc::c_ulong,
    ) as *mut mln_lang_struct_t;
    if ptr.is_null() {
        (*lex).error = 1 as libc::c_int;
        return 0 as *mut mln_lang_struct_t;
    }
    if type_0 as libc::c_uint != LANG_TK_EOF as libc::c_int as libc::c_uint {
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
        return 0 as *mut mln_lang_struct_t;
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
unsafe extern "C" fn mln_lang_rbrace_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_RBRACE);
}
unsafe extern "C" fn mln_lang_vertl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_VERTL);
}
unsafe extern "C" fn mln_lang_lbrace_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_LBRACE);
}
unsafe extern "C" fn mln_lang_fulstp_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_FULSTP);
}
unsafe extern "C" fn mln_lang_xor_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_XOR);
}
unsafe extern "C" fn mln_lang_rsquar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_RSQUAR);
}
unsafe extern "C" fn mln_lang_lpar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_LPAR);
}
unsafe extern "C" fn mln_lang_bslash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_BSLASH);
}
unsafe extern "C" fn mln_lang_lsquar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_LSQUAR);
}
unsafe extern "C" fn mln_lang_shift(
    mut spattr: *mut mln_sys_parse_attr,
    mut stack: *mut *mut mln_stack_t,
    mut la: *mut *mut mln_factor_t,
    mut state: *mut mln_sauto_t,
    mut is_reduce: *mut mln_sauto_t,
    mut sh: *mut mln_shift_t,
) -> libc::c_int {
    if *is_reduce == 0 as libc::c_int as libc::c_long {
        (**la).cur_state = *state;
        if mln_stack_push(*stack, *la as *mut libc::c_void) < 0 as libc::c_int {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"mln_lang_shift\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if (*spattr).type_0 == 0 as libc::c_int {
            let mut new_f: *mut mln_factor_t = mln_lang_factor_copy(
                *la as *mut libc::c_void,
                (*spattr).pool as *mut libc::c_void,
            ) as *mut mln_factor_t;
            *la = 0 as *mut mln_factor_t;
            if new_f.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mln_lang_shift\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if mln_queue_append((*(*spattr).p).cur_queue, new_f as *mut libc::c_void)
                < 0 as libc::c_int
            {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mln_lang_shift\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"Queue is full.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mln_lang_factor_destroy(new_f as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if (*(*(*spattr).p).cur_queue).nr_element >= (*(*(*spattr).p).cur_queue).qlen
            {
                let mut old_spattr: mln_sys_parse_attr = mln_sys_parse_attr {
                    pool: 0 as *mut mln_alloc_t,
                    p: 0 as *mut mln_parser_t,
                    lex: 0 as *mut mln_lex_t,
                    tbl: 0 as *mut mln_pg_shift_tbl_t,
                    prod_tbl: 0 as *mut mln_production_t,
                    udata: 0 as *mut libc::c_void,
                    type_0: 0,
                    done: 0,
                };
                memcpy(
                    &mut old_spattr as *mut mln_sys_parse_attr as *mut libc::c_void,
                    spattr as *const libc::c_void,
                    ::core::mem::size_of::<mln_sys_parse_attr>() as libc::c_ulong,
                );
                old_spattr.type_0 = 1 as libc::c_int;
                old_spattr.done = 0 as libc::c_int;
                mln_lang_sys_parse(&mut old_spattr);
            }
            let mut token: *mut mln_lang_struct_t = mln_lang_token((*spattr).lex);
            if token.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mln_lang_shift\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"Get token error. %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    mln_lex_strerror((*spattr).lex),
                );
                return -(1 as libc::c_int);
            }
            *la = mln_lang_factor_init(
                token as *mut libc::c_void,
                M_P_TERM,
                (*token).type_0 as libc::c_int,
                *state,
                (*token).line,
                (*token).file,
            );
            if (*la).is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mln_lang_shift\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mln_lang_free(token);
                return -(1 as libc::c_int);
            }
        } else if (*spattr).type_0 == 1 as libc::c_int {
            *la = 0 as *mut mln_factor_t;
            if (*(*(*spattr).p).cur_queue).nr_element == 0 {
                return 1 as libc::c_int;
            }
            *la = mln_queue_get((*(*spattr).p).cur_queue) as *mut mln_factor_t;
            mln_queue_remove((*(*spattr).p).cur_queue);
        } else {
            *la = 0 as *mut mln_factor_t;
            if (*(*(*spattr).p).err_queue).nr_element == 0 {
                return 1 as libc::c_int;
            }
            *la = mln_queue_get((*(*spattr).p).err_queue) as *mut mln_factor_t;
            mln_queue_remove((*(*spattr).p).err_queue);
        }
    }
    *state = (*sh).index as mln_sauto_t;
    if (*spattr).type_0 == 1 as libc::c_int
        && *is_reduce == 0 as libc::c_int as libc::c_long
        && (*spattr).done == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *is_reduce = 0 as libc::c_int as mln_sauto_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_reduce_launcher(
    mut st: *mut mln_stack_t,
    mut state: *mut mln_sauto_t,
    mut prod_tbl_0: *mut mln_production_t,
    mut sh: *mut mln_shift_t,
    mut udata: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut rights: *mut *mut mln_factor_t = 0 as *mut *mut mln_factor_t;
    rights = calloc(
        (*sh).nr_args as libc::c_ulong,
        ::core::mem::size_of::<*mut mln_factor_t>() as libc::c_ulong,
    ) as *mut *mut mln_factor_t;
    if rights.is_null() && (*sh).nr_args != 0 {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_reduce_launcher\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut left: *mut mln_factor_t = mln_lang_factor_init(
        0 as *mut libc::c_void,
        M_P_NONTERM,
        (*sh).left_type,
        *state,
        0 as libc::c_int as mln_u32_t,
        0 as *mut mln_string_t,
    );
    if left.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_reduce_launcher\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        free(rights as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let mut i: mln_u32_t = 0;
    let mut line: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut file: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut right: *mut mln_factor_t = 0 as *mut mln_factor_t;
    i = 0 as libc::c_int as mln_u32_t;
    while i < (*sh).nr_args {
        right = mln_stack_pop(st) as *mut mln_factor_t;
        if right.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"mln_lang_reduce_launcher\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"Fatal error. State shift table messed up.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            abort();
        }
        let ref mut fresh9 = *rights
            .offset(
                ((*sh).nr_args)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(i) as isize,
            );
        *fresh9 = right;
        if (*right).line > line {
            line = (*right).line;
            file = (*right).file;
        }
        *state = (*right).cur_state;
        i = i.wrapping_add(1);
        i;
    }
    (*left).line = line;
    (*left).cur_state = *state;
    if !file.is_null() {
        (*left)
            .file = ({
            let mut __s: *mut mln_string_t = file;
            (*__s).set_ref_0((*__s).ref_0() + 1);
            (*__s).ref_0();
            __s
        });
    }
    let mut pp: *mut mln_production_t = &mut *prod_tbl_0
        .offset((*sh).rule_index as isize) as *mut mln_production_t;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if type_0 == 1 as libc::c_int && ((*pp).func).is_some() {
        ret = ((*pp).func).expect("non-null function pointer")(left, rights, udata);
    }
    i = 0 as libc::c_int as mln_u32_t;
    while i < (*sh).nr_args {
        mln_lang_factor_destroy(*rights.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(rights as *mut libc::c_void);
    if ret < 0 as libc::c_int
        || mln_stack_push(st, left as *mut libc::c_void) < 0 as libc::c_int
    {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"mln_lang_reduce_launcher\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_lang_factor_destroy(left as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_err_dup_iterate_handler(
    mut q_node: *mut libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut eq: *mut mln_err_queue_s = udata as *mut mln_err_queue_s;
    if (*eq).opr == 0 as libc::c_int && (*eq).index == (*eq).pos {
        (*eq).index = ((*eq).index).wrapping_add(1);
        (*eq).index;
        return 0 as libc::c_int;
    }
    let mut f: *mut mln_factor_t = mln_lang_factor_copy(
        q_node as *mut mln_factor_t as *mut libc::c_void,
        (*eq).pool as *mut libc::c_void,
    ) as *mut mln_factor_t;
    if f.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"mln_lang_err_dup_iterate_handler\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*eq).opr == 1 as libc::c_int && (*eq).index == (*eq).pos {
        (*f).token_type = (*eq).ctype;
    }
    if mln_queue_append((*eq).q, f as *mut libc::c_void) < 0 as libc::c_int {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"mln_lang_err_dup_iterate_handler\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"Queue is full.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mln_lang_factor_destroy(f as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*eq).index = ((*eq).index).wrapping_add(1);
    (*eq).index;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_err_dup(
    mut spattr: *mut mln_sys_parse_attr,
    mut pos: mln_uauto_t,
    mut ctype: libc::c_int,
    mut opr: libc::c_int,
) -> libc::c_int {
    let mut p: *mut mln_parser_t = (*spattr).p;
    if !((*p).err_stack).is_null() {
        mln_stack_destroy((*p).err_stack);
        (*p).err_stack = 0 as *mut mln_stack_t;
    }
    if !((*p).err_la).is_null() {
        mln_lang_factor_destroy((*p).err_la as *mut libc::c_void);
        (*p).err_la = 0 as *mut mln_factor_t;
    }
    if !((*p).err_queue).is_null() {
        mln_queue_destroy((*p).err_queue);
        (*p).err_queue = 0 as *mut mln_queue_t;
    }
    (*p).err_stack = mln_stack_dup((*p).old_stack, (*spattr).pool as *mut libc::c_void);
    if ((*p).err_stack).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_lang_err_dup\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*p)
        .err_la = mln_lang_factor_copy(
        (*p).old_la as *mut libc::c_void,
        (*spattr).pool as *mut libc::c_void,
    ) as *mut mln_factor_t;
    if ((*p).err_la).is_null() && !((*p).old_la).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_lang_err_dup\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_stack_destroy((*p).err_stack);
        (*p).err_stack = 0 as *mut mln_stack_t;
        return -(1 as libc::c_int);
    }
    (*p).err_state = (*p).old_state;
    (*p).err_reduce = (*p).old_reduce;
    (*p)
        .err_queue = mln_queue_init(
        16 as libc::c_int as mln_uauto_t,
        Some(mln_lang_factor_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ((*p).err_queue).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mln_lang_err_dup\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_stack_destroy((*p).err_stack);
        (*p).err_stack = 0 as *mut mln_stack_t;
        mln_lang_factor_destroy((*p).err_la as *mut libc::c_void);
        (*p).err_la = 0 as *mut mln_factor_t;
        return -(1 as libc::c_int);
    }
    let mut eq: mln_err_queue_s = mln_err_queue_s {
        pool: 0 as *mut mln_alloc_t,
        q: 0 as *mut mln_queue_t,
        index: 0,
        pos: 0,
        opr: 0,
        ctype: 0,
    };
    eq.pool = (*spattr).pool;
    eq.q = (*p).err_queue;
    eq.index = 0 as libc::c_int as mln_uauto_t;
    eq.pos = pos;
    eq.opr = opr;
    eq.ctype = ctype;
    if mln_queue_iterate(
        (*p).cur_queue,
        Some(
            mln_lang_err_dup_iterate_handler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut eq as *mut mln_err_queue_s as *mut libc::c_void,
    ) < 0 as libc::c_int
    {
        mln_stack_destroy((*p).err_stack);
        (*p).err_stack = 0 as *mut mln_stack_t;
        mln_lang_factor_destroy((*p).err_la as *mut libc::c_void);
        (*p).err_la = 0 as *mut mln_factor_t;
        mln_queue_destroy((*p).err_queue);
        (*p).err_queue = 0 as *mut mln_queue_t;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_factor_init(
    mut data: *mut libc::c_void,
    mut data_type: factor_data_type,
    mut token_type: libc::c_int,
    mut cur_state: mln_sauto_t,
    mut line: mln_u32_t,
    mut file: *mut mln_string_t,
) -> *mut mln_factor_t {
    let mut f: *mut mln_factor_t = malloc(
        ::core::mem::size_of::<mln_factor_t>() as libc::c_ulong,
    ) as *mut mln_factor_t;
    if f.is_null() {
        return 0 as *mut mln_factor_t;
    }
    (*f).data = data;
    (*f).data_type = data_type;
    (*f).nonterm_free_handler = None;
    (*f).cur_state = cur_state;
    (*f).token_type = token_type;
    (*f).line = line;
    if file.is_null() {
        (*f).file = 0 as *mut mln_string_t;
    } else {
        (*f)
            .file = ({
            let mut __s: *mut mln_string_t = file;
            (*__s).set_ref_0((*__s).ref_0() + 1);
            (*__s).ref_0();
            __s
        });
    }
    return f;
}
unsafe extern "C" fn mln_lang_err_recover(
    mut spattr: *mut mln_sys_parse_attr,
    mut pos: mln_uauto_t,
    mut ctype: libc::c_int,
    mut opr: libc::c_int,
) -> libc::c_int {
    let mut p: *mut mln_parser_t = (*spattr).p;
    mln_stack_destroy((*p).cur_stack);
    (*p).cur_stack = mln_stack_dup((*p).err_stack, (*spattr).pool as *mut libc::c_void);
    if ((*p).cur_stack).is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"mln_lang_err_recover\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*(*p).cur_stack)
        .free_handler = Some(
        mln_lang_factor_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*(*p).cur_stack)
        .copy_handler = Some(
        mln_lang_factor_copy
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> *mut libc::c_void,
    );
    if ((*p).cur_la).is_null() {
        let mut token: *mut mln_lang_struct_t = mln_lang_token((*spattr).lex);
        if token.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"mln_lang_err_recover\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"Get token error. %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                mln_lex_strerror((*spattr).lex),
            );
            return -(1 as libc::c_int);
        }
        (*p)
            .cur_la = mln_lang_factor_init(
            token as *mut libc::c_void,
            M_P_TERM,
            (*token).type_0 as libc::c_int,
            (*p).cur_state,
            (*token).line,
            (*token).file,
        );
        if ((*p).cur_la).is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"mln_lang_err_recover\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"No memory.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            mln_lang_free(token);
            return -(1 as libc::c_int);
        }
    }
    (*p).cur_state = (*p).err_state;
    (*p).cur_reduce = (*p).err_reduce;
    if opr == 1 as libc::c_int {
        let mut f: *mut mln_factor_t = mln_queue_search((*p).cur_queue, pos)
            as *mut mln_factor_t;
        if f.is_null() {
            _mln_sys_log(
                error,
                b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"mln_lang_err_recover\0"))
                    .as_ptr(),
                90 as libc::c_int,
                b"Current stack messed up.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            abort();
        }
        (*f).token_type = ctype;
    } else {
        mln_queue_free_index((*p).cur_queue, pos);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_err_process(
    mut spattr: *mut mln_sys_parse_attr,
    mut opr: libc::c_int,
) -> libc::c_int {
    let mut ctype: libc::c_int = 0;
    let mut max: libc::c_int = (*(*spattr).tbl).type_val + 1 as libc::c_int;
    let mut f: *mut mln_factor_t = 0 as *mut mln_factor_t;
    let mut pos: mln_sauto_t = 0;
    let mut nr_element: mln_uauto_t = (*(*(*spattr).p).cur_queue).nr_element;
    if nr_element == 0 {
        return -(1 as libc::c_int);
    }
    pos = nr_element.wrapping_sub(1 as libc::c_int as libc::c_ulong) as mln_sauto_t;
    while pos >= 0 as libc::c_int as libc::c_long {
        ctype = 0 as libc::c_int;
        while ctype < max {
            f = mln_queue_search((*(*spattr).p).cur_queue, pos as mln_uauto_t)
                as *mut mln_factor_t;
            if f.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"mln_lang_err_process\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"Queue messed up.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                abort();
            }
            if !(ctype == (*f).token_type) {
                if mln_lang_err_dup(spattr, pos as mln_uauto_t, ctype, opr)
                    < 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                let mut err_spattr: mln_sys_parse_attr = mln_sys_parse_attr {
                    pool: 0 as *mut mln_alloc_t,
                    p: 0 as *mut mln_parser_t,
                    lex: 0 as *mut mln_lex_t,
                    tbl: 0 as *mut mln_pg_shift_tbl_t,
                    prod_tbl: 0 as *mut mln_production_t,
                    udata: 0 as *mut libc::c_void,
                    type_0: 0,
                    done: 0,
                };
                memcpy(
                    &mut err_spattr as *mut mln_sys_parse_attr as *mut libc::c_void,
                    spattr as *const libc::c_void,
                    ::core::mem::size_of::<mln_sys_parse_attr>() as libc::c_ulong,
                );
                err_spattr.type_0 = 2 as libc::c_int;
                err_spattr.done = 0 as libc::c_int;
                if mln_lang_sys_parse(&mut err_spattr) >= 0 as libc::c_int {
                    if mln_lang_err_recover(spattr, pos as mln_uauto_t, ctype, opr)
                        < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    return 0 as libc::c_int;
                }
                if opr == 0 as libc::c_int {
                    break;
                }
            }
            ctype += 1;
            ctype;
        }
        pos -= 1;
        pos;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_lang_sys_parse(
    mut spattr: *mut mln_sys_parse_attr,
) -> libc::c_int {
    let mut stack: *mut *mut mln_stack_t = 0 as *mut *mut mln_stack_t;
    let mut la: *mut *mut mln_factor_t = 0 as *mut *mut mln_factor_t;
    let mut state: *mut mln_sauto_t = 0 as *mut mln_sauto_t;
    let mut is_reduce: *mut mln_sauto_t = 0 as *mut mln_sauto_t;
    let mut p: *mut mln_parser_t = (*spattr).p;
    let mut type_0: libc::c_int = (*spattr).type_0;
    if type_0 == 0 as libc::c_int {
        stack = &mut (*p).cur_stack;
        la = &mut (*p).cur_la;
        state = &mut (*p).cur_state;
        is_reduce = &mut (*p).cur_reduce;
    } else if type_0 == 1 as libc::c_int {
        stack = &mut (*p).old_stack;
        la = &mut (*p).old_la;
        state = &mut (*p).old_state;
        is_reduce = &mut (*p).old_reduce;
    } else {
        stack = &mut (*p).err_stack;
        la = &mut (*p).err_la;
        state = &mut (*p).err_state;
        is_reduce = &mut (*p).err_reduce;
    }
    if (**stack).nr_node == 0 {
        if type_0 == 0 as libc::c_int {
            let mut token: *mut mln_lang_struct_t = mln_lang_token((*spattr).lex);
            if token.is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"mln_lang_sys_parse\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"Get token error. %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    mln_lex_strerror((*spattr).lex),
                );
                return -(1 as libc::c_int);
            }
            *la = mln_lang_factor_init(
                token as *mut libc::c_void,
                M_P_TERM,
                (*token).type_0 as libc::c_int,
                *state,
                (*token).line,
                (*token).file,
            );
            if (*la).is_null() {
                _mln_sys_log(
                    error,
                    b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"mln_lang_sys_parse\0"))
                        .as_ptr(),
                    90 as libc::c_int,
                    b"No memory.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mln_lang_free(token);
                return -(1 as libc::c_int);
            }
        } else if type_0 == 1 as libc::c_int {
            *la = mln_queue_get((*p).cur_queue) as *mut mln_factor_t;
            mln_queue_remove((*p).cur_queue);
        } else {
            *la = mln_queue_get((*p).err_queue) as *mut mln_factor_t;
            mln_queue_remove((*p).err_queue);
        }
    }
    let mut sh: *mut mln_shift_t = 0 as *mut mln_shift_t;
    let mut state_type: mln_u64_t = 0;
    let mut col_index: mln_u64_t = 0;
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut is_recovered: libc::c_int = 0 as libc::c_int;
    let mut failed_type: mln_sauto_t = -(1 as libc::c_int) as mln_sauto_t;
    let mut failedline: mln_sauto_t = -(1 as libc::c_int) as mln_sauto_t;
    let mut top: *mut mln_factor_t = 0 as *mut mln_factor_t;
    loop {
        if *is_reduce != 0 {
            top = (if ((**stack).top).is_null() {
                0 as *mut libc::c_void
            } else {
                (*(**stack).top).data
            }) as *mut mln_factor_t;
            col_index = (*top).token_type as mln_u64_t;
        } else {
            col_index = (**la).token_type as mln_u64_t;
        }
        sh = &mut *(*((*(*spattr).tbl).tbl).offset(*state as isize))
            .offset(col_index as isize) as *mut mln_shift_t;
        state_type = (*sh).type_0 as mln_u64_t;
        if state_type == 1 as libc::c_int as libc::c_ulong {
            is_recovered = 0 as libc::c_int;
            ret = mln_lang_shift(spattr, stack, la, state, is_reduce, sh);
            if ret > 0 as libc::c_int {
                break;
            }
            if ret < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        } else if state_type == 2 as libc::c_int as libc::c_ulong {
            is_recovered = 0 as libc::c_int;
            if mln_lang_reduce_launcher(
                *stack,
                state,
                (*spattr).prod_tbl,
                sh,
                (*spattr).udata,
                (*spattr).type_0,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            *is_reduce = 1 as libc::c_int as mln_sauto_t;
        } else if state_type == 0 as libc::c_int as libc::c_ulong {
            if is_recovered != 0 {
                return -(1 as libc::c_int);
            }
            if type_0 == 0 as libc::c_int {
                if failed != 0 && failed_type == (**la).token_type as libc::c_long
                    && failedline == (**la).line as libc::c_long
                {
                    return -(1 as libc::c_int);
                }
                failed = 1 as libc::c_int;
                failed_type = (**la).token_type as mln_sauto_t;
                failedline = (**la).line as mln_sauto_t;
                let mut name: mln_s8ptr_t = 0 as mln_s8ptr_t;
                if (**la).token_type != LANG_TK_EOF as libc::c_int
                    && !((**la).data).is_null()
                {
                    name = (*(*((**la).data as *mut mln_lang_struct_t)).text).data
                        as mln_s8ptr_t;
                }
                if ((**la).file).is_null() {
                    _mln_sys_log(
                        none,
                        b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"mln_lang_sys_parse\0"))
                            .as_ptr(),
                        90 as libc::c_int,
                        b"line %d: Parse Error: Illegal token%s%s%s\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        if *is_reduce != 0 { (*top).line } else { (**la).line },
                        if name.is_null() {
                            b".\0" as *const u8 as *const libc::c_char
                        } else {
                            b" nearby '\0" as *const u8 as *const libc::c_char
                        },
                        if name.is_null() {
                            b" \0" as *const u8 as *const libc::c_char
                        } else {
                            name as *const libc::c_char
                        },
                        if name.is_null() {
                            b" \0" as *const u8 as *const libc::c_char
                        } else {
                            b"'.\0" as *const u8 as *const libc::c_char
                        },
                    );
                } else {
                    _mln_sys_log(
                        none,
                        b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"mln_lang_sys_parse\0"))
                            .as_ptr(),
                        90 as libc::c_int,
                        b"%s:%d: Parse Error: Illegal token%s%s%s\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*(**la).file).data as *mut libc::c_char,
                        if *is_reduce != 0 { (*top).line } else { (**la).line },
                        if name.is_null() {
                            b".\0" as *const u8 as *const libc::c_char
                        } else {
                            b" nearby '\0" as *const u8 as *const libc::c_char
                        },
                        if name.is_null() {
                            b" \0" as *const u8 as *const libc::c_char
                        } else {
                            name as *const libc::c_char
                        },
                        if name.is_null() {
                            b" \0" as *const u8 as *const libc::c_char
                        } else {
                            b"'.\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                if (**la).token_type == LANG_TK_EOF as libc::c_int {
                    break;
                }
                ret = mln_lang_err_process(spattr, 1 as libc::c_int);
                if ret >= 0 as libc::c_int {
                    is_recovered = 1 as libc::c_int;
                } else {
                    ret = mln_lang_err_process(spattr, 0 as libc::c_int);
                    if ret < 0 as libc::c_int {
                        break;
                    }
                    is_recovered = 1 as libc::c_int;
                }
            } else {
                return -(1 as libc::c_int)
            }
        } else {
            is_recovered = 0 as libc::c_int;
            if type_0 == 0 as libc::c_int {
                (**la).cur_state = *state;
                if mln_queue_append((*p).cur_queue, *la as *mut libc::c_void)
                    < 0 as libc::c_int
                {
                    _mln_sys_log(
                        error,
                        b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"mln_lang_sys_parse\0"))
                            .as_ptr(),
                        90 as libc::c_int,
                        b"Queue is full.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                *la = 0 as *mut mln_factor_t;
                let mut old_spattr: mln_sys_parse_attr = mln_sys_parse_attr {
                    pool: 0 as *mut mln_alloc_t,
                    p: 0 as *mut mln_parser_t,
                    lex: 0 as *mut mln_lex_t,
                    tbl: 0 as *mut mln_pg_shift_tbl_t,
                    prod_tbl: 0 as *mut mln_production_t,
                    udata: 0 as *mut libc::c_void,
                    type_0: 0,
                    done: 0,
                };
                memcpy(
                    &mut old_spattr as *mut mln_sys_parse_attr as *mut libc::c_void,
                    spattr as *const libc::c_void,
                    ::core::mem::size_of::<mln_sys_parse_attr>() as libc::c_ulong,
                );
                old_spattr.type_0 = 1 as libc::c_int;
                old_spattr.done = 1 as libc::c_int;
                mln_lang_sys_parse(&mut old_spattr);
            }
            break;
        }
    }
    return if failed == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn mln_lang_factor_copy(
    mut ptr: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() || data.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut src: *mut mln_factor_t = ptr as *mut mln_factor_t;
    let mut dest: *mut mln_factor_t = malloc(
        ::core::mem::size_of::<mln_factor_t>() as libc::c_ulong,
    ) as *mut mln_factor_t;
    if dest.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*dest).data = 0 as *mut libc::c_void;
    if !((*src).data).is_null() {
        if (*src).data_type as libc::c_uint == M_P_TERM as libc::c_int as libc::c_uint {
            (*dest).data = mln_lang_lex_dup(pool, (*src).data);
            if ((*dest).data).is_null() {
                free(dest as *mut libc::c_void);
                return 0 as *mut libc::c_void;
            }
        }
    }
    (*dest).data_type = (*src).data_type;
    (*dest).nonterm_free_handler = (*src).nonterm_free_handler;
    (*dest).token_type = (*src).token_type;
    (*dest).cur_state = (*src).cur_state;
    (*dest).line = (*src).line;
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
unsafe extern "C" fn mln_lang_factor_destroy(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let mut f: *mut mln_factor_t = ptr as *mut mln_factor_t;
    if (*f).data_type as libc::c_uint == M_P_TERM as libc::c_int as libc::c_uint {
        if !((*f).data).is_null() {
            mln_lang_free((*f).data as *mut mln_lang_struct_t);
        }
    } else if !((*f).data).is_null() && ((*f).nonterm_free_handler).is_some() {
        ((*f).nonterm_free_handler).expect("non-null function pointer")((*f).data);
    }
    if !((*f).file).is_null() {
        let mut __s: *mut mln_string_t = (*f).file;
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
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn mln_lang_parser_init() -> *mut mln_parser_t {
    let mut p: *mut mln_parser_t = malloc(
        ::core::mem::size_of::<mln_parser_t>() as libc::c_ulong,
    ) as *mut mln_parser_t;
    if p.is_null() {
        return 0 as *mut mln_parser_t;
    }
    (*p)
        .cur_stack = mln_stack_init(
        Some(mln_lang_factor_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(
            mln_lang_factor_copy
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
    if !((*p).cur_stack).is_null() {
        (*p).cur_la = 0 as *mut mln_factor_t;
        (*p).cur_state = 0 as libc::c_int as mln_sauto_t;
        (*p).cur_reduce = 0 as libc::c_int as mln_sauto_t;
        (*p)
            .old_stack = mln_stack_init(
            Some(
                mln_lang_factor_destroy as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            Some(
                mln_lang_factor_copy
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        );
        if !((*p).old_stack).is_null() {
            (*p).old_la = 0 as *mut mln_factor_t;
            (*p).old_state = 0 as libc::c_int as mln_sauto_t;
            (*p).old_reduce = 0 as libc::c_int as mln_sauto_t;
            (*p).err_stack = 0 as *mut mln_stack_t;
            (*p).err_la = 0 as *mut mln_factor_t;
            (*p).err_state = 0 as libc::c_int as mln_sauto_t;
            (*p).err_reduce = 0 as libc::c_int as mln_sauto_t;
            (*p).err_queue = 0 as *mut mln_queue_t;
            (*p)
                .cur_queue = mln_queue_init(
                16 as libc::c_int as mln_uauto_t,
                Some(
                    mln_lang_factor_destroy
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
            if ((*p).cur_queue).is_null() {
                mln_stack_destroy((*p).old_stack);
            } else {
                return p
            }
        }
        mln_stack_destroy((*p).cur_stack);
    }
    free(p as *mut libc::c_void);
    return 0 as *mut mln_parser_t;
}
unsafe extern "C" fn mln_lang_parse(
    mut pattr: *mut mln_parse_attr,
) -> *mut libc::c_void {
    let mut tbl: *mut mln_pg_shift_tbl_t = (*pattr).pg_data as *mut mln_pg_shift_tbl_t;
    let mut p: *mut mln_parser_t = mln_lang_parser_init();
    if p.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"mln_lang_parse\0"))
                .as_ptr(),
            90 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    let mut spattr: mln_sys_parse_attr = mln_sys_parse_attr {
        pool: 0 as *mut mln_alloc_t,
        p: 0 as *mut mln_parser_t,
        lex: 0 as *mut mln_lex_t,
        tbl: 0 as *mut mln_pg_shift_tbl_t,
        prod_tbl: 0 as *mut mln_production_t,
        udata: 0 as *mut libc::c_void,
        type_0: 0,
        done: 0,
    };
    spattr.pool = (*pattr).pool;
    spattr.p = p;
    spattr.lex = (*pattr).lex;
    spattr.tbl = tbl;
    spattr.prod_tbl = (*pattr).prod_tbl;
    spattr.udata = (*pattr).udata;
    spattr.type_0 = 0 as libc::c_int;
    spattr.done = 0 as libc::c_int;
    let mut ret: mln_u8ptr_t = 0 as mln_u8ptr_t;
    let mut rc: libc::c_int = mln_lang_sys_parse(&mut spattr);
    if rc >= 0 as libc::c_int {
        let mut top: *mut mln_factor_t = (if ((*(*spattr.p).old_stack).top).is_null() {
            0 as *mut libc::c_void
        } else {
            (*(*(*spattr.p).old_stack).top).data
        }) as *mut mln_factor_t;
        if !top.is_null() {
            ret = (*top).data as mln_u8ptr_t;
            (*top).data = 0 as *mut libc::c_void;
        }
    }
    mln_lang_parser_destroy(p);
    return ret as *mut libc::c_void;
}
unsafe extern "C" fn mln_lang_at_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_AT);
}
unsafe extern "C" fn mln_lang_ques_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_QUES);
}
unsafe extern "C" fn mln_lang_equal_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_EQUAL);
}
unsafe extern "C" fn mln_lang_lex_preprocess_define(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
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
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if mln_lex_is_letter(c) == 0
            && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
        {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        mln_lex_stepback(lex, c);
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
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
            return 0 as *mut mln_lang_struct_t;
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
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int != '\n' as i32 {
            (*lex).result_pos = (*lex).result_buf;
            loop {
                c = mln_lex_getchar(lex);
                if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return 0 as *mut mln_lang_struct_t;
                }
                if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
                    continue;
                }
                if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int
                    || c as libc::c_int == '\n' as i32
                {
                    current_block_60 = 14850226301948533577;
                    break;
                }
                mln_lex_stepback(lex, c);
                current_block_60 = 13883539237973446562;
                break;
            }
            match current_block_60 {
                14850226301948533577 => {}
                _ => {
                    loop {
                        loop {
                            c = mln_lex_getchar(lex);
                            if c as libc::c_int
                                == -(2 as libc::c_int) as libc::c_char as libc::c_int
                            {
                                return 0 as *mut mln_lang_struct_t;
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
                                return 0 as *mut mln_lang_struct_t;
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
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        let mut __s: *mut mln_string_t = k;
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
        rn = mln_rbtree_node_new((*lex).macros, lm as *mut libc::c_void);
        if rn.is_null() {
            (*lex).error = 1 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        mln_rbtree_insert((*lex).macros, rn);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_lex_preprocess_include(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
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
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int != '"' as i32 {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '"' as i32 {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
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
        return 0 as *mut mln_lang_struct_t;
    }
    if mln_lex_check_file_loop(lex, &mut path) < 0 as libc::c_int {
        return 0 as *mut mln_lang_struct_t;
    }
    if mln_lex_push_input_file_stream(lex, &mut path) < 0 as libc::c_int {
        return 0 as *mut mln_lang_struct_t;
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_lex_preprocess_if(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut ret: libc::c_int = 0;
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    (*lex).result_pos = (*lex).result_buf;
    (*lpd).if_level = ((*lpd).if_level).wrapping_add(1);
    (*lpd).if_level;
    if (*lex).ignore() != 0 {
        return mln_lang_token(lex);
    }
    ret = mln_lex_condition_test(lex);
    if ret < 0 as libc::c_int {
        return 0 as *mut mln_lang_struct_t;
    }
    if ret != 0 {
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_add(1);
        (*lpd).if_matched;
        (*lex).set_ignore(0 as libc::c_int as mln_u32_t);
    } else {
        (*lex).set_ignore(1 as libc::c_int as mln_u32_t);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_lex_preprocess_else(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    if (*lpd).if_level == 0 as libc::c_int as libc::c_ulong {
        (*lex).error = 15 as libc::c_int;
        return 0 as *mut mln_lang_struct_t;
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
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_lex_preprocess_endif(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut lpd: *mut mln_lex_preprocess_data_t = data as *mut mln_lex_preprocess_data_t;
    if (*lpd).if_level == 0 as libc::c_int as libc::c_ulong {
        (*lex).error = 15 as libc::c_int;
        return 0 as *mut mln_lang_struct_t;
    }
    let fresh18 = (*lpd).if_level;
    (*lpd).if_level = ((*lpd).if_level).wrapping_sub(1);
    if (*lpd).if_matched == fresh18 {
        (*lpd).if_matched = ((*lpd).if_matched).wrapping_sub(1);
        (*lpd).if_matched;
    }
    (*lex)
        .set_ignore(!((*lpd).if_matched == (*lpd).if_level) as libc::c_int as mln_u32_t);
    (*lex).result_pos = (*lex).result_buf;
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_lex_preprocess_undef(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
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
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32 {
            continue;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).error = 9 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if mln_lex_is_letter(c) == 0
            && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
        {
            (*lex).error = 2 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        mln_lex_stepback(lex, c);
        break;
    }
    (*lex).result_pos = (*lex).result_buf;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
        {
            break;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
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
    return mln_lang_token(lex);
}
unsafe extern "C" fn mln_lang_nums_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
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
        return 0 as *mut mln_lang_struct_t;
    }
    if mln_lex_is_letter(c) == 0
        && !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32)
    {
        mln_lex_stepback(lex, c);
        return mln_lang_new(lex, LANG_TK_AT);
    }
    (*lex).result_pos = (*lex).result_buf;
    while mln_lex_is_letter(c) != 0
        || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
    {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
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
    phend = mln_lang_preprocess_handlers
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[mln_preprocess_handler_t; 6]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<mln_preprocess_handler_t>() as libc::c_ulong,
                ) as isize,
        );
    ph = mln_lang_preprocess_handlers.as_mut_ptr();
    while ph < phend {
        if mln_string_strcmp(&mut (*ph).command, &mut tmp) == 0 {
            (*lex).result_pos = (*lex).result_buf;
            return ((*ph).handler).expect("non-null function pointer")(lex, data)
                as *mut mln_lang_struct_t;
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
            return 0 as *mut mln_lang_struct_t;
        }
        (*lex).result_pos = (*lex).result_buf;
        return mln_lang_token(lex);
    }
    (*lex).error = 13 as libc::c_int;
    return 0 as *mut mln_lang_struct_t;
}
unsafe extern "C" fn mln_lang_set_hooks(mut lex: *mut mln_lex_t) {
    let mut hooks: *mut mln_lex_hooks_t = &mut (*lex).hooks;
    if ((*hooks).excl_handler).is_some() {
        mln_lang_handlers[0 as libc::c_int as usize].handler = (*hooks).excl_handler;
        mln_lang_handlers[0 as libc::c_int as usize].data = (*hooks).excl_data;
    }
    if ((*hooks).dblq_handler).is_some() {
        mln_lang_handlers[1 as libc::c_int as usize].handler = (*hooks).dblq_handler;
        mln_lang_handlers[1 as libc::c_int as usize].data = (*hooks).dblq_data;
    }
    if ((*hooks).nums_handler).is_some() {
        mln_lang_handlers[2 as libc::c_int as usize].handler = (*hooks).nums_handler;
        mln_lang_handlers[2 as libc::c_int as usize].data = (*hooks).nums_data;
    }
    if ((*hooks).doll_handler).is_some() {
        mln_lang_handlers[3 as libc::c_int as usize].handler = (*hooks).doll_handler;
        mln_lang_handlers[3 as libc::c_int as usize].data = (*hooks).doll_data;
    }
    if ((*hooks).perc_handler).is_some() {
        mln_lang_handlers[4 as libc::c_int as usize].handler = (*hooks).perc_handler;
        mln_lang_handlers[4 as libc::c_int as usize].data = (*hooks).perc_data;
    }
    if ((*hooks).amp_handler).is_some() {
        mln_lang_handlers[5 as libc::c_int as usize].handler = (*hooks).amp_handler;
        mln_lang_handlers[5 as libc::c_int as usize].data = (*hooks).amp_data;
    }
    if ((*hooks).sglq_handler).is_some() {
        mln_lang_handlers[6 as libc::c_int as usize].handler = (*hooks).sglq_handler;
        mln_lang_handlers[6 as libc::c_int as usize].data = (*hooks).slgq_data;
    }
    if ((*hooks).lpar_handler).is_some() {
        mln_lang_handlers[7 as libc::c_int as usize].handler = (*hooks).lpar_handler;
        mln_lang_handlers[7 as libc::c_int as usize].data = (*hooks).lpar_data;
    }
    if ((*hooks).rpar_handler).is_some() {
        mln_lang_handlers[8 as libc::c_int as usize].handler = (*hooks).rpar_handler;
        mln_lang_handlers[8 as libc::c_int as usize].data = (*hooks).rpar_data;
    }
    if ((*hooks).ast_handler).is_some() {
        mln_lang_handlers[9 as libc::c_int as usize].handler = (*hooks).ast_handler;
        mln_lang_handlers[9 as libc::c_int as usize].data = (*hooks).ast_data;
    }
    if ((*hooks).plus_handler).is_some() {
        mln_lang_handlers[10 as libc::c_int as usize].handler = (*hooks).plus_handler;
        mln_lang_handlers[10 as libc::c_int as usize].data = (*hooks).plus_data;
    }
    if ((*hooks).comma_handler).is_some() {
        mln_lang_handlers[11 as libc::c_int as usize].handler = (*hooks).comma_handler;
        mln_lang_handlers[11 as libc::c_int as usize].data = (*hooks).comma_data;
    }
    if ((*hooks).sub_handler).is_some() {
        mln_lang_handlers[12 as libc::c_int as usize].handler = (*hooks).sub_handler;
        mln_lang_handlers[12 as libc::c_int as usize].data = (*hooks).sub_data;
    }
    if ((*hooks).period_handler).is_some() {
        mln_lang_handlers[13 as libc::c_int as usize].handler = (*hooks).period_handler;
        mln_lang_handlers[13 as libc::c_int as usize].data = (*hooks).period_data;
    }
    if ((*hooks).slash_handler).is_some() {
        mln_lang_handlers[14 as libc::c_int as usize].handler = (*hooks).slash_handler;
        mln_lang_handlers[14 as libc::c_int as usize].data = (*hooks).slash_data;
    }
    if ((*hooks).colon_handler).is_some() {
        mln_lang_handlers[15 as libc::c_int as usize].handler = (*hooks).colon_handler;
        mln_lang_handlers[15 as libc::c_int as usize].data = (*hooks).colon_data;
    }
    if ((*hooks).semic_handler).is_some() {
        mln_lang_handlers[16 as libc::c_int as usize].handler = (*hooks).semic_handler;
        mln_lang_handlers[16 as libc::c_int as usize].data = (*hooks).semic_data;
    }
    if ((*hooks).lagl_handler).is_some() {
        mln_lang_handlers[17 as libc::c_int as usize].handler = (*hooks).lagl_handler;
        mln_lang_handlers[17 as libc::c_int as usize].data = (*hooks).lagl_data;
    }
    if ((*hooks).equal_handler).is_some() {
        mln_lang_handlers[18 as libc::c_int as usize].handler = (*hooks).equal_handler;
        mln_lang_handlers[18 as libc::c_int as usize].data = (*hooks).equal_data;
    }
    if ((*hooks).ragl_handler).is_some() {
        mln_lang_handlers[19 as libc::c_int as usize].handler = (*hooks).ragl_handler;
        mln_lang_handlers[19 as libc::c_int as usize].data = (*hooks).ragl_data;
    }
    if ((*hooks).ques_handler).is_some() {
        mln_lang_handlers[20 as libc::c_int as usize].handler = (*hooks).ques_handler;
        mln_lang_handlers[20 as libc::c_int as usize].data = (*hooks).ques_data;
    }
    if ((*hooks).at_handler).is_some() {
        mln_lang_handlers[21 as libc::c_int as usize].handler = (*hooks).at_handler;
        mln_lang_handlers[21 as libc::c_int as usize].data = (*hooks).at_data;
    }
    if ((*hooks).lsquar_handler).is_some() {
        mln_lang_handlers[22 as libc::c_int as usize].handler = (*hooks).lsquar_handler;
        mln_lang_handlers[22 as libc::c_int as usize].data = (*hooks).lsquar_data;
    }
    if ((*hooks).bslash_handler).is_some() {
        mln_lang_handlers[23 as libc::c_int as usize].handler = (*hooks).bslash_handler;
        mln_lang_handlers[23 as libc::c_int as usize].data = (*hooks).bslash_data;
    }
    if ((*hooks).rsquar_handler).is_some() {
        mln_lang_handlers[24 as libc::c_int as usize].handler = (*hooks).rsquar_handler;
        mln_lang_handlers[24 as libc::c_int as usize].data = (*hooks).rsquar_data;
    }
    if ((*hooks).xor_handler).is_some() {
        mln_lang_handlers[25 as libc::c_int as usize].handler = (*hooks).xor_handler;
        mln_lang_handlers[25 as libc::c_int as usize].data = (*hooks).xor_data;
    }
    if ((*hooks).under_handler).is_some() {
        mln_lang_handlers[26 as libc::c_int as usize].handler = (*hooks).under_handler;
        mln_lang_handlers[26 as libc::c_int as usize].data = (*hooks).under_data;
    }
    if ((*hooks).fulstp_handler).is_some() {
        mln_lang_handlers[27 as libc::c_int as usize].handler = (*hooks).fulstp_handler;
        mln_lang_handlers[27 as libc::c_int as usize].data = (*hooks).fulstp_data;
    }
    if ((*hooks).lbrace_handler).is_some() {
        mln_lang_handlers[28 as libc::c_int as usize].handler = (*hooks).lbrace_handler;
        mln_lang_handlers[28 as libc::c_int as usize].data = (*hooks).lbrace_data;
    }
    if ((*hooks).vertl_handler).is_some() {
        mln_lang_handlers[29 as libc::c_int as usize].handler = (*hooks).vertl_handler;
        mln_lang_handlers[29 as libc::c_int as usize].data = (*hooks).vertl_data;
    }
    if ((*hooks).rbrace_handler).is_some() {
        mln_lang_handlers[30 as libc::c_int as usize].handler = (*hooks).rbrace_handler;
        mln_lang_handlers[30 as libc::c_int as usize].data = (*hooks).rbrace_data;
    }
    if ((*hooks).dash_handler).is_some() {
        mln_lang_handlers[31 as libc::c_int as usize].handler = (*hooks).dash_handler;
        mln_lang_handlers[31 as libc::c_int as usize].data = (*hooks).dash_data;
    }
}
unsafe extern "C" fn mln_lang_excl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_EXCL);
}
unsafe extern "C" fn mln_lang_dblq_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_DBLQ);
}
unsafe extern "C" fn mln_lang_nums_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_NUMS);
}
unsafe extern "C" fn mln_lang_doll_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_DOLL);
}
unsafe extern "C" fn mln_lang_perc_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_PERC);
}
unsafe extern "C" fn mln_lang_amp_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_AMP);
}
unsafe extern "C" fn mln_lang_sglq_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_SGLQ);
}
unsafe extern "C" fn mln_lang_lagl_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_LAGL);
}
unsafe extern "C" fn mln_lang_rpar_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_RPAR);
}
unsafe extern "C" fn mln_lang_ast_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_AST);
}
unsafe extern "C" fn mln_lang_plus_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_PLUS);
}
unsafe extern "C" fn mln_lang_comma_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_COMMA);
}
unsafe extern "C" fn mln_lang_sub_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_SUB);
}
unsafe extern "C" fn mln_lang_period_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_PERIOD);
}
unsafe extern "C" fn mln_lang_slash_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_SLASH);
}
unsafe extern "C" fn mln_lang_colon_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_COLON);
}
unsafe extern "C" fn mln_lang_semic_default_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    return mln_lang_new(lex, LANG_TK_SEMIC);
}
static mut keywords: [mln_string_t; 16] = [mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
}; 16];
static mut prod_tbl: [mln_production_t; 112] = unsafe {
    [
        {
            let mut init = mln_production_t {
                production: b"start: stm LANG_TK_EOF\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_start
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: blockstm stm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_stm_block
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: funcdef stm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_stmfunc
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: LANG_TK_ID LANG_TK_LBRACE setstm LANG_TK_RBRACE stm\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_stmset
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: LANG_TK_ID LANG_TK_COLON stm\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_labelstm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: LANG_TK_WHILE LANG_TK_LPAR exp LANG_TK_RPAR blockstm stm\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_whilestm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: LANG_TK_FOR LANG_TK_LPAR exp LANG_TK_SEMIC exp LANG_TK_SEMIC exp LANG_TK_RPAR blockstm stm\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_forstm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: LANG_TK_SWITCH LANG_TK_LPAR exp LANG_TK_RPAR LANG_TK_LBRACE switchstm LANG_TK_RBRACE stm\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_switchstm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"stm: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"setstm: LANG_TK_ID LANG_TK_SEMIC setstm\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_setstm_var
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"setstm: funcdef setstm\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_setstm_func
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"setstm: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"funcdef: LANG_TK_AT LANG_TK_ID LANG_TK_LPAR exp LANG_TK_RPAR use LANG_TK_LBRACE stm LANG_TK_RBRACE\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_funcdef
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"use: LANG_TK_DOLL LANG_TK_LPAR exp LANG_TK_RPAR\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_funcdef_closure
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"use: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"switchstm: switchprefix LANG_TK_COLON stm switchstm\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_switchstm__
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"switchstm: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"switchprefix: LANG_TK_DEFAULT\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"switchprefix: LANG_TK_CASE factor\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_switchprefix
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: exp LANG_TK_SEMIC\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_blockstmexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_LBRACE stm LANG_TK_RBRACE\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_blockstmstm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_CONTINUE LANG_TK_SEMIC\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_continue
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_BREAK LANG_TK_SEMIC\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_break
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_RETURN exp LANG_TK_SEMIC\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_return
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_GOTO LANG_TK_ID LANG_TK_SEMIC\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_goto
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"blockstm: LANG_TK_IF LANG_TK_LPAR exp LANG_TK_RPAR blockstm else_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_ifstm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"else_exp: LANG_TK_ELSE blockstm\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_elsestm
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"else_exp: LANG_TK_FI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"exp: assign_exp explist\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_exp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"explist: LANG_TK_COMMA exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_explist
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"explist: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"assign_exp: logiclow_exp __assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_EQUAL assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpeq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_PLUSEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexppluseq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_SUBEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpsubeq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_LMOVEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexplmoveq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_RMOVEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexprmoveq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_MULEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpmuleq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_DIVEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpdiveq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_OREQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexporeq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_ANDEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpandeq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_XOREQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpxoreq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: LANG_TK_MODEQ assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_assignexpmodeq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__assign_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"logiclow_exp: logichigh_exp __logiclow_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logiclowexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logiclow_exp: LANG_TK_LOWOR logiclow_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logiclowexpor
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logiclow_exp: LANG_TK_LOWAND logiclow_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logiclowexpand
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logiclow_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"logichigh_exp: relativelow_exp __logichigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logichighexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logichigh_exp: LANG_TK_VERTL logichigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logichighor
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logichigh_exp: LANG_TK_AMP logichigh_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logichighand
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logichigh_exp: LANG_TK_XOR logichigh_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_logichighxor
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__logichigh_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"relativelow_exp: relativehigh_exp __relativelow_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativelowexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativelow_exp: LANG_TK_DEQUAL relativelow_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativeloweq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativelow_exp: LANG_TK_NONEQUAL relativelow_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativelownoneq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativelow_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"relativehigh_exp: move_exp __relativehigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativehighexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativehigh_exp: LANG_TK_LAGL relativehigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativehighless
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativehigh_exp: LANG_TK_LESSEQ relativehigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativehighlesseq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativehigh_exp: LANG_TK_RAGL relativehigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativehighgreater
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativehigh_exp: LANG_TK_GREATEREQ relativehigh_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_relativehighgreatereq
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__relativehigh_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"move_exp: addsub_exp __move_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_moveexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__move_exp: LANG_TK_LMOVE move_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_moveleft
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__move_exp: LANG_TK_RMOVE move_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_moveright
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__move_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"addsub_exp: muldiv_exp __addsub_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_addsubexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__addsub_exp: LANG_TK_PLUS addsub_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_addsubplus
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__addsub_exp: LANG_TK_SUB addsub_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_addsubsub
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__addsub_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"muldiv_exp: not_exp __muldiv_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_muldivexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__muldiv_exp: LANG_TK_AST muldiv_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_muldivmul
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__muldiv_exp: LANG_TK_SLASH muldiv_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_muldivdiv
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__muldiv_exp: LANG_TK_PERC muldiv_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_muldivmod
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__muldiv_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"not_exp: LANG_TK_EXCL not_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_notnot
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"not_exp: suffix_exp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_notsuffix
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"suffix_exp: locate_exp __suffix_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_suffixexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__suffix_exp: LANG_TK_INC\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_suffixinc
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__suffix_exp: LANG_TK_DECR\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_suffixdec
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__suffix_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"locate_exp: spec_exp __locate_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_locateexp
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__locate_exp: LANG_TK_LSQUAR exp LANG_TK_RSQUAR __locate_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_locateindex
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__locate_exp: LANG_TK_PERIOD LANG_TK_ID __locate_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_locateproperty
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__locate_exp: LANG_TK_LPAR exp LANG_TK_RPAR __locate_exp\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_locatefunc
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"__locate_exp: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_SUB spec_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specnegative
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_DASH spec_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specreverse
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_AMP spec_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specrefer
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_INC spec_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specinc
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_DECR spec_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specdec
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_DOLL LANG_TK_ID\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specnew
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: LANG_TK_LPAR exp LANG_TK_RPAR\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specparenth
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"spec_exp: factor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_specfactor
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_TRUE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factortrue
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_FALSE\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorfalse
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_NIL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factornil
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_ID\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorid
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_OCT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorint
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_DEC\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorint
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_HEX\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorint
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_REAL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorreal
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_STRING\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorstring
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"factor: LANG_TK_LSQUAR elemlist LANG_TK_RSQUAR\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_factorarray
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemlist: assign_exp elemval elemnext\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_elemlist
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemlist: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemval: LANG_TK_COLON assign_exp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_elemval
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemval: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemnext: LANG_TK_COMMA elemlist\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    mln_lang_semantic_elemnext
                        as unsafe extern "C" fn(
                            *mut mln_factor_t,
                            *mut *mut mln_factor_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = mln_production_t {
                production: b"elemnext: \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: None,
            };
            init
        },
    ]
};
static mut mln_lang_env: mln_string_t = mln_string_t {
    data: 0 as *mut libc::c_uchar,
    len: 0,
    data_ref_pool_ref_0: [0; 4],
    c2rust_padding: [0; 4],
};
unsafe extern "C" fn mln_lang_sub_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '-' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_DECR);
    }
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_SUBEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_SUB);
}
unsafe extern "C" fn mln_lang_plus_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '+' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_INC);
    }
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_PLUSEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_PLUS);
}
unsafe extern "C" fn mln_lang_ast_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_MULEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_AST);
}
unsafe extern "C" fn mln_lang_lagl_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '<' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        c = mln_lex_getchar(lex);
        if c as libc::c_int != '=' as i32 {
            mln_lex_stepback(lex, c);
            return mln_lang_new(lex, LANG_TK_LMOVE);
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_LMOVEQ);
    }
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_LESSEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_LAGL);
}
unsafe extern "C" fn mln_lang_ragl_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '>' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        c = mln_lex_getchar(lex);
        if c as libc::c_int != '=' as i32 {
            mln_lex_stepback(lex, c);
            return mln_lang_new(lex, LANG_TK_RMOVE);
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_RMOVEQ);
    }
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_GREATEREQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_RAGL);
}
unsafe extern "C" fn mln_lang_vertl_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_OREQ);
    }
    if c as libc::c_int == '|' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_LOWOR);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_VERTL);
}
unsafe extern "C" fn mln_lang_amp_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_ANDEQ);
    }
    if c as libc::c_int == '&' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_LOWAND);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_AMP);
}
unsafe extern "C" fn mln_lang_dash_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_ANTIEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_DASH);
}
unsafe extern "C" fn mln_lang_xor_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_XOREQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_XOR);
}
unsafe extern "C" fn mln_lang_perc_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_MODEQ);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_PERC);
}
unsafe extern "C" fn mln_lang_equal_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_DEQUAL);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_EQUAL);
}
unsafe extern "C" fn mln_lang_excl_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_NONEQUAL);
    }
    mln_lex_stepback(lex, c);
    return mln_lang_new(lex, LANG_TK_EXCL);
}
unsafe extern "C" fn mln_lang_sglq_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    (*lex).result_pos = (*lex).result_buf;
    let mut c: libc::c_char = 0;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '\'' as i32 {
            break;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).line = ((*lex).line).wrapping_add(1);
            (*lex).line;
        }
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
    }
    return mln_lang_new(lex, LANG_TK_STRING);
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
unsafe extern "C" fn mln_lang_dblq_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    (*lex).result_pos = (*lex).result_buf;
    let mut c: libc::c_char = 0;
    loop {
        c = mln_lex_getchar(lex);
        if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == -(1 as libc::c_int) as libc::c_char as libc::c_int {
            (*lex).error = 8 as libc::c_int;
            return 0 as *mut mln_lang_struct_t;
        }
        if c as libc::c_int == '"' as i32 {
            break;
        }
        if c as libc::c_int == '\n' as i32 {
            (*lex).line = ((*lex).line).wrapping_add(1);
            (*lex).line;
        }
        if mln_get_char(lex, c) < 0 as libc::c_int {
            return 0 as *mut mln_lang_struct_t;
        }
    }
    return mln_lang_new(lex, LANG_TK_STRING);
}
unsafe extern "C" fn mln_lang_slash_handler(
    mut lex: *mut mln_lex_t,
    mut data: *mut libc::c_void,
) -> *mut mln_lang_struct_t {
    let mut c: libc::c_char = mln_lex_getchar(lex);
    if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
        return 0 as *mut mln_lang_struct_t;
    }
    if c as libc::c_int == '*' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        loop {
            c = mln_lex_getchar(lex);
            if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
                return 0 as *mut mln_lang_struct_t;
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
                        return 0 as *mut mln_lang_struct_t;
                    }
                    c = mln_lex_getchar(lex);
                    if c as libc::c_int
                        == -(2 as libc::c_int) as libc::c_char as libc::c_int
                    {
                        return 0 as *mut mln_lang_struct_t;
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
                                return 0 as *mut mln_lang_struct_t;
                            }
                            break;
                        }
                    }
                }
                if mln_lex_putchar(lex, c)
                    == -(2 as libc::c_int) as libc::c_char as libc::c_int
                {
                    return 0 as *mut mln_lang_struct_t;
                }
            }
        }
    } else if c as libc::c_int == '/' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        loop {
            c = mln_lex_getchar(lex);
            if c as libc::c_int == -(2 as libc::c_int) as libc::c_char as libc::c_int {
                return 0 as *mut mln_lang_struct_t;
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
                return 0 as *mut mln_lang_struct_t
            }
        }
    } else if c as libc::c_int == '=' as i32 {
        if mln_lex_putchar(lex, c) == -(2 as libc::c_int) as libc::c_char as libc::c_int
        {
            return 0 as *mut mln_lang_struct_t;
        }
        return mln_lang_new(lex, LANG_TK_DIVEQ);
    } else {
        mln_lex_stepback(lex, c);
        return mln_lang_new(lex, LANG_TK_SLASH);
    }
    (*lex).result_pos = (*lex).result_buf;
    return mln_lang_token(lex);
}
#[inline]
unsafe extern "C" fn mln_lang_stm_new(
    mut pool: *mut mln_alloc_t,
    mut data: *mut libc::c_void,
    mut type_0: mln_lang_stm_type_t,
    mut next: *mut mln_lang_stm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_stm_t {
    let mut ls: *mut mln_lang_stm_t = 0 as *mut mln_lang_stm_t;
    ls = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_stm_t>() as libc::c_ulong)
        as *mut mln_lang_stm_t;
    if ls.is_null() {
        return 0 as *mut mln_lang_stm_t;
    }
    (*ls).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ls).file = mln_string_pool_dup(pool, file);
            ((*ls).file).is_null()
        }
    {
        mln_alloc_free(ls as *mut libc::c_void);
        return 0 as *mut mln_lang_stm_t;
    }
    (*ls).line = line;
    (*ls).type_0 = type_0;
    match type_0 as libc::c_uint {
        0 => {
            (*ls).data.block = data as *mut mln_lang_block_t;
        }
        1 => {
            (*ls).data.func = data as *mut mln_lang_funcdef_t;
        }
        2 => {
            (*ls).data.setdef = data as *mut mln_lang_set_t;
        }
        3 => {
            (*ls).data.pos = data as *mut mln_string_t;
        }
        4 => {
            (*ls).data.sw = data as *mut mln_lang_switch_t;
        }
        5 => {
            (*ls).data.w = data as *mut mln_lang_while_t;
        }
        _ => {
            (*ls).data.f = data as *mut mln_lang_for_t;
        }
    }
    (*ls).next = next;
    (*ls).jump = 0 as *mut libc::c_void;
    (*ls).jump_type = 0 as libc::c_int;
    return ls;
}
unsafe extern "C" fn mln_lang_stm_free(mut data: *mut libc::c_void) {
    let mut stm: *mut mln_lang_stm_t = 0 as *mut mln_lang_stm_t;
    let mut next: *mut mln_lang_stm_t = data as *mut mln_lang_stm_t;
    let mut type_0: mln_lang_stm_type_t = M_STM_BLOCK;
    while !next.is_null() {
        stm = next;
        type_0 = (*stm).type_0;
        match type_0 as libc::c_uint {
            0 => {
                if !((*stm).data.block).is_null() {
                    mln_lang_block_free((*stm).data.block as *mut libc::c_void);
                }
            }
            1 => {
                if !((*stm).data.func).is_null() {
                    mln_lang_funcdef_free((*stm).data.func as *mut libc::c_void);
                }
            }
            2 => {
                if !((*stm).data.setdef).is_null() {
                    mln_lang_set_free((*stm).data.setdef as *mut libc::c_void);
                }
            }
            3 => {
                if !((*stm).data.pos).is_null() {
                    let mut __s: *mut mln_string_t = (*stm).data.pos;
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
                }
            }
            4 => {
                if !((*stm).data.sw).is_null() {
                    mln_lang_switch_free((*stm).data.sw as *mut libc::c_void);
                }
            }
            5 => {
                if !((*stm).data.w).is_null() {
                    mln_lang_while_free((*stm).data.w as *mut libc::c_void);
                }
            }
            _ => {
                if !((*stm).data.f).is_null() {
                    mln_lang_for_free((*stm).data.f as *mut libc::c_void);
                }
            }
        }
        next = (*stm).next;
        if !((*stm).file).is_null() {
            let mut __s: *mut mln_string_t = (*stm).file;
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
        mln_alloc_free(stm as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_funcdef_new(
    mut pool: *mut mln_alloc_t,
    mut name: *mut mln_string_t,
    mut exp: *mut mln_lang_exp_t,
    mut closure: *mut mln_lang_exp_t,
    mut stm: *mut mln_lang_stm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_funcdef_t {
    let mut lf: *mut mln_lang_funcdef_t = 0 as *mut mln_lang_funcdef_t;
    lf = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_funcdef_t>() as libc::c_ulong)
        as *mut mln_lang_funcdef_t;
    if lf.is_null() {
        return 0 as *mut mln_lang_funcdef_t;
    }
    (*lf).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lf).file = mln_string_pool_dup(pool, file);
            ((*lf).file).is_null()
        }
    {
        mln_alloc_free(lf as *mut libc::c_void);
        return 0 as *mut mln_lang_funcdef_t;
    }
    (*lf).line = line;
    (*lf).name = name;
    (*lf).args = exp;
    (*lf).stm = stm;
    (*lf).closure = closure;
    return lf;
}
unsafe extern "C" fn mln_lang_funcdef_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lf: *mut mln_lang_funcdef_t = data as *mut mln_lang_funcdef_t;
    if !((*lf).name).is_null() {
        let mut __s: *mut mln_string_t = (*lf).name;
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
    if !((*lf).args).is_null() {
        mln_lang_exp_free((*lf).args as *mut libc::c_void);
    }
    if !((*lf).stm).is_null() {
        mln_lang_stm_free((*lf).stm as *mut libc::c_void);
    }
    if !((*lf).closure).is_null() {
        mln_lang_exp_free((*lf).closure as *mut libc::c_void);
    }
    if !((*lf).file).is_null() {
        let mut __s: *mut mln_string_t = (*lf).file;
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
    mln_alloc_free(lf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_set_new(
    mut pool: *mut mln_alloc_t,
    mut name: *mut mln_string_t,
    mut stm: *mut mln_lang_setstm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_set_t {
    let mut lc: *mut mln_lang_set_t = 0 as *mut mln_lang_set_t;
    lc = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_set_t>() as libc::c_ulong)
        as *mut mln_lang_set_t;
    if lc.is_null() {
        return 0 as *mut mln_lang_set_t;
    }
    (*lc).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lc).file = mln_string_pool_dup(pool, file);
            ((*lc).file).is_null()
        }
    {
        mln_alloc_free(lc as *mut libc::c_void);
        return 0 as *mut mln_lang_set_t;
    }
    (*lc).line = line;
    (*lc).name = name;
    (*lc).stm = stm;
    return lc;
}
unsafe extern "C" fn mln_lang_set_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lc: *mut mln_lang_set_t = data as *mut mln_lang_set_t;
    if !((*lc).name).is_null() {
        let mut __s: *mut mln_string_t = (*lc).name;
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
    if !((*lc).stm).is_null() {
        mln_lang_setstm_free((*lc).stm as *mut libc::c_void);
    }
    if !((*lc).file).is_null() {
        let mut __s: *mut mln_string_t = (*lc).file;
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
    mln_alloc_free(lc as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_setstm_new(
    mut pool: *mut mln_alloc_t,
    mut data: *mut libc::c_void,
    mut type_0: mln_lang_setstm_type_t,
    mut next: *mut mln_lang_setstm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_setstm_t {
    let mut lc: *mut mln_lang_setstm_t = 0 as *mut mln_lang_setstm_t;
    lc = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_setstm_t>() as libc::c_ulong)
        as *mut mln_lang_setstm_t;
    if lc.is_null() {
        return 0 as *mut mln_lang_setstm_t;
    }
    (*lc).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lc).file = mln_string_pool_dup(pool, file);
            ((*lc).file).is_null()
        }
    {
        mln_alloc_free(lc as *mut libc::c_void);
        return 0 as *mut mln_lang_setstm_t;
    }
    (*lc).line = line;
    (*lc).type_0 = type_0;
    match type_0 as libc::c_uint {
        0 => {
            (*lc).data.var = data as *mut mln_string_t;
        }
        _ => {
            (*lc).data.func = data as *mut mln_lang_funcdef_t;
        }
    }
    (*lc).next = next;
    return lc;
}
unsafe extern "C" fn mln_lang_setstm_free(mut data: *mut libc::c_void) {
    let mut lc: *mut mln_lang_setstm_t = 0 as *mut mln_lang_setstm_t;
    let mut next: *mut mln_lang_setstm_t = data as *mut mln_lang_setstm_t;
    while !next.is_null() {
        lc = next;
        match (*lc).type_0 as libc::c_uint {
            0 => {
                if !((*lc).data.var).is_null() {
                    let mut __s: *mut mln_string_t = (*lc).data.var;
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
            }
            _ => {
                if !((*lc).data.func).is_null() {
                    mln_lang_funcdef_free((*lc).data.func as *mut libc::c_void);
                }
            }
        }
        next = (*lc).next;
        if !((*lc).file).is_null() {
            let mut __s: *mut mln_string_t = (*lc).file;
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
        mln_alloc_free(lc as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_block_new(
    mut pool: *mut mln_alloc_t,
    mut data: *mut libc::c_void,
    mut type_0: mln_lang_block_type_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_block_t {
    let mut lb: *mut mln_lang_block_t = 0 as *mut mln_lang_block_t;
    lb = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_block_t>() as libc::c_ulong)
        as *mut mln_lang_block_t;
    if lb.is_null() {
        return 0 as *mut mln_lang_block_t;
    }
    (*lb).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lb).file = mln_string_pool_dup(pool, file);
            ((*lb).file).is_null()
        }
    {
        mln_alloc_free(lb as *mut libc::c_void);
        return 0 as *mut mln_lang_block_t;
    }
    (*lb).line = line;
    (*lb).type_0 = type_0;
    match type_0 as libc::c_uint {
        0 | 4 => {
            (*lb).data.exp = data as *mut mln_lang_exp_t;
        }
        1 => {
            (*lb).data.stm = data as *mut mln_lang_stm_t;
        }
        5 => {
            (*lb).data.pos = data as *mut mln_string_t;
        }
        6 => {
            (*lb).data.i = data as *mut mln_lang_if_t;
        }
        _ => {
            (*lb).data.pos = 0 as *mut mln_string_t;
        }
    }
    (*lb).jump = 0 as *mut libc::c_void;
    (*lb).jump_type = 0 as libc::c_int;
    return lb;
}
unsafe extern "C" fn mln_lang_block_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lb: *mut mln_lang_block_t = data as *mut mln_lang_block_t;
    let mut type_0: mln_lang_block_type_t = (*lb).type_0;
    match type_0 as libc::c_uint {
        0 | 4 => {
            if !((*lb).data.exp).is_null() {
                mln_lang_exp_free((*lb).data.exp as *mut libc::c_void);
            }
        }
        1 => {
            if !((*lb).data.stm).is_null() {
                mln_lang_stm_free((*lb).data.stm as *mut libc::c_void);
            }
        }
        5 => {
            if !((*lb).data.pos).is_null() {
                let mut __s: *mut mln_string_t = (*lb).data.pos;
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
        }
        6 => {
            if !((*lb).data.i).is_null() {
                mln_lang_if_free((*lb).data.i as *mut libc::c_void);
            }
        }
        _ => {}
    }
    if !((*lb).file).is_null() {
        let mut __s: *mut mln_string_t = (*lb).file;
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
    }
    mln_alloc_free(lb as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_switch_new(
    mut pool: *mut mln_alloc_t,
    mut exp: *mut mln_lang_exp_t,
    mut switchstm: *mut mln_lang_switchstm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_switch_t {
    let mut ls: *mut mln_lang_switch_t = 0 as *mut mln_lang_switch_t;
    ls = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_switch_t>() as libc::c_ulong)
        as *mut mln_lang_switch_t;
    if ls.is_null() {
        return 0 as *mut mln_lang_switch_t;
    }
    (*ls).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ls).file = mln_string_pool_dup(pool, file);
            ((*ls).file).is_null()
        }
    {
        mln_alloc_free(ls as *mut libc::c_void);
        return 0 as *mut mln_lang_switch_t;
    }
    (*ls).line = line;
    (*ls).condition = exp;
    (*ls).switchstm = switchstm;
    return ls;
}
unsafe extern "C" fn mln_lang_switch_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut ls: *mut mln_lang_switch_t = data as *mut mln_lang_switch_t;
    if !((*ls).condition).is_null() {
        mln_lang_exp_free((*ls).condition as *mut libc::c_void);
    }
    if !((*ls).switchstm).is_null() {
        mln_lang_switchstm_free((*ls).switchstm as *mut libc::c_void);
    }
    if !((*ls).file).is_null() {
        let mut __s: *mut mln_string_t = (*ls).file;
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
    }
    mln_alloc_free(ls as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_switchstm_new(
    mut pool: *mut mln_alloc_t,
    mut factor: *mut mln_lang_factor_t,
    mut stm: *mut mln_lang_stm_t,
    mut next: *mut mln_lang_switchstm_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_switchstm_t {
    let mut ls: *mut mln_lang_switchstm_t = 0 as *mut mln_lang_switchstm_t;
    ls = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_switchstm_t>() as libc::c_ulong,
    ) as *mut mln_lang_switchstm_t;
    if ls.is_null() {
        return 0 as *mut mln_lang_switchstm_t;
    }
    (*ls).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ls).file = mln_string_pool_dup(pool, file);
            ((*ls).file).is_null()
        }
    {
        mln_alloc_free(ls as *mut libc::c_void);
        return 0 as *mut mln_lang_switchstm_t;
    }
    (*ls).line = line;
    (*ls).factor = factor;
    (*ls).stm = stm;
    (*ls).next = next;
    return ls;
}
unsafe extern "C" fn mln_lang_switchstm_free(mut data: *mut libc::c_void) {
    let mut ls: *mut mln_lang_switchstm_t = 0 as *mut mln_lang_switchstm_t;
    let mut next: *mut mln_lang_switchstm_t = data as *mut mln_lang_switchstm_t;
    while !next.is_null() {
        ls = next;
        if !((*ls).factor).is_null() {
            mln_lang_factor_free((*ls).factor as *mut libc::c_void);
        }
        if !((*ls).stm).is_null() {
            mln_lang_stm_free((*ls).stm as *mut libc::c_void);
        }
        next = (*ls).next;
        if !((*ls).file).is_null() {
            let mut __s: *mut mln_string_t = (*ls).file;
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
        }
        mln_alloc_free(ls as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_while_new(
    mut pool: *mut mln_alloc_t,
    mut exp: *mut mln_lang_exp_t,
    mut blockstm: *mut mln_lang_block_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_while_t {
    let mut lw: *mut mln_lang_while_t = 0 as *mut mln_lang_while_t;
    lw = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_while_t>() as libc::c_ulong)
        as *mut mln_lang_while_t;
    if lw.is_null() {
        return 0 as *mut mln_lang_while_t;
    }
    (*lw).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lw).file = mln_string_pool_dup(pool, file);
            ((*lw).file).is_null()
        }
    {
        mln_alloc_free(lw as *mut libc::c_void);
        return 0 as *mut mln_lang_while_t;
    }
    (*lw).line = line;
    (*lw).condition = exp;
    (*lw).blockstm = blockstm;
    return lw;
}
unsafe extern "C" fn mln_lang_while_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lw: *mut mln_lang_while_t = data as *mut mln_lang_while_t;
    if !((*lw).condition).is_null() {
        mln_lang_exp_free((*lw).condition as *mut libc::c_void);
    }
    if !((*lw).blockstm).is_null() {
        mln_lang_block_free((*lw).blockstm as *mut libc::c_void);
    }
    if !((*lw).file).is_null() {
        let mut __s: *mut mln_string_t = (*lw).file;
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
    }
    mln_alloc_free(lw as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_for_new(
    mut pool: *mut mln_alloc_t,
    mut init_exp: *mut mln_lang_exp_t,
    mut condition: *mut mln_lang_exp_t,
    mut mod_exp: *mut mln_lang_exp_t,
    mut blockstm: *mut mln_lang_block_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_for_t {
    let mut lf: *mut mln_lang_for_t = 0 as *mut mln_lang_for_t;
    lf = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_for_t>() as libc::c_ulong)
        as *mut mln_lang_for_t;
    if lf.is_null() {
        return 0 as *mut mln_lang_for_t;
    }
    (*lf).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lf).file = mln_string_pool_dup(pool, file);
            ((*lf).file).is_null()
        }
    {
        mln_alloc_free(lf as *mut libc::c_void);
        return 0 as *mut mln_lang_for_t;
    }
    (*lf).line = line;
    (*lf).init_exp = init_exp;
    (*lf).condition = condition;
    (*lf).mod_exp = mod_exp;
    (*lf).blockstm = blockstm;
    return lf;
}
unsafe extern "C" fn mln_lang_for_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lf: *mut mln_lang_for_t = data as *mut mln_lang_for_t;
    if !((*lf).init_exp).is_null() {
        mln_lang_exp_free((*lf).init_exp as *mut libc::c_void);
    }
    if !((*lf).condition).is_null() {
        mln_lang_exp_free((*lf).condition as *mut libc::c_void);
    }
    if !((*lf).mod_exp).is_null() {
        mln_lang_exp_free((*lf).mod_exp as *mut libc::c_void);
    }
    if !((*lf).blockstm).is_null() {
        mln_lang_block_free((*lf).blockstm as *mut libc::c_void);
    }
    if !((*lf).file).is_null() {
        let mut __s: *mut mln_string_t = (*lf).file;
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
    }
    mln_alloc_free(lf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_if_new(
    mut pool: *mut mln_alloc_t,
    mut condition: *mut mln_lang_exp_t,
    mut truestm: *mut mln_lang_block_t,
    mut falsestm: *mut mln_lang_block_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_if_t {
    let mut li: *mut mln_lang_if_t = 0 as *mut mln_lang_if_t;
    li = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_if_t>() as libc::c_ulong)
        as *mut mln_lang_if_t;
    if li.is_null() {
        return 0 as *mut mln_lang_if_t;
    }
    (*li).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*li).file = mln_string_pool_dup(pool, file);
            ((*li).file).is_null()
        }
    {
        mln_alloc_free(li as *mut libc::c_void);
        return 0 as *mut mln_lang_if_t;
    }
    (*li).line = line;
    (*li).condition = condition;
    (*li).blockstm = truestm;
    (*li).elsestm = falsestm;
    return li;
}
unsafe extern "C" fn mln_lang_if_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut li: *mut mln_lang_if_t = data as *mut mln_lang_if_t;
    if !((*li).condition).is_null() {
        mln_lang_exp_free((*li).condition as *mut libc::c_void);
    }
    if !((*li).blockstm).is_null() {
        mln_lang_block_free((*li).blockstm as *mut libc::c_void);
    }
    if !((*li).elsestm).is_null() {
        mln_lang_block_free((*li).elsestm as *mut libc::c_void);
    }
    if !((*li).file).is_null() {
        let mut __s: *mut mln_string_t = (*li).file;
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
    }
    mln_alloc_free(li as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_exp_new(
    mut pool: *mut mln_alloc_t,
    mut assign: *mut mln_lang_assign_t,
    mut next: *mut mln_lang_exp_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_exp_t {
    let mut le: *mut mln_lang_exp_t = 0 as *mut mln_lang_exp_t;
    le = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_exp_t>() as libc::c_ulong)
        as *mut mln_lang_exp_t;
    if le.is_null() {
        return 0 as *mut mln_lang_exp_t;
    }
    (*le).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*le).file = mln_string_pool_dup(pool, file);
            ((*le).file).is_null()
        }
    {
        mln_alloc_free(le as *mut libc::c_void);
        return 0 as *mut mln_lang_exp_t;
    }
    (*le).line = line;
    (*le).assign = assign;
    (*le).next = next;
    (*le).jump = 0 as *mut libc::c_void;
    (*le).type_0 = 0 as libc::c_int;
    return le;
}
unsafe extern "C" fn mln_lang_exp_free(mut data: *mut libc::c_void) {
    let mut le: *mut mln_lang_exp_t = 0 as *mut mln_lang_exp_t;
    let mut next: *mut mln_lang_exp_t = data as *mut mln_lang_exp_t;
    while !next.is_null() {
        le = next;
        if !((*le).assign).is_null() {
            mln_lang_assign_free((*le).assign as *mut libc::c_void);
        }
        next = (*le).next;
        if !((*le).file).is_null() {
            let mut __s: *mut mln_string_t = (*le).file;
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
        }
        mln_alloc_free(le as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_assign_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_logiclow_t,
    mut op: mln_lang_assign_op_t,
    mut right: *mut mln_lang_assign_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_assign_t {
    let mut la: *mut mln_lang_assign_t = 0 as *mut mln_lang_assign_t;
    la = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_assign_t>() as libc::c_ulong)
        as *mut mln_lang_assign_t;
    if la.is_null() {
        return 0 as *mut mln_lang_assign_t;
    }
    (*la).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*la).file = mln_string_pool_dup(pool, file);
            ((*la).file).is_null()
        }
    {
        mln_alloc_free(la as *mut libc::c_void);
        return 0 as *mut mln_lang_assign_t;
    }
    (*la).line = line;
    (*la).left = left;
    (*la).op = op;
    (*la).right = right;
    (*la).jump = 0 as *mut libc::c_void;
    (*la).type_0 = 0 as libc::c_int;
    return la;
}
unsafe extern "C" fn mln_lang_assign_free(mut data: *mut libc::c_void) {
    let mut la: *mut mln_lang_assign_t = 0 as *mut mln_lang_assign_t;
    let mut right: *mut mln_lang_assign_t = data as *mut mln_lang_assign_t;
    while !right.is_null() {
        la = right;
        if !((*la).left).is_null() {
            mln_lang_logiclow_free((*la).left as *mut libc::c_void);
        }
        right = (*la).right;
        if !((*la).file).is_null() {
            let mut __s: *mut mln_string_t = (*la).file;
            if !__s.is_null() {
                let ref mut fresh51 = (*__s).ref_0();
                let fresh52 = *fresh51;
                *fresh51 = (*fresh51).wrapping_sub(1);
                if fresh52 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(la as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_assign_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_assign_op_t,
    mut assign: *mut mln_lang_assign_t,
) -> *mut mln_lang_assign_tmp_t {
    let mut lat: *mut mln_lang_assign_tmp_t = 0 as *mut mln_lang_assign_tmp_t;
    lat = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_assign_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_assign_tmp_t;
    if lat.is_null() {
        return 0 as *mut mln_lang_assign_tmp_t;
    }
    (*lat).op = op;
    (*lat).assign = assign;
    return lat;
}
unsafe extern "C" fn mln_lang_assign_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lat: *mut mln_lang_assign_tmp_t = data as *mut mln_lang_assign_tmp_t;
    if !((*lat).assign).is_null() {
        mln_lang_assign_free((*lat).assign as *mut libc::c_void);
    }
    mln_alloc_free(lat as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_logiclow_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_logichigh_t,
    mut op: mln_lang_logiclow_op_t,
    mut right: *mut mln_lang_logiclow_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_logiclow_t {
    let mut ll: *mut mln_lang_logiclow_t = 0 as *mut mln_lang_logiclow_t;
    ll = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_logiclow_t>() as libc::c_ulong,
    ) as *mut mln_lang_logiclow_t;
    if ll.is_null() {
        return 0 as *mut mln_lang_logiclow_t;
    }
    (*ll).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ll).file = mln_string_pool_dup(pool, file);
            ((*ll).file).is_null()
        }
    {
        mln_alloc_free(ll as *mut libc::c_void);
        return 0 as *mut mln_lang_logiclow_t;
    }
    (*ll).line = line;
    (*ll).left = left;
    (*ll).op = op;
    (*ll).right = right;
    (*ll).jump = 0 as *mut libc::c_void;
    (*ll).type_0 = 0 as libc::c_int;
    return ll;
}
unsafe extern "C" fn mln_lang_logiclow_free(mut data: *mut libc::c_void) {
    let mut ll: *mut mln_lang_logiclow_t = 0 as *mut mln_lang_logiclow_t;
    let mut right: *mut mln_lang_logiclow_t = data as *mut mln_lang_logiclow_t;
    while !right.is_null() {
        ll = right;
        if !((*ll).left).is_null() {
            mln_lang_logichigh_free((*ll).left as *mut libc::c_void);
        }
        right = (*ll).right;
        if !((*ll).file).is_null() {
            let mut __s: *mut mln_string_t = (*ll).file;
            if !__s.is_null() {
                let ref mut fresh53 = (*__s).ref_0();
                let fresh54 = *fresh53;
                *fresh53 = (*fresh53).wrapping_sub(1);
                if fresh54 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(ll as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_logiclow_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_logiclow_op_t,
    mut logiclow: *mut mln_lang_logiclow_t,
) -> *mut mln_lang_logiclow_tmp_t {
    let mut llt: *mut mln_lang_logiclow_tmp_t = 0 as *mut mln_lang_logiclow_tmp_t;
    llt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_logiclow_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_logiclow_tmp_t;
    if llt.is_null() {
        return 0 as *mut mln_lang_logiclow_tmp_t;
    }
    (*llt).op = op;
    (*llt).logiclow = logiclow;
    return llt;
}
unsafe extern "C" fn mln_lang_logiclow_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut llt: *mut mln_lang_logiclow_tmp_t = data as *mut mln_lang_logiclow_tmp_t;
    if !((*llt).logiclow).is_null() {
        mln_lang_logiclow_free((*llt).logiclow as *mut libc::c_void);
    }
    mln_alloc_free(llt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_logichigh_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_relativelow_t,
    mut op: mln_lang_logichigh_op_t,
    mut right: *mut mln_lang_logichigh_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_logichigh_t {
    let mut ll: *mut mln_lang_logichigh_t = 0 as *mut mln_lang_logichigh_t;
    ll = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_logichigh_t>() as libc::c_ulong,
    ) as *mut mln_lang_logichigh_t;
    if ll.is_null() {
        return 0 as *mut mln_lang_logichigh_t;
    }
    (*ll).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ll).file = mln_string_pool_dup(pool, file);
            ((*ll).file).is_null()
        }
    {
        mln_alloc_free(ll as *mut libc::c_void);
        return 0 as *mut mln_lang_logichigh_t;
    }
    (*ll).line = line;
    (*ll).left = left;
    (*ll).op = op;
    (*ll).right = right;
    (*ll).jump = 0 as *mut libc::c_void;
    (*ll).type_0 = 0 as libc::c_int;
    return ll;
}
unsafe extern "C" fn mln_lang_logichigh_free(mut data: *mut libc::c_void) {
    let mut ll: *mut mln_lang_logichigh_t = 0 as *mut mln_lang_logichigh_t;
    let mut right: *mut mln_lang_logichigh_t = data as *mut mln_lang_logichigh_t;
    while !right.is_null() {
        ll = right;
        if !((*ll).left).is_null() {
            mln_lang_relativelow_free((*ll).left as *mut libc::c_void);
        }
        right = (*ll).right;
        if !((*ll).file).is_null() {
            let mut __s: *mut mln_string_t = (*ll).file;
            if !__s.is_null() {
                let ref mut fresh55 = (*__s).ref_0();
                let fresh56 = *fresh55;
                *fresh55 = (*fresh55).wrapping_sub(1);
                if fresh56 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(ll as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_logichigh_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_logichigh_op_t,
    mut logichigh: *mut mln_lang_logichigh_t,
) -> *mut mln_lang_logichigh_tmp_t {
    let mut llt: *mut mln_lang_logichigh_tmp_t = 0 as *mut mln_lang_logichigh_tmp_t;
    llt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_logichigh_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_logichigh_tmp_t;
    if llt.is_null() {
        return 0 as *mut mln_lang_logichigh_tmp_t;
    }
    (*llt).op = op;
    (*llt).logichigh = logichigh;
    return llt;
}
unsafe extern "C" fn mln_lang_logichigh_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut llt: *mut mln_lang_logichigh_tmp_t = data as *mut mln_lang_logichigh_tmp_t;
    if !((*llt).logichigh).is_null() {
        mln_lang_logichigh_free((*llt).logichigh as *mut libc::c_void);
    }
    mln_alloc_free(llt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_relativelow_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_relativehigh_t,
    mut op: mln_lang_relativelow_op_t,
    mut right: *mut mln_lang_relativelow_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_relativelow_t {
    let mut lr: *mut mln_lang_relativelow_t = 0 as *mut mln_lang_relativelow_t;
    lr = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_relativelow_t>() as libc::c_ulong,
    ) as *mut mln_lang_relativelow_t;
    if lr.is_null() {
        return 0 as *mut mln_lang_relativelow_t;
    }
    (*lr).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lr).file = mln_string_pool_dup(pool, file);
            ((*lr).file).is_null()
        }
    {
        mln_alloc_free(lr as *mut libc::c_void);
        return 0 as *mut mln_lang_relativelow_t;
    }
    (*lr).line = line;
    (*lr).left = left;
    (*lr).op = op;
    (*lr).right = right;
    (*lr).jump = 0 as *mut libc::c_void;
    (*lr).type_0 = 0 as libc::c_int;
    return lr;
}
unsafe extern "C" fn mln_lang_relativelow_free(mut data: *mut libc::c_void) {
    let mut lr: *mut mln_lang_relativelow_t = 0 as *mut mln_lang_relativelow_t;
    let mut right: *mut mln_lang_relativelow_t = data as *mut mln_lang_relativelow_t;
    while !right.is_null() {
        lr = right;
        if !((*lr).left).is_null() {
            mln_lang_relativehigh_free((*lr).left as *mut libc::c_void);
        }
        right = (*lr).right;
        if !((*lr).file).is_null() {
            let mut __s: *mut mln_string_t = (*lr).file;
            if !__s.is_null() {
                let ref mut fresh57 = (*__s).ref_0();
                let fresh58 = *fresh57;
                *fresh57 = (*fresh57).wrapping_sub(1);
                if fresh58 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(lr as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_relativelow_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_relativelow_op_t,
    mut relativelow: *mut mln_lang_relativelow_t,
) -> *mut mln_lang_relativelow_tmp_t {
    let mut lrt: *mut mln_lang_relativelow_tmp_t = 0 as *mut mln_lang_relativelow_tmp_t;
    lrt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_relativelow_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_relativelow_tmp_t;
    if lrt.is_null() {
        return 0 as *mut mln_lang_relativelow_tmp_t;
    }
    (*lrt).op = op;
    (*lrt).relativelow = relativelow;
    return lrt;
}
unsafe extern "C" fn mln_lang_relativelow_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lrt: *mut mln_lang_relativelow_tmp_t = data
        as *mut mln_lang_relativelow_tmp_t;
    if !((*lrt).relativelow).is_null() {
        mln_lang_relativelow_free((*lrt).relativelow as *mut libc::c_void);
    }
    mln_alloc_free(lrt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_relativehigh_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_move_t,
    mut op: mln_lang_relativehigh_op_t,
    mut right: *mut mln_lang_relativehigh_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_relativehigh_t {
    let mut lr: *mut mln_lang_relativehigh_t = 0 as *mut mln_lang_relativehigh_t;
    lr = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_relativehigh_t>() as libc::c_ulong,
    ) as *mut mln_lang_relativehigh_t;
    if lr.is_null() {
        return 0 as *mut mln_lang_relativehigh_t;
    }
    (*lr).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lr).file = mln_string_pool_dup(pool, file);
            ((*lr).file).is_null()
        }
    {
        mln_alloc_free(lr as *mut libc::c_void);
        return 0 as *mut mln_lang_relativehigh_t;
    }
    (*lr).line = line;
    (*lr).left = left;
    (*lr).op = op;
    (*lr).right = right;
    (*lr).jump = 0 as *mut libc::c_void;
    (*lr).type_0 = 0 as libc::c_int;
    return lr;
}
unsafe extern "C" fn mln_lang_relativehigh_free(mut data: *mut libc::c_void) {
    let mut lr: *mut mln_lang_relativehigh_t = 0 as *mut mln_lang_relativehigh_t;
    let mut right: *mut mln_lang_relativehigh_t = data as *mut mln_lang_relativehigh_t;
    while !right.is_null() {
        lr = right;
        if !((*lr).left).is_null() {
            mln_lang_move_free((*lr).left as *mut libc::c_void);
        }
        right = (*lr).right;
        if !((*lr).file).is_null() {
            let mut __s: *mut mln_string_t = (*lr).file;
            if !__s.is_null() {
                let ref mut fresh59 = (*__s).ref_0();
                let fresh60 = *fresh59;
                *fresh59 = (*fresh59).wrapping_sub(1);
                if fresh60 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(lr as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_relativehigh_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_relativehigh_op_t,
    mut relativehigh: *mut mln_lang_relativehigh_t,
) -> *mut mln_lang_relativehigh_tmp_t {
    let mut lrt: *mut mln_lang_relativehigh_tmp_t = 0
        as *mut mln_lang_relativehigh_tmp_t;
    lrt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_relativehigh_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_relativehigh_tmp_t;
    if lrt.is_null() {
        return 0 as *mut mln_lang_relativehigh_tmp_t;
    }
    (*lrt).op = op;
    (*lrt).relativehigh = relativehigh;
    return lrt;
}
unsafe extern "C" fn mln_lang_relativehigh_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lrt: *mut mln_lang_relativehigh_tmp_t = data
        as *mut mln_lang_relativehigh_tmp_t;
    if !((*lrt).relativehigh).is_null() {
        mln_lang_relativehigh_free((*lrt).relativehigh as *mut libc::c_void);
    }
    mln_alloc_free(lrt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_move_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_addsub_t,
    mut op: mln_lang_move_op_t,
    mut right: *mut mln_lang_move_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_move_t {
    let mut lm: *mut mln_lang_move_t = 0 as *mut mln_lang_move_t;
    lm = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_move_t>() as libc::c_ulong)
        as *mut mln_lang_move_t;
    if lm.is_null() {
        return 0 as *mut mln_lang_move_t;
    }
    (*lm).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lm).file = mln_string_pool_dup(pool, file);
            ((*lm).file).is_null()
        }
    {
        mln_alloc_free(lm as *mut libc::c_void);
        return 0 as *mut mln_lang_move_t;
    }
    (*lm).line = line;
    (*lm).left = left;
    (*lm).op = op;
    (*lm).right = right;
    (*lm).jump = 0 as *mut libc::c_void;
    (*lm).type_0 = 0 as libc::c_int;
    return lm;
}
unsafe extern "C" fn mln_lang_move_free(mut data: *mut libc::c_void) {
    let mut lm: *mut mln_lang_move_t = 0 as *mut mln_lang_move_t;
    let mut right: *mut mln_lang_move_t = data as *mut mln_lang_move_t;
    while !right.is_null() {
        lm = right;
        if !((*lm).left).is_null() {
            mln_lang_addsub_free((*lm).left as *mut libc::c_void);
        }
        right = (*lm).right;
        if !((*lm).file).is_null() {
            let mut __s: *mut mln_string_t = (*lm).file;
            if !__s.is_null() {
                let ref mut fresh61 = (*__s).ref_0();
                let fresh62 = *fresh61;
                *fresh61 = (*fresh61).wrapping_sub(1);
                if fresh62 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(lm as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_move_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_move_op_t,
    mut move_0: *mut mln_lang_move_t,
) -> *mut mln_lang_move_tmp_t {
    let mut lmt: *mut mln_lang_move_tmp_t = 0 as *mut mln_lang_move_tmp_t;
    lmt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_move_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_move_tmp_t;
    if lmt.is_null() {
        return 0 as *mut mln_lang_move_tmp_t;
    }
    (*lmt).op = op;
    (*lmt).move_0 = move_0;
    return lmt;
}
unsafe extern "C" fn mln_lang_move_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lmt: *mut mln_lang_move_tmp_t = data as *mut mln_lang_move_tmp_t;
    if !((*lmt).move_0).is_null() {
        mln_lang_move_free((*lmt).move_0 as *mut libc::c_void);
    }
    mln_alloc_free(lmt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_addsub_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_muldiv_t,
    mut op: mln_lang_addsub_op_t,
    mut right: *mut mln_lang_addsub_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_addsub_t {
    let mut la: *mut mln_lang_addsub_t = 0 as *mut mln_lang_addsub_t;
    la = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_addsub_t>() as libc::c_ulong)
        as *mut mln_lang_addsub_t;
    if la.is_null() {
        return 0 as *mut mln_lang_addsub_t;
    }
    (*la).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*la).file = mln_string_pool_dup(pool, file);
            ((*la).file).is_null()
        }
    {
        mln_alloc_free(la as *mut libc::c_void);
        return 0 as *mut mln_lang_addsub_t;
    }
    (*la).line = line;
    (*la).left = left;
    (*la).op = op;
    (*la).right = right;
    (*la).jump = 0 as *mut libc::c_void;
    (*la).type_0 = 0 as libc::c_int;
    return la;
}
unsafe extern "C" fn mln_lang_addsub_free(mut data: *mut libc::c_void) {
    let mut la: *mut mln_lang_addsub_t = 0 as *mut mln_lang_addsub_t;
    let mut right: *mut mln_lang_addsub_t = data as *mut mln_lang_addsub_t;
    while !right.is_null() {
        la = right;
        if !((*la).left).is_null() {
            mln_lang_muldiv_free((*la).left as *mut libc::c_void);
        }
        right = (*la).right;
        if !((*la).file).is_null() {
            let mut __s: *mut mln_string_t = (*la).file;
            if !__s.is_null() {
                let ref mut fresh63 = (*__s).ref_0();
                let fresh64 = *fresh63;
                *fresh63 = (*fresh63).wrapping_sub(1);
                if fresh64 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(la as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_addsub_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_addsub_op_t,
    mut addsub: *mut mln_lang_addsub_t,
) -> *mut mln_lang_addsub_tmp_t {
    let mut lat: *mut mln_lang_addsub_tmp_t = 0 as *mut mln_lang_addsub_tmp_t;
    lat = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_addsub_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_addsub_tmp_t;
    if lat.is_null() {
        return 0 as *mut mln_lang_addsub_tmp_t;
    }
    (*lat).op = op;
    (*lat).addsub = addsub;
    return lat;
}
unsafe extern "C" fn mln_lang_addsub_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lat: *mut mln_lang_addsub_tmp_t = data as *mut mln_lang_addsub_tmp_t;
    if !((*lat).addsub).is_null() {
        mln_lang_addsub_free((*lat).addsub as *mut libc::c_void);
    }
    mln_alloc_free(lat as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_muldiv_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_not_t,
    mut op: mln_lang_muldiv_op_t,
    mut right: *mut mln_lang_muldiv_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_muldiv_t {
    let mut lm: *mut mln_lang_muldiv_t = 0 as *mut mln_lang_muldiv_t;
    lm = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_muldiv_t>() as libc::c_ulong)
        as *mut mln_lang_muldiv_t;
    if lm.is_null() {
        return 0 as *mut mln_lang_muldiv_t;
    }
    (*lm).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lm).file = mln_string_pool_dup(pool, file);
            ((*lm).file).is_null()
        }
    {
        mln_alloc_free(lm as *mut libc::c_void);
        return 0 as *mut mln_lang_muldiv_t;
    }
    (*lm).line = line;
    (*lm).left = left;
    (*lm).op = op;
    (*lm).right = right;
    (*lm).jump = 0 as *mut libc::c_void;
    (*lm).type_0 = 0 as libc::c_int;
    return lm;
}
unsafe extern "C" fn mln_lang_muldiv_free(mut data: *mut libc::c_void) {
    let mut lm: *mut mln_lang_muldiv_t = 0 as *mut mln_lang_muldiv_t;
    let mut right: *mut mln_lang_muldiv_t = data as *mut mln_lang_muldiv_t;
    while !right.is_null() {
        lm = right;
        if !((*lm).left).is_null() {
            mln_lang_not_free((*lm).left as *mut libc::c_void);
        }
        right = (*lm).right;
        if !((*lm).file).is_null() {
            let mut __s: *mut mln_string_t = (*lm).file;
            if !__s.is_null() {
                let ref mut fresh65 = (*__s).ref_0();
                let fresh66 = *fresh65;
                *fresh65 = (*fresh65).wrapping_sub(1);
                if fresh66 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(lm as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_muldiv_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_muldiv_op_t,
    mut muldiv: *mut mln_lang_muldiv_t,
) -> *mut mln_lang_muldiv_tmp_t {
    let mut lmt: *mut mln_lang_muldiv_tmp_t = 0 as *mut mln_lang_muldiv_tmp_t;
    lmt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_muldiv_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_muldiv_tmp_t;
    if lmt.is_null() {
        return 0 as *mut mln_lang_muldiv_tmp_t;
    }
    (*lmt).op = op;
    (*lmt).muldiv = muldiv;
    return lmt;
}
unsafe extern "C" fn mln_lang_muldiv_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lmt: *mut mln_lang_muldiv_tmp_t = data as *mut mln_lang_muldiv_tmp_t;
    if !((*lmt).muldiv).is_null() {
        mln_lang_muldiv_free((*lmt).muldiv as *mut libc::c_void);
    }
    mln_alloc_free(lmt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_not_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_not_op_t,
    mut data: *mut libc::c_void,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_not_t {
    let mut ln: *mut mln_lang_not_t = 0 as *mut mln_lang_not_t;
    ln = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_not_t>() as libc::c_ulong)
        as *mut mln_lang_not_t;
    if ln.is_null() {
        return 0 as *mut mln_lang_not_t;
    }
    (*ln).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ln).file = mln_string_pool_dup(pool, file);
            ((*ln).file).is_null()
        }
    {
        mln_alloc_free(ln as *mut libc::c_void);
        return 0 as *mut mln_lang_not_t;
    }
    (*ln).line = line;
    (*ln).op = op;
    match op as libc::c_uint {
        1 => {
            (*ln).right.not = data as *mut mln_lang_not_t;
        }
        _ => {
            (*ln).right.suffix = data as *mut mln_lang_suffix_t;
        }
    }
    (*ln).jump = 0 as *mut libc::c_void;
    (*ln).type_0 = 0 as libc::c_int;
    return ln;
}
unsafe extern "C" fn mln_lang_not_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut ln: *mut mln_lang_not_t = data as *mut mln_lang_not_t;
    let mut fr: *mut mln_lang_not_t = 0 as *mut mln_lang_not_t;
    loop {
        fr = ln;
        if !((*ln).op as libc::c_uint == M_NOT_NOT as libc::c_int as libc::c_uint) {
            break;
        }
        ln = (*ln).right.not;
        if !((*fr).file).is_null() {
            let mut __s: *mut mln_string_t = (*fr).file;
            if !__s.is_null() {
                let ref mut fresh67 = (*__s).ref_0();
                let fresh68 = *fresh67;
                *fresh67 = (*fresh67).wrapping_sub(1);
                if fresh68 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(fr as *mut libc::c_void);
    }
    mln_lang_suffix_free((*fr).right.suffix as *mut libc::c_void);
    if !((*fr).file).is_null() {
        let mut __s: *mut mln_string_t = (*fr).file;
        if !__s.is_null() {
            let ref mut fresh69 = (*__s).ref_0();
            let fresh70 = *fresh69;
            *fresh69 = (*fresh69).wrapping_sub(1);
            if fresh70 <= 1 as libc::c_int as libc::c_ulong {
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
    mln_alloc_free(fr as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_suffix_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_locate_t,
    mut op: mln_lang_suffix_op_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_suffix_t {
    let mut ls: *mut mln_lang_suffix_t = 0 as *mut mln_lang_suffix_t;
    ls = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_suffix_t>() as libc::c_ulong)
        as *mut mln_lang_suffix_t;
    if ls.is_null() {
        return 0 as *mut mln_lang_suffix_t;
    }
    (*ls).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ls).file = mln_string_pool_dup(pool, file);
            ((*ls).file).is_null()
        }
    {
        mln_alloc_free(ls as *mut libc::c_void);
        return 0 as *mut mln_lang_suffix_t;
    }
    (*ls).line = line;
    (*ls).left = left;
    (*ls).op = op;
    (*ls).jump = 0 as *mut libc::c_void;
    (*ls).type_0 = 0 as libc::c_int;
    return ls;
}
unsafe extern "C" fn mln_lang_suffix_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut ls: *mut mln_lang_suffix_t = data as *mut mln_lang_suffix_t;
    if !((*ls).left).is_null() {
        mln_lang_locate_free((*ls).left as *mut libc::c_void);
    }
    if !((*ls).file).is_null() {
        let mut __s: *mut mln_string_t = (*ls).file;
        if !__s.is_null() {
            let ref mut fresh71 = (*__s).ref_0();
            let fresh72 = *fresh71;
            *fresh71 = (*fresh71).wrapping_sub(1);
            if fresh72 <= 1 as libc::c_int as libc::c_ulong {
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
    mln_alloc_free(ls as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_suffix_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_suffix_op_t,
) -> *mut mln_lang_suffix_tmp_t {
    let mut lst: *mut mln_lang_suffix_tmp_t = 0 as *mut mln_lang_suffix_tmp_t;
    lst = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_suffix_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_suffix_tmp_t;
    if lst.is_null() {
        return 0 as *mut mln_lang_suffix_tmp_t;
    }
    (*lst).op = op;
    return lst;
}
unsafe extern "C" fn mln_lang_suffix_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    mln_alloc_free(data);
}
#[inline]
unsafe extern "C" fn mln_lang_locate_new(
    mut pool: *mut mln_alloc_t,
    mut left: *mut mln_lang_spec_t,
    mut op: mln_lang_locate_op_t,
    mut right: *mut libc::c_void,
    mut next: *mut mln_lang_locate_tmp_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_locate_t {
    let mut ll: *mut mln_lang_locate_t = 0 as *mut mln_lang_locate_t;
    let mut n: *mut mln_lang_locate_t = 0 as *mut mln_lang_locate_t;
    ll = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_locate_t>() as libc::c_ulong)
        as *mut mln_lang_locate_t;
    if ll.is_null() {
        return 0 as *mut mln_lang_locate_t;
    }
    (*ll).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ll).file = mln_string_pool_dup(pool, file);
            ((*ll).file).is_null()
        }
    {
        mln_alloc_free(ll as *mut libc::c_void);
        return 0 as *mut mln_lang_locate_t;
    }
    (*ll).line = line;
    (*ll).left = left;
    (*ll).op = op;
    match op as libc::c_uint {
        1 | 3 => {
            (*ll).right.exp = right as *mut mln_lang_exp_t;
        }
        2 => {
            (*ll).right.id = right as *mut mln_string_t;
        }
        _ => {
            (*ll).right.exp = 0 as *mut mln_lang_exp_t;
        }
    }
    (*ll).jump = 0 as *mut libc::c_void;
    (*ll).type_0 = 0 as libc::c_int;
    (*ll).next = 0 as *mut mln_lang_locate_t;
    if !next.is_null() {
        let mut tmp: *mut mln_lang_locate_t = ll;
        while !next.is_null() {
            n = mln_alloc_m(
                pool,
                ::core::mem::size_of::<mln_lang_locate_t>() as libc::c_ulong,
            ) as *mut mln_lang_locate_t;
            if n.is_null() {
                mln_lang_locate_free(ll as *mut libc::c_void);
                return 0 as *mut mln_lang_locate_t;
            }
            (*n).file = 0 as *mut mln_string_t;
            if !file.is_null()
                && {
                    (*n).file = mln_string_pool_dup(pool, file);
                    ((*n).file).is_null()
                }
            {
                mln_alloc_free(n as *mut libc::c_void);
                mln_lang_locate_free(ll as *mut libc::c_void);
                return 0 as *mut mln_lang_locate_t;
            }
            (*n).line = line;
            (*n).left = 0 as *mut mln_lang_spec_t;
            (*n).op = (*next).op;
            (*n).next = 0 as *mut mln_lang_locate_t;
            match (*n).op as libc::c_uint {
                1 | 3 => {
                    (*n).right.exp = (*next).locate.exp;
                    (*next).locate.exp = 0 as *mut mln_lang_exp_t;
                }
                2 => {
                    (*n).right.id = (*next).locate.id;
                    (*next).locate.id = 0 as *mut mln_string_t;
                }
                _ => {
                    (*n).right.exp = 0 as *mut mln_lang_exp_t;
                }
            }
            (*tmp).next = n;
            tmp = n;
            next = (*next).next;
        }
    }
    return ll;
}
unsafe extern "C" fn mln_lang_locate_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut ll: *mut mln_lang_locate_t = data as *mut mln_lang_locate_t;
    let mut next: *mut mln_lang_locate_t = 0 as *mut mln_lang_locate_t;
    let mut op: mln_lang_locate_op_t = M_LOCATE_NONE;
    loop {
        op = (*ll).op;
        if !((*ll).left).is_null() {
            mln_lang_spec_free((*ll).left as *mut libc::c_void);
        }
        match op as libc::c_uint {
            1 | 3 => {
                if !((*ll).right.exp).is_null() {
                    mln_lang_exp_free((*ll).right.exp as *mut libc::c_void);
                }
            }
            2 => {
                if !((*ll).right.id).is_null() {
                    let mut __s: *mut mln_string_t = (*ll).right.id;
                    if !__s.is_null() {
                        let ref mut fresh73 = (*__s).ref_0();
                        let fresh74 = *fresh73;
                        *fresh73 = (*fresh73).wrapping_sub(1);
                        if fresh74 <= 1 as libc::c_int as libc::c_ulong {
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
            }
            _ => {}
        }
        next = (*ll).next;
        if !((*ll).file).is_null() {
            let mut __s: *mut mln_string_t = (*ll).file;
            if !__s.is_null() {
                let ref mut fresh75 = (*__s).ref_0();
                let fresh76 = *fresh75;
                *fresh75 = (*fresh75).wrapping_sub(1);
                if fresh76 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(ll as *mut libc::c_void);
        if next.is_null() {
            break;
        }
        ll = next;
    };
}
#[inline]
unsafe extern "C" fn mln_lang_locate_tmp_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_locate_op_t,
    mut data: *mut libc::c_void,
    mut next: *mut mln_lang_locate_tmp_t,
) -> *mut mln_lang_locate_tmp_t {
    let mut llt: *mut mln_lang_locate_tmp_t = 0 as *mut mln_lang_locate_tmp_t;
    llt = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_locate_tmp_t>() as libc::c_ulong,
    ) as *mut mln_lang_locate_tmp_t;
    if llt.is_null() {
        return 0 as *mut mln_lang_locate_tmp_t;
    }
    (*llt).op = op;
    match op as libc::c_uint {
        1 | 3 => {
            (*llt).locate.exp = data as *mut mln_lang_exp_t;
        }
        2 => {
            (*llt).locate.id = data as *mut mln_string_t;
        }
        _ => {
            (*llt).locate.exp = 0 as *mut mln_lang_exp_t;
        }
    }
    (*llt).next = next;
    return llt;
}
unsafe extern "C" fn mln_lang_locate_tmp_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut llt: *mut mln_lang_locate_tmp_t = data as *mut mln_lang_locate_tmp_t;
    let mut op: mln_lang_locate_op_t = (*llt).op;
    match op as libc::c_uint {
        1 | 3 => {
            if !((*llt).locate.exp).is_null() {
                mln_lang_assign_free((*llt).locate.exp as *mut libc::c_void);
            }
        }
        2 => {
            if !((*llt).locate.id).is_null() {
                let mut __s: *mut mln_string_t = (*llt).locate.id;
                if !__s.is_null() {
                    let ref mut fresh77 = (*__s).ref_0();
                    let fresh78 = *fresh77;
                    *fresh77 = (*fresh77).wrapping_sub(1);
                    if fresh78 <= 1 as libc::c_int as libc::c_ulong {
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
        }
        _ => {}
    }
    if !((*llt).next).is_null() {
        mln_lang_locate_tmp_free((*llt).next as *mut libc::c_void);
    }
    mln_alloc_free(llt as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_spec_new(
    mut pool: *mut mln_alloc_t,
    mut op: mln_lang_spec_op_t,
    mut data: *mut libc::c_void,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_spec_t {
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_spec_t>() as libc::c_ulong)
        as *mut mln_lang_spec_t;
    if ls.is_null() {
        return 0 as *mut mln_lang_spec_t;
    }
    (*ls).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*ls).file = mln_string_pool_dup(pool, file);
            ((*ls).file).is_null()
        }
    {
        mln_alloc_free(ls as *mut libc::c_void);
        return 0 as *mut mln_lang_spec_t;
    }
    (*ls).line = line;
    (*ls).op = op;
    match op as libc::c_uint {
        0 | 1 | 2 | 3 | 4 => {
            (*ls).data.spec = data as *mut mln_lang_spec_t;
        }
        5 => {
            (*ls).data.set_name = data as *mut mln_string_t;
        }
        6 => {
            (*ls).data.exp = data as *mut mln_lang_exp_t;
        }
        _ => {
            (*ls).data.factor = data as *mut mln_lang_factor_t;
        }
    }
    return ls;
}
unsafe extern "C" fn mln_lang_spec_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    let mut right: *mut mln_lang_spec_t = data as *mut mln_lang_spec_t;
    let mut op: mln_lang_spec_op_t = M_SPEC_NEGATIVE;
    while !right.is_null() {
        ls = right;
        op = (*ls).op;
        match op as libc::c_uint {
            0 | 1 | 2 | 3 | 4 => {
                right = (*ls).data.spec;
            }
            5 => {
                if !((*ls).data.set_name).is_null() {
                    let mut __s: *mut mln_string_t = (*ls).data.set_name;
                    if !__s.is_null() {
                        let ref mut fresh79 = (*__s).ref_0();
                        let fresh80 = *fresh79;
                        *fresh79 = (*fresh79).wrapping_sub(1);
                        if fresh80 <= 1 as libc::c_int as libc::c_ulong {
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
                right = 0 as *mut mln_lang_spec_t;
            }
            6 => {
                if !((*ls).data.exp).is_null() {
                    mln_lang_exp_free((*ls).data.exp as *mut libc::c_void);
                }
                right = 0 as *mut mln_lang_spec_t;
            }
            _ => {
                if !((*ls).data.factor).is_null() {
                    mln_lang_factor_free((*ls).data.factor as *mut libc::c_void);
                }
                right = 0 as *mut mln_lang_spec_t;
            }
        }
        if !((*ls).file).is_null() {
            let mut __s: *mut mln_string_t = (*ls).file;
            if !__s.is_null() {
                let ref mut fresh81 = (*__s).ref_0();
                let fresh82 = *fresh81;
                *fresh81 = (*fresh81).wrapping_sub(1);
                if fresh82 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(ls as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn mln_lang_factor_new(
    mut pool: *mut mln_alloc_t,
    mut type_0: mln_lang_factor_type_t,
    mut data: *mut libc::c_void,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_factor_t {
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_alloc_m(pool, ::core::mem::size_of::<mln_lang_factor_t>() as libc::c_ulong)
        as *mut mln_lang_factor_t;
    if lf.is_null() {
        return 0 as *mut mln_lang_factor_t;
    }
    (*lf).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*lf).file = mln_string_pool_dup(pool, file);
            ((*lf).file).is_null()
        }
    {
        mln_alloc_free(lf as *mut libc::c_void);
        return 0 as *mut mln_lang_factor_t;
    }
    (*lf).line = line;
    (*lf).type_0 = type_0;
    match type_0 as libc::c_uint {
        0 => {
            (*lf).data.b = *(data as mln_u8ptr_t);
        }
        1 | 5 => {
            (*lf).data.s_id = data as *mut mln_string_t;
        }
        2 => {
            (*lf).data.i = *(data as *mut mln_s64_t);
        }
        3 => {
            (*lf).data.f = *(data as *mut libc::c_double);
        }
        4 => {
            (*lf).data.array = data as *mut mln_lang_elemlist_t;
        }
        _ => {
            (*lf).data.ptr = 0 as *mut mln_u8ptr_t;
        }
    }
    return lf;
}
unsafe extern "C" fn mln_lang_factor_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut lf: *mut mln_lang_factor_t = data as *mut mln_lang_factor_t;
    let mut type_0: mln_lang_factor_type_t = (*lf).type_0;
    match type_0 as libc::c_uint {
        1 | 5 => {
            if !((*lf).data.s_id).is_null() {
                let mut __s: *mut mln_string_t = (*lf).data.s_id;
                if !__s.is_null() {
                    let ref mut fresh83 = (*__s).ref_0();
                    let fresh84 = *fresh83;
                    *fresh83 = (*fresh83).wrapping_sub(1);
                    if fresh84 <= 1 as libc::c_int as libc::c_ulong {
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
        }
        4 => {
            if !((*lf).data.array).is_null() {
                mln_lang_elemlist_free((*lf).data.array as *mut libc::c_void);
            }
        }
        0 | 2 | 3 | _ => {}
    }
    if !((*lf).file).is_null() {
        let mut __s: *mut mln_string_t = (*lf).file;
        if !__s.is_null() {
            let ref mut fresh85 = (*__s).ref_0();
            let fresh86 = *fresh85;
            *fresh85 = (*fresh85).wrapping_sub(1);
            if fresh86 <= 1 as libc::c_int as libc::c_ulong {
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
    mln_alloc_free(lf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_lang_elemlist_new(
    mut pool: *mut mln_alloc_t,
    mut key: *mut mln_lang_assign_t,
    mut val: *mut mln_lang_assign_t,
    mut next: *mut mln_lang_elemlist_t,
    mut line: mln_u64_t,
    mut file: *mut mln_string_t,
) -> *mut mln_lang_elemlist_t {
    let mut le: *mut mln_lang_elemlist_t = 0 as *mut mln_lang_elemlist_t;
    le = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_lang_elemlist_t>() as libc::c_ulong,
    ) as *mut mln_lang_elemlist_t;
    if le.is_null() {
        return 0 as *mut mln_lang_elemlist_t;
    }
    (*le).file = 0 as *mut mln_string_t;
    if !file.is_null()
        && {
            (*le).file = mln_string_pool_dup(pool, file);
            ((*le).file).is_null()
        }
    {
        mln_alloc_free(le as *mut libc::c_void);
        return 0 as *mut mln_lang_elemlist_t;
    }
    (*le).line = line;
    (*le).key = key;
    (*le).val = val;
    (*le).next = next;
    return le;
}
unsafe extern "C" fn mln_lang_elemlist_free(mut data: *mut libc::c_void) {
    let mut le: *mut mln_lang_elemlist_t = 0 as *mut mln_lang_elemlist_t;
    let mut next: *mut mln_lang_elemlist_t = data as *mut mln_lang_elemlist_t;
    while !next.is_null() {
        le = next;
        if !((*le).key).is_null() {
            mln_lang_assign_free((*le).key as *mut libc::c_void);
        }
        if !((*le).val).is_null() {
            mln_lang_assign_free((*le).val as *mut libc::c_void);
        }
        next = (*le).next;
        if !((*le).file).is_null() {
            let mut __s: *mut mln_string_t = (*le).file;
            if !__s.is_null() {
                let ref mut fresh87 = (*__s).ref_0();
                let fresh88 = *fresh87;
                *fresh87 = (*fresh87).wrapping_sub(1);
                if fresh88 <= 1 as libc::c_int as libc::c_ulong {
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
        mln_alloc_free(le as *mut libc::c_void);
    }
}
unsafe extern "C" fn mln_lang_semantic_start(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(0 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(0 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh89 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh89 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_stm_block(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut stm: *mut mln_lang_stm_t = 0 as *mut mln_lang_stm_t;
    stm = mln_lang_stm_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data,
        M_STM_BLOCK,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh90 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh90 = 0 as *mut libc::c_void;
    let ref mut fresh91 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh91 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_stmfunc(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut stm: *mut mln_lang_stm_t = 0 as *mut mln_lang_stm_t;
    stm = mln_lang_stm_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data,
        M_STM_FUNC,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        return -(1 as libc::c_int);
    }
    let ref mut fresh92 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh92 = 0 as *mut libc::c_void;
    let ref mut fresh93 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh93 = 0 as *mut libc::c_void;
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_stmset(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut stm: *mut mln_lang_stm_t = 0 as *mut mln_lang_stm_t;
    let mut clas: *mut mln_lang_set_t = 0 as *mut mln_lang_set_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut name: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    clas = mln_lang_set_new(
        pool,
        name,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_setstm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if clas.is_null() {
        let mut __s: *mut mln_string_t = name;
        if !__s.is_null() {
            let ref mut fresh94 = (*__s).ref_0();
            let fresh95 = *fresh94;
            *fresh94 = (*fresh94).wrapping_sub(1);
            if fresh95 <= 1 as libc::c_int as libc::c_ulong {
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
    stm = mln_lang_stm_new(
        pool,
        clas as *mut libc::c_void,
        M_STM_SET,
        (**right.offset(4 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        mln_lang_set_free(clas as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let ref mut fresh96 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh96 = 0 as *mut libc::c_void;
    let ref mut fresh97 = (**right.offset(4 as libc::c_int as isize)).data;
    *fresh97 = 0 as *mut libc::c_void;
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_setstm_var(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut var: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if var.is_null() {
        return -(1 as libc::c_int);
    }
    let mut lc: *mut mln_lang_setstm_t = mln_lang_setstm_new(
        pool,
        var as *mut libc::c_void,
        M_SETSTM_VAR,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_setstm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lc.is_null() {
        let mut __s: *mut mln_string_t = var;
        if !__s.is_null() {
            let ref mut fresh98 = (*__s).ref_0();
            let fresh99 = *fresh98;
            *fresh98 = (*fresh98).wrapping_sub(1);
            if fresh99 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = lc as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_setstm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh100 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh100 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_setstm_func(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lc: *mut mln_lang_setstm_t = mln_lang_setstm_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data,
        M_SETSTM_FUNC,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_setstm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lc.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lc as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_setstm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh101 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh101 = 0 as *mut libc::c_void;
    let ref mut fresh102 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh102 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_blockstmexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lb: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data,
        M_BLOCK_EXP,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lb.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lb as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh103 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh103 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_blockstmstm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lb: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        (**right.offset(1 as libc::c_int as isize)).data,
        M_BLOCK_STM,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lb.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lb as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh104 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh104 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_labelstm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut label: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if label.is_null() {
        return -(1 as libc::c_int);
    }
    let mut stm: *mut mln_lang_stm_t = mln_lang_stm_new(
        pool,
        label as *mut libc::c_void,
        M_STM_LABEL,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        let mut __s: *mut mln_string_t = label;
        if !__s.is_null() {
            let ref mut fresh105 = (*__s).ref_0();
            let fresh106 = *fresh105;
            *fresh105 = (*fresh105).wrapping_sub(1);
            if fresh106 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh107 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh107 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_whilestm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut w: *mut mln_lang_while_t = mln_lang_while_new(
        pool,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(4 as libc::c_int as isize)).data as *mut mln_lang_block_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if w.is_null() {
        return -(1 as libc::c_int);
    }
    let mut stm: *mut mln_lang_stm_t = mln_lang_stm_new(
        pool,
        w as *mut libc::c_void,
        M_STM_WHILE,
        (**right.offset(5 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        mln_lang_while_free(w as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh108 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh108 = 0 as *mut libc::c_void;
    let ref mut fresh109 = (**right.offset(4 as libc::c_int as isize)).data;
    *fresh109 = 0 as *mut libc::c_void;
    let ref mut fresh110 = (**right.offset(5 as libc::c_int as isize)).data;
    *fresh110 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_forstm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut f: *mut mln_lang_for_t = mln_lang_for_new(
        pool,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(4 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(6 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(8 as libc::c_int as isize)).data as *mut mln_lang_block_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    let mut stm: *mut mln_lang_stm_t = mln_lang_stm_new(
        pool,
        f as *mut libc::c_void,
        M_STM_FOR,
        (**right.offset(9 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        mln_lang_for_free(f as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh111 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh111 = 0 as *mut libc::c_void;
    let ref mut fresh112 = (**right.offset(4 as libc::c_int as isize)).data;
    *fresh112 = 0 as *mut libc::c_void;
    let ref mut fresh113 = (**right.offset(6 as libc::c_int as isize)).data;
    *fresh113 = 0 as *mut libc::c_void;
    let ref mut fresh114 = (**right.offset(8 as libc::c_int as isize)).data;
    *fresh114 = 0 as *mut libc::c_void;
    let ref mut fresh115 = (**right.offset(9 as libc::c_int as isize)).data;
    *fresh115 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_ifstm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut i: *mut mln_lang_if_t = mln_lang_if_new(
        pool,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(4 as libc::c_int as isize)).data as *mut mln_lang_block_t,
        (**right.offset(5 as libc::c_int as isize)).data as *mut mln_lang_block_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if i.is_null() {
        return -(1 as libc::c_int);
    }
    let mut lb: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        i as *mut libc::c_void,
        M_BLOCK_IF,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lb.is_null() {
        mln_lang_if_free(i as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*left).data = lb as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh116 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh116 = 0 as *mut libc::c_void;
    let ref mut fresh117 = (**right.offset(4 as libc::c_int as isize)).data;
    *fresh117 = 0 as *mut libc::c_void;
    let ref mut fresh118 = (**right.offset(5 as libc::c_int as isize)).data;
    *fresh118 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_switchstm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut sw: *mut mln_lang_switch_t = mln_lang_switch_new(
        pool,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(5 as libc::c_int as isize)).data as *mut mln_lang_switchstm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if sw.is_null() {
        return -(1 as libc::c_int);
    }
    let mut stm: *mut mln_lang_stm_t = mln_lang_stm_new(
        pool,
        sw as *mut libc::c_void,
        M_STM_SWITCH,
        (**right.offset(7 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if stm.is_null() {
        mln_lang_switch_free(sw as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*left).data = stm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_stm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh119 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh119 = 0 as *mut libc::c_void;
    let ref mut fresh120 = (**right.offset(5 as libc::c_int as isize)).data;
    *fresh120 = 0 as *mut libc::c_void;
    let ref mut fresh121 = (**right.offset(7 as libc::c_int as isize)).data;
    *fresh121 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_funcdef(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(1 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut name: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    let mut lf: *mut mln_lang_funcdef_t = mln_lang_funcdef_new(
        pool,
        name,
        (**right.offset(3 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(5 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (**right.offset(7 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        let mut __s: *mut mln_string_t = name;
        if !__s.is_null() {
            let ref mut fresh122 = (*__s).ref_0();
            let fresh123 = *fresh122;
            *fresh122 = (*fresh122).wrapping_sub(1);
            if fresh123 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_funcdef_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh124 = (**right.offset(3 as libc::c_int as isize)).data;
    *fresh124 = 0 as *mut libc::c_void;
    let ref mut fresh125 = (**right.offset(5 as libc::c_int as isize)).data;
    *fresh125 = 0 as *mut libc::c_void;
    let ref mut fresh126 = (**right.offset(7 as libc::c_int as isize)).data;
    *fresh126 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_funcdef_closure(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(2 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(2 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh127 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh127 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_switchstm__(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_switchstm_t = mln_lang_switchstm_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_factor_t,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_stm_t,
        (**right.offset(3 as libc::c_int as isize)).data as *mut mln_lang_switchstm_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_switchstm_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh128 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh128 = 0 as *mut libc::c_void;
    let ref mut fresh129 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh129 = 0 as *mut libc::c_void;
    let ref mut fresh130 = (**right.offset(3 as libc::c_int as isize)).data;
    *fresh130 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_switchprefix(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(1 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(1 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh131 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh131 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_elsestm(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(1 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(1 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh132 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh132 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_continue(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut block: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        0 as *mut libc::c_void,
        M_BLOCK_CONTINUE,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if block.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = block as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_break(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut block: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        0 as *mut libc::c_void,
        M_BLOCK_BREAK,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if block.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = block as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_return(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut block: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        (**right.offset(1 as libc::c_int as isize)).data,
        M_BLOCK_RETURN,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if block.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = block as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh133 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh133 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_goto(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(1 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut pos: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if pos.is_null() {
        return -(1 as libc::c_int);
    }
    let mut block: *mut mln_lang_block_t = mln_lang_block_new(
        pool,
        pos as *mut libc::c_void,
        M_BLOCK_GOTO,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if block.is_null() {
        let mut __s: *mut mln_string_t = pos;
        if !__s.is_null() {
            let ref mut fresh134 = (*__s).ref_0();
            let fresh135 = *fresh134;
            *fresh134 = (*fresh134).wrapping_sub(1);
            if fresh135 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = block as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_block_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_exp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut exp: *mut mln_lang_exp_t = mln_lang_exp_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_exp_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if exp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = exp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_exp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh136 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh136 = 0 as *mut libc::c_void;
    let ref mut fresh137 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh137 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_explist(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(1 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(1 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh138 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh138 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut assign: *mut mln_lang_assign_t = 0 as *mut mln_lang_assign_t;
    let mut r: *mut mln_lang_assign_t = 0 as *mut mln_lang_assign_t;
    let mut op: mln_lang_assign_op_t = M_ASSIGN_NONE;
    let mut tmp: *mut mln_lang_assign_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_assign_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).assign;
        (*tmp).assign = 0 as *mut mln_lang_assign_t;
    }
    assign = mln_lang_assign_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_logiclow_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if assign.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = assign as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh139 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh139 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpeq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_EQUAL,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh140 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh140 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexppluseq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_PLUSEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh141 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh141 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpsubeq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_SUBEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh142 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh142 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexplmoveq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_LMOVEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh143 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh143 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexprmoveq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_RMOVEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh144 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh144 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpmuleq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_MULEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh145 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh145 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpdiveq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_DIVEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh146 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh146 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexporeq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_OREQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh147 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh147 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpandeq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_ANDEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh148 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh148 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpxoreq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_XOREQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh149 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh149 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_assignexpmodeq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_assign_tmp_t = mln_lang_assign_tmp_new(
        pool,
        M_ASSIGN_MODEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_assign_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_assign_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh150 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh150 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logiclowexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ll: *mut mln_lang_logiclow_t = 0 as *mut mln_lang_logiclow_t;
    let mut r: *mut mln_lang_logiclow_t = 0 as *mut mln_lang_logiclow_t;
    let mut op: mln_lang_logiclow_op_t = M_LOGICLOW_NONE;
    let mut tmp: *mut mln_lang_logiclow_tmp_t = (**right
        .offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_logiclow_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).logiclow;
        (*tmp).logiclow = 0 as *mut mln_lang_logiclow_t;
    }
    ll = mln_lang_logiclow_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_logichigh_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ll.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ll as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logiclow_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh151 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh151 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logiclowexpor(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_logiclow_tmp_t = 0 as *mut mln_lang_logiclow_tmp_t;
    tmp = mln_lang_logiclow_tmp_new(
        pool,
        M_LOGICLOW_OR,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_logiclow_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logiclow_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh152 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh152 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logiclowexpand(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_logiclow_tmp_t = 0 as *mut mln_lang_logiclow_tmp_t;
    tmp = mln_lang_logiclow_tmp_new(
        pool,
        M_LOGICLOW_AND,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_logiclow_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logiclow_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh153 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh153 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logichighexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ll: *mut mln_lang_logichigh_t = 0 as *mut mln_lang_logichigh_t;
    let mut r: *mut mln_lang_logichigh_t = 0 as *mut mln_lang_logichigh_t;
    let mut op: mln_lang_logichigh_op_t = M_LOGICHIGH_NONE;
    let mut tmp: *mut mln_lang_logichigh_tmp_t = (**right
        .offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_logichigh_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).logichigh;
        (*tmp).logichigh = 0 as *mut mln_lang_logichigh_t;
    }
    ll = mln_lang_logichigh_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_relativelow_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ll.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ll as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logichigh_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh154 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh154 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logichighor(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_logichigh_tmp_t = 0 as *mut mln_lang_logichigh_tmp_t;
    tmp = mln_lang_logichigh_tmp_new(
        pool,
        M_LOGICHIGH_OR,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_logichigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logichigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh155 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh155 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logichighand(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_logichigh_tmp_t = 0 as *mut mln_lang_logichigh_tmp_t;
    tmp = mln_lang_logichigh_tmp_new(
        pool,
        M_LOGICHIGH_AND,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_logichigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logichigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh156 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh156 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_logichighxor(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_logichigh_tmp_t = 0 as *mut mln_lang_logichigh_tmp_t;
    tmp = mln_lang_logichigh_tmp_new(
        pool,
        M_LOGICHIGH_XOR,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_logichigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_logichigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh157 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh157 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativelowexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lr: *mut mln_lang_relativelow_t = 0 as *mut mln_lang_relativelow_t;
    let mut r: *mut mln_lang_relativelow_t = 0 as *mut mln_lang_relativelow_t;
    let mut op: mln_lang_relativelow_op_t = M_RELATIVELOW_NONE;
    let mut tmp: *mut mln_lang_relativelow_tmp_t = (**right
        .offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_relativelow_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).relativelow;
        (*tmp).relativelow = 0 as *mut mln_lang_relativelow_t;
    }
    lr = mln_lang_relativelow_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_relativehigh_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lr.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lr as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativelow_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh158 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh158 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativeloweq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativelow_tmp_t = 0 as *mut mln_lang_relativelow_tmp_t;
    tmp = mln_lang_relativelow_tmp_new(
        pool,
        M_RELATIVELOW_EQUAL,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativelow_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativelow_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh159 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh159 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativelownoneq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativelow_tmp_t = 0 as *mut mln_lang_relativelow_tmp_t;
    tmp = mln_lang_relativelow_tmp_new(
        pool,
        M_RELATIVELOW_NEQUAL,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativelow_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativelow_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh160 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh160 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativehighexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lr: *mut mln_lang_relativehigh_t = 0 as *mut mln_lang_relativehigh_t;
    let mut r: *mut mln_lang_relativehigh_t = 0 as *mut mln_lang_relativehigh_t;
    let mut op: mln_lang_relativehigh_op_t = M_RELATIVEHIGH_NONE;
    let mut tmp: *mut mln_lang_relativehigh_tmp_t = (**right
        .offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_relativehigh_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).relativehigh;
        (*tmp).relativehigh = 0 as *mut mln_lang_relativehigh_t;
    }
    lr = mln_lang_relativehigh_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_move_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lr.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lr as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativehigh_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh161 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh161 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativehighless(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativehigh_tmp_t = 0
        as *mut mln_lang_relativehigh_tmp_t;
    tmp = mln_lang_relativehigh_tmp_new(
        pool,
        M_RELATIVEHIGH_LESS,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativehigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativehigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh162 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh162 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativehighlesseq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativehigh_tmp_t = 0
        as *mut mln_lang_relativehigh_tmp_t;
    tmp = mln_lang_relativehigh_tmp_new(
        pool,
        M_RELATIVEHIGH_LESSEQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativehigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativehigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh163 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh163 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativehighgreater(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativehigh_tmp_t = 0
        as *mut mln_lang_relativehigh_tmp_t;
    tmp = mln_lang_relativehigh_tmp_new(
        pool,
        M_RELATIVEHIGH_GREATER,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativehigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativehigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh164 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh164 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_relativehighgreatereq(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_relativehigh_tmp_t = 0
        as *mut mln_lang_relativehigh_tmp_t;
    tmp = mln_lang_relativehigh_tmp_new(
        pool,
        M_RELATIVEHIGH_GREATEREQ,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_relativehigh_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_relativehigh_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh165 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh165 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_moveexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut move_0: *mut mln_lang_move_t = 0 as *mut mln_lang_move_t;
    let mut r: *mut mln_lang_move_t = 0 as *mut mln_lang_move_t;
    let mut op: mln_lang_move_op_t = M_MOVE_NONE;
    let mut tmp: *mut mln_lang_move_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_move_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).move_0;
        (*tmp).move_0 = 0 as *mut mln_lang_move_t;
    }
    move_0 = mln_lang_move_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_addsub_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if move_0.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = move_0 as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_move_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh166 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh166 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_moveleft(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_move_tmp_t = 0 as *mut mln_lang_move_tmp_t;
    tmp = mln_lang_move_tmp_new(
        pool,
        M_MOVE_LMOVE,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_move_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_move_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh167 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh167 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_moveright(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_move_tmp_t = 0 as *mut mln_lang_move_tmp_t;
    tmp = mln_lang_move_tmp_new(
        pool,
        M_MOVE_RMOVE,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_move_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_move_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh168 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh168 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_addsubexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut la: *mut mln_lang_addsub_t = 0 as *mut mln_lang_addsub_t;
    let mut r: *mut mln_lang_addsub_t = 0 as *mut mln_lang_addsub_t;
    let mut op: mln_lang_addsub_op_t = M_ADDSUB_NONE;
    let mut tmp: *mut mln_lang_addsub_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_addsub_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).addsub;
        (*tmp).addsub = 0 as *mut mln_lang_addsub_t;
    }
    la = mln_lang_addsub_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_muldiv_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if la.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = la as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_addsub_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh169 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh169 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_addsubplus(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_addsub_tmp_t = 0 as *mut mln_lang_addsub_tmp_t;
    tmp = mln_lang_addsub_tmp_new(
        pool,
        M_ADDSUB_PLUS,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_addsub_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_addsub_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh170 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh170 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_addsubsub(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_addsub_tmp_t = 0 as *mut mln_lang_addsub_tmp_t;
    tmp = mln_lang_addsub_tmp_new(
        pool,
        M_ADDSUB_SUB,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_addsub_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_addsub_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh171 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh171 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_muldivexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lm: *mut mln_lang_muldiv_t = 0 as *mut mln_lang_muldiv_t;
    let mut r: *mut mln_lang_muldiv_t = 0 as *mut mln_lang_muldiv_t;
    let mut op: mln_lang_muldiv_op_t = M_MULDIV_NONE;
    let mut tmp: *mut mln_lang_muldiv_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_muldiv_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
        r = (*tmp).muldiv;
        (*tmp).muldiv = 0 as *mut mln_lang_muldiv_t;
    }
    lm = mln_lang_muldiv_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_not_t,
        op,
        r,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lm.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lm as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_muldiv_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh172 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh172 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_muldivmul(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_muldiv_tmp_t = 0 as *mut mln_lang_muldiv_tmp_t;
    tmp = mln_lang_muldiv_tmp_new(
        pool,
        M_MULDIV_MUL,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_muldiv_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_muldiv_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh173 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh173 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_muldivdiv(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_muldiv_tmp_t = 0 as *mut mln_lang_muldiv_tmp_t;
    tmp = mln_lang_muldiv_tmp_new(
        pool,
        M_MULDIV_DIV,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_muldiv_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_muldiv_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh174 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh174 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_muldivmod(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_muldiv_tmp_t = 0 as *mut mln_lang_muldiv_tmp_t;
    tmp = mln_lang_muldiv_tmp_new(
        pool,
        M_MULDIV_MOD,
        (**right.offset(1 as libc::c_int as isize)).data as *mut mln_lang_muldiv_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_muldiv_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh175 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh175 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_notnot(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ln: *mut mln_lang_not_t = 0 as *mut mln_lang_not_t;
    ln = mln_lang_not_new(
        pool,
        M_NOT_NOT,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ln.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ln as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_not_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh176 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh176 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_notsuffix(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ln: *mut mln_lang_not_t = 0 as *mut mln_lang_not_t;
    ln = mln_lang_not_new(
        pool,
        M_NOT_NONE,
        (**right.offset(0 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ln.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ln as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_not_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh177 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh177 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_suffixexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_suffix_t = 0 as *mut mln_lang_suffix_t;
    let mut op: mln_lang_suffix_op_t = M_SUFFIX_NONE;
    let mut tmp: *mut mln_lang_suffix_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_suffix_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
    }
    ls = mln_lang_suffix_new(
        pool,
        (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_locate_t,
        op,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_suffix_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh178 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh178 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_suffixinc(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_suffix_tmp_t = 0 as *mut mln_lang_suffix_tmp_t;
    tmp = mln_lang_suffix_tmp_new(pool, M_SUFFIX_INC);
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_suffix_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_suffixdec(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_suffix_tmp_t = 0 as *mut mln_lang_suffix_tmp_t;
    tmp = mln_lang_suffix_tmp_new(pool, M_SUFFIX_DEC);
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_suffix_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_locateexp(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ll: *mut mln_lang_locate_t = 0 as *mut mln_lang_locate_t;
    let mut op: mln_lang_locate_op_t = M_LOCATE_NONE;
    let mut tmp: *mut mln_lang_locate_tmp_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_locate_tmp_t;
    if !tmp.is_null() {
        op = (*tmp).op;
    }
    match op as libc::c_uint {
        1 | 3 => {
            ll = mln_lang_locate_new(
                pool,
                (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_spec_t,
                op,
                (*tmp).locate.exp as *mut libc::c_void,
                (*tmp).next,
                (*left).line as mln_u64_t,
                (*left).file,
            );
            if ll.is_null() {
                return -(1 as libc::c_int);
            }
            (*tmp).locate.exp = 0 as *mut mln_lang_exp_t;
        }
        2 => {
            ll = mln_lang_locate_new(
                pool,
                (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_spec_t,
                op,
                (*tmp).locate.id as *mut libc::c_void,
                (*tmp).next,
                (*left).line as mln_u64_t,
                (*left).file,
            );
            if ll.is_null() {
                return -(1 as libc::c_int);
            }
            (*tmp).locate.id = 0 as *mut mln_string_t;
        }
        _ => {
            ll = mln_lang_locate_new(
                pool,
                (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_spec_t,
                op,
                0 as *mut libc::c_void,
                0 as *mut mln_lang_locate_tmp_t,
                (*left).line as mln_u64_t,
                (*left).file,
            );
            if ll.is_null() {
                return -(1 as libc::c_int);
            }
        }
    }
    (*left).data = ll as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_locate_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh179 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh179 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_locateindex(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_locate_tmp_t = 0 as *mut mln_lang_locate_tmp_t;
    tmp = mln_lang_locate_tmp_new(
        pool,
        M_LOCATE_INDEX,
        (**right.offset(1 as libc::c_int as isize)).data,
        (**right.offset(3 as libc::c_int as isize)).data as *mut mln_lang_locate_tmp_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_locate_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh180 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh180 = 0 as *mut libc::c_void;
    let ref mut fresh181 = (**right.offset(3 as libc::c_int as isize)).data;
    *fresh181 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_locateproperty(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_locate_tmp_t = 0 as *mut mln_lang_locate_tmp_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(1 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut id: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if id.is_null() {
        return -(1 as libc::c_int);
    }
    tmp = mln_lang_locate_tmp_new(
        pool,
        M_LOCATE_PROPERTY,
        id as *mut libc::c_void,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_locate_tmp_t,
    );
    if tmp.is_null() {
        let mut __s: *mut mln_string_t = id;
        if !__s.is_null() {
            let ref mut fresh182 = (*__s).ref_0();
            let fresh183 = *fresh182;
            *fresh182 = (*fresh182).wrapping_sub(1);
            if fresh183 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_locate_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh184 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh184 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_locatefunc(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut tmp: *mut mln_lang_locate_tmp_t = 0 as *mut mln_lang_locate_tmp_t;
    tmp = mln_lang_locate_tmp_new(
        pool,
        M_LOCATE_FUNC,
        (**right.offset(1 as libc::c_int as isize)).data,
        (**right.offset(3 as libc::c_int as isize)).data as *mut mln_lang_locate_tmp_t,
    );
    if tmp.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = tmp as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_locate_tmp_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh185 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh185 = 0 as *mut libc::c_void;
    let ref mut fresh186 = (**right.offset(3 as libc::c_int as isize)).data;
    *fresh186 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specnegative(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_NEGATIVE,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh187 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh187 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specreverse(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_REVERSE,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh188 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh188 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specrefer(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_REFER,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh189 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh189 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specinc(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_INC,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh190 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh190 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specdec(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_DEC,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh191 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh191 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specnew(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut spec: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(1 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut name: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    spec = mln_lang_spec_new(
        pool,
        M_SPEC_NEW,
        name as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if spec.is_null() {
        let mut __s: *mut mln_string_t = name;
        if !__s.is_null() {
            let ref mut fresh192 = (*__s).ref_0();
            let fresh193 = *fresh192;
            *fresh192 = (*fresh192).wrapping_sub(1);
            if fresh193 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = spec as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specparenth(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_PARENTH,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh194 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh194 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_specfactor(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_spec_t = 0 as *mut mln_lang_spec_t;
    ls = mln_lang_spec_new(
        pool,
        M_SPEC_FACTOR,
        (**right.offset(0 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if ls.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = ls as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_spec_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh195 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh195 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factortrue(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    let mut t: mln_u8_t = 1 as libc::c_int as mln_u8_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_BOOL,
        &mut t as *mut mln_u8_t as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorfalse(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    let mut t: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_BOOL,
        &mut t as *mut mln_u8_t as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factornil(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_NIL,
        0 as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorid(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut id: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if id.is_null() {
        return -(1 as libc::c_int);
    }
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_ID,
        id as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        let mut __s: *mut mln_string_t = id;
        if !__s.is_null() {
            let ref mut fresh196 = (*__s).ref_0();
            let fresh197 = *fresh196;
            *fresh196 = (*fresh196).wrapping_sub(1);
            if fresh197 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorint(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut num: [libc::c_char; 2048] = [
        0 as libc::c_int as libc::c_char,
    ];
    if (*(*ls).text).len
        > (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"mln_lang_semantic_factorint\0"))
                .as_ptr(),
            3548 as libc::c_int,
            b"Integer too long. %S\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ls).text,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        num.as_mut_ptr() as *mut libc::c_void,
        (*(*ls).text).data as *const libc::c_void,
        (*(*ls).text).len,
    );
    num[(*(*ls).text).len as usize] = 0 as libc::c_int as libc::c_char;
    let mut i: mln_s64_t = 0;
    if (*(*ls).text).len > 1 as libc::c_int as libc::c_ulong
        && num[0 as libc::c_int as usize] as libc::c_int == '0' as i32
    {
        if num[1 as libc::c_int as usize] as libc::c_int == 'x' as i32
            || num[1 as libc::c_int as usize] as libc::c_int == 'X' as i32
        {
            sscanf(
                num.as_mut_ptr(),
                b"%lx\0" as *const u8 as *const libc::c_char,
                &mut i as *mut mln_s64_t,
            );
        } else {
            sscanf(
                num.as_mut_ptr(),
                b"%lo\0" as *const u8 as *const libc::c_char,
                &mut i as *mut mln_s64_t,
            );
        }
    } else {
        sscanf(
            num.as_mut_ptr(),
            b"%ld\0" as *const u8 as *const libc::c_char,
            &mut i as *mut mln_s64_t,
        );
    }
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_INT,
        &mut i as *mut mln_s64_t as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorreal(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut num: [libc::c_char; 2048] = [
        0 as libc::c_int as libc::c_char,
    ];
    if (*(*ls).text).len
        > (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        _mln_sys_log(
            error,
            b"src/mln_lang_ast.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"mln_lang_semantic_factorreal\0"))
                .as_ptr(),
            3592 as libc::c_int,
            b"Real number too long. %S\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ls).text,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        num.as_mut_ptr() as *mut libc::c_void,
        (*(*ls).text).data as *const libc::c_void,
        (*(*ls).text).len,
    );
    num[(*(*ls).text).len as usize] = 0 as libc::c_int as libc::c_char;
    let mut f: libc::c_double = atof(num.as_mut_ptr());
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_REAL,
        &mut f as *mut libc::c_double as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorstring(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut ls: *mut mln_lang_struct_t = (**right.offset(0 as libc::c_int as isize)).data
        as *mut mln_lang_struct_t;
    let mut s: *mut mln_string_t = mln_string_pool_dup(pool, (*ls).text);
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_STRING,
        s as *mut libc::c_void,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        let mut __s: *mut mln_string_t = s;
        if !__s.is_null() {
            let ref mut fresh198 = (*__s).ref_0();
            let fresh199 = *fresh198;
            *fresh198 = (*fresh198).wrapping_sub(1);
            if fresh199 <= 1 as libc::c_int as libc::c_ulong {
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
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_factorarray(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut lf: *mut mln_lang_factor_t = 0 as *mut mln_lang_factor_t;
    lf = mln_lang_factor_new(
        pool,
        M_FACTOR_ARRAY,
        (**right.offset(1 as libc::c_int as isize)).data,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if lf.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = lf as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_factor_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh200 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh200 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_elemlist(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut pool: *mut mln_alloc_t = data as *mut mln_alloc_t;
    let mut le: *mut mln_lang_elemlist_t = 0 as *mut mln_lang_elemlist_t;
    let mut val: *mut mln_lang_assign_t = (**right.offset(1 as libc::c_int as isize))
        .data as *mut mln_lang_assign_t;
    let mut key: *mut mln_lang_assign_t = 0 as *mut mln_lang_assign_t;
    if val.is_null() {
        val = (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_assign_t;
    } else {
        key = (**right.offset(0 as libc::c_int as isize)).data as *mut mln_lang_assign_t;
    }
    le = mln_lang_elemlist_new(
        pool,
        key,
        val,
        (**right.offset(2 as libc::c_int as isize)).data as *mut mln_lang_elemlist_t,
        (*left).line as mln_u64_t,
        (*left).file,
    );
    if le.is_null() {
        return -(1 as libc::c_int);
    }
    (*left).data = le as *mut libc::c_void;
    (*left)
        .nonterm_free_handler = Some(
        mln_lang_elemlist_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let ref mut fresh201 = (**right.offset(0 as libc::c_int as isize)).data;
    *fresh201 = 0 as *mut libc::c_void;
    let ref mut fresh202 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh202 = 0 as *mut libc::c_void;
    let ref mut fresh203 = (**right.offset(2 as libc::c_int as isize)).data;
    *fresh203 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_elemval(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(1 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(1 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh204 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh204 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_lang_semantic_elemnext(
    mut left: *mut mln_factor_t,
    mut right: *mut *mut mln_factor_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    (*left).data = (**right.offset(1 as libc::c_int as isize)).data;
    (*left)
        .nonterm_free_handler = (**right.offset(1 as libc::c_int as isize))
        .nonterm_free_handler;
    let ref mut fresh205 = (**right.offset(1 as libc::c_int as isize)).data;
    *fresh205 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_lang_ast_file_open(
    mut file_path: *mut mln_string_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: size_t = if (*file_path).len >= 1024 as libc::c_int as libc::c_ulong {
        1023 as libc::c_int as libc::c_ulong
    } else {
        (*file_path).len
    };
    let mut path: [libc::c_char; 1024] = [0; 1024];
    let mut melang_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_path: [libc::c_char; 1024] = [0; 1024];
    memcpy(
        path.as_mut_ptr() as *mut libc::c_void,
        (*file_path).data as *const libc::c_void,
        len,
    );
    path[len as usize] = 0 as libc::c_int as libc::c_char;
    if path[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
        fd = open(path.as_mut_ptr(), 0 as libc::c_int);
    } else if access(path.as_mut_ptr(), 0 as libc::c_int) == 0 {
        fd = open(path.as_mut_ptr(), 0 as libc::c_int);
    } else {
        let mut current_block_20: u64;
        melang_path = getenv(mln_lang_env.data as *mut libc::c_char);
        if !melang_path.is_null() {
            let mut end: *mut libc::c_char = strchr(melang_path, ';' as i32);
            let mut found: libc::c_int = 0 as libc::c_int;
            while !end.is_null() {
                *end = 0 as libc::c_int as libc::c_char;
                n = snprintf(
                    tmp_path.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    melang_path,
                    path.as_mut_ptr(),
                );
                tmp_path[n as usize] = 0 as libc::c_int as libc::c_char;
                if access(tmp_path.as_mut_ptr(), 0 as libc::c_int) == 0 {
                    fd = open(tmp_path.as_mut_ptr(), 0 as libc::c_int);
                    found = 1 as libc::c_int;
                    break;
                } else {
                    melang_path = end.offset(1 as libc::c_int as isize);
                    end = strchr(melang_path, ';' as i32);
                }
            }
            if found == 0 {
                if *melang_path != 0 {
                    n = snprintf(
                        tmp_path.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        melang_path,
                        path.as_mut_ptr(),
                    );
                    tmp_path[n as usize] = 0 as libc::c_int as libc::c_char;
                    fd = open(tmp_path.as_mut_ptr(), 0 as libc::c_int);
                    current_block_20 = 13472856163611868459;
                } else {
                    current_block_20 = 10125163415517497719;
                }
            } else {
                current_block_20 = 13472856163611868459;
            }
        } else {
            current_block_20 = 10125163415517497719;
        }
        match current_block_20 {
            10125163415517497719 => {
                n = snprintf(
                    tmp_path.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    mln_path_melang_lib(),
                    path.as_mut_ptr(),
                );
                tmp_path[n as usize] = 0 as libc::c_int as libc::c_char;
                fd = open(tmp_path.as_mut_ptr(), 0 as libc::c_int);
            }
            _ => {}
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn mln_lang_ast_parser_generate() -> *mut libc::c_void {
    return mln_lang_parser_generate(
        prod_tbl.as_mut_ptr(),
        (::core::mem::size_of::<[mln_production_t; 112]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<mln_production_t>() as libc::c_ulong)
            as mln_u32_t,
        &mut mln_lang_env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_lang_ast_parser_destroy(mut data: *mut libc::c_void) {
    if !data.is_null() {
        mln_lang_pg_data_free(data);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_lang_ast_generate(
    mut pool: *mut mln_alloc_t,
    mut state_tbl: *mut libc::c_void,
    mut data: *mut mln_string_t,
    mut data_type: mln_u32_t,
) -> *mut libc::c_void {
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
    let mut lex: *mut mln_lex_t = 0 as *mut mln_lex_t;
    let mut pattr: mln_parse_attr = mln_parse_attr {
        pool: 0 as *mut mln_alloc_t,
        prod_tbl: 0 as *mut mln_production_t,
        lex: 0 as *mut mln_lex_t,
        pg_data: 0 as *mut libc::c_void,
        udata: 0 as *mut libc::c_void,
    };
    let mut internal_pool: *mut mln_alloc_t = 0 as *mut mln_alloc_t;
    let mut ret: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    internal_pool = mln_alloc_init(0 as *mut mln_alloc_t);
    if internal_pool.is_null() {
        return 0 as *mut libc::c_void;
    }
    lattr.pool = internal_pool;
    lattr.keywords = keywords.as_mut_ptr();
    memset(
        &mut hooks as *mut mln_lex_hooks_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_lex_hooks_t>() as libc::c_ulong,
    );
    hooks
        .sub_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_sub_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .plus_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_plus_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .ast_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_ast_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .lagl_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_lagl_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .ragl_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_ragl_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .vertl_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_vertl_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .amp_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_amp_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .dash_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_dash_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .xor_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_xor_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .perc_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_perc_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .equal_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_equal_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .excl_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_excl_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .sglq_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_sglq_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .dblq_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_dblq_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    hooks
        .slash_handler = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut mln_lex_t,
                *mut libc::c_void,
            ) -> *mut mln_lang_struct_t,
        >,
        lex_hook,
    >(
        Some(
            mln_lang_slash_handler
                as unsafe extern "C" fn(
                    *mut mln_lex_t,
                    *mut libc::c_void,
                ) -> *mut mln_lang_struct_t,
        ),
    );
    lattr.hooks = &mut hooks;
    lattr.set_preprocess(1 as libc::c_int as mln_u32_t);
    lattr.type_0 = data_type;
    lattr.data = data;
    lattr.env = &mut mln_lang_env;
    if lattr.preprocess() != 0 {
        let mut lpd: *mut mln_lex_preprocess_data_t = mln_lex_preprocess_data_new(
            lattr.pool,
        );
        if lpd.is_null() {
            lex = 0 as *mut mln_lex_t;
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_nums_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_nums_handler
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
                    ),
                );
                (*lattr.hooks).nums_data = lpd as *mut libc::c_void;
            }
            lex = mln_lex_init(&mut lattr);
            if !lex.is_null() {
                if !(lattr.hooks).is_null() {
                    mln_lang_set_hooks(lex);
                }
                (*lex).preprocess_data = lpd;
            } else {
                mln_lex_preprocess_data_free(lpd);
            }
        }
    } else {
        lex = mln_lex_init(&mut lattr);
        if !lex.is_null() && !(lattr.hooks).is_null() {
            mln_lang_set_hooks(lex);
        }
    }
    if lex.is_null() {
        mln_alloc_destroy(internal_pool);
        return 0 as *mut libc::c_void;
    }
    pattr.pool = internal_pool;
    pattr.prod_tbl = prod_tbl.as_mut_ptr();
    pattr.lex = lex;
    pattr.pg_data = state_tbl;
    pattr.udata = pool as *mut libc::c_void;
    ret = mln_lang_parse(&mut pattr) as mln_u8ptr_t;
    if ret.is_null() {
        mln_lex_destroy(lex);
        mln_alloc_destroy(internal_pool);
        return 0 as *mut libc::c_void;
    }
    mln_lex_destroy(lex);
    mln_alloc_destroy(internal_pool);
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn mln_lang_ast_free(mut ast: *mut libc::c_void) {
    if ast.is_null() {
        return;
    }
    mln_lang_stm_free(ast);
}
unsafe extern "C" fn run_static_initializers() {
    mln_lang_preprocess_handlers = [
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_define
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_include
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_if
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_else
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_endif
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
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
                        ) -> *mut mln_lang_struct_t,
                    >,
                    lex_hook,
                >(
                    Some(
                        mln_lang_lex_preprocess_undef
                            as unsafe extern "C" fn(
                                *mut mln_lex_t,
                                *mut libc::c_void,
                            ) -> *mut mln_lang_struct_t,
                    ),
                ),
            };
            init
        },
    ];
    keywords = [
        {
            let mut init = mln_string_t {
                data_ref_pool_ref_0: [0; 4],
                c2rust_padding: [0; 4],
                data: b"if\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"else\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"while\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"for\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"continue\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"break\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"return\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"goto\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"nil\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"true\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"false\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"switch\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"case\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"default\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
                data: b"fi\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
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
    mln_lang_env = {
        let mut init = mln_string_t {
            data_ref_pool_ref_0: [0; 4],
            c2rust_padding: [0; 4],
            data: b"MELANG_PATH\0" as *const u8 as *const libc::c_char as mln_u8ptr_t,
            len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        };
        init.set_data_ref(1 as libc::c_int as mln_uauto_t);
        init.set_pool(0 as libc::c_int as mln_uauto_t);
        init.set_ref_0(1 as libc::c_int as mln_uauto_t);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
