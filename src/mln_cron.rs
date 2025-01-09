use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn mln_string_slice_free(array: *mut mln_string_t);
    fn mln_string_slice(
        s: *mut mln_string_t,
        sep_array: *const libc::c_char,
    ) -> *mut mln_string_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn mln_time2utc(tm: time_t, uc: *mut utctime);
    fn mln_utc2time(uc: *mut utctime) -> time_t;
    fn mln_utc_adjust(uc: *mut utctime);
    fn mln_month_days(year: libc::c_long, month: libc::c_long) -> libc::c_long;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type mln_u64_t = libc::c_ulong;
pub type mln_u8ptr_t = *mut libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utctime {
    pub year: libc::c_long,
    pub month: libc::c_long,
    pub day: libc::c_long,
    pub hour: libc::c_long,
    pub minute: libc::c_long,
    pub second: libc::c_long,
    pub week: libc::c_long,
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mln_cron_parse(
    mut exp: *mut mln_string_t,
    mut base: time_t,
) -> time_t {
    let mut u: utctime = utctime {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0,
        week: 0,
    };
    let mut arr: *mut mln_string_t = 0 as *mut mln_string_t;
    let mut tmp: time_t = 0;
    let mut week: libc::c_long = 0;
    mln_time2utc(base, &mut u);
    arr = mln_string_slice(exp, b" \t\0" as *const u8 as *const libc::c_char);
    if arr.is_null() {
        return 0 as libc::c_int as time_t;
    }
    u
        .minute = mln_cron_parse_minute(
        &mut *arr.offset(0 as libc::c_int as isize),
        u.minute,
    );
    if u.minute < 0 as libc::c_int as libc::c_long {
        mln_string_slice_free(arr);
        return 0 as libc::c_int as time_t;
    }
    mln_utc_adjust(&mut u);
    tmp = mln_utc2time(&mut u);
    u
        .hour = mln_cron_parse_hour(
        &mut *arr.offset(1 as libc::c_int as isize),
        u.hour,
        (tmp > base) as libc::c_int,
    );
    if u.hour < 0 as libc::c_int as libc::c_long {
        mln_string_slice_free(arr);
        return 0 as libc::c_int as time_t;
    }
    mln_utc_adjust(&mut u);
    tmp = mln_utc2time(&mut u);
    u
        .day = mln_cron_parse_day(
        &mut *arr.offset(2 as libc::c_int as isize),
        u.year,
        u.month,
        u.day,
        (tmp > base) as libc::c_int,
    );
    if u.day < 0 as libc::c_int as libc::c_long {
        mln_string_slice_free(arr);
        return 0 as libc::c_int as time_t;
    }
    mln_utc_adjust(&mut u);
    tmp = mln_utc2time(&mut u);
    u
        .month = mln_cron_parse_month(
        &mut *arr.offset(3 as libc::c_int as isize),
        u.month,
        (tmp > base) as libc::c_int,
    );
    if u.month < 0 as libc::c_int as libc::c_long {
        mln_string_slice_free(arr);
        return 0 as libc::c_int as time_t;
    }
    mln_utc_adjust(&mut u);
    tmp = mln_utc2time(&mut u);
    week = mln_cron_parse_week(
        &mut *arr.offset(4 as libc::c_int as isize),
        u.week,
        (tmp > base) as libc::c_int,
    );
    if week < 0 as libc::c_int as libc::c_long {
        mln_string_slice_free(arr);
        return 0 as libc::c_int as time_t;
    }
    u.day += week - u.week;
    mln_utc_adjust(&mut u);
    mln_string_slice_free(arr);
    return mln_utc2time(&mut u);
}
unsafe extern "C" fn mln_cron_parse_minute(
    mut smin: *mut mln_string_t,
    mut min: libc::c_long,
) -> libc::c_long {
    if (*smin).len == 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*smin).len == 1 as libc::c_int as libc::c_ulong
        && *((*smin).data).offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
    {
        return min + 1 as libc::c_int as libc::c_long;
    }
    let mut period: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut save: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tmp: libc::c_long = 0;
    let mut head: mln_u8ptr_t = (*smin).data;
    let mut p: mln_u8ptr_t = (*smin).data;
    let mut end: mln_u8ptr_t = ((*smin).data)
        .offset((*smin).len as isize)
        .offset(-(1 as libc::c_int as isize));
    while end >= p {
        if *end as libc::c_int == '/' as i32 {
            period = atol(end.offset(1 as libc::c_int as isize) as *mut libc::c_char);
            if period < 1 as libc::c_int as libc::c_long
                || period >= 60 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
            *end = 0 as libc::c_int as libc::c_uchar;
            break;
        } else {
            end = end.offset(-1);
            end;
        }
    }
    if end < p {
        end = ((*smin).data).offset((*smin).len as isize);
    }
    while p < end {
        match *p as libc::c_int {
            44 => {
                if p == head || p > head.offset(2 as libc::c_int as isize) {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                *p = 0 as libc::c_int as libc::c_uchar;
                tmp = atol(head as *mut libc::c_char);
                if tmp < 1 as libc::c_int as libc::c_long
                    || tmp >= 60 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                if tmp < min {
                    tmp += 60 as libc::c_int as libc::c_long;
                }
                if save == 0 {
                    save = if min == tmp { min + period } else { tmp };
                } else if tmp == min && period < save - min || tmp - min < save - min {
                    save = if min == tmp { min + period } else { tmp };
                }
                p = p.offset(1);
                head = p;
            }
            42 => {
                save = min
                    + (if period != 0 {
                        period
                    } else {
                        1 as libc::c_int as libc::c_long
                    });
                p = p.offset(1);
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    head = p;
                } else {
                    head = p;
                }
            }
            _ => {
                let fresh0 = p;
                p = p.offset(1);
                if !(*fresh0 as libc::c_int >= '0' as i32
                    && {
                        let fresh1 = p;
                        p = p.offset(1);
                        *fresh1 as libc::c_int <= '9' as i32
                    })
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
    }
    if p > head {
        if p > head.offset(2 as libc::c_int as isize) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tmp = atol(head as *mut libc::c_char);
        if tmp < 1 as libc::c_int as libc::c_long
            || tmp >= 60 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if tmp < min {
            tmp += 60 as libc::c_int as libc::c_long;
        }
        if save == 0 {
            save = if min == tmp { min + period } else { tmp };
        } else if tmp == min && period < save - min || tmp - min < save - min {
            save = if min == tmp { min + period } else { tmp };
        }
    }
    return save;
}
unsafe extern "C" fn mln_cron_parse_hour(
    mut shour: *mut mln_string_t,
    mut hour: libc::c_long,
    mut greater: libc::c_int,
) -> libc::c_long {
    if (*shour).len == 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*shour).len == 1 as libc::c_int as libc::c_ulong
        && *((*shour).data).offset(0 as libc::c_int as isize) as libc::c_int
            == '*' as i32
    {
        return hour;
    }
    let mut period: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut save: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tmp: libc::c_long = 0;
    let mut head: mln_u8ptr_t = (*shour).data;
    let mut p: mln_u8ptr_t = (*shour).data;
    let mut end: mln_u8ptr_t = ((*shour).data).offset((*shour).len as isize);
    end = end.offset(-1);
    end;
    while end >= p {
        if *end as libc::c_int == '/' as i32 {
            period = atol(end.offset(1 as libc::c_int as isize) as *mut libc::c_char);
            if period < 1 as libc::c_int as libc::c_long
                || period >= 24 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
            *end = 0 as libc::c_int as libc::c_uchar;
            break;
        } else {
            end = end.offset(-1);
            end;
        }
    }
    if end < p {
        end = ((*shour).data).offset((*shour).len as isize);
    }
    while p < end {
        match *p as libc::c_int {
            44 => {
                if p == head || p > head.offset(2 as libc::c_int as isize) {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                *p = 0 as libc::c_int as libc::c_uchar;
                tmp = atol(head as *mut libc::c_char);
                if tmp < 1 as libc::c_int as libc::c_long
                    || tmp >= 24 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                if tmp < hour {
                    tmp += 24 as libc::c_int as libc::c_long;
                }
                if save == 0 {
                    save = if hour == tmp && greater == 0 { hour + period } else { tmp };
                } else if tmp == hour {
                    if greater != 0 {
                        save = tmp;
                    } else if period < save - hour {
                        save = hour + period;
                    }
                } else if tmp - hour < save - hour {
                    save = tmp;
                }
                p = p.offset(1);
                head = p;
            }
            42 => {
                save = if greater != 0 {
                    hour
                } else {
                    hour
                        + (if period != 0 {
                            period
                        } else {
                            1 as libc::c_int as libc::c_long
                        })
                };
                p = p.offset(1);
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    head = p;
                } else {
                    head = p;
                }
            }
            _ => {
                let fresh2 = p;
                p = p.offset(1);
                if !(*fresh2 as libc::c_int >= '0' as i32
                    && {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 as libc::c_int <= '9' as i32
                    })
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
    }
    if p > head {
        if p > head.offset(2 as libc::c_int as isize) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tmp = atol(head as *mut libc::c_char);
        if tmp < 1 as libc::c_int as libc::c_long
            || tmp >= 24 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if tmp < hour {
            tmp += 24 as libc::c_int as libc::c_long;
        }
        if save == 0 {
            save = if hour == tmp && greater == 0 { hour + period } else { tmp };
        } else if tmp == hour {
            if greater != 0 {
                save = tmp;
            } else if period < save - hour {
                save = hour + period;
            }
        } else if tmp - hour < save - hour {
            save = tmp;
        }
    }
    return save;
}
unsafe extern "C" fn mln_cron_parse_day(
    mut sday: *mut mln_string_t,
    mut year: libc::c_long,
    mut month: libc::c_long,
    mut day: libc::c_long,
    mut greater: libc::c_int,
) -> libc::c_long {
    if (*sday).len == 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*sday).len == 1 as libc::c_int as libc::c_ulong
        && *((*sday).data).offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
    {
        return day;
    }
    let mut period: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut save: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tmp: libc::c_long = 0;
    let mut head: mln_u8ptr_t = (*sday).data;
    let mut p: mln_u8ptr_t = (*sday).data;
    let mut end: mln_u8ptr_t = ((*sday).data)
        .offset((*sday).len as isize)
        .offset(-(1 as libc::c_int as isize));
    while end >= p {
        if *end as libc::c_int == '/' as i32 {
            period = atol(end.offset(1 as libc::c_int as isize) as *mut libc::c_char);
            if period < 1 as libc::c_int as libc::c_long
                || period >= 32 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
            *end = 0 as libc::c_int as libc::c_uchar;
            break;
        } else {
            end = end.offset(-1);
            end;
        }
    }
    if end < p {
        end = ((*sday).data).offset((*sday).len as isize);
    }
    while p < end {
        match *p as libc::c_int {
            44 => {
                if p == head || p > head.offset(2 as libc::c_int as isize) {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                *p = 0 as libc::c_int as libc::c_uchar;
                tmp = atol(head as *mut libc::c_char);
                if tmp < 1 as libc::c_int as libc::c_long
                    || tmp >= 32 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                if tmp < day {
                    tmp += mln_month_days(year, month);
                }
                if save == 0 {
                    save = if day == tmp && greater == 0 { day + period } else { tmp };
                } else if tmp == day {
                    if greater != 0 {
                        save = tmp;
                    } else if period < save - day {
                        save = day + period;
                    }
                } else if tmp - day < save - day {
                    save = tmp;
                }
                p = p.offset(1);
                head = p;
            }
            42 => {
                save = if greater != 0 {
                    day
                } else {
                    day
                        + (if period != 0 {
                            period
                        } else {
                            1 as libc::c_int as libc::c_long
                        })
                };
                p = p.offset(1);
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    head = p;
                } else {
                    head = p;
                }
            }
            _ => {
                let fresh4 = p;
                p = p.offset(1);
                if !(*fresh4 as libc::c_int >= '0' as i32
                    && {
                        let fresh5 = p;
                        p = p.offset(1);
                        *fresh5 as libc::c_int <= '9' as i32
                    })
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
    }
    if p > head {
        if p > head.offset(2 as libc::c_int as isize) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tmp = atol(head as *mut libc::c_char);
        if tmp < 1 as libc::c_int as libc::c_long
            || tmp >= 32 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if tmp < day {
            tmp += mln_month_days(year, month);
        }
        if save == 0 {
            save = if day == tmp && greater == 0 { day + period } else { tmp };
        } else if tmp == day {
            if greater != 0 {
                save = tmp;
            } else if period < save - day {
                save = day + period;
            }
        } else if tmp - day < save - day {
            save = tmp;
        }
    }
    return save;
}
unsafe extern "C" fn mln_cron_parse_month(
    mut smon: *mut mln_string_t,
    mut mon: libc::c_long,
    mut greater: libc::c_int,
) -> libc::c_long {
    if (*smon).len == 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*smon).len == 1 as libc::c_int as libc::c_ulong
        && *((*smon).data).offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
    {
        return mon;
    }
    let mut period: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut save: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tmp: libc::c_long = 0;
    let mut head: mln_u8ptr_t = (*smon).data;
    let mut p: mln_u8ptr_t = (*smon).data;
    let mut end: mln_u8ptr_t = ((*smon).data).offset((*smon).len as isize);
    end = end.offset(-1);
    end;
    while end >= p {
        if *end as libc::c_int == '/' as i32 {
            period = atol(end.offset(1 as libc::c_int as isize) as *mut libc::c_char);
            if period < 1 as libc::c_int as libc::c_long
                || period >= 13 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
            *end = 0 as libc::c_int as libc::c_uchar;
            break;
        } else {
            end = end.offset(-1);
            end;
        }
    }
    if end < p {
        end = ((*smon).data).offset((*smon).len as isize);
    }
    while p < end {
        match *p as libc::c_int {
            44 => {
                if p == head || p > head.offset(2 as libc::c_int as isize) {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                *p = 0 as libc::c_int as libc::c_uchar;
                tmp = atol(head as *mut libc::c_char);
                if tmp < 1 as libc::c_int as libc::c_long
                    || tmp >= 13 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                if tmp < mon {
                    tmp += 12 as libc::c_int as libc::c_long;
                }
                if save == 0 {
                    save = if mon == tmp && greater == 0 { mon + period } else { tmp };
                } else if tmp == mon {
                    if greater != 0 {
                        save = tmp;
                    } else if period < save - mon {
                        save = mon + period;
                    }
                } else if tmp - mon < save - mon {
                    save = tmp;
                }
                p = p.offset(1);
                head = p;
            }
            42 => {
                save = if greater != 0 {
                    mon
                } else {
                    mon
                        + (if period != 0 {
                            period
                        } else {
                            1 as libc::c_int as libc::c_long
                        })
                };
                p = p.offset(1);
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    head = p;
                } else {
                    head = p;
                }
            }
            _ => {
                let fresh6 = p;
                p = p.offset(1);
                if !(*fresh6 as libc::c_int >= '0' as i32
                    && {
                        let fresh7 = p;
                        p = p.offset(1);
                        *fresh7 as libc::c_int <= '9' as i32
                    })
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
    }
    if p > head {
        if p > head.offset(2 as libc::c_int as isize) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tmp = atol(head as *mut libc::c_char);
        if tmp < 1 as libc::c_int as libc::c_long
            || tmp >= 13 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if tmp < mon {
            tmp += 12 as libc::c_int as libc::c_long;
        }
        if save == 0 {
            save = if mon == tmp && greater == 0 { mon + period } else { tmp };
        } else if tmp == mon {
            if greater != 0 {
                save = tmp;
            } else if period < save - mon {
                save = mon + period;
            }
        } else if tmp - mon < save - mon {
            save = tmp;
        }
    }
    return save;
}
unsafe extern "C" fn mln_cron_parse_week(
    mut sweek: *mut mln_string_t,
    mut week: libc::c_long,
    mut greater: libc::c_int,
) -> libc::c_long {
    if (*sweek).len == 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*sweek).len == 1 as libc::c_int as libc::c_ulong
        && *((*sweek).data).offset(0 as libc::c_int as isize) as libc::c_int
            == '*' as i32
    {
        return week;
    }
    let mut period: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut save: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tmp: libc::c_long = 0;
    let mut head: mln_u8ptr_t = (*sweek).data;
    let mut p: mln_u8ptr_t = (*sweek).data;
    let mut end: mln_u8ptr_t = ((*sweek).data).offset((*sweek).len as isize);
    end = end.offset(-1);
    end;
    while end >= p {
        if *end as libc::c_int == '/' as i32 {
            period = atol(end.offset(1 as libc::c_int as isize) as *mut libc::c_char);
            if period < 0 as libc::c_int as libc::c_long
                || period >= 7 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
            *end = 0 as libc::c_int as libc::c_uchar;
            break;
        } else {
            end = end.offset(-1);
            end;
        }
    }
    if end < p {
        end = ((*sweek).data).offset((*sweek).len as isize);
    }
    while p < end {
        match *p as libc::c_int {
            44 => {
                if p == head || p > head.offset(1 as libc::c_int as isize) {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                *p = 0 as libc::c_int as libc::c_uchar;
                tmp = atol(head as *mut libc::c_char);
                if tmp < 0 as libc::c_int as libc::c_long
                    || tmp >= 7 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
                if tmp < week {
                    tmp += 7 as libc::c_int as libc::c_long;
                }
                if save == 0 {
                    save = if week == tmp && greater == 0 { week + period } else { tmp };
                } else if tmp == week {
                    if greater != 0 {
                        save = tmp;
                    } else if period < save - week {
                        save = week + period;
                    }
                } else if tmp - week < save - week {
                    save = tmp;
                }
                p = p.offset(1);
                head = p;
            }
            42 => {
                save = if greater != 0 {
                    week
                } else {
                    week
                        + (if period != 0 {
                            period
                        } else {
                            1 as libc::c_int as libc::c_long
                        })
                };
                p = p.offset(1);
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    head = p;
                } else {
                    head = p;
                }
            }
            _ => {
                let fresh8 = p;
                p = p.offset(1);
                if !(*fresh8 as libc::c_int >= '0' as i32
                    && {
                        let fresh9 = p;
                        p = p.offset(1);
                        *fresh9 as libc::c_int <= '9' as i32
                    })
                {
                    return -(1 as libc::c_int) as libc::c_long;
                }
            }
        }
    }
    if p > head {
        if p > head.offset(1 as libc::c_int as isize) {
            return -(1 as libc::c_int) as libc::c_long;
        }
        tmp = atol(head as *mut libc::c_char);
        if tmp < 0 as libc::c_int as libc::c_long
            || tmp >= 7 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if tmp < week {
            tmp += 7 as libc::c_int as libc::c_long;
        }
        if save == 0 {
            save = if week == tmp && greater == 0 { week + period } else { tmp };
        } else if tmp == week {
            if greater != 0 {
                save = tmp;
            } else if period < save - week {
                save = week + period;
            }
        } else if tmp - week < save - week {
            save = tmp;
        }
    }
    return save;
}
