use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type mln_s8_t = libc::c_char;
pub type mln_u32_t = libc::c_uint;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_iothread_msg_s {
    pub prev: *mut mln_iothread_msg_s,
    pub next: *mut mln_iothread_msg_s,
    #[bitfield(name = "feedback", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "hold", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "2..=31")]
    pub feedback_hold_padding: [u8; 4],
    pub type_0: mln_u32_t,
    pub data: *mut libc::c_void,
    pub mutex: pthread_mutex_t,
}
pub type mln_iothread_msg_t = mln_iothread_msg_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_iothread_s {
    pub io_lock: pthread_mutex_t,
    pub user_lock: pthread_mutex_t,
    pub io_fd: libc::c_int,
    pub user_fd: libc::c_int,
    pub handler: mln_iothread_msg_process_t,
    pub io_head: *mut mln_iothread_msg_t,
    pub io_tail: *mut mln_iothread_msg_t,
    pub entry: mln_iothread_entry_t,
    pub args: *mut libc::c_void,
    pub tids: *mut pthread_t,
    pub nthread: mln_u32_t,
    pub user_head: *mut mln_iothread_msg_t,
    pub user_tail: *mut mln_iothread_msg_t,
}
pub type mln_iothread_entry_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type mln_iothread_msg_process_t = Option::<
    unsafe extern "C" fn(
        *mut mln_iothread_t,
        mln_iothread_ep_type_t,
        *mut mln_iothread_msg_t,
    ) -> (),
>;
pub type mln_iothread_ep_type_t = libc::c_uint;
pub const user_thread: mln_iothread_ep_type_t = 1;
pub const io_thread: mln_iothread_ep_type_t = 0;
pub type mln_iothread_t = mln_iothread_s;
pub const SOCK_STREAM: __socket_type = 1;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
#[inline]
unsafe extern "C" fn mln_iothread_msg_chain_del(
    mut head: *mut *mut mln_iothread_msg_t,
    mut tail: *mut *mut mln_iothread_msg_t,
    mut node: *mut mln_iothread_msg_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_iothread_msg_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_iothread_msg_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_iothread_msg_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_iothread_msg_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_iothread_msg_chain_add(
    mut head: *mut *mut mln_iothread_msg_t,
    mut tail: *mut *mut mln_iothread_msg_t,
    mut node: *mut mln_iothread_msg_t,
) {
    (*node).next = 0 as *mut mln_iothread_msg_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[no_mangle]
pub unsafe extern "C" fn mln_iothread_init(
    mut t: *mut mln_iothread_t,
    mut nthread: mln_u32_t,
    mut entry: mln_iothread_entry_t,
    mut args: *mut libc::c_void,
    mut handler: mln_iothread_msg_process_t,
) -> libc::c_int {
    let mut i: mln_u32_t = 0;
    let mut fds: [libc::c_int; 2] = [0; 2];
    if nthread == 0 || entry.is_none() {
        return -(1 as libc::c_int);
    }
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*t).io_fd = fds[0 as libc::c_int as usize];
    (*t).user_fd = fds[1 as libc::c_int as usize];
    mln_iothread_fd_nonblock_set((*t).io_fd);
    mln_iothread_fd_nonblock_set((*t).user_fd);
    (*t).entry = entry;
    (*t).args = args;
    (*t).handler = handler;
    pthread_mutex_init(&mut (*t).io_lock, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut (*t).user_lock, 0 as *const pthread_mutexattr_t);
    (*t).io_tail = 0 as *mut mln_iothread_msg_t;
    (*t).io_head = (*t).io_tail;
    (*t).user_tail = 0 as *mut mln_iothread_msg_t;
    (*t).user_head = (*t).user_tail;
    (*t).nthread = nthread;
    (*t)
        .tids = calloc(
        (*t).nthread as libc::c_ulong,
        ::core::mem::size_of::<pthread_t>() as libc::c_ulong,
    ) as *mut pthread_t;
    if ((*t).tids).is_null() {
        close(fds[0 as libc::c_int as usize]);
        close(fds[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as mln_u32_t;
    while i < (*t).nthread {
        if pthread_create(
            ((*t).tids).offset(i as isize),
            0 as *const pthread_attr_t,
            (*t).entry,
            (*t).args,
        ) != 0 as libc::c_int
        {
            mln_iothread_destroy(t);
            return -(1 as libc::c_int);
        }
        if pthread_detach(*((*t).tids).offset(i as isize)) != 0 as libc::c_int {
            mln_iothread_destroy(t);
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_iothread_destroy(mut t: *mut mln_iothread_t) {
    if t.is_null() {
        return;
    }
    if !((*t).tids).is_null() {
        let mut i: mln_u32_t = 0;
        i = 0 as libc::c_int as mln_u32_t;
        while i < (*t).nthread {
            pthread_cancel(*((*t).tids).offset(i as isize));
            pthread_join(*((*t).tids).offset(i as isize), 0 as *mut *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*t).tids as *mut libc::c_void);
    }
    close((*t).io_fd);
    close((*t).user_fd);
}
#[no_mangle]
pub unsafe extern "C" fn mln_iothread_send(
    mut t: *mut mln_iothread_t,
    mut type_0: mln_u32_t,
    mut data: *mut libc::c_void,
    mut to: mln_iothread_ep_type_t,
    mut feedback: mln_u32_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut plock: *mut pthread_mutex_t = 0 as *mut pthread_mutex_t;
    let mut msg: *mut mln_iothread_msg_t = 0 as *mut mln_iothread_msg_t;
    let mut head: *mut *mut mln_iothread_msg_t = 0 as *mut *mut mln_iothread_msg_t;
    let mut tail: *mut *mut mln_iothread_msg_t = 0 as *mut *mut mln_iothread_msg_t;
    if to as libc::c_uint == io_thread as libc::c_int as libc::c_uint {
        fd = (*t).user_fd;
        plock = &mut (*t).io_lock;
        head = &mut (*t).io_head;
        tail = &mut (*t).io_tail;
    } else {
        fd = (*t).io_fd;
        plock = &mut (*t).user_lock;
        head = &mut (*t).user_head;
        tail = &mut (*t).user_tail;
    }
    msg = mln_iothread_msg_new(type_0, data, feedback as libc::c_int);
    if msg.is_null() {
        return -(1 as libc::c_int);
    }
    if feedback != 0 {
        pthread_mutex_lock(&mut (*msg).mutex);
    }
    pthread_mutex_lock(plock);
    mln_iothread_msg_chain_add(head, tail, msg);
    if *head == *tail && *head == msg
        && send(
            fd,
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
            0 as libc::c_int,
        ) != 1 as libc::c_int as libc::c_long
    {
        mln_iothread_msg_chain_del(head, tail, msg);
        if feedback != 0 {
            pthread_mutex_unlock(&mut (*msg).mutex);
        }
        pthread_mutex_unlock(plock);
        mln_iothread_msg_free(msg);
        return 1 as libc::c_int;
    }
    pthread_mutex_unlock(plock);
    if feedback != 0 {
        pthread_mutex_lock(&mut (*msg).mutex);
        pthread_mutex_unlock(&mut (*msg).mutex);
        mln_iothread_msg_free(msg);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_iothread_recv(
    mut t: *mut mln_iothread_t,
    mut from: mln_iothread_ep_type_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut c: mln_s8_t = 0;
    let mut plock: *mut pthread_mutex_t = 0 as *mut pthread_mutex_t;
    let mut msg: *mut mln_iothread_msg_t = 0 as *mut mln_iothread_msg_t;
    let mut pos: *mut mln_iothread_msg_t = 0 as *mut mln_iothread_msg_t;
    let mut head: *mut *mut mln_iothread_msg_t = 0 as *mut *mut mln_iothread_msg_t;
    let mut tail: *mut *mut mln_iothread_msg_t = 0 as *mut *mut mln_iothread_msg_t;
    if from as libc::c_uint == io_thread as libc::c_int as libc::c_uint {
        fd = (*t).user_fd;
        plock = &mut (*t).user_lock;
        head = &mut (*t).user_head;
        tail = &mut (*t).user_tail;
    } else {
        fd = (*t).io_fd;
        plock = &mut (*t).io_lock;
        head = &mut (*t).io_head;
        tail = &mut (*t).io_tail;
    }
    pthread_mutex_lock(plock);
    pos = *head;
    loop {
        msg = *head;
        if msg.is_null() {
            break;
        }
        mln_iothread_msg_chain_del(head, tail, msg);
        if ((*t).handler).is_some() {
            ((*t).handler).expect("non-null function pointer")(t, from, msg);
        }
        if (*msg).feedback() != 0 {
            if (*msg).hold() == 0 {
                pthread_mutex_unlock(&mut (*msg).mutex);
            }
        } else {
            mln_iothread_msg_free(msg);
        }
        n += 1;
        n;
    }
    if pos != *head {
        recv(
            fd,
            &mut c as *mut mln_s8_t as *mut libc::c_void,
            1 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    }
    pthread_mutex_unlock(plock);
    return n;
}
#[inline]
unsafe extern "C" fn mln_iothread_msg_new(
    mut type_0: mln_u32_t,
    mut data: *mut libc::c_void,
    mut feedback: libc::c_int,
) -> *mut mln_iothread_msg_t {
    let mut msg: *mut mln_iothread_msg_t = malloc(
        ::core::mem::size_of::<mln_iothread_msg_t>() as libc::c_ulong,
    ) as *mut mln_iothread_msg_t;
    if msg.is_null() {
        return 0 as *mut mln_iothread_msg_t;
    }
    (*msg).set_feedback(feedback as mln_u32_t);
    (*msg).set_hold(0 as libc::c_int as mln_u32_t);
    (*msg).type_0 = type_0;
    (*msg).data = data;
    (*msg).next = 0 as *mut mln_iothread_msg_s;
    (*msg).prev = (*msg).next;
    if feedback != 0
        && pthread_mutex_init(&mut (*msg).mutex, 0 as *const pthread_mutexattr_t)
            != 0 as libc::c_int
    {
        free(msg as *mut libc::c_void);
        return 0 as *mut mln_iothread_msg_t;
    }
    return msg;
}
#[inline]
unsafe extern "C" fn mln_iothread_msg_free(mut msg: *mut mln_iothread_msg_t) {
    if msg.is_null() {
        return;
    }
    if (*msg).feedback() != 0 {
        pthread_mutex_destroy(&mut (*msg).mutex);
    }
    free(msg as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_iothread_fd_nonblock_set(mut fd: libc::c_int) {
    let mut flg: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
    fcntl(fd, 4 as libc::c_int, flg | 0o4000 as libc::c_int);
}
