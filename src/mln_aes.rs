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
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_off_t = off_t;
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
pub struct mln_aes_t {
    pub bits: mln_u32_t,
    pub w: [mln_u32_t; 60],
}

impl Default for mln_aes_t {
    fn default() -> Self {
        mln_aes_t {
            bits: 0,          // 初始化为零
            w: [0; 60],       // 初始化为零
        }
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
static mut sbox: [mln_u8_t; 256] = [
    0x63 as libc::c_int as mln_u8_t,
    0x7c as libc::c_int as mln_u8_t,
    0x77 as libc::c_int as mln_u8_t,
    0x7b as libc::c_int as mln_u8_t,
    0xf2 as libc::c_int as mln_u8_t,
    0x6b as libc::c_int as mln_u8_t,
    0x6f as libc::c_int as mln_u8_t,
    0xc5 as libc::c_int as mln_u8_t,
    0x30 as libc::c_int as mln_u8_t,
    0x1 as libc::c_int as mln_u8_t,
    0x67 as libc::c_int as mln_u8_t,
    0x2b as libc::c_int as mln_u8_t,
    0xfe as libc::c_int as mln_u8_t,
    0xd7 as libc::c_int as mln_u8_t,
    0xab as libc::c_int as mln_u8_t,
    0x76 as libc::c_int as mln_u8_t,
    0xca as libc::c_int as mln_u8_t,
    0x82 as libc::c_int as mln_u8_t,
    0xc9 as libc::c_int as mln_u8_t,
    0x7d as libc::c_int as mln_u8_t,
    0xfa as libc::c_int as mln_u8_t,
    0x59 as libc::c_int as mln_u8_t,
    0x47 as libc::c_int as mln_u8_t,
    0xf0 as libc::c_int as mln_u8_t,
    0xad as libc::c_int as mln_u8_t,
    0xd4 as libc::c_int as mln_u8_t,
    0xa2 as libc::c_int as mln_u8_t,
    0xaf as libc::c_int as mln_u8_t,
    0x9c as libc::c_int as mln_u8_t,
    0xa4 as libc::c_int as mln_u8_t,
    0x72 as libc::c_int as mln_u8_t,
    0xc0 as libc::c_int as mln_u8_t,
    0xb7 as libc::c_int as mln_u8_t,
    0xfd as libc::c_int as mln_u8_t,
    0x93 as libc::c_int as mln_u8_t,
    0x26 as libc::c_int as mln_u8_t,
    0x36 as libc::c_int as mln_u8_t,
    0x3f as libc::c_int as mln_u8_t,
    0xf7 as libc::c_int as mln_u8_t,
    0xcc as libc::c_int as mln_u8_t,
    0x34 as libc::c_int as mln_u8_t,
    0xa5 as libc::c_int as mln_u8_t,
    0xe5 as libc::c_int as mln_u8_t,
    0xf1 as libc::c_int as mln_u8_t,
    0x71 as libc::c_int as mln_u8_t,
    0xd8 as libc::c_int as mln_u8_t,
    0x31 as libc::c_int as mln_u8_t,
    0x15 as libc::c_int as mln_u8_t,
    0x4 as libc::c_int as mln_u8_t,
    0xc7 as libc::c_int as mln_u8_t,
    0x23 as libc::c_int as mln_u8_t,
    0xc3 as libc::c_int as mln_u8_t,
    0x18 as libc::c_int as mln_u8_t,
    0x96 as libc::c_int as mln_u8_t,
    0x5 as libc::c_int as mln_u8_t,
    0x9a as libc::c_int as mln_u8_t,
    0x7 as libc::c_int as mln_u8_t,
    0x12 as libc::c_int as mln_u8_t,
    0x80 as libc::c_int as mln_u8_t,
    0xe2 as libc::c_int as mln_u8_t,
    0xeb as libc::c_int as mln_u8_t,
    0x27 as libc::c_int as mln_u8_t,
    0xb2 as libc::c_int as mln_u8_t,
    0x75 as libc::c_int as mln_u8_t,
    0x9 as libc::c_int as mln_u8_t,
    0x83 as libc::c_int as mln_u8_t,
    0x2c as libc::c_int as mln_u8_t,
    0x1a as libc::c_int as mln_u8_t,
    0x1b as libc::c_int as mln_u8_t,
    0x6e as libc::c_int as mln_u8_t,
    0x5a as libc::c_int as mln_u8_t,
    0xa0 as libc::c_int as mln_u8_t,
    0x52 as libc::c_int as mln_u8_t,
    0x3b as libc::c_int as mln_u8_t,
    0xd6 as libc::c_int as mln_u8_t,
    0xb3 as libc::c_int as mln_u8_t,
    0x29 as libc::c_int as mln_u8_t,
    0xe3 as libc::c_int as mln_u8_t,
    0x2f as libc::c_int as mln_u8_t,
    0x84 as libc::c_int as mln_u8_t,
    0x53 as libc::c_int as mln_u8_t,
    0xd1 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    0xed as libc::c_int as mln_u8_t,
    0x20 as libc::c_int as mln_u8_t,
    0xfc as libc::c_int as mln_u8_t,
    0xb1 as libc::c_int as mln_u8_t,
    0x5b as libc::c_int as mln_u8_t,
    0x6a as libc::c_int as mln_u8_t,
    0xcb as libc::c_int as mln_u8_t,
    0xbe as libc::c_int as mln_u8_t,
    0x39 as libc::c_int as mln_u8_t,
    0x4a as libc::c_int as mln_u8_t,
    0x4c as libc::c_int as mln_u8_t,
    0x58 as libc::c_int as mln_u8_t,
    0xcf as libc::c_int as mln_u8_t,
    0xd0 as libc::c_int as mln_u8_t,
    0xef as libc::c_int as mln_u8_t,
    0xaa as libc::c_int as mln_u8_t,
    0xfb as libc::c_int as mln_u8_t,
    0x43 as libc::c_int as mln_u8_t,
    0x4d as libc::c_int as mln_u8_t,
    0x33 as libc::c_int as mln_u8_t,
    0x85 as libc::c_int as mln_u8_t,
    0x45 as libc::c_int as mln_u8_t,
    0xf9 as libc::c_int as mln_u8_t,
    0x2 as libc::c_int as mln_u8_t,
    0x7f as libc::c_int as mln_u8_t,
    0x50 as libc::c_int as mln_u8_t,
    0x3c as libc::c_int as mln_u8_t,
    0x9f as libc::c_int as mln_u8_t,
    0xa8 as libc::c_int as mln_u8_t,
    0x51 as libc::c_int as mln_u8_t,
    0xa3 as libc::c_int as mln_u8_t,
    0x40 as libc::c_int as mln_u8_t,
    0x8f as libc::c_int as mln_u8_t,
    0x92 as libc::c_int as mln_u8_t,
    0x9d as libc::c_int as mln_u8_t,
    0x38 as libc::c_int as mln_u8_t,
    0xf5 as libc::c_int as mln_u8_t,
    0xbc as libc::c_int as mln_u8_t,
    0xb6 as libc::c_int as mln_u8_t,
    0xda as libc::c_int as mln_u8_t,
    0x21 as libc::c_int as mln_u8_t,
    0x10 as libc::c_int as mln_u8_t,
    0xff as libc::c_int as mln_u8_t,
    0xf3 as libc::c_int as mln_u8_t,
    0xd2 as libc::c_int as mln_u8_t,
    0xcd as libc::c_int as mln_u8_t,
    0xc as libc::c_int as mln_u8_t,
    0x13 as libc::c_int as mln_u8_t,
    0xec as libc::c_int as mln_u8_t,
    0x5f as libc::c_int as mln_u8_t,
    0x97 as libc::c_int as mln_u8_t,
    0x44 as libc::c_int as mln_u8_t,
    0x17 as libc::c_int as mln_u8_t,
    0xc4 as libc::c_int as mln_u8_t,
    0xa7 as libc::c_int as mln_u8_t,
    0x7e as libc::c_int as mln_u8_t,
    0x3d as libc::c_int as mln_u8_t,
    0x64 as libc::c_int as mln_u8_t,
    0x5d as libc::c_int as mln_u8_t,
    0x19 as libc::c_int as mln_u8_t,
    0x73 as libc::c_int as mln_u8_t,
    0x60 as libc::c_int as mln_u8_t,
    0x81 as libc::c_int as mln_u8_t,
    0x4f as libc::c_int as mln_u8_t,
    0xdc as libc::c_int as mln_u8_t,
    0x22 as libc::c_int as mln_u8_t,
    0x2a as libc::c_int as mln_u8_t,
    0x90 as libc::c_int as mln_u8_t,
    0x88 as libc::c_int as mln_u8_t,
    0x46 as libc::c_int as mln_u8_t,
    0xee as libc::c_int as mln_u8_t,
    0xb8 as libc::c_int as mln_u8_t,
    0x14 as libc::c_int as mln_u8_t,
    0xde as libc::c_int as mln_u8_t,
    0x5e as libc::c_int as mln_u8_t,
    0xb as libc::c_int as mln_u8_t,
    0xdb as libc::c_int as mln_u8_t,
    0xe0 as libc::c_int as mln_u8_t,
    0x32 as libc::c_int as mln_u8_t,
    0x3a as libc::c_int as mln_u8_t,
    0xa as libc::c_int as mln_u8_t,
    0x49 as libc::c_int as mln_u8_t,
    0x6 as libc::c_int as mln_u8_t,
    0x24 as libc::c_int as mln_u8_t,
    0x5c as libc::c_int as mln_u8_t,
    0xc2 as libc::c_int as mln_u8_t,
    0xd3 as libc::c_int as mln_u8_t,
    0xac as libc::c_int as mln_u8_t,
    0x62 as libc::c_int as mln_u8_t,
    0x91 as libc::c_int as mln_u8_t,
    0x95 as libc::c_int as mln_u8_t,
    0xe4 as libc::c_int as mln_u8_t,
    0x79 as libc::c_int as mln_u8_t,
    0xe7 as libc::c_int as mln_u8_t,
    0xc8 as libc::c_int as mln_u8_t,
    0x37 as libc::c_int as mln_u8_t,
    0x6d as libc::c_int as mln_u8_t,
    0x8d as libc::c_int as mln_u8_t,
    0xd5 as libc::c_int as mln_u8_t,
    0x4e as libc::c_int as mln_u8_t,
    0xa9 as libc::c_int as mln_u8_t,
    0x6c as libc::c_int as mln_u8_t,
    0x56 as libc::c_int as mln_u8_t,
    0xf4 as libc::c_int as mln_u8_t,
    0xea as libc::c_int as mln_u8_t,
    0x65 as libc::c_int as mln_u8_t,
    0x7a as libc::c_int as mln_u8_t,
    0xae as libc::c_int as mln_u8_t,
    0x8 as libc::c_int as mln_u8_t,
    0xba as libc::c_int as mln_u8_t,
    0x78 as libc::c_int as mln_u8_t,
    0x25 as libc::c_int as mln_u8_t,
    0x2e as libc::c_int as mln_u8_t,
    0x1c as libc::c_int as mln_u8_t,
    0xa6 as libc::c_int as mln_u8_t,
    0xb4 as libc::c_int as mln_u8_t,
    0xc6 as libc::c_int as mln_u8_t,
    0xe8 as libc::c_int as mln_u8_t,
    0xdd as libc::c_int as mln_u8_t,
    0x74 as libc::c_int as mln_u8_t,
    0x1f as libc::c_int as mln_u8_t,
    0x4b as libc::c_int as mln_u8_t,
    0xbd as libc::c_int as mln_u8_t,
    0x8b as libc::c_int as mln_u8_t,
    0x8a as libc::c_int as mln_u8_t,
    0x70 as libc::c_int as mln_u8_t,
    0x3e as libc::c_int as mln_u8_t,
    0xb5 as libc::c_int as mln_u8_t,
    0x66 as libc::c_int as mln_u8_t,
    0x48 as libc::c_int as mln_u8_t,
    0x3 as libc::c_int as mln_u8_t,
    0xf6 as libc::c_int as mln_u8_t,
    0xe as libc::c_int as mln_u8_t,
    0x61 as libc::c_int as mln_u8_t,
    0x35 as libc::c_int as mln_u8_t,
    0x57 as libc::c_int as mln_u8_t,
    0xb9 as libc::c_int as mln_u8_t,
    0x86 as libc::c_int as mln_u8_t,
    0xc1 as libc::c_int as mln_u8_t,
    0x1d as libc::c_int as mln_u8_t,
    0x9e as libc::c_int as mln_u8_t,
    0xe1 as libc::c_int as mln_u8_t,
    0xf8 as libc::c_int as mln_u8_t,
    0x98 as libc::c_int as mln_u8_t,
    0x11 as libc::c_int as mln_u8_t,
    0x69 as libc::c_int as mln_u8_t,
    0xd9 as libc::c_int as mln_u8_t,
    0x8e as libc::c_int as mln_u8_t,
    0x94 as libc::c_int as mln_u8_t,
    0x9b as libc::c_int as mln_u8_t,
    0x1e as libc::c_int as mln_u8_t,
    0x87 as libc::c_int as mln_u8_t,
    0xe9 as libc::c_int as mln_u8_t,
    0xce as libc::c_int as mln_u8_t,
    0x55 as libc::c_int as mln_u8_t,
    0x28 as libc::c_int as mln_u8_t,
    0xdf as libc::c_int as mln_u8_t,
    0x8c as libc::c_int as mln_u8_t,
    0xa1 as libc::c_int as mln_u8_t,
    0x89 as libc::c_int as mln_u8_t,
    0xd as libc::c_int as mln_u8_t,
    0xbf as libc::c_int as mln_u8_t,
    0xe6 as libc::c_int as mln_u8_t,
    0x42 as libc::c_int as mln_u8_t,
    0x68 as libc::c_int as mln_u8_t,
    0x41 as libc::c_int as mln_u8_t,
    0x99 as libc::c_int as mln_u8_t,
    0x2d as libc::c_int as mln_u8_t,
    0xf as libc::c_int as mln_u8_t,
    0xb0 as libc::c_int as mln_u8_t,
    0x54 as libc::c_int as mln_u8_t,
    0xbb as libc::c_int as mln_u8_t,
    0x16 as libc::c_int as mln_u8_t,
];
static mut rsbox: [mln_u8_t; 256] = [
    0x52 as libc::c_int as mln_u8_t,
    0x9 as libc::c_int as mln_u8_t,
    0x6a as libc::c_int as mln_u8_t,
    0xd5 as libc::c_int as mln_u8_t,
    0x30 as libc::c_int as mln_u8_t,
    0x36 as libc::c_int as mln_u8_t,
    0xa5 as libc::c_int as mln_u8_t,
    0x38 as libc::c_int as mln_u8_t,
    0xbf as libc::c_int as mln_u8_t,
    0x40 as libc::c_int as mln_u8_t,
    0xa3 as libc::c_int as mln_u8_t,
    0x9e as libc::c_int as mln_u8_t,
    0x81 as libc::c_int as mln_u8_t,
    0xf3 as libc::c_int as mln_u8_t,
    0xd7 as libc::c_int as mln_u8_t,
    0xfb as libc::c_int as mln_u8_t,
    0x7c as libc::c_int as mln_u8_t,
    0xe3 as libc::c_int as mln_u8_t,
    0x39 as libc::c_int as mln_u8_t,
    0x82 as libc::c_int as mln_u8_t,
    0x9b as libc::c_int as mln_u8_t,
    0x2f as libc::c_int as mln_u8_t,
    0xff as libc::c_int as mln_u8_t,
    0x87 as libc::c_int as mln_u8_t,
    0x34 as libc::c_int as mln_u8_t,
    0x8e as libc::c_int as mln_u8_t,
    0x43 as libc::c_int as mln_u8_t,
    0x44 as libc::c_int as mln_u8_t,
    0xc4 as libc::c_int as mln_u8_t,
    0xde as libc::c_int as mln_u8_t,
    0xe9 as libc::c_int as mln_u8_t,
    0xcb as libc::c_int as mln_u8_t,
    0x54 as libc::c_int as mln_u8_t,
    0x7b as libc::c_int as mln_u8_t,
    0x94 as libc::c_int as mln_u8_t,
    0x32 as libc::c_int as mln_u8_t,
    0xa6 as libc::c_int as mln_u8_t,
    0xc2 as libc::c_int as mln_u8_t,
    0x23 as libc::c_int as mln_u8_t,
    0x3d as libc::c_int as mln_u8_t,
    0xee as libc::c_int as mln_u8_t,
    0x4c as libc::c_int as mln_u8_t,
    0x95 as libc::c_int as mln_u8_t,
    0xb as libc::c_int as mln_u8_t,
    0x42 as libc::c_int as mln_u8_t,
    0xfa as libc::c_int as mln_u8_t,
    0xc3 as libc::c_int as mln_u8_t,
    0x4e as libc::c_int as mln_u8_t,
    0x8 as libc::c_int as mln_u8_t,
    0x2e as libc::c_int as mln_u8_t,
    0xa1 as libc::c_int as mln_u8_t,
    0x66 as libc::c_int as mln_u8_t,
    0x28 as libc::c_int as mln_u8_t,
    0xd9 as libc::c_int as mln_u8_t,
    0x24 as libc::c_int as mln_u8_t,
    0xb2 as libc::c_int as mln_u8_t,
    0x76 as libc::c_int as mln_u8_t,
    0x5b as libc::c_int as mln_u8_t,
    0xa2 as libc::c_int as mln_u8_t,
    0x49 as libc::c_int as mln_u8_t,
    0x6d as libc::c_int as mln_u8_t,
    0x8b as libc::c_int as mln_u8_t,
    0xd1 as libc::c_int as mln_u8_t,
    0x25 as libc::c_int as mln_u8_t,
    0x72 as libc::c_int as mln_u8_t,
    0xf8 as libc::c_int as mln_u8_t,
    0xf6 as libc::c_int as mln_u8_t,
    0x64 as libc::c_int as mln_u8_t,
    0x86 as libc::c_int as mln_u8_t,
    0x68 as libc::c_int as mln_u8_t,
    0x98 as libc::c_int as mln_u8_t,
    0x16 as libc::c_int as mln_u8_t,
    0xd4 as libc::c_int as mln_u8_t,
    0xa4 as libc::c_int as mln_u8_t,
    0x5c as libc::c_int as mln_u8_t,
    0xcc as libc::c_int as mln_u8_t,
    0x5d as libc::c_int as mln_u8_t,
    0x65 as libc::c_int as mln_u8_t,
    0xb6 as libc::c_int as mln_u8_t,
    0x92 as libc::c_int as mln_u8_t,
    0x6c as libc::c_int as mln_u8_t,
    0x70 as libc::c_int as mln_u8_t,
    0x48 as libc::c_int as mln_u8_t,
    0x50 as libc::c_int as mln_u8_t,
    0xfd as libc::c_int as mln_u8_t,
    0xed as libc::c_int as mln_u8_t,
    0xb9 as libc::c_int as mln_u8_t,
    0xda as libc::c_int as mln_u8_t,
    0x5e as libc::c_int as mln_u8_t,
    0x15 as libc::c_int as mln_u8_t,
    0x46 as libc::c_int as mln_u8_t,
    0x57 as libc::c_int as mln_u8_t,
    0xa7 as libc::c_int as mln_u8_t,
    0x8d as libc::c_int as mln_u8_t,
    0x9d as libc::c_int as mln_u8_t,
    0x84 as libc::c_int as mln_u8_t,
    0x90 as libc::c_int as mln_u8_t,
    0xd8 as libc::c_int as mln_u8_t,
    0xab as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    0x8c as libc::c_int as mln_u8_t,
    0xbc as libc::c_int as mln_u8_t,
    0xd3 as libc::c_int as mln_u8_t,
    0xa as libc::c_int as mln_u8_t,
    0xf7 as libc::c_int as mln_u8_t,
    0xe4 as libc::c_int as mln_u8_t,
    0x58 as libc::c_int as mln_u8_t,
    0x5 as libc::c_int as mln_u8_t,
    0xb8 as libc::c_int as mln_u8_t,
    0xb3 as libc::c_int as mln_u8_t,
    0x45 as libc::c_int as mln_u8_t,
    0x6 as libc::c_int as mln_u8_t,
    0xd0 as libc::c_int as mln_u8_t,
    0x2c as libc::c_int as mln_u8_t,
    0x1e as libc::c_int as mln_u8_t,
    0x8f as libc::c_int as mln_u8_t,
    0xca as libc::c_int as mln_u8_t,
    0x3f as libc::c_int as mln_u8_t,
    0xf as libc::c_int as mln_u8_t,
    0x2 as libc::c_int as mln_u8_t,
    0xc1 as libc::c_int as mln_u8_t,
    0xaf as libc::c_int as mln_u8_t,
    0xbd as libc::c_int as mln_u8_t,
    0x3 as libc::c_int as mln_u8_t,
    0x1 as libc::c_int as mln_u8_t,
    0x13 as libc::c_int as mln_u8_t,
    0x8a as libc::c_int as mln_u8_t,
    0x6b as libc::c_int as mln_u8_t,
    0x3a as libc::c_int as mln_u8_t,
    0x91 as libc::c_int as mln_u8_t,
    0x11 as libc::c_int as mln_u8_t,
    0x41 as libc::c_int as mln_u8_t,
    0x4f as libc::c_int as mln_u8_t,
    0x67 as libc::c_int as mln_u8_t,
    0xdc as libc::c_int as mln_u8_t,
    0xea as libc::c_int as mln_u8_t,
    0x97 as libc::c_int as mln_u8_t,
    0xf2 as libc::c_int as mln_u8_t,
    0xcf as libc::c_int as mln_u8_t,
    0xce as libc::c_int as mln_u8_t,
    0xf0 as libc::c_int as mln_u8_t,
    0xb4 as libc::c_int as mln_u8_t,
    0xe6 as libc::c_int as mln_u8_t,
    0x73 as libc::c_int as mln_u8_t,
    0x96 as libc::c_int as mln_u8_t,
    0xac as libc::c_int as mln_u8_t,
    0x74 as libc::c_int as mln_u8_t,
    0x22 as libc::c_int as mln_u8_t,
    0xe7 as libc::c_int as mln_u8_t,
    0xad as libc::c_int as mln_u8_t,
    0x35 as libc::c_int as mln_u8_t,
    0x85 as libc::c_int as mln_u8_t,
    0xe2 as libc::c_int as mln_u8_t,
    0xf9 as libc::c_int as mln_u8_t,
    0x37 as libc::c_int as mln_u8_t,
    0xe8 as libc::c_int as mln_u8_t,
    0x1c as libc::c_int as mln_u8_t,
    0x75 as libc::c_int as mln_u8_t,
    0xdf as libc::c_int as mln_u8_t,
    0x6e as libc::c_int as mln_u8_t,
    0x47 as libc::c_int as mln_u8_t,
    0xf1 as libc::c_int as mln_u8_t,
    0x1a as libc::c_int as mln_u8_t,
    0x71 as libc::c_int as mln_u8_t,
    0x1d as libc::c_int as mln_u8_t,
    0x29 as libc::c_int as mln_u8_t,
    0xc5 as libc::c_int as mln_u8_t,
    0x89 as libc::c_int as mln_u8_t,
    0x6f as libc::c_int as mln_u8_t,
    0xb7 as libc::c_int as mln_u8_t,
    0x62 as libc::c_int as mln_u8_t,
    0xe as libc::c_int as mln_u8_t,
    0xaa as libc::c_int as mln_u8_t,
    0x18 as libc::c_int as mln_u8_t,
    0xbe as libc::c_int as mln_u8_t,
    0x1b as libc::c_int as mln_u8_t,
    0xfc as libc::c_int as mln_u8_t,
    0x56 as libc::c_int as mln_u8_t,
    0x3e as libc::c_int as mln_u8_t,
    0x4b as libc::c_int as mln_u8_t,
    0xc6 as libc::c_int as mln_u8_t,
    0xd2 as libc::c_int as mln_u8_t,
    0x79 as libc::c_int as mln_u8_t,
    0x20 as libc::c_int as mln_u8_t,
    0x9a as libc::c_int as mln_u8_t,
    0xdb as libc::c_int as mln_u8_t,
    0xc0 as libc::c_int as mln_u8_t,
    0xfe as libc::c_int as mln_u8_t,
    0x78 as libc::c_int as mln_u8_t,
    0xcd as libc::c_int as mln_u8_t,
    0x5a as libc::c_int as mln_u8_t,
    0xf4 as libc::c_int as mln_u8_t,
    0x1f as libc::c_int as mln_u8_t,
    0xdd as libc::c_int as mln_u8_t,
    0xa8 as libc::c_int as mln_u8_t,
    0x33 as libc::c_int as mln_u8_t,
    0x88 as libc::c_int as mln_u8_t,
    0x7 as libc::c_int as mln_u8_t,
    0xc7 as libc::c_int as mln_u8_t,
    0x31 as libc::c_int as mln_u8_t,
    0xb1 as libc::c_int as mln_u8_t,
    0x12 as libc::c_int as mln_u8_t,
    0x10 as libc::c_int as mln_u8_t,
    0x59 as libc::c_int as mln_u8_t,
    0x27 as libc::c_int as mln_u8_t,
    0x80 as libc::c_int as mln_u8_t,
    0xec as libc::c_int as mln_u8_t,
    0x5f as libc::c_int as mln_u8_t,
    0x60 as libc::c_int as mln_u8_t,
    0x51 as libc::c_int as mln_u8_t,
    0x7f as libc::c_int as mln_u8_t,
    0xa9 as libc::c_int as mln_u8_t,
    0x19 as libc::c_int as mln_u8_t,
    0xb5 as libc::c_int as mln_u8_t,
    0x4a as libc::c_int as mln_u8_t,
    0xd as libc::c_int as mln_u8_t,
    0x2d as libc::c_int as mln_u8_t,
    0xe5 as libc::c_int as mln_u8_t,
    0x7a as libc::c_int as mln_u8_t,
    0x9f as libc::c_int as mln_u8_t,
    0x93 as libc::c_int as mln_u8_t,
    0xc9 as libc::c_int as mln_u8_t,
    0x9c as libc::c_int as mln_u8_t,
    0xef as libc::c_int as mln_u8_t,
    0xa0 as libc::c_int as mln_u8_t,
    0xe0 as libc::c_int as mln_u8_t,
    0x3b as libc::c_int as mln_u8_t,
    0x4d as libc::c_int as mln_u8_t,
    0xae as libc::c_int as mln_u8_t,
    0x2a as libc::c_int as mln_u8_t,
    0xf5 as libc::c_int as mln_u8_t,
    0xb0 as libc::c_int as mln_u8_t,
    0xc8 as libc::c_int as mln_u8_t,
    0xeb as libc::c_int as mln_u8_t,
    0xbb as libc::c_int as mln_u8_t,
    0x3c as libc::c_int as mln_u8_t,
    0x83 as libc::c_int as mln_u8_t,
    0x53 as libc::c_int as mln_u8_t,
    0x99 as libc::c_int as mln_u8_t,
    0x61 as libc::c_int as mln_u8_t,
    0x17 as libc::c_int as mln_u8_t,
    0x2b as libc::c_int as mln_u8_t,
    0x4 as libc::c_int as mln_u8_t,
    0x7e as libc::c_int as mln_u8_t,
    0xba as libc::c_int as mln_u8_t,
    0x77 as libc::c_int as mln_u8_t,
    0xd6 as libc::c_int as mln_u8_t,
    0x26 as libc::c_int as mln_u8_t,
    0xe1 as libc::c_int as mln_u8_t,
    0x69 as libc::c_int as mln_u8_t,
    0x14 as libc::c_int as mln_u8_t,
    0x63 as libc::c_int as mln_u8_t,
    0x55 as libc::c_int as mln_u8_t,
    0x21 as libc::c_int as mln_u8_t,
    0xc as libc::c_int as mln_u8_t,
    0x7d as libc::c_int as mln_u8_t,
];
static mut rcon: [mln_u32_t; 11] = [
    0 as libc::c_int as mln_u32_t,
    0x1000000 as libc::c_int as mln_u32_t,
    0x2000000 as libc::c_int as mln_u32_t,
    0x4000000 as libc::c_int as mln_u32_t,
    0x8000000 as libc::c_int as mln_u32_t,
    0x10000000 as libc::c_int as mln_u32_t,
    0x20000000 as libc::c_int as mln_u32_t,
    0x40000000 as libc::c_int as mln_u32_t,
    0x80000000 as libc::c_uint,
    0x1b000000 as libc::c_int as mln_u32_t,
    0x36000000 as libc::c_int as mln_u32_t,
];
#[no_mangle]
pub unsafe extern "C" fn mln_aes_init(
    mut a: *mut mln_aes_t,
    mut key: mln_u8ptr_t,
    mut bits: mln_u32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nk: mln_u32_t = 0;
    let mut times: mln_u32_t = 0;
    let mut temp: mln_u32_t = 0;
    let mut roundkey: *mut mln_u32_t = ((*a).w).as_mut_ptr();
    match bits {
        0 => {
            nk = 4 as libc::c_int as mln_u32_t;
            times = ((10 as libc::c_int + 1 as libc::c_int) * 4 as libc::c_int)
                as mln_u32_t;
        }
        1 => {
            nk = 6 as libc::c_int as mln_u32_t;
            times = ((12 as libc::c_int + 1 as libc::c_int) * 4 as libc::c_int)
                as mln_u32_t;
        }
        2 => {
            nk = 8 as libc::c_int as mln_u32_t;
            times = ((14 as libc::c_int + 1 as libc::c_int) * 4 as libc::c_int)
                as mln_u32_t;
        }
        _ => return -(1 as libc::c_int),
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < nk {
        *roundkey.offset(i as isize) = 0 as libc::c_int as mln_u32_t;
        let ref mut fresh1 = *roundkey.offset(i as isize);
        *fresh1
            |= (*key.offset((i << 2 as libc::c_int) as isize) as mln_u32_t
                & 0xff as libc::c_int as libc::c_uint) << 24 as libc::c_int
                | (*key.offset(((i << 2 as libc::c_int) + 1 as libc::c_int) as isize)
                    as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << 16 as libc::c_int
                | (*key.offset(((i << 2 as libc::c_int) + 2 as libc::c_int) as isize)
                    as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << 8 as libc::c_int
                | *key.offset(((i << 2 as libc::c_int) + 3 as libc::c_int) as isize)
                    as mln_u32_t & 0xff as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    while (i as libc::c_uint) < times {
        temp = *roundkey.offset((i - 1 as libc::c_int) as isize);
        if (i as libc::c_uint).wrapping_rem(nk) == 0 as libc::c_int as libc::c_uint {
            temp = (sbox[((temp >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint
                | (temp & 0xffffff as libc::c_int as libc::c_uint) << 8 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as usize] as mln_u32_t
                & 0xff as libc::c_int as libc::c_uint
                | (sbox[((temp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                    | (temp & 0xffffff as libc::c_int as libc::c_uint)
                        << 8 as libc::c_int) >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize] as mln_u32_t
                    & 0xff as libc::c_int as libc::c_uint) << 8 as libc::c_int
                | (sbox[((temp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                    | (temp & 0xffffff as libc::c_int as libc::c_uint)
                        << 8 as libc::c_int) >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize] as mln_u32_t
                    & 0xff as libc::c_int as libc::c_uint) << 16 as libc::c_int
                | (sbox[((temp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                    | (temp & 0xffffff as libc::c_int as libc::c_uint)
                        << 8 as libc::c_int) >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize] as mln_u32_t
                    & 0xff as libc::c_int as libc::c_uint) << 24 as libc::c_int)
                ^ rcon[(i as libc::c_uint).wrapping_div(nk) as usize];
        } else if bits == 2 as libc::c_int as libc::c_uint
            && (i as libc::c_uint).wrapping_rem(nk) == 4 as libc::c_int as libc::c_uint
        {
            temp = sbox[(temp & 0xff as libc::c_int as libc::c_uint) as usize]
                as mln_u32_t & 0xff as libc::c_int as libc::c_uint
                | (sbox[(temp >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << 8 as libc::c_int
                | (sbox[(temp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << 16 as libc::c_int
                | (sbox[(temp >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as usize] as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << 24 as libc::c_int;
        }
        *roundkey
            .offset(
                i as isize,
            ) = *roundkey.offset((i as libc::c_uint).wrapping_sub(nk) as isize) ^ temp;
        i += 1;
        i;
    }
    while i < 60 as libc::c_int {
        *roundkey.offset(i as isize) = 0 as libc::c_int as mln_u32_t;
        i += 1;
        i;
    }
    (*a).bits = bits;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_new(
    mut key: mln_u8ptr_t,
    mut bits: mln_u32_t,
) -> *mut mln_aes_t {
    let mut a: *mut mln_aes_t = malloc(
        ::core::mem::size_of::<mln_aes_t>() as libc::c_ulong,
    ) as *mut mln_aes_t;
    if a.is_null() {
        return 0 as *mut mln_aes_t;
    }
    mln_aes_init(a, key, bits);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_pool_new(
    mut pool: *mut mln_alloc_t,
    mut key: mln_u8ptr_t,
    mut bits: mln_u32_t,
) -> *mut mln_aes_t {
    let mut a: *mut mln_aes_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_aes_t>() as libc::c_ulong,
    ) as *mut mln_aes_t;
    if a.is_null() {
        return 0 as *mut mln_aes_t;
    }
    mln_aes_init(a, key, bits);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_free(mut a: *mut mln_aes_t) {
    if a.is_null() {
        return;
    }
    free(a as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_pool_free(mut a: *mut mln_aes_t) {
    if a.is_null() {
        return;
    }
    mln_alloc_free(a as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_aes_addroundkey(
    mut state: *mut mln_u32_t,
    mut roundkey: *mut mln_u32_t,
    mut round: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: mln_u8_t = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            b = (*state.offset(j as isize)
                >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
            let ref mut fresh2 = *state.offset(j as isize);
            *fresh2
                ^= (b as mln_u32_t)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            b = (b as libc::c_uint
                ^ *roundkey.offset((round * 4 as libc::c_int + i) as isize)
                    >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(j as libc::c_ulong) << 3 as libc::c_int)
                    & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
            let ref mut fresh3 = *state.offset(j as isize);
            *fresh3
                |= (b as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn mln_aes_mixcolume(mut state: *mut mln_u32_t) {
    let mut i: libc::c_int = 0;
    let mut _0: mln_u8_t = 0;
    let mut _1: mln_u8_t = 0;
    let mut _2: mln_u8_t = 0;
    let mut _3: mln_u8_t = 0;
    let mut b: mln_u8_t = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        _0 = (*state.offset(0 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _1 = (*state.offset(1 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _2 = (*state.offset(2 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _3 = (*state.offset(3 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        let ref mut fresh4 = *state.offset(0 as libc::c_int as isize);
        *fresh4
            ^= (_0 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        b = ((_0 as libc::c_int) << 1 as libc::c_int
            ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                * 0x1b as libc::c_int
            ^ ((_1 as libc::c_int) << 1 as libc::c_int
                ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int ^ _1 as libc::c_int) & 0xff as libc::c_int
            ^ _2 as libc::c_int & 0xff as libc::c_int
            ^ _3 as libc::c_int & 0xff as libc::c_int) as mln_u8_t;
        let ref mut fresh5 = *state.offset(0 as libc::c_int as isize);
        *fresh5
            |= (b as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh6 = *state.offset(1 as libc::c_int as isize);
        *fresh6
            ^= (_1 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        b = (_0 as libc::c_int & 0xff as libc::c_int
            ^ ((_1 as libc::c_int) << 1 as libc::c_int
                ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int)
            ^ ((_2 as libc::c_int) << 1 as libc::c_int
                ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int ^ _2 as libc::c_int) & 0xff as libc::c_int
            ^ _3 as libc::c_int & 0xff as libc::c_int) as mln_u8_t;
        let ref mut fresh7 = *state.offset(1 as libc::c_int as isize);
        *fresh7
            |= (b as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh8 = *state.offset(2 as libc::c_int as isize);
        *fresh8
            ^= (_2 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        b = (_0 as libc::c_int & 0xff as libc::c_int
            ^ _1 as libc::c_int & 0xff as libc::c_int
            ^ ((_2 as libc::c_int) << 1 as libc::c_int
                ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int)
            ^ ((_3 as libc::c_int) << 1 as libc::c_int
                ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int ^ _3 as libc::c_int) & 0xff as libc::c_int)
            as mln_u8_t;
        let ref mut fresh9 = *state.offset(2 as libc::c_int as isize);
        *fresh9
            |= (b as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh10 = *state.offset(3 as libc::c_int as isize);
        *fresh10
            ^= (_3 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        b = (((_0 as libc::c_int) << 1 as libc::c_int
            ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                * 0x1b as libc::c_int ^ _0 as libc::c_int) & 0xff as libc::c_int
            ^ _1 as libc::c_int & 0xff as libc::c_int
            ^ _2 as libc::c_int & 0xff as libc::c_int
            ^ ((_3 as libc::c_int) << 1 as libc::c_int
                ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int)) as mln_u8_t;
        let ref mut fresh11 = *state.offset(3 as libc::c_int as isize);
        *fresh11
            |= (b as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn mln_aes_bytesub(mut state: *mut mln_u32_t) {
    let mut i: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut b: mln_u8_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            b = (*state.offset(j as isize)
                >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
            let ref mut fresh12 = *state.offset(j as isize);
            *fresh12
                ^= (b as mln_u32_t)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            let ref mut fresh13 = *state.offset(j as isize);
            *fresh13
                |= (sbox[b as usize] as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn mln_aes_shiftrow(mut state: *mut mln_u32_t) {
    *state
        .offset(
            1 as libc::c_int as isize,
        ) = (*state.offset(1 as libc::c_int as isize) << 8 as libc::c_int
        | *state.offset(1 as libc::c_int as isize) >> 24 as libc::c_int)
        & 0xffffffff as libc::c_uint;
    *state
        .offset(
            2 as libc::c_int as isize,
        ) = (*state.offset(2 as libc::c_int as isize) << 16 as libc::c_int
        | *state.offset(2 as libc::c_int as isize) >> 16 as libc::c_int)
        & 0xffffffff as libc::c_uint;
    *state
        .offset(
            3 as libc::c_int as isize,
        ) = (*state.offset(3 as libc::c_int as isize) << 24 as libc::c_int
        | *state.offset(3 as libc::c_int as isize) >> 8 as libc::c_int)
        & 0xffffffff as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_encrypt(
    mut a: *mut mln_aes_t,
    mut text: mln_u8ptr_t,
) -> libc::c_int {
    let mut state: [mln_u32_t; 4] = [
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
    ];
    let mut i: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut round: mln_u32_t = 0;
    let mut nr: mln_u32_t = 0;
    match (*a).bits {
        0 => {
            nr = 10 as libc::c_int as mln_u32_t;
        }
        1 => {
            nr = 12 as libc::c_int as mln_u32_t;
        }
        2 => {
            nr = 14 as libc::c_int as mln_u32_t;
        }
        _ => return -(1 as libc::c_int),
    }
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            state[j as usize]
                |= ((*text.offset((i << 2 as libc::c_int).wrapping_add(j) as isize)
                    as libc::c_int & 0xff as libc::c_int) as mln_u32_t)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    mln_aes_addroundkey(state.as_mut_ptr(), ((*a).w).as_mut_ptr(), 0 as libc::c_int);
    round = 1 as libc::c_int as mln_u32_t;
    while round < nr {
        mln_aes_bytesub(state.as_mut_ptr());
        mln_aes_shiftrow(state.as_mut_ptr());
        mln_aes_mixcolume(state.as_mut_ptr());
        mln_aes_addroundkey(
            state.as_mut_ptr(),
            ((*a).w).as_mut_ptr(),
            round as libc::c_int,
        );
        round = round.wrapping_add(1);
        round;
    }
    mln_aes_bytesub(state.as_mut_ptr());
    mln_aes_shiftrow(state.as_mut_ptr());
    mln_aes_addroundkey(state.as_mut_ptr(), ((*a).w).as_mut_ptr(), nr as libc::c_int);
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            *text
                .offset(
                    (i << 2 as libc::c_int).wrapping_add(j) as isize,
                ) = (state[j as usize]
                >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_aes_decrypt(
    mut a: *mut mln_aes_t,
    mut cipher: mln_u8ptr_t,
) -> libc::c_int {
    let mut state: [mln_u32_t; 4] = [
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
        0 as libc::c_int as mln_u32_t,
    ];
    let mut i: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut round: mln_u32_t = 0;
    let mut nr: mln_u32_t = 0;
    match (*a).bits {
        0 => {
            nr = 10 as libc::c_int as mln_u32_t;
        }
        1 => {
            nr = 12 as libc::c_int as mln_u32_t;
        }
        2 => {
            nr = 14 as libc::c_int as mln_u32_t;
        }
        _ => return -(1 as libc::c_int),
    }
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            state[j as usize]
                |= ((*cipher.offset((i << 2 as libc::c_int).wrapping_add(j) as isize)
                    as libc::c_int & 0xff as libc::c_int) as mln_u32_t)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    mln_aes_addroundkey(state.as_mut_ptr(), ((*a).w).as_mut_ptr(), nr as libc::c_int);
    round = nr.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while round > 0 as libc::c_int as libc::c_uint {
        mln_aes_invshiftrow(state.as_mut_ptr());
        mln_aes_invbytesub(state.as_mut_ptr());
        mln_aes_addroundkey(
            state.as_mut_ptr(),
            ((*a).w).as_mut_ptr(),
            round as libc::c_int,
        );
        mln_aes_invmixcolume(state.as_mut_ptr());
        round = round.wrapping_sub(1);
        round;
    }
    mln_aes_invshiftrow(state.as_mut_ptr());
    mln_aes_invbytesub(state.as_mut_ptr());
    mln_aes_addroundkey(state.as_mut_ptr(), ((*a).w).as_mut_ptr(), 0 as libc::c_int);
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            *cipher
                .offset(
                    (i << 2 as libc::c_int).wrapping_add(j) as isize,
                ) = (state[j as usize]
                >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_aes_invshiftrow(mut state: *mut mln_u32_t) {
    *state
        .offset(
            1 as libc::c_int as isize,
        ) = (*state.offset(1 as libc::c_int as isize) >> 8 as libc::c_int
        | *state.offset(1 as libc::c_int as isize) << 24 as libc::c_int)
        & 0xffffffff as libc::c_uint;
    *state
        .offset(
            2 as libc::c_int as isize,
        ) = (*state.offset(2 as libc::c_int as isize) >> 16 as libc::c_int
        | *state.offset(2 as libc::c_int as isize) << 16 as libc::c_int)
        & 0xffffffff as libc::c_uint;
    *state
        .offset(
            3 as libc::c_int as isize,
        ) = (*state.offset(3 as libc::c_int as isize) >> 24 as libc::c_int
        | *state.offset(3 as libc::c_int as isize) << 8 as libc::c_int)
        & 0xffffffff as libc::c_uint;
}
#[inline]
unsafe extern "C" fn mln_aes_invbytesub(mut state: *mut mln_u32_t) {
    let mut i: mln_u32_t = 0;
    let mut j: mln_u32_t = 0;
    let mut b: mln_u8_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while (i as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong {
        j = 0 as libc::c_int as mln_u32_t;
        while (j as libc::c_ulong) < ::core::mem::size_of::<mln_u32_t>() as libc::c_ulong
        {
            b = (*state.offset(j as isize)
                >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
            let ref mut fresh14 = *state.offset(j as isize);
            *fresh14
                ^= (b as mln_u32_t)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            let ref mut fresh15 = *state.offset(j as isize);
            *fresh15
                |= (rsbox[b as usize] as mln_u32_t & 0xff as libc::c_int as libc::c_uint)
                    << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn mln_aes_invmixcolume(mut state: *mut mln_u32_t) {
    let mut i: libc::c_int = 0;
    let mut _0: mln_u8_t = 0;
    let mut _1: mln_u8_t = 0;
    let mut _2: mln_u8_t = 0;
    let mut _3: mln_u8_t = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        _0 = (*state.offset(0 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _1 = (*state.offset(1 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _2 = (*state.offset(2 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        _3 = (*state.offset(3 as libc::c_int as isize)
            >> ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_uint) as mln_u8_t;
        let ref mut fresh16 = *state.offset(0 as libc::c_int as isize);
        *fresh16
            ^= (_0 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh17 = *state.offset(0 as libc::c_int as isize);
        *fresh17
            |= ((((((_0 as libc::c_int) << 1 as libc::c_int
                ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ ((((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) >> 7 as libc::c_int
                    & 1 as libc::c_int) * 0x1b as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int)
                ^ ((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int)) & 0xff as libc::c_int
                ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                    ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ ((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) ^ _1 as libc::c_int)
                    & 0xff as libc::c_int
                ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                    ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ _2 as libc::c_int) & 0xff as libc::c_int
                ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                    ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ _3 as libc::c_int) & 0xff as libc::c_int) as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh18 = *state.offset(1 as libc::c_int as isize);
        *fresh18
            ^= (_1 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh19 = *state.offset(1 as libc::c_int as isize);
        *fresh19
            |= ((((((_0 as libc::c_int) << 1 as libc::c_int
                ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ ((((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) >> 7 as libc::c_int
                    & 1 as libc::c_int) * 0x1b as libc::c_int ^ _0 as libc::c_int)
                & 0xff as libc::c_int
                ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                    ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ ((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int)) & 0xff as libc::c_int
                ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                    ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ ((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) ^ _2 as libc::c_int)
                    & 0xff as libc::c_int
                ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                    ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ _3 as libc::c_int) & 0xff as libc::c_int) as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh20 = *state.offset(2 as libc::c_int as isize);
        *fresh20
            ^= (_2 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh21 = *state.offset(2 as libc::c_int as isize);
        *fresh21
            |= ((((((_0 as libc::c_int) << 1 as libc::c_int
                ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ ((((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) >> 7 as libc::c_int
                    & 1 as libc::c_int) * 0x1b as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) ^ _0 as libc::c_int)
                & 0xff as libc::c_int
                ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                    ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ _1 as libc::c_int) & 0xff as libc::c_int
                ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                    ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ ((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int)) & 0xff as libc::c_int
                ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                    ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ ((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) ^ _3 as libc::c_int)
                    & 0xff as libc::c_int) as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh22 = *state.offset(3 as libc::c_int as isize);
        *fresh22
            ^= (_3 as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        let ref mut fresh23 = *state.offset(3 as libc::c_int as isize);
        *fresh23
            |= ((((((_0 as libc::c_int) << 1 as libc::c_int
                ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ (((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) >> 7 as libc::c_int & 1 as libc::c_int)
                    * 0x1b as libc::c_int) << 1 as libc::c_int
                ^ ((((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_0 as libc::c_int) << 1 as libc::c_int
                        ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) >> 7 as libc::c_int
                    & 1 as libc::c_int) * 0x1b as libc::c_int
                ^ ((_0 as libc::c_int) << 1 as libc::c_int
                    ^ (_0 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) ^ _0 as libc::c_int) & 0xff as libc::c_int
                ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                    ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_1 as libc::c_int) << 1 as libc::c_int
                        ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_1 as libc::c_int) << 1 as libc::c_int
                            ^ (_1 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ _1 as libc::c_int) & 0xff as libc::c_int
                ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                    ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_2 as libc::c_int) << 1 as libc::c_int
                        ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_2 as libc::c_int) << 1 as libc::c_int
                            ^ (_2 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ _2 as libc::c_int) & 0xff as libc::c_int
                ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                    ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                        * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) >> 7 as libc::c_int
                        & 1 as libc::c_int) * 0x1b as libc::c_int) << 1 as libc::c_int
                    ^ ((((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                        >> 7 as libc::c_int & 1 as libc::c_int) * 0x1b as libc::c_int
                    ^ (((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int) << 1 as libc::c_int
                        ^ (((_3 as libc::c_int) << 1 as libc::c_int
                            ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                                * 0x1b as libc::c_int) >> 7 as libc::c_int
                            & 1 as libc::c_int) * 0x1b as libc::c_int)
                    ^ ((_3 as libc::c_int) << 1 as libc::c_int
                        ^ (_3 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
                            * 0x1b as libc::c_int)) & 0xff as libc::c_int) as mln_u32_t)
                << ((::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i as libc::c_ulong) << 3 as libc::c_int);
        i += 1;
        i;
    }
}
