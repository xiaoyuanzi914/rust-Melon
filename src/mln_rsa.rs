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
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn mln_string_dup(str: *mut mln_string_t) -> *mut mln_string_t;
    fn mln_string_strcmp(s1: *mut mln_string_t, s2: *mut mln_string_t) -> libc::c_int;
    fn mln_bignum_assign(
        bn: *mut mln_bignum_t,
        sval: mln_s8ptr_t,
        len: mln_u32_t,
    ) -> libc::c_int;
    fn mln_bignum_add(dest: *mut mln_bignum_t, src: *mut mln_bignum_t);
    fn mln_bignum_sub(dest: *mut mln_bignum_t, src: *mut mln_bignum_t);
    fn mln_bignum_mul(dest: *mut mln_bignum_t, src: *mut mln_bignum_t);
    fn mln_bignum_div(
        dest: *mut mln_bignum_t,
        src: *mut mln_bignum_t,
        quotient: *mut mln_bignum_t,
    ) -> libc::c_int;
    fn mln_bignum_pwr(
        dest: *mut mln_bignum_t,
        exponent: *mut mln_bignum_t,
        mod_0: *mut mln_bignum_t,
    ) -> libc::c_int;
    fn mln_bignum_compare(bn1: *mut mln_bignum_t, bn2: *mut mln_bignum_t) -> libc::c_int;
    fn mln_bignum_abs_compare(
        bn1: *mut mln_bignum_t,
        bn2: *mut mln_bignum_t,
    ) -> libc::c_int;
    fn mln_bignum_bit_test(bn: *mut mln_bignum_t, index: mln_u32_t) -> libc::c_int;
    fn mln_bignum_prime(res: *mut mln_bignum_t, bitwidth: mln_u32_t) -> libc::c_int;
    fn mln_bignum_extend_eulid(
        a: *mut mln_bignum_t,
        b: *mut mln_bignum_t,
        x: *mut mln_bignum_t,
        y: *mut mln_bignum_t,
    ) -> libc::c_int;
    fn mln_bignum_i2osp(
        n: *mut mln_bignum_t,
        buf: mln_u8ptr_t,
        len: mln_size_t,
    ) -> libc::c_int;
    fn mln_bignum_os2ip(
        n: *mut mln_bignum_t,
        buf: mln_u8ptr_t,
        len: mln_size_t,
    ) -> libc::c_int;
    fn mln_md5_init(m: *mut mln_md5_t);
    fn mln_md5_calc(
        m: *mut mln_md5_t,
        input: mln_u8ptr_t,
        len: mln_uauto_t,
        is_last: mln_u32_t,
    );
    fn mln_md5_tobytes(m: *mut mln_md5_t, buf: mln_u8ptr_t, len: mln_u32_t);
    fn mln_sha1_init(s: *mut mln_sha1_t);
    fn mln_sha1_calc(
        s: *mut mln_sha1_t,
        input: mln_u8ptr_t,
        len: mln_uauto_t,
        is_last: mln_u32_t,
    );
    fn mln_sha1_tobytes(s: *mut mln_sha1_t, buf: mln_u8ptr_t, len: mln_u32_t);
    fn mln_sha256_init(s: *mut mln_sha256_t);
    fn mln_sha256_calc(
        s: *mut mln_sha256_t,
        input: mln_u8ptr_t,
        len: mln_uauto_t,
        is_last: mln_u32_t,
    );
    fn mln_sha256_tobytes(s: *mut mln_sha256_t, buf: mln_u8ptr_t, len: mln_u32_t);
    fn mln_asn1_decode_ref(
        data: *mut libc::c_void,
        len: mln_u64_t,
        err: *mut libc::c_int,
        pool: *mut mln_alloc_t,
    ) -> *mut mln_asn1_deresult_t;
    fn mln_asn1_deresult_free(res: *mut mln_asn1_deresult_t);
    fn mln_asn1_deresult_content_get(
        res: *mut mln_asn1_deresult_t,
        index: mln_u32_t,
    ) -> *mut mln_asn1_deresult_t;
    fn mln_asn1_enresult_init(
        res: *mut mln_asn1_enresult_t,
        pool: *mut mln_alloc_t,
    ) -> libc::c_int;
    fn mln_asn1_enresult_destroy(res: *mut mln_asn1_enresult_t);
    fn mln_asn1_encode_octetstring(
        res: *mut mln_asn1_enresult_t,
        octets: mln_u8ptr_t,
        n: mln_u64_t,
    ) -> libc::c_int;
    fn mln_asn1_encode_null(res: *mut mln_asn1_enresult_t) -> libc::c_int;
    fn mln_asn1_encode_object_identifier(
        res: *mut mln_asn1_enresult_t,
        oid: mln_u8ptr_t,
        n: mln_u64_t,
    ) -> libc::c_int;
    fn mln_asn1_encode_sequence(res: *mut mln_asn1_enresult_t) -> libc::c_int;
    fn mln_asn1_encode_merge(
        dest: *mut mln_asn1_enresult_t,
        src: *mut mln_asn1_enresult_t,
    ) -> libc::c_int;
    fn mln_asn1_enresult_get_content(
        res: *mut mln_asn1_enresult_t,
        index: mln_u32_t,
        buf: *mut mln_u8ptr_t,
        len: *mut mln_u64_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type mln_u8_t = libc::c_uchar;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_rsa_key_t {
    pub n: mln_bignum_t,
    pub ed: mln_bignum_t,
    pub p: mln_bignum_t,
    pub q: mln_bignum_t,
    pub dp: mln_bignum_t,
    pub dq: mln_bignum_t,
    pub qinv: mln_bignum_t,
    #[bitfield(name = "pwr", ty = "mln_u32_t", bits = "0..=0")]
    pub pwr: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_asn1_enresult_t {
    pub pool: *mut mln_alloc_t,
    pub size: mln_u64_t,
    pub contents: *mut mln_string_t,
    pub max: mln_u32_t,
    pub pos: mln_u32_t,
    #[bitfield(name = "_class", ty = "mln_u32_t", bits = "0..=1")]
    #[bitfield(name = "is_struct", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "ident", ty = "mln_u32_t", bits = "3..=31")]
    pub _class_is_struct_ident: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_EMSAPKCS1V15_HASH_s {
    pub digest_algorithm: mln_u8ptr_t,
    pub len: mln_size_t,
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
pub struct mln_md5_t {
    pub A: mln_u32_t,
    pub B: mln_u32_t,
    pub C: mln_u32_t,
    pub D: mln_u32_t,
    pub length: mln_u64_t,
    pub pos: mln_u32_t,
    pub buf: [mln_u8_t; 64],
}
pub type mln_asn1_deresult_t = mln_asn1_deresult_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_asn1_deresult_s {
    pub pool: *mut mln_alloc_t,
    pub code_buf: mln_u8ptr_t,
    pub code_len: mln_u64_t,
    pub contents: *mut *mut mln_asn1_deresult_t,
    pub max: mln_u32_t,
    pub pos: mln_u32_t,
    #[bitfield(name = "_class", ty = "mln_u32_t", bits = "0..=1")]
    #[bitfield(name = "is_struct", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "ident", ty = "mln_u32_t", bits = "3..=30")]
    #[bitfield(name = "free", ty = "mln_u32_t", bits = "31..=31")]
    pub _class_is_struct_ident_free: [u8; 4],
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
                    current_block = 8494144263657749559;
                    break;
                }
                am = am.offset(1);
                am;
            }
            match current_block {
                8494144263657749559 => {}
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
        current_block_8 = 8090402267511802060;
    } else {
        as_0 = (*pool).shm_head;
        while !as_0.is_null() {
            if mln_alloc_shm_allowed(as_0, &mut Boff, &mut boff, size) != 0 {
                break;
            }
            as_0 = (*as_0).next;
        }
        if as_0.is_null() {
            current_block_8 = 8090402267511802060;
        } else {
            current_block_8 = 2979737022853876585;
        }
    }
    match current_block_8 {
        8090402267511802060 => {
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
static mut EMSAPKCS1V15_HASH_MD5: [mln_u8_t; 8] = [
    0x2a as libc::c_int as mln_u8_t,
    0x86 as libc::c_int as mln_u8_t,
    0x48 as libc::c_int as mln_u8_t,
    0x86 as libc::c_int as mln_u8_t,
    0xf7 as libc::c_int as mln_u8_t,
    0xd as libc::c_int as mln_u8_t,
    0x2 as libc::c_int as mln_u8_t,
    0x5 as libc::c_int as mln_u8_t,
];
static mut EMSAPKCS1V15_HASH_SHA1: [mln_u8_t; 5] = [
    0x2b as libc::c_int as mln_u8_t,
    0xe as libc::c_int as mln_u8_t,
    0x3 as libc::c_int as mln_u8_t,
    0x2 as libc::c_int as mln_u8_t,
    0x1a as libc::c_int as mln_u8_t,
];
static mut EMSAPKCS1V15_HASH_SHA256: [mln_u8_t; 9] = [
    0x60 as libc::c_int as mln_u8_t,
    0x86 as libc::c_int as mln_u8_t,
    0x48 as libc::c_int as mln_u8_t,
    0x1 as libc::c_int as mln_u8_t,
    0x65 as libc::c_int as mln_u8_t,
    0x3 as libc::c_int as mln_u8_t,
    0x4 as libc::c_int as mln_u8_t,
    0x2 as libc::c_int as mln_u8_t,
    0x1 as libc::c_int as mln_u8_t,
];
#[no_mangle]
pub static mut EMSAPKCS1V15_HASH: [mln_EMSAPKCS1V15_HASH_s; 3] = unsafe {
    [
        {
            let mut init = mln_EMSAPKCS1V15_HASH_s {
                digest_algorithm: EMSAPKCS1V15_HASH_MD5.as_ptr() as *mut _,
                len: ::core::mem::size_of::<[mln_u8_t; 8]>() as libc::c_ulong,
            };
            init
        },
        {
            let mut init = mln_EMSAPKCS1V15_HASH_s {
                digest_algorithm: EMSAPKCS1V15_HASH_SHA1.as_ptr() as *mut _,
                len: ::core::mem::size_of::<[mln_u8_t; 5]>() as libc::c_ulong,
            };
            init
        },
        {
            let mut init = mln_EMSAPKCS1V15_HASH_s {
                digest_algorithm: EMSAPKCS1V15_HASH_SHA256.as_ptr() as *mut _,
                len: ::core::mem::size_of::<[mln_u8_t; 9]>() as libc::c_ulong,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn mln_rsa_key_new() -> *mut mln_rsa_key_t {
    return malloc(::core::mem::size_of::<mln_rsa_key_t>() as libc::c_ulong)
        as *mut mln_rsa_key_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rsa_key_pool_new(
    mut pool: *mut mln_alloc_t,
) -> *mut mln_rsa_key_t {
    return mln_alloc_m(pool, ::core::mem::size_of::<mln_rsa_key_t>() as libc::c_ulong)
        as *mut mln_rsa_key_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rsa_key_free(mut key: *mut mln_rsa_key_t) {
    if key.is_null() {
        return;
    }
    free(key as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rsa_key_pool_free(mut key: *mut mln_rsa_key_t) {
    if key.is_null() {
        return;
    }
    mln_alloc_free(key as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rsa_key_generate(
    mut pub_0: *mut mln_rsa_key_t,
    mut pri: *mut mln_rsa_key_t,
    mut bits: mln_u32_t,
) -> libc::c_int {
    if bits <= 88 as libc::c_int as libc::c_uint
        || bits > 8224 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    let mut p: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut q: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut n: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut phi_n: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut one: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut d: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    mln_bignum_assign(
        &mut one,
        b"1\0" as *const u8 as *const libc::c_char as mln_s8ptr_t,
        1 as libc::c_int as mln_u32_t,
    );
    let mut e: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmpp_1: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    let mut tmpq_1: mln_bignum_t = mln_bignum_t {
        tag: 0,
        length: 0,
        data: [0; 257],
    };
    loop {
        loop {
            if mln_bignum_prime(
                &mut p,
                (bits >> 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if mln_bignum_prime(&mut q, bits >> 1 as libc::c_int) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if mln_bignum_compare(&mut p, &mut q) != 0 {
                break;
            }
        }
        n = p;
        mln_bignum_mul(&mut n, &mut q);
        if mln_bignum_bit_test(
            &mut n,
            bits.wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
        {
            continue;
        }
        if !pub_0.is_null() {
            (*pub_0).p = p;
            (*pub_0).q = q;
            if mln_bignum_extend_eulid(
                &mut q,
                &mut p,
                &mut (*pub_0).qinv,
                0 as *mut mln_bignum_t,
            ) < 0 as libc::c_int
            {
                continue;
            }
            if mln_bignum_compare(&mut (*pub_0).qinv, &mut one) < 0 as libc::c_int {
                continue;
            }
        }
        if !pri.is_null() {
            (*pri).p = p;
            (*pri).q = q;
            if mln_bignum_extend_eulid(
                &mut q,
                &mut p,
                &mut (*pri).qinv,
                0 as *mut mln_bignum_t,
            ) < 0 as libc::c_int
            {
                continue;
            }
            if mln_bignum_compare(&mut (*pri).qinv, &mut one) < 0 as libc::c_int {
                continue;
            }
        }
        mln_bignum_sub(&mut p, &mut one);
        mln_bignum_sub(&mut q, &mut one);
        tmpp_1 = p;
        tmpq_1 = q;
        phi_n = p;
        mln_bignum_mul(&mut phi_n, &mut q);
        mln_bignum_assign(
            &mut e,
            b"0x10001\0" as *const u8 as *const libc::c_char as mln_s8ptr_t,
            7 as libc::c_int as mln_u32_t,
        );
        if mln_bignum_extend_eulid(&mut e, &mut phi_n, &mut d, 0 as *mut mln_bignum_t)
            < 0 as libc::c_int
        {
            continue;
        }
        if !(mln_bignum_compare(&mut d, &mut e) <= 0 as libc::c_int) {
            break;
        }
    }
    if !pub_0.is_null() {
        (*pub_0).n = n;
        (*pub_0).ed = e;
        (*pub_0).dq = e;
        (*pub_0).dp = (*pub_0).dq;
        mln_bignum_div(&mut (*pub_0).dp, &mut tmpp_1, 0 as *mut mln_bignum_t);
        mln_bignum_div(&mut (*pub_0).dq, &mut tmpq_1, 0 as *mut mln_bignum_t);
        (*pub_0).set_pwr(0 as libc::c_int as mln_u32_t);
    }
    if !pri.is_null() {
        (*pri).n = n;
        (*pri).ed = d;
        (*pri).dq = d;
        (*pri).dp = (*pri).dq;
        mln_bignum_div(&mut (*pri).dp, &mut tmpp_1, 0 as *mut mln_bignum_t);
        mln_bignum_div(&mut (*pri).dq, &mut tmpq_1, 0 as *mut mln_bignum_t);
        (*pri).set_pwr(0 as libc::c_int as mln_u32_t);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_rsa_rsaep_rsadp(
    mut key: *mut mln_rsa_key_t,
    mut in_0: *mut mln_bignum_t,
    mut out: *mut mln_bignum_t,
) -> libc::c_int {
    let mut current_block_12: u64;
    if (*key).pwr() as libc::c_int != 0
        || mln_bignum_abs_compare(&mut (*key).ed, &mut (*key).p) <= 0 as libc::c_int
    {
        current_block_12 = 3623866930031121610;
    } else {
        let mut m1: mln_bignum_t = mln_bignum_t {
            tag: 0,
            length: 0,
            data: [0; 257],
        };
        let mut m2: mln_bignum_t = mln_bignum_t {
            tag: 0,
            length: 0,
            data: [0; 257],
        };
        m2 = *in_0;
        m1 = m2;
        mln_bignum_pwr(&mut m1, &mut (*key).dp, &mut (*key).p);
        mln_bignum_pwr(&mut m2, &mut (*key).dq, &mut (*key).q);
        mln_bignum_sub(&mut m1, &mut m2);
        if m1.tag == 1 as libc::c_int as libc::c_uint {
            current_block_12 = 3623866930031121610;
        } else {
            mln_bignum_mul(&mut m1, &mut (*key).qinv);
            mln_bignum_div(&mut m1, &mut (*key).p, 0 as *mut mln_bignum_t);
            mln_bignum_mul(&mut m1, &mut (*key).q);
            mln_bignum_add(&mut m1, &mut m2);
            mln_bignum_div(&mut m1, &mut (*key).n, 0 as *mut mln_bignum_t);
            *out = m1;
            current_block_12 = 2968425633554183086;
        }
    }
    match current_block_12 {
        3623866930031121610 => {
            if mln_bignum_pwr(in_0, &mut (*key).ed, &mut (*key).n) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            *out = *in_0;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_rsa_pub_padding(
    mut in_0: mln_u8ptr_t,
    mut inlen: mln_size_t,
    mut out: mln_u8ptr_t,
    mut keylen: mln_size_t,
) {
    let mut j: mln_size_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut val: mln_u32_t = 0;
    let fresh1 = out;
    out = out.offset(1);
    *fresh1 = 0 as libc::c_int as libc::c_uchar;
    let fresh2 = out;
    out = out.offset(1);
    *fresh2 = 0x2 as libc::c_int as libc::c_uchar;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    val = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec) as mln_u32_t;
    j = keylen.wrapping_sub(inlen).wrapping_sub(3 as libc::c_int as libc::c_ulong);
    while j > 0 as libc::c_int as libc::c_ulong {
        loop {
            *out = rand_r(&mut val) as libc::c_uchar;
            val = *out as mln_u32_t;
            if !(val == 0 as libc::c_int as libc::c_uint) {
                break;
            }
            gettimeofday(&mut tv, 0 as *mut libc::c_void);
            val = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec)
                as mln_u32_t;
        }
        out = out.offset(1);
        out;
        j = j.wrapping_sub(1);
        j;
    }
    let fresh3 = out;
    out = out.offset(1);
    *fresh3 = 0 as libc::c_int as libc::c_uchar;
    memcpy(out as *mut libc::c_void, in_0 as *const libc::c_void, inlen);
}
#[inline]
unsafe extern "C" fn mln_rsa_pri_padding(
    mut in_0: mln_u8ptr_t,
    mut inlen: mln_size_t,
    mut out: mln_u8ptr_t,
    mut keylen: mln_size_t,
) {
    let mut j: mln_size_t = 0;
    let fresh4 = out;
    out = out.offset(1);
    *fresh4 = 0 as libc::c_int as libc::c_uchar;
    let fresh5 = out;
    out = out.offset(1);
    *fresh5 = 0x1 as libc::c_int as libc::c_uchar;
    j = keylen.wrapping_sub(inlen).wrapping_sub(3 as libc::c_int as libc::c_ulong);
    while j > 0 as libc::c_int as libc::c_ulong {
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 = 0xff as libc::c_int as libc::c_uchar;
        j = j.wrapping_sub(1);
        j;
    }
    let fresh7 = out;
    out = out.offset(1);
    *fresh7 = 0 as libc::c_int as libc::c_uchar;
    memcpy(out as *mut libc::c_void, in_0 as *const libc::c_void, inlen);
}
#[inline]
unsafe extern "C" fn mln_rsa_anti_padding_public(
    mut in_0: mln_u8ptr_t,
    mut len: mln_size_t,
) -> mln_u8ptr_t {
    if in_0.is_null() || len == 0 as libc::c_int as libc::c_ulong
        || *in_0 as libc::c_int != 0 as libc::c_int
    {
        return 0 as mln_u8ptr_t;
    }
    let mut p: mln_u8ptr_t = in_0.offset(2 as libc::c_int as isize);
    let mut end: mln_u8ptr_t = in_0.offset(len as isize);
    if *in_0.offset(1 as libc::c_int as isize) as libc::c_int != 0x2 as libc::c_int {
        return 0 as mln_u8ptr_t;
    }
    while *p as libc::c_int != 0 as libc::c_int && p < end {
        p = p.offset(1);
        p;
    }
    if p >= end || p.offset(1 as libc::c_int as isize) >= end {
        return 0 as mln_u8ptr_t;
    }
    let fresh8 = p;
    p = p.offset(1);
    if *fresh8 as libc::c_int != 0 as libc::c_int {
        return 0 as mln_u8ptr_t;
    }
    return p;
}
#[inline]
unsafe extern "C" fn mln_rsa_anti_padding_private(
    mut in_0: mln_u8ptr_t,
    mut len: mln_size_t,
) -> mln_u8ptr_t {
    if in_0.is_null() || len == 0 as libc::c_int as libc::c_ulong
        || *in_0 as libc::c_int != 0 as libc::c_int
    {
        return 0 as mln_u8ptr_t;
    }
    let mut p: mln_u8ptr_t = in_0.offset(2 as libc::c_int as isize);
    let mut end: mln_u8ptr_t = in_0.offset(len as isize);
    if *in_0.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        while *p as libc::c_int == 0 as libc::c_int && p < end {
            p = p.offset(1);
            p;
        }
        if p >= end {
            return 0 as mln_u8ptr_t;
        }
    } else if *in_0.offset(1 as libc::c_int as isize) as libc::c_int == 1 as libc::c_int
    {
        while *p as libc::c_int == 0xff as libc::c_int && p < end {
            p = p.offset(1);
            p;
        }
        if p >= end || p.offset(1 as libc::c_int as isize) >= end {
            return 0 as mln_u8ptr_t;
        }
        let fresh9 = p;
        p = p.offset(1);
        if *fresh9 as libc::c_int != 0 as libc::c_int {
            return 0 as mln_u8ptr_t;
        }
    } else {
        return 0 as mln_u8ptr_t
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSAESPKCS1V15_public_encrypt(
    mut pub_0: *mut mln_rsa_key_t,
    mut text: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut nlen: mln_size_t = ((*pub_0).n.length << 2 as libc::c_int) as mln_size_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut in_0: mln_u8ptr_t = (*text).data;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut i: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut len: mln_size_t = (*text).len;
    let mut sum: mln_size_t = 0;
    if nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong) < (*text).len {
        return 0 as *mut mln_string_t;
    }
    sum = if len
        .wrapping_div(nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong))
        .wrapping_mul(nlen)
        .wrapping_add(
            len.wrapping_rem(nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong)),
        ) != 0
    {
        nlen
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    buf = malloc(sum) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    p = buf;
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut num: mln_bignum_t = {
            let mut init = mln_bignum_t {
                tag: 0 as libc::c_int as mln_u32_t,
                length: 0 as libc::c_int as mln_u32_t,
                data: [
                    0 as libc::c_int as mln_u64_t,
                ],
            };
            init
        };
        i = if len > nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong) {
            nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong)
        } else {
            len
        };
        mln_rsa_pub_padding(in_0, i, p, nlen);
        if mln_bignum_os2ip(&mut num, p, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_rsa_rsaep_rsadp(pub_0, &mut num, &mut num) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_bignum_i2osp(&mut num, p, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        len = (len as libc::c_ulong).wrapping_sub(i) as mln_size_t as mln_size_t;
        in_0 = in_0.offset(i as isize);
        p = p.offset(nlen as isize);
    }
    ({
        tmp.data = buf;
        tmp.len = sum;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    free(buf as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSAESPKCS1V15_public_decrypt(
    mut pub_0: *mut mln_rsa_key_t,
    mut cipher: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut nlen: mln_size_t = ((*pub_0).n.length << 2 as libc::c_int) as mln_size_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pos: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut in_0: mln_u8ptr_t = (*cipher).data;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut len: mln_size_t = (*cipher).len;
    let mut sum: mln_size_t = 0 as libc::c_int as mln_size_t;
    if (*cipher).len < 11 as libc::c_int as libc::c_ulong
        || ((*cipher).len).wrapping_rem(nlen) != 0
    {
        return 0 as *mut mln_string_t;
    }
    buf = malloc((*cipher).len) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    p = buf;
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut num: mln_bignum_t = {
            let mut init = mln_bignum_t {
                tag: 0 as libc::c_int as mln_u32_t,
                length: 0 as libc::c_int as mln_u32_t,
                data: [
                    0 as libc::c_int as mln_u64_t,
                ],
            };
            init
        };
        if mln_bignum_os2ip(&mut num, in_0, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_rsa_rsaep_rsadp(pub_0, &mut num, &mut num) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_bignum_i2osp(&mut num, in_0, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        pos = mln_rsa_anti_padding_private(in_0, nlen);
        if pos.is_null() {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        memcpy(
            p as *mut libc::c_void,
            pos as *const libc::c_void,
            nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong),
        );
        sum = (sum as libc::c_ulong)
            .wrapping_add(
                nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong),
            ) as mln_size_t as mln_size_t;
        p = p
            .offset(
                nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong)
                    as isize,
            );
        in_0 = in_0.offset(nlen as isize);
        len = (len as libc::c_ulong).wrapping_sub(nlen) as mln_size_t as mln_size_t;
    }
    ({
        tmp.data = buf;
        tmp.len = sum;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    free(buf as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSAESPKCS1V15_private_encrypt(
    mut pri: *mut mln_rsa_key_t,
    mut text: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut nlen: mln_size_t = ((*pri).n.length << 2 as libc::c_int) as mln_size_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut in_0: mln_u8ptr_t = (*text).data;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut i: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut len: mln_size_t = (*text).len;
    let mut sum: mln_size_t = 0;
    if nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong) < (*text).len {
        return 0 as *mut mln_string_t;
    }
    sum = if len
        .wrapping_div(nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong))
        .wrapping_mul(nlen)
        .wrapping_add(
            len.wrapping_rem(nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong)),
        ) != 0
    {
        nlen
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    buf = malloc(sum) as mln_u8ptr_t;
    p = buf;
    if p.is_null() {
        return 0 as *mut mln_string_t;
    }
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut num: mln_bignum_t = {
            let mut init = mln_bignum_t {
                tag: 0 as libc::c_int as mln_u32_t,
                length: 0 as libc::c_int as mln_u32_t,
                data: [
                    0 as libc::c_int as mln_u64_t,
                ],
            };
            init
        };
        i = if len > nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong) {
            nlen.wrapping_sub(11 as libc::c_int as libc::c_ulong)
        } else {
            len
        };
        mln_rsa_pri_padding(in_0, i, p, nlen);
        if mln_bignum_os2ip(&mut num, p, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_rsa_rsaep_rsadp(pri, &mut num, &mut num) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_bignum_i2osp(&mut num, p, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        p = p.offset(nlen as isize);
        len = (len as libc::c_ulong).wrapping_sub(i) as mln_size_t as mln_size_t;
        in_0 = in_0.offset(i as isize);
    }
    ({
        tmp.data = buf;
        tmp.len = sum;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    free(buf as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSAESPKCS1V15_private_decrypt(
    mut pri: *mut mln_rsa_key_t,
    mut cipher: *mut mln_string_t,
) -> *mut mln_string_t {
    let mut nlen: mln_size_t = ((*pri).n.length << 2 as libc::c_int) as mln_size_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pos: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut in_0: mln_u8ptr_t = (*cipher).data;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut len: mln_size_t = (*cipher).len;
    let mut sum: mln_size_t = 0 as libc::c_int as mln_size_t;
    if (*cipher).len < 11 as libc::c_int as libc::c_ulong
        || ((*cipher).len).wrapping_rem(nlen) != 0
    {
        return 0 as *mut mln_string_t;
    }
    buf = malloc((*cipher).len) as mln_u8ptr_t;
    if buf.is_null() {
        return 0 as *mut mln_string_t;
    }
    p = buf;
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut num: mln_bignum_t = {
            let mut init = mln_bignum_t {
                tag: 0 as libc::c_int as mln_u32_t,
                length: 0 as libc::c_int as mln_u32_t,
                data: [
                    0 as libc::c_int as mln_u64_t,
                ],
            };
            init
        };
        if mln_bignum_os2ip(&mut num, in_0, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_rsa_rsaep_rsadp(pri, &mut num, &mut num) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        if mln_bignum_i2osp(&mut num, in_0, nlen) < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        pos = mln_rsa_anti_padding_public(in_0, nlen);
        if pos.is_null() {
            free(buf as *mut libc::c_void);
            return 0 as *mut mln_string_t;
        }
        memcpy(
            p as *mut libc::c_void,
            pos as *const libc::c_void,
            nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong),
        );
        sum = (sum as libc::c_ulong)
            .wrapping_add(
                nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong),
            ) as mln_size_t as mln_size_t;
        p = p
            .offset(
                nlen.wrapping_sub(pos.offset_from(in_0) as libc::c_long as libc::c_ulong)
                    as isize,
            );
        in_0 = in_0.offset(nlen as isize);
        len = (len as libc::c_ulong).wrapping_sub(nlen) as mln_size_t as mln_size_t;
    }
    ({
        tmp.data = buf;
        tmp.len = sum;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_string_dup(&mut tmp);
    free(buf as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSAESPKCS1V15_free(mut s: *mut mln_string_t) {
    if s.is_null() {
        return;
    }
    let mut __s: *mut mln_string_t = s;
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
#[inline]
unsafe extern "C" fn mln_EMSAPKCS1V15_encode(
    mut pool: *mut mln_alloc_t,
    mut m: *mut mln_string_t,
    mut hash_type: mln_u32_t,
) -> *mut mln_string_t {
    let mut hashval: [mln_u8_t; 32] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut hlen: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut res: mln_asn1_enresult_t = mln_asn1_enresult_t {
        pool: 0 as *mut mln_alloc_t,
        size: 0,
        contents: 0 as *mut mln_string_t,
        max: 0,
        pos: 0,
        _class_is_struct_ident: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut dres: mln_asn1_enresult_t = mln_asn1_enresult_t {
        pool: 0 as *mut mln_alloc_t,
        size: 0,
        contents: 0 as *mut mln_string_t,
        max: 0,
        pos: 0,
        _class_is_struct_ident: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    match hash_type {
        0 => {
            let mut md5: mln_md5_t = mln_md5_t {
                A: 0,
                B: 0,
                C: 0,
                D: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_md5_init(&mut md5);
            mln_md5_calc(&mut md5, (*m).data, (*m).len, 1 as libc::c_int as mln_u32_t);
            hlen = 16 as libc::c_int as mln_u64_t;
            mln_md5_tobytes(&mut md5, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        1 => {
            let mut sha1: mln_sha1_t = mln_sha1_t {
                H0: 0,
                H1: 0,
                H2: 0,
                H3: 0,
                H4: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_sha1_init(&mut sha1);
            mln_sha1_calc(&mut sha1, (*m).data, (*m).len, 1 as libc::c_int as mln_u32_t);
            hlen = 20 as libc::c_int as mln_u64_t;
            mln_sha1_tobytes(&mut sha1, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        2 => {
            let mut sha256: mln_sha256_t = mln_sha256_t {
                H0: 0,
                H1: 0,
                H2: 0,
                H3: 0,
                H4: 0,
                H5: 0,
                H6: 0,
                H7: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_sha256_init(&mut sha256);
            mln_sha256_calc(
                &mut sha256,
                (*m).data,
                (*m).len,
                1 as libc::c_int as mln_u32_t,
            );
            hlen = 32 as libc::c_int as mln_u64_t;
            mln_sha256_tobytes(&mut sha256, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        _ => return 0 as *mut mln_string_t,
    }
    if mln_asn1_enresult_init(&mut res, pool) != 0 as libc::c_int {
        return 0 as *mut mln_string_t;
    }
    if !(mln_asn1_encode_object_identifier(
        &mut res,
        EMSAPKCS1V15_HASH[hash_type as usize].digest_algorithm,
        EMSAPKCS1V15_HASH[hash_type as usize].len,
    ) != 0 as libc::c_int)
    {
        if !(mln_asn1_encode_null(&mut res) != 0 as libc::c_int) {
            if !(mln_asn1_encode_sequence(&mut res) != 0 as libc::c_int) {
                if !(mln_asn1_enresult_init(&mut dres, pool) != 0 as libc::c_int) {
                    if mln_asn1_encode_octetstring(&mut dres, hashval.as_mut_ptr(), hlen)
                        != 0 as libc::c_int
                    {
                        mln_asn1_enresult_destroy(&mut dres);
                    } else if mln_asn1_encode_merge(&mut res, &mut dres)
                        != 0 as libc::c_int
                    {
                        mln_asn1_enresult_destroy(&mut dres);
                    } else {
                        mln_asn1_enresult_destroy(&mut dres);
                        if !(mln_asn1_encode_sequence(&mut res) != 0 as libc::c_int) {
                            if !(mln_asn1_enresult_get_content(
                                &mut res,
                                0 as libc::c_int as mln_u32_t,
                                &mut tmp.data,
                                &mut tmp.len as *mut mln_u64_t,
                            ) != 0 as libc::c_int)
                            {
                                ret = mln_string_dup(&mut tmp);
                                if !ret.is_null() {
                                    mln_asn1_enresult_destroy(&mut res);
                                    return ret;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mln_asn1_enresult_destroy(&mut res);
    return 0 as *mut mln_string_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSASSAPKCS1V15_sign(
    mut pool: *mut mln_alloc_t,
    mut pri: *mut mln_rsa_key_t,
    mut m: *mut mln_string_t,
    mut hash_type: mln_u32_t,
) -> *mut mln_string_t {
    let mut current_block: u64;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut em: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut q: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut len: mln_u64_t = 4096 as libc::c_int as mln_u64_t;
    let mut left: mln_u64_t = 0;
    let mut n: mln_u64_t = 0;
    let mut k: mln_u64_t = 0;
    k = ((*pri).n.length << 2 as libc::c_int)
        .wrapping_sub(11 as libc::c_int as libc::c_uint) as mln_u64_t;
    if k < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut mln_string_t;
    }
    em = mln_EMSAPKCS1V15_encode(pool, m, hash_type);
    if em.is_null() {
        return 0 as *mut mln_string_t;
    }
    buf = malloc(len) as mln_u8ptr_t;
    if !buf.is_null() {
        p = buf;
        q = (*em).data;
        left = (*em).len;
        loop {
            if !(left > 0 as libc::c_int as libc::c_ulong) {
                current_block = 14576567515993809846;
                break;
            }
            n = if left > k { k } else { left };
            left = (left as libc::c_ulong).wrapping_sub(n) as mln_u64_t as mln_u64_t;
            ({
                tmp.data = q;
                tmp.len = n;
                tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                &mut tmp;
                &mut tmp
            });
            q = q.offset(n as isize);
            ret = mln_RSAESPKCS1V15_private_encrypt(pri, &mut tmp);
            if ret.is_null() {
                free(buf as *mut libc::c_void);
                current_block = 16309728171090065370;
                break;
            } else {
                if len.wrapping_sub(p.offset_from(buf) as libc::c_long as libc::c_ulong)
                    < (*ret).len
                {
                    len = (len as libc::c_ulong).wrapping_add(len >> 1 as libc::c_int)
                        as mln_u64_t as mln_u64_t;
                    let mut diff: mln_u64_t = p.offset_from(buf) as libc::c_long
                        as mln_u64_t;
                    let mut ptr: mln_u8ptr_t = realloc(buf as *mut libc::c_void, len)
                        as mln_u8ptr_t;
                    if ptr.is_null() {
                        free(buf as *mut libc::c_void);
                        current_block = 16309728171090065370;
                        break;
                    } else {
                        buf = ptr;
                        p = ptr.offset(diff as isize);
                    }
                }
                memcpy(
                    p as *mut libc::c_void,
                    (*ret).data as *const libc::c_void,
                    (*ret).len,
                );
                p = p.offset((*ret).len as isize);
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
            }
        }
        match current_block {
            16309728171090065370 => {}
            _ => {
                let mut __s: *mut mln_string_t = em;
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
                ({
                    tmp.data = buf;
                    tmp.len = p.offset_from(buf) as libc::c_long as mln_u64_t;
                    tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                    tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                    tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                    &mut tmp;
                    &mut tmp
                });
                ret = mln_string_dup(&mut tmp);
                if ret.is_null() {
                    free(buf as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                    return ret;
                }
            }
        }
    }
    let mut __s: *mut mln_string_t = em;
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
    return 0 as *mut mln_string_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_RSASSAPKCS1V15_verify(
    mut pool: *mut mln_alloc_t,
    mut pub_0: *mut mln_rsa_key_t,
    mut m: *mut mln_string_t,
    mut s: *mut mln_string_t,
) -> libc::c_int {
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut q: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut end: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut len: mln_u64_t = 4096 as libc::c_int as mln_u64_t;
    let mut n: mln_size_t = ((*pub_0).n.length << 2 as libc::c_int) as mln_size_t;
    let mut hashval: [mln_u8_t; 32] = [
        0 as libc::c_int as mln_u8_t,
    ];
    let mut hlen: mln_size_t = 0;
    let mut hash_type: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    if ((*s).len).wrapping_rem(n) != 0 {
        return -(1 as libc::c_int);
    }
    buf = malloc(len) as mln_u8ptr_t;
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    p = buf;
    q = (*s).data;
    end = ((*s).data).offset((*s).len as isize);
    while q < end {
        ({
            tmp.data = q;
            tmp.len = n;
            tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
            tmp.set_pool(0 as libc::c_int as mln_uauto_t);
            tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
            &mut tmp;
            &mut tmp
        });
        ret = mln_RSAESPKCS1V15_public_decrypt(pub_0, &mut tmp);
        if ret.is_null() {
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        if len.wrapping_sub(p.offset_from(buf) as libc::c_long as libc::c_ulong)
            < (*ret).len
        {
            let mut ptr: mln_u8ptr_t = 0 as *mut libc::c_uchar;
            let mut diff: mln_u64_t = p.offset_from(buf) as libc::c_long as mln_u64_t;
            len = (len as libc::c_ulong).wrapping_add(len >> 1 as libc::c_int)
                as mln_u64_t as mln_u64_t;
            ptr = realloc(buf as *mut libc::c_void, len) as mln_u8ptr_t;
            if ptr.is_null() {
                let mut __s: *mut mln_string_t = ret;
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
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            buf = ptr;
            p = ptr.offset(diff as isize);
        }
        memcpy(p as *mut libc::c_void, (*ret).data as *const libc::c_void, (*ret).len);
        p = p.offset((*ret).len as isize);
        q = q.offset(n as isize);
        let mut __s: *mut mln_string_t = ret;
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
    }
    ({
        tmp.data = buf;
        tmp.len = p.offset_from(buf) as libc::c_long as mln_u64_t;
        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
        &mut tmp;
        &mut tmp
    });
    ret = mln_EMSAPKCS1V15_decode(pool, &mut tmp, &mut hash_type);
    if ret.is_null() {
        free(buf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    free(buf as *mut libc::c_void);
    match hash_type {
        0 => {
            let mut md5: mln_md5_t = mln_md5_t {
                A: 0,
                B: 0,
                C: 0,
                D: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_md5_init(&mut md5);
            mln_md5_calc(&mut md5, (*m).data, (*m).len, 1 as libc::c_int as mln_u32_t);
            hlen = 16 as libc::c_int as mln_size_t;
            mln_md5_tobytes(&mut md5, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        1 => {
            let mut sha1: mln_sha1_t = mln_sha1_t {
                H0: 0,
                H1: 0,
                H2: 0,
                H3: 0,
                H4: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_sha1_init(&mut sha1);
            mln_sha1_calc(&mut sha1, (*m).data, (*m).len, 1 as libc::c_int as mln_u32_t);
            hlen = 20 as libc::c_int as mln_size_t;
            mln_sha1_tobytes(&mut sha1, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        2 => {
            let mut sha256: mln_sha256_t = mln_sha256_t {
                H0: 0,
                H1: 0,
                H2: 0,
                H3: 0,
                H4: 0,
                H5: 0,
                H6: 0,
                H7: 0,
                length: 0,
                pos: 0,
                buf: [0; 64],
            };
            mln_sha256_init(&mut sha256);
            mln_sha256_calc(
                &mut sha256,
                (*m).data,
                (*m).len,
                1 as libc::c_int as mln_u32_t,
            );
            hlen = 32 as libc::c_int as mln_size_t;
            mln_sha256_tobytes(&mut sha256, hashval.as_mut_ptr(), hlen as mln_u32_t);
        }
        _ => {
            let mut __s: *mut mln_string_t = ret;
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
            return -(1 as libc::c_int);
        }
    }
    if (*ret).len != hlen
        || memcmp(
            (*ret).data as *const libc::c_void,
            hashval.as_mut_ptr() as *const libc::c_void,
            hlen,
        ) != 0
    {
        let mut __s: *mut mln_string_t = ret;
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
        return -(1 as libc::c_int);
    }
    let mut __s: *mut mln_string_t = ret;
    if !__s.is_null() {
        let ref mut fresh26 = (*__s).ref_0();
        let fresh27 = *fresh26;
        *fresh26 = (*fresh26).wrapping_sub(1);
        if fresh27 <= 1 as libc::c_int as libc::c_ulong {
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
#[inline]
unsafe extern "C" fn mln_EMSAPKCS1V15_decode(
    mut pool: *mut mln_alloc_t,
    mut e: *mut mln_string_t,
    mut hash_type: *mut mln_u32_t,
) -> *mut mln_string_t {
    let mut res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    let mut sub_res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    let mut ssub_res: *mut mln_asn1_deresult_t = 0 as *mut mln_asn1_deresult_t;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut ret: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut t: mln_string_t = mln_string_t {
        data: 0 as *mut libc::c_uchar,
        len: 0,
        data_ref_pool_ref_0: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut code_buf: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut code_len: mln_u64_t = 0;
    let mut p: *mut mln_EMSAPKCS1V15_HASH_s = 0 as *mut mln_EMSAPKCS1V15_HASH_s;
    let mut end: *mut mln_EMSAPKCS1V15_HASH_s = 0 as *mut mln_EMSAPKCS1V15_HASH_s;
    res = mln_asn1_decode_ref((*e).data as *mut libc::c_void, (*e).len, &mut err, pool);
    if res.is_null() {
        return 0 as *mut mln_string_t;
    }
    if !((*res).ident() as libc::c_int != 16 as libc::c_int
        || (*res).pos != 2 as libc::c_int as libc::c_uint)
    {
        sub_res = mln_asn1_deresult_content_get(res, 0 as libc::c_int as mln_u32_t);
        if !((*sub_res).ident() as libc::c_int != 16 as libc::c_int
            || (*sub_res).pos != 2 as libc::c_int as libc::c_uint)
        {
            ssub_res = mln_asn1_deresult_content_get(
                sub_res,
                0 as libc::c_int as mln_u32_t,
            );
            if !((*ssub_res).ident() as libc::c_int != 6 as libc::c_int) {
                ssub_res = mln_asn1_deresult_content_get(
                    ssub_res,
                    0 as libc::c_int as mln_u32_t,
                );
                if !ssub_res.is_null() {
                    code_buf = (*ssub_res).code_buf;
                    code_len = (*ssub_res).code_len;
                    ({
                        tmp.data = code_buf;
                        tmp.len = code_len;
                        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                        &mut tmp;
                        &mut tmp
                    });
                    p = EMSAPKCS1V15_HASH.as_mut_ptr();
                    end = EMSAPKCS1V15_HASH
                        .as_mut_ptr()
                        .offset(
                            (::core::mem::size_of::<[mln_EMSAPKCS1V15_HASH_s; 3]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<mln_EMSAPKCS1V15_HASH_s>()
                                        as libc::c_ulong,
                                ) as isize,
                        );
                    while p < end {
                        ({
                            t.data = (*p).digest_algorithm;
                            t.len = (*p).len;
                            t.set_data_ref(1 as libc::c_int as mln_uauto_t);
                            t.set_pool(0 as libc::c_int as mln_uauto_t);
                            t.set_ref_0(1 as libc::c_int as mln_uauto_t);
                            &mut t;
                            &mut t
                        });
                        if mln_string_strcmp(&mut tmp, &mut t) == 0 {
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    if !(p >= end) {
                        *hash_type = p.offset_from(EMSAPKCS1V15_HASH.as_mut_ptr())
                            as libc::c_long as mln_u32_t;
                        ssub_res = mln_asn1_deresult_content_get(
                            sub_res,
                            1 as libc::c_int as mln_u32_t,
                        );
                        if !((*ssub_res).ident() as libc::c_int != 5 as libc::c_int
                            || (*ssub_res).pos != 1 as libc::c_int as libc::c_uint)
                        {
                            ssub_res = mln_asn1_deresult_content_get(
                                ssub_res,
                                0 as libc::c_int as mln_u32_t,
                            );
                            if !((*ssub_res).code_len
                                != 0 as libc::c_int as libc::c_ulong)
                            {
                                sub_res = mln_asn1_deresult_content_get(
                                    res,
                                    1 as libc::c_int as mln_u32_t,
                                );
                                if !((*sub_res).ident() as libc::c_int != 4 as libc::c_int
                                    || (*sub_res).pos != 1 as libc::c_int as libc::c_uint)
                                {
                                    sub_res = mln_asn1_deresult_content_get(
                                        sub_res,
                                        0 as libc::c_int as mln_u32_t,
                                    );
                                    ({
                                        tmp.data = (*sub_res).code_buf;
                                        tmp.len = (*sub_res).code_len;
                                        tmp.set_data_ref(1 as libc::c_int as mln_uauto_t);
                                        tmp.set_pool(0 as libc::c_int as mln_uauto_t);
                                        tmp.set_ref_0(1 as libc::c_int as mln_uauto_t);
                                        &mut tmp;
                                        &mut tmp
                                    });
                                    ret = mln_string_dup(&mut tmp);
                                    mln_asn1_deresult_free(res);
                                    return ret;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mln_asn1_deresult_free(res);
    return 0 as *mut mln_string_t;
}
