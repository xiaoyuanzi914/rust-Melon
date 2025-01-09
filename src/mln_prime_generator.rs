use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn mln_prime_generate(mut n: mln_u32_t) -> mln_u32_t {
    if n <= 2 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int as mln_u32_t;
    }
    if n >= 1073741824 as libc::c_int as libc::c_uint {
        return 1073741827 as libc::c_int as mln_u32_t;
    }
    let mut a: mln_u32_t = 0;
    let mut prim: mln_u32_t = if n.wrapping_rem(2 as libc::c_int as libc::c_uint) != 0 {
        n
    } else {
        n.wrapping_add(1 as libc::c_int as libc::c_uint)
    };
    let mut s: libc::c_int = 0;
    loop {
        s = (if prim <= 4 as libc::c_int as libc::c_uint {
            1 as libc::c_int as libc::c_uint
        } else {
            prim.wrapping_sub(1 as libc::c_int as libc::c_uint) >> 2 as libc::c_int
        }) as libc::c_int;
        while s >= 0 as libc::c_int {
            a = rand_scope(n, prim);
            if witness(a, prim) != 0 {
                break;
            }
            s -= 1;
            s;
        }
        if s < 0 as libc::c_int {
            break;
        }
        prim = (prim as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as mln_u32_t as mln_u32_t;
    }
    return prim;
}
#[inline]
unsafe extern "C" fn rand_scope(mut low: mln_u32_t, mut high: mln_u32_t) -> mln_u32_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut r: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    while r == 0 {
        tv.tv_sec = 0 as libc::c_int as __time_t;
        tv.tv_usec = tv.tv_sec;
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        srand(
            (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec)
                as libc::c_uint,
        );
        r = (rand() as mln_u32_t).wrapping_add(low).wrapping_rem(high);
    }
    return r;
}
#[inline]
unsafe extern "C" fn seperate(
    mut num: mln_u32_t,
    mut pwr: *mut mln_u32_t,
    mut odd: *mut mln_u32_t,
) {
    *pwr = 0 as libc::c_int as mln_u32_t;
    while num.wrapping_rem(2 as libc::c_int as libc::c_uint) == 0 {
        num >>= 1 as libc::c_int;
        *pwr = (*pwr).wrapping_add(1);
        *pwr;
    }
    *odd = num;
}
#[inline]
unsafe extern "C" fn modular_expoinentiation(
    mut base: mln_u32_t,
    mut pwr: mln_u32_t,
    mut n: mln_u32_t,
) -> mln_u64_t {
    let mut i: libc::c_int = 0;
    let mut d: mln_u64_t = 1 as libc::c_int as mln_u64_t;
    i = (::core::mem::size_of::<mln_u32_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        d = (d as libc::c_ulong).wrapping_mul(d) as mln_u64_t as mln_u64_t;
        d = (d as libc::c_ulong).wrapping_rem(n as libc::c_ulong) as mln_u64_t
            as mln_u64_t;
        if ((1 as libc::c_int) << i) as libc::c_uint & pwr != 0 {
            d = (d as libc::c_ulong).wrapping_mul(base as libc::c_ulong) as mln_u64_t
                as mln_u64_t;
            d = (d as libc::c_ulong).wrapping_rem(n as libc::c_ulong) as mln_u64_t
                as mln_u64_t;
        }
        i -= 1;
        i;
    }
    return d;
}
#[inline]
unsafe extern "C" fn witness(mut base: mln_u32_t, mut prim: mln_u32_t) -> mln_u32_t {
    let mut pwr: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut odd: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    seperate(prim.wrapping_sub(1 as libc::c_int as libc::c_uint), &mut pwr, &mut odd);
    let mut new_x: mln_u64_t = 0 as libc::c_int as mln_u64_t;
    let mut x: mln_u64_t = modular_expoinentiation(base, odd, prim);
    let mut i: mln_u32_t = 0;
    i = 0 as libc::c_int as mln_u32_t;
    while i < pwr {
        new_x = x.wrapping_mul(x).wrapping_rem(prim as libc::c_ulong);
        if new_x == 1 as libc::c_int as libc::c_ulong
            && x != 1 as libc::c_int as libc::c_ulong
            && x != prim.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
        {
            return 1 as libc::c_int as mln_u32_t;
        }
        x = new_x;
        i = i.wrapping_add(1);
        i;
    }
    if new_x != 1 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int as mln_u32_t;
    }
    return 0 as libc::c_int as mln_u32_t;
}
