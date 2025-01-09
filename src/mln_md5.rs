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
pub struct mln_md5_t {
    pub A: mln_u32_t,
    pub B: mln_u32_t,
    pub C: mln_u32_t,
    pub D: mln_u32_t,
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
    let mut s_0: libc::c_int = -(1 as libc::c_int);
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
                s_0 = -(1 as libc::c_int);
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
                        s_0 = i;
                        save = p;
                    }
                    j -= 1;
                    if j <= 0 as libc::c_int {
                        break;
                    }
                } else if !save.is_null() {
                    j = -(1 as libc::c_int);
                    s_0 = -(1 as libc::c_int);
                    save = 0 as mln_u8ptr_t;
                }
                i -= 1;
                i;
            }
            if !save.is_null() && j == 0 {
                *Boff = save.offset_from(((*as_0).bitmap).as_mut_ptr()) as libc::c_long;
                *boff = s_0 as mln_off_t;
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
static mut s: [[mln_u32_t; 4]; 4] = [
    [
        7 as libc::c_int as mln_u32_t,
        12 as libc::c_int as mln_u32_t,
        17 as libc::c_int as mln_u32_t,
        22 as libc::c_int as mln_u32_t,
    ],
    [
        5 as libc::c_int as mln_u32_t,
        9 as libc::c_int as mln_u32_t,
        14 as libc::c_int as mln_u32_t,
        20 as libc::c_int as mln_u32_t,
    ],
    [
        4 as libc::c_int as mln_u32_t,
        11 as libc::c_int as mln_u32_t,
        16 as libc::c_int as mln_u32_t,
        23 as libc::c_int as mln_u32_t,
    ],
    [
        6 as libc::c_int as mln_u32_t,
        10 as libc::c_int as mln_u32_t,
        15 as libc::c_int as mln_u32_t,
        21 as libc::c_int as mln_u32_t,
    ],
];
static mut ti: [[mln_u32_t; 16]; 4] = [
    [
        0xd76aa478 as libc::c_uint,
        0xe8c7b756 as libc::c_uint,
        0x242070db as libc::c_int as mln_u32_t,
        0xc1bdceee as libc::c_uint,
        0xf57c0faf as libc::c_uint,
        0x4787c62a as libc::c_int as mln_u32_t,
        0xa8304613 as libc::c_uint,
        0xfd469501 as libc::c_uint,
        0x698098d8 as libc::c_int as mln_u32_t,
        0x8b44f7af as libc::c_uint,
        0xffff5bb1 as libc::c_uint,
        0x895cd7be as libc::c_uint,
        0x6b901122 as libc::c_int as mln_u32_t,
        0xfd987193 as libc::c_uint,
        0xa679438e as libc::c_uint,
        0x49b40821 as libc::c_int as mln_u32_t,
    ],
    [
        0xf61e2562 as libc::c_uint,
        0xc040b340 as libc::c_uint,
        0x265e5a51 as libc::c_int as mln_u32_t,
        0xe9b6c7aa as libc::c_uint,
        0xd62f105d as libc::c_uint,
        0x2441453 as libc::c_int as mln_u32_t,
        0xd8a1e681 as libc::c_uint,
        0xe7d3fbc8 as libc::c_uint,
        0x21e1cde6 as libc::c_int as mln_u32_t,
        0xc33707d6 as libc::c_uint,
        0xf4d50d87 as libc::c_uint,
        0x455a14ed as libc::c_int as mln_u32_t,
        0xa9e3e905 as libc::c_uint,
        0xfcefa3f8 as libc::c_uint,
        0x676f02d9 as libc::c_int as mln_u32_t,
        0x8d2a4c8a as libc::c_uint,
    ],
    [
        0xfffa3942 as libc::c_uint,
        0x8771f681 as libc::c_uint,
        0x6d9d6122 as libc::c_int as mln_u32_t,
        0xfde5380c as libc::c_uint,
        0xa4beea44 as libc::c_uint,
        0x4bdecfa9 as libc::c_int as mln_u32_t,
        0xf6bb4b60 as libc::c_uint,
        0xbebfbc70 as libc::c_uint,
        0x289b7ec6 as libc::c_int as mln_u32_t,
        0xeaa127fa as libc::c_uint,
        0xd4ef3085 as libc::c_uint,
        0x4881d05 as libc::c_int as mln_u32_t,
        0xd9d4d039 as libc::c_uint,
        0xe6db99e5 as libc::c_uint,
        0x1fa27cf8 as libc::c_int as mln_u32_t,
        0xc4ac5665 as libc::c_uint,
    ],
    [
        0xf4292244 as libc::c_uint,
        0x432aff97 as libc::c_int as mln_u32_t,
        0xab9423a7 as libc::c_uint,
        0xfc93a039 as libc::c_uint,
        0x655b59c3 as libc::c_int as mln_u32_t,
        0x8f0ccc92 as libc::c_uint,
        0xffeff47d as libc::c_uint,
        0x85845dd1 as libc::c_uint,
        0x6fa87e4f as libc::c_int as mln_u32_t,
        0xfe2ce6e0 as libc::c_uint,
        0xa3014314 as libc::c_uint,
        0x4e0811a1 as libc::c_int as mln_u32_t,
        0xf7537e82 as libc::c_uint,
        0xbd3af235 as libc::c_uint,
        0x2ad7d2bb as libc::c_int as mln_u32_t,
        0xeb86d391 as libc::c_uint,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn mln_md5_init(mut m: *mut mln_md5_t) {
    (*m).A = 0x67452301 as libc::c_int as mln_u32_t;
    (*m).B = 0xefcdab89 as libc::c_uint;
    (*m).C = 0x98badcfe as libc::c_uint;
    (*m).D = 0x10325476 as libc::c_int as mln_u32_t;
    (*m).length = 0 as libc::c_int as mln_u64_t;
    (*m).pos = 0 as libc::c_int as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_new() -> *mut mln_md5_t {
    let mut m: *mut mln_md5_t = malloc(
        ::core::mem::size_of::<mln_md5_t>() as libc::c_ulong,
    ) as *mut mln_md5_t;
    if m.is_null() {
        return m;
    }
    mln_md5_init(m);
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_pool_new(mut pool: *mut mln_alloc_t) -> *mut mln_md5_t {
    let mut m: *mut mln_md5_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_md5_t>() as libc::c_ulong,
    ) as *mut mln_md5_t;
    if m.is_null() {
        return m;
    }
    mln_md5_init(m);
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_free(mut m: *mut mln_md5_t) {
    if m.is_null() {
        return;
    }
    free(m as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_pool_free(mut m: *mut mln_md5_t) {
    if m.is_null() {
        return;
    }
    mln_alloc_free(m as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_calc(
    mut m: *mut mln_md5_t,
    mut input: mln_u8ptr_t,
    mut len: mln_uauto_t,
    mut is_last: mln_u32_t,
) {
    let mut size: mln_uauto_t = 0;
    (*m)
        .length = ((*m).length as libc::c_ulong).wrapping_add(len) as mln_u64_t
        as mln_u64_t;
    while len.wrapping_add((*m).pos as libc::c_ulong)
        > 64 as libc::c_int as libc::c_ulong
    {
        size = (64 as libc::c_int as libc::c_uint).wrapping_sub((*m).pos) as mln_uauto_t;
        memcpy(
            &mut *((*m).buf).as_mut_ptr().offset((*m).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            size,
        );
        len = (len as libc::c_ulong).wrapping_sub(size) as mln_uauto_t as mln_uauto_t;
        input = input.offset(size as isize);
        mln_md5_calc_block(m);
        (*m).pos = 0 as libc::c_int as mln_u32_t;
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            &mut *((*m).buf).as_mut_ptr().offset((*m).pos as isize) as *mut mln_u8_t
                as *mut libc::c_void,
            input as *const libc::c_void,
            len,
        );
        (*m)
            .pos = ((*m).pos as libc::c_ulong).wrapping_add(len) as mln_u32_t
            as mln_u32_t;
    }
    if is_last != 0 {
        if (*m).pos < 56 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*m).buf).as_mut_ptr().offset((*m).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (56 as libc::c_int as libc::c_uint).wrapping_sub((*m).pos)
                    as libc::c_ulong,
            );
            (*m)
                .buf[(*m).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        } else if (*m).pos < 64 as libc::c_int as libc::c_uint {
            memset(
                &mut *((*m).buf).as_mut_ptr().offset((*m).pos as isize) as *mut mln_u8_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                (64 as libc::c_int as libc::c_uint).wrapping_sub((*m).pos)
                    as libc::c_ulong,
            );
            (*m)
                .buf[(*m).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
            mln_md5_calc_block(m);
            (*m).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*m).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
        } else {
            mln_md5_calc_block(m);
            (*m).pos = 0 as libc::c_int as mln_u32_t;
            memset(
                ((*m).buf).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                56 as libc::c_int as libc::c_ulong,
            );
            (*m)
                .buf[(*m).pos
                as usize] = ((1 as libc::c_int) << 7 as libc::c_int) as mln_u8_t;
        }
        (*m).length <<= 3 as libc::c_int;
        (*m)
            .buf[56 as libc::c_int
            as usize] = ((*m).length & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[57 as libc::c_int
            as usize] = ((*m).length >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[58 as libc::c_int
            as usize] = ((*m).length >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[59 as libc::c_int
            as usize] = ((*m).length >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[60 as libc::c_int
            as usize] = ((*m).length >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[61 as libc::c_int
            as usize] = ((*m).length >> 40 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[62 as libc::c_int
            as usize] = ((*m).length >> 48 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        (*m)
            .buf[63 as libc::c_int
            as usize] = ((*m).length >> 56 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        mln_md5_calc_block(m);
        (*m).pos = 0 as libc::c_int as mln_u32_t;
    }
}
#[inline]
unsafe extern "C" fn mln_md5_calc_block(mut m: *mut mln_md5_t) {
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut j: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut group: [mln_u32_t; 16] = [0; 16];
    let mut a: mln_u32_t = (*m).A;
    let mut b: mln_u32_t = (*m).B;
    let mut c: mln_u32_t = (*m).C;
    let mut d: mln_u32_t = (*m).D;
    while i < 64 as libc::c_int as libc::c_uint {
        group[j as usize] = 0 as libc::c_int as mln_u32_t;
        let fresh1 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= ((*m).buf[fresh1 as usize] as libc::c_int & 0xff as libc::c_int)
                as libc::c_uint;
        let fresh2 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= (((*m).buf[fresh2 as usize] as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int) as libc::c_uint;
        let fresh3 = i;
        i = i.wrapping_add(1);
        group[j as usize]
            |= (((*m).buf[fresh3 as usize] as libc::c_int & 0xff as libc::c_int)
                << 16 as libc::c_int) as libc::c_uint;
        let fresh4 = i;
        i = i.wrapping_add(1);
        let fresh5 = j;
        j = j.wrapping_add(1);
        group[fresh5 as usize]
            |= (((*m).buf[fresh4 as usize] as libc::c_int & 0xff as libc::c_int)
                << 24 as libc::c_int) as libc::c_uint;
    }
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & c | !b & d)
                .wrapping_add(group[0 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][0 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(group[0 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][0 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & b | !a & c)
                .wrapping_add(group[1 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][1 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(group[1 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][1 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & a | !d & b)
                .wrapping_add(group[2 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][2 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(group[2 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][2 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & d | !c & a)
                .wrapping_add(group[3 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][3 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(group[3 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][3 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & c | !b & d)
                .wrapping_add(group[4 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][4 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(group[4 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][4 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & b | !a & c)
                .wrapping_add(group[5 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][5 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(group[5 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][5 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & a | !d & b)
                .wrapping_add(group[6 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][6 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(group[6 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][6 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & d | !c & a)
                .wrapping_add(group[7 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][7 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(group[7 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][7 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & c | !b & d)
                .wrapping_add(group[8 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][8 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(group[8 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][8 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & b | !a & c)
                .wrapping_add(group[9 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][9 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(group[9 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][9 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & a | !d & b)
                .wrapping_add(group[10 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][10 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(group[10 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][10 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & d | !c & a)
                .wrapping_add(group[11 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][11 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(group[11 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][11 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & c | !b & d)
                .wrapping_add(group[12 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][12 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(group[12 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][12 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & b | !a & c)
                .wrapping_add(group[13 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][13 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(group[13 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][13 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & a | !d & b)
                .wrapping_add(group[14 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][14 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(group[14 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][14 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & d | !c & a)
                .wrapping_add(group[15 as libc::c_int as usize])
                .wrapping_add(ti[0 as libc::c_int as usize][15 as libc::c_int as usize])
                << s[0 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(group[15 as libc::c_int as usize])
                    .wrapping_add(
                        ti[0 as libc::c_int as usize][15 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[0 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & d | c & !d)
                .wrapping_add(group[1 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][0 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & d | c & !d)
                    .wrapping_add(group[1 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][0 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & c | b & !c)
                .wrapping_add(group[6 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][1 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & c | b & !c)
                    .wrapping_add(group[6 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][1 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & b | a & !b)
                .wrapping_add(group[11 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][2 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & b | a & !b)
                    .wrapping_add(group[11 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][2 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & a | d & !a)
                .wrapping_add(group[0 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][3 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & a | d & !a)
                    .wrapping_add(group[0 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][3 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & d | c & !d)
                .wrapping_add(group[5 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][4 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & d | c & !d)
                    .wrapping_add(group[5 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][4 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & c | b & !c)
                .wrapping_add(group[10 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][5 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & c | b & !c)
                    .wrapping_add(group[10 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][5 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & b | a & !b)
                .wrapping_add(group[15 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][6 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & b | a & !b)
                    .wrapping_add(group[15 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][6 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & a | d & !a)
                .wrapping_add(group[4 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][7 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & a | d & !a)
                    .wrapping_add(group[4 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][7 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & d | c & !d)
                .wrapping_add(group[9 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][8 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & d | c & !d)
                    .wrapping_add(group[9 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][8 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & c | b & !c)
                .wrapping_add(group[14 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][9 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & c | b & !c)
                    .wrapping_add(group[14 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][9 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & b | a & !b)
                .wrapping_add(group[3 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][10 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & b | a & !b)
                    .wrapping_add(group[3 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][10 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & a | d & !a)
                .wrapping_add(group[8 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][11 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & a | d & !a)
                    .wrapping_add(group[8 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][11 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b & d | c & !d)
                .wrapping_add(group[13 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][12 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b & d | c & !d)
                    .wrapping_add(group[13 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][12 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a & c | b & !c)
                .wrapping_add(group[2 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][13 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a & c | b & !c)
                    .wrapping_add(group[2 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][13 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d & b | a & !b)
                .wrapping_add(group[7 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][14 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d & b | a & !b)
                    .wrapping_add(group[7 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][14 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c & a | d & !a)
                .wrapping_add(group[12 as libc::c_int as usize])
                .wrapping_add(ti[1 as libc::c_int as usize][15 as libc::c_int as usize])
                << s[1 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c & a | d & !a)
                    .wrapping_add(group[12 as libc::c_int as usize])
                    .wrapping_add(
                        ti[1 as libc::c_int as usize][15 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[1 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(group[5 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][0 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(group[5 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][0 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(group[8 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][1 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(group[8 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][1 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d ^ a ^ b)
                .wrapping_add(group[11 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][2 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(group[11 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][2 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c ^ d ^ a)
                .wrapping_add(group[14 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][3 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(group[14 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][3 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(group[1 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][4 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(group[1 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][4 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(group[4 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][5 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(group[4 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][5 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d ^ a ^ b)
                .wrapping_add(group[7 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][6 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(group[7 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][6 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c ^ d ^ a)
                .wrapping_add(group[10 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][7 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(group[10 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][7 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(group[13 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][8 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(group[13 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][8 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(group[0 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][9 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(group[0 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][9 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d ^ a ^ b)
                .wrapping_add(group[3 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][10 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(group[3 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][10 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c ^ d ^ a)
                .wrapping_add(group[6 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][11 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(group[6 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][11 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(group[9 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][12 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(group[9 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][12 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(a ^ b ^ c)
                .wrapping_add(group[12 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][13 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(group[12 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][13 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(d ^ a ^ b)
                .wrapping_add(group[15 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][14 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(group[15 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][14 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(c ^ d ^ a)
                .wrapping_add(group[2 as libc::c_int as usize])
                .wrapping_add(ti[2 as libc::c_int as usize][15 as libc::c_int as usize])
                << s[2 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(group[2 as libc::c_int as usize])
                    .wrapping_add(
                        ti[2 as libc::c_int as usize][15 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[2 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(c ^ (b | !d))
                .wrapping_add(group[0 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][0 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(group[0 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][0 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(b ^ (a | !c))
                .wrapping_add(group[7 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][1 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(group[7 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][1 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(a ^ (d | !b))
                .wrapping_add(group[14 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][2 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(group[14 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][2 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(d ^ (c | !a))
                .wrapping_add(group[5 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][3 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(group[5 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][3 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(c ^ (b | !d))
                .wrapping_add(group[12 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][4 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(group[12 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][4 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(b ^ (a | !c))
                .wrapping_add(group[3 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][5 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(group[3 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][5 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(a ^ (d | !b))
                .wrapping_add(group[10 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][6 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(group[10 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][6 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(d ^ (c | !a))
                .wrapping_add(group[1 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][7 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(group[1 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][7 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(c ^ (b | !d))
                .wrapping_add(group[8 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][8 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(group[8 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][8 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(b ^ (a | !c))
                .wrapping_add(group[15 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][9 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(group[15 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][9 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(a ^ (d | !b))
                .wrapping_add(group[6 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][10 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(group[6 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][10 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(d ^ (c | !a))
                .wrapping_add(group[13 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][11 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(group[13 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][11 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    a = b
        .wrapping_add(
            a
                .wrapping_add(c ^ (b | !d))
                .wrapping_add(group[4 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][12 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][0 as libc::c_int as usize]
                | a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(group[4 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][12 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][0 as libc::c_int as usize],
                        ),
        );
    d = a
        .wrapping_add(
            d
                .wrapping_add(b ^ (a | !c))
                .wrapping_add(group[11 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][13 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][1 as libc::c_int as usize]
                | d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(group[11 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][13 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][1 as libc::c_int as usize],
                        ),
        );
    c = d
        .wrapping_add(
            c
                .wrapping_add(a ^ (d | !b))
                .wrapping_add(group[2 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][14 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][2 as libc::c_int as usize]
                | c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(group[2 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][14 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][2 as libc::c_int as usize],
                        ),
        );
    b = c
        .wrapping_add(
            b
                .wrapping_add(d ^ (c | !a))
                .wrapping_add(group[9 as libc::c_int as usize])
                .wrapping_add(ti[3 as libc::c_int as usize][15 as libc::c_int as usize])
                << s[3 as libc::c_int as usize][3 as libc::c_int as usize]
                | b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(group[9 as libc::c_int as usize])
                    .wrapping_add(
                        ti[3 as libc::c_int as usize][15 as libc::c_int as usize],
                    )
                    >> (32 as libc::c_int as libc::c_uint)
                        .wrapping_sub(
                            s[3 as libc::c_int as usize][3 as libc::c_int as usize],
                        ),
        );
    (*m).A = ((*m).A as libc::c_uint).wrapping_add(a) as mln_u32_t as mln_u32_t;
    (*m).B = ((*m).B as libc::c_uint).wrapping_add(b) as mln_u32_t as mln_u32_t;
    (*m).C = ((*m).C as libc::c_uint).wrapping_add(c) as mln_u32_t as mln_u32_t;
    (*m).D = ((*m).D as libc::c_uint).wrapping_add(d) as mln_u32_t as mln_u32_t;
    (*m).A &= 0xffffffff as libc::c_uint;
    (*m).B &= 0xffffffff as libc::c_uint;
    (*m).C &= 0xffffffff as libc::c_uint;
    (*m).D &= 0xffffffff as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_tobytes(
    mut m: *mut mln_md5_t,
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
        ) = ((*m).A & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh7 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh7 as isize,
        ) = ((*m).A >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh8 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh8 as isize,
        ) = ((*m).A >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh9 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh9 as isize,
        ) = ((*m).A >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh10 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh10 as isize,
        ) = ((*m).B & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh11 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh11 as isize,
        ) = ((*m).B >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh12 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh12 as isize,
        ) = ((*m).B >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh13 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh13 as isize,
        ) = ((*m).B >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh14 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh14 as isize,
        ) = ((*m).C & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh15 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh15 as isize,
        ) = ((*m).C >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh16 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh16 as isize,
        ) = ((*m).C >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh17 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh17 as isize,
        ) = ((*m).C >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh18 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh18 as isize,
        ) = ((*m).D & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh19 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh19 as isize,
        ) = ((*m).D >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh20 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh20 as isize,
        ) = ((*m).D >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    if i >= len {
        return;
    }
    let fresh21 = i;
    i = i.wrapping_add(1);
    *buf
        .offset(
            fresh21 as isize,
        ) = ((*m).D >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn mln_md5_hex_tostring(mut c: mln_u8_t) -> mln_s8_t {
    return (if (c as libc::c_int) < 10 as libc::c_int {
        '0' as i32 + c as libc::c_int
    } else {
        'a' as i32 + (c as libc::c_int - 10 as libc::c_int)
    }) as mln_s8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_tostring(
    mut m: *mut mln_md5_t,
    mut buf: mln_s8ptr_t,
    mut len: mln_u32_t,
) {
    if buf.is_null() || len == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut i: mln_u32_t = 0;
    let mut bytes: [mln_u8_t; 16] = [
        0 as libc::c_int as mln_u8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    mln_md5_tobytes(
        m,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[mln_u8_t; 16]>() as libc::c_ulong as mln_u32_t,
    );
    len = (if len as libc::c_ulong
        > (::core::mem::size_of::<[mln_u8_t; 16]>() as libc::c_ulong) << 1 as libc::c_int
    {
        ::core::mem::size_of::<[mln_u8_t; 16]>() as libc::c_ulong
    } else {
        (len.wrapping_sub(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int)
            as libc::c_ulong
    }) as mln_u32_t;
    i = 0 as libc::c_int as mln_u32_t;
    while i < len {
        let fresh22 = buf;
        buf = buf.offset(1);
        *fresh22 = mln_md5_hex_tostring(
            (bytes[i as usize] as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                as mln_u8_t,
        );
        let fresh23 = buf;
        buf = buf.offset(1);
        *fresh23 = mln_md5_hex_tostring(
            (bytes[i as usize] as libc::c_int & 0xf as libc::c_int) as mln_u8_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    *buf = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn mln_md5_dump(mut m: *mut mln_md5_t) {
    printf(
        b"%lx %lx %lx %lx\n\0" as *const u8 as *const libc::c_char,
        (*m).A as libc::c_ulong,
        (*m).B as libc::c_ulong,
        (*m).C as libc::c_ulong,
        (*m).D as libc::c_ulong,
    );
}
