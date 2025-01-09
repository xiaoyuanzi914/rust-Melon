use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_s8_t = libc::c_char;
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s8ptr_t = *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_sha1_t {
    pub H0: mln_u32_t,
    pub H1: mln_u32_t,
    pub H2: mln_u32_t,
    pub H3: mln_u32_t,
    pub H4: mln_u32_t,
    pub length: mln_u64_t,
    pub pos: mln_u32_t,
    pub buf: [mln_u8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_sha256_t {
    pub H0: mln_u32_t,
    pub H1: mln_u32_t,
    pub H2: mln_u32_t,
    pub H3: mln_u32_t,
    pub H4: mln_u32_t,
    pub H5: mln_u32_t,
    pub H6: mln_u32_t,
    pub H7: mln_u32_t,
    pub length: mln_u64_t,
    pub pos: mln_u32_t,
    pub buf: [mln_u8_t; 64],
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
                    current_block = 10752631921686271448;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                10752631921686271448 => {}
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
        current_block_8 = 18278753667732557088;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 18278753667732557088;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        18278753667732557088 => {
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
static mut k: [mln_u32_t; 4] = [
    0x5a827999 as libc::c_int as mln_u32_t,
    0x6ed9eba1 as libc::c_int as mln_u32_t,
    0x8f1bbcdc as libc::c_uint,
    0xca62c1d6 as libc::c_uint,
];
#[inline]
unsafe extern "C" fn mln_sha_hex_tostring(mut c: mln_u8_t) -> mln_s8_t {
    return (if (c as libc::c_int) < 10 as libc::c_int {
        '0' as i32 + c as libc::c_int
    } else {
        'a' as i32 + (c as libc::c_int - 10 as libc::c_int)
    }) as mln_s8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_init(mut s: *mut mln_sha1_t) {
    (*s).H0 = 0x67452301 as libc::c_int as mln_u32_t;
    (*s).H1 = 0xefcdab89 as libc::c_uint;
    (*s).H2 = 0x98badcfe as libc::c_uint;
    (*s).H3 = 0x10325476 as libc::c_int as mln_u32_t;
    (*s).H4 = 0xc3d2e1f0 as libc::c_uint;
    (*s).length = 0 as libc::c_int as mln_u64_t;
    (*s).pos = 0 as libc::c_int as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_new() -> *mut mln_sha1_t {
    let mut s: *mut mln_sha1_t = malloc(
        ::core::mem::size_of::<mln_sha1_t>() as libc::c_ulong,
    ) as *mut mln_sha1_t;
    if s.is_null() {
        return 0 as *mut mln_sha1_t;
    }
    mln_sha1_init(s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_pool_new(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_sha1_t {
    let mut s: *mut mln_sha1_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_sha1_t>() as libc::c_ulong,
    ) as *mut mln_sha1_t;
    if s.is_null() {
        return 0 as *mut mln_sha1_t;
    }
    mln_sha1_init(s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_free(mut s: *mut mln_sha1_t) {
    if s.is_null() {
        return;
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_pool_free(mut s: *mut mln_sha1_t) {
    if s.is_null() {
        return;
    }
    mln_alloc_free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_calc(
    mut s: *mut mln_sha1_t,
    mut input: mln_u8ptr_t,
    mut len: mln_uauto_t,
    mut is_last: mln_u32_t,
) {
    let mut size: mln_uauto_t = 0;
    (*s)
        .length = ((*s).length as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    while len.wrapping_add((*s).pos as libc::c_ulong)
        > 64 as libc::c_int as libc::c_ulong
    {
        size = (64 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos) as mln_uauto_t;
        memcpy(
            &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            size,
        );
        len = (len as libc::c_ulong).wrapping_sub(size) as mln_uauto_t as mln_uauto_t;
        input = input.offset(size as isize);
        mln_sha1_calc_block(s);
        (*s).pos = 0 as libc::c_int as mln_u32_t;
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            len,
        );
        (*s)
            .pos = ((*s).pos as libc::c_ulong).wrapping_add(len) as mln_u32_t
            as mln_u32_t;
    }
    if is_last != 0 {
        if (*s).pos < 56 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (56 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos)
                    as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        } else if (*s).pos < 64 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (64 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos)
                    as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
            mln_sha1_calc_block(s);
            (*s).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*s).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
        } else {
            mln_sha1_calc_block(s);
            (*s).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*s).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        }
        (*s).length <<= 3 as libc::c_int;
        (*s)
            .buf[56 as libc::c_int
            as usize] = ((*s).length >> 56 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[57 as libc::c_int
            as usize] = ((*s).length >> 48 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[58 as libc::c_int
            as usize] = ((*s).length >> 40 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[59 as libc::c_int
            as usize] = ((*s).length >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[60 as libc::c_int
            as usize] = ((*s).length >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[61 as libc::c_int
            as usize] = ((*s).length >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[62 as libc::c_int
            as usize] = ((*s).length >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[63 as libc::c_int
            as usize] = ((*s).length & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        mln_sha1_calc_block(s);
        (*s).pos = 0 as libc::c_int as mln_u32_t;
    }
}
#[inline]
unsafe extern "C" fn mln_sha1_calc_block(mut s: *mut mln_sha1_t) {
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut j: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut group: [mln_u32_t; 16] = [0; 16];
    let mut a: mln_u32_t = (*s).H0;
    let mut b: mln_u32_t = (*s).H1;
    let mut c: mln_u32_t = (*s).H2;
    let mut d: mln_u32_t = (*s).H3;
    let mut e: mln_u32_t = (*s).H4;
    while i < 64 as libc::c_int as libc::c_uint {
        group[j as usize] = 0 as libc::c_int as mln_u32_t;
        let fresh1 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= (((*s).buf[fresh1 as usize] as libc::c_int & 0xff as libc::c_int)
                << 24 as libc::c_int) as libc::c_uint;
        let fresh2 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= (((*s).buf[fresh2 as usize] as libc::c_int & 0xff as libc::c_int)
                << 16 as libc::c_int) as libc::c_uint;
        let fresh3 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= (((*s).buf[fresh3 as usize] as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int) as libc::c_uint;
        let fresh4 = i;
        i = i.wrapping_add(1);
        let fresh5 = j;
        j = j.wrapping_add(1);
        group[fresh5 as usize]
            |= ((*s).buf[fresh4 as usize] as libc::c_int & 0xff as libc::c_int)
                as libc::c_uint;
    }
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | !b & d)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[0 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | !a & c)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[1 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | !e & b)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[2 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | !d & a)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[3 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | !c & e)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[4 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | !b & d)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[5 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | !a & c)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[6 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | !e & b)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[7 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | !d & a)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[8 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | !c & e)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[9 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | !b & d)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[10 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | !a & c)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[11 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | !e & b)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[12 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | !d & a)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[13 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | !c & e)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[14 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | !b & d)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[15 as libc::c_int as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(16 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(16 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(16 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(16 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(16 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(16 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(16 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | !a & c)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[(16 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(17 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(17 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(17 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(17 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(17 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(17 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(17 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | !e & b)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[(17 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(18 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(18 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(18 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(18 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(18 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(18 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(18 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | !d & a)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[(18 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(19 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(19 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(19 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(19 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(19 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(19 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(19 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | !c & e)
                .wrapping_add(k[0 as libc::c_int as usize])
                .wrapping_add(group[(19 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(20 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(20 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(20 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(20 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(20 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(20 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(20 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(20 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(21 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(21 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(21 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(21 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(21 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(21 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(21 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(21 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(22 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(22 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(22 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(22 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(22 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(22 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(22 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(22 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(23 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(23 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(23 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(23 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(23 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(23 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(23 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(23 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(24 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(24 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(24 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(24 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(24 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(24 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(24 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(24 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(25 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(25 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(25 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(25 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(25 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(25 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(25 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(25 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(26 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(26 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(26 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(26 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(26 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(26 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(26 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(26 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(27 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(27 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(27 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(27 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(27 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(27 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(27 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(27 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(28 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(28 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(28 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(28 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(28 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(28 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(28 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(28 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(29 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(29 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(29 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(29 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(29 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(29 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(29 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(29 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(30 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(30 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(30 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(30 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(30 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(30 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(30 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(30 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(31 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(31 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(31 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(31 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(31 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(31 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(31 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(31 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(32 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(32 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(32 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(32 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(32 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(32 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(32 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(32 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(33 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(33 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(33 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(33 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(33 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(33 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(33 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(33 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(34 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(34 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(34 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(34 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(34 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(34 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(34 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(34 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(35 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(35 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(35 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(35 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(35 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(35 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(35 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(35 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(36 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(36 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(36 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(36 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(36 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(36 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(36 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(36 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(37 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(37 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(37 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(37 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(37 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(37 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(37 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(37 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(38 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(38 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(38 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(38 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(38 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(38 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(38 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(38 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(39 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(39 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(39 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(39 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(39 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(39 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(39 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[1 as libc::c_int as usize])
                .wrapping_add(group[(39 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(40 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(40 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(40 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(40 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(40 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(40 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(40 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | d & (b | c))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(40 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(41 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(41 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(41 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(41 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(41 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(41 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(41 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | c & (a | b))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(41 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(42 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(42 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(42 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(42 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(42 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(42 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(42 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | b & (e | a))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(42 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(43 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(43 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(43 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(43 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(43 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(43 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(43 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | a & (d | e))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(43 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(44 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(44 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(44 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(44 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(44 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(44 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(44 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | e & (c | d))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(44 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(45 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(45 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(45 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(45 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(45 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(45 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(45 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | d & (b | c))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(45 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(46 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(46 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(46 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(46 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(46 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(46 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(46 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | c & (a | b))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(46 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(47 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(47 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(47 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(47 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(47 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(47 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(47 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | b & (e | a))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(47 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(48 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(48 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(48 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(48 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(48 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(48 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(48 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | a & (d | e))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(48 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(49 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(49 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(49 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(49 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(49 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(49 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(49 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | e & (c | d))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(49 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(50 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(50 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(50 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(50 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(50 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(50 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(50 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | d & (b | c))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(50 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(51 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(51 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(51 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(51 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(51 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(51 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(51 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | c & (a | b))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(51 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(52 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(52 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(52 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(52 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(52 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(52 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(52 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | b & (e | a))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(52 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(53 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(53 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(53 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(53 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(53 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(53 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(53 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | a & (d | e))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(53 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(54 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(54 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(54 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(54 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(54 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(54 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(54 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | e & (c | d))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(54 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(55 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(55 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(55 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(55 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(55 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(55 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(55 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b & c | d & (b | c))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(55 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(56 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(56 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(56 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(56 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(56 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(56 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(56 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a & b | c & (a | b))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(56 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(57 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(57 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(57 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(57 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(57 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(57 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(57 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e & a | b & (e | a))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(57 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(58 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(58 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(58 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(58 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(58 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(58 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(58 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d & e | a & (d | e))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(58 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(59 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(59 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(59 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(59 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(59 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(59 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(59 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c & d | e & (c | d))
                .wrapping_add(k[2 as libc::c_int as usize])
                .wrapping_add(group[(59 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(60 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(60 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(60 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(60 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(60 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(60 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(60 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(60 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(61 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(61 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(61 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(61 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(61 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(61 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(61 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(61 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(62 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(62 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(62 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(62 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(62 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(62 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(62 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(62 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(63 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(63 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(63 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(63 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(63 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(63 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(63 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(63 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(64 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(64 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(64 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(64 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(64 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(64 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(64 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(64 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(65 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(65 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(65 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(65 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(65 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(65 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(65 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(65 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(66 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(66 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(66 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(66 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(66 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(66 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(66 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(66 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(67 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(67 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(67 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(67 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(67 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(67 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(67 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(67 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(68 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(68 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(68 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(68 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(68 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(68 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(68 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(68 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(68 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(68 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(69 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(69 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(69 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(69 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(69 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(69 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(69 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(69 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(69 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(69 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(70 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(70 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(70 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(70 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(70 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(70 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(70 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(70 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(70 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(70 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(71 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(71 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(71 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(71 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(71 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(71 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(71 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(71 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(71 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(71 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(72 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(72 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(72 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(72 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(72 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(72 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(72 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(72 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(72 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(72 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(73 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(73 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(73 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(73 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(73 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(73 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(73 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(73 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(73 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(73 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(74 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(74 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(74 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(74 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(74 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(74 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(74 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(74 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(74 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(74 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(75 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(75 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(75 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(75 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(75 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(75 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(75 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(75 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(75 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    e = (e as libc::c_uint)
        .wrapping_add(
            (a << 5 as libc::c_int
                | (a & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(75 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    b = b << 30 as libc::c_int
        | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(76 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(76 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(76 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(76 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(76 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(76 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(76 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(76 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(76 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (e << 5 as libc::c_int
                | (e & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(76 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    a = a << 30 as libc::c_int
        | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(77 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(77 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(77 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(77 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(77 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(77 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(77 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(77 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(77 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d << 5 as libc::c_int
                | (d & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(e ^ a ^ b)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(77 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    e = e << 30 as libc::c_int
        | (e & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(78 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(78 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(78 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(78 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(78 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(78 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(78 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(78 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(78 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c << 5 as libc::c_int
                | (c & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(d ^ e ^ a)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(78 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    d = d << 30 as libc::c_int
        | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    group[(79 as libc::c_int & 0xf as libc::c_int)
        as usize] = (group[(79 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
        as usize]
        ^ group[(79 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(79 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ group[(79 as libc::c_int & 0xf as libc::c_int) as usize]) << 1 as libc::c_int
        | ((group[(79 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(79 as libc::c_int - 8 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ group[(79 as libc::c_int - 14 as libc::c_int & 0xf as libc::c_int)
                as usize] ^ group[(79 as libc::c_int & 0xf as libc::c_int) as usize])
            & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 1 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b << 5 as libc::c_int
                | (b & 0xffffffff as libc::c_uint)
                    >> 32 as libc::c_int - 5 as libc::c_int)
                .wrapping_add(c ^ d ^ e)
                .wrapping_add(k[3 as libc::c_int as usize])
                .wrapping_add(group[(79 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as mln_u32_t as mln_u32_t;
    c = c << 30 as libc::c_int
        | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 30 as libc::c_int;
    (*s).H0 = ((*s).H0 as libc::c_uint).wrapping_add(a) as mln_u32_t as mln_u32_t;
    (*s).H1 = ((*s).H1 as libc::c_uint).wrapping_add(b) as mln_u32_t as mln_u32_t;
    (*s).H2 = ((*s).H2 as libc::c_uint).wrapping_add(c) as mln_u32_t as mln_u32_t;
    (*s).H3 = ((*s).H3 as libc::c_uint).wrapping_add(d) as mln_u32_t as mln_u32_t;
    (*s).H4 = ((*s).H4 as libc::c_uint).wrapping_add(e) as mln_u32_t as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_tobytes(
    mut s: *mut mln_sha1_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_u32_t,
) {
    if len == 0 as libc::c_int as libc::c_uint || buf.is_null() {
        return;
    }
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let fresh6 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh6 as isize,
        ) = ((*s).H0 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh7 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh7 as isize,
        ) = ((*s).H0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh8 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh8 as isize,
        ) = ((*s).H0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh9 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh9 as isize,
        ) = ((*s).H0 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh10 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh10 as isize,
        ) = ((*s).H1 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh11 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh11 as isize,
        ) = ((*s).H1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh12 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh12 as isize,
        ) = ((*s).H1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh13 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh13 as isize,
        ) = ((*s).H1 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh14 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh14 as isize,
        ) = ((*s).H2 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh15 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh15 as isize,
        ) = ((*s).H2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh16 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh16 as isize,
        ) = ((*s).H2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh17 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh17 as isize,
        ) = ((*s).H2 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh18 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh18 as isize,
        ) = ((*s).H3 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh19 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh19 as isize,
        ) = ((*s).H3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh20 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh20 as isize,
        ) = ((*s).H3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh21 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh21 as isize,
        ) = ((*s).H3 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh22 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh22 as isize,
        ) = ((*s).H4 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh23 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh23 as isize,
        ) = ((*s).H4 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh24 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh24 as isize,
        ) = ((*s).H4 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh25 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh25 as isize,
        ) = ((*s).H4 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_tostring(
    mut s: *mut mln_sha1_t,
    mut buf: mln_s8ptr_t,
    mut len: mln_u32_t,
) {
    if buf.is_null() || len == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut i: mln_u32_t = 0;
    let mut bytes: [mln_u8_t; 20] = [
        0 as libc::c_int as mln_u8_t,
    ];
    mln_sha1_tobytes(
        s,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong as mln_u32_t,
    );
    len = (if len as libc::c_ulong
        > (::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong) << 1 as libc::c_int
    {
        ::core::mem::size_of::<[mln_u8_t; 20]>() as libc::c_ulong
    } else {
        (len.wrapping_sub(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int)
            as libc::c_ulong
    }) as mln_u32_t;
    i = 0 as libc::c_int as mln_u32_t;
    while i < len {
        let fresh26 = buf;
        buf = buf.offset(1);
        *fresh26 = mln_sha_hex_tostring(
            (bytes[i as usize] as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                as mln_u8_t,
        );
        let fresh27 = buf;
        buf = buf.offset(1);
        *fresh27 = mln_sha_hex_tostring(
            (bytes[i as usize] as libc::c_int & 0xf as libc::c_int) as mln_u8_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    *buf = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha1_dump(mut s: *mut mln_sha1_t) {
    printf(
        b"%lx %lx %lx %lx %lx\n\0" as *const u8 as *const libc::c_char,
        (*s).H0 as libc::c_ulong,
        (*s).H1 as libc::c_ulong,
        (*s).H2 as libc::c_ulong,
        (*s).H3 as libc::c_ulong,
        (*s).H4 as libc::c_ulong,
    );
}
static mut sha256_round_constant: [mln_u32_t; 64] = [
    0x428a2f98 as libc::c_int as mln_u32_t,
    0x71374491 as libc::c_int as mln_u32_t,
    0xb5c0fbcf as libc::c_uint,
    0xe9b5dba5 as libc::c_uint,
    0x3956c25b as libc::c_int as mln_u32_t,
    0x59f111f1 as libc::c_int as mln_u32_t,
    0x923f82a4 as libc::c_uint,
    0xab1c5ed5 as libc::c_uint,
    0xd807aa98 as libc::c_uint,
    0x12835b01 as libc::c_int as mln_u32_t,
    0x243185be as libc::c_int as mln_u32_t,
    0x550c7dc3 as libc::c_int as mln_u32_t,
    0x72be5d74 as libc::c_int as mln_u32_t,
    0x80deb1fe as libc::c_uint,
    0x9bdc06a7 as libc::c_uint,
    0xc19bf174 as libc::c_uint,
    0xe49b69c1 as libc::c_uint,
    0xefbe4786 as libc::c_uint,
    0xfc19dc6 as libc::c_int as mln_u32_t,
    0x240ca1cc as libc::c_int as mln_u32_t,
    0x2de92c6f as libc::c_int as mln_u32_t,
    0x4a7484aa as libc::c_int as mln_u32_t,
    0x5cb0a9dc as libc::c_int as mln_u32_t,
    0x76f988da as libc::c_int as mln_u32_t,
    0x983e5152 as libc::c_uint,
    0xa831c66d as libc::c_uint,
    0xb00327c8 as libc::c_uint,
    0xbf597fc7 as libc::c_uint,
    0xc6e00bf3 as libc::c_uint,
    0xd5a79147 as libc::c_uint,
    0x6ca6351 as libc::c_int as mln_u32_t,
    0x14292967 as libc::c_int as mln_u32_t,
    0x27b70a85 as libc::c_int as mln_u32_t,
    0x2e1b2138 as libc::c_int as mln_u32_t,
    0x4d2c6dfc as libc::c_int as mln_u32_t,
    0x53380d13 as libc::c_int as mln_u32_t,
    0x650a7354 as libc::c_int as mln_u32_t,
    0x766a0abb as libc::c_int as mln_u32_t,
    0x81c2c92e as libc::c_uint,
    0x92722c85 as libc::c_uint,
    0xa2bfe8a1 as libc::c_uint,
    0xa81a664b as libc::c_uint,
    0xc24b8b70 as libc::c_uint,
    0xc76c51a3 as libc::c_uint,
    0xd192e819 as libc::c_uint,
    0xd6990624 as libc::c_uint,
    0xf40e3585 as libc::c_uint,
    0x106aa070 as libc::c_int as mln_u32_t,
    0x19a4c116 as libc::c_int as mln_u32_t,
    0x1e376c08 as libc::c_int as mln_u32_t,
    0x2748774c as libc::c_int as mln_u32_t,
    0x34b0bcb5 as libc::c_int as mln_u32_t,
    0x391c0cb3 as libc::c_int as mln_u32_t,
    0x4ed8aa4a as libc::c_int as mln_u32_t,
    0x5b9cca4f as libc::c_int as mln_u32_t,
    0x682e6ff3 as libc::c_int as mln_u32_t,
    0x748f82ee as libc::c_int as mln_u32_t,
    0x78a5636f as libc::c_int as mln_u32_t,
    0x84c87814 as libc::c_uint,
    0x8cc70208 as libc::c_uint,
    0x90befffa as libc::c_uint,
    0xa4506ceb as libc::c_uint,
    0xbef9a3f7 as libc::c_uint,
    0xc67178f2 as libc::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_init(mut s: *mut mln_sha256_t) {
    (*s).H0 = 0x6a09e667 as libc::c_int as mln_u32_t;
    (*s).H1 = 0xbb67ae85 as libc::c_uint;
    (*s).H2 = 0x3c6ef372 as libc::c_int as mln_u32_t;
    (*s).H3 = 0xa54ff53a as libc::c_uint;
    (*s).H4 = 0x510e527f as libc::c_int as mln_u32_t;
    (*s).H5 = 0x9b05688c as libc::c_uint;
    (*s).H6 = 0x1f83d9ab as libc::c_int as mln_u32_t;
    (*s).H7 = 0x5be0cd19 as libc::c_int as mln_u32_t;
    (*s).length = 0 as libc::c_int as mln_u64_t;
    (*s).pos = 0 as libc::c_int as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_new() -> *mut mln_sha256_t {
    let mut s: *mut mln_sha256_t = malloc(
        ::core::mem::size_of::<mln_sha256_t>() as libc::c_ulong,
    ) as *mut mln_sha256_t;
    if s.is_null() {
        return 0 as *mut mln_sha256_t;
    }
    mln_sha256_init(s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_pool_new(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_sha256_t {
    let mut s: *mut mln_sha256_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_sha256_t>() as libc::c_ulong,
    ) as *mut mln_sha256_t;
    if s.is_null() {
        return 0 as *mut mln_sha256_t;
    }
    mln_sha256_init(s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_free(mut s: *mut mln_sha256_t) {
    if s.is_null() {
        return;
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_pool_free(mut s: *mut mln_sha256_t) {
    if s.is_null() {
        return;
    }
    mln_alloc_free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_calc(
    mut s: *mut mln_sha256_t,
    mut input: mln_u8ptr_t,
    mut len: mln_uauto_t,
    mut is_last: mln_u32_t,
) {
    let mut size: mln_uauto_t = 0;
    (*s)
        .length = ((*s).length as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    while len.wrapping_add((*s).pos as libc::c_ulong)
        > 64 as libc::c_int as libc::c_ulong
    {
        size = (64 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos) as mln_uauto_t;
        memcpy(
            &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            size,
        );
        len = (len as libc::c_ulong).wrapping_sub(size) as mln_uauto_t as mln_uauto_t;
        input = input.offset(size as isize);
        mln_sha256_calc_block(s);
        (*s).pos = 0 as libc::c_int as mln_u32_t;
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            len,
        );
        (*s)
            .pos = ((*s).pos as libc::c_ulong).wrapping_add(len) as mln_u32_t
            as mln_u32_t;
    }
    if is_last != 0 {
        if (*s).pos < 56 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (56 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos)
                    as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        } else if (*s).pos < 64 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*s).buf).as_mut_ptr().offset((*s).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (64 as libc::c_int as libc::c_uint).wrapping_sub((*s).pos)
                    as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
            mln_sha256_calc_block(s);
            (*s).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*s).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
        } else {
            mln_sha256_calc_block(s);
            (*s).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*s).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
            (*s)
                .buf[(*s).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        }
        (*s).length <<= 3 as libc::c_int;
        (*s)
            .buf[56 as libc::c_int
            as usize] = ((*s).length >> 56 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[57 as libc::c_int
            as usize] = ((*s).length >> 48 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[58 as libc::c_int
            as usize] = ((*s).length >> 40 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[59 as libc::c_int
            as usize] = ((*s).length >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[60 as libc::c_int
            as usize] = ((*s).length >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[61 as libc::c_int
            as usize] = ((*s).length >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[62 as libc::c_int
            as usize] = ((*s).length >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*s)
            .buf[63 as libc::c_int
            as usize] = ((*s).length & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        mln_sha256_calc_block(s);
        (*s).pos = 0 as libc::c_int as mln_u32_t;
    }
}
#[inline]
unsafe extern "C" fn mln_sha256_safe_add(
    mut x: mln_u32_t,
    mut y: mln_u32_t,
) -> mln_u32_t {
    let mut lsw: mln_u32_t = (x & 0xffff as libc::c_int as libc::c_uint)
        .wrapping_add(y & 0xffff as libc::c_int as libc::c_uint);
    let mut msw: mln_u32_t = (x >> 16 as libc::c_int)
        .wrapping_add(y >> 16 as libc::c_int)
        .wrapping_add(lsw >> 16 as libc::c_int);
    return msw << 16 as libc::c_int | lsw & 0xffff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn mln_sha256_calc_block(mut s: *mut mln_sha256_t) {
    let mut h0: mln_u32_t = 0;
    let mut h1: mln_u32_t = 0;
    let mut h2: mln_u32_t = 0;
    let mut h3: mln_u32_t = 0;
    let mut h4: mln_u32_t = 0;
    let mut h5: mln_u32_t = 0;
    let mut h6: mln_u32_t = 0;
    let mut h7: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut t1: mln_u32_t = 0;
    let mut t2: mln_u32_t = 0;
    let mut group: [mln_u32_t; 64] = [
        0 as libc::c_int as mln_u32_t,
    ];
    j = 0 as libc::c_int as mln_u32_t;
    while j < 64 as libc::c_int as libc::c_uint {
        group[(j >> 2 as libc::c_int) as usize]
            |= (((*s).buf[j as usize] as libc::c_int)
                << ((3 as libc::c_int as libc::c_uint)
                    .wrapping_sub(j.wrapping_rem(4 as libc::c_int as libc::c_uint))
                    << 3 as libc::c_int)) as libc::c_uint;
        j = j.wrapping_add(1);
        j;
    }
    h0 = (*s).H0;
    h1 = (*s).H1;
    h2 = (*s).H2;
    h3 = (*s).H3;
    h4 = (*s).H4;
    h5 = (*s).H5;
    h6 = (*s).H6;
    h7 = (*s).H7;
    j = 0 as libc::c_int as mln_u32_t;
    while j < 64 as libc::c_int as libc::c_uint {
        if j >= 16 as libc::c_int as libc::c_uint {
            group[j
                as usize] = mln_sha256_safe_add(
                mln_sha256_safe_add(
                    mln_sha256_safe_add(
                        (group[j.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                            >> 17 as libc::c_int
                            | group[j.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                as usize] << 32 as libc::c_int - 17 as libc::c_int)
                            ^ (group[j.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                as usize] >> 19 as libc::c_int
                                | group[j.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                    as usize] << 32 as libc::c_int - 19 as libc::c_int)
                            ^ group[j.wrapping_sub(2 as libc::c_int as libc::c_uint)
                                as usize] >> 10 as libc::c_int,
                        group[j.wrapping_sub(7 as libc::c_int as libc::c_uint) as usize],
                    ),
                    (group[j.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                        >> 7 as libc::c_int
                        | group[j.wrapping_sub(15 as libc::c_int as libc::c_uint)
                            as usize] << 32 as libc::c_int - 7 as libc::c_int)
                        ^ (group[j.wrapping_sub(15 as libc::c_int as libc::c_uint)
                            as usize] >> 18 as libc::c_int
                            | group[j.wrapping_sub(15 as libc::c_int as libc::c_uint)
                                as usize] << 32 as libc::c_int - 18 as libc::c_int)
                        ^ group[j.wrapping_sub(15 as libc::c_int as libc::c_uint)
                            as usize] >> 3 as libc::c_int,
                ),
                group[j.wrapping_sub(16 as libc::c_int as libc::c_uint) as usize],
            );
        }
        t1 = mln_sha256_safe_add(
            mln_sha256_safe_add(
                mln_sha256_safe_add(
                    mln_sha256_safe_add(
                        h7,
                        (h4 >> 6 as libc::c_int
                            | h4 << 32 as libc::c_int - 6 as libc::c_int)
                            ^ (h4 >> 11 as libc::c_int
                                | h4 << 32 as libc::c_int - 11 as libc::c_int)
                            ^ (h4 >> 25 as libc::c_int
                                | h4 << 32 as libc::c_int - 25 as libc::c_int),
                    ),
                    h4 & h5 ^ !h4 & h6,
                ),
                sha256_round_constant[j as usize],
            ),
            group[j as usize],
        );
        t2 = mln_sha256_safe_add(
            (h0 >> 2 as libc::c_int | h0 << 32 as libc::c_int - 2 as libc::c_int)
                ^ (h0 >> 13 as libc::c_int | h0 << 32 as libc::c_int - 13 as libc::c_int)
                ^ (h0 >> 22 as libc::c_int
                    | h0 << 32 as libc::c_int - 22 as libc::c_int),
            h0 & h1 ^ h0 & h2 ^ h1 & h2,
        );
        h7 = h6;
        h6 = h5;
        h5 = h4;
        h4 = mln_sha256_safe_add(h3, t1);
        h3 = h2;
        h2 = h1;
        h1 = h0;
        h0 = mln_sha256_safe_add(t1, t2);
        j = j.wrapping_add(1);
        j;
    }
    (*s).H0 = mln_sha256_safe_add(h0, (*s).H0);
    (*s).H1 = mln_sha256_safe_add(h1, (*s).H1);
    (*s).H2 = mln_sha256_safe_add(h2, (*s).H2);
    (*s).H3 = mln_sha256_safe_add(h3, (*s).H3);
    (*s).H4 = mln_sha256_safe_add(h4, (*s).H4);
    (*s).H5 = mln_sha256_safe_add(h5, (*s).H5);
    (*s).H6 = mln_sha256_safe_add(h6, (*s).H6);
    (*s).H7 = mln_sha256_safe_add(h7, (*s).H7);
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_tobytes(
    mut s: *mut mln_sha256_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_u32_t,
) {
    if len == 0 as libc::c_int as libc::c_uint || buf.is_null() {
        return;
    }
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let fresh28 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh28 as isize,
        ) = ((*s).H0 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh29 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh29 as isize,
        ) = ((*s).H0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh30 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh30 as isize,
        ) = ((*s).H0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh31 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh31 as isize,
        ) = ((*s).H0 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh32 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh32 as isize,
        ) = ((*s).H1 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh33 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh33 as isize,
        ) = ((*s).H1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh34 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh34 as isize,
        ) = ((*s).H1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh35 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh35 as isize,
        ) = ((*s).H1 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh36 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh36 as isize,
        ) = ((*s).H2 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh37 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh37 as isize,
        ) = ((*s).H2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh38 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh38 as isize,
        ) = ((*s).H2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh39 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh39 as isize,
        ) = ((*s).H2 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh40 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh40 as isize,
        ) = ((*s).H3 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh41 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh41 as isize,
        ) = ((*s).H3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh42 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh42 as isize,
        ) = ((*s).H3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh43 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh43 as isize,
        ) = ((*s).H3 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh44 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh44 as isize,
        ) = ((*s).H4 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh45 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh45 as isize,
        ) = ((*s).H4 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh46 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh46 as isize,
        ) = ((*s).H4 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh47 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh47 as isize,
        ) = ((*s).H4 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    let fresh48 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh48 as isize,
        ) = ((*s).H5 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh49 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh49 as isize,
        ) = ((*s).H5 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh50 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh50 as isize,
        ) = ((*s).H5 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh51 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh51 as isize,
        ) = ((*s).H5 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    let fresh52 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh52 as isize,
        ) = ((*s).H6 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh53 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh53 as isize,
        ) = ((*s).H6 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh54 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh54 as isize,
        ) = ((*s).H6 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh55 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh55 as isize,
        ) = ((*s).H6 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    let fresh56 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh56 as isize,
        ) = ((*s).H7 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh57 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh57 as isize,
        ) = ((*s).H7 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh58 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh58 as isize,
        ) = ((*s).H7 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh59 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh59 as isize,
        ) = ((*s).H7 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_tostring(
    mut s: *mut mln_sha256_t,
    mut buf: mln_s8ptr_t,
    mut len: mln_u32_t,
) {
    if buf.is_null() || len == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut i: mln_u32_t = 0;
    let mut bytes: [mln_u8_t; 32] = [
        0 as libc::c_int as mln_u8_t,
    ];
    mln_sha256_tobytes(
        s,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 32]>() as libc::c_ulong as mln_u32_t,
    );
    len = (if len as libc::c_ulong
        > (::core::mem::size_of::<[mln_u8_t; 32]>() as libc::c_ulong) << 1 as libc::c_int
    {
        ::core::mem::size_of::<[mln_u8_t; 32]>() as libc::c_ulong
    } else {
        (len.wrapping_sub(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int)
            as libc::c_ulong
    }) as mln_u32_t;
    i = 0 as libc::c_int as mln_u32_t;
    while i < len {
        let fresh60 = buf;
        buf = buf.offset(1);
        *fresh60 = mln_sha_hex_tostring(
            (bytes[i as usize] as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                as mln_u8_t,
        );
        let fresh61 = buf;
        buf = buf.offset(1);
        *fresh61 = mln_sha_hex_tostring(
            (bytes[i as usize] as libc::c_int & 0xf as libc::c_int) as mln_u8_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    *buf = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mln_sha256_dump(mut s: *mut mln_sha256_t) {
    printf(
        b"%lx %lx %lx %lx %lx %lx %lx %lx\n\0" as *const u8 as *const libc::c_char,
        (*s).H0 as libc::c_ulong,
        (*s).H1 as libc::c_ulong,
        (*s).H2 as libc::c_ulong,
        (*s).H3 as libc::c_ulong,
        (*s).H4 as libc::c_ulong,
        (*s).H5 as libc::c_ulong,
        (*s).H6 as libc::c_ulong,
        (*s).H7 as libc::c_ulong,
    );
}
