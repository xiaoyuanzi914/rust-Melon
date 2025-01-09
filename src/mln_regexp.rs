use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn mln_array_push(arr: *mut mln_array_t) -> *mut libc::c_void;
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
pub type array_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type array_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type array_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_array_t {
    pub elts: *mut libc::c_void,
    pub size: mln_size_t,
    pub nalloc: mln_size_t,
    pub nelts: mln_size_t,
    pub pool: *mut libc::c_void,
    pub pool_alloc: array_pool_alloc_handler,
    pub pool_free: array_pool_free_handler,
    pub free: array_free,
}
pub type mln_reg_match_result_t = mln_array_t;
unsafe extern "C" fn mln_match_here(
    mut flag: libc::c_uint,
    mut regexp: *mut libc::c_char,
    mut text: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut textlen: libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut steplen: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut c_0: libc::c_int = 0;
    let mut len_0: libc::c_int = 0;
    let mut c_n: libc::c_int = 0;
    let mut len_n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    loop {
        c_0 = mln_get_char(flag, regexp, reglen);
        len_0 = mln_get_length(regexp, reglen);
        steplen = len_0;
        if reglen < 1 as libc::c_int {
            return textlen;
        }
        if flag & 0xffff as libc::c_int as libc::c_uint == 0 {
            if flag & 0x1000000 as libc::c_int as libc::c_uint == 0 {
                ret = mln_process_or(
                    flag,
                    &mut regexp,
                    &mut reglen,
                    &mut text,
                    &mut textlen,
                    matches,
                );
                if ret < 0 as libc::c_int {
                    return -(1 as libc::c_int)
                } else if ret > 0 as libc::c_int {
                    continue;
                }
            }
            if c_0 == 163 as libc::c_int {
                let mut c: libc::c_int = 0;
                let mut len: libc::c_int = reglen;
                count = 0 as libc::c_int;
                while len > 0 as libc::c_int {
                    c = mln_get_char(flag, regexp.offset((reglen - len) as isize), len);
                    if c == 163 as libc::c_int {
                        count += 1;
                        count;
                    }
                    if c == 164 as libc::c_int
                        && {
                            count -= 1;
                            count == 0 as libc::c_int
                        }
                    {
                        break;
                    }
                    len -= mln_get_length(regexp.offset((reglen - len) as isize), len);
                }
                if len <= 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                steplen = reglen - len + len_0;
                if reglen - len == len_0 {
                    regexp = regexp.offset((len_0 << 1 as libc::c_int) as isize);
                    reglen -= len_0 << 1 as libc::c_int;
                    continue;
                }
            }
            if c_0 == 165 as libc::c_int {
                let mut c_1: libc::c_int = 0;
                let mut len_1: libc::c_int = reglen;
                count = 0 as libc::c_int;
                while len_1 > 0 as libc::c_int {
                    c_1 = mln_get_char(
                        flag,
                        regexp.offset((reglen - len_1) as isize),
                        len_1,
                    );
                    if c_1 == 165 as libc::c_int {
                        count += 1;
                        count;
                    }
                    if c_1 == 166 as libc::c_int
                        && {
                            count -= 1;
                            count == 0 as libc::c_int
                        }
                    {
                        break;
                    }
                    len_1
                        -= mln_get_length(
                            regexp.offset((reglen - len_1) as isize),
                            len_1,
                        );
                }
                if len_1 <= 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                steplen = reglen - len_1 + len_0;
                if reglen - len_1 == len_0 {
                    regexp = regexp.offset((len_0 << 1 as libc::c_int) as isize);
                    reglen -= len_0 << 1 as libc::c_int;
                    continue;
                }
            }
            if steplen < reglen {
                c_n = mln_get_char(
                    flag,
                    regexp.offset(steplen as isize),
                    reglen - steplen,
                );
                len_n = mln_get_length(
                    regexp.offset(steplen as isize),
                    reglen - steplen,
                );
                if c_n == 168 as libc::c_int {
                    return mln_match_star(
                        regexp,
                        steplen,
                        regexp.offset(steplen as isize).offset(len_n as isize),
                        text,
                        reglen - steplen - len_n,
                        textlen,
                        matches,
                    );
                }
                if c_n == 172 as libc::c_int {
                    return mln_match_plus(
                        regexp,
                        steplen,
                        regexp.offset(steplen as isize).offset(len_n as isize),
                        text,
                        reglen - steplen - len_n,
                        textlen,
                        matches,
                    );
                }
                if c_n == 171 as libc::c_int {
                    return mln_match_question(
                        regexp,
                        steplen,
                        regexp.offset(steplen as isize).offset(len_n as isize),
                        text,
                        reglen - steplen - len_n,
                        textlen,
                        matches,
                    );
                }
                if c_n == 161 as libc::c_int {
                    let mut part: libc::c_int = 1 as libc::c_int;
                    let mut min: libc::c_int = 0 as libc::c_int;
                    let mut max: libc::c_int = 0 as libc::c_int;
                    let mut existent: libc::c_int = 0 as libc::c_int;
                    let mut c_2: libc::c_int = 0;
                    let mut len_2: libc::c_int = reglen;
                    let mut l: libc::c_int = mln_get_length(
                        regexp.offset(steplen as isize),
                        reglen - steplen,
                    );
                    count = 0 as libc::c_int;
                    while len_2 > steplen {
                        c_2 = mln_get_char(
                            flag,
                            regexp
                                .offset(steplen as isize)
                                .offset((reglen - len_2) as isize),
                            reglen - steplen - (reglen - len_2),
                        );
                        if c_2 == ',' as i32 {
                            part += 1;
                            part;
                            len_2
                                -= mln_get_length(
                                    regexp
                                        .offset(steplen as isize)
                                        .offset((reglen - len_2) as isize),
                                    reglen - steplen - (reglen - len_2),
                                );
                        } else if c_2 == 161 as libc::c_int {
                            count += 1;
                            count;
                            len_2
                                -= mln_get_length(
                                    regexp
                                        .offset(steplen as isize)
                                        .offset((reglen - len_2) as isize),
                                    reglen - steplen - (reglen - len_2),
                                );
                        } else if c_2 == 162 as libc::c_int {
                            count -= 1;
                            if count == 0 as libc::c_int {
                                break;
                            }
                            len_2
                                -= mln_get_length(
                                    regexp
                                        .offset(steplen as isize)
                                        .offset((reglen - len_2) as isize),
                                    reglen - steplen - (reglen - len_2),
                                );
                        } else {
                            if !(c_2 >= '0' as i32 && c_2 <= '9' as i32) {
                                return -(1 as libc::c_int);
                            }
                            existent = 1 as libc::c_int;
                            len_2
                                -= mln_get_length(
                                    regexp
                                        .offset(steplen as isize)
                                        .offset((reglen - len_2) as isize),
                                    reglen - steplen - (reglen - len_2),
                                );
                        }
                    }
                    if len_2 <= steplen || existent == 0 || part > 2 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    mln_match_get_limit(
                        regexp.offset(steplen as isize).offset(l as isize),
                        reglen - len_2 - l,
                        &mut min,
                        &mut max,
                    );
                    if max > 0 as libc::c_int && min > max {
                        return -(1 as libc::c_int);
                    }
                    return mln_match_brace(
                        regexp,
                        steplen,
                        regexp
                            .offset(steplen as isize)
                            .offset((reglen - len_2) as isize)
                            .offset(l as isize),
                        text,
                        len_2 - steplen - l,
                        textlen,
                        min,
                        max,
                        matches,
                    );
                }
            }
            if c_0 == 167 as libc::c_int
                && flag & 0x2000000 as libc::c_int as libc::c_uint != 0
            {
                regexp = regexp.offset(len_0 as isize);
                reglen -= len_0;
                continue;
            } else {
                flag &= !(0x2000000 as libc::c_int as libc::c_uint);
                if c_0 == 169 as libc::c_int {
                    return if textlen < 1 as libc::c_int {
                        textlen
                    } else {
                        -(1 as libc::c_int)
                    };
                }
                if flag & 0x800000 as libc::c_int as libc::c_uint != 0 && reglen > len_0
                    && textlen > 0 as libc::c_int
                {
                    let mut sub_c: libc::c_int = 0;
                    let mut sub_len: libc::c_int = 0;
                    sub_c = mln_get_char(
                        flag,
                        regexp.offset(len_0 as isize),
                        reglen - len_0,
                    );
                    sub_len = mln_get_length(
                        regexp.offset(len_0 as isize),
                        reglen - len_0,
                    );
                    if sub_c == 173 as libc::c_int && reglen > len_0 + sub_len {
                        sub_c = mln_get_char(
                            flag,
                            regexp.offset(len_0 as isize).offset(sub_len as isize),
                            reglen - len_0 - sub_len,
                        );
                        sub_len = mln_get_length(
                            regexp.offset(len_0 as isize).offset(sub_len as isize),
                            reglen - len_0 - sub_len,
                        );
                        if *text.offset(0 as libc::c_int as isize) as libc::c_int >= c_0
                            && *text.offset(0 as libc::c_int as isize) as libc::c_int
                                <= sub_c
                        {
                            textlen -= 1;
                            return textlen;
                        }
                        return -(1 as libc::c_int);
                    }
                }
                if c_0 == 130 as libc::c_int && textlen > 0 as libc::c_int {
                    if !(*text as libc::c_int >= '0' as i32
                        && *text as libc::c_int <= '9' as i32)
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 131 as libc::c_int && textlen > 0 as libc::c_int {
                    if *text as libc::c_int >= '0' as i32
                        && *text as libc::c_int <= '9' as i32
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 128 as libc::c_int && textlen > 0 as libc::c_int {
                    if !(*text as libc::c_int >= 'a' as i32
                        && *text as libc::c_int <= 'z' as i32
                        || *text as libc::c_int >= 'A' as i32
                            && *text as libc::c_int <= 'Z' as i32)
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 129 as libc::c_int && textlen > 0 as libc::c_int {
                    if *text as libc::c_int >= 'a' as i32
                        && *text as libc::c_int <= 'z' as i32
                        || *text as libc::c_int >= 'A' as i32
                            && *text as libc::c_int <= 'Z' as i32
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 132 as libc::c_int && textlen > 0 as libc::c_int {
                    if !(*text as libc::c_int == ' ' as i32
                        || *text as libc::c_int == '\t' as i32
                        || *text as libc::c_int == '\n' as i32
                        || *text as libc::c_int == '\u{c}' as i32
                        || *text as libc::c_int == '\r' as i32
                        || *text as libc::c_int == '\u{b}' as i32)
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 133 as libc::c_int && textlen > 0 as libc::c_int {
                    if *text as libc::c_int == ' ' as i32
                        || *text as libc::c_int == '\t' as i32
                        || *text as libc::c_int == '\n' as i32
                        || *text as libc::c_int == '\u{c}' as i32
                        || *text as libc::c_int == '\r' as i32
                        || *text as libc::c_int == '\u{b}' as i32
                    {
                        return -(1 as libc::c_int);
                    }
                    text = text.offset(1);
                    text;
                    textlen -= 1;
                    textlen;
                    regexp = regexp.offset(len_0 as isize);
                    reglen -= len_0;
                    continue;
                } else if c_0 == 170 as libc::c_int && textlen > 0 as libc::c_int {
                    regexp = regexp.offset(len_0 as isize);
                    text = text.offset(1);
                    text;
                    reglen -= len_0;
                    textlen -= 1;
                    textlen;
                    continue;
                }
            }
        }
        if c_0 == 165 as libc::c_int {
            if mln_match_square(regexp, steplen, &mut text, &mut textlen, matches)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            regexp = regexp.offset(steplen as isize);
            reglen -= steplen;
        } else if c_0 == 163 as libc::c_int {
            let mut left: libc::c_int = mln_match_here(
                0x2000000 as libc::c_int as libc::c_uint,
                regexp.offset(len_0 as isize),
                text,
                steplen - (len_0 << 1 as libc::c_int),
                textlen,
                matches,
            );
            if left < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !matches.is_null() {
                let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
                s = mln_array_push(matches) as *mut mln_string_t;
                if s.is_null() {
                    return -(1 as libc::c_int);
                }
                ({
                    (*s).data = text as mln_u8ptr_t;
                    (*s).len = (textlen - left) as mln_u64_t;
                    (*s).set_data_ref(1 as libc::c_int as mln_uauto_t);
                    (*s).set_pool(0 as libc::c_int as mln_uauto_t);
                    (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
                    s
                });
            }
            regexp = regexp.offset(steplen as isize);
            reglen -= steplen;
            text = text.offset((textlen - left) as isize);
            textlen = left;
        } else if textlen > 0 as libc::c_int
            && c_0
                == *text.offset(0 as libc::c_int as isize) as libc::c_uchar
                    as libc::c_int
        {
            if flag & 0xffff as libc::c_int as libc::c_uint != 0 {
                return textlen;
            }
            regexp = regexp.offset(len_0 as isize);
            text = text.offset(1);
            text;
            reglen -= len_0;
            textlen -= 1;
            textlen;
        } else {
            return -(1 as libc::c_int)
        }
    };
}
#[inline]
unsafe extern "C" fn mln_process_or(
    mut flag: libc::c_uint,
    mut regexp: *mut *mut libc::c_char,
    mut reglen: *mut libc::c_int,
    mut text: *mut *mut libc::c_char,
    mut textlen: *mut libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut rexp: *mut libc::c_char = *regexp;
    let mut rlen: libc::c_int = *reglen;
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut last: libc::c_int = 0 as libc::c_int;
    let mut loop_0: libc::c_int = 0 as libc::c_int;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut match_len: libc::c_int = 0;
    loop {
        c = mln_get_char(flag, rexp, rlen);
        len = mln_get_length(rexp, rlen);
        left = rlen;
        count = 0 as libc::c_int;
        if c == 165 as libc::c_int {
            while left > 0 as libc::c_int {
                c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                left -= mln_get_length(rexp.offset((rlen - left) as isize), left);
                if c == 165 as libc::c_int {
                    count += 1;
                    count;
                }
                if c == 166 as libc::c_int
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    }
                {
                    break;
                }
            }
        } else if c == 163 as libc::c_int {
            while left > 0 as libc::c_int {
                c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                left -= mln_get_length(rexp.offset((rlen - left) as isize), left);
                if c == 163 as libc::c_int {
                    count += 1;
                    count;
                }
                if c == 164 as libc::c_int
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    }
                {
                    break;
                }
            }
        } else {
            if last == 1 as libc::c_int
                && (c == 174 as libc::c_int || rlen <= 0 as libc::c_int)
            {
                if rlen > 0 as libc::c_int {
                    rexp = rexp.offset(len as isize);
                    rlen -= len;
                }
                mln_adjust_or_pos(flag, &mut rexp, &mut rlen);
                return mln_or_return_val(regexp, reglen, rexp, rlen, rv);
            }
            left -= len;
        }
        last = 0 as libc::c_int;
        if left <= 0 as libc::c_int {
            if loop_0 == 0 as libc::c_int {
                return mln_or_return_val(regexp, reglen, rexp, rlen, rv);
            }
            loop_0 = 0 as libc::c_int;
            match_len = rlen;
        } else {
            c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
            len = mln_get_length(rexp.offset((rlen - left) as isize), left);
            if c == 168 as libc::c_int || c == 171 as libc::c_int
                || c == 172 as libc::c_int
            {
                left -= len;
                c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                len = mln_get_length(rexp.offset((rlen - left) as isize), left);
            } else if flag & 0x800000 as libc::c_int as libc::c_uint != 0
                && c == 173 as libc::c_int
            {
                left -= len;
                left -= mln_get_length(rexp.offset((rlen - left) as isize), left);
                c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                len = mln_get_length(rexp.offset((rlen - left) as isize), left);
            } else if c == 161 as libc::c_int {
                while left > 0 as libc::c_int {
                    c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                    left -= mln_get_length(rexp.offset((rlen - left) as isize), left);
                    if c == 161 as libc::c_int {
                        count += 1;
                        count;
                    }
                    if c == 162 as libc::c_int
                        && {
                            count -= 1;
                            count == 0 as libc::c_int
                        }
                    {
                        break;
                    }
                }
                c = mln_get_char(flag, rexp.offset((rlen - left) as isize), left);
                len = mln_get_length(rexp.offset((rlen - left) as isize), left);
            }
            if c != 174 as libc::c_int {
                if loop_0 == 0 as libc::c_int {
                    return mln_or_return_val(regexp, reglen, rexp, rlen, rv);
                }
                loop_0 = 0 as libc::c_int;
                match_len = if left > 0 as libc::c_int { rlen - left } else { rlen };
            } else {
                rv = 1 as libc::c_int;
                loop_0 = rv;
                last = loop_0;
                match_len = rlen - left;
            }
        }
        ret = mln_match_here(
            flag | 0x1000000 as libc::c_int as libc::c_uint,
            rexp,
            *text,
            match_len,
            *textlen,
            matches,
        );
        rexp = rexp.offset(match_len as isize);
        rlen -= match_len;
        if left > 0 as libc::c_int && c == 174 as libc::c_int {
            rexp = rexp.offset(len as isize);
            rlen -= len;
        }
        if ret >= 0 as libc::c_int {
            mln_adjust_or_pos(flag, &mut rexp, &mut rlen);
            *text = (*text).offset((*textlen - ret) as isize);
            *textlen = ret;
            return mln_or_return_val(regexp, reglen, rexp, rlen, rv);
        }
        if !(loop_0 != 0) {
            break;
        }
    }
    return mln_or_return_val(regexp, reglen, rexp, rlen, -(1 as libc::c_int));
}
unsafe extern "C" fn mln_or_return_val(
    mut regexp: *mut *mut libc::c_char,
    mut reglen: *mut libc::c_int,
    mut rexp: *mut libc::c_char,
    mut rlen: libc::c_int,
    mut rv: libc::c_int,
) -> libc::c_int {
    *regexp = rexp;
    *reglen = rlen;
    return rv;
}
#[inline]
unsafe extern "C" fn mln_adjust_or_pos(
    mut flag: libc::c_uint,
    mut rexp: *mut *mut libc::c_char,
    mut rlen: *mut libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    loop {
        if *rlen == 0 as libc::c_int {
            return;
        }
        c = mln_get_char(flag, *rexp, *rlen);
        len = mln_get_length(*rexp, *rlen);
        count = 0 as libc::c_int;
        if c == 165 as libc::c_int {
            while *rlen > 0 as libc::c_int {
                c = mln_get_char(flag, *rexp, *rlen);
                len = mln_get_length(*rexp, *rlen);
                *rexp = (*rexp).offset(len as isize);
                *rlen -= len;
                if c == 165 as libc::c_int {
                    count += 1;
                    count;
                }
                if c == 166 as libc::c_int
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    }
                {
                    break;
                }
            }
        } else if c == 163 as libc::c_int {
            while *rlen > 0 as libc::c_int {
                c = mln_get_char(flag, *rexp, *rlen);
                len = mln_get_length(*rexp, *rlen);
                *rexp = (*rexp).offset(len as isize);
                *rlen -= len;
                if c == 163 as libc::c_int {
                    count += 1;
                    count;
                }
                if c == 164 as libc::c_int
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    }
                {
                    break;
                }
            }
        } else {
            *rexp = (*rexp).offset(len as isize);
            *rlen -= len;
        }
        if *rlen <= 0 as libc::c_int {
            return;
        }
        c = mln_get_char(flag, *rexp, *rlen);
        len = mln_get_length(*rexp, *rlen);
        if c == 168 as libc::c_int || c == 171 as libc::c_int || c == 172 as libc::c_int
        {
            *rexp = (*rexp).offset(len as isize);
            *rlen -= len;
            if *rlen <= 0 as libc::c_int {
                return;
            }
            c = mln_get_char(flag, *rexp, *rlen);
            len = mln_get_length(*rexp, *rlen);
        } else if flag & 0x800000 as libc::c_int as libc::c_uint != 0
            && c == 173 as libc::c_int
        {
            *rexp = (*rexp).offset(len as isize);
            *rlen -= len;
            if *rlen <= 0 as libc::c_int {
                return;
            }
            len = mln_get_length(*rexp, *rlen);
            *rexp = (*rexp).offset(len as isize);
            *rlen -= len;
            if *rlen <= 0 as libc::c_int {
                return;
            }
            c = mln_get_char(flag, *rexp, *rlen);
            len = mln_get_length(*rexp, *rlen);
        } else if c == 161 as libc::c_int {
            while *rlen > 0 as libc::c_int {
                c = mln_get_char(flag, *rexp, *rlen);
                len = mln_get_length(*rexp, *rlen);
                *rexp = (*rexp).offset(len as isize);
                *rlen -= len;
                if c == 163 as libc::c_int {
                    count += 1;
                    count;
                }
                if c == 164 as libc::c_int
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    }
                {
                    break;
                }
            }
            if *rlen <= 0 as libc::c_int {
                return;
            }
            c = mln_get_char(flag, *rexp, *rlen);
            len = mln_get_length(*rexp, *rlen);
        }
        if !(c == 174 as libc::c_int) {
            break;
        }
        *rexp = (*rexp).offset(len as isize);
        *rlen -= len;
    };
}
#[inline]
unsafe extern "C" fn mln_match_square(
    mut regexp: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut text: *mut *mut libc::c_char,
    mut textlen: *mut libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut reverse: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut steplen: libc::c_int = 0;
    let mut end_c: libc::c_int = 0;
    let mut tmp_c: libc::c_int = 0;
    let mut tmp_len: libc::c_int = 0;
    len = mln_get_length(regexp, reglen);
    regexp = regexp.offset(len as isize);
    reglen -= len;
    c = mln_get_char(0x800000 as libc::c_int as libc::c_uint, regexp, reglen);
    len = mln_get_length(regexp, reglen);
    if c == 167 as libc::c_int {
        reverse = 1 as libc::c_int;
        regexp = regexp.offset(len as isize);
        reglen -= len;
    }
    while reglen > 0 as libc::c_int {
        c = mln_get_char(0x800000 as libc::c_int as libc::c_uint, regexp, reglen);
        len = mln_get_length(regexp, reglen);
        if c == 166 as libc::c_int {
            break;
        }
        match c {
            161 => {
                end_c = 162 as libc::c_int;
            }
            163 => {
                end_c = 164 as libc::c_int;
            }
            165 => {
                end_c = 166 as libc::c_int;
            }
            _ => {
                if c > 160 as libc::c_int {
                    regexp = regexp.offset(len as isize);
                    reglen -= len;
                    continue;
                } else {
                    end_c = 0 as libc::c_int;
                }
            }
        }
        if end_c == 0 as libc::c_int {
            steplen = len;
            if reglen > len {
                let mut sub_c: libc::c_int = 0;
                let mut sub_len: libc::c_int = 0;
                sub_c = mln_get_char(
                    0x800000 as libc::c_int as libc::c_uint,
                    regexp.offset(len as isize),
                    reglen - len,
                );
                sub_len = mln_get_length(regexp.offset(len as isize), reglen - len);
                if sub_c == 173 as libc::c_int && reglen > len + sub_len {
                    steplen
                        += sub_len
                            + mln_get_length(
                                regexp.offset(len as isize).offset(sub_len as isize),
                                reglen - len - sub_len,
                            );
                }
            }
        } else {
            steplen = -(1 as libc::c_int);
            count = 0 as libc::c_int;
            tmp_len = reglen;
            while tmp_len > 1 as libc::c_int {
                tmp_c = mln_get_char(
                    0x800000 as libc::c_int as libc::c_uint,
                    regexp.offset((reglen - tmp_len) as isize),
                    tmp_len,
                );
                tmp_len
                    -= mln_get_length(
                        regexp.offset((reglen - tmp_len) as isize),
                        tmp_len,
                    );
                if tmp_c == c {
                    count += 1;
                    count;
                }
                if !(tmp_c == end_c
                    && {
                        count -= 1;
                        count == 0 as libc::c_int
                    })
                {
                    continue;
                }
                steplen = reglen - tmp_len;
                break;
            }
            if steplen < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if steplen == len << 1 as libc::c_int {
                regexp = regexp.offset(steplen as isize);
                reglen = tmp_len;
                continue;
            }
        }
        if *textlen <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        left = mln_match_here(
            0x800000 as libc::c_int as libc::c_uint,
            regexp,
            *text,
            steplen,
            *textlen,
            matches,
        );
        if left >= 0 as libc::c_int {
            if reverse == 0 {
                *text = (*text).offset((*textlen - left) as isize);
                *textlen = left;
                return left;
            } else {
                return -(1 as libc::c_int)
            }
        }
        regexp = regexp.offset(steplen as isize);
        reglen -= steplen;
    }
    if reverse != 0 {
        if *textlen <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        *textlen -= 1;
        *textlen;
        *text = (*text).offset(1);
        *text;
        return *textlen;
    }
    if *textlen <= 0 as libc::c_int {
        return *textlen;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_match_get_limit(
    mut regexp: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut min: *mut libc::c_int,
    mut max: *mut libc::c_int,
) {
    let mut val: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = regexp.offset(reglen as isize);
    val = 0 as libc::c_int;
    p = regexp;
    while p < end && *p as libc::c_int != ',' as i32 {
        val += val * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
        p = p.offset(1);
        p;
    }
    *max = val;
    *min = *max;
    if p >= end {
        return;
    }
    *max = -(1 as libc::c_int);
    val = 0 as libc::c_int;
    p = p.offset(1);
    p;
    while p < end {
        val += val * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
        p = p.offset(1);
        p;
    }
    if val == 0 as libc::c_int {
        return;
    }
    *max = val;
}
unsafe extern "C" fn mln_match_star(
    mut mregexp: *mut libc::c_char,
    mut mreglen: libc::c_int,
    mut regexp: *mut libc::c_char,
    mut text: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut textlen: libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut dot: libc::c_char = 170 as libc::c_int as libc::c_char;
    let mut record_text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut record_len: libc::c_int = -(1 as libc::c_int);
    if textlen <= 0 as libc::c_int {
        return textlen;
    }
    if mreglen > 1 as libc::c_int {
        let mut found: libc::c_int = 0 as libc::c_int;
        loop {
            ret = mln_match_here(
                0 as libc::c_int as libc::c_uint,
                mregexp,
                text,
                mreglen,
                textlen,
                matches,
            );
            if ret < 0 as libc::c_int {
                if reglen <= 0 as libc::c_int {
                    return if found != 0 { textlen } else { ret };
                }
                ret = mln_match_here(
                    0 as libc::c_int as libc::c_uint,
                    regexp,
                    text,
                    reglen,
                    textlen,
                    matches,
                );
                if found != 0 {
                    return if ret < 0 as libc::c_int { textlen } else { ret }
                } else {
                    return ret
                }
            } else {
                found = 1 as libc::c_int;
                text = text.offset((textlen - ret) as isize);
                textlen = ret;
                if textlen > 0 as libc::c_int {
                    continue;
                }
                if reglen > 0 as libc::c_int {
                    return mln_match_here(
                        0 as libc::c_int as libc::c_uint,
                        regexp,
                        text,
                        reglen,
                        textlen,
                        matches,
                    );
                }
                return 0 as libc::c_int;
            }
        }
    }
    while textlen > 0 as libc::c_int
        && (mln_match_here(
            168 as libc::c_int as libc::c_uint,
            mregexp,
            text,
            mreglen,
            textlen,
            matches,
        ) >= 0 as libc::c_int
            || mln_match_here(
                168 as libc::c_int as libc::c_uint,
                mregexp,
                &mut dot,
                mreglen,
                1 as libc::c_int,
                matches,
            ) >= 0 as libc::c_int)
    {
        text = text.offset(1);
        text;
        textlen -= 1;
        textlen;
        if reglen > 0 as libc::c_int {
            if mln_match_here(
                0 as libc::c_int as libc::c_uint,
                regexp,
                text,
                reglen,
                textlen,
                0 as *mut mln_reg_match_result_t,
            ) >= 0 as libc::c_int
            {
                record_text = text;
                record_len = textlen;
            }
        }
    }
    if reglen > 0 as libc::c_int {
        if !record_text.is_null() {
            text = record_text;
            textlen = record_len;
        }
        return mln_match_here(
            0 as libc::c_int as libc::c_uint,
            regexp,
            text,
            reglen,
            textlen,
            matches,
        );
    }
    return textlen;
}
unsafe extern "C" fn mln_match_plus(
    mut mregexp: *mut libc::c_char,
    mut mreglen: libc::c_int,
    mut regexp: *mut libc::c_char,
    mut text: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut textlen: libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut dot: libc::c_char = 170 as libc::c_int as libc::c_char;
    if mreglen > 1 as libc::c_int {
        loop {
            ret = mln_match_here(
                0 as libc::c_int as libc::c_uint,
                mregexp,
                text,
                mreglen,
                textlen,
                matches,
            );
            if ret < 0 as libc::c_int {
                if found == 0 as libc::c_int {
                    return ret;
                }
                return mln_match_here(
                    0 as libc::c_int as libc::c_uint,
                    regexp,
                    text,
                    reglen,
                    textlen,
                    matches,
                );
            } else {
                found = 1 as libc::c_int;
                text = text.offset((textlen - ret) as isize);
                textlen = ret;
                if textlen > 0 as libc::c_int {
                    continue;
                }
                if reglen > 0 as libc::c_int {
                    return mln_match_here(
                        0 as libc::c_int as libc::c_uint,
                        regexp,
                        text,
                        reglen,
                        textlen,
                        matches,
                    );
                }
                return 0 as libc::c_int;
            }
        }
    }
    while textlen > 0 as libc::c_int
        && (mln_match_here(
            172 as libc::c_int as libc::c_uint,
            mregexp,
            text,
            mreglen,
            textlen,
            matches,
        ) >= 0 as libc::c_int
            || mln_match_here(
                172 as libc::c_int as libc::c_uint,
                mregexp,
                &mut dot,
                mreglen,
                1 as libc::c_int,
                matches,
            ) >= 0 as libc::c_int)
    {
        found = 1 as libc::c_int;
        text = text.offset(1);
        text;
        textlen -= 1;
        textlen;
    }
    if found != 0 {
        if textlen > 0 as libc::c_int {
            return mln_match_here(
                0 as libc::c_int as libc::c_uint,
                regexp,
                text,
                reglen,
                textlen,
                matches,
            );
        }
        return textlen;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mln_match_question(
    mut mregexp: *mut libc::c_char,
    mut mreglen: libc::c_int,
    mut regexp: *mut libc::c_char,
    mut text: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut textlen: libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if mreglen > 1 as libc::c_int {
        ret = mln_match_here(
            0 as libc::c_int as libc::c_uint,
            mregexp,
            text,
            mreglen,
            textlen,
            matches,
        );
        if ret >= 0 as libc::c_int {
            text = text.offset((textlen - ret) as isize);
            textlen = ret;
        }
        return mln_match_here(
            0 as libc::c_int as libc::c_uint,
            regexp,
            text,
            reglen,
            textlen,
            matches,
        );
    }
    if mln_match_here(
        171 as libc::c_int as libc::c_uint,
        mregexp,
        text,
        mreglen,
        textlen,
        matches,
    ) >= 0 as libc::c_int
    {
        return mln_match_here(
            0 as libc::c_int as libc::c_uint,
            regexp,
            text.offset(1 as libc::c_int as isize),
            reglen,
            textlen - 1 as libc::c_int,
            matches,
        );
    }
    return mln_match_here(
        0 as libc::c_int as libc::c_uint,
        regexp,
        text,
        reglen,
        textlen,
        matches,
    );
}
unsafe extern "C" fn mln_match_brace(
    mut mregexp: *mut libc::c_char,
    mut mreglen: libc::c_int,
    mut regexp: *mut libc::c_char,
    mut text: *mut libc::c_char,
    mut reglen: libc::c_int,
    mut textlen: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut dot: libc::c_char = 170 as libc::c_int as libc::c_char;
    if mreglen > 1 as libc::c_int {
        loop {
            ret = mln_match_here(
                0 as libc::c_int as libc::c_uint,
                mregexp,
                text,
                mreglen,
                textlen,
                matches,
            );
            if ret < 0 as libc::c_int {
                if reglen <= 0 as libc::c_int || found < min {
                    return ret;
                }
                return mln_match_here(
                    0 as libc::c_int as libc::c_uint,
                    regexp,
                    text,
                    reglen,
                    textlen,
                    matches,
                );
            } else {
                found += 1;
                found;
                text = text.offset((textlen - ret) as isize);
                textlen = ret;
                if textlen > 0 as libc::c_int && (max < 0 as libc::c_int || found < max)
                {
                    continue;
                }
                if textlen <= 0 as libc::c_int && reglen <= 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
                return mln_match_here(
                    0 as libc::c_int as libc::c_uint,
                    regexp,
                    text,
                    reglen,
                    textlen,
                    matches,
                );
            }
        }
    }
    while textlen > 0 as libc::c_int
        && (mln_match_here(
            161 as libc::c_int as libc::c_uint,
            mregexp,
            text,
            mreglen,
            textlen,
            matches,
        ) >= 0 as libc::c_int
            || mln_match_here(
                161 as libc::c_int as libc::c_uint,
                mregexp,
                &mut dot,
                mreglen,
                1 as libc::c_int,
                matches,
            ) >= 0 as libc::c_int)
    {
        found += 1;
        found;
        text = text.offset(1);
        text;
        textlen -= 1;
        textlen;
        if max > 0 as libc::c_int && found >= max {
            break;
        }
    }
    if found >= min {
        if textlen > 0 as libc::c_int || reglen > 0 as libc::c_int {
            return mln_match_here(
                0 as libc::c_int as libc::c_uint,
                regexp,
                text,
                reglen,
                textlen,
                matches,
            );
        }
        return textlen;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_get_char(
    mut flag: libc::c_uint,
    mut s: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if len <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
        && len > 1 as libc::c_int
    {
        match *s.offset(1 as libc::c_int as isize) as libc::c_int {
            110 => return '\n' as i32,
            116 => return '\t' as i32,
            45 => return '-' as i32,
            124 => return '|' as i32,
            92 => return '\\' as i32,
            119 => return 128 as libc::c_int,
            87 => return 129 as libc::c_int,
            100 => return 130 as libc::c_int,
            68 => return 131 as libc::c_int,
            115 => return 132 as libc::c_int,
            83 => return 133 as libc::c_int,
            _ => return *s.offset(1 as libc::c_int as isize) as libc::c_int,
        }
    }
    match *s.offset(0 as libc::c_int as isize) as libc::c_int {
        123 => return 161 as libc::c_int,
        125 => return 162 as libc::c_int,
        40 => return 163 as libc::c_int,
        41 => return 164 as libc::c_int,
        91 => return 165 as libc::c_int,
        93 => return 166 as libc::c_int,
        94 => return 167 as libc::c_int,
        42 => return 168 as libc::c_int,
        36 => return 169 as libc::c_int,
        46 => return 170 as libc::c_int,
        63 => return 171 as libc::c_int,
        43 => return 172 as libc::c_int,
        45 => {
            if flag & 0x800000 as libc::c_int as libc::c_uint != 0 {
                return 173 as libc::c_int;
            }
            return '-' as i32;
        }
        124 => return 174 as libc::c_int,
        _ => {}
    }
    return *s.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_get_length(
    mut s: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
        && len > 1 as libc::c_int
    {
        match *s.offset(1 as libc::c_int as isize) as libc::c_int {
            110 | 116 | 45 | 124 | 119 | 87 | 100 | 68 | 115 | 83 | 92 | _ => {}
        }
        return 2 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_reg_match(
    mut exp: *mut mln_string_t,
    mut text: *mut mln_string_t,
    mut matches: *mut mln_reg_match_result_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut s: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut p: mln_u8ptr_t = (*text).data;
    let mut len: mln_size_t = (*text).len;
    loop {
        ret = mln_match_here(
            0x2000000 as libc::c_int as libc::c_uint,
            (*exp).data as *mut libc::c_char,
            p as *mut libc::c_char,
            (*exp).len as libc::c_int,
            len as libc::c_int,
            matches,
        );
        if ret < 0 as libc::c_int {
            if (*exp).len != 0
                && *((*exp).data).offset(0 as libc::c_int as isize) as libc::c_int
                    != '^' as i32 && len > 0 as libc::c_int as libc::c_ulong
            {
                p = p.offset(1);
                p;
                len = len.wrapping_sub(1);
                len;
            } else {
                return (*matches).nelts as libc::c_int
            }
        } else {
            if !(len.wrapping_sub(ret as libc::c_ulong)
                > 0 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            s = mln_array_push(matches) as *mut mln_string_t;
            if s.is_null() {
                return -(1 as libc::c_int);
            }
            ({
                (*s).data = p;
                (*s).len = len.wrapping_sub(ret as libc::c_ulong);
                (*s).set_data_ref(1 as libc::c_int as mln_uauto_t);
                (*s).set_pool(0 as libc::c_int as mln_uauto_t);
                (*s).set_ref_0(1 as libc::c_int as mln_uauto_t);
                s
            });
            if !(ret as libc::c_ulong >= (*exp).len) {
                break;
            }
            p = p.offset(len.wrapping_sub(ret as libc::c_ulong) as isize);
            len = ret as mln_size_t;
        }
    }
    return (*matches).nelts as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_reg_equal(
    mut exp: *mut mln_string_t,
    mut text: *mut mln_string_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut p: mln_u8ptr_t = (*text).data;
    let mut len: mln_size_t = (*text).len;
    loop {
        ret = mln_match_here(
            0x2000000 as libc::c_int as libc::c_uint,
            (*exp).data as *mut libc::c_char,
            p as *mut libc::c_char,
            (*exp).len as libc::c_int,
            len as libc::c_int,
            0 as *mut mln_reg_match_result_t,
        );
        if ret < 0 as libc::c_int {
            if (*exp).len != 0
                && *((*exp).data).offset(0 as libc::c_int as isize) as libc::c_int
                    != '^' as i32 && len > 0 as libc::c_int as libc::c_ulong
            {
                p = p.offset(1);
                p;
                len = len.wrapping_sub(1);
                len;
            } else {
                return ret
            }
        } else {
            return if ret != 0 && (*exp).len != 0
                && *((*exp).data)
                    .offset(
                        ((*exp).len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int != '$' as i32
            {
                0 as libc::c_int
            } else {
                ret
            }
        }
    };
}
