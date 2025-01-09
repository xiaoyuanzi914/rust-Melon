use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_des_t {
    pub key: mln_u64_t,
    pub sub_keys: [mln_u64_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_3des_t {
    pub _1key: mln_des_t,
    pub _2key: mln_des_t,
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
static mut begin_permutation: [mln_u8_t; 64] = [
    57 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    59 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    61 as libc::c_int as mln_u8_t,
    53 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    37 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    63 as libc::c_int as mln_u8_t,
    55 as libc::c_int as mln_u8_t,
    47 as libc::c_int as mln_u8_t,
    39 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    56 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    58 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    42 as libc::c_int as mln_u8_t,
    34 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    60 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    62 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
];
static mut key_permutation: [mln_u8_t; 56] = [
    56 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    57 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    58 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    42 as libc::c_int as mln_u8_t,
    34 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    59 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    62 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    61 as libc::c_int as mln_u8_t,
    53 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    37 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    60 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
];
static mut move_times: [mln_u8_t; 16] = [
    1 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
];
static mut compression_permutation: [mln_u8_t; 48] = [
    13 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    39 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    47 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    55 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
];
static mut extension_permutation: [mln_u8_t; 48] = [
    31 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
];
static mut s_box: [[mln_u8_t; 64]; 8] = [
    [
        14 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
    ],
    [
        15 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
    ],
    [
        10 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
    ],
    [
        7 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
    ],
    [
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
    ],
    [
        12 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
    ],
    [
        4 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
    ],
    [
        13 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        2 as libc::c_int as mln_u8_t,
        1 as libc::c_int as mln_u8_t,
        14 as libc::c_int as mln_u8_t,
        7 as libc::c_int as mln_u8_t,
        4 as libc::c_int as mln_u8_t,
        10 as libc::c_int as mln_u8_t,
        8 as libc::c_int as mln_u8_t,
        13 as libc::c_int as mln_u8_t,
        15 as libc::c_int as mln_u8_t,
        12 as libc::c_int as mln_u8_t,
        9 as libc::c_int as mln_u8_t,
        0 as libc::c_int as mln_u8_t,
        3 as libc::c_int as mln_u8_t,
        5 as libc::c_int as mln_u8_t,
        6 as libc::c_int as mln_u8_t,
        11 as libc::c_int as mln_u8_t,
    ],
];
static mut p_permutation: [mln_u8_t; 32] = [
    15 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
];
static mut final_permutation: [mln_u8_t; 64] = [
    39 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    47 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    55 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    63 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    62 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    37 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    53 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    61 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    60 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    59 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    34 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    42 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    58 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    57 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    56 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
];
#[no_mangle]
pub unsafe extern "C" fn mln_des_init(mut d: *mut mln_des_t, mut key: mln_u64_t) {
    let mut _56key: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut _h28key: mln_u64_t = 0;
    let mut _l28key: mln_u64_t = 0;
    let mut i: mln_s32_t = 0;
    let mut j: mln_s32_t = 0;
    let mut scan: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = key_permutation
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[mln_u8_t; 56]>() as libc::c_ulong as isize);
    (*d).key = key;
    i = 55 as libc::c_int;
    scan = key_permutation.as_mut_ptr();
    while scan < end {
        _56key
            |= (key >> 63 as libc::c_int - *scan as libc::c_int
                & 0x1 as libc::c_int as libc::c_ulong) << i;
        scan = scan.offset(1);
        scan;
        i -= 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        _h28key = _56key >> 28 as libc::c_int
            & 0xfffffff as libc::c_int as libc::c_ulong;
        _l28key = _56key & 0xfffffff as libc::c_int as libc::c_ulong;
        _h28key = (_h28key >> 28 as libc::c_int - move_times[j as usize] as libc::c_int
            | _h28key << move_times[j as usize] as libc::c_int)
            & 0xfffffff as libc::c_int as libc::c_ulong;
        _l28key = (_l28key >> 28 as libc::c_int - move_times[j as usize] as libc::c_int
            | _l28key << move_times[j as usize] as libc::c_int)
            & 0xfffffff as libc::c_int as libc::c_ulong;
        _56key = ((_h28key << 28 as libc::c_int | _l28key) as libc::c_ulonglong
            & 0xffffffffffffff as libc::c_ulonglong) as mln_u64_t;
        end = compression_permutation
            .as_mut_ptr()
            .offset(::core::mem::size_of::<[mln_u8_t; 48]>() as libc::c_ulong as isize);
        (*d).sub_keys[j as usize] = 0 as libc::c_int as mln_u64_t;
        i = 47 as libc::c_int;
        scan = compression_permutation.as_mut_ptr();
        while scan < end {
            (*d).sub_keys[j as usize]
                |= (_56key >> 55 as libc::c_int - *scan as libc::c_int
                    & 0x1 as libc::c_int as libc::c_ulong) << i;
            scan = scan.offset(1);
            scan;
            i -= 1;
            i;
        }
        (*d)
            .sub_keys[j
            as usize] = ((*d).sub_keys[j as usize] as libc::c_ulonglong
            & 0xffffffffffff as libc::c_ulonglong) as mln_u64_t;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_des_new(mut key: mln_u64_t) -> *mut mln_des_t {
    let mut d: *mut mln_des_t = malloc(
        ::core::mem::size_of::<mln_des_t>() as libc::c_ulong,
    ) as *mut mln_des_t;
    if d.is_null() {
        return 0 as *mut mln_des_t;
    }
    mln_des_init(d, key);
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn mln_des_pool_new(
    mut pool: *mut mln_alloc_t,
    mut key: mln_u64_t,
) -> *mut mln_des_t {
    let mut d: *mut mln_des_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_des_t>() as libc::c_ulong,
    ) as *mut mln_des_t;
    if d.is_null() {
        return 0 as *mut mln_des_t;
    }
    mln_des_init(d, key);
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn mln_des_free(mut d: *mut mln_des_t) {
    if d.is_null() {
        return;
    }
    free(d as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_des_pool_free(mut d: *mut mln_des_t) {
    if d.is_null() {
        return;
    }
    mln_alloc_free(d as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_des(
    mut d: *mut mln_des_t,
    mut msg: mln_u64_t,
    mut is_encrypt: mln_u32_t,
) -> mln_u64_t {
    let mut i: mln_s32_t = 0;
    let mut left: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut right: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut tmp: mln_u64_t = 0;
    mln_des_begin_permute(&mut msg);
    left = msg >> 32 as libc::c_int & 0xffffffff as libc::c_uint as libc::c_ulong;
    right = msg & 0xffffffff as libc::c_uint as libc::c_ulong;
    if is_encrypt != 0 {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            tmp = right;
            mln_des_extension_permute(&mut right);
            right ^= (*d).sub_keys[i as usize];
            mln_des_s_permute(&mut right);
            mln_des_p_permute(&mut right);
            right ^= left;
            left = tmp;
            i += 1;
            i;
        }
    } else {
        i = 15 as libc::c_int;
        while i >= 0 as libc::c_int {
            tmp = right;
            mln_des_extension_permute(&mut right);
            right ^= (*d).sub_keys[i as usize];
            mln_des_s_permute(&mut right);
            mln_des_p_permute(&mut right);
            right ^= left;
            left = tmp;
            i -= 1;
            i;
        }
    }
    msg = (right & 0xffffffff as libc::c_uint as libc::c_ulong) << 32 as libc::c_int
        | left & 0xffffffff as libc::c_uint as libc::c_ulong;
    mln_des_final_permute(&mut msg);
    return msg;
}
#[inline]
unsafe extern "C" fn mln_des_begin_permute(mut msg: *mut mln_u64_t) {
    let mut dup: mln_u64_t = *msg;
    let mut ret: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut i: mln_s32_t = 0;
    let mut scan: *mut mln_u8_t = 0 as *mut mln_u8_t;
    let mut end: *mut mln_u8_t = begin_permutation
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[mln_u8_t; 64]>() as libc::c_ulong as isize);
    i = 63 as libc::c_int;
    scan = begin_permutation.as_mut_ptr();
    while scan < end {
        ret
            |= (dup >> 63 as libc::c_int - *scan as libc::c_int
                & 0x1 as libc::c_int as libc::c_ulong) << i;
        scan = scan.offset(1);
        scan;
        i -= 1;
        i;
    }
    *msg = ret;
}
#[inline]
unsafe extern "C" fn mln_des_extension_permute(mut right: *mut mln_u64_t) {
    let mut dup: mln_u64_t = *right;
    let mut ret: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut i: mln_s32_t = 0;
    let mut scan: *mut mln_u8_t = 0 as *mut mln_u8_t;
    let mut end: *mut mln_u8_t = extension_permutation
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[mln_u8_t; 48]>() as libc::c_ulong as isize);
    i = 47 as libc::c_int;
    scan = extension_permutation.as_mut_ptr();
    while scan < end {
        ret
            |= (dup >> 31 as libc::c_int - *scan as libc::c_int
                & 0x1 as libc::c_int as libc::c_ulong) << i;
        scan = scan.offset(1);
        scan;
        i -= 1;
        i;
    }
    *right = (ret as libc::c_ulonglong & 0xffffffffffff as libc::c_ulonglong)
        as mln_u64_t;
}
#[inline]
unsafe extern "C" fn mln_des_s_permute(mut right: *mut mln_u64_t) {
    let mut dup: mln_u64_t = *right;
    let mut ret: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut _64tmp: mln_u64_t = 0;
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut index: mln_u32_t = 0;
    let mut _32tmp: mln_u32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < 8 as libc::c_int as libc::c_uint {
        _64tmp = dup >> i.wrapping_mul(6 as libc::c_int as libc::c_uint)
            & 0x3f as libc::c_int as libc::c_ulong;
        _32tmp = (_64tmp & 0x1 as libc::c_int as libc::c_ulong
            | (_64tmp >> 5 as libc::c_int & 0x1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) as mln_u32_t;
        index = (_64tmp >> 1 as libc::c_int & 0xf as libc::c_int as libc::c_ulong)
            .wrapping_add((_32tmp << 4 as libc::c_int) as libc::c_ulong) as mln_u32_t;
        ret
            |= ((s_box[(7 as libc::c_int as libc::c_uint).wrapping_sub(i)
                as usize][index as usize] as libc::c_int & 0xf as libc::c_int)
                as mln_u64_t) << (i << 2 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    *right = ret & 0xffffffff as libc::c_uint as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn mln_des_p_permute(mut right: *mut mln_u64_t) {
    let mut dup: mln_u64_t = *right;
    let mut ret: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut i: mln_s32_t = 0;
    let mut scan: *mut mln_u8_t = 0 as *mut mln_u8_t;
    let mut end: *mut mln_u8_t = p_permutation
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[mln_u8_t; 32]>() as libc::c_ulong as isize);
    i = 31 as libc::c_int;
    scan = p_permutation.as_mut_ptr();
    while scan < end {
        ret
            |= (dup >> 31 as libc::c_int - *scan as libc::c_int
                & 0x1 as libc::c_int as libc::c_ulong) << i;
        scan = scan.offset(1);
        scan;
        i -= 1;
        i;
    }
    *right = ret & 0xffffffff as libc::c_uint as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn mln_des_final_permute(mut msg: *mut mln_u64_t) {
    let mut dup: mln_u64_t = *msg;
    let mut ret: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut scan: *mut mln_u8_t = 0 as *mut mln_u8_t;
    let mut end: *mut mln_u8_t = final_permutation
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[mln_u8_t; 64]>() as libc::c_ulong as isize);
    scan = final_permutation.as_mut_ptr();
    while scan < end {
        ret |= (dup >> *scan as libc::c_int & 0x1 as libc::c_int as libc::c_ulong) << i;
        scan = scan.offset(1);
        scan;
        i = i.wrapping_add(1);
        i;
    }
    *msg = ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_des_buf(
    mut d: *mut mln_des_t,
    mut in_0: mln_u8ptr_t,
    mut inlen: mln_uauto_t,
    mut out: mln_u8ptr_t,
    mut outlen: mln_uauto_t,
    mut fill: mln_u8_t,
    mut is_encrypt: mln_u32_t,
) {
    let mut i: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    let mut input: mln_u64_t = 0;
    let mut output: mln_u64_t = 0;
    while inlen != 0 {
        input = 0 as libc::c_int as mln_u64_t;
        i = 0 as libc::c_int as mln_uauto_t;
        while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong
            && inlen > 0 as libc::c_int as libc::c_ulong
        {
            input
                |= (*in_0 as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                    << ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i) << 3 as libc::c_int);
            i = i.wrapping_add(1);
            i;
            inlen = inlen.wrapping_sub(1);
            inlen;
            in_0 = in_0.offset(1);
            in_0;
        }
        if i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
            while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
                input
                    |= (fill as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i) << 3 as libc::c_int);
                i = i.wrapping_add(1);
                i;
            }
        }
        output = mln_des(d, input, is_encrypt);
        i = 0 as libc::c_int as mln_uauto_t;
        while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
            if outlen == 0 as libc::c_int as libc::c_ulong {
                return;
            }
            *out = (output
                >> ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
            outlen = outlen.wrapping_sub(1);
            outlen;
            out = out.offset(1);
            out;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_init(
    mut tdes: *mut mln_3des_t,
    mut key1: mln_u64_t,
    mut key2: mln_u64_t,
) {
    mln_des_init(&mut (*tdes)._1key, key1);
    mln_des_init(&mut (*tdes)._2key, key2);
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_new(
    mut key1: mln_u64_t,
    mut key2: mln_u64_t,
) -> *mut mln_3des_t {
    let mut _3d: *mut mln_3des_t = malloc(
        ::core::mem::size_of::<mln_3des_t>() as libc::c_ulong,
    ) as *mut mln_3des_t;
    if _3d.is_null() {
        return 0 as *mut mln_3des_t;
    }
    mln_3des_init(_3d, key1, key2);
    return _3d;
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_pool_new(
    mut pool: *mut mln_alloc_t,
    mut key1: mln_u64_t,
    mut key2: mln_u64_t,
) -> *mut mln_3des_t {
    let mut _3d: *mut mln_3des_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_3des_t>() as libc::c_ulong,
    ) as *mut mln_3des_t;
    if _3d.is_null() {
        return 0 as *mut mln_3des_t;
    }
    mln_3des_init(_3d, key1, key2);
    return _3d;
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_free(mut tdes: *mut mln_3des_t) {
    if tdes.is_null() {
        return;
    }
    free(tdes as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_pool_free(mut tdes: *mut mln_3des_t) {
    if tdes.is_null() {
        return;
    }
    mln_alloc_free(tdes as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des(
    mut tdes: *mut mln_3des_t,
    mut msg: mln_u64_t,
    mut is_encrypt: mln_u32_t,
) -> mln_u64_t {
    if is_encrypt != 0 {
        return mln_des(
            &mut (*tdes)._1key,
            mln_des(
                &mut (*tdes)._2key,
                mln_des(&mut (*tdes)._1key, msg, 1 as libc::c_int as mln_u32_t),
                0 as libc::c_int as mln_u32_t,
            ),
            1 as libc::c_int as mln_u32_t,
        );
    }
    return mln_des(
        &mut (*tdes)._1key,
        mln_des(
            &mut (*tdes)._2key,
            mln_des(&mut (*tdes)._1key, msg, 0 as libc::c_int as mln_u32_t),
            1 as libc::c_int as mln_u32_t,
        ),
        0 as libc::c_int as mln_u32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_3des_buf(
    mut tdes: *mut mln_3des_t,
    mut in_0: mln_u8ptr_t,
    mut inlen: mln_uauto_t,
    mut out: mln_u8ptr_t,
    mut outlen: mln_uauto_t,
    mut fill: mln_u8_t,
    mut is_encrypt: mln_u32_t,
) {
    let mut i: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    let mut input: mln_u64_t = 0;
    let mut output: mln_u64_t = 0;
    while inlen != 0 {
        input = 0 as libc::c_int as mln_u64_t;
        i = 0 as libc::c_int as mln_uauto_t;
        while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong
            && inlen > 0 as libc::c_int as libc::c_ulong
        {
            input
                |= (*in_0 as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                    << ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i) << 3 as libc::c_int);
            i = i.wrapping_add(1);
            i;
            inlen = inlen.wrapping_sub(1);
            inlen;
            in_0 = in_0.offset(1);
            in_0;
        }
        if i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
            while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
                input
                    |= (fill as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i) << 3 as libc::c_int);
                i = i.wrapping_add(1);
                i;
            }
        }
        output = mln_3des(tdes, input, is_encrypt);
        i = 0 as libc::c_int as mln_uauto_t;
        while i < ::core::mem::size_of::<mln_u64_t>() as libc::c_ulong {
            if outlen == 0 as libc::c_int as libc::c_ulong {
                return;
            }
            *out = (output
                >> ((::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
            outlen = outlen.wrapping_sub(1);
            outlen;
            out = out.offset(1);
            out;
        }
    }
}
