use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
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
                    current_block = 17999661984222547796;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                17999661984222547796 => {}
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
        current_block_8 = 16379945107832692948;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 16379945107832692948;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        16379945107832692948 => {
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
#[inline]
unsafe extern "C" fn mln_string_assign(
    mut s: *mut libc::c_char,
    mut len: mln_u32_t,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = 0 as *mut mln_string_t;
    str = malloc(::core::mem::size_of::<mln_string_t>() as libc::c_ulong)
        as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*str).data = s as mln_u8ptr_t;
    (*str).len = len as mln_u64_t;
    (*str).set_pool(0 as libc::c_int as mln_uauto_t);
    (*str).set_data_ref(1 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_buf_new(
    mut buf: mln_u8ptr_t,
    mut len: mln_u64_t,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*str).data = buf;
    (*str).len = len;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(0 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_buf_pool_new(
    mut pool: *mut mln_alloc_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_u64_t,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*str).data = buf;
    (*str).len = len;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(1 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_new(
    mut pool: *mut mln_alloc_t,
    mut s: *const libc::c_char,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    if s.is_null() {
        (*str).data = 0 as mln_u8ptr_t;
        (*str).len = 0 as libc::c_int as mln_u64_t;
        (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
        (*str).set_pool(1 as libc::c_int as mln_uauto_t);
        (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
        return str;
    }
    let mut len: mln_s32_t = strlen(s) as mln_s32_t;
    (*str)
        .data = mln_alloc_m(pool, (len + 1 as libc::c_int) as mln_size_t) as mln_u8ptr_t;
    if ((*str).data).is_null() {
        mln_alloc_free(str as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    memcpy(
        (*str).data as *mut libc::c_void,
        s as *const libc::c_void,
        len as libc::c_ulong,
    );
    *((*str).data).offset(len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*str).len = len as mln_u64_t;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(1 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_new(
    mut s: *const libc::c_char,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    if s.is_null() {
        (*str).data = 0 as mln_u8ptr_t;
        (*str).len = 0 as libc::c_int as mln_u64_t;
        (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
        (*str).set_pool(0 as libc::c_int as mln_uauto_t);
        (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
        return str;
    }
    let mut len: mln_s32_t = strlen(s) as mln_s32_t;
    (*str).data = malloc((len + 1 as libc::c_int) as libc::c_ulong) as mln_u8ptr_t;
    if ((*str).data).is_null() {
        free(str as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    memcpy(
        (*str).data as *mut libc::c_void,
        s as *const libc::c_void,
        len as libc::c_ulong,
    );
    *((*str).data).offset(len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*str).len = len as mln_u64_t;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(0 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_dup(
    mut str: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut s: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s)
        .data = malloc(((*str).len).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    if ((*s).data).is_null() {
        free(s as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if !((*str).data).is_null() {
        memcpy(
            (*s).data as *mut libc::c_void,
            (*str).data as *const libc::c_void,
            (*str).len,
        );
    }
    *((*s).data).offset((*str).len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*s).len = (*str).len;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(0 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_dup(
    mut pool: *mut mln_alloc_t,
    mut str: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut s: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s)
        .data = mln_alloc_m(
        pool,
        ((*str).len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as mln_u8ptr_t;
    if ((*s).data).is_null() {
        mln_alloc_free(s as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if !((*str).data).is_null() {
        memcpy(
            (*s).data as *mut libc::c_void,
            (*str).data as *const libc::c_void,
            (*str).len,
        );
    }
    *((*s).data).offset((*str).len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*s).len = (*str).len;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(1 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_alloc(mut size: mln_s32_t) -> *mut mln_string_t {
    if size < 0 as libc::c_int {
        return 0 as *mut mln_string_t;
    }
    let mut s: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s).data = malloc((size + 1 as libc::c_int) as libc::c_ulong) as mln_u8ptr_t;
    if ((*s).data).is_null() {
        free(s as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    (*s).len = size as mln_u64_t;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(0 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_alloc(
    mut pool: *mut mln_alloc_t,
    mut size: mln_s32_t,
) -> *mut mln_string_t {
    if size < 0 as libc::c_int {
        return 0 as *mut mln_string_t;
    }
    let mut s: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s)
        .data = mln_alloc_m(pool, (size + 1 as libc::c_int) as mln_size_t)
        as mln_u8ptr_t;
    if ((*s).data).is_null() {
        mln_alloc_free(s as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    (*s).len = size as mln_u64_t;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(1 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_ndup(
    mut str: *mut libc::c_char,
    mut size: mln_s32_t,
) -> *mut mln_string_t {
    if size < 0 as libc::c_int {
        return 0 as *mut mln_string_t;
    }
    let mut s: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s).data = malloc((size + 1 as libc::c_int) as libc::c_ulong) as mln_u8ptr_t;
    if ((*s).data).is_null() {
        free(s as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    memcpy(
        (*s).data as *mut libc::c_void,
        str as *const libc::c_void,
        size as libc::c_ulong,
    );
    *((*s).data).offset(size as isize) = 0 as libc::c_int as libc::c_uchar;
    (*s).len = size as mln_u64_t;
    (*s).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*s).set_pool(0 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_ref_dup(
    mut str: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut s: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*s).data = (*str).data;
    (*s).len = (*str).len;
    (*s).set_data_ref(1 as libc::c_int as mln_uauto_t);
    (*s).set_pool(0 as libc::c_int as mln_uauto_t);
    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_ref_dup(
    mut s: *mut libc::c_char,
) -> *mut mln_string_t {
    if s.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut str: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    (*str).data = s as mln_u8ptr_t;
    (*str).len = strlen(s);
    (*str).set_data_ref(1 as libc::c_int as mln_uauto_t);
    (*str).set_pool(0 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_concat(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
    mut sep: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut size: mln_size_t = 0 as libc::c_int as mln_size_t;
    if !s1.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*s1).len) as mln_size_t
            as mln_size_t;
        if !s2.is_null() && !sep.is_null() {
            size = (size as libc::c_ulong).wrapping_add((*sep).len) as mln_size_t
                as mln_size_t;
        }
    }
    if !s2.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*s2).len) as mln_size_t
            as mln_size_t;
    }
    (*str)
        .data = malloc(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    p = (*str).data;
    if p.is_null() {
        free(str as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if !s1.is_null() {
        memcpy(p as *mut libc::c_void, (*s1).data as *const libc::c_void, (*s1).len);
        p = p.offset((*s1).len as isize);
        if !s2.is_null() && !sep.is_null() {
            memcpy(
                p as *mut libc::c_void,
                (*sep).data as *const libc::c_void,
                (*sep).len,
            );
            p = p.offset((*sep).len as isize);
        }
    }
    if !s2.is_null() {
        memcpy(p as *mut libc::c_void, (*s2).data as *const libc::c_void, (*s2).len);
        p = p.offset((*s2).len as isize);
    }
    *p = 0 as libc::c_int as libc::c_uchar;
    (*str).len = size;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(0 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_concat(
    mut pool: *mut mln_alloc_t,
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
    mut sep: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut str: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if str.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut size: mln_size_t = 0 as libc::c_int as mln_size_t;
    if !s1.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*s1).len) as mln_size_t
            as mln_size_t;
        if !s2.is_null() && !sep.is_null() {
            size = (size as libc::c_ulong).wrapping_add((*sep).len) as mln_size_t
                as mln_size_t;
        }
    }
    if !s2.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*s2).len) as mln_size_t
            as mln_size_t;
    }
    (*str)
        .data = mln_alloc_m(pool, size.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    p = (*str).data;
    if p.is_null() {
        mln_alloc_free(str as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if !s1.is_null() {
        memcpy(p as *mut libc::c_void, (*s1).data as *const libc::c_void, (*s1).len);
        p = p.offset((*s1).len as isize);
        if !s2.is_null() && !sep.is_null() {
            memcpy(
                p as *mut libc::c_void,
                (*sep).data as *const libc::c_void,
                (*sep).len,
            );
            p = p.offset((*sep).len as isize);
        }
    }
    if !s2.is_null() {
        memcpy(p as *mut libc::c_void, (*s2).data as *const libc::c_void, (*s2).len);
        p = p.offset((*s2).len as isize);
    }
    *p = 0 as libc::c_int as libc::c_uchar;
    (*str).len = size;
    (*str).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*str).set_pool(1 as libc::c_int as mln_uauto_t);
    (*str).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strseqcmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*s1).len > (*s2).len {
        if (*s2).len == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
        ret = memcmp(
            (*s1).data as *const libc::c_void,
            (*s2).data as *const libc::c_void,
            (*s2).len,
        );
        return if ret != 0 { ret } else { 1 as libc::c_int };
    } else if (*s1).len < (*s2).len {
        if (*s1).len == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
        ret = memcmp(
            (*s1).data as *const libc::c_void,
            (*s2).data as *const libc::c_void,
            (*s1).len,
        );
        return if ret != 0 { ret } else { -(1 as libc::c_int) };
    }
    if (*s1).len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    return memcmp(
        (*s1).data as *const libc::c_void,
        (*s2).data as *const libc::c_void,
        (*s1).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strcmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
) -> libc::c_int {
    if s1 == s2 || (*s1).data == (*s2).data {
        return 0 as libc::c_int;
    }
    if (*s1).len > (*s2).len {
        return 1 as libc::c_int;
    }
    if (*s1).len < (*s2).len {
        return -(1 as libc::c_int);
    }
    if (*s1).len > 280 as libc::c_int as libc::c_ulong
        || ((*s1).len).wrapping_rem(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
            != 0
    {
        return memcmp(
            (*s1).data as *const libc::c_void,
            (*s2).data as *const libc::c_void,
            (*s1).len,
        );
    }
    let mut i1: *mut mln_u32_t = (*s1).data as *mut mln_u32_t;
    let mut i2: *mut mln_u32_t = (*s2).data as *mut mln_u32_t;
    let mut i: mln_u32_t = 0;
    let mut res: mln_s32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < (*s1).len {
        let fresh1 = i1;
        i1 = i1.offset(1);
        let fresh2 = i2;
        i2 = i2.offset(1);
        res = (*fresh1).wrapping_sub(*fresh2) as mln_s32_t;
        if res != 0 as libc::c_int {
            return res;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
            as mln_u32_t as mln_u32_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_strcmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    if (*s1).data == s2 as mln_u8ptr_t {
        return 0 as libc::c_int;
    }
    let mut len: mln_u32_t = strlen(s2) as mln_u32_t;
    if (*s1).len > len as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if (*s1).len < len as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if (*s1).len > 280 as libc::c_int as libc::c_ulong
        || (len as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong) != 0
    {
        return memcmp(
            (*s1).data as *const libc::c_void,
            s2 as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    let mut i1: *mut mln_u32_t = (*s1).data as *mut mln_u32_t;
    let mut i2: *mut mln_u32_t = s2 as *mut mln_u32_t;
    let mut i: mln_u32_t = 0;
    let mut res: mln_s32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < len {
        let fresh3 = i1;
        i1 = i1.offset(1);
        let fresh4 = i2;
        i2 = i2.offset(1);
        res = (*fresh3).wrapping_sub(*fresh4) as mln_s32_t;
        if res != 0 as libc::c_int {
            return res;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
            as mln_u32_t as mln_u32_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strncmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
    mut n: mln_u32_t,
) -> libc::c_int {
    if s1 == s2 || (*s1).data == (*s2).data {
        return 0 as libc::c_int;
    }
    if (*s1).len < n as libc::c_ulong || (*s2).len < n as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if n > 280 as libc::c_int as libc::c_uint
        || (n as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong) != 0
    {
        return memcmp(
            (*s1).data as *const libc::c_void,
            (*s2).data as *const libc::c_void,
            n as libc::c_ulong,
        );
    }
    let mut i1: *mut mln_u32_t = (*s1).data as *mut mln_u32_t;
    let mut i2: *mut mln_u32_t = (*s2).data as *mut mln_u32_t;
    let mut i: mln_u32_t = 0;
    let mut res: mln_s32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < n {
        let fresh5 = i1;
        i1 = i1.offset(1);
        let fresh6 = i2;
        i2 = i2.offset(1);
        res = (*fresh5).wrapping_sub(*fresh6) as mln_s32_t;
        if res != 0 as libc::c_int {
            return res;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
            as mln_u32_t as mln_u32_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_strncmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut libc::c_char,
    mut n: mln_u32_t,
) -> libc::c_int {
    if (*s1).data == s2 as mln_u8ptr_t {
        return 0 as libc::c_int;
    }
    if (*s1).len < n as libc::c_ulong || strlen(s2) < n as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if n > 280 as libc::c_int as libc::c_uint
        || (n as libc::c_ulong)
            .wrapping_rem(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong) != 0
    {
        return memcmp(
            (*s1).data as *const libc::c_void,
            s2 as *const libc::c_void,
            n as libc::c_ulong,
        );
    }
    let mut i1: *mut mln_u32_t = (*s1).data as *mut mln_u32_t;
    let mut i2: *mut mln_u32_t = s2 as *mut mln_u32_t;
    let mut i: mln_u32_t = 0;
    let mut res: mln_s32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < n {
        let fresh7 = i1;
        i1 = i1.offset(1);
        let fresh8 = i2;
        i2 = i2.offset(1);
        res = (*fresh7).wrapping_sub(*fresh8) as mln_s32_t;
        if res != 0 as libc::c_int {
            return res;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
            as mln_u32_t as mln_u32_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strcasecmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
) -> libc::c_int {
    if s1 == s2 || (*s1).data == (*s2).data {
        return 0 as libc::c_int;
    }
    if (*s1).len > (*s2).len {
        return 1 as libc::c_int;
    }
    if (*s1).len < (*s2).len {
        return -(1 as libc::c_int);
    }
    return strncasecmp(
        (*s1).data as *mut libc::c_char,
        (*s2).data as *mut libc::c_char,
        (*s1).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strncasecmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
    mut n: mln_u32_t,
) -> libc::c_int {
    if s1 == s2 || (*s1).data == (*s2).data {
        return 0 as libc::c_int;
    }
    if (*s1).len < n as libc::c_ulong || (*s2).len < n as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return strncasecmp(
        (*s1).data as *mut libc::c_char,
        (*s2).data as *mut libc::c_char,
        n as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_strcasecmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    if (*s1).data == s2 as mln_u8ptr_t {
        return 0 as libc::c_int;
    }
    let mut len: mln_u32_t = strlen(s2) as mln_u32_t;
    if (*s1).len > len as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if (*s1).len < len as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return strncasecmp((*s1).data as *mut libc::c_char, s2, len as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_strncasecmp(
    mut s1: *mut mln_string_t,
    mut s2: *mut libc::c_char,
    mut n: mln_u32_t,
) -> libc::c_int {
    if (*s1).data == s2 as mln_u8ptr_t {
        return 0 as libc::c_int;
    }
    let mut len: mln_u32_t = strlen(s2) as mln_u32_t;
    if (*s1).len < n as libc::c_ulong || len < n {
        return -(1 as libc::c_int);
    }
    return strncasecmp((*s1).data as *mut libc::c_char, s2, n as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strstr(
    mut text: *mut mln_string_t,
    mut pattern: *mut mln_string_t,
) -> *mut libc::c_char {
    if text == pattern || (*text).data == (*pattern).data {
        return (*text).data as *mut libc::c_char;
    }
    return strstr(
        (*text).data as *mut libc::c_char,
        (*pattern).data as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_strstr(
    mut text: *mut mln_string_t,
    mut pattern: *mut libc::c_char,
) -> *mut libc::c_char {
    if (*text).data == pattern as mln_u8ptr_t {
        return (*text).data as *mut libc::c_char;
    }
    return strstr((*text).data as *mut libc::c_char, pattern);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_new_strstr(
    mut text: *mut mln_string_t,
    mut pattern: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut p: *mut libc::c_char = mln_string_strstr(text, pattern);
    if p.is_null() {
        return 0 as *mut mln_string_t;
    }
    return mln_string_assign(
        p,
        ((*text).len)
            .wrapping_sub(
                p.offset_from((*text).data as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            ) as mln_u32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_new_const_strstr(
    mut text: *mut mln_string_t,
    mut pattern: *mut libc::c_char,
) -> *mut mln_string_t {
    let mut p: *mut libc::c_char = mln_string_const_strstr(text, pattern);
    if p.is_null() {
        return 0 as *mut mln_string_t;
    }
    return mln_string_assign(
        p,
        ((*text).len)
            .wrapping_sub(
                p.offset_from((*text).data as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            ) as mln_u32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_kmp(
    mut text: *mut mln_string_t,
    mut pattern: *mut mln_string_t,
) -> *mut libc::c_char {
    if text == pattern || (*text).data == (*pattern).data {
        return (*text).data as *mut libc::c_char;
    }
    return kmp_string_match(
        (*text).data as *mut libc::c_char,
        (*pattern).data as *mut libc::c_char,
        (*text).len as libc::c_int,
        (*pattern).len as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_const_kmp(
    mut text: *mut mln_string_t,
    mut pattern: *mut libc::c_char,
) -> *mut libc::c_char {
    if (*text).data == pattern as mln_u8ptr_t {
        return (*text).data as *mut libc::c_char;
    }
    return kmp_string_match(
        (*text).data as *mut libc::c_char,
        pattern,
        (*text).len as libc::c_int,
        strlen(pattern) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_new_kmp(
    mut text: *mut mln_string_t,
    mut pattern: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut p: *mut libc::c_char = mln_string_kmp(text, pattern);
    if p.is_null() {
        return 0 as *mut mln_string_t;
    }
    return mln_string_assign(
        p,
        ((*text).len)
            .wrapping_sub(
                p.offset_from((*text).data as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            ) as mln_u32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_new_const_kmp(
    mut text: *mut mln_string_t,
    mut pattern: *mut libc::c_char,
) -> *mut mln_string_t {
    let mut p: *mut libc::c_char = mln_string_const_kmp(text, pattern);
    if p.is_null() {
        return 0 as *mut mln_string_t;
    }
    return mln_string_assign(
        p,
        ((*text).len)
            .wrapping_sub(
                p.offset_from((*text).data as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            ) as mln_u32_t,
    );
}
#[inline]
unsafe extern "C" fn kmp_string_match(
    mut text: *mut libc::c_char,
    mut pattern: *const libc::c_char,
    mut text_len: libc::c_int,
    mut pattern_len: libc::c_int,
) -> *mut libc::c_char {
    let mut shift: *mut libc::c_int = compute_prefix_function(pattern, pattern_len);
    if shift.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut q: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < text_len {
        while q > 0 as libc::c_int
            && *pattern.offset(q as isize) as libc::c_int
                != *text.offset(i as isize) as libc::c_int
        {
            q = *shift.offset((q - 1 as libc::c_int) as isize);
        }
        if *pattern.offset(q as isize) as libc::c_int
            == *text.offset(i as isize) as libc::c_int
        {
            q += 1;
            q;
        }
        if q == pattern_len {
            free(shift as *mut libc::c_void);
            return &mut *text.offset((i - pattern_len + 1 as libc::c_int) as isize)
                as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    free(shift as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn compute_prefix_function(
    mut pattern: *const libc::c_char,
    mut m: libc::c_int,
) -> *mut libc::c_int {
    let mut shift: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(m as libc::c_ulong),
    ) as *mut libc::c_int;
    if shift.is_null() {
        return 0 as *mut libc::c_int;
    }
    *shift.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut q: libc::c_int = 0;
    q = 1 as libc::c_int;
    while q < m {
        while k > 0 as libc::c_int
            && *pattern.offset(k as isize) as libc::c_int
                != *pattern.offset(q as isize) as libc::c_int
        {
            k = *shift.offset((k - 1 as libc::c_int) as isize);
        }
        if *pattern.offset(k as isize) as libc::c_int
            == *pattern.offset(q as isize) as libc::c_int
        {
            k += 1;
            k;
        }
        *shift.offset(q as isize) = k;
        q += 1;
        q;
    }
    return shift;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_slice(
    mut s: *mut mln_string_t,
    mut sep_array: *const libc::c_char,
) -> *mut mln_string_t {
    let mut ps: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut mln_string_t = mln_string_dup(s);
    let mut ascii: [mln_u8_t; 256] = [
        0 as libc::c_int as mln_u8_t,
    ];
    if tmp.is_null() {
        return 0 as *mut mln_string_t;
    }
    ps = sep_array;
    while *ps as libc::c_int != 0 as libc::c_int {
        ascii[*ps as mln_u8_t as usize] = 1 as libc::c_int as mln_u8_t;
        ps = ps.offset(1);
        ps;
    }
    return mln_string_slice_recursive(
        (*tmp).data as *mut libc::c_char,
        (*tmp).len,
        ascii.as_mut_ptr(),
        1 as libc::c_int,
        tmp,
    );
}
unsafe extern "C" fn mln_string_slice_recursive(
    mut s: *mut libc::c_char,
    mut len: mln_u64_t,
    mut ascii: mln_u8ptr_t,
    mut cnt: libc::c_int,
    mut save: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut jmp_ascii: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = s.offset(len as isize);
    jmp_ascii = s;
    while jmp_ascii < end {
        if *ascii.offset(*jmp_ascii as mln_u8_t as isize) == 0 {
            break;
        }
        *jmp_ascii = 0 as libc::c_int as libc::c_char;
        jmp_ascii = jmp_ascii.offset(1);
        jmp_ascii;
    }
    if jmp_ascii >= end {
        let mut ret: *mut mln_string_t = malloc(
            (::core::mem::size_of::<mln_string_t>() as libc::c_ulong)
                .wrapping_mul(cnt as libc::c_ulong),
        ) as *mut mln_string_t;
        if ret.is_null() {
            return 0 as *mut mln_string_t;
        }
        let ref mut fresh9 = (*ret.offset((cnt - 1 as libc::c_int) as isize)).data;
        *fresh9 = save as mln_u8ptr_t;
        (*ret.offset((cnt - 1 as libc::c_int) as isize))
            .len = 0 as libc::c_int as mln_u64_t;
        let ref mut fresh10 = *ret.offset((cnt - 1 as libc::c_int) as isize);
        (*fresh10).set_data_ref(0 as libc::c_int as mln_uauto_t);
        let ref mut fresh11 = *ret.offset((cnt - 1 as libc::c_int) as isize);
        (*fresh11).set_pool(0 as libc::c_int as mln_uauto_t);
        let ref mut fresh12 = *ret.offset((cnt - 1 as libc::c_int) as isize);
        (*fresh12).set_ref_0(1 as libc::c_int as mln_uauto_t);
        return ret;
    }
    cnt += 1;
    cnt;
    let mut jmp_valid: *mut libc::c_char = 0 as *mut libc::c_char;
    jmp_valid = jmp_ascii;
    while jmp_valid < end {
        if *ascii.offset(*jmp_valid as mln_u8_t as isize) != 0 {
            break;
        }
        jmp_valid = jmp_valid.offset(1);
        jmp_valid;
    }
    let mut array: *mut mln_string_t = mln_string_slice_recursive(
        jmp_valid,
        len.wrapping_sub(jmp_valid.offset_from(s) as libc::c_long as libc::c_ulong),
        ascii,
        cnt,
        save,
    );
    if array.is_null() {
        return 0 as *mut mln_string_t;
    }
    let ref mut fresh13 = (*array.offset((cnt - 2 as libc::c_int) as isize)).data;
    *fresh13 = jmp_ascii as mln_u8ptr_t;
    (*array.offset((cnt - 2 as libc::c_int) as isize))
        .len = jmp_valid.offset_from(jmp_ascii) as libc::c_long as mln_u64_t;
    let ref mut fresh14 = *array.offset((cnt - 2 as libc::c_int) as isize);
    (*fresh14).set_data_ref(1 as libc::c_int as mln_uauto_t);
    let ref mut fresh15 = *array.offset((cnt - 2 as libc::c_int) as isize);
    (*fresh15).set_pool(0 as libc::c_int as mln_uauto_t);
    let ref mut fresh16 = *array.offset((cnt - 2 as libc::c_int) as isize);
    (*fresh16).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_slice_free(mut array: *mut mln_string_t) {
    let mut s: *mut mln_string_t = &mut *array.offset(0 as libc::c_int as isize)
        as *mut mln_string_t;
    while (*s).len != 0 {
        s = s.offset(1);
        s;
    }
    let mut __s: *mut mln_string_t = (*s).data as *mut mln_string_t;
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
    free(array as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_strcat(
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut ret: *mut mln_string_t = malloc(
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if ret.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut len: mln_u64_t = ((*s1).len).wrapping_add((*s2).len);
    if len == 0 as libc::c_int as libc::c_ulong {
        (*ret).data = 0 as mln_u8ptr_t;
        (*ret).len = 0 as libc::c_int as mln_u64_t;
        (*ret).set_data_ref(0 as libc::c_int as mln_uauto_t);
        (*ret).set_pool(0 as libc::c_int as mln_uauto_t);
        (*ret).set_ref_0(1 as libc::c_int as mln_uauto_t);
        return ret;
    }
    (*ret)
        .data = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    if ((*ret).data).is_null() {
        free(ret as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if (*s1).len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            (*ret).data as *mut libc::c_void,
            (*s1).data as *const libc::c_void,
            (*s1).len,
        );
    }
    if (*s2).len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            ((*ret).data).offset((*s1).len as isize) as *mut libc::c_void,
            (*s2).data as *const libc::c_void,
            (*s2).len,
        );
    }
    *((*ret).data).offset(len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*ret).len = len;
    (*ret).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*ret).set_pool(0 as libc::c_int as mln_uauto_t);
    (*ret).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_strcat(
    mut pool: *mut mln_alloc_t,
    mut s1: *mut mln_string_t,
    mut s2: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut ret: *mut mln_string_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_string_t>() as libc::c_ulong,
    ) as *mut mln_string_t;
    if ret.is_null() {
        return 0 as *mut mln_string_t;
    }
    let mut len: mln_u64_t = ((*s1).len).wrapping_add((*s2).len);
    if len == 0 as libc::c_int as libc::c_ulong {
        (*ret).data = 0 as mln_u8ptr_t;
        (*ret).len = 0 as libc::c_int as mln_u64_t;
        (*ret).set_data_ref(0 as libc::c_int as mln_uauto_t);
        (*ret).set_pool(1 as libc::c_int as mln_uauto_t);
        (*ret).set_ref_0(1 as libc::c_int as mln_uauto_t);
        return ret;
    }
    (*ret)
        .data = mln_alloc_m(pool, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as mln_u8ptr_t;
    if ((*ret).data).is_null() {
        mln_alloc_free(ret as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    if (*s1).len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            (*ret).data as *mut libc::c_void,
            (*s1).data as *const libc::c_void,
            (*s1).len,
        );
    }
    if (*s2).len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            ((*ret).data).offset((*s1).len as isize) as *mut libc::c_void,
            (*s2).data as *const libc::c_void,
            (*s2).len,
        );
    }
    *((*ret).data).offset(len as isize) = 0 as libc::c_int as libc::c_uchar;
    (*ret).len = len;
    (*ret).set_data_ref(0 as libc::c_int as mln_uauto_t);
    (*ret).set_pool(1 as libc::c_int as mln_uauto_t);
    (*ret).set_ref_0(1 as libc::c_int as mln_uauto_t);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_trim(
    mut s: *mut mln_string_t,
    mut mask: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut chars: [mln_u8_t; 256] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    i = 0 as libc::c_int as mln_size_t;
    while i < (*mask).len {
        chars[*((*mask).data).offset(i as isize)
            as usize] = 1 as libc::c_int as mln_u8_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < (*s).len {
        if chars[*((*s).data).offset(i as isize) as usize] == 0 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    j = (*s).len;
    while j > i {
        if chars[*((*s).data)
            .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as usize]
            == 0
        {
            break;
        }
        j = j.wrapping_sub(1);
        j;
    }
    ({
        tmp.data = &mut *((*s).data).offset(i as isize) as *mut libc::c_uchar;
        tmp.len = j.wrapping_sub(i);
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    return mln_string_dup(&mut tmp);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_pool_trim(
    mut pool: *mut mln_alloc_t,
    mut s: *mut mln_string_t,
    mut mask: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut chars: [mln_u8_t; 256] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    i = 0 as libc::c_int as mln_size_t;
    while i < (*mask).len {
        chars[*((*mask).data).offset(i as isize)
            as usize] = 1 as libc::c_int as mln_u8_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < (*s).len {
        if chars[*((*s).data).offset(i as isize) as usize] == 0 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    j = (*s).len;
    while j > i {
        if chars[*((*s).data)
            .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as usize]
            == 0
        {
            break;
        }
        j = j.wrapping_sub(1);
        j;
    }
    ({
        tmp.data = &mut *((*s).data).offset(i as isize) as *mut libc::c_uchar;
        tmp.len = j.wrapping_sub(i);
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    return mln_string_pool_dup(pool, &mut tmp);
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_upper(mut s: *mut mln_string_t) {
    let mut chars: [mln_u8_t; 256] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut upper: [mln_u8_t; 27] = [0; 27]; // 创建一个空的 mln_u8_t 数组
    let mut lower: [mln_u8_t; 27] = [0; 27]; // 创建另一个空的 mln_u8_t 数组

    // 将字节数据复制到数组中
    upper.copy_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0");
    lower.copy_from_slice(b"abcdefghijklmnopqrstuvwxyz\0");

    //let mut upper: [mln_u8_t; 27] = *::core::mem::transmute::<
        //&[u8; 27],
        //&mut [mln_u8_t; 27],
    //>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0");
    //let mut lower: [mln_u8_t; 27] = *::core::mem::transmute::<
        //&[u8; 27],
        //&mut [mln_u8_t; 27],
    //>(b"abcdefghijklmnopqrstuvwxyz\0");
    let mut i: mln_size_t = 0;
    i = 0 as libc::c_int as mln_size_t;
    while i
        < (::core::mem::size_of::<[mln_u8_t; 27]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        chars[lower[i as usize] as usize] = upper[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < (*s).len {
        if chars[*((*s).data).offset(i as isize) as usize] != 0 {
            *((*s).data)
                .offset(i as isize) = chars[*((*s).data).offset(i as isize) as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_string_lower(mut s: *mut mln_string_t) {
    let mut chars: [mln_u8_t; 256] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut upper: [mln_u8_t; 27] = [0; 27]; // 创建一个空的 mln_u8_t 数组
    let mut lower: [mln_u8_t; 27] = [0; 27]; // 创建另一个空的 mln_u8_t 数组

    // 将字节数据复制到数组中
    upper.copy_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0");
    lower.copy_from_slice(b"abcdefghijklmnopqrstuvwxyz\0");

    //let mut upper: [mln_u8_t; 27] = *::core::mem::transmute::<
        //&[u8; 27],
        //&mut [mln_u8_t; 27],
    //>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0");
    //let mut lower: [mln_u8_t; 27] = *::core::mem::transmute::<
        //&[u8; 27],
        //&mut [mln_u8_t; 27],
    //>(b"abcdefghijklmnopqrstuvwxyz\0");
    let mut i: mln_size_t = 0;
    i = 0 as libc::c_int as mln_size_t;
    while i
        < (::core::mem::size_of::<[mln_u8_t; 27]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        chars[upper[i as usize] as usize] = lower[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < (*s).len {
        if chars[*((*s).data).offset(i as isize) as usize] != 0 {
            *((*s).data)
                .offset(i as isize) = chars[*((*s).data).offset(i as isize) as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
}
