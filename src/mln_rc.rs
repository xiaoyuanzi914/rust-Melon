use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_uauto_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn mln_rc4_init(
    mut s: mln_u8ptr_t,
    mut key: mln_u8ptr_t,
    mut len: mln_uauto_t,
) {
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut j: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut tmp: mln_u8_t = 0;
    let mut k: [mln_uauto_t; 256] = [
        0 as libc::c_int as mln_uauto_t,
    ];
    i = 0 as libc::c_int as mln_u32_t;
    while i < 256 as libc::c_int as libc::c_uint {
        *s.offset(i as isize) = i as libc::c_uchar;
        k[i
            as usize] = *key.offset((i as libc::c_ulong).wrapping_rem(len) as isize)
            as mln_uauto_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as mln_u32_t;
    while i < 256 as libc::c_int as libc::c_uint {
        j = (j.wrapping_add(*s.offset(i as isize) as libc::c_uint) as libc::c_ulong)
            .wrapping_add(k[i as usize])
            .wrapping_rem(256 as libc::c_int as libc::c_ulong) as mln_u32_t;
        tmp = *s.offset(i as isize);
        *s.offset(i as isize) = *s.offset(j as isize);
        *s.offset(j as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_rc4_calc(
    mut s: mln_u8ptr_t,
    mut data: mln_u8ptr_t,
    mut len: mln_uauto_t,
) {
    let mut i: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut j: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut t: mln_u32_t = 0;
    let mut tmp: mln_u8_t = 0;
    let mut stmp: [mln_u8_t; 256] = [0; 256];
    let mut k: mln_uauto_t = 0;
    memcpy(
        stmp.as_mut_ptr() as *mut libc::c_void,
        s as *const libc::c_void,
        ::core::mem::size_of::<[mln_u8_t; 256]>() as libc::c_ulong,
    );
    k = 0 as libc::c_int as mln_uauto_t;
    while k < len {
        i = i
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(256 as libc::c_int as libc::c_uint);
        j = j
            .wrapping_add(stmp[i as usize] as libc::c_uint)
            .wrapping_rem(256 as libc::c_int as libc::c_uint);
        tmp = stmp[i as usize];
        stmp[i as usize] = stmp[j as usize];
        stmp[j as usize] = tmp;
        t = ((stmp[i as usize] as libc::c_int + stmp[j as usize] as libc::c_int)
            % 256 as libc::c_int) as mln_u32_t;
        let ref mut fresh0 = *data.offset(k as isize);
        *fresh0 = (*fresh0 as libc::c_int ^ stmp[t as usize] as libc::c_int)
            as libc::c_uchar;
        k = k.wrapping_add(1);
        k;
    }
}
