use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type mln_u32_t = libc::c_uint;
pub type mln_size_t = size_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_matrix_t {
    pub row: mln_size_t,
    pub col: mln_size_t,
    pub data: *mut libc::c_double,
    #[bitfield(name = "is_ref", ty = "mln_u32_t", bits = "0..=0")]
    pub is_ref: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[no_mangle]
pub unsafe extern "C" fn mln_matrix_new(
    mut row: mln_size_t,
    mut col: mln_size_t,
    mut data: *mut libc::c_double,
    mut is_ref: mln_u32_t,
) -> *mut mln_matrix_t {
    let mut matrix: *mut mln_matrix_t = 0 as *mut mln_matrix_t;
    matrix = malloc(::core::mem::size_of::<mln_matrix_t>() as libc::c_ulong)
        as *mut mln_matrix_t;
    if matrix.is_null() {
        return 0 as *mut mln_matrix_t;
    }
    (*matrix).row = row;
    (*matrix).col = col;
    (*matrix).data = data;
    (*matrix).set_is_ref(is_ref);
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn mln_matrix_free(mut matrix: *mut mln_matrix_t) {
    if matrix.is_null() {
        return;
    }
    if !((*matrix).data).is_null() && (*matrix).is_ref() == 0 {
        free((*matrix).data as *mut libc::c_void);
    }
    free(matrix as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_matrix_mul(
    mut m1: *mut mln_matrix_t,
    mut m2: *mut mln_matrix_t,
) -> *mut mln_matrix_t {
    if (*m1).col != (*m2).row {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    let mut data: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut tmp: libc::c_double = 0.;
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut k: mln_size_t = 0;
    let mut ret: *mut mln_matrix_t = 0 as *mut mln_matrix_t;
    let mut m1row: mln_size_t = (*m1).row;
    let mut m1col: mln_size_t = (*m1).col;
    let mut m2col: mln_size_t = (*m2).col;
    let mut m1data: *mut libc::c_double = (*m1).data;
    let mut m2data: *mut libc::c_double = (*m2).data;
    data = calloc(
        m1row,
        m2col.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    ret = mln_matrix_new(m1row, m2col, data, 0 as libc::c_int as mln_u32_t);
    if ret.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < m1row {
        k = 0 as libc::c_int as mln_size_t;
        while k < m1col {
            tmp = *m1data.offset(i.wrapping_mul(m1col).wrapping_add(k) as isize);
            j = 0 as libc::c_int as mln_size_t;
            while j < m2col {
                *data.offset(i.wrapping_mul(m2col).wrapping_add(j) as isize)
                    += tmp
                        * *m2data.offset(k.wrapping_mul(m2col).wrapping_add(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_matrix_inverse(
    mut matrix: *mut mln_matrix_t,
) -> *mut mln_matrix_t {
    if matrix.is_null() || (*matrix).row != (*matrix).col {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    let mut ret: *mut mln_matrix_t = 0 as *mut mln_matrix_t;
    let mut data: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut origin: *mut libc::c_double = (*matrix).data;
    let mut tmp: libc::c_double = 0.;
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut k: mln_size_t = 0;
    let mut m: mln_size_t = 0;
    let mut n: mln_size_t = ((*matrix).row).wrapping_mul((*matrix).col);
    let mut len: mln_size_t = (*matrix).row;
    data = malloc(
        n.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    ret = mln_matrix_new(
        (*matrix).row,
        (*matrix).col,
        data,
        0 as libc::c_int as mln_u32_t,
    );
    if ret.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_matrix_t;
    }
    i = 0 as libc::c_int as mln_size_t;
    k = 0 as libc::c_int as mln_size_t;
    while i < n {
        j = 0 as libc::c_int as mln_size_t;
        while j < (*ret).col {
            *data
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (if j == k { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_double;
            j = j.wrapping_add(1);
            j;
        }
        i = (i as libc::c_ulong).wrapping_add((*ret).col) as mln_size_t as mln_size_t;
        k = k.wrapping_add(1);
        k;
    }
    m = 0 as libc::c_int as mln_size_t;
    i = 0 as libc::c_int as mln_size_t;
    while i < n {
        tmp = *origin.offset(i.wrapping_add(m) as isize);
        k = i;
        j = i.wrapping_add(len);
        while j < n {
            if fabs(*origin.offset(j.wrapping_add(m) as isize)) > fabs(tmp) {
                tmp = *origin.offset(j.wrapping_add(m) as isize);
                k = j;
            }
            j = (j as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        }
        if k != i {
            j = 0 as libc::c_int as mln_size_t;
            while j < len {
                tmp = *origin.offset(i.wrapping_add(j) as isize);
                *origin
                    .offset(
                        i.wrapping_add(j) as isize,
                    ) = *origin.offset(k.wrapping_add(j) as isize);
                *origin.offset(k.wrapping_add(j) as isize) = tmp;
                tmp = *data.offset(i.wrapping_add(j) as isize);
                *data
                    .offset(
                        i.wrapping_add(j) as isize,
                    ) = *data.offset(k.wrapping_add(j) as isize);
                *data.offset(k.wrapping_add(j) as isize) = tmp;
                j = j.wrapping_add(1);
                j;
            }
        }
        if fabs(*origin.offset(i.wrapping_add(m) as isize)) < 1e-6f64 {
            mln_matrix_free(ret);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut mln_matrix_t;
        }
        tmp = *origin.offset(i.wrapping_add(m) as isize);
        j = 0 as libc::c_int as mln_size_t;
        while j < len {
            *origin.offset(i.wrapping_add(j) as isize) /= tmp;
            *data.offset(i.wrapping_add(j) as isize) /= tmp;
            j = j.wrapping_add(1);
            j;
        }
        j = 0 as libc::c_int as mln_size_t;
        while j < n {
            if j != i {
                tmp = *origin.offset(j.wrapping_add(m) as isize);
                k = 0 as libc::c_int as mln_size_t;
                while k < len {
                    *origin.offset(j.wrapping_add(k) as isize)
                        -= *origin.offset(i.wrapping_add(k) as isize) * tmp;
                    *data.offset(j.wrapping_add(k) as isize)
                        -= *data.offset(i.wrapping_add(k) as isize) * tmp;
                    k = k.wrapping_add(1);
                    k;
                }
            }
            j = (j as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        }
        i = (i as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        m = m.wrapping_add(1);
        m;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mln_matrix_dump(mut matrix: *mut mln_matrix_t) {
    if matrix.is_null() {
        return;
    }
    let mut i: mln_size_t = 0;
    let mut sum: mln_size_t = ((*matrix).row).wrapping_mul((*matrix).col);
    printf(
        b"Matrix row:%lu col:%lu\n \0" as *const u8 as *const libc::c_char,
        (*matrix).row,
        (*matrix).col,
    );
    i = 0 as libc::c_int as mln_size_t;
    while i < sum {
        if i != 0 && i.wrapping_rem((*matrix).col) == 0 {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%f \0" as *const u8 as *const libc::c_char,
            *((*matrix).data).offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
