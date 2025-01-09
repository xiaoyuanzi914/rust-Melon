use ::libc;
use ::c2rust_bitfields;
use core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn mln_string_buf_new(buf: mln_u8ptr_t, len: mln_u64_t) -> *mut mln_string_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u64_t = libc::c_ulong;
pub type mln_s64_t = libc::c_long;
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
pub struct mln_bignum_t {
    pub tag: mln_u32_t,
    pub length: mln_u32_t,
    pub data: [mln_u64_t; 257],
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
                    current_block = 2748034850436832188;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                2748034850436832188 => {}
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
        current_block_8 = 15746134293738618547;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 15746134293738618547;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        15746134293738618547 => {
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
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_new() -> *mut mln_bignum_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    ) as *mut mln_bignum_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_pool_new(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_bignum_t {
    return mln_alloc_c(pool, ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong)
        as *mut mln_bignum_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_free(mut bn: *mut mln_bignum_t) {
    if bn.is_null() {
        return;
    }
    free(bn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_pool_free(mut bn: *mut mln_bignum_t) {
    if bn.is_null() {
        return;
    }
    mln_alloc_free(bn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_dup(mut bn: *mut mln_bignum_t) -> *mut mln_bignum_t {
    let mut target: *mut mln_bignum_t = malloc(
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    ) as *mut mln_bignum_t;
    if target.is_null() {
        return 0 as *mut mln_bignum_t;
    }
    (*target).tag = (*bn).tag;
    (*target).length = (*bn).length;
    memcpy(
        ((*target).data).as_mut_ptr() as *mut libc::c_void,
        ((*bn).data).as_mut_ptr() as *const libc::c_void,
        ((*bn).length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<mln_u64_t>() as libc::c_ulong),
    );
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_pool_dup(
    mut pool: *mut mln_alloc_t,
    mut bn: *mut mln_bignum_t,
) -> *mut mln_bignum_t {
    let mut target: *mut mln_bignum_t = mln_alloc_m(
        pool,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    ) as *mut mln_bignum_t;
    if target.is_null() {
        return 0 as *mut mln_bignum_t;
    }
    (*target).tag = (*bn).tag;
    (*target).length = (*bn).length;
    memcpy(
        ((*target).data).as_mut_ptr() as *mut libc::c_void,
        ((*bn).data).as_mut_ptr() as *const libc::c_void,
        ((*bn).length as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<mln_u64_t>() as libc::c_ulong),
    );
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_assign(
    mut bn: *mut mln_bignum_t,
    mut sval: mln_s8ptr_t,
    mut len: mln_u32_t,
) -> libc::c_int {
    let mut tag: mln_u32_t = 0;
    if *sval.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        tag = 1 as libc::c_int as mln_u32_t;
        sval = sval.offset(1);
        sval;
        len = len.wrapping_sub(1);
        len;
    } else {
        tag = 0 as libc::c_int as mln_u32_t;
    }
    if *sval.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        if *sval.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32 {
            return mln_bignum_assign_hex(
                bn,
                sval.offset(2 as libc::c_int as isize),
                tag,
                len.wrapping_sub(2 as libc::c_int as libc::c_uint),
            )
        } else {
            return mln_bignum_assign_oct(
                bn,
                sval.offset(1 as libc::c_int as isize),
                tag,
                len.wrapping_sub(1 as libc::c_int as libc::c_uint),
            )
        }
    }
    return mln_bignum_assign_dec(bn, sval, tag, len);
}
#[inline]
unsafe extern "C" fn mln_bignum_assign_hex(
    mut bn: *mut mln_bignum_t,
    mut sval: mln_s8ptr_t,
    mut tag: mln_u32_t,
    mut len: mln_u32_t,
) -> libc::c_int {
    if len > (8224 as libc::c_int / 4 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    let mut p: mln_s8ptr_t = sval
        .offset(len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut i: mln_s32_t = 0;
    let mut j: mln_s32_t = 0 as libc::c_int;
    let mut l: mln_s32_t = 0 as libc::c_int;
    let mut b: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    let mut tmp: *mut mln_u64_t = ((*bn).data).as_mut_ptr();
    memset(
        bn as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    );
    (*bn).tag = 0 as libc::c_int as mln_u32_t;
    i = 0 as libc::c_int;
    while p >= sval {
        if i % 2 as libc::c_int != 0 {
            if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
                b = (b as libc::c_int
                    | (*p as libc::c_int - '0' as i32) << 4 as libc::c_int
                        & 0xf0 as libc::c_int) as mln_u8_t;
            } else if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'f' as i32
            {
                b = (b as libc::c_int
                    | (*p as libc::c_int - 'a' as i32 + 10 as libc::c_int)
                        << 4 as libc::c_int & 0xf0 as libc::c_int) as mln_u8_t;
            } else if *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'F' as i32
            {
                b = (b as libc::c_int
                    | (*p as libc::c_int - 'A' as i32 + 10 as libc::c_int)
                        << 4 as libc::c_int & 0xf0 as libc::c_int) as mln_u8_t;
            } else {
                return -(1 as libc::c_int)
            }
        } else {
            b = 0 as libc::c_int as mln_u8_t;
            if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
                b = (b as libc::c_int
                    | *p as libc::c_int - '0' as i32 & 0xf as libc::c_int) as mln_u8_t;
            } else if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'f' as i32
            {
                b = (b as libc::c_int
                    | *p as libc::c_int - 'a' as i32 + 10 as libc::c_int
                        & 0xf as libc::c_int) as mln_u8_t;
            } else if *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'F' as i32
            {
                b = (b as libc::c_int
                    | *p as libc::c_int - 'A' as i32 + 10 as libc::c_int
                        & 0xf as libc::c_int) as mln_u8_t;
            } else {
                return -(1 as libc::c_int)
            }
        }
        if i % 2 as libc::c_int != 0 || p == sval {
            let ref mut fresh1 = *tmp.offset(l as isize);
            *fresh1 |= (b as mln_u64_t) << (j << 3 as libc::c_int);
            if j % 4 as libc::c_int == 3 as libc::c_int {
                l += 1;
                l;
                j = 0 as libc::c_int;
            } else {
                j += 1;
                j;
            }
        }
        p = p.offset(-1);
        p;
        i += 1;
        i;
    }
    (*bn).tag = tag;
    i = 257 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *tmp.offset(i as isize) != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        i -= 1;
        i;
    }
    (*bn).length = (i + 1 as libc::c_int) as mln_u32_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_bignum_assign_oct(
    mut bn: *mut mln_bignum_t,
    mut sval: mln_s8ptr_t,
    mut tag: mln_u32_t,
    mut len: mln_u32_t,
) -> libc::c_int {
    if len > (8224 as libc::c_int / 3 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    let mut p: mln_s8ptr_t = sval
        .offset(len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut tmp: *mut mln_u64_t = ((*bn).data).as_mut_ptr();
    let mut j: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut l: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut i: mln_s32_t = 0;
    let mut b: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    memset(
        bn as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    );
    (*bn).tag = 0 as libc::c_int as mln_u32_t;
    i = 0 as libc::c_int;
    while p >= sval {
        if i % 3 as libc::c_int == 0 as libc::c_int {
            b = 0 as libc::c_int as mln_u8_t;
            if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
                return -(1 as libc::c_int);
            }
            b = (b as libc::c_int | *p as libc::c_int - '0' as i32 & 0x7 as libc::c_int)
                as mln_u8_t;
        } else if i % 3 as libc::c_int == 1 as libc::c_int {
            if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
                return -(1 as libc::c_int);
            }
            b = (b as libc::c_int
                | (*p as libc::c_int - '0' as i32) << 3 as libc::c_int
                    & 0x38 as libc::c_int) as mln_u8_t;
        } else {
            if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '3' as i32 {
                return -(1 as libc::c_int);
            }
            b = (b as libc::c_int
                | (*p as libc::c_int - '0' as i32) << 6 as libc::c_int
                    & 0xc0 as libc::c_int) as mln_u8_t;
        }
        if i % 3 as libc::c_int == 2 as libc::c_int || p == sval {
            let ref mut fresh2 = *tmp.offset(l as isize);
            *fresh2 |= (b as mln_u64_t) << (j << 3 as libc::c_int);
            if j.wrapping_rem(4 as libc::c_int as libc::c_uint)
                == 3 as libc::c_int as libc::c_uint
            {
                l = l.wrapping_add(1);
                l;
                j = 0 as libc::c_int as mln_u32_t;
            } else {
                j = j.wrapping_add(1);
                j;
            }
        }
        p = p.offset(-1);
        p;
        i += 1;
        i;
    }
    (*bn).tag = tag;
    i = 257 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *tmp.offset(i as isize) != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        i -= 1;
        i;
    }
    (*bn).length = (i + 1 as libc::c_int) as mln_u32_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_bignum_assign_dec(
    mut bn: *mut mln_bignum_t,
    mut sval: mln_s8ptr_t,
    mut tag: mln_u32_t,
    mut len: mln_u32_t,
) -> libc::c_int {
    if len > (8224 as libc::c_int / 4 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    let mut p: mln_s8ptr_t = sval
        .offset(len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut cnt: mln_u32_t = 0;
    let mut tmp: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    memset(
        bn as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    );
    (*bn).tag = 0 as libc::c_int as mln_u32_t;
    cnt = 0 as libc::c_int as mln_u32_t;
    while p >= sval {
        if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32) {
            return -(1 as libc::c_int);
        }
        memset(
            &mut tmp as *mut mln_bignum_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        tmp.tag = 0 as libc::c_int as mln_u32_t;
        mln_bignum_dec_recursive(
            cnt,
            (*p as libc::c_int - '0' as i32) as mln_u32_t,
            &mut tmp,
        );
        __mln_bignum_add(bn, &mut tmp);
        p = p.offset(-1);
        p;
        cnt = cnt.wrapping_add(1);
        cnt;
    }
    (*bn).tag = tag;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_bignum_dec_recursive(
    mut rec_times: mln_u32_t,
    mut loop_times: mln_u32_t,
    mut tmp: *mut mln_bignum_t,
) {
    if rec_times == 0 {
        let mut one: mln_bignum_t = {
            let mut init = mln_bignum_t {
                tag: 0 as libc::c_int as mln_u32_t,
                length: 1 as libc::c_int as mln_u32_t,
                data: [
                    0 as libc::c_int as mln_u64_t,
                ],
            };
            init
        };
        one.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
        memset(
            tmp as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        while loop_times > 0 as libc::c_int as libc::c_uint {
            __mln_bignum_add(tmp, &mut one);
            loop_times = loop_times.wrapping_sub(1);
            loop_times;
        }
    } else {
        let mut val: mln_bignum_t = mln_bignum_t {
            tag: 0,
            length: 0,
            data: [0; 257],
        };
        memset(
            &mut val as *mut mln_bignum_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        val.tag = 0 as libc::c_int as mln_u32_t;
        mln_bignum_dec_recursive(
            rec_times.wrapping_sub(1 as libc::c_int as libc::c_uint),
            10 as libc::c_int as mln_u32_t,
            &mut val,
        );
        while loop_times > 0 as libc::c_int as libc::c_uint {
            __mln_bignum_add(tmp, &mut val);
            loop_times = loop_times.wrapping_sub(1);
            loop_times;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_add(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    __mln_bignum_add(dest, src);
}
#[inline]
unsafe extern "C" fn __mln_bignum_add(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    if (*dest).tag != (*src).tag {
        if (*dest).tag == 1 as libc::c_int as libc::c_uint {
            let mut tmp: mln_bignum_t = *src;
            (*dest).tag = 0 as libc::c_int as mln_u32_t;
            __mln_bignum_sub(&mut tmp, dest);
            *dest = tmp;
        } else {
            (*src).tag = 0 as libc::c_int as mln_u32_t;
            __mln_bignum_sub(dest, src);
            (*src).tag = 1 as libc::c_int as mln_u32_t;
        }
        return;
    }
    let mut carry: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut dest_data: *mut mln_u64_t = ((*dest).data).as_mut_ptr();
    let mut max: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut min: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut end: *mut mln_u64_t = 0 as *mut mln_u64_t;
    if (*dest).length < (*src).length {
        max = ((*src).data).as_mut_ptr();
        min = ((*dest).data).as_mut_ptr();
        end = max.offset((*src).length as isize);
    } else {
        max = ((*dest).data).as_mut_ptr();
        min = ((*src).data).as_mut_ptr();
        end = max.offset((*dest).length as isize);
    }
    while max < end {
        *dest_data = (*max).wrapping_add(*min).wrapping_add(carry);
        carry = 0 as libc::c_int as mln_u64_t;
        if *dest_data >= 0x100000000 as libc::c_ulonglong as mln_u64_t {
            *dest_data = (*dest_data as libc::c_ulong)
                .wrapping_sub(0x100000000 as libc::c_ulonglong as mln_u64_t) as mln_u64_t
                as mln_u64_t;
            carry = 1 as libc::c_int as mln_u64_t;
        }
        max = max.offset(1);
        max;
        min = min.offset(1);
        min;
        dest_data = dest_data.offset(1);
        dest_data;
    }
    if carry != 0
        && dest_data < ((*dest).data).as_mut_ptr().offset(257 as libc::c_int as isize)
    {
        let fresh3 = dest_data;
        dest_data = dest_data.offset(1);
        *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(carry) as mln_u64_t
            as mln_u64_t;
        carry = 0 as libc::c_int as mln_u64_t;
    }
    (*dest)
        .length = dest_data.offset_from(((*dest).data).as_mut_ptr()) as libc::c_long
        as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_sub(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    __mln_bignum_sub(dest, src);
}
#[inline]
unsafe extern "C" fn __mln_bignum_sub(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    if (*dest).tag != (*src).tag {
        if (*dest).tag == 1 as libc::c_int as libc::c_uint {
            (*src).tag = 1 as libc::c_int as mln_u32_t;
            __mln_bignum_add(dest, src);
            (*src).tag = 0 as libc::c_int as mln_u32_t;
        } else {
            (*src).tag = 0 as libc::c_int as mln_u32_t;
            __mln_bignum_add(dest, src);
            (*src).tag = 1 as libc::c_int as mln_u32_t;
        }
        return;
    }
    let mut ret: libc::c_int = 0;
    ret = __mln_bignum_compare(dest, src);
    if ret == 0 as libc::c_int {
        memset(
            dest as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        (*dest).tag = 0 as libc::c_int as mln_u32_t;
    } else if ret < 0 as libc::c_int {
        if (*dest).tag == 1 as libc::c_int as libc::c_uint {
            (*dest).tag = 0 as libc::c_int as mln_u32_t;
            (*src).tag = 0 as libc::c_int as mln_u32_t;
            __mln_bignum_sub_core(dest, src);
            (*dest).tag = 1 as libc::c_int as mln_u32_t;
            (*src).tag = 1 as libc::c_int as mln_u32_t;
        } else {
            let mut tmp: mln_bignum_t = *src;
            __mln_bignum_sub_core(&mut tmp, dest);
            *dest = tmp;
            (*dest).tag = 1 as libc::c_int as mln_u32_t;
        }
    } else if (*dest).tag == 1 as libc::c_int as libc::c_uint {
        let mut tmp_0: mln_bignum_t = *src;
        (*dest).tag = 0 as libc::c_int as mln_u32_t;
        tmp_0.tag = 0 as libc::c_int as mln_u32_t;
        __mln_bignum_sub_core(&mut tmp_0, dest);
        *dest = tmp_0;
        (*dest).tag = 0 as libc::c_int as mln_u32_t;
    } else {
        __mln_bignum_sub_core(dest, src);
    };
}
#[inline]
unsafe extern "C" fn __mln_bignum_sub_core(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    let mut borrow: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut dest_data: *mut mln_u64_t = ((*dest).data).as_mut_ptr();
    let mut src_data: *mut mln_u64_t = ((*src).data).as_mut_ptr();
    let mut end: *mut mln_u64_t = ((*dest).data)
        .as_mut_ptr()
        .offset((*dest).length as isize);
    while dest_data < end {
        if (*src_data).wrapping_add(borrow as libc::c_ulong) > *dest_data {
            *dest_data = (*dest_data)
                .wrapping_add(0x100000000 as libc::c_ulonglong as mln_u64_t)
                .wrapping_sub((*src_data).wrapping_add(borrow as libc::c_ulong));
            borrow = 1 as libc::c_int as mln_u32_t;
        } else {
            *dest_data = (*dest_data as libc::c_ulong)
                .wrapping_sub((*src_data).wrapping_add(borrow as libc::c_ulong))
                as mln_u64_t as mln_u64_t;
            borrow = 0 as libc::c_int as mln_u32_t;
        }
        dest_data = dest_data.offset(1);
        dest_data;
        src_data = src_data.offset(1);
        src_data;
    }
    if borrow == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"borrow == 0\0" as *const u8 as *const libc::c_char,
            b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
            383 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void __mln_bignum_sub_core(mln_bignum_t *, mln_bignum_t *)\0"))
                .as_ptr(),
        );
    }
    'c_9840: {
        if borrow == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"borrow == 0\0" as *const u8 as *const libc::c_char,
                b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"void __mln_bignum_sub_core(mln_bignum_t *, mln_bignum_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    dest_data = ((*dest).data).as_mut_ptr();
    end = end.offset(-1);
    end;
    while end >= dest_data {
        if *end != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        end = end.offset(-1);
        end;
    }
    (*dest)
        .length = (if end < dest_data {
        0 as libc::c_int as libc::c_long
    } else {
        end.offset_from(dest_data) as libc::c_long + 1 as libc::c_int as libc::c_long
    }) as mln_u32_t;
}
#[inline]
unsafe extern "C" fn __mln_bignum_mul(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    let mut tag: mln_u32_t = (*dest).tag ^ (*src).tag;
    if (*src).length == 1 as libc::c_int as libc::c_uint {
        __mln_bignum_mul_word(dest, (*src).data[0 as libc::c_int as usize] as mln_s64_t);
        (*dest).tag = tag;
        return;
    } else if (*dest).length == 1 as libc::c_int as libc::c_uint {
        let mut tmp: mln_bignum_t = *src;
        __mln_bignum_mul_word(
            &mut tmp,
            (*dest).data[0 as libc::c_int as usize] as mln_s64_t,
        );
        *dest = tmp;
        (*dest).tag = tag;
        return;
    }
    let mut res: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 0 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    if __mln_bignum_abs_compare(dest, &mut res) == 0
        || __mln_bignum_abs_compare(src, &mut res) == 0
    {
        *dest = res;
        return;
    }
    let mut data: *mut mln_u64_t = (res.data).as_mut_ptr();
    let mut tmp_0: mln_u64_t = 0;
    let mut dest_data: *mut mln_u64_t = ((*dest).data).as_mut_ptr();
    let mut src_data: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut dend: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut send: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut last: *mut mln_u64_t = (res.data)
        .as_mut_ptr()
        .offset(257 as libc::c_int as isize);
    dend = ((*dest).data).as_mut_ptr().offset((*dest).length as isize);
    while dest_data < dend {
        src_data = ((*src).data).as_mut_ptr();
        send = ((*src).data).as_mut_ptr().offset((*src).length as isize);
        data = (res.data)
            .as_mut_ptr()
            .offset(
                dest_data.offset_from(((*dest).data).as_mut_ptr()) as libc::c_long
                    as isize,
            );
        if !(data >= last) {
            while src_data < send && data < last {
                tmp_0 = (*dest_data).wrapping_mul(*src_data).wrapping_add(*data);
                *data = tmp_0
                    .wrapping_rem(0x100000000 as libc::c_ulonglong as mln_u64_t);
                if data.offset(1 as libc::c_int as isize) < last {
                    let ref mut fresh4 = *data.offset(1 as libc::c_int as isize);
                    *fresh4 = (*fresh4 as libc::c_ulong)
                        .wrapping_add(tmp_0 >> 32 as libc::c_int) as mln_u64_t
                        as mln_u64_t;
                }
                src_data = src_data.offset(1);
                src_data;
                data = data.offset(1);
                data;
            }
        }
        dest_data = dest_data.offset(1);
        dest_data;
    }
    res.length = data.offset_from((res.data).as_mut_ptr()) as libc::c_long as mln_u32_t;
    if data < last && *data != 0 as libc::c_int as libc::c_ulong {
        res.length = (res.length).wrapping_add(1);
        res.length;
    }
    res.tag = tag;
    *dest = res;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_mul(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
) {
    __mln_bignum_mul(dest, src);
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_div(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
    mut quotient: *mut mln_bignum_t,
) -> libc::c_int {
    return __mln_bignum_div(dest, src, quotient);
}
#[inline]
unsafe extern "C" fn __mln_bignum_nmul(
    mut a: *mut mln_u64_t,
    mut b: mln_u64_t,
    mut r: *mut mln_u64_t,
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    b &= 0xffffffff as libc::c_uint as libc::c_ulong;
    if b == 0 as libc::c_int as libc::c_ulong {
        memset(
            r as *mut libc::c_void,
            0 as libc::c_int,
            (n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<mln_u64_t>() as libc::c_ulong),
        );
        return 0 as libc::c_int;
    }
    if b == 1 as libc::c_int as libc::c_ulong {
        memcpy(
            r as *mut libc::c_void,
            a as *const libc::c_void,
            (::core::mem::size_of::<mln_u64_t>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        );
        return 0 as libc::c_int;
    }
    let mut c: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    n -= m;
    while m > 0 as libc::c_int {
        *r = (*a).wrapping_mul(b).wrapping_add(c);
        c = *r >> 32 as libc::c_int;
        *r = (*r as libc::c_ulong)
            .wrapping_rem(0x100000000 as libc::c_ulonglong as mln_u64_t) as mln_u64_t
            as mln_u64_t;
        m -= 1;
        m;
        a = a.offset(1);
        a;
        r = r.offset(1);
        r;
    }
    while n > 0 as libc::c_int {
        *r = c.wrapping_rem(0x100000000 as libc::c_ulonglong as mln_u64_t);
        c >>= 32 as libc::c_int;
        n -= 1;
        n;
        r = r.offset(1);
        r;
    }
    return (c > 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __mln_bignum_nsbb(
    mut a: *mut mln_u64_t,
    mut b: *mut mln_u64_t,
    mut r: *mut mln_u64_t,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0 as libc::c_int;
    while n > 0 as libc::c_int {
        if *a < (*b).wrapping_add(c as libc::c_ulong) {
            *r = (*a)
                .wrapping_add(0x100000000 as libc::c_ulonglong as mln_u64_t)
                .wrapping_sub(*b)
                .wrapping_sub(c as libc::c_ulong);
            c = 1 as libc::c_int;
        } else {
            *r = (*a).wrapping_sub(*b).wrapping_sub(c as libc::c_ulong);
            c = 0 as libc::c_int;
        }
        n -= 1;
        n;
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
        r = r.offset(1);
        r;
    }
    return c;
}
#[inline]
unsafe extern "C" fn __mln_bignum_nadc(
    mut a: *mut mln_u64_t,
    mut b: *mut mln_u64_t,
    mut r: *mut mln_u64_t,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0 as libc::c_int;
    while n > 0 as libc::c_int {
        *r = (*a).wrapping_add(*b).wrapping_add(c as libc::c_ulong);
        if *r >= 0x100000000 as libc::c_ulonglong as mln_u64_t {
            *r = (*r as libc::c_ulong)
                .wrapping_sub(0x100000000 as libc::c_ulonglong as mln_u64_t) as mln_u64_t
                as mln_u64_t;
            c = 1 as libc::c_int;
        } else {
            c = 0 as libc::c_int;
        }
        n -= 1;
        n;
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
        r = r.offset(1);
        r;
    }
    return c;
}
#[inline]
unsafe extern "C" fn __mln_bignum_ndiv(
    mut a: *mut mln_u64_t,
    mut b: mln_u64_t,
    mut r: *mut mln_u64_t,
    mut n: libc::c_int,
) -> libc::c_int {
    b &= 0xffffffff as libc::c_uint as libc::c_ulong;
    if b == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if b == 1 as libc::c_int as libc::c_ulong {
        memcpy(
            r as *mut libc::c_void,
            a as *const libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<mln_u64_t>() as libc::c_ulong),
        );
        let mut data: *mut mln_u64_t = r
            .offset(n as isize)
            .offset(-(1 as libc::c_int as isize));
        while data >= r && *data == 0 as libc::c_int as libc::c_ulong {
            data = data.offset(-1);
            data;
        }
        return (if data < r {
            0 as libc::c_int as libc::c_long
        } else {
            data.offset_from(r) as libc::c_long + 1 as libc::c_int as libc::c_long
        }) as libc::c_int;
    }
    let mut dest_data: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut data_0: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut tmp: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    dest_data = a.offset(n as isize).offset(-(1 as libc::c_int as isize));
    data_0 = r.offset(n as isize).offset(-(1 as libc::c_int as isize));
    let mut len: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    while dest_data >= a {
        tmp = (tmp as libc::c_ulong).wrapping_add(*dest_data) as mln_u64_t as mln_u64_t;
        *data_0 = tmp.wrapping_div(b);
        if len == 0 && *data_0 != 0 {
            len = (data_0.offset_from(r) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as mln_u32_t;
        }
        data_0 = data_0.offset(-1);
        data_0;
        tmp = (tmp as libc::c_ulong).wrapping_rem(b) as mln_u64_t as mln_u64_t;
        tmp <<= 32 as libc::c_int;
        dest_data = dest_data.offset(-1);
        dest_data;
    }
    return len as libc::c_int;
}
#[inline]
unsafe extern "C" fn __mln_bignum_div(
    mut dest: *mut mln_bignum_t,
    mut src: *mut mln_bignum_t,
    mut quotient: *mut mln_bignum_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tag: mln_u32_t = (*dest).tag ^ (*src).tag;
    if (*src).length == 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    if (*src).length == 1 as libc::c_int as libc::c_uint {
        let mut ret_0: libc::c_int = __mln_bignum_div_word(
            dest,
            (*src).data[0 as libc::c_int as usize] as mln_s64_t,
            quotient,
        );
        if ret_0 < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*dest).tag = tag;
        if !quotient.is_null() {
            (*quotient).tag = tag;
        }
        return 0 as libc::c_int;
    }
    ret = __mln_bignum_abs_compare(dest, src);
    if ret == 0 as libc::c_int {
        memset(
            dest as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        if !quotient.is_null() {
            memset(
                quotient as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
            );
            (*quotient).tag = tag;
            (*quotient).length = 1 as libc::c_int as mln_u32_t;
            (*quotient).data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
        }
        return 0 as libc::c_int;
    } else if ret < 0 as libc::c_int {
        (*dest).tag = tag;
        if !quotient.is_null() {
            memset(
                quotient as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
            );
        }
        return 0 as libc::c_int;
    }
    let mut ddata: [mln_u64_t; 258] = [0; 258];
    let mut sdata: [mln_u64_t; 258] = [0; 258];
    let mut tmp: [mln_u64_t; 258] = [0; 258];
    let mut pd: *mut mln_u64_t = ddata.as_mut_ptr();
    let mut ps: *mut mln_u64_t = sdata.as_mut_ptr();
    let mut pr: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut q: mln_u64_t = 0;
    let mut r: mln_u64_t = 0;
    let mut d: mln_u64_t = 0;
    let mut t: mln_u64_t = 0;
    let mut v_n: mln_u64_t = 0;
    let mut dlen: libc::c_int = (*dest).length as libc::c_int;
    let mut slen: libc::c_int = (*src).length as libc::c_int;
    let mut j: libc::c_int = 0;
    if !quotient.is_null() {
        memset(
            quotient as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        pr = &mut *((*quotient).data).as_mut_ptr().offset((dlen - slen) as isize)
            as *mut mln_u64_t;
    }
    sdata[slen as usize] = 0 as libc::c_int as mln_u64_t;
    ddata[dlen as usize] = sdata[slen as usize];
    d = (0x100000000 as libc::c_ulonglong as mln_u64_t)
        .wrapping_div(
            ((*src).data[(slen - 1 as libc::c_int) as usize])
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    __mln_bignum_nmul(((*dest).data).as_mut_ptr(), d, pd, dlen + 1 as libc::c_int, dlen);
    __mln_bignum_nmul(((*src).data).as_mut_ptr(), d, ps, slen, slen);
    v_n = *ps.offset((slen - 1 as libc::c_int) as isize);
    j = dlen - slen;
    while j >= 0 as libc::c_int {
        t = (*pd.offset((j + slen) as isize) << 32 as libc::c_int)
            .wrapping_add(*pd.offset((j + slen - 1 as libc::c_int) as isize));
        q = t.wrapping_div(v_n);
        r = t.wrapping_rem(v_n);
        t = *pd.offset((j + slen - 2 as libc::c_int) as isize);
        while q == 0x100000000 as libc::c_ulonglong as mln_u64_t
            || q.wrapping_mul(*ps.offset((slen - 2 as libc::c_int) as isize))
                > (r << 32 as libc::c_int).wrapping_add(t)
        {
            q = q.wrapping_sub(1);
            q;
            r = (r as libc::c_ulong).wrapping_add(v_n) as mln_u64_t as mln_u64_t;
            if r >= 0x100000000 as libc::c_ulonglong as mln_u64_t {
                break;
            }
        }
        __mln_bignum_nmul(
            ps,
            q,
            tmp.as_mut_ptr(),
            slen + 1 as libc::c_int,
            slen + 1 as libc::c_int,
        );
        if __mln_bignum_nsbb(
            pd.offset(j as isize),
            tmp.as_mut_ptr(),
            pd.offset(j as isize),
            slen + 1 as libc::c_int,
        ) != 0
        {
            q = q.wrapping_sub(1);
            q;
            __mln_bignum_nadc(
                pd.offset(j as isize),
                ps,
                pd.offset(j as isize),
                slen + 1 as libc::c_int,
            );
        }
        if !pr.is_null() {
            let fresh5 = pr;
            pr = pr.offset(-1);
            *fresh5 = q;
        }
        j -= 1;
        j;
    }
    if !quotient.is_null() {
        j = dlen - slen;
        (*quotient)
            .length = (if *((*quotient).data).as_mut_ptr().offset(j as isize) != 0 {
            j + 1 as libc::c_int
        } else {
            j
        }) as mln_u32_t;
        (*quotient).tag = tag;
    }
    memset(
        dest as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    );
    (*dest)
        .length = __mln_bignum_ndiv(
        ddata.as_mut_ptr(),
        d,
        ((*dest).data).as_mut_ptr(),
        slen,
    ) as mln_u32_t;
    (*dest).tag = tag;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_pwr(
    mut dest: *mut mln_bignum_t,
    mut exponent: *mut mln_bignum_t,
    mut mod_0: *mut mln_bignum_t,
) -> libc::c_int {
    return __mln_bignum_pwr(dest, exponent, mod_0);
}
#[inline]
unsafe extern "C" fn __mln_bignum_pwr(
    mut dest: *mut mln_bignum_t,
    mut exponent: *mut mln_bignum_t,
    mut mod_0: *mut mln_bignum_t,
) -> libc::c_int {
    if (*exponent).tag == 1 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    let mut x: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut i: mln_s32_t = 0;
    i = ((*exponent).length << 5 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as mln_s32_t;
    while i >= 0 as libc::c_int {
        if __mln_bignum_bit_test(exponent, i as mln_u32_t) != 0 {
            break;
        }
        i -= 1;
        i;
    }
    if i < 0 as libc::c_int {
        memset(
            dest as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        (*dest).tag = 0 as libc::c_int as mln_u32_t;
        (*dest).length = 1 as libc::c_int as mln_u32_t;
        (*dest).data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
        return 0 as libc::c_int;
    }
    x = *dest;
    if !mod_0.is_null() {
        i -= 1;
        i;
        while i >= 0 as libc::c_int {
            __mln_bignum_mul(dest, dest);
            if (*dest).length >= (*mod_0).length {
                if __mln_bignum_div(dest, mod_0, 0 as *mut mln_bignum_t)
                    < 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
            }
            if __mln_bignum_bit_test(exponent, i as mln_u32_t) != 0 {
                __mln_bignum_mul(dest, &mut x);
                if (*dest).length >= (*mod_0).length {
                    if __mln_bignum_div(dest, mod_0, 0 as *mut mln_bignum_t)
                        < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                }
            }
            i -= 1;
            i;
        }
    } else {
        i -= 1;
        i;
        while i >= 0 as libc::c_int {
            __mln_bignum_mul(dest, dest);
            if __mln_bignum_bit_test(exponent, i as mln_u32_t) != 0 {
                __mln_bignum_mul(dest, &mut x);
            }
            i -= 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __mln_bignum_abs_compare(
    mut bn1: *mut mln_bignum_t,
    mut bn2: *mut mln_bignum_t,
) -> libc::c_int {
    if (*bn1).length != (*bn2).length {
        if (*bn1).length > (*bn2).length {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if (*bn1).length == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    let mut data1: *mut mln_u64_t = ((*bn1).data)
        .as_mut_ptr()
        .offset((*bn1).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut data2: *mut mln_u64_t = ((*bn2).data)
        .as_mut_ptr()
        .offset((*bn2).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut end: *mut mln_u64_t = ((*bn1).data).as_mut_ptr();
    while data1 >= end {
        if *data1 > *data2 {
            return 1 as libc::c_int
        } else if *data1 < *data2 {
            return -(1 as libc::c_int)
        }
        data1 = data1.offset(-1);
        data1;
        data2 = data2.offset(-1);
        data2;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_abs_compare(
    mut bn1: *mut mln_bignum_t,
    mut bn2: *mut mln_bignum_t,
) -> libc::c_int {
    return __mln_bignum_abs_compare(bn1, bn2);
}
#[inline]
unsafe extern "C" fn __mln_bignum_compare(
    mut bn1: *mut mln_bignum_t,
    mut bn2: *mut mln_bignum_t,
) -> libc::c_int {
    if (*bn1).length == (*bn2).length
        && (*bn1).length == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*bn1).tag != (*bn2).tag {
        if (*bn1).tag == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if (*bn1).length != (*bn2).length {
        if (*bn1).tag == 0 as libc::c_int as libc::c_uint {
            if (*bn1).length > (*bn2).length {
                return 1 as libc::c_int;
            }
            return -(1 as libc::c_int);
        } else {
            if (*bn1).length > (*bn2).length {
                return -(1 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
    }
    let mut data1: *mut mln_u64_t = ((*bn1).data)
        .as_mut_ptr()
        .offset((*bn1).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut data2: *mut mln_u64_t = ((*bn2).data)
        .as_mut_ptr()
        .offset((*bn2).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut end: *mut mln_u64_t = ((*bn1).data).as_mut_ptr();
    while data1 >= end {
        if *data1 > *data2 {
            return 1 as libc::c_int
        } else if *data1 < *data2 {
            return -(1 as libc::c_int)
        }
        data1 = data1.offset(-1);
        data1;
        data2 = data2.offset(-1);
        data2;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_compare(
    mut bn1: *mut mln_bignum_t,
    mut bn2: *mut mln_bignum_t,
) -> libc::c_int {
    return __mln_bignum_compare(bn1, bn2);
}
#[inline]
unsafe extern "C" fn __mln_bignum_bit_test(
    mut bn: *mut mln_bignum_t,
    mut index: mln_u32_t,
) -> libc::c_int {
    return if (*bn).data[index.wrapping_div(32 as libc::c_int as libc::c_uint) as usize]
        & (1 as libc::c_int as mln_u64_t)
            << index.wrapping_rem(32 as libc::c_int as libc::c_uint) != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_bit_test(
    mut bn: *mut mln_bignum_t,
    mut index: mln_u32_t,
) -> libc::c_int {
    if index >= 8224 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    return __mln_bignum_bit_test(bn, index);
}
#[inline]
unsafe extern "C" fn __mln_bignum_left_shift(
    mut bn: *mut mln_bignum_t,
    mut n: mln_u32_t,
) {
    if n == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut step: mln_u32_t = n.wrapping_div(32 as libc::c_int as libc::c_uint);
    let mut off: mln_u32_t = n.wrapping_rem(32 as libc::c_int as libc::c_uint);
    let mut len: mln_u32_t = 0;
    if step >= 257 as libc::c_int as libc::c_uint {
        return;
    }
    let mut s: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut d: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut end: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut cur: mln_u64_t = 0;
    let mut last: *mut mln_u64_t = 0 as *mut mln_u64_t;
    end = ((*bn).data).as_mut_ptr();
    s = ((*bn).data)
        .as_mut_ptr()
        .offset((*bn).length as isize)
        .offset(-(1 as libc::c_int as isize));
    if ((*bn).length).wrapping_add(step) > 257 as libc::c_int as libc::c_uint {
        s = s
            .offset(
                -(((*bn).length)
                    .wrapping_add(step)
                    .wrapping_sub(257 as libc::c_int as libc::c_uint) as isize),
            );
    }
    d = s.offset(step as isize);
    len = d.offset_from(((*bn).data).as_mut_ptr()) as libc::c_long as mln_u32_t;
    if off != 0 {
        let mut tmp: mln_u64_t = 0 as libc::c_int as mln_u64_t;
        last = if d
            < ((*bn).data)
                .as_mut_ptr()
                .offset(257 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            len = len.wrapping_add(1);
            len;
            d.offset(1 as libc::c_int as isize)
        } else {
            &mut tmp
        };
        while s >= end {
            *d = *s;
            cur = *d;
            *last
                |= cur >> (32 as libc::c_int as libc::c_uint).wrapping_sub(off)
                    & 0xffffffff as libc::c_uint as libc::c_ulong;
            *d = *d << off & 0xffffffff as libc::c_uint as libc::c_ulong;
            last = d;
            s = s.offset(-1);
            s;
            d = d.offset(-1);
            d;
        }
        while d >= end {
            *d = 0 as libc::c_int as mln_u64_t;
            d = d.offset(-1);
            d;
        }
    } else {
        while s >= end {
            *d = *s;
            s = s.offset(-1);
            s;
            d = d.offset(-1);
            d;
        }
        while d >= end {
            *d = 0 as libc::c_int as mln_u64_t;
            d = d.offset(-1);
            d;
        }
    }
    d = ((*bn).data).as_mut_ptr().offset(len as isize);
    end = ((*bn).data).as_mut_ptr();
    while d >= end {
        if *d != 0 {
            break;
        }
        d = d.offset(-1);
        d;
    }
    (*bn)
        .length = (if d < end {
        0 as libc::c_int as libc::c_long
    } else {
        d.offset_from(end) as libc::c_long + 1 as libc::c_int as libc::c_long
    }) as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_left_shift(
    mut bn: *mut mln_bignum_t,
    mut n: mln_u32_t,
) {
    __mln_bignum_left_shift(bn, n);
}
#[inline]
unsafe extern "C" fn __mln_bignum_right_shift(
    mut bn: *mut mln_bignum_t,
    mut n: mln_u32_t,
) {
    if n == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut step: mln_u32_t = n.wrapping_div(32 as libc::c_int as libc::c_uint);
    let mut off: mln_u32_t = n.wrapping_rem(32 as libc::c_int as libc::c_uint);
    if step >= 257 as libc::c_int as libc::c_uint {
        return;
    }
    if step >= (*bn).length {
        memset(
            bn as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        return;
    }
    let mut s: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut d: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut end: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut cur: mln_u64_t = 0;
    let mut last: *mut mln_u64_t = 0 as *mut mln_u64_t;
    end = ((*bn).data).as_mut_ptr().offset((*bn).length as isize);
    s = ((*bn).data).as_mut_ptr().offset(step as isize);
    d = ((*bn).data).as_mut_ptr();
    if off != 0 {
        let mut tmp: mln_u64_t = 0 as libc::c_int as mln_u64_t;
        last = &mut tmp;
        while s < end {
            *d = *s;
            cur = *d;
            *last
                |= cur << (32 as libc::c_int as libc::c_uint).wrapping_sub(off)
                    & 0xffffffff as libc::c_uint as libc::c_ulong;
            *d = *d >> off & 0xffffffff as libc::c_uint as libc::c_ulong;
            last = d;
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
        }
        s = d;
        while d < end {
            *d = 0 as libc::c_int as mln_u64_t;
            d = d.offset(1);
            d;
        }
    } else {
        while s < end {
            *d = *s;
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
        }
        s = d;
        while d < end {
            *d = 0 as libc::c_int as mln_u64_t;
            d = d.offset(1);
            d;
        }
    }
    d = ((*bn).data).as_mut_ptr();
    while s >= d {
        if *s != 0 {
            break;
        }
        s = s.offset(-1);
        s;
    }
    (*bn)
        .length = (if s < d {
        0 as libc::c_int as libc::c_long
    } else {
        s.offset_from(d) as libc::c_long + 1 as libc::c_int as libc::c_long
    }) as mln_u32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_right_shift(
    mut bn: *mut mln_bignum_t,
    mut n: mln_u32_t,
) {
    __mln_bignum_right_shift(bn, n);
}
#[inline]
unsafe extern "C" fn __mln_bignum_mul_word(
    mut dest: *mut mln_bignum_t,
    mut src: mln_s64_t,
) {
    let mut tag: mln_u32_t = 0;
    let mut dest_data: *mut mln_u64_t = ((*dest).data).as_mut_ptr();
    let mut dend: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut carry: mln_u64_t = 0;
    if src < 0 as libc::c_int as libc::c_long {
        tag = (*dest).tag ^ 1 as libc::c_int as libc::c_uint;
        src = -src;
    } else {
        tag = (*dest).tag ^ 0 as libc::c_int as libc::c_uint;
    }
    src &= 0xffffffff as libc::c_uint as libc::c_long;
    carry = 0 as libc::c_int as mln_u64_t;
    dend = ((*dest).data).as_mut_ptr().offset((*dest).length as isize);
    while dest_data < dend {
        *dest_data = (*dest_data).wrapping_mul(src as libc::c_ulong).wrapping_add(carry);
        carry = *dest_data >> 32 as libc::c_int;
        *dest_data = (*dest_data as libc::c_ulong)
            .wrapping_rem(0x100000000 as libc::c_ulonglong as mln_u64_t) as mln_u64_t
            as mln_u64_t;
        dest_data = dest_data.offset(1);
        dest_data;
    }
    while carry != 0 {
        if dest_data >= ((*dest).data).as_mut_ptr().offset(257 as libc::c_int as isize) {
            break;
        }
        *dest_data = (*dest_data as libc::c_ulong).wrapping_add(carry) as mln_u64_t
            as mln_u64_t;
        carry = *dest_data >> 32 as libc::c_int;
        *dest_data = (*dest_data as libc::c_ulong)
            .wrapping_rem(0x100000000 as libc::c_ulonglong as mln_u64_t) as mln_u64_t
            as mln_u64_t;
        (*dest).length = ((*dest).length).wrapping_add(1);
        (*dest).length;
    }
    (*dest).tag = tag;
}
#[inline]
unsafe extern "C" fn __mln_bignum_div_word(
    mut dest: *mut mln_bignum_t,
    mut src: mln_s64_t,
    mut quotient: *mut mln_bignum_t,
) -> libc::c_int {
    if src == 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    let mut tag: mln_u32_t = if src < 0 as libc::c_int as libc::c_long {
        (*dest).tag ^ 1 as libc::c_int as libc::c_uint
    } else {
        (*dest).tag ^ 0 as libc::c_int as libc::c_uint
    };
    src &= 0xffffffff as libc::c_uint as libc::c_long;
    if src == 1 as libc::c_int as libc::c_long
        || src == -(1 as libc::c_int) as libc::c_long
    {
        (*dest).tag = tag;
        if !quotient.is_null() {
            *quotient = *dest;
        }
        memset(
            dest as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        return 0 as libc::c_int;
    }
    if (*dest).length == 0 as libc::c_int as libc::c_uint {
        if !quotient.is_null() {
            memset(
                quotient as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
            );
        }
        return 0 as libc::c_int;
    }
    let mut dest_data: *mut mln_u64_t = ((*dest).data)
        .as_mut_ptr()
        .offset((*dest).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut end: *mut mln_u64_t = ((*dest).data).as_mut_ptr();
    let mut data: *mut mln_u64_t = 0 as *mut mln_u64_t;
    let mut tmp: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    if !quotient.is_null() {
        memset(
            quotient as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
        (*quotient).length = (*dest).length;
        (*quotient).tag = tag;
    }
    if *dest_data < src as libc::c_ulong {
        tmp = *dest_data << 32 as libc::c_int;
        let fresh6 = dest_data;
        dest_data = dest_data.offset(-1);
        *fresh6 = 0 as libc::c_int as mln_u64_t;
        if !quotient.is_null() {
            (*quotient).length = ((*quotient).length).wrapping_sub(1);
            (*quotient).length;
        }
    }
    if !quotient.is_null() {
        data = ((*quotient).data)
            .as_mut_ptr()
            .offset((*quotient).length as isize)
            .offset(-(1 as libc::c_int as isize));
    }
    while dest_data >= end {
        tmp = (tmp as libc::c_ulong).wrapping_add(*dest_data) as mln_u64_t as mln_u64_t;
        if !data.is_null() {
            let fresh7 = data;
            data = data.offset(-1);
            *fresh7 = tmp.wrapping_div(src as libc::c_ulong);
        }
        tmp = (tmp as libc::c_ulong).wrapping_rem(src as libc::c_ulong) as mln_u64_t
            as mln_u64_t;
        tmp <<= 32 as libc::c_int;
        let fresh8 = dest_data;
        dest_data = dest_data.offset(-1);
        *fresh8 = 0 as libc::c_int as mln_u64_t;
    }
    (*dest).tag = tag;
    (*dest)
        .length = (if tmp != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
        as mln_u32_t;
    (*dest).data[0 as libc::c_int as usize] = tmp >> 32 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_dump(mut bn: *mut mln_bignum_t) {
    fprintf(
        stderr,
        b"Tag: %s\n\0" as *const u8 as *const libc::c_char,
        if (*bn).tag == 0 as libc::c_int as libc::c_uint {
            b"+\0" as *const u8 as *const libc::c_char
        } else {
            b"-\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(stderr, b"Length: %u\n\0" as *const u8 as *const libc::c_char, (*bn).length);
    let mut i: mln_u32_t = 0;
    fprintf(stderr, b"Data:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as mln_u32_t;
    while i < 257 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"\t%lx\n\0" as *const u8 as *const libc::c_char,
            (*bn).data[i as usize],
        );
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(stderr);
}
#[inline]
unsafe extern "C" fn mln_bignum_seperate(
    mut pwr: *mut mln_u32_t,
    mut odd: *mut mln_bignum_t,
) {
    let mut tmp: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut mod_0: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut two: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 1 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    two.data[0 as libc::c_int as usize] = 2 as libc::c_int as mln_u64_t;
    *pwr = 0 as libc::c_int as mln_u32_t;
    loop {
        mod_0 = *odd;
        if __mln_bignum_div(&mut mod_0, &mut two, &mut tmp) >= 0 as libc::c_int {} else {
            __assert_fail(
                b"__mln_bignum_div(&mod, &two, &tmp) >= 0\0" as *const u8
                    as *const libc::c_char,
                b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
                942 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void mln_bignum_seperate(mln_u32_t *, mln_bignum_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_14765: {
            if __mln_bignum_div(&mut mod_0, &mut two, &mut tmp) >= 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"__mln_bignum_div(&mod, &two, &tmp) >= 0\0" as *const u8
                        as *const libc::c_char,
                    b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
                    942 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"void mln_bignum_seperate(mln_u32_t *, mln_bignum_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        if mod_0.length != 0 {
            break;
        }
        *odd = tmp;
        *pwr = (*pwr).wrapping_add(1);
        *pwr;
    };
}
#[inline]
unsafe extern "C" fn mln_bignum_witness(
    mut base: *mut mln_bignum_t,
    mut prim: *mut mln_bignum_t,
) -> mln_u32_t {
    let mut pwr: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut i: mln_u32_t = 0;
    let mut tmp: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut new_x: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 0 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    let mut x: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut odd: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut num: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 1 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    num.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
    odd = *prim;
    mln_bignum_sub(&mut odd, &mut num);
    mln_bignum_seperate(&mut pwr, &mut odd);
    x = *base;
    __mln_bignum_pwr(&mut x, &mut odd, prim);
    i = 0 as libc::c_int as mln_u32_t;
    while i < pwr {
        new_x = x;
        num.data[0 as libc::c_int as usize] = 2 as libc::c_int as mln_u64_t;
        if __mln_bignum_pwr(&mut new_x, &mut num, prim) >= 0 as libc::c_int {} else {
            __assert_fail(
                b"__mln_bignum_pwr(&new_x, &num, prim) >= 0\0" as *const u8
                    as *const libc::c_char,
                b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
                968 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"mln_u32_t mln_bignum_witness(mln_bignum_t *, mln_bignum_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_14629: {
            if __mln_bignum_pwr(&mut new_x, &mut num, prim) >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"__mln_bignum_pwr(&new_x, &num, prim) >= 0\0" as *const u8
                        as *const libc::c_char,
                    b"src/mln_bignum.c\0" as *const u8 as *const libc::c_char,
                    968 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 61],
                        &[libc::c_char; 61],
                    >(b"mln_u32_t mln_bignum_witness(mln_bignum_t *, mln_bignum_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        num.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
        tmp = *prim;
        __mln_bignum_sub(&mut tmp, &mut num);
        if __mln_bignum_abs_compare(&mut new_x, &mut num) == 0 as libc::c_int
            && __mln_bignum_abs_compare(&mut x, &mut num) != 0
            && __mln_bignum_abs_compare(&mut x, &mut tmp) != 0
        {
            return 1 as libc::c_int as mln_u32_t;
        }
        x = new_x;
        i = i.wrapping_add(1);
        i;
    }
    if __mln_bignum_abs_compare(&mut new_x, &mut num) != 0 {
        return 1 as libc::c_int as mln_u32_t;
    }
    return 0 as libc::c_int as mln_u32_t;
}
#[inline]
unsafe extern "C" fn mln_bignum_random_prime(
    mut bn: *mut mln_bignum_t,
    mut bitwidth: mln_u32_t,
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memset(
        bn as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
    );
    (*bn).tag = 0 as libc::c_int as mln_u32_t;
    let mut data: *mut mln_u64_t = ((*bn).data).as_mut_ptr();
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    let mut val: mln_u32_t = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long
        + tv.tv_usec) as mln_u32_t;
    let mut times: mln_u32_t = bitwidth.wrapping_div(32 as libc::c_int as libc::c_uint);
    let mut off: mln_u32_t = 0;
    let mut i: mln_s32_t = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < times {
        val = rand_r(&mut val) as mln_u32_t;
        *data
            .offset(
                i as isize,
            ) = val as mln_u64_t & 0xffffffff as libc::c_uint as libc::c_ulong;
        i += 1;
        i;
    }
    off = bitwidth.wrapping_rem(32 as libc::c_int as libc::c_uint);
    if off != 0 {
        *data
            .offset(
                i as isize,
            ) = (rand_r(&mut val) as mln_u64_t)
            .wrapping_mul(0xfdfd as libc::c_int as libc::c_ulong)
            & 0xffffffff as libc::c_uint as libc::c_ulong;
        let ref mut fresh9 = *data.offset(i as isize);
        *fresh9
            |= (1 as libc::c_int as mln_u64_t)
                << off.wrapping_sub(1 as libc::c_int as libc::c_uint);
        *data.offset(i as isize)
            <<= (64 as libc::c_int as libc::c_uint).wrapping_sub(off);
        *data.offset(i as isize)
            >>= (64 as libc::c_int as libc::c_uint).wrapping_sub(off);
        (*bn).length = (i + 1 as libc::c_int) as mln_u32_t;
        let ref mut fresh10 = *data.offset(0 as libc::c_int as isize);
        *fresh10 |= 1 as libc::c_int as libc::c_ulong;
    } else if times == 0 as libc::c_int as libc::c_uint {
        memset(
            bn as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_bignum_t>() as libc::c_ulong,
        );
    } else {
        if *data.offset((i - 1 as libc::c_int) as isize)
            == 0 as libc::c_int as libc::c_ulong
        {
            gettimeofday(&mut tv, 0 as *mut libc::c_void);
            val = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec)
                as mln_u32_t;
            *data
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = rand_r(&mut val) as mln_u32_t as mln_u64_t;
        }
        let ref mut fresh11 = *data.offset((i - 1 as libc::c_int) as isize);
        *fresh11 |= 0x80000000 as libc::c_uint as libc::c_ulong;
        (*bn).length = i as mln_u32_t;
        let ref mut fresh12 = *data.offset(0 as libc::c_int as isize);
        *fresh12 |= 1 as libc::c_int as libc::c_ulong;
    };
}
#[inline]
unsafe extern "C" fn mln_bignum_random_scope(
    mut bn: *mut mln_bignum_t,
    mut bitwidth: mln_u32_t,
    mut max: *mut mln_bignum_t,
) {
    let mut width: mln_u32_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut val: mln_u32_t = 0;
    loop {
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        val = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec)
            as mln_u32_t;
        width = (rand_r(&mut val) as mln_u32_t).wrapping_rem(bitwidth);
        if !(width < 2 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    mln_bignum_random_prime(bn, width);
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_prime(
    mut res: *mut mln_bignum_t,
    mut bitwidth: mln_u32_t,
) -> libc::c_int {
    if bitwidth > (8224 as libc::c_int >> 1 as libc::c_int) as libc::c_uint
        || bitwidth < 3 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    let mut prime: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmp: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut one: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 1 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    one.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
    let mut times: mln_u32_t = 0;
    loop {
        mln_bignum_random_prime(&mut prime, bitwidth);
        if __mln_bignum_abs_compare(&mut prime, &mut one) <= 0 as libc::c_int {
            continue;
        }
        times = if bitwidth <= 512 as libc::c_int as libc::c_uint {
            4 as libc::c_int as libc::c_uint
        } else {
            bitwidth >> 9 as libc::c_int
        };
        while times > 0 as libc::c_int as libc::c_uint {
            mln_bignum_random_scope(&mut tmp, bitwidth, &mut prime);
            if mln_bignum_witness(&mut tmp, &mut prime) != 0 {
                break;
            }
            times = times.wrapping_sub(1);
            times;
        }
        if times == 0 as libc::c_int as libc::c_uint {
            break;
        }
    }
    *res = prime;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_extend_eulid(
    mut a: *mut mln_bignum_t,
    mut b: *mut mln_bignum_t,
    mut x: *mut mln_bignum_t,
    mut y: *mut mln_bignum_t,
) -> libc::c_int {
    let mut m: mln_bignum_t = *a;
    let mut n: mln_bignum_t = *b;
    let mut r: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut q: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmp: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmpx: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmpy: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut zero: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 0 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    let mut x0: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 1 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    x0.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
    let mut y0: mln_bignum_t = zero;
    let mut x1: mln_bignum_t = zero;
    let mut y1: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 1 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    y1.data[0 as libc::c_int as usize] = 1 as libc::c_int as mln_u64_t;
    tmpx = x1;
    tmpy = y1;
    r = m;
    if __mln_bignum_div(&mut r, &mut n, &mut q) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    while __mln_bignum_compare(&mut r, &mut zero) > 0 as libc::c_int {
        tmp = q;
        __mln_bignum_mul(&mut tmp, &mut x1);
        tmpx = x0;
        __mln_bignum_sub(&mut tmpx, &mut tmp);
        tmp = q;
        __mln_bignum_mul(&mut tmp, &mut y1);
        tmpy = y0;
        __mln_bignum_sub(&mut tmpy, &mut tmp);
        x0 = x1;
        y0 = y1;
        x1 = tmpx;
        y1 = tmpy;
        m = n;
        n = r;
        r = m;
        if __mln_bignum_div(&mut r, &mut n, &mut q) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if !x.is_null() {
        *x = tmpx;
    }
    if !y.is_null() {
        *y = tmpy;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_i2osp(
    mut n: *mut mln_bignum_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_size_t,
) -> libc::c_int {
    if (*n).tag == 1 as libc::c_int as libc::c_uint
        || ((*n).length << 2 as libc::c_int) as libc::c_ulong > len
    {
        return -(1 as libc::c_int);
    }
    let mut p: *mut mln_u64_t = ((*n).data)
        .as_mut_ptr()
        .offset((*n).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut end: *mut mln_u64_t = ((*n).data).as_mut_ptr();
    let mut max: mln_size_t = len
        .wrapping_sub(((*n).length << 2 as libc::c_int) as libc::c_ulong);
    loop {
        let fresh13 = max;
        max = max.wrapping_sub(1);
        if !(fresh13 != 0) {
            break;
        }
        let fresh14 = buf;
        buf = buf.offset(1);
        *fresh14 = 0 as libc::c_int as libc::c_uchar;
    }
    while p >= end {
        let fresh15 = buf;
        buf = buf.offset(1);
        *fresh15 = (*p >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh16 = buf;
        buf = buf.offset(1);
        *fresh16 = (*p >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh17 = buf;
        buf = buf.offset(1);
        *fresh17 = (*p >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        let fresh18 = buf;
        buf = buf.offset(1);
        *fresh18 = (*p & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        p = p.offset(-1);
        p;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_os2ip(
    mut n: *mut mln_bignum_t,
    mut buf: mln_u8ptr_t,
    mut len: mln_size_t,
) -> libc::c_int {
    if len > ((257 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    *n = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 0 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    let mut data: *mut mln_u64_t = ((*n).data).as_mut_ptr();
    let mut p: mln_u8ptr_t = buf
        .offset(len as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut i: mln_size_t = 0 as libc::c_int as mln_size_t;
    while p >= buf {
        let fresh19 = p;
        p = p.offset(-1);
        *data |= (*fresh19 as mln_u64_t) << i;
        if i >= 24 as libc::c_int as libc::c_ulong {
            let fresh20 = data;
            data = data.offset(1);
            *fresh20 &= 0xffffffff as libc::c_uint as libc::c_ulong;
            i = 0 as libc::c_int as mln_size_t;
        } else {
            i = (i as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
                as mln_size_t as mln_size_t;
        }
    }
    (*n)
        .length = (if i != 0 {
        data.offset_from(((*n).data).as_mut_ptr()) as libc::c_long
            + 1 as libc::c_int as libc::c_long
    } else {
        data.offset_from(((*n).data).as_mut_ptr()) as libc::c_long
    }) as mln_u32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_bignum_tostring(
    mut n: *mut mln_bignum_t,
) -> *mut mln_string_t {
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut tmp: mln_u8_t = 0;
    let mut i: mln_u32_t = 0;
    let mut size: mln_u32_t = (*n).length << 6 as libc::c_int;
    let mut zero: mln_bignum_t = {
        let mut init = mln_bignum_t {
            tag: 0 as libc::c_int as mln_u32_t,
            length: 0 as libc::c_int as mln_u32_t,
            data: [
                0 as libc::c_int as mln_u64_t,
            ],
        };
        init
    };
    let mut quotient: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut neg: mln_u32_t = ((*n).tag == 1 as libc::c_int as libc::c_uint)
        as libc::c_int as mln_u32_t;
    let mut dup: mln_bignum_t = *n;
    if size == 0 {
        size = size.wrapping_add(1);
        size;
    }
    buf = malloc(size.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    p = buf;
    while mln_bignum_compare(&mut dup, &mut zero) != 0 {
        if __mln_bignum_div_word(&mut dup, 10 as libc::c_int as mln_s64_t, &mut quotient)
            < 0 as libc::c_int
        {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = (dup.data[0 as libc::c_int as usize] as mln_u8_t as libc::c_int
            + '0' as i32 as mln_u8_t as libc::c_int) as libc::c_uchar;
        dup = quotient;
    }
    if neg != 0 {
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = '-' as i32 as mln_u8_t;
    }
    size = p.offset_from(buf) as libc::c_long as mln_u32_t;
    i = 0 as libc::c_int as mln_u32_t;
    while i < size >> 1 as libc::c_int {
        tmp = *buf.offset(i as isize);
        *buf
            .offset(
                i as isize,
            ) = *buf
            .offset(
                size.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            );
        *buf
            .offset(
                size.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ) = tmp;
        i = i.wrapping_add(1);
        i;
    }
    if p == buf {
        let fresh23 = p;
        p = p.offset(1);
        *fresh23 = '0' as i32 as libc::c_uchar;
    }
    *p = 0 as libc::c_int as libc::c_uchar;
    ret = mln_string_buf_new(buf, p.offset_from(buf) as libc::c_long as mln_u64_t);
    if ret.is_null() {
        free(buf as *mut libc::c_void);
        return 0 as *mut mln_string_t;
    }
    return ret;
}
