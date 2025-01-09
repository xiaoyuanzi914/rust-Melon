use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn pthread_self() -> pthread_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_span_s {
    pub begin: timeval,
    pub end: timeval,
    pub file: *const libc::c_char,
    pub func: *const libc::c_char,
    pub line: libc::c_int,
    pub subspans_head: *mut mln_span_s,
    pub subspans_tail: *mut mln_span_s,
    pub parent: *mut mln_span_s,
    pub prev: *mut mln_span_s,
    pub next: *mut mln_span_s,
}
pub type mln_span_t = mln_span_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_span_stack_node_s {
    pub span: *mut mln_span_t,
    pub next: *mut mln_span_stack_node_s,
}
pub type mln_span_stack_node_t = mln_span_stack_node_s;
pub type mln_span_dump_cb_t = Option::<
    unsafe extern "C" fn(*mut mln_span_t, libc::c_int, *mut libc::c_void) -> (),
>;
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
#[no_mangle]
pub static mut __mln_span_stack_top: *mut mln_span_stack_node_t = 0
    as *const mln_span_stack_node_t as *mut mln_span_stack_node_t;
#[no_mangle]
pub static mut __mln_span_stack_bottom: *mut mln_span_stack_node_t = 0
    as *const mln_span_stack_node_t as *mut mln_span_stack_node_t;
#[no_mangle]
pub static mut mln_span_root: *mut mln_span_t = 0 as *const mln_span_t
    as *mut mln_span_t;
#[no_mangle]
pub static mut mln_span_registered_thread: pthread_t = 0;
#[inline]
unsafe extern "C" fn mln_span_chain_add(
    mut head: *mut *mut mln_span_t,
    mut tail: *mut *mut mln_span_t,
    mut node: *mut mln_span_t,
) {
    if head.is_null() || tail.is_null() || node.is_null() {
        return;
    }
    (*node).next = 0 as *mut mln_span_s;
    (*node).prev = (*node).next;
    if (*head).is_null() {
        *tail = node;
        *head = *tail;
        return;
    }
    (**tail).next = node;
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_span_chain_del(
    mut head: *mut *mut mln_span_t,
    mut tail: *mut *mut mln_span_t,
    mut node: *mut mln_span_t,
) {
    if head.is_null() || tail.is_null() || node.is_null() {
        return;
    }
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_span_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_span_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_span_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_span_s;
    (*node).prev = (*node).next;
}
unsafe extern "C" fn mln_span_stack_node_free(mut s: *mut mln_span_stack_node_t) {
    if s.is_null() {
        return;
    }
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn mln_span_stack_top() -> *mut mln_span_t {
    return if __mln_span_stack_top.is_null() {
        0 as *mut mln_span_t
    } else {
        (*__mln_span_stack_top).span
    };
}
unsafe extern "C" fn mln_span_stack_push(mut span: *mut mln_span_t) -> libc::c_int {
    let mut s: *mut mln_span_stack_node_t = 0 as *mut mln_span_stack_node_t;
    s = malloc(::core::mem::size_of::<mln_span_stack_node_t>() as libc::c_ulong)
        as *mut mln_span_stack_node_t;
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    (*s).span = span;
    (*s).next = __mln_span_stack_top;
    if __mln_span_stack_top.is_null() {
        __mln_span_stack_bottom = s;
    }
    __mln_span_stack_top = s;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_span_stack_pop() -> *mut mln_span_t {
    let mut s: *mut mln_span_stack_node_t = 0 as *mut mln_span_stack_node_t;
    s = __mln_span_stack_top;
    if s.is_null() {
        return 0 as *mut mln_span_t;
    }
    let mut span: *mut mln_span_t = (*s).span;
    if __mln_span_stack_bottom == __mln_span_stack_top {
        __mln_span_stack_bottom = 0 as *mut mln_span_stack_node_t;
        __mln_span_stack_top = __mln_span_stack_bottom;
    } else {
        __mln_span_stack_top = (*s).next;
    }
    mln_span_stack_node_free(s);
    return span;
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_stack_free() {
    let mut s: *mut mln_span_stack_node_t = 0 as *mut mln_span_stack_node_t;
    loop {
        s = __mln_span_stack_top;
        if s.is_null() {
            break;
        }
        if __mln_span_stack_bottom == __mln_span_stack_top {
            __mln_span_stack_bottom = 0 as *mut mln_span_stack_node_t;
            __mln_span_stack_top = __mln_span_stack_bottom;
        } else {
            __mln_span_stack_top = (*s).next;
        }
        mln_span_stack_node_free(s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_new(
    mut parent: *mut mln_span_t,
    mut file: *const libc::c_char,
    mut func: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut mln_span_t {
    let mut s: *mut mln_span_t = 0 as *mut mln_span_t;
    s = malloc(::core::mem::size_of::<mln_span_t>() as libc::c_ulong) as *mut mln_span_t;
    if s.is_null() {
        return 0 as *mut mln_span_t;
    }
    memset(
        &mut (*s).begin as *mut timeval as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    memset(
        &mut (*s).end as *mut timeval as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    (*s).file = file;
    (*s).func = func;
    (*s).line = line;
    (*s).subspans_tail = 0 as *mut mln_span_s;
    (*s).subspans_head = (*s).subspans_tail;
    (*s).next = 0 as *mut mln_span_s;
    (*s).prev = (*s).next;
    (*s).parent = parent;
    if !parent.is_null() {
        mln_span_chain_add(
            &mut (*parent).subspans_head,
            &mut (*parent).subspans_tail,
            s,
        );
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_free(mut s: *mut mln_span_t) {
    if s.is_null() {
        return;
    }
    let mut sub: *mut mln_span_t = 0 as *mut mln_span_t;
    loop {
        sub = (*s).subspans_head;
        if sub.is_null() {
            break;
        }
        mln_span_chain_del(&mut (*s).subspans_head, &mut (*s).subspans_tail, sub);
        mln_span_free(sub);
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_entry(
    mut fptr: *mut libc::c_void,
    mut file: *const libc::c_char,
    mut func: *const libc::c_char,
    mut line: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut span: *mut mln_span_t = 0 as *mut mln_span_t;
    if pthread_equal(mln_span_registered_thread, pthread_self()) == 0 {
        return 0 as libc::c_int;
    }
    span = mln_span_new(mln_span_stack_top(), file, func, line);
    if span.is_null() {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"int mln_span_entry(void *, const char *, const char *, int, ...)\0"))
                .as_ptr(),
        );
        'c_3652: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
                158 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"int mln_span_entry(void *, const char *, const char *, int, ...)\0"))
                    .as_ptr(),
            );
        };
        return 0 as libc::c_int;
    }
    if mln_span_stack_push(span) < 0 as libc::c_int {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"int mln_span_entry(void *, const char *, const char *, int, ...)\0"))
                .as_ptr(),
        );
        'c_3547: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"int mln_span_entry(void *, const char *, const char *, int, ...)\0"))
                    .as_ptr(),
            );
        };
        return 0 as libc::c_int;
    }
    if mln_span_root.is_null() {
        mln_span_root = span;
    }
    gettimeofday(&mut (*span).begin, 0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_exit(
    mut fptr: *mut libc::c_void,
    mut file: *const libc::c_char,
    mut func: *const libc::c_char,
    mut line: libc::c_int,
    mut ret: *mut libc::c_void,
    mut args: ...
) {
    if pthread_equal(mln_span_registered_thread, pthread_self()) == 0 {
        return;
    }
    let mut span: *mut mln_span_t = mln_span_stack_pop();
    if span.is_null() {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void mln_span_exit(void *, const char *, const char *, int, void *, ...)\0",
            ))
                .as_ptr(),
        );
        'c_3823: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"src/mln_span.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void mln_span_exit(void *, const char *, const char *, int, void *, ...)\0",
                ))
                    .as_ptr(),
            );
        };
        return;
    }
    gettimeofday(&mut (*span).end, 0 as *mut libc::c_void);
}
unsafe extern "C" fn __mln_span_dump(
    mut s: *mut mln_span_t,
    mut cb: mln_span_dump_cb_t,
    mut data: *mut libc::c_void,
    mut level: libc::c_int,
) {
    if s.is_null() || cb.is_none() {
        return;
    }
    let mut sub: *mut mln_span_t = 0 as *mut mln_span_t;
    cb.expect("non-null function pointer")(s, level, data);
    sub = (*s).subspans_head;
    while !sub.is_null() {
        __mln_span_dump(sub, cb, data, level + 1 as libc::c_int);
        sub = (*sub).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mln_span_dump(
    mut cb: mln_span_dump_cb_t,
    mut data: *mut libc::c_void,
) {
    __mln_span_dump(mln_span_root, cb, data, 0 as libc::c_int);
}
