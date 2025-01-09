use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_rbtree_insert(t: *mut mln_rbtree_t, node: *mut mln_rbtree_node_t);
    fn mln_rbtree_search(
        t: *mut mln_rbtree_t,
        key: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_node_new(
        t: *mut mln_rbtree_t,
        data: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_iterate(
        t: *mut mln_rbtree_t,
        handler: rbtree_iterate_handler,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
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
pub type off_t = __off_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
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
pub type mln_log_level_t = libc::c_uint;
pub const error: mln_log_level_t = 4;
pub const warn: mln_log_level_t = 3;
pub const debug: mln_log_level_t = 2;
pub const report: mln_log_level_t = 1;
pub const none: mln_log_level_t = 0;
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
pub type nonterm_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type factor_data_type = libc::c_uint;
pub const M_P_NONTERM: factor_data_type = 1;
pub const M_P_TERM: factor_data_type = 0;
pub type mln_pg_rule_t = mln_pg_rule_s;
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
pub type mln_pg_item_t = mln_pg_item_s;
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
pub type mln_pg_state_t = mln_pg_state_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_info {
    pub change: *mut libc::c_int,
    pub map: *mut libc::c_int,
    pub r: *mut mln_pg_rule_t,
    pub tk: *mut mln_pg_token_t,
    pub s: *mut mln_pg_state_t,
    pub item: *mut mln_pg_item_t,
    pub nr_rule: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_pg_state {
    pub change: *mut libc::c_int,
    pub r: *mut mln_pg_rule_t,
    pub s: *mut mln_pg_state_t,
    pub item: *mut mln_pg_item_t,
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
unsafe extern "C" fn mln_item_chain_add(
    mut head: *mut *mut mln_pg_item_t,
    mut tail: *mut *mut mln_pg_item_t,
    mut node: *mut mln_pg_item_t,
) {
    (*node).next = 0 as *mut mln_pg_item_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
unsafe extern "C" fn mln_item_chain_del(
    mut head: *mut *mut mln_pg_item_t,
    mut tail: *mut *mut mln_pg_item_t,
    mut node: *mut mln_pg_item_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_pg_item_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_pg_item_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_pg_item_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_pg_item_s;
    (*node).prev = (*node).next;
}
unsafe extern "C" fn mln_state_chain_add(
    mut head: *mut *mut mln_pg_state_t,
    mut tail: *mut *mut mln_pg_state_t,
    mut node: *mut mln_pg_state_t,
) {
    (*node).next = 0 as *mut mln_pg_state_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
unsafe extern "C" fn mln_state_chain_del(
    mut head: *mut *mut mln_pg_state_t,
    mut tail: *mut *mut mln_pg_state_t,
    mut node: *mut mln_pg_state_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_pg_state_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_pg_state_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_pg_state_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_pg_state_s;
    (*node).prev = (*node).next;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_token_new(
    mut token: *mut mln_string_t,
    mut nr_rule: mln_u32_t,
) -> *mut mln_pg_token_t {
    let mut t: *mut mln_pg_token_t = malloc(
        ::core::mem::size_of::<mln_pg_token_t>() as libc::c_ulong,
    ) as *mut mln_pg_token_t;
    if t.is_null() {
        return 0 as *mut mln_pg_token_t;
    }
    (*t).token = 0 as *mut mln_string_t;
    (*t).first_set = 0 as *mut mln_rbtree_t;
    (*t).follow_set = 0 as *mut mln_rbtree_t;
    (*t).right_rule_index = 0 as *mut mln_u32_t;
    (*t).left_rule_index = 0 as *mut mln_u32_t;
    (*t).type_0 = -(1 as libc::c_int);
    (*t).set_is_nonterminal(0 as libc::c_int as mln_u32_t);
    (*t).set_is_nullable(0 as libc::c_int as mln_u32_t);
    (*t).token = mln_string_dup(token);
    if ((*t).token).is_null() {
        mln_pg_token_free(t as *mut libc::c_void);
        return 0 as *mut mln_pg_token_t;
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
        mln_pg_token_rbtree_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr.data_free = None;
    (*t).first_set = mln_rbtree_new(&mut rbattr);
    if ((*t).first_set).is_null() {
        mln_pg_token_free(t as *mut libc::c_void);
        return 0 as *mut mln_pg_token_t;
    }
    (*t).follow_set = mln_rbtree_new(&mut rbattr);
    if ((*t).follow_set).is_null() {
        mln_pg_token_free(t as *mut libc::c_void);
        return 0 as *mut mln_pg_token_t;
    }
    (*t)
        .right_rule_index = calloc(
        nr_rule as libc::c_ulong,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    ) as *mut mln_u32_t;
    if ((*t).right_rule_index).is_null() {
        mln_pg_token_free(t as *mut libc::c_void);
        return 0 as *mut mln_pg_token_t;
    }
    (*t)
        .left_rule_index = calloc(
        nr_rule as libc::c_ulong,
        ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong,
    ) as *mut mln_u32_t;
    if ((*t).left_rule_index).is_null() {
        mln_pg_token_free(t as *mut libc::c_void);
        return 0 as *mut mln_pg_token_t;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_token_free(mut token: *mut libc::c_void) {
    if token.is_null() {
        return;
    }
    let mut t: *mut mln_pg_token_t = token as *mut mln_pg_token_t;
    if !((*t).token).is_null() {
        let mut __s: *mut mln_string_t = (*t).token;
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
    if !((*t).first_set).is_null() {
        mln_rbtree_free((*t).first_set);
    }
    if !((*t).follow_set).is_null() {
        mln_rbtree_free((*t).follow_set);
    }
    if !((*t).right_rule_index).is_null() {
        free((*t).right_rule_index as *mut libc::c_void);
    }
    if !((*t).left_rule_index).is_null() {
        free((*t).left_rule_index as *mut libc::c_void);
    }
    free(t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_token_rbtree_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    return (*(data1 as *mut mln_pg_token_t)).type_0
        - (*(data2 as *mut mln_pg_token_t)).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_map_hash_calc(
    mut h: *mut mln_hash_t,
    mut key: *mut libc::c_void,
) -> mln_u64_t {
    let mut sum: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut p: *mut libc::c_char = key as *mut libc::c_char;
    while *p as libc::c_int != 0 as libc::c_int {
        sum = (sum as libc::c_ulong)
            .wrapping_add((*p as libc::c_int * 65599 as libc::c_int) as libc::c_ulong)
            as mln_u64_t as mln_u64_t;
        sum = (sum as libc::c_ulong).wrapping_rem((*h).len) as mln_u64_t as mln_u64_t;
        p = p.offset(1);
        p;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_map_hash_cmp(
    mut h: *mut mln_hash_t,
    mut key1: *mut libc::c_void,
    mut key2: *mut libc::c_void,
) -> libc::c_int {
    return (strcmp(key1 as *mut libc::c_char, key2 as *mut libc::c_char) == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_map_hash_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    free(data);
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_item_new() -> *mut mln_pg_item_t {
    let mut item: *mut mln_pg_item_t = malloc(
        ::core::mem::size_of::<mln_pg_item_t>() as libc::c_ulong,
    ) as *mut mln_pg_item_t;
    if item.is_null() {
        return 0 as *mut mln_pg_item_t;
    }
    (*item).prev = 0 as *mut mln_pg_item_s;
    (*item).next = 0 as *mut mln_pg_item_s;
    (*item).lookahead_set = 0 as *mut mln_rbtree_t;
    (*item).read = 0 as *mut mln_pg_token_t;
    (*item).goto_id = -(1 as libc::c_int) as mln_sauto_t;
    (*item).rule = 0 as *mut mln_pg_rule_t;
    (*item).pos = 0 as libc::c_int as mln_u32_t;
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
        mln_pg_token_rbtree_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr.data_free = None;
    (*item).lookahead_set = mln_rbtree_new(&mut rbattr);
    if ((*item).lookahead_set).is_null() {
        mln_pg_item_free(item);
        return 0 as *mut mln_pg_item_t;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_item_free(mut item: *mut mln_pg_item_t) {
    if item.is_null() {
        return;
    }
    if !((*item).lookahead_set).is_null() {
        mln_rbtree_free((*item).lookahead_set);
    }
    free(item as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_state_new() -> *mut mln_pg_state_t {
    let mut s: *mut mln_pg_state_t = malloc(
        ::core::mem::size_of::<mln_pg_state_t>() as libc::c_ulong,
    ) as *mut mln_pg_state_t;
    if s.is_null() {
        return 0 as *mut mln_pg_state_t;
    }
    (*s).id = -(1 as libc::c_int) as mln_sauto_t;
    (*s).input = 0 as *mut mln_pg_token_t;
    (*s).head = 0 as *mut mln_pg_item_t;
    (*s).tail = 0 as *mut mln_pg_item_t;
    (*s).prev = 0 as *mut mln_pg_state_s;
    (*s).next = 0 as *mut mln_pg_state_s;
    (*s).q_next = 0 as *mut mln_pg_state_s;
    (*s).nr_item = 0 as libc::c_int as mln_u64_t;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_state_free(mut s: *mut mln_pg_state_t) {
    let mut it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    loop {
        it = (*s).head;
        if it.is_null() {
            break;
        }
        mln_item_chain_del(&mut (*s).head, &mut (*s).tail, it);
        mln_pg_item_free(it);
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_calc_info_init(
    mut pci: *mut mln_pg_calc_info_s,
    mut first_input: *mut mln_pg_token_t,
    mut rule: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) -> libc::c_int {
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
        mln_pg_calc_info_cmp
            as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
    );
    rbattr.data_free = None;
    (*pci).tree = mln_rbtree_new(&mut rbattr);
    if ((*pci).tree).is_null() {
        return -(1 as libc::c_int);
    }
    (*pci).head = 0 as *mut mln_pg_state_t;
    (*pci).tail = 0 as *mut mln_pg_state_t;
    (*pci).id_counter = 0 as libc::c_int as mln_sauto_t;
    (*pci).first_input = first_input;
    (*pci).rule = rule;
    (*pci).nr_rule = nr_rule;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_calc_info_destroy(mut pci: *mut mln_pg_calc_info_s) {
    if pci.is_null() {
        return;
    }
    if !((*pci).tree).is_null() {
        mln_rbtree_free((*pci).tree);
    }
    let mut s: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    loop {
        s = (*pci).head;
        if s.is_null() {
            break;
        }
        mln_state_chain_del(&mut (*pci).head, &mut (*pci).tail, s);
        mln_pg_state_free(s);
    };
}
unsafe extern "C" fn mln_pg_calc_info_cmp(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *mut mln_pg_state_t = data1 as *mut mln_pg_state_t;
    let mut s2: *mut mln_pg_state_t = data2 as *mut mln_pg_state_t;
    if (*s1).input > (*s2).input {
        return 1 as libc::c_int;
    }
    if (*s1).input < (*s2).input {
        return -(1 as libc::c_int);
    }
    if (*s1).nr_item > (*s2).nr_item {
        return 1 as libc::c_int;
    }
    if (*s1).nr_item < (*s2).nr_item {
        return -(1 as libc::c_int);
    }
    let mut nr_match: mln_sauto_t = 0 as libc::c_int as mln_sauto_t;
    let mut it1: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut it2: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    it1 = (*s1).head;
    while !it1.is_null() {
        it2 = (*s2).head;
        while !it2.is_null() {
            if (*it1).rule == (*it2).rule && (*it1).pos == (*it2).pos {
                nr_match += 1;
                nr_match;
                break;
            } else {
                it2 = (*it2).next;
            }
        }
        it1 = (*it1).next;
    }
    if (*s1).nr_item > nr_match as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if nr_match as libc::c_ulong == (*s1).nr_item {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_calc_nullable(
    mut map: *mut libc::c_int,
    mut r: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut change: libc::c_int = 1 as libc::c_int;
    let mut nr_null: libc::c_int = 0;
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut pr: *mut mln_pg_rule_t = 0 as *mut mln_pg_rule_t;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < nr_rule {
        ::core::ptr::write_volatile(map.offset(i as isize), 1 as libc::c_int);
        i += 1;
        i;
    }
    while change != 0 {
        change = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < nr_rule {
            if !(*map.offset(i as isize) == 0 as libc::c_int) {
                ::core::ptr::write_volatile(map.offset(i as isize), 0 as libc::c_int);
                pr = &mut *r.offset(i as isize) as *mut mln_pg_rule_t;
                if !((*(*pr).left).is_nullable() != 0) {
                    nr_null = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while (j as libc::c_uint) < (*pr).nr_right {
                        tk = *((*pr).rights).offset(j as isize);
                        if (*tk).is_nullable() != 0 {
                            nr_null += 1;
                            nr_null;
                        }
                        if (*tk).is_nonterminal() == 0 {
                            rn = mln_rbtree_search(
                                (*tk).first_set,
                                tk as *mut libc::c_void,
                            );
                            if rn
                                == &mut (*(*tk).first_set).nil as *mut mln_rbtree_node_t
                            {
                                rn = mln_rbtree_node_new(
                                    (*tk).first_set,
                                    tk as *mut libc::c_void,
                                );
                                if rn.is_null() {
                                    _mln_sys_log(
                                        error,
                                        b"src/mln_parser_generator.c\0" as *const u8
                                            as *const libc::c_char,
                                        (*::core::mem::transmute::<
                                            &[u8; 21],
                                            &[libc::c_char; 21],
                                        >(b"mln_pg_calc_nullable\0"))
                                            .as_ptr(),
                                        324 as libc::c_int,
                                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    return -(1 as libc::c_int);
                                }
                                mln_rbtree_insert((*tk).first_set, rn);
                            }
                        }
                        j += 1;
                        j;
                    }
                    if nr_null as libc::c_uint == (*pr).nr_right {
                        k = 0 as libc::c_int;
                        while (k as libc::c_uint) < nr_rule {
                            if *((*(*pr).left).right_rule_index).offset(k as isize) != 0
                            {
                                ::core::ptr::write_volatile(
                                    map.offset(k as isize),
                                    1 as libc::c_int,
                                );
                            }
                            k += 1;
                            k;
                        }
                        change = 1 as libc::c_int;
                        (*(*pr).left).set_is_nullable(1 as libc::c_int as mln_u32_t);
                    }
                }
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_calc_first(
    mut map: *mut libc::c_int,
    mut r: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut change: libc::c_int = 1 as libc::c_int;
    let mut pr: *mut mln_pg_rule_t = 0 as *mut mln_pg_rule_t;
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut info: mln_pg_info = mln_pg_info {
        change: 0 as *mut libc::c_int,
        map: 0 as *mut libc::c_int,
        r: 0 as *mut mln_pg_rule_t,
        tk: 0 as *mut mln_pg_token_t,
        s: 0 as *mut mln_pg_state_t,
        item: 0 as *mut mln_pg_item_t,
        nr_rule: 0,
    };
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < nr_rule {
        ::core::ptr::write_volatile(map.offset(i as isize), 1 as libc::c_int);
        i += 1;
        i;
    }
    while change != 0 {
        change = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < nr_rule {
            if !(*map.offset(i as isize) == 0 as libc::c_int) {
                ::core::ptr::write_volatile(map.offset(i as isize), 0 as libc::c_int);
                pr = &mut *r.offset(i as isize) as *mut mln_pg_rule_t;
                j = 0 as libc::c_int;
                while (j as libc::c_uint) < (*pr).nr_right {
                    tk = *((*pr).rights).offset(j as isize);
                    info.change = &mut change;
                    info.map = map;
                    info.r = pr;
                    info.tk = (*pr).left;
                    info.nr_rule = nr_rule;
                    if mln_rbtree_iterate(
                        (*tk).first_set,
                        Some(
                            mln_pg_calc_first_rbtree_iterate_handler
                                as unsafe extern "C" fn(
                                    *mut mln_rbtree_node_t,
                                    *mut libc::c_void,
                                ) -> libc::c_int,
                        ),
                        &mut info as *mut mln_pg_info as *mut libc::c_void,
                    ) < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    if (*tk).is_nullable() as libc::c_int == 0 as libc::c_int {
                        break;
                    }
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_calc_first_rbtree_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut pgi: *mut mln_pg_info = udata as *mut mln_pg_info;
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut dest: *mut mln_rbtree_t = (*(*pgi).tk).first_set;
    let mut rn: *mut mln_rbtree_node_t = mln_rbtree_search(
        dest,
        tk as *mut libc::c_void,
    );
    if !(rn == &mut (*dest).nil as *mut mln_rbtree_node_t) {
        return 0 as libc::c_int;
    }
    rn = mln_rbtree_node_new(dest, tk as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"mln_pg_calc_first_rbtree_iterate_handler\0"))
                .as_ptr(),
            394 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(dest, rn);
    *(*pgi).change = 1 as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while (k as libc::c_uint) < (*pgi).nr_rule {
        if *((*(*pgi).tk).right_rule_index).offset(k as isize) != 0 {
            ::core::ptr::write_volatile(
                ((*pgi).map).offset(k as isize),
                1 as libc::c_int,
            );
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_calc_follow(
    mut map: *mut libc::c_int,
    mut r: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut change: libc::c_int = 1 as libc::c_int;
    let mut nr_null: libc::c_int = 0;
    let mut pr: *mut mln_pg_rule_t = 0 as *mut mln_pg_rule_t;
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut behind: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut info: mln_pg_info = mln_pg_info {
        change: 0 as *mut libc::c_int,
        map: 0 as *mut libc::c_int,
        r: 0 as *mut mln_pg_rule_t,
        tk: 0 as *mut mln_pg_token_t,
        s: 0 as *mut mln_pg_state_t,
        item: 0 as *mut mln_pg_item_t,
        nr_rule: 0,
    };
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < nr_rule {
        ::core::ptr::write_volatile(map.offset(i as isize), 1 as libc::c_int);
        i += 1;
        i;
    }
    while change != 0 {
        change = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < nr_rule {
            if !(*map.offset(i as isize) == 0 as libc::c_int) {
                ::core::ptr::write_volatile(map.offset(i as isize), 0 as libc::c_int);
                pr = &mut *r.offset(i as isize) as *mut mln_pg_rule_t;
                nr_null = 1 as libc::c_int;
                j = ((*pr).nr_right).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                while j >= 0 as libc::c_int {
                    tk = *((*pr).rights).offset(j as isize);
                    if nr_null != 0
                        && nr_null as libc::c_uint
                            == ((*pr).nr_right).wrapping_sub(j as libc::c_uint)
                    {
                        info.change = &mut change;
                        info.map = map;
                        info.r = pr;
                        info.tk = tk;
                        info.nr_rule = nr_rule;
                        if mln_rbtree_iterate(
                            (*(*pr).left).follow_set,
                            Some(
                                mln_pg_calc_follow_rbtree_iterate_handler
                                    as unsafe extern "C" fn(
                                        *mut mln_rbtree_node_t,
                                        *mut libc::c_void,
                                    ) -> libc::c_int,
                            ),
                            &mut info as *mut mln_pg_info as *mut libc::c_void,
                        ) < 0 as libc::c_int
                        {
                            return -(1 as libc::c_int);
                        }
                        k = 1 as libc::c_int;
                        while k <= nr_null - 1 as libc::c_int {
                            behind = *((*pr).rights).offset((j + k) as isize);
                            info.change = &mut change;
                            info.map = map;
                            info.r = pr;
                            info.tk = tk;
                            info.nr_rule = nr_rule;
                            if mln_rbtree_iterate(
                                (*behind).first_set,
                                Some(
                                    mln_pg_calc_follow_rbtree_iterate_handler
                                        as unsafe extern "C" fn(
                                            *mut mln_rbtree_node_t,
                                            *mut libc::c_void,
                                        ) -> libc::c_int,
                                ),
                                &mut info as *mut mln_pg_info as *mut libc::c_void,
                            ) < 0 as libc::c_int
                            {
                                return -(1 as libc::c_int);
                            }
                            k += 1;
                            k;
                        }
                    } else {
                        k = 1 as libc::c_int;
                        while k <= nr_null + 1 as libc::c_int {
                            behind = *((*pr).rights).offset((j + k) as isize);
                            info.change = &mut change;
                            info.map = map;
                            info.r = pr;
                            info.tk = tk;
                            info.nr_rule = nr_rule;
                            if mln_rbtree_iterate(
                                (*behind).first_set,
                                Some(
                                    mln_pg_calc_follow_rbtree_iterate_handler
                                        as unsafe extern "C" fn(
                                            *mut mln_rbtree_node_t,
                                            *mut libc::c_void,
                                        ) -> libc::c_int,
                                ),
                                &mut info as *mut mln_pg_info as *mut libc::c_void,
                            ) < 0 as libc::c_int
                            {
                                return -(1 as libc::c_int);
                            }
                            k += 1;
                            k;
                        }
                    }
                    nr_null = if (*tk).is_nullable() as libc::c_int != 0 {
                        nr_null + 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    j -= 1;
                    j;
                }
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_calc_follow_rbtree_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut pgi: *mut mln_pg_info = udata as *mut mln_pg_info;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut dest: *mut mln_rbtree_t = (*(*pgi).tk).follow_set;
    rn = mln_rbtree_search(dest, tk as *mut libc::c_void);
    if !(rn == &mut (*dest).nil as *mut mln_rbtree_node_t) {
        return 0 as libc::c_int;
    }
    rn = mln_rbtree_node_new(dest, tk as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"mln_pg_calc_follow_rbtree_iterate_handler\0"))
                .as_ptr(),
            490 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(dest, rn);
    *(*pgi).change = 1 as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while (k as libc::c_uint) < (*pgi).nr_rule {
        if *((*(*pgi).tk).right_rule_index).offset(k as isize) != 0 {
            ::core::ptr::write_volatile(
                ((*pgi).map).offset(k as isize),
                1 as libc::c_int,
            );
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_closure(
    mut s: *mut mln_pg_state_t,
    mut r: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut change: libc::c_int = 1 as libc::c_int;
    let mut item: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut new_it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut next_tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut pr: *mut mln_pg_rule_t = 0 as *mut mln_pg_rule_t;
    let mut info: mln_pg_state = mln_pg_state {
        change: 0 as *mut libc::c_int,
        r: 0 as *mut mln_pg_rule_t,
        s: 0 as *mut mln_pg_state_t,
        item: 0 as *mut mln_pg_item_t,
    };
    while change != 0 {
        change = 0 as libc::c_int;
        item = (*s).head;
        while !item.is_null() {
            if !((*(*item).rule).nr_right == (*item).pos) {
                tk = *((*(*item).rule).rights).offset((*item).pos as isize);
                if !((*tk).is_nonterminal() == 0) {
                    let mut current_block_31: u64;
                    i = 0 as libc::c_int;
                    while (i as libc::c_uint) < nr_rule {
                        if !(*((*tk).left_rule_index).offset(i as isize)
                            == 0 as libc::c_int as libc::c_uint)
                        {
                            pr = &mut *r.offset(i as isize) as *mut mln_pg_rule_t;
                            new_it = mln_pg_rule_duplicate(
                                s,
                                pr,
                                0 as libc::c_int as mln_u32_t,
                            );
                            if new_it.is_null() {
                                new_it = mln_pg_item_new();
                                if new_it.is_null() {
                                    _mln_sys_log(
                                        error,
                                        b"src/mln_parser_generator.c\0" as *const u8
                                            as *const libc::c_char,
                                        (*::core::mem::transmute::<
                                            &[u8; 15],
                                            &[libc::c_char; 15],
                                        >(b"mln_pg_closure\0"))
                                            .as_ptr(),
                                        529 as libc::c_int,
                                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    return -(1 as libc::c_int);
                                }
                                (*s).nr_item = ((*s).nr_item).wrapping_add(1);
                                (*s).nr_item;
                                mln_item_chain_add(&mut (*s).head, &mut (*s).tail, new_it);
                                (*new_it).rule = pr;
                                (*new_it).pos = 0 as libc::c_int as mln_u32_t;
                            }
                            if ((*item).pos)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                < (*(*item).rule).nr_right
                            {
                                next_tk = *((*(*item).rule).rights)
                                    .offset(
                                        ((*item).pos).wrapping_add(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    );
                                info.change = &mut change;
                                info.r = pr;
                                info.s = s;
                                info.item = new_it;
                                if mln_rbtree_iterate(
                                    (*next_tk).first_set,
                                    Some(
                                        mln_pg_closure_rbtree_iterate_handler
                                            as unsafe extern "C" fn(
                                                *mut mln_rbtree_node_t,
                                                *mut libc::c_void,
                                            ) -> libc::c_int,
                                    ),
                                    &mut info as *mut mln_pg_state as *mut libc::c_void,
                                ) < 0 as libc::c_int
                                {
                                    return -(1 as libc::c_int);
                                }
                                if (*next_tk).is_nullable() == 0 {
                                    current_block_31 = 17216689946888361452;
                                } else {
                                    current_block_31 = 4068382217303356765;
                                }
                            } else {
                                current_block_31 = 4068382217303356765;
                            }
                            match current_block_31 {
                                17216689946888361452 => {}
                                _ => {
                                    info.change = &mut change;
                                    info.r = pr;
                                    info.s = s;
                                    info.item = new_it;
                                    if mln_rbtree_iterate(
                                        (*item).lookahead_set,
                                        Some(
                                            mln_pg_closure_rbtree_iterate_handler
                                                as unsafe extern "C" fn(
                                                    *mut mln_rbtree_node_t,
                                                    *mut libc::c_void,
                                                ) -> libc::c_int,
                                        ),
                                        &mut info as *mut mln_pg_state as *mut libc::c_void,
                                    ) < 0 as libc::c_int
                                    {
                                        return -(1 as libc::c_int);
                                    }
                                }
                            }
                        }
                        i += 1;
                        i;
                    }
                }
            }
            item = (*item).next;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_rule_duplicate(
    mut s: *mut mln_pg_state_t,
    mut r: *mut mln_pg_rule_t,
    mut pos: mln_u32_t,
) -> *mut mln_pg_item_t {
    let mut item: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    item = (*s).head;
    while !item.is_null() {
        if (*item).rule == r && (*item).pos == pos {
            return item;
        }
        item = (*item).next;
    }
    return 0 as *mut mln_pg_item_t;
}
unsafe extern "C" fn mln_pg_closure_rbtree_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut mln_pg_state = udata as *mut mln_pg_state;
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut new_it: *mut mln_pg_item_t = (*info).item;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    rn = mln_rbtree_search((*new_it).lookahead_set, tk as *mut libc::c_void);
    if !(rn == &mut (*(*new_it).lookahead_set).nil as *mut mln_rbtree_node_t) {
        return 0 as libc::c_int;
    }
    rn = mln_rbtree_node_new((*new_it).lookahead_set, tk as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"mln_pg_closure_rbtree_iterate_handler\0"))
                .as_ptr(),
            593 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert((*new_it).lookahead_set, rn);
    *(*info).change = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_pg_state_enqueue(
    mut head: *mut *mut mln_pg_state_t,
    mut tail: *mut *mut mln_pg_state_t,
    mut s: *mut mln_pg_state_t,
) {
    (*s).q_next = 0 as *mut mln_pg_state_s;
    if (*head).is_null() {
        *tail = s;
        *head = *tail;
        return;
    }
    (**tail).q_next = s;
    *tail = s;
}
#[inline]
unsafe extern "C" fn mln_pg_state_dequeue(
    mut head: *mut *mut mln_pg_state_t,
    mut tail: *mut *mut mln_pg_state_t,
) -> *mut mln_pg_state_t {
    let mut s: *mut mln_pg_state_t = *head;
    if s.is_null() {
        return 0 as *mut mln_pg_state_t;
    }
    *head = (*s).q_next;
    if (*head).is_null() {
        *tail = 0 as *mut mln_pg_state_t;
    }
    (*s).q_next = 0 as *mut mln_pg_state_s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_goto(mut pci: *mut mln_pg_calc_info_s) -> libc::c_int {
    let mut tree: *mut mln_rbtree_t = (*pci).tree;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    let mut nr_end: mln_u32_t = 0;
    let mut nr_cnt: mln_u32_t = 0;
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut new_s: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    let mut tmp_s: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    let mut save: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut new_it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut q_head: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    let mut q_tail: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    let mut s: *mut mln_pg_state_t = 0 as *mut mln_pg_state_t;
    s = mln_pg_state_new();
    if s.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"mln_pg_goto\0"))
                .as_ptr(),
            638 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let fresh3 = (*pci).id_counter;
    (*pci).id_counter = (*pci).id_counter + 1;
    (*s).id = fresh3;
    (*s).input = (*pci).first_input;
    new_it = mln_pg_item_new();
    if new_it.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"mln_pg_goto\0"))
                .as_ptr(),
            645 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_pg_state_free(s);
        return -(1 as libc::c_int);
    }
    (*new_it)
        .rule = &mut *((*pci).rule).offset(0 as libc::c_int as isize)
        as *mut mln_pg_rule_t;
    mln_item_chain_add(&mut (*s).head, &mut (*s).tail, new_it);
    (*s).nr_item = ((*s).nr_item).wrapping_add(1);
    (*s).nr_item;
    rn = mln_rbtree_node_new(tree, s as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"mln_pg_goto\0"))
                .as_ptr(),
            653 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        mln_pg_state_free(s);
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert(tree, rn);
    mln_state_chain_add(&mut (*pci).head, &mut (*pci).tail, s);
    mln_pg_state_enqueue(&mut q_head, &mut q_tail, s);
    if mln_pg_closure(s, (*pci).rule, (*pci).nr_rule) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    loop {
        s = mln_pg_state_dequeue(&mut q_head, &mut q_tail);
        if s.is_null() {
            break;
        }
        save = (*s).head;
        while !save.is_null() {
            if !((*(*save).rule).nr_right == (*save).pos) {
                nr_cnt = 0 as libc::c_int as mln_u32_t;
                nr_end = nr_cnt;
                tk = *((*(*save).rule).rights).offset((*save).pos as isize);
                it = (*save).prev;
                while !it.is_null() {
                    nr_cnt = nr_cnt.wrapping_add(1);
                    nr_cnt;
                    if (*(*it).rule).nr_right == (*it).pos {
                        nr_end = nr_end.wrapping_add(1);
                        nr_end;
                    } else if *((*(*it).rule).rights).offset((*it).pos as isize) == tk {
                        break;
                    }
                    it = (*it).prev;
                }
                if !(!it.is_null()
                    || nr_end == nr_cnt && nr_end != 0 as libc::c_int as libc::c_uint)
                {
                    new_s = mln_pg_state_new();
                    if new_s.is_null() {
                        _mln_sys_log(
                            error,
                            b"src/mln_parser_generator.c\0" as *const u8
                                as *const libc::c_char,
                            (*::core::mem::transmute::<
                                &[u8; 12],
                                &[libc::c_char; 12],
                            >(b"mln_pg_goto\0"))
                                .as_ptr(),
                            685 as libc::c_int,
                            b"No memory.\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                    (*new_s).input = tk;
                    it = save;
                    while !it.is_null() {
                        if !((*(*it).rule).nr_right == (*it).pos) {
                            if !(*((*(*it).rule).rights).offset((*it).pos as isize)
                                != tk)
                            {
                                new_it = mln_pg_item_new();
                                if new_it.is_null() {
                                    _mln_sys_log(
                                        error,
                                        b"src/mln_parser_generator.c\0" as *const u8
                                            as *const libc::c_char,
                                        (*::core::mem::transmute::<
                                            &[u8; 12],
                                            &[libc::c_char; 12],
                                        >(b"mln_pg_goto\0"))
                                            .as_ptr(),
                                        694 as libc::c_int,
                                        b"No memory.\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    mln_pg_state_free(new_s);
                                    return -(1 as libc::c_int);
                                }
                                (*new_s).nr_item = ((*new_s).nr_item).wrapping_add(1);
                                (*new_s).nr_item;
                                mln_item_chain_add(
                                    &mut (*new_s).head,
                                    &mut (*new_s).tail,
                                    new_it,
                                );
                                (*it).read = tk;
                                (*new_it).rule = (*it).rule;
                                (*new_it)
                                    .pos = ((*it).pos)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint);
                                if mln_rbtree_iterate(
                                    (*it).lookahead_set,
                                    Some(
                                        mln_pg_goto_iterate_handler
                                            as unsafe extern "C" fn(
                                                *mut mln_rbtree_node_t,
                                                *mut libc::c_void,
                                            ) -> libc::c_int,
                                    ),
                                    new_it as *mut libc::c_void,
                                ) < 0 as libc::c_int
                                {
                                    mln_pg_state_free(new_s);
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                        it = (*it).next;
                    }
                    if mln_pg_closure(new_s, (*pci).rule, (*pci).nr_rule)
                        < 0 as libc::c_int
                    {
                        mln_pg_state_free(new_s);
                        return -(1 as libc::c_int);
                    }
                    rn = mln_rbtree_search(tree, new_s as *mut libc::c_void);
                    if !(rn == &mut (*tree).nil as *mut mln_rbtree_node_t) {
                        tmp_s = (*rn).data as *mut mln_pg_state_t;
                        if mln_pg_state_duplicate(&mut q_head, &mut q_tail, tmp_s, new_s)
                            < 0 as libc::c_int
                        {
                            mln_pg_state_free(new_s);
                            return -(1 as libc::c_int);
                        }
                        mln_pg_state_free(new_s);
                        new_s = tmp_s;
                    } else {
                        let fresh4 = (*pci).id_counter;
                        (*pci).id_counter = (*pci).id_counter + 1;
                        (*new_s).id = fresh4;
                        rn = mln_rbtree_node_new(tree, new_s as *mut libc::c_void);
                        if rn.is_null() {
                            _mln_sys_log(
                                error,
                                b"src/mln_parser_generator.c\0" as *const u8
                                    as *const libc::c_char,
                                (*::core::mem::transmute::<
                                    &[u8; 12],
                                    &[libc::c_char; 12],
                                >(b"mln_pg_goto\0"))
                                    .as_ptr(),
                                729 as libc::c_int,
                                b"No memory.\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            mln_pg_state_free(new_s);
                            return -(1 as libc::c_int);
                        }
                        mln_rbtree_insert(tree, rn);
                        mln_state_chain_add(&mut (*pci).head, &mut (*pci).tail, new_s);
                        mln_pg_state_enqueue(&mut q_head, &mut q_tail, new_s);
                    }
                    it = save;
                    while !it.is_null() {
                        if !((*(*it).rule).nr_right == (*it).pos) {
                            if *((*(*it).rule).rights).offset((*it).pos as isize) == tk {
                                (*it).goto_id = (*new_s).id;
                            }
                        }
                        it = (*it).next;
                    }
                }
            }
            save = (*save).next;
        }
    }
    (*(*pci).head).input = 0 as *mut mln_pg_token_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_goto_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut it: *mut mln_pg_item_t = udata as *mut mln_pg_item_t;
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut rn: *mut mln_rbtree_node_t = mln_rbtree_node_new(
        (*it).lookahead_set,
        tk as *mut libc::c_void,
    );
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"mln_pg_goto_iterate_handler\0"))
                .as_ptr(),
            757 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert((*it).lookahead_set, rn);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_pg_state_duplicate(
    mut q_head: *mut *mut mln_pg_state_t,
    mut q_tail: *mut *mut mln_pg_state_t,
    mut dest: *mut mln_pg_state_t,
    mut src: *mut mln_pg_state_t,
) -> libc::c_int {
    let mut change: libc::c_int = 0;
    let mut added: libc::c_int = 0 as libc::c_int;
    let mut dit: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut sit: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut info: mln_pg_state = mln_pg_state {
        change: 0 as *mut libc::c_int,
        r: 0 as *mut mln_pg_rule_t,
        s: 0 as *mut mln_pg_state_t,
        item: 0 as *mut mln_pg_item_t,
    };
    dit = (*dest).head;
    while !dit.is_null() {
        change = 0 as libc::c_int;
        sit = (*src).head;
        while !sit.is_null() {
            if (*dit).rule == (*sit).rule && (*dit).pos == (*sit).pos {
                info.change = &mut change;
                info.item = dit;
                if mln_rbtree_iterate(
                    (*sit).lookahead_set,
                    Some(
                        mln_pg_goto_la_iterate_handler
                            as unsafe extern "C" fn(
                                *mut mln_rbtree_node_t,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    &mut info as *mut mln_pg_state as *mut libc::c_void,
                ) < 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                if change != 0 && added == 0 {
                    added = 1 as libc::c_int;
                    if ((*dest).q_next).is_null() && *q_tail != dest {
                        mln_pg_state_enqueue(q_head, q_tail, dest);
                    }
                }
                break;
            } else {
                sit = (*sit).next;
            }
        }
        dit = (*dit).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_goto_la_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut mln_pg_state = udata as *mut mln_pg_state;
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    rn = mln_rbtree_search((*(*info).item).lookahead_set, tk as *mut libc::c_void);
    if !(rn == &mut (*(*(*info).item).lookahead_set).nil as *mut mln_rbtree_node_t) {
        return 0 as libc::c_int;
    }
    rn = mln_rbtree_node_new((*(*info).item).lookahead_set, tk as *mut libc::c_void);
    if rn.is_null() {
        _mln_sys_log(
            error,
            b"src/mln_parser_generator.c\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"mln_pg_goto_la_iterate_handler\0"))
                .as_ptr(),
            807 as libc::c_int,
            b"No memory.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    mln_rbtree_insert((*(*info).item).lookahead_set, rn);
    *(*info).change = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_output_token(
    mut tree: *mut mln_rbtree_t,
    mut r: *mut mln_pg_rule_t,
    mut nr_rule: mln_u32_t,
) {
    let mut tk: *mut mln_pg_token_t = 0 as *mut mln_pg_token_t;
    let mut pr: *mut mln_pg_rule_t = 0 as *mut mln_pg_rule_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    printf(b"RULE:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < nr_rule {
        pr = &mut *r.offset(i as isize) as *mut mln_pg_rule_t;
        printf(
            b"No.%d %s->\0" as *const u8 as *const libc::c_char,
            i,
            (*(*(*pr).left).token).data as *mut libc::c_char,
        );
        j = 0 as libc::c_int;
        while (j as libc::c_uint) < (*pr).nr_right {
            tk = *((*pr).rights).offset(j as isize);
            printf(
                b"%s \0" as *const u8 as *const libc::c_char,
                (*(*tk).token).data as *mut libc::c_char,
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"TOKEN:\n\0" as *const u8 as *const libc::c_char);
    mln_rbtree_iterate(
        tree,
        Some(
            mln_pg_output_token_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn mln_pg_output_token_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    printf(
        b"[%s]:\0" as *const u8 as *const libc::c_char,
        (*(*tk).token).data as *mut libc::c_char,
    );
    printf(
        b"%d %s %s\n\0" as *const u8 as *const libc::c_char,
        (*tk).type_0,
        if (*tk).is_nonterminal() as libc::c_int != 0 {
            b"Nonterminal\0" as *const u8 as *const libc::c_char
        } else {
            b"Terminal\0" as *const u8 as *const libc::c_char
        },
        if (*tk).is_nullable() as libc::c_int != 0 {
            b"Nullable\0" as *const u8 as *const libc::c_char
        } else {
            b"Unnullable\0" as *const u8 as *const libc::c_char
        },
    );
    printf(b"First set:\n\0" as *const u8 as *const libc::c_char);
    mln_rbtree_iterate(
        (*tk).first_set,
        Some(
            mln_pg_output_token_set_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Follow set:\n\0" as *const u8 as *const libc::c_char);
    mln_rbtree_iterate(
        (*tk).follow_set,
        Some(
            mln_pg_output_token_set_iterate_handler
                as unsafe extern "C" fn(
                    *mut mln_rbtree_node_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_pg_output_token_set_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut tk: *mut mln_pg_token_t = (*node).data as *mut mln_pg_token_t;
    printf(
        b"[%s] \0" as *const u8 as *const libc::c_char,
        (*(*tk).token).data as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_pg_output_state(mut s: *mut mln_pg_state_t) {
    let mut it: *mut mln_pg_item_t = 0 as *mut mln_pg_item_t;
    let mut i: mln_u32_t = 0;
    printf(b"STATES:\n\0" as *const u8 as *const libc::c_char);
    while !s.is_null() {
        printf(
            b"State %ld: input: [%s] nr_item:%llu\n\0" as *const u8
                as *const libc::c_char,
            (*s).id,
            if ((*s).input).is_null() {
                b"(null)\0" as *const u8 as *const libc::c_char
            } else {
                (*(*(*s).input).token).data as *mut libc::c_char as *const libc::c_char
            },
            (*s).nr_item as libc::c_ulonglong,
        );
        printf(b"Items:\n\0" as *const u8 as *const libc::c_char);
        it = (*s).head;
        while !it.is_null() {
            printf(
                b"rule: %s->\0" as *const u8 as *const libc::c_char,
                (*(*(*(*it).rule).left).token).data as *mut libc::c_char,
            );
            i = 0 as libc::c_int as mln_u32_t;
            while i < (*(*it).rule).nr_right {
                if (*it).pos == i {
                    printf(b".\0" as *const u8 as *const libc::c_char);
                }
                printf(
                    b" %s\0" as *const u8 as *const libc::c_char,
                    (*(**((*(*it).rule).rights).offset(i as isize)).token).data
                        as *mut libc::c_char,
                );
                i = i.wrapping_add(1);
                i;
            }
            printf(
                b"\t\tread:[%s] goto_id:%ld\0" as *const u8 as *const libc::c_char,
                if ((*it).read).is_null() {
                    b"(null)\0" as *const u8 as *const libc::c_char
                } else {
                    (*(*(*it).read).token).data as *mut libc::c_char
                        as *const libc::c_char
                },
                (*it).goto_id,
            );
            printf(b"\tLookahead:\0" as *const u8 as *const libc::c_char);
            mln_rbtree_iterate(
                (*it).lookahead_set,
                Some(
                    mln_pg_output_state_iterate_handler
                        as unsafe extern "C" fn(
                            *mut mln_rbtree_node_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            it = (*it).next;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        s = (*s).next;
    }
}
unsafe extern "C" fn mln_pg_output_state_iterate_handler(
    mut node: *mut mln_rbtree_node_t,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    printf(
        b" %s\0" as *const u8 as *const libc::c_char,
        (*(*((*node).data as *mut mln_pg_token_t)).token).data as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
