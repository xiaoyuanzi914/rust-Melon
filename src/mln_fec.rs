use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u16_t = libc::c_ushort;
pub type mln_u32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fec_result_t {
    pub packets: *mut *mut mln_string_t,
    pub nr_packets: mln_size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_fec_t {
    pub seq_no: mln_u16_t,
    #[bitfield(name = "pt", ty = "mln_u16_t", bits = "0..=6")]
    pub pt: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
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
unsafe extern "C" fn mln_string_vector_new(mut n: mln_size_t) -> *mut *mut mln_string_t {
    return calloc(
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<*mut mln_string_t>() as libc::c_ulong,
    ) as *mut *mut mln_string_t;
}
#[inline]
unsafe extern "C" fn mln_string_vector_free(mut vec: *mut *mut mln_string_t) {
    if vec.is_null() {
        return;
    }
    let mut p: *mut *mut mln_string_t = vec;
    while !(*p).is_null() {
        let mut __s: *mut mln_string_t = *p;
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
        p = p.offset(1);
        p;
    }
    free(vec as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_fec_xor(
    mut data1: *mut mln_string_t,
    mut data2: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut p1: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p2: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    if (*data1).len > (*data2).len {
        ret = mln_string_dup(data1);
        if ret.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut mln_string_t;
        }
        p1 = (*ret).data;
        p2 = (*data2).data;
        end = ((*data2).data).offset((*data2).len as isize);
        while p2 < end {
            *p1 = (*p1 as libc::c_int ^ *p2 as libc::c_int) as libc::c_uchar;
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
    } else {
        ret = mln_string_dup(data2);
        if ret.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut mln_string_t;
        }
        p1 = (*data1).data;
        p2 = (*ret).data;
        end = ((*data1).data).offset((*data1).len as isize);
        while p1 < end {
            *p2 = (*p2 as libc::c_int ^ *p1 as libc::c_int) as libc::c_uchar;
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
    }
    return ret;
}
unsafe extern "C" fn mln_fec_result_new(
    mut packets: *mut *mut mln_string_t,
    mut nr_packets: mln_size_t,
) -> *mut mln_fec_result_t {
    let mut fr: *mut mln_fec_result_t = 0 as *mut mln_fec_result_t;
    fr = malloc(::core::mem::size_of::<mln_fec_result_t>() as libc::c_ulong)
        as *mut mln_fec_result_t;
    if fr.is_null() {
        return 0 as *mut mln_fec_result_t;
    }
    (*fr).packets = packets;
    (*fr).nr_packets = nr_packets;
    return fr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fec_result_free(mut fr: *mut mln_fec_result_t) {
    if fr.is_null() {
        return;
    }
    if !((*fr).packets).is_null() {
        mln_string_vector_free((*fr).packets);
    }
    free(fr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_fec_new() -> *mut mln_fec_t {
    let mut fec: *mut mln_fec_t = 0 as *mut mln_fec_t;
    fec = malloc(::core::mem::size_of::<mln_fec_t>() as libc::c_ulong) as *mut mln_fec_t;
    if fec.is_null() {
        return 0 as *mut mln_fec_t;
    }
    (*fec).seq_no = 0 as libc::c_int as mln_u16_t;
    (*fec).set_pt(0 as libc::c_int as mln_u16_t);
    return fec;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fec_free(mut fec: *mut mln_fec_t) {
    if fec.is_null() {
        return;
    }
    free(fec as *mut libc::c_void);
}
unsafe extern "C" fn mln_fec_generate_fecpacket_fecheader(
    mut fec: *mut mln_fec_t,
    mut n: mln_size_t,
    mut packets: *mut mln_u8ptr_t,
    mut packlen: *mut mln_u16_t,
    mut buf: mln_u8ptr_t,
    mut len: *mut mln_size_t,
) -> libc::c_int {
    let mut tmp16: mln_u16_t = 0;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut t: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut header: [mln_u8_t; 10] = [
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
    ];
    let mut sn: [mln_u8_t; 2] = [0; 2];
    let mut end: *mut mln_u8ptr_t = packets.offset(n as isize);
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    sn[0 as libc::c_int
        as usize] = *(*packets.offset(0 as libc::c_int as isize))
        .offset(2 as libc::c_int as isize);
    sn[1 as libc::c_int
        as usize] = *(*packets.offset(0 as libc::c_int as isize))
        .offset(3 as libc::c_int as isize);
    ({
        tmp.data = header.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = ::core::mem::size_of::<[mln_u8_t; 10]>() as libc::c_ulong;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    if ret.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    while packets < end {
        t = ret;
        tmp16 = (*packlen as libc::c_int - 12 as libc::c_int) as mln_u16_t;
        memcpy(
            header.as_mut_ptr() as *mut libc::c_void,
            *packets as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        p = header.as_mut_ptr().offset(8 as libc::c_int as isize);
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < 2 as libc::c_int as libc::c_ulong {
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = (tmp16 as mln_u64_t
                >> ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
        }
        ({
            tmp.data = header.as_mut_ptr() as mln_u8ptr_t;
            tmp.len = ::core::mem::size_of::<[mln_u8_t; 10]>() as libc::c_ulong;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
        ret = mln_fec_xor(&mut tmp, t);
        let mut __s: *mut mln_string_t = t;
        if !__s.is_null() {
            let ref mut fresh4 = (*__s).ref_0();
            let fresh5 = *fresh4;
            *fresh4 = (*fresh4).wrapping_sub(1);
            if fresh5 <= 1 as libc::c_int as libc::c_ulong {
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
        if ret.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        packets = packets.offset(1);
        packets;
        packlen = packlen.offset(1);
        packlen;
    }
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (*((*ret).data).offset(0 as libc::c_int as isize) as libc::c_int
        & 0x3f as libc::c_int) as libc::c_uchar;
    if n > 16 as libc::c_int as libc::c_ulong {
        let ref mut fresh6 = *buf.offset(0 as libc::c_int as isize);
        *fresh6 = (*fresh6 as libc::c_int | 0x40 as libc::c_int) as libc::c_uchar;
    }
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = *((*ret).data).offset(1 as libc::c_int as isize);
    *buf.offset(2 as libc::c_int as isize) = sn[0 as libc::c_int as usize];
    *buf.offset(3 as libc::c_int as isize) = sn[1 as libc::c_int as usize];
    *buf
        .offset(
            4 as libc::c_int as isize,
        ) = *((*ret).data).offset(4 as libc::c_int as isize);
    *buf
        .offset(
            5 as libc::c_int as isize,
        ) = *((*ret).data).offset(5 as libc::c_int as isize);
    *buf
        .offset(
            6 as libc::c_int as isize,
        ) = *((*ret).data).offset(6 as libc::c_int as isize);
    *buf
        .offset(
            7 as libc::c_int as isize,
        ) = *((*ret).data).offset(7 as libc::c_int as isize);
    *buf
        .offset(
            8 as libc::c_int as isize,
        ) = *((*ret).data).offset(8 as libc::c_int as isize);
    *buf
        .offset(
            9 as libc::c_int as isize,
        ) = *((*ret).data).offset(9 as libc::c_int as isize);
    *len = (*len as libc::c_ulong).wrapping_add(10 as libc::c_int as libc::c_ulong)
        as mln_size_t as mln_size_t;
    let mut __s: *mut mln_string_t = ret;
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_fec_generate_fecpacket_fecbody(
    mut fec: *mut mln_fec_t,
    mut n: mln_size_t,
    mut packets: *mut mln_u8ptr_t,
    mut packlen: *mut mln_u16_t,
    mut buf: mln_u8ptr_t,
    mut len: *mut mln_size_t,
) -> libc::c_int {
    let mut c: mln_u8_t = 0 as libc::c_int as mln_u8_t;
    let mut mask: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut end: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut p: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut t: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut tmp_sn: mln_u16_t = 0;
    let mut sn_base: mln_u16_t = 0;
    let mut pl: *mut mln_u16_t = 0 as *mut mln_u16_t;
    ptr = (*packets).offset(2 as libc::c_int as isize);
    let mut i: size_t = 0;
    sn_base = 0 as libc::c_int as mln_u16_t;
    i = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        sn_base = (sn_base as libc::c_ulong
            | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                << ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    << 3 as libc::c_int)) as mln_u16_t;
        i = i.wrapping_add(1);
        i;
        ptr = ptr.offset(1);
        ptr;
    }
    ptr = (*packets.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .offset(2 as libc::c_int as isize);
    let mut i_0: size_t = 0;
    tmp_sn = 0 as libc::c_int as mln_u16_t;
    i_0 = 0 as libc::c_int as size_t;
    while i_0 < 2 as libc::c_int as libc::c_ulong {
        tmp_sn = (tmp_sn as libc::c_ulong
            | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                << ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i_0)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    << 3 as libc::c_int)) as mln_u16_t;
        i_0 = i_0.wrapping_add(1);
        i_0;
        ptr = ptr.offset(1);
        ptr;
    }
    ({
        tmp.data = &mut c as *mut mln_u8_t as mln_u8ptr_t;
        tmp.len = 1 as libc::c_int as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    if ret.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    end = packets.offset(n as isize);
    p = packets;
    pl = packlen;
    while p < end {
        t = ret;
        ({
            tmp.data = *p;
            tmp.len = *pl as mln_u64_t;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
        ret = mln_fec_xor(&mut tmp, t);
        let mut __s: *mut mln_string_t = t;
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
        if ret.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        p = p.offset(1);
        p;
        pl = pl.offset(1);
        pl;
    }
    let mut i_1: size_t = 0;
    i_1 = 0 as libc::c_int as size_t;
    while i_1 < 2 as libc::c_int as libc::c_ulong {
        let fresh11 = buf;
        buf = buf.offset(1);
        *fresh11 = ((*ret).len
            >> ((2 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i_1)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    *len = (*len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
        as mln_size_t as mln_size_t;
    if n > 16 as libc::c_int as libc::c_ulong
        || tmp_sn as libc::c_int - sn_base as libc::c_int > 16 as libc::c_int
    {
        p = packets;
        end = packets.offset(n as isize);
        while p < end {
            ptr = (*p).offset(2 as libc::c_int as isize);
            let mut i_2: size_t = 0;
            tmp_sn = 0 as libc::c_int as mln_u16_t;
            i_2 = 0 as libc::c_int as size_t;
            while i_2 < 2 as libc::c_int as libc::c_ulong {
                tmp_sn = (tmp_sn as libc::c_ulong
                    | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i_2)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int)) as mln_u16_t;
                i_2 = i_2.wrapping_add(1);
                i_2;
                ptr = ptr.offset(1);
                ptr;
            }
            mask
                |= (1 as libc::c_int as mln_u64_t)
                    << 47 as libc::c_int
                        - (tmp_sn as libc::c_int - sn_base as libc::c_int);
            p = p.offset(1);
            p;
        }
        let mut i_3: size_t = 0;
        i_3 = 0 as libc::c_int as size_t;
        while i_3 < 6 as libc::c_int as libc::c_ulong {
            let fresh12 = buf;
            buf = buf.offset(1);
            *fresh12 = (mask
                >> ((6 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i_3)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            i_3 = i_3.wrapping_add(1);
            i_3;
        }
        *len = (*len as libc::c_ulong).wrapping_add(6 as libc::c_int as libc::c_ulong)
            as mln_size_t as mln_size_t;
    } else {
        p = packets;
        end = packets.offset(n as isize);
        while p < end {
            ptr = (*p).offset(2 as libc::c_int as isize);
            let mut i_4: size_t = 0;
            tmp_sn = 0 as libc::c_int as mln_u16_t;
            i_4 = 0 as libc::c_int as size_t;
            while i_4 < 2 as libc::c_int as libc::c_ulong {
                tmp_sn = (tmp_sn as libc::c_ulong
                    | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i_4)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int)) as mln_u16_t;
                i_4 = i_4.wrapping_add(1);
                i_4;
                ptr = ptr.offset(1);
                ptr;
            }
            mask
                |= (1 as libc::c_int as mln_u64_t)
                    << 15 as libc::c_int
                        - (tmp_sn as libc::c_int - sn_base as libc::c_int);
            p = p.offset(1);
            p;
        }
        let mut i_5: size_t = 0;
        i_5 = 0 as libc::c_int as size_t;
        while i_5 < 2 as libc::c_int as libc::c_ulong {
            let fresh13 = buf;
            buf = buf.offset(1);
            *fresh13 = (mask
                >> ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i_5)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
                & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
            i_5 = i_5.wrapping_add(1);
            i_5;
        }
        *len = (*len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as mln_size_t as mln_size_t;
    }
    memcpy(
        buf as *mut libc::c_void,
        ((*ret).data).offset(12 as libc::c_int as isize) as *const libc::c_void,
        ((*ret).len).wrapping_sub(12 as libc::c_int as libc::c_ulong),
    );
    *len = (*len as libc::c_ulong)
        .wrapping_add(((*ret).len).wrapping_sub(12 as libc::c_int as libc::c_ulong))
        as mln_size_t as mln_size_t;
    let mut __s: *mut mln_string_t = ret;
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_fec_generate_fecpacket(
    mut fec: *mut mln_fec_t,
    mut n: mln_size_t,
    mut packets: *mut mln_u8ptr_t,
    mut packlen: *mut mln_u16_t,
) -> *mut mln_string_t {
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut len: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut mod_0: mln_size_t = 0;
    let mut buf: [mln_u8_t; 1472] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut p: *mut mln_u8_t = 0 as *mut mln_u8_t;
    if mln_fec_generate_fecpacket_fecheader(
        fec,
        n,
        packets,
        packlen,
        buf.as_mut_ptr().offset(12 as libc::c_int as isize),
        &mut len,
    ) < 0 as libc::c_int
    {
        return 0 as *mut mln_string_t;
    }
    if mln_fec_generate_fecpacket_fecbody(
        fec,
        n,
        packets,
        packlen,
        buf
            .as_mut_ptr()
            .offset(12 as libc::c_int as isize)
            .offset(10 as libc::c_int as isize),
        &mut len,
    ) < 0 as libc::c_int
    {
        return 0 as *mut mln_string_t;
    }
    len = (len as libc::c_ulong).wrapping_add(12 as libc::c_int as libc::c_ulong)
        as mln_size_t as mln_size_t;
    buf[0 as libc::c_int
        as usize] = (buf[0 as libc::c_int as usize] as libc::c_int | 0x80 as libc::c_int)
        as mln_u8_t;
    mod_0 = len.wrapping_rem(4 as libc::c_int as libc::c_ulong);
    if mod_0 != 0 {
        buf[len.wrapping_add(mod_0).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = mod_0 as mln_u8_t;
        buf[0 as libc::c_int
            as usize] = (buf[0 as libc::c_int as usize] as libc::c_int
            | 0x20 as libc::c_int) as mln_u8_t;
        len = (len as libc::c_ulong).wrapping_add(mod_0) as mln_size_t as mln_size_t;
    }
    buf[1 as libc::c_int as usize] = (*fec).pt() as mln_u8_t;
    p = buf.as_mut_ptr().offset(2 as libc::c_int as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = ((*fec).seq_no as mln_u64_t
            >> ((2 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_ulong) as mln_u8_t;
        i = i.wrapping_add(1);
        i;
    }
    (*fec).seq_no = ((*fec).seq_no).wrapping_add(1);
    (*fec).seq_no;
    memcpy(
        p as *mut libc::c_void,
        &mut *(*packets
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .offset(4 as libc::c_int as isize) as *mut libc::c_uchar
            as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    ({
        tmp.data = buf.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = len;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    if ret.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_string_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fec_encode(
    mut fec: *mut mln_fec_t,
    mut packets: *mut *mut uint8_t,
    mut packlen: *mut uint16_t,
    mut n: size_t,
    mut group_size: uint16_t,
) -> *mut mln_fec_result_t {
    let mut pl: *mut uint16_t = 0 as *mut uint16_t;
    let mut plend: *mut uint16_t = 0 as *mut uint16_t;
    let mut p: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut vec: *mut *mut mln_string_t = 0 as *mut *mut mln_string_t;
    let mut next: *mut *mut mln_string_t = 0 as *mut *mut mln_string_t;
    let mut result: *mut mln_fec_result_t = 0 as *mut mln_fec_result_t;
    if fec.is_null() || packets.is_null() || packlen.is_null() || n == 0
        || group_size as libc::c_int > 48 as libc::c_int || group_size == 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    pl = packlen;
    plend = packlen.offset(n as isize);
    while pl < plend {
        if (*pl as libc::c_int) < 12 as libc::c_int
            || *pl as libc::c_int > 1442 as libc::c_int
        {
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut mln_fec_result_t;
        }
        pl = pl.offset(1);
        pl;
    }
    vec = mln_string_vector_new(n.wrapping_div(group_size as libc::c_ulong));
    if vec.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    next = vec;
    p = packets as *mut mln_u8ptr_t;
    pend = (packets as *mut mln_u8ptr_t).offset(n as isize);
    pl = packlen;
    plend = packlen.offset(n as isize);
    while p < pend {
        *next = mln_fec_generate_fecpacket(fec, group_size as mln_size_t, p, pl);
        if (*next).is_null() {
            mln_string_vector_free(vec);
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut mln_fec_result_t;
        }
        p = p.offset(group_size as libc::c_int as isize);
        pl = pl.offset(group_size as libc::c_int as isize);
        next = next.offset(1);
        next;
    }
    result = mln_fec_result_new(vec, n.wrapping_div(group_size as libc::c_ulong));
    if result.is_null() {
        mln_string_vector_free(vec);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    return result;
}
#[inline]
unsafe extern "C" fn mln_fec_recover_header_info_get(
    mut fec_packet: *mut mln_string_t,
    mut sn_base: *mut mln_u16_t,
    mut ssrc: *mut mln_u32_t,
    mut mask: *mut mln_u64_t,
    mut is_long: *mut mln_u16_t,
) {
    let mut p: mln_u8ptr_t = ((*fec_packet).data).offset(8 as libc::c_int as isize);
    if !ssrc.is_null() {
        let mut i: size_t = 0;
        *ssrc = 0 as libc::c_int as mln_u32_t;
        i = 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            *ssrc = (*ssrc as libc::c_ulong
                | (*p as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                    << ((4 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        << 3 as libc::c_int)) as mln_u32_t;
            i = i.wrapping_add(1);
            i;
            p = p.offset(1);
            p;
        }
    } else {
        p = p.offset(4 as libc::c_int as isize);
    }
    if !is_long.is_null() {
        if *p as libc::c_int & 0x40 as libc::c_int != 0 {
            *is_long = 1 as libc::c_int as mln_u16_t;
        } else {
            *is_long = 0 as libc::c_int as mln_u16_t;
        }
    }
    p = p.offset(2 as libc::c_int as isize);
    if !sn_base.is_null() {
        let mut i_0: size_t = 0;
        *sn_base = 0 as libc::c_int as mln_u16_t;
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < 2 as libc::c_int as libc::c_ulong {
            *sn_base = (*sn_base as libc::c_ulong
                | (*p as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                    << ((2 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(i_0)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        << 3 as libc::c_int)) as mln_u16_t;
            i_0 = i_0.wrapping_add(1);
            i_0;
            p = p.offset(1);
            p;
        }
    } else {
        p = p.offset(2 as libc::c_int as isize);
    }
    p = p.offset(8 as libc::c_int as isize);
    if !mask.is_null() {
        if *is_long != 0 {
            let mut i_1: size_t = 0;
            *mask = 0 as libc::c_int as mln_u64_t;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < 6 as libc::c_int as libc::c_ulong {
                *mask
                    |= (*p as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((6 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i_1)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int);
                i_1 = i_1.wrapping_add(1);
                i_1;
                p = p.offset(1);
                p;
            }
        } else {
            let mut i_2: size_t = 0;
            *mask = 0 as libc::c_int as mln_u64_t;
            i_2 = 0 as libc::c_int as size_t;
            while i_2 < 2 as libc::c_int as libc::c_ulong {
                *mask
                    |= (*p as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i_2)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int);
                i_2 = i_2.wrapping_add(1);
                i_2;
                p = p.offset(1);
                p;
            }
        }
    }
}
unsafe extern "C" fn mln_fec_decode_header(
    mut fec: *mut mln_fec_t,
    mut fec_packet: *mut mln_string_t,
    mut buf: mln_u8ptr_t,
    mut blen: *mut mln_size_t,
    mut packets: *mut mln_u8ptr_t,
    mut packlen: *mut mln_u16_t,
    mut n: mln_size_t,
    mut body_len: *mut mln_u16_t,
) -> libc::c_int {
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut mask: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut ssrc: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut t: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut res: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut p: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pl: *mut mln_u16_t = 0 as *mut mln_u16_t;
    let mut seq_no: mln_u16_t = 0;
    let mut is_long: mln_u16_t = 0 as libc::c_int as mln_u16_t;
    let mut sn_base: mln_u16_t = 0 as libc::c_int as mln_u16_t;
    let mut bit_string: [mln_u8_t; 10] = [
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
    ];
    mln_fec_recover_header_info_get(
        fec_packet,
        &mut sn_base,
        &mut ssrc,
        &mut mask,
        &mut is_long,
    );
    p = packets;
    pend = packets.offset(n as isize);
    pl = packlen;
    ({
        tmp.data = bit_string.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = ::core::mem::size_of::<[mln_u8_t; 10]>() as libc::c_ulong;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    res = mln_string_dup(&mut tmp);
    if res.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut current_block_37: u64;
    while p < pend {
        if !(*(*p).offset(1 as libc::c_int as isize) as libc::c_int & 0x7f as libc::c_int
            == (*fec).pt() as libc::c_int)
        {
            ptr = (*p).offset(2 as libc::c_int as isize);
            let mut i: size_t = 0;
            seq_no = 0 as libc::c_int as mln_u16_t;
            i = 0 as libc::c_int as size_t;
            while i < 2 as libc::c_int as libc::c_ulong {
                seq_no = (seq_no as libc::c_ulong
                    | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int)) as mln_u16_t;
                i = i.wrapping_add(1);
                i;
                ptr = ptr.offset(1);
                ptr;
            }
            if !((seq_no as libc::c_int) < sn_base as libc::c_int) {
                if is_long != 0 {
                    if mask
                        & (1 as libc::c_int as mln_u64_t)
                            << 47 as libc::c_int
                                - (seq_no as libc::c_int - sn_base as libc::c_int) == 0
                    {
                        current_block_37 = 13183875560443969876;
                    } else {
                        mask
                            &= !((1 as libc::c_int as mln_u64_t)
                                << 47 as libc::c_int
                                    - (seq_no as libc::c_int - sn_base as libc::c_int));
                        current_block_37 = 13472856163611868459;
                    }
                } else if mask
                    & (1 as libc::c_int as mln_u64_t)
                        << 15 as libc::c_int
                            - (seq_no as libc::c_int - sn_base as libc::c_int) == 0
                {
                    current_block_37 = 13183875560443969876;
                } else {
                    mask
                        &= !((1 as libc::c_int as mln_u64_t)
                            << 15 as libc::c_int
                                - (seq_no as libc::c_int - sn_base as libc::c_int));
                    current_block_37 = 13472856163611868459;
                }
                match current_block_37 {
                    13183875560443969876 => {}
                    _ => {
                        t = res;
                        memcpy(
                            bit_string.as_mut_ptr() as *mut libc::c_void,
                            *p as *const libc::c_void,
                            8 as libc::c_int as libc::c_ulong,
                        );
                        ptr = bit_string.as_mut_ptr().offset(8 as libc::c_int as isize);
                        *body_len = (*pl as libc::c_int - 12 as libc::c_int)
                            as mln_u16_t;
                        let mut i_0: size_t = 0;
                        i_0 = 0 as libc::c_int as size_t;
                        while i_0 < 2 as libc::c_int as libc::c_ulong {
                            let fresh17 = ptr;
                            ptr = ptr.offset(1);
                            *fresh17 = (*body_len as mln_u64_t
                                >> ((2 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(i_0)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    << 3 as libc::c_int) & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                            i_0 = i_0.wrapping_add(1);
                            i_0;
                        }
                        ({
                            tmp.data = bit_string.as_mut_ptr() as mln_u8ptr_t;
                            tmp
                                .len = ::core::mem::size_of::<[mln_u8_t; 10]>()
                                as libc::c_ulong;
                            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                            &mut tmp;
                            &mut tmp
                        });
                        res = mln_fec_xor(t, &mut tmp);
                        let mut __s: *mut mln_string_t = t;
                        if !__s.is_null() {
                            let ref mut fresh18 = (*__s).ref_0();
                            let fresh19 = *fresh18;
                            *fresh18 = (*fresh18).wrapping_sub(1);
                            if fresh19 <= 1 as libc::c_int as libc::c_ulong {
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
                        if res.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            return -(1 as libc::c_int);
                        }
                    }
                }
            }
        }
        p = p.offset(1);
        p;
        pl = pl.offset(1);
        pl;
    }
    if mask == 0 {
        let mut __s: *mut mln_string_t = res;
        if !__s.is_null() {
            let ref mut fresh20 = (*__s).ref_0();
            let fresh21 = *fresh20;
            *fresh20 = (*fresh20).wrapping_sub(1);
            if fresh21 <= 1 as libc::c_int as libc::c_ulong {
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
        *blen = 0 as libc::c_int as mln_size_t;
        return 0 as libc::c_int;
    }
    seq_no = 0 as libc::c_int as mln_u16_t;
    while mask != 0 {
        if mask & 0x1 as libc::c_int as libc::c_ulong != 0 {
            if is_long != 0 {
                seq_no = (47 as libc::c_int - seq_no as libc::c_int) as mln_u16_t;
            } else {
                seq_no = (15 as libc::c_int - seq_no as libc::c_int) as mln_u16_t;
            }
            seq_no = (seq_no as libc::c_int + sn_base as libc::c_int) as mln_u16_t;
            mask >>= 1 as libc::c_int;
            break;
        } else {
            mask >>= 1 as libc::c_int;
            seq_no = seq_no.wrapping_add(1);
            seq_no;
        }
    }
    if mask != 0 {
        let mut __s: *mut mln_string_t = res;
        if !__s.is_null() {
            let ref mut fresh22 = (*__s).ref_0();
            let fresh23 = *fresh22;
            *fresh22 = (*fresh22).wrapping_sub(1);
            if fresh23 <= 1 as libc::c_int as libc::c_ulong {
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
        *__errno_location() = 14 as libc::c_int;
        return -(1 as libc::c_int);
    }
    memcpy(
        bit_string.as_mut_ptr() as *mut libc::c_void,
        ((*fec_packet).data).offset(12 as libc::c_int as isize) as *const libc::c_void,
        ::core::mem::size_of::<[mln_u8_t; 10]>() as libc::c_ulong,
    );
    ({
        tmp.data = bit_string.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = ::core::mem::size_of::<[mln_u8_t; 10]>() as libc::c_ulong;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    t = res;
    res = mln_fec_xor(t, &mut tmp);
    let mut __s: *mut mln_string_t = t;
    if !__s.is_null() {
        let ref mut fresh24 = (*__s).ref_0();
        let fresh25 = *fresh24;
        *fresh24 = (*fresh24).wrapping_sub(1);
        if fresh25 <= 1 as libc::c_int as libc::c_ulong {
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
    if res.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ptr = (*res).data;
    let fresh26 = ptr;
    ptr = ptr.offset(1);
    let fresh27 = buf;
    buf = buf.offset(1);
    *fresh27 = (*fresh26 as libc::c_int & 0x3f as libc::c_int | 0x80 as libc::c_int)
        as libc::c_uchar;
    let fresh28 = ptr;
    ptr = ptr.offset(1);
    let fresh29 = buf;
    buf = buf.offset(1);
    *fresh29 = *fresh28;
    let mut i_1: size_t = 0;
    i_1 = 0 as libc::c_int as size_t;
    while i_1 < 2 as libc::c_int as libc::c_ulong {
        let fresh30 = buf;
        buf = buf.offset(1);
        *fresh30 = (seq_no as mln_u64_t
            >> ((2 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i_1)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    ptr = ptr.offset(2 as libc::c_int as isize);
    let fresh31 = ptr;
    ptr = ptr.offset(1);
    let fresh32 = buf;
    buf = buf.offset(1);
    *fresh32 = *fresh31;
    let fresh33 = ptr;
    ptr = ptr.offset(1);
    let fresh34 = buf;
    buf = buf.offset(1);
    *fresh34 = *fresh33;
    let fresh35 = ptr;
    ptr = ptr.offset(1);
    let fresh36 = buf;
    buf = buf.offset(1);
    *fresh36 = *fresh35;
    let fresh37 = ptr;
    ptr = ptr.offset(1);
    let fresh38 = buf;
    buf = buf.offset(1);
    *fresh38 = *fresh37;
    let mut i_2: size_t = 0;
    i_2 = 0 as libc::c_int as size_t;
    while i_2 < 4 as libc::c_int as libc::c_ulong {
        let fresh39 = buf;
        buf = buf.offset(1);
        *fresh39 = (ssrc as mln_u64_t
            >> ((4 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i_2)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int)
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        i_2 = i_2.wrapping_add(1);
        i_2;
    }
    *blen = 12 as libc::c_int as mln_size_t;
    let mut i_3: size_t = 0;
    *body_len = 0 as libc::c_int as mln_u16_t;
    i_3 = 0 as libc::c_int as size_t;
    while i_3 < 2 as libc::c_int as libc::c_ulong {
        *body_len = (*body_len as libc::c_ulong
            | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                << ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i_3)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    << 3 as libc::c_int)) as mln_u16_t;
        i_3 = i_3.wrapping_add(1);
        i_3;
        ptr = ptr.offset(1);
        ptr;
    }
    let mut __s: *mut mln_string_t = res;
    if !__s.is_null() {
        let ref mut fresh40 = (*__s).ref_0();
        let fresh41 = *fresh40;
        *fresh40 = (*fresh40).wrapping_sub(1);
        if fresh41 <= 1 as libc::c_int as libc::c_ulong {
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_fec_decode_body(
    mut fec: *mut mln_fec_t,
    mut fec_packet: *mut mln_string_t,
    mut buf: mln_u8ptr_t,
    mut blen: *mut mln_size_t,
    mut packets: *mut mln_u8ptr_t,
    mut packlen: *mut mln_u16_t,
    mut n: mln_size_t,
    mut body_len: mln_u16_t,
) -> libc::c_int {
    let mut mask: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut seq_no: mln_u16_t = 0;
    let mut sn_base: mln_u16_t = 0 as libc::c_int as mln_u16_t;
    let mut protect_len: mln_u16_t = 0 as libc::c_int as mln_u16_t;
    let mut pl: *mut mln_u16_t = 0 as *mut mln_u16_t;
    let mut is_long: mln_u16_t = 0 as libc::c_int as mln_u16_t;
    let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut t: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut res: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut body: [mln_u8_t; 1472] = [
        0 as libc::c_int as mln_u8_t,
    ];
    mln_fec_recover_header_info_get(
        fec_packet,
        &mut sn_base,
        0 as *mut mln_u32_t,
        &mut mask,
        &mut is_long,
    );
    ptr = ((*fec_packet).data).offset(22 as libc::c_int as isize);
    let mut i: size_t = 0;
    protect_len = 0 as libc::c_int as mln_u16_t;
    i = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        protect_len = (protect_len as libc::c_ulong
            | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                << ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    << 3 as libc::c_int)) as mln_u16_t;
        i = i.wrapping_add(1);
        i;
        ptr = ptr.offset(1);
        ptr;
    }
    ptr = if is_long as libc::c_int != 0 {
        ptr.offset(6 as libc::c_int as isize)
    } else {
        ptr.offset(2 as libc::c_int as isize)
    };
    if (ptr.offset_from((*fec_packet).data) as libc::c_long as libc::c_ulong)
        < (*fec_packet).len
    {
        memcpy(
            body.as_mut_ptr().offset(12 as libc::c_int as isize) as *mut libc::c_void,
            ptr as *const libc::c_void,
            (protect_len as libc::c_int - 12 as libc::c_int) as libc::c_ulong,
        );
    }
    ({
        tmp.data = body.as_mut_ptr() as mln_u8ptr_t;
        tmp.len = protect_len as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    res = mln_string_dup(&mut tmp);
    if res.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int);
    }
    p = packets;
    pend = packets.offset(n as isize);
    pl = packlen;
    let mut current_block_39: u64;
    while p < pend {
        if !(*(*p).offset(1 as libc::c_int as isize) as libc::c_int & 0x7f as libc::c_int
            == (*fec).pt() as libc::c_int)
        {
            ptr = (*p).offset(2 as libc::c_int as isize);
            let mut i_0: size_t = 0;
            seq_no = 0 as libc::c_int as mln_u16_t;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < 2 as libc::c_int as libc::c_ulong {
                seq_no = (seq_no as libc::c_ulong
                    | (*ptr as mln_u64_t & 0xff as libc::c_int as libc::c_ulong)
                        << ((2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(i_0)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            << 3 as libc::c_int)) as mln_u16_t;
                i_0 = i_0.wrapping_add(1);
                i_0;
                ptr = ptr.offset(1);
                ptr;
            }
            if !((seq_no as libc::c_int) < sn_base as libc::c_int) {
                if is_long != 0 {
                    if mask
                        & (1 as libc::c_int as mln_u64_t)
                            << 47 as libc::c_int
                                - (seq_no as libc::c_int - sn_base as libc::c_int) == 0
                    {
                        current_block_39 = 6009453772311597924;
                    } else {
                        mask
                            &= !((1 as libc::c_int as mln_u64_t)
                                << 47 as libc::c_int
                                    - (seq_no as libc::c_int - sn_base as libc::c_int));
                        current_block_39 = 11913429853522160501;
                    }
                } else if mask
                    & (1 as libc::c_int as mln_u64_t)
                        << 15 as libc::c_int
                            - (seq_no as libc::c_int - sn_base as libc::c_int) == 0
                {
                    current_block_39 = 6009453772311597924;
                } else {
                    mask
                        &= !((1 as libc::c_int as mln_u64_t)
                            << 15 as libc::c_int
                                - (seq_no as libc::c_int - sn_base as libc::c_int));
                    current_block_39 = 11913429853522160501;
                }
                match current_block_39 {
                    6009453772311597924 => {}
                    _ => {
                        t = res;
                        ({
                            tmp.data = *p;
                            tmp
                                .len = (if *pl as libc::c_int > protect_len as libc::c_int {
                                protect_len as libc::c_int
                            } else {
                                *pl as libc::c_int
                            }) as mln_u64_t;
                            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                            &mut tmp;
                            &mut tmp
                        });
                        res = mln_fec_xor(t, &mut tmp);
                        let mut __s: *mut mln_string_t = t;
                        if !__s.is_null() {
                            let ref mut fresh42 = (*__s).ref_0();
                            let fresh43 = *fresh42;
                            *fresh42 = (*fresh42).wrapping_sub(1);
                            if fresh43 <= 1 as libc::c_int as libc::c_ulong {
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
                        if res.is_null() {
                            *__errno_location() = 12 as libc::c_int;
                            return -(1 as libc::c_int);
                        }
                    }
                }
            }
        }
        p = p.offset(1);
        p;
        pl = pl.offset(1);
        pl;
    }
    memcpy(
        buf as *mut libc::c_void,
        ((*res).data).offset(12 as libc::c_int as isize) as *const libc::c_void,
        body_len as libc::c_ulong,
    );
    let mut __s: *mut mln_string_t = res;
    if !__s.is_null() {
        let ref mut fresh44 = (*__s).ref_0();
        let fresh45 = *fresh44;
        *fresh44 = (*fresh44).wrapping_sub(1);
        if fresh45 <= 1 as libc::c_int as libc::c_ulong {
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
    *blen = (*blen as libc::c_ulong).wrapping_add(body_len as libc::c_ulong)
        as mln_size_t as mln_size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_fec_decode(
    mut fec: *mut mln_fec_t,
    mut packets: *mut *mut uint8_t,
    mut packlen: *mut uint16_t,
    mut n: size_t,
) -> *mut mln_fec_result_t {
    let mut res: *mut mln_fec_result_t = 0 as *mut mln_fec_result_t;
    let mut p: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pl: *mut uint16_t = 0 as *mut uint16_t;
    let mut plend: *mut uint16_t = 0 as *mut uint16_t;
    let mut fec_cnt: uint16_t = 0 as libc::c_int as uint16_t;
    let mut body_len: uint16_t = 0 as libc::c_int as uint16_t;
    let mut fec_packet: mln_string_t = {
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
    };
    let mut vec: *mut *mut mln_string_t = 0 as *mut *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut buf: [mln_u8_t; 1472] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut blen: mln_size_t = 0 as libc::c_int as mln_size_t;
    if fec.is_null() || packets.is_null() || packlen.is_null() || n == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    p = packets as *mut mln_u8ptr_t;
    pl = packlen;
    plend = packlen.offset(n as isize);
    while pl < plend {
        if (*pl as libc::c_int) < 12 as libc::c_int
            || *pl as libc::c_int > 1472 as libc::c_int
        {
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut mln_fec_result_t;
        }
        if *(*p).offset(1 as libc::c_int as isize) as libc::c_int & 0x7f as libc::c_int
            == (*fec).pt() as libc::c_int
        {
            if (*pl as libc::c_int) < 26 as libc::c_int
                || *(*p).offset(12 as libc::c_int as isize) as libc::c_int
                    & 0x40 as libc::c_int != 0
                    && (*pl as libc::c_int) < 30 as libc::c_int
            {
                *__errno_location() = 22 as libc::c_int;
                return 0 as *mut mln_fec_result_t;
            }
            ({
                fec_packet.data = *p;
                fec_packet.len = *pl as mln_u64_t;
                fec_packet.set_data_ref(1 as libc::c_int as mln_uauto_t);
                fec_packet.set_pool(0 as libc::c_int as mln_uauto_t);
                fec_packet.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut fec_packet;
                &mut fec_packet
            });
            fec_cnt = fec_cnt.wrapping_add(1);
            fec_cnt;
        }
        pl = pl.offset(1);
        pl;
        p = p.offset(1);
        p;
    }
    if fec_cnt as libc::c_int > 1 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    if !(fec_cnt == 0) {
        if mln_fec_decode_header(
            fec,
            &mut fec_packet,
            buf.as_mut_ptr(),
            &mut blen,
            packets as *mut mln_u8ptr_t,
            packlen,
            n,
            &mut body_len,
        ) < 0 as libc::c_int
        {
            return 0 as *mut mln_fec_result_t;
        }
        if !(blen == 0) {
            if mln_fec_decode_body(
                fec,
                &mut fec_packet,
                buf.as_mut_ptr().offset(blen as isize),
                &mut blen,
                packets as *mut mln_u8ptr_t,
                packlen,
                n,
                body_len,
            ) < 0 as libc::c_int
            {
                return 0 as *mut mln_fec_result_t;
            }
            vec = mln_string_vector_new(1 as libc::c_int as mln_size_t);
            if vec.is_null() {
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut mln_fec_result_t;
            }
            ({
                tmp.data = buf.as_mut_ptr() as mln_u8ptr_t;
                tmp.len = blen;
                tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut tmp;
                &mut tmp
            });
            *vec = mln_string_dup(&mut tmp);
            if (*vec).is_null() {
                mln_string_vector_free(vec);
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut mln_fec_result_t;
            }
            res = mln_fec_result_new(vec, 1 as libc::c_int as mln_size_t);
            if res.is_null() {
                mln_string_vector_free(vec);
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut mln_fec_result_t;
            }
            return res;
        }
    }
    res = mln_fec_result_new(
        0 as *mut *mut mln_string_t,
        0 as libc::c_int as mln_size_t,
    );
    if res.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_fec_result_t;
    }
    return res;
}
