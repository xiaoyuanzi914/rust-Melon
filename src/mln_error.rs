use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type mln_u64_t = libc::c_ulong;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type mln_uauto_t = libc::c_ulong;
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
pub type mln_error_cb_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
>;
#[no_mangle]
pub static mut mln_error_filenames: *mut mln_string_t = 0 as *const mln_string_t
    as *mut mln_string_t;
#[no_mangle]
pub static mut mln_error_errmsgs: *mut mln_string_t = 0 as *const mln_string_t
    as *mut mln_string_t;
#[no_mangle]
pub static mut mln_error_nfile: mln_size_t = 0 as libc::c_int as mln_size_t;
#[no_mangle]
pub static mut mln_error_nmsg: mln_size_t = 0 as libc::c_int as mln_size_t;
#[no_mangle]
pub static mut mln_error_udata: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut mln_error_callback: mln_error_cb_t = None;
#[no_mangle]
pub unsafe extern "C" fn mln_error_init(
    mut filenames: *mut mln_string_t,
    mut errmsgs: *mut mln_string_t,
    mut nfile: mln_size_t,
    mut nmsg: mln_size_t,
) {
    mln_error_filenames = filenames;
    mln_error_errmsgs = errmsgs;
    mln_error_nfile = nfile;
    mln_error_nmsg = nmsg;
}
#[no_mangle]
pub unsafe extern "C" fn mln_error_string(
    mut err: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: mln_size_t,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut b: *mut libc::c_char = buf as *mut libc::c_char;
    let mut f: *mut libc::c_char = b"Unkown File\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut l: *mut libc::c_char = b"Unknown Line\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut c: *mut libc::c_char = b"Unknown Code\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut intstr: [libc::c_char; 8] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if err > 0 as libc::c_int {
        i = snprintf(
            b,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"Invalid error code\0" as *const u8 as *const libc::c_char,
        );
    } else if err < 0 as libc::c_int {
        err = -err;
        i = err >> 22 as libc::c_int & 0x1ff as libc::c_int;
        line = err >> 8 as libc::c_int & 0x3fff as libc::c_int;
        err &= 0xff as libc::c_int;
        snprintf(
            intstr.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%d\0" as *const u8 as *const libc::c_char,
            line,
        );
        i = snprintf(
            b,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%s:%s:%s\0" as *const u8 as *const libc::c_char,
            if i == 0x1ff as libc::c_int || mln_error_filenames.is_null() {
                f
            } else {
                (*mln_error_filenames.offset(i as isize)).data as *mut libc::c_char
            },
            if line == 0x3fff as libc::c_int { l } else { intstr.as_mut_ptr() },
            if err == 0xff as libc::c_int || mln_error_errmsgs.is_null() {
                c
            } else {
                (*mln_error_errmsgs.offset(err as isize)).data as *mut libc::c_char
            },
        );
    } else {
        i = snprintf(
            b,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"%s\0" as *const u8 as *const libc::c_char,
            if mln_error_errmsgs.is_null() {
                b"Success\0" as *const u8 as *const libc::c_char
            } else {
                (*mln_error_errmsgs.offset(err as isize)).data as *mut libc::c_char
                    as *const libc::c_char
            },
        );
    }
    *b.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn mln_error_callback_set(
    mut cb: mln_error_cb_t,
    mut udata: *mut libc::c_void,
) {
    mln_error_callback = cb;
    mln_error_udata = udata;
}
