use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn mln_rbtree_new(attr: *mut mln_rbtree_attr) -> *mut mln_rbtree_t;
    fn mln_rbtree_free(t: *mut mln_rbtree_t);
    fn mln_rbtree_delete(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn mln_rbtree_node_new(
        t: *mut mln_rbtree_t,
        data: *mut libc::c_void,
    ) -> *mut mln_rbtree_node_t;
    fn mln_rbtree_node_free(t: *mut mln_rbtree_t, n: *mut mln_rbtree_node_t);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mln_fheap_new(
        min_val: *mut libc::c_void,
        attr: *mut mln_fheap_attr,
    ) -> *mut mln_fheap_t;
    fn mln_fheap_new_fast(
        min_val: *mut libc::c_void,
        cmp: fheap_cmp,
        copy: fheap_copy,
        key_free: fheap_key_free,
    ) -> *mut mln_fheap_t;
    fn mln_fheap_node_new(
        fh: *mut mln_fheap_t,
        key: *mut libc::c_void,
    ) -> *mut mln_fheap_node_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
pub type mln_size_t = size_t;
pub type mln_uauto_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_rbtree_node_s {
    pub data: *mut libc::c_void,
    pub prev: *mut mln_rbtree_node_s,
    pub next: *mut mln_rbtree_node_s,
    pub parent: *mut mln_rbtree_node_s,
    pub left: *mut mln_rbtree_node_s,
    pub right: *mut mln_rbtree_node_s,
    #[bitfield(name = "nofree", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "color", ty = "mln_u32_t", bits = "1..=31")]
    pub nofree_color: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_rbtree_node_t = mln_rbtree_node_s;
pub type rbtree_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type rbtree_free_data = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type rbtree_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type rbtree_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type rbtree_color = libc::c_uint;
pub const M_RB_BLACK: rbtree_color = 1;
pub const M_RB_RED: rbtree_color = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_rbtree_attr {
    pub pool: *mut libc::c_void,
    pub pool_alloc: rbtree_pool_alloc_handler,
    pub pool_free: rbtree_pool_free_handler,
    pub cmp: rbtree_cmp,
    pub data_free: rbtree_free_data,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct rbtree_s {
    pub pool: *mut libc::c_void,
    pub pool_alloc: rbtree_pool_alloc_handler,
    pub pool_free: rbtree_pool_free_handler,
    pub cmp: rbtree_cmp,
    pub data_free: rbtree_free_data,
    pub nil: mln_rbtree_node_t,
    pub root: *mut mln_rbtree_node_t,
    pub min: *mut mln_rbtree_node_t,
    pub head: *mut mln_rbtree_node_t,
    pub tail: *mut mln_rbtree_node_t,
    pub iter: *mut mln_rbtree_node_t,
    pub nr_node: mln_uauto_t,
    #[bitfield(name = "del", ty = "mln_u32_t", bits = "0..=0")]
    pub del: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mln_rbtree_t = rbtree_s;
pub type mln_fheap_mark = libc::c_uint;
pub const FHEAP_TRUE: mln_fheap_mark = 1;
pub const FHEAP_FALSE: mln_fheap_mark = 0;
pub type fheap_cmp = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type fheap_copy = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type fheap_key_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type fheap_pool_alloc_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, mln_size_t) -> *mut libc::c_void,
>;
pub type fheap_pool_free_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fheap_attr {
    pub pool: *mut libc::c_void,
    pub pool_alloc: fheap_pool_alloc_handler,
    pub pool_free: fheap_pool_free_handler,
    pub cmp: fheap_cmp,
    pub copy: fheap_copy,
    pub key_free: fheap_key_free,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_fheap_node_s {
    pub key: *mut libc::c_void,
    pub parent: *mut mln_fheap_node_s,
    pub child: *mut mln_fheap_node_s,
    pub left: *mut mln_fheap_node_s,
    pub right: *mut mln_fheap_node_s,
    pub degree: mln_size_t,
    #[bitfield(name = "nofree", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "mark", ty = "mln_u32_t", bits = "1..=31")]
    pub nofree_mark: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type mln_fheap_node_t = mln_fheap_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_fheap_t {
    pub root_list: *mut mln_fheap_node_t,
    pub min: *mut mln_fheap_node_t,
    pub cmp: fheap_cmp,
    pub copy: fheap_copy,
    pub key_free: fheap_key_free,
    pub num: mln_size_t,
    pub min_val: *mut libc::c_void,
    pub pool: *mut libc::c_void,
    pub pool_alloc: fheap_pool_alloc_handler,
    pub pool_free: fheap_pool_free_handler,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_event_s {
    pub fd_lock: pthread_mutex_t,
    pub timer_lock: pthread_mutex_t,
    pub cb_lock: pthread_mutex_t,
    pub callback: dispatch_callback,
    pub callback_data: *mut libc::c_void,
    #[bitfield(name = "is_break", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "1..=31")]
    pub is_break_padding: [u8; 4],
    pub epollfd: libc::c_int,
    pub unusedfd: libc::c_int,
    pub ev_fd_tree: *mut mln_rbtree_t,
    pub ev_fd_wait_head: *mut mln_event_desc_t,
    pub ev_fd_wait_tail: *mut mln_event_desc_t,
    pub ev_fd_active_head: *mut mln_event_desc_t,
    pub ev_fd_active_tail: *mut mln_event_desc_t,
    pub ev_fd_timeout_heap: *mut mln_fheap_t,
    pub ev_timer_heap: *mut mln_fheap_t,
}
pub type mln_event_desc_t = mln_event_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_event_desc_s {
    pub prev: *mut mln_event_desc_s,
    pub next: *mut mln_event_desc_s,
    pub act_prev: *mut mln_event_desc_s,
    pub act_next: *mut mln_event_desc_s,
    pub type_0: mln_event_type,
    pub flag: mln_u32_t,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tm: mln_event_tm_t,
    pub fd: mln_event_fd_t,
}
pub type mln_event_fd_t = mln_event_fd_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_event_fd_s {
    pub fd: libc::c_int,
    pub active_flag: mln_u32_t,
    #[bitfield(name = "in_process", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "is_clear", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "in_active", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "rd_oneshot", ty = "mln_u32_t", bits = "3..=3")]
    #[bitfield(name = "wr_oneshot", ty = "mln_u32_t", bits = "4..=4")]
    #[bitfield(name = "err_oneshot", ty = "mln_u32_t", bits = "5..=5")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "6..=31")]
    pub in_process_is_clear_in_active_rd_oneshot_wr_oneshot_err_oneshot_padding: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub rcv_data: *mut libc::c_void,
    pub rcv_handler: ev_fd_handler,
    pub snd_data: *mut libc::c_void,
    pub snd_handler: ev_fd_handler,
    pub err_data: *mut libc::c_void,
    pub err_handler: ev_fd_handler,
    pub timeout_data: *mut libc::c_void,
    pub timeout_handler: ev_fd_handler,
    pub timeout_node: *mut mln_fheap_node_t,
    pub end_us: mln_u64_t,
}
pub type ev_fd_handler = Option::<
    unsafe extern "C" fn(*mut mln_event_t, libc::c_int, *mut libc::c_void) -> (),
>;
pub type mln_event_t = mln_event_s;
pub type mln_event_tm_t = mln_event_tm_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_event_tm_s {
    pub data: *mut libc::c_void,
    pub handler: ev_tm_handler,
    pub end_tm: mln_uauto_t,
}
pub type ev_tm_handler = Option::<
    unsafe extern "C" fn(*mut mln_event_t, *mut libc::c_void) -> (),
>;
pub type mln_event_type = libc::c_uint;
pub const M_EV_TM: mln_event_type = 1;
pub const M_EV_FD: mln_event_type = 0;
pub type dispatch_callback = Option::<
    unsafe extern "C" fn(*mut mln_event_t, *mut libc::c_void) -> (),
>;
pub type mln_event_timer_t = mln_fheap_node_t;
#[inline]
unsafe extern "C" fn mln_rbtree_chain_add(
    mut head: *mut *mut mln_rbtree_node_t,
    mut tail: *mut *mut mln_rbtree_node_t,
    mut node: *mut mln_rbtree_node_t,
) {
    (*node).next = 0 as *mut mln_rbtree_node_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn mln_rbtree_left_rotate(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    if (*n).right == &mut (*t).nil as *mut mln_rbtree_node_t {
        return;
    }
    let mut tmp: *mut mln_rbtree_node_t = (*n).right;
    (*n).right = (*tmp).left;
    if (*tmp).left != &mut (*t).nil as *mut mln_rbtree_node_t {
        (*(*tmp).left).parent = n;
    }
    (*tmp).parent = (*n).parent;
    if (*n).parent == &mut (*t).nil as *mut mln_rbtree_node_t {
        (*t).root = tmp;
    } else if n == (*(*n).parent).left {
        (*(*n).parent).left = tmp;
    } else {
        (*(*n).parent).right = tmp;
    }
    (*tmp).left = n;
    (*n).parent = tmp;
}
#[inline]
unsafe extern "C" fn mln_rbtree_right_rotate(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    if (*n).left == &mut (*t).nil as *mut mln_rbtree_node_t {
        return;
    }
    let mut tmp: *mut mln_rbtree_node_t = (*n).left;
    (*n).left = (*tmp).right;
    if (*tmp).right != &mut (*t).nil as *mut mln_rbtree_node_t {
        (*(*tmp).right).parent = n;
    }
    (*tmp).parent = (*n).parent;
    if (*n).parent == &mut (*t).nil as *mut mln_rbtree_node_t {
        (*t).root = tmp;
    } else if n == (*(*n).parent).right {
        (*(*n).parent).right = tmp;
    } else {
        (*(*n).parent).left = tmp;
    }
    (*tmp).right = n;
    (*n).parent = tmp;
}
#[inline]
unsafe extern "C" fn rbtree_insert_fixup(
    mut t: *mut mln_rbtree_t,
    mut n: *mut mln_rbtree_node_t,
) {
    let mut tmp: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    while (*(*n).parent).color() as libc::c_int == M_RB_RED as libc::c_int {
        if (*n).parent == (*(*(*n).parent).parent).left {
            tmp = (*(*(*n).parent).parent).right;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*(*n).parent).parent;
            } else {
                if n == (*(*n).parent).right {
                    n = (*n).parent;
                    mln_rbtree_left_rotate(t, n);
                }
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_right_rotate(t, (*(*n).parent).parent);
            }
        } else {
            tmp = (*(*(*n).parent).parent).left;
            if (*tmp).color() as libc::c_int == M_RB_RED as libc::c_int {
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*tmp).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                n = (*(*n).parent).parent;
            } else {
                if n == (*(*n).parent).left {
                    n = (*n).parent;
                    mln_rbtree_right_rotate(t, n);
                }
                (*(*n).parent).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
                (*(*(*n).parent).parent).set_color(M_RB_RED as libc::c_int as mln_u32_t);
                mln_rbtree_left_rotate(t, (*(*n).parent).parent);
            }
        }
    }
    (*(*t).root).set_color(M_RB_BLACK as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_add_child(
    mut root: *mut *mut mln_fheap_node_t,
    mut node: *mut mln_fheap_node_t,
) {
    if (*root).is_null() {
        *root = node;
        return;
    }
    (*node).right = *root;
    (*node).left = (**root).left;
    (**root).left = node;
    (*(*node).left).right = node;
}
#[inline]
unsafe extern "C" fn mln_fheap_del_child(
    mut root: *mut *mut mln_fheap_node_t,
    mut node: *mut mln_fheap_node_t,
) {
    if *root == node {
        if (*node).right == node {
            *root = 0 as *mut mln_fheap_node_t;
        } else {
            *root = (*node).right;
            (*(*node).right).left = (*node).left;
            (*(*node).left).right = (*node).right;
        }
    } else {
        (*(*node).right).left = (*node).left;
        (*(*node).left).right = (*node).right;
    }
    (*node).left = node;
    (*node).right = (*node).left;
}
#[inline]
unsafe extern "C" fn mln_fheap_link(
    mut fh: *mut mln_fheap_t,
    mut y: *mut mln_fheap_node_t,
    mut x: *mut mln_fheap_node_t,
) {
    mln_fheap_del_child(&mut (*fh).root_list, y);
    mln_fheap_add_child(&mut (*x).child, y);
    (*y).parent = x;
    (*x).degree = ((*x).degree).wrapping_add(1);
    (*x).degree;
    (*y).set_mark(FHEAP_FALSE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_consolidate(
    mut fh: *mut mln_fheap_t,
    mut cmp: fheap_cmp,
) {
    let mut array: [*mut mln_fheap_node_t; 65] = [0 as *mut mln_fheap_node_t; 65];
    memset(
        array.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<*mut mln_fheap_node_t>() as libc::c_ulong)
            .wrapping_mul(65 as libc::c_int as libc::c_ulong),
    );
    let mut x: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut y: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut w: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut tmp: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut d: mln_size_t = 0;
    let mut mark: mln_size_t = 0 as libc::c_int as mln_size_t;
    w = (*fh).root_list;
    while !w.is_null() && !(w == (*fh).root_list && mark != 0) {
        if w == (*fh).root_list {
            mark = mark.wrapping_add(1);
            mark;
        }
        x = w;
        w = (*w).right;
        d = (*x).degree;
        while !(array[d as usize]).is_null() {
            y = array[d as usize];
            if cmp.expect("non-null function pointer")((*y).key, (*x).key) == 0 {
                tmp = x;
                x = y;
                y = tmp;
            }
            if y == w {
                w = (*w).right;
            }
            mln_fheap_link(fh, y, x);
            array[d as usize] = 0 as *mut mln_fheap_node_t;
            d = d.wrapping_add(1);
            d;
        }
        array[d as usize] = x;
    }
    (*fh).min = 0 as *mut mln_fheap_node_t;
    let mut i: mln_size_t = 0;
    let mut root_list: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    i = 0 as libc::c_int as mln_size_t;
    while i < 65 as libc::c_int as libc::c_ulong {
        if !(array[i as usize]).is_null() {
            mln_fheap_del_child(&mut (*fh).root_list, array[i as usize]);
            mln_fheap_add_child(&mut root_list, array[i as usize]);
            (*array[i as usize]).parent = 0 as *mut mln_fheap_node_s;
            if ((*fh).min).is_null() {
                (*fh).min = array[i as usize];
            } else if cmp
                .expect(
                    "non-null function pointer",
                )((*array[i as usize]).key, (*(*fh).min).key) == 0
            {
                (*fh).min = array[i as usize];
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    (*fh).root_list = root_list;
}
#[inline]
unsafe extern "C" fn mln_fheap_cut(
    mut fh: *mut mln_fheap_t,
    mut x: *mut mln_fheap_node_t,
    mut y: *mut mln_fheap_node_t,
) {
    mln_fheap_del_child(&mut (*y).child, x);
    (*y).degree = ((*y).degree).wrapping_sub(1);
    (*y).degree;
    mln_fheap_add_child(&mut (*fh).root_list, x);
    (*x).parent = 0 as *mut mln_fheap_node_s;
    (*x).set_mark(FHEAP_FALSE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_cascading_cut(
    mut fh: *mut mln_fheap_t,
    mut y: *mut mln_fheap_node_t,
) {
    let mut z: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    loop {
        z = (*y).parent;
        if z.is_null() {
            return;
        }
        if (*y).mark() as libc::c_int == FHEAP_FALSE as libc::c_int {
            break;
        }
        mln_fheap_cut(fh, y, z);
        y = z;
    }
    (*y).set_mark(FHEAP_TRUE as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_fheap_remove_child(
    mut root: *mut *mut mln_fheap_node_t,
) -> *mut mln_fheap_node_t {
    if (*root).is_null() {
        return 0 as *mut mln_fheap_node_t;
    }
    let mut ret: *mut mln_fheap_node_t = *root;
    if (*ret).right == ret {
        *root = 0 as *mut mln_fheap_node_t;
    } else {
        *root = (*ret).right;
        (*(*ret).right).left = (*ret).left;
        (*(*ret).left).right = (*ret).right;
    }
    (*ret).right = ret;
    (*ret).left = (*ret).right;
    return ret;
}
#[no_mangle]
pub static mut fheap_min: mln_event_desc_t = {
    let mut init = mln_event_desc_s {
        prev: 0 as *const mln_event_desc_s as *mut mln_event_desc_s,
        next: 0 as *const mln_event_desc_s as *mut mln_event_desc_s,
        act_prev: 0 as *const mln_event_desc_s as *mut mln_event_desc_s,
        act_next: 0 as *const mln_event_desc_s as *mut mln_event_desc_s,
        type_0: M_EV_TM,
        flag: 0 as libc::c_int as mln_u32_t,
        data: C2RustUnnamed {
            tm: {
                let mut init = mln_event_tm_s {
                    data: 0 as *const libc::c_void as *mut libc::c_void,
                    handler: None,
                    end_tm: 0,
                };
                init
            },
        },
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn mln_event_new() -> *mut mln_event_t {
    let mut rc: libc::c_int = 0;
    let mut ev: *mut mln_event_t = 0 as *mut mln_event_t;
    ev = malloc(::core::mem::size_of::<mln_event_t>() as libc::c_ulong)
        as *mut mln_event_t;
    if ev.is_null() {
        return 0 as *mut mln_event_t;
    }
    (*ev).callback = None;
    (*ev).callback_data = 0 as *mut libc::c_void;
    (*ev).ev_fd_tree = mln_rbtree_new(0 as *mut mln_rbtree_attr);
    if !((*ev).ev_fd_tree).is_null() {
        (*ev).ev_fd_wait_head = 0 as *mut mln_event_desc_t;
        (*ev).ev_fd_wait_tail = 0 as *mut mln_event_desc_t;
        (*ev).ev_fd_active_head = 0 as *mut mln_event_desc_t;
        (*ev).ev_fd_active_tail = 0 as *mut mln_event_desc_t;
        (*ev)
            .ev_fd_timeout_heap = mln_fheap_new(
            &mut fheap_min as *mut mln_event_desc_t as *mut libc::c_void,
            0 as *mut mln_fheap_attr,
        );
        if !((*ev).ev_fd_timeout_heap).is_null() {
            (*ev)
                .ev_timer_heap = mln_fheap_new_fast(
                &mut fheap_min as *mut mln_event_desc_t as *mut libc::c_void,
                Some(
                    mln_event_fheap_timer_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
                Some(
                    mln_event_fheap_timer_copy
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                Some(
                    mln_event_desc_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
            if !((*ev).ev_timer_heap).is_null() {
                (*ev).set_is_break(0 as libc::c_int as mln_u32_t);
                (*ev).epollfd = epoll_create(1024 as libc::c_int);
                if !((*ev).epollfd < 0 as libc::c_int) {
                    (*ev).unusedfd = epoll_create(1024 as libc::c_int);
                    if (*ev).unusedfd < 0 as libc::c_int {
                        close((*ev).epollfd);
                    } else {
                        rc = pthread_mutex_init(
                            &mut (*ev).fd_lock,
                            0 as *const pthread_mutexattr_t,
                        );
                        if pthread_mutex_init(
                            &mut (*ev).timer_lock,
                            0 as *const pthread_mutexattr_t,
                        ) != 0 as libc::c_int
                        {
                            rc = -(1 as libc::c_int);
                        }
                        if pthread_mutex_init(
                            &mut (*ev).cb_lock,
                            0 as *const pthread_mutexattr_t,
                        ) != 0 as libc::c_int
                        {
                            rc = -(1 as libc::c_int);
                        }
                        if rc != 0 {
                            pthread_mutex_destroy(&mut (*ev).fd_lock);
                            pthread_mutex_destroy(&mut (*ev).timer_lock);
                            pthread_mutex_destroy(&mut (*ev).cb_lock);
                            close((*ev).epollfd);
                        } else {
                            return ev
                        }
                    }
                }
                if !((*ev).ev_timer_heap).is_null() {
                    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        fn_0 = ({
                            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> libc::c_int,
                                >,
                                fheap_cmp,
                            >(
                                Some(
                                    mln_event_fheap_timer_cmp
                                        as unsafe extern "C" fn(
                                            *const libc::c_void,
                                            *const libc::c_void,
                                        ) -> libc::c_int,
                                ),
                            );
                            if cmp.is_none() {
                                cmp = (*(*ev).ev_timer_heap).cmp;
                            }
                            let mut z: *mut mln_fheap_node_t = (*(*ev).ev_timer_heap)
                                .min;
                            if !z.is_null() {
                                let mut child: *mut mln_fheap_node_t = 0
                                    as *mut mln_fheap_node_t;
                                loop {
                                    child = mln_fheap_remove_child(&mut (*z).child);
                                    if child.is_null() {
                                        break;
                                    }
                                    mln_fheap_add_child(
                                        &mut (*(*ev).ev_timer_heap).root_list,
                                        child,
                                    );
                                    (*child).parent = 0 as *mut mln_fheap_node_s;
                                }
                                let mut right: *mut mln_fheap_node_t = (*z).right;
                                mln_fheap_del_child(
                                    &mut (*(*ev).ev_timer_heap).root_list,
                                    z,
                                );
                                if z == right {
                                    (*(*ev).ev_timer_heap).min = 0 as *mut mln_fheap_node_t;
                                } else {
                                    (*(*ev).ev_timer_heap).min = right;
                                    mln_fheap_consolidate((*ev).ev_timer_heap, cmp);
                                }
                                (*(*ev).ev_timer_heap)
                                    .num = ((*(*ev).ev_timer_heap).num).wrapping_sub(1);
                                (*(*ev).ev_timer_heap).num;
                            }
                            z
                        });
                        if fn_0.is_null() {
                            break;
                        }
                        let mut f: fheap_key_free = ::core::mem::transmute::<
                            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                            fheap_key_free,
                        >(
                            Some(
                                mln_event_desc_free
                                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
                            ),
                        );
                        if f.is_none() {
                            f = (*(*ev).ev_timer_heap).key_free;
                        }
                        if !fn_0.is_null() {
                            if f.is_some() && !((*fn_0).key).is_null() {
                                f.expect("non-null function pointer")((*fn_0).key);
                            }
                            if (*fn_0).nofree() == 0 {
                                if !((*(*ev).ev_timer_heap).pool).is_null() {
                                    ((*(*ev).ev_timer_heap).pool_free)
                                        .expect(
                                            "non-null function pointer",
                                        )(fn_0 as *mut libc::c_void);
                                } else {
                                    free(fn_0 as *mut libc::c_void);
                                }
                            }
                        }
                    }
                    if !((*(*ev).ev_timer_heap).pool).is_null() {
                        ((*(*ev).ev_timer_heap).pool_free)
                            .expect(
                                "non-null function pointer",
                            )((*ev).ev_timer_heap as *mut libc::c_void);
                    } else {
                        free((*ev).ev_timer_heap as *mut libc::c_void);
                    }
                }
            }
            if !((*ev).ev_fd_timeout_heap).is_null() {
                let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                loop {
                    fn_0 = ({
                        let mut cmp: fheap_cmp = ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                            >,
                            fheap_cmp,
                        >(
                            Some(
                                mln_event_fd_timeout_cmp
                                    as unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> libc::c_int,
                            ),
                        );
                        if cmp.is_none() {
                            cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                        }
                        let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap)
                            .min;
                        if !z.is_null() {
                            let mut child: *mut mln_fheap_node_t = 0
                                as *mut mln_fheap_node_t;
                            loop {
                                child = mln_fheap_remove_child(&mut (*z).child);
                                if child.is_null() {
                                    break;
                                }
                                mln_fheap_add_child(
                                    &mut (*(*ev).ev_fd_timeout_heap).root_list,
                                    child,
                                );
                                (*child).parent = 0 as *mut mln_fheap_node_s;
                            }
                            let mut right: *mut mln_fheap_node_t = (*z).right;
                            mln_fheap_del_child(
                                &mut (*(*ev).ev_fd_timeout_heap).root_list,
                                z,
                            );
                            if z == right {
                                (*(*ev).ev_fd_timeout_heap)
                                    .min = 0 as *mut mln_fheap_node_t;
                            } else {
                                (*(*ev).ev_fd_timeout_heap).min = right;
                                mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                            }
                            (*(*ev).ev_fd_timeout_heap)
                                .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                            (*(*ev).ev_fd_timeout_heap).num;
                        }
                        z
                    });
                    if fn_0.is_null() {
                        break;
                    }
                    let mut f: fheap_key_free = ::core::mem::transmute::<
                        *mut libc::c_void,
                        fheap_key_free,
                    >(0 as *mut libc::c_void);
                    if f.is_none() {
                        f = (*(*ev).ev_fd_timeout_heap).key_free;
                    }
                    if !fn_0.is_null() {
                        if f.is_some() && !((*fn_0).key).is_null() {
                            f.expect("non-null function pointer")((*fn_0).key);
                        }
                        if (*fn_0).nofree() == 0 {
                            if !((*(*ev).ev_fd_timeout_heap).pool).is_null() {
                                ((*(*ev).ev_fd_timeout_heap).pool_free)
                                    .expect(
                                        "non-null function pointer",
                                    )(fn_0 as *mut libc::c_void);
                            } else {
                                free(fn_0 as *mut libc::c_void);
                            }
                        }
                    }
                }
                if !((*(*ev).ev_fd_timeout_heap).pool).is_null() {
                    ((*(*ev).ev_fd_timeout_heap).pool_free)
                        .expect(
                            "non-null function pointer",
                        )((*ev).ev_fd_timeout_heap as *mut libc::c_void);
                } else {
                    free((*ev).ev_fd_timeout_heap as *mut libc::c_void);
                }
            }
        }
        mln_rbtree_free((*ev).ev_fd_tree);
    }
    free(ev as *mut libc::c_void);
    return 0 as *mut mln_event_t;
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_free(mut ev: *mut mln_event_t) {
    if ev.is_null() {
        return;
    }
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    if !((*ev).ev_fd_timeout_heap).is_null() {
        let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
        loop {
            fn_0 = ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*ev).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(&mut (*(*ev).ev_fd_timeout_heap).root_list, z);
                    if z == right {
                        (*(*ev).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*ev).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                    }
                    (*(*ev).ev_fd_timeout_heap)
                        .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*ev).ev_fd_timeout_heap).num;
                }
                z
            });
            if fn_0.is_null() {
                break;
            }
            let mut f: fheap_key_free = ::core::mem::transmute::<
                *mut libc::c_void,
                fheap_key_free,
            >(0 as *mut libc::c_void);
            if f.is_none() {
                f = (*(*ev).ev_fd_timeout_heap).key_free;
            }
            if !fn_0.is_null() {
                if f.is_some() && !((*fn_0).key).is_null() {
                    f.expect("non-null function pointer")((*fn_0).key);
                }
                if (*fn_0).nofree() == 0 {
                    if !((*(*ev).ev_fd_timeout_heap).pool).is_null() {
                        ((*(*ev).ev_fd_timeout_heap).pool_free)
                            .expect(
                                "non-null function pointer",
                            )(fn_0 as *mut libc::c_void);
                    } else {
                        free(fn_0 as *mut libc::c_void);
                    }
                }
            }
        }
        if !((*(*ev).ev_fd_timeout_heap).pool).is_null() {
            ((*(*ev).ev_fd_timeout_heap).pool_free)
                .expect(
                    "non-null function pointer",
                )((*ev).ev_fd_timeout_heap as *mut libc::c_void);
        } else {
            free((*ev).ev_fd_timeout_heap as *mut libc::c_void);
        }
    }
    mln_rbtree_free((*ev).ev_fd_tree);
    loop {
        ed = (*ev).ev_fd_wait_head;
        if ed.is_null() {
            break;
        }
        ev_fd_wait_chain_del(&mut (*ev).ev_fd_wait_head, &mut (*ev).ev_fd_wait_tail, ed);
        mln_event_desc_free(ed as *mut libc::c_void);
    }
    if !((*ev).ev_timer_heap).is_null() {
        let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
        loop {
            fn_0 = ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fheap_timer_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*ev).ev_timer_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*ev).ev_timer_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*ev).ev_timer_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(&mut (*(*ev).ev_timer_heap).root_list, z);
                    if z == right {
                        (*(*ev).ev_timer_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*ev).ev_timer_heap).min = right;
                        mln_fheap_consolidate((*ev).ev_timer_heap, cmp);
                    }
                    (*(*ev).ev_timer_heap)
                        .num = ((*(*ev).ev_timer_heap).num).wrapping_sub(1);
                    (*(*ev).ev_timer_heap).num;
                }
                z
            });
            if fn_0.is_null() {
                break;
            }
            let mut f: fheap_key_free = ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                fheap_key_free,
            >(
                Some(
                    mln_event_desc_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
            if f.is_none() {
                f = (*(*ev).ev_timer_heap).key_free;
            }
            if !fn_0.is_null() {
                if f.is_some() && !((*fn_0).key).is_null() {
                    f.expect("non-null function pointer")((*fn_0).key);
                }
                if (*fn_0).nofree() == 0 {
                    if !((*(*ev).ev_timer_heap).pool).is_null() {
                        ((*(*ev).ev_timer_heap).pool_free)
                            .expect(
                                "non-null function pointer",
                            )(fn_0 as *mut libc::c_void);
                    } else {
                        free(fn_0 as *mut libc::c_void);
                    }
                }
            }
        }
        if !((*(*ev).ev_timer_heap).pool).is_null() {
            ((*(*ev).ev_timer_heap).pool_free)
                .expect(
                    "non-null function pointer",
                )((*ev).ev_timer_heap as *mut libc::c_void);
        } else {
            free((*ev).ev_timer_heap as *mut libc::c_void);
        }
    }
    close((*ev).epollfd);
    close((*ev).unusedfd);
    pthread_mutex_destroy(&mut (*ev).fd_lock);
    pthread_mutex_destroy(&mut (*ev).timer_lock);
    pthread_mutex_destroy(&mut (*ev).cb_lock);
    free(ev as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_event_desc_free(mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    free(data);
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_timer_set(
    mut event: *mut mln_event_t,
    mut msec: mln_u32_t,
    mut data: *mut libc::c_void,
    mut tm_handler: ev_tm_handler,
) -> *mut mln_event_timer_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    let mut end: mln_uauto_t = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long
        + tv.tv_usec
        + msec.wrapping_mul(1000 as libc::c_int as libc::c_uint) as libc::c_long)
        as mln_uauto_t;
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    ed = malloc(::core::mem::size_of::<mln_event_desc_t>() as libc::c_ulong)
        as *mut mln_event_desc_t;
    if ed.is_null() {
        return 0 as *mut mln_event_timer_t;
    }
    (*ed).type_0 = M_EV_TM;
    (*ed).flag = 0 as libc::c_int as mln_u32_t;
    (*ed).data.tm.data = data;
    (*ed).data.tm.handler = tm_handler;
    (*ed).data.tm.end_tm = end;
    (*ed).prev = 0 as *mut mln_event_desc_s;
    (*ed).next = 0 as *mut mln_event_desc_s;
    (*ed).act_prev = 0 as *mut mln_event_desc_s;
    (*ed).act_next = 0 as *mut mln_event_desc_s;
    let mut fn_0: *mut mln_fheap_node_t = mln_fheap_node_new(
        (*event).ev_timer_heap,
        ed as *mut libc::c_void,
    );
    if fn_0.is_null() {
        free(ed as *mut libc::c_void);
        return 0 as *mut mln_event_timer_t;
    }
    pthread_mutex_lock(&mut (*event).timer_lock);
    ({
        let mut cmp: fheap_cmp = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
            fheap_cmp,
        >(
            Some(
                mln_event_fheap_timer_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        if cmp.is_none() {
            cmp = (*(*event).ev_timer_heap).cmp;
        }
        mln_fheap_add_child(&mut (*(*event).ev_timer_heap).root_list, fn_0);
        (*fn_0).parent = 0 as *mut mln_fheap_node_s;
        if ((*(*event).ev_timer_heap).min).is_null() {
            (*(*event).ev_timer_heap).min = fn_0;
        } else if cmp
            .expect(
                "non-null function pointer",
            )((*fn_0).key, (*(*(*event).ev_timer_heap).min).key) == 0
        {
            (*(*event).ev_timer_heap).min = fn_0;
        }
        (*(*event).ev_timer_heap).num = ((*(*event).ev_timer_heap).num).wrapping_add(1);
        (*(*event).ev_timer_heap).num;
        (*(*event).ev_timer_heap).num
    });
    pthread_mutex_unlock(&mut (*event).timer_lock);
    return fn_0;
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_timer_cancel(
    mut event: *mut mln_event_t,
    mut timer: *mut mln_event_timer_t,
) {
    pthread_mutex_lock(&mut (*event).timer_lock);
    ({
        ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fheap_timer_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*event).ev_timer_heap).cmp;
            }
            let mut cp: fheap_copy = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                >,
                fheap_copy,
            >(
                Some(
                    mln_event_fheap_timer_copy
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
            );
            if cp.is_none() {
                cp = (*(*event).ev_timer_heap).copy;
            }
            let mut r: libc::c_int = 0 as libc::c_int;
            if cmp
                .expect(
                    "non-null function pointer",
                )((*timer).key, (*(*event).ev_timer_heap).min_val) == 0
            {
                r = -(1 as libc::c_int);
            } else {
                cp
                    .expect(
                        "non-null function pointer",
                    )((*timer).key, (*(*event).ev_timer_heap).min_val);
                let mut y: *mut mln_fheap_node_t = (*timer).parent;
                if !y.is_null()
                    && cmp.expect("non-null function pointer")((*timer).key, (*y).key)
                        == 0
                {
                    mln_fheap_cut((*event).ev_timer_heap, timer, y);
                    mln_fheap_cascading_cut((*event).ev_timer_heap, y);
                }
                if timer != (*(*event).ev_timer_heap).min
                    && cmp
                        .expect(
                            "non-null function pointer",
                        )((*timer).key, (*(*(*event).ev_timer_heap).min).key) == 0
                {
                    (*(*event).ev_timer_heap).min = timer;
                }
            }
            r
        });
        ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fheap_timer_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*event).ev_timer_heap).cmp;
            }
            let mut z: *mut mln_fheap_node_t = (*(*event).ev_timer_heap).min;
            if !z.is_null() {
                let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                loop {
                    child = mln_fheap_remove_child(&mut (*z).child);
                    if child.is_null() {
                        break;
                    }
                    mln_fheap_add_child(&mut (*(*event).ev_timer_heap).root_list, child);
                    (*child).parent = 0 as *mut mln_fheap_node_s;
                }
                let mut right: *mut mln_fheap_node_t = (*z).right;
                mln_fheap_del_child(&mut (*(*event).ev_timer_heap).root_list, z);
                if z == right {
                    (*(*event).ev_timer_heap).min = 0 as *mut mln_fheap_node_t;
                } else {
                    (*(*event).ev_timer_heap).min = right;
                    mln_fheap_consolidate((*event).ev_timer_heap, cmp);
                }
                (*(*event).ev_timer_heap)
                    .num = ((*(*event).ev_timer_heap).num).wrapping_sub(1);
                (*(*event).ev_timer_heap).num;
            }
            z
        });
        ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fheap_timer_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*event).ev_timer_heap).cmp;
            }
            let mut z: *mut mln_fheap_node_t = (*(*event).ev_timer_heap).min;
            if !z.is_null() {
                let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                loop {
                    child = mln_fheap_remove_child(&mut (*z).child);
                    if child.is_null() {
                        break;
                    }
                    mln_fheap_add_child(&mut (*(*event).ev_timer_heap).root_list, child);
                    (*child).parent = 0 as *mut mln_fheap_node_s;
                }
                let mut right: *mut mln_fheap_node_t = (*z).right;
                mln_fheap_del_child(&mut (*(*event).ev_timer_heap).root_list, z);
                if z == right {
                    (*(*event).ev_timer_heap).min = 0 as *mut mln_fheap_node_t;
                } else {
                    (*(*event).ev_timer_heap).min = right;
                    mln_fheap_consolidate((*event).ev_timer_heap, cmp);
                }
                (*(*event).ev_timer_heap)
                    .num = ((*(*event).ev_timer_heap).num).wrapping_sub(1);
                (*(*event).ev_timer_heap).num;
            }
            z
        })
    });
    let mut f: fheap_key_free = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        fheap_key_free,
    >(Some(mln_event_desc_free as unsafe extern "C" fn(*mut libc::c_void) -> ()));
    if f.is_none() {
        f = (*(*event).ev_timer_heap).key_free;
    }
    if !timer.is_null() {
        if f.is_some() && !((*timer).key).is_null() {
            f.expect("non-null function pointer")((*timer).key);
        }
        if (*timer).nofree() == 0 {
            if !((*(*event).ev_timer_heap).pool).is_null() {
                ((*(*event).ev_timer_heap).pool_free)
                    .expect("non-null function pointer")(timer as *mut libc::c_void);
            } else {
                free(timer as *mut libc::c_void);
            }
        }
    }
    pthread_mutex_unlock(&mut (*event).timer_lock);
}
#[inline]
unsafe extern "C" fn mln_event_timer_process(mut event: *mut mln_event_t) {
    let mut now: mln_uauto_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    now = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec)
        as mln_uauto_t;
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    loop {
        if pthread_mutex_trylock(&mut (*event).timer_lock) != 0 {
            return;
        }
        fn_0 = (*(*event).ev_timer_heap).min;
        if fn_0.is_null() {
            pthread_mutex_unlock(&mut (*event).timer_lock);
            return;
        }
        ed = (*fn_0).key as *mut mln_event_desc_t;
        if (*ed).data.tm.end_tm > now {
            pthread_mutex_unlock(&mut (*event).timer_lock);
            return;
        }
        fn_0 = ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fheap_timer_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*event).ev_timer_heap).cmp;
            }
            let mut z: *mut mln_fheap_node_t = (*(*event).ev_timer_heap).min;
            if !z.is_null() {
                let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                loop {
                    child = mln_fheap_remove_child(&mut (*z).child);
                    if child.is_null() {
                        break;
                    }
                    mln_fheap_add_child(&mut (*(*event).ev_timer_heap).root_list, child);
                    (*child).parent = 0 as *mut mln_fheap_node_s;
                }
                let mut right: *mut mln_fheap_node_t = (*z).right;
                mln_fheap_del_child(&mut (*(*event).ev_timer_heap).root_list, z);
                if z == right {
                    (*(*event).ev_timer_heap).min = 0 as *mut mln_fheap_node_t;
                } else {
                    (*(*event).ev_timer_heap).min = right;
                    mln_fheap_consolidate((*event).ev_timer_heap, cmp);
                }
                (*(*event).ev_timer_heap)
                    .num = ((*(*event).ev_timer_heap).num).wrapping_sub(1);
                (*(*event).ev_timer_heap).num;
            }
            z
        });
        pthread_mutex_unlock(&mut (*event).timer_lock);
        if ((*ed).data.tm.handler).is_some() {
            ((*ed).data.tm.handler)
                .expect("non-null function pointer")(event, (*ed).data.tm.data);
        }
        let mut f: fheap_key_free = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            fheap_key_free,
        >(Some(mln_event_desc_free as unsafe extern "C" fn(*mut libc::c_void) -> ()));
        if f.is_none() {
            f = (*(*event).ev_timer_heap).key_free;
        }
        if !fn_0.is_null() {
            if f.is_some() && !((*fn_0).key).is_null() {
                f.expect("non-null function pointer")((*fn_0).key);
            }
            if (*fn_0).nofree() == 0 {
                if !((*(*event).ev_timer_heap).pool).is_null() {
                    ((*(*event).ev_timer_heap).pool_free)
                        .expect("non-null function pointer")(fn_0 as *mut libc::c_void);
                } else {
                    free(fn_0 as *mut libc::c_void);
                }
            }
        }
        if !((*event).is_break() == 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_fd_timeout_handler_set(
    mut event: *mut mln_event_t,
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut timeout_handler: ev_fd_handler,
) {
    pthread_mutex_lock(&mut (*event).fd_lock);
    let mut tmp: mln_event_desc_t = mln_event_desc_t {
        prev: 0 as *mut mln_event_desc_s,
        next: 0 as *mut mln_event_desc_s,
        act_prev: 0 as *mut mln_event_desc_s,
        act_next: 0 as *mut mln_event_desc_s,
        type_0: M_EV_FD,
        flag: 0,
        data: C2RustUnnamed {
            tm: mln_event_tm_t {
                data: 0 as *mut libc::c_void,
                handler: None,
                end_tm: 0,
            },
        },
    };
    memset(
        &mut tmp as *mut mln_event_desc_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_event_desc_t>() as libc::c_ulong,
    );
    tmp.type_0 = M_EV_FD;
    tmp.data.fd.fd = fd;
    let mut rn: *mut mln_rbtree_node_t = ({
        let mut tree: *mut mln_rbtree_t = (*event).ev_fd_tree;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_event_rbtree_fd_cmp(
                    &mut tmp as *mut mln_event_desc_t as *const libc::c_void,
                    (*ret_node).data,
                );
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    let mut ed: *mut mln_event_desc_t = (*rn).data as *mut mln_event_desc_t;
    (*ed).data.fd.timeout_data = data;
    (*ed).data.fd.timeout_handler = timeout_handler;
    pthread_mutex_unlock(&mut (*event).fd_lock);
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_fd_set(
    mut event: *mut mln_event_t,
    mut fd: libc::c_int,
    mut flag: mln_u32_t,
    mut timeout_ms: libc::c_int,
    mut data: *mut libc::c_void,
    mut fd_handler: ev_fd_handler,
) -> libc::c_int {
    pthread_mutex_lock(&mut (*event).fd_lock);
    if flag == 0x80 as libc::c_int as mln_u32_t {
        mln_event_fd_clr_set(event, fd);
        pthread_mutex_unlock(&mut (*event).fd_lock);
        return 0 as libc::c_int;
    }
    let mut tmp: mln_event_desc_t = mln_event_desc_t {
        prev: 0 as *mut mln_event_desc_s,
        next: 0 as *mut mln_event_desc_s,
        act_prev: 0 as *mut mln_event_desc_s,
        act_next: 0 as *mut mln_event_desc_s,
        type_0: M_EV_FD,
        flag: 0,
        data: C2RustUnnamed {
            tm: mln_event_tm_t {
                data: 0 as *mut libc::c_void,
                handler: None,
                end_tm: 0,
            },
        },
    };
    memset(
        &mut tmp as *mut mln_event_desc_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_event_desc_t>() as libc::c_ulong,
    );
    tmp.type_0 = M_EV_FD;
    tmp.data.fd.fd = fd;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    rn = ({
        let mut tree: *mut mln_rbtree_t = (*event).ev_fd_tree;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_event_rbtree_fd_cmp(
                    &mut tmp as *mut mln_event_desc_t as *const libc::c_void,
                    (*ret_node).data,
                );
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    if !(rn == &mut (*(*event).ev_fd_tree).nil as *mut mln_rbtree_node_t) {
        if flag & 0x40 as libc::c_int as mln_u32_t != 0 {
            if flag & 0x10 as libc::c_int as mln_u32_t != 0 {
                mln_event_fd_nonblock_set(fd);
            }
            if flag & 0x20 as libc::c_int as mln_u32_t != 0 {
                mln_event_fd_block_set(fd);
            }
            if mln_event_fd_append_set(
                event,
                (*rn).data as *mut mln_event_desc_t,
                fd,
                flag,
                timeout_ms,
                data,
                fd_handler,
                1 as libc::c_int,
            ) < 0 as libc::c_int
            {
                pthread_mutex_unlock(&mut (*event).fd_lock);
                return -(1 as libc::c_int);
            }
        } else {
            if flag & 0x10 as libc::c_int as mln_u32_t != 0 {
                mln_event_fd_nonblock_set(fd);
            } else {
                mln_event_fd_block_set(fd);
            }
            if mln_event_fd_normal_set(
                event,
                (*rn).data as *mut mln_event_desc_t,
                fd,
                flag,
                timeout_ms,
                data,
                fd_handler,
                (if ((*((*rn).data as *mut mln_event_desc_t)).data.fd).is_clear()
                    as libc::c_int != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }),
            ) < 0 as libc::c_int
            {
                pthread_mutex_unlock(&mut (*event).fd_lock);
                return -(1 as libc::c_int);
            }
        }
        pthread_mutex_unlock(&mut (*event).fd_lock);
        return 0 as libc::c_int;
    }
    if flag & 0x10 as libc::c_int as mln_u32_t != 0 {
        mln_event_fd_nonblock_set(fd);
    } else {
        mln_event_fd_block_set(fd);
    }
    if mln_event_fd_normal_set(
        event,
        0 as *mut mln_event_desc_t,
        fd,
        flag,
        timeout_ms,
        data,
        fd_handler,
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        pthread_mutex_unlock(&mut (*event).fd_lock);
        return -(1 as libc::c_int);
    }
    pthread_mutex_unlock(&mut (*event).fd_lock);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_event_fd_normal_set(
    mut event: *mut mln_event_t,
    mut ed: *mut mln_event_desc_t,
    mut fd: libc::c_int,
    mut flag: mln_u32_t,
    mut timeout_ms: libc::c_int,
    mut data: *mut libc::c_void,
    mut fd_handler: ev_fd_handler,
    mut other_mark: libc::c_int,
) -> libc::c_int {
    if ed.is_null() {
        ed = malloc(::core::mem::size_of::<mln_event_desc_t>() as libc::c_ulong)
            as *mut mln_event_desc_t;
        if ed.is_null() {
            return -(1 as libc::c_int);
        }
        (*ed).type_0 = M_EV_FD;
        (*ed).flag = 0 as libc::c_int as mln_u32_t;
        memset(
            &mut (*ed).data.fd as *mut mln_event_fd_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<mln_event_fd_t>() as libc::c_ulong,
        );
        (*ed).data.fd.fd = fd;
        (*ed).next = 0 as *mut mln_event_desc_s;
        (*ed).prev = 0 as *mut mln_event_desc_s;
        (*ed).act_next = 0 as *mut mln_event_desc_s;
        (*ed).act_prev = 0 as *mut mln_event_desc_s;
        let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
        rn = mln_rbtree_node_new((*event).ev_fd_tree, ed as *mut libc::c_void);
        if rn.is_null() {
            free(ed as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        ({
            let mut tree: *mut mln_rbtree_t = (*event).ev_fd_tree;
            let mut y: *mut mln_rbtree_node_t = &mut (*tree).nil;
            let mut x: *mut mln_rbtree_node_t = (*tree).root;
            let mut nil: *mut mln_rbtree_node_t = &mut (*tree).nil;
            while x != nil {
                y = x;
                if mln_event_rbtree_fd_cmp((*rn).data, (*x).data) < 0 as libc::c_int {
                    x = (*x).left;
                } else {
                    x = (*x).right;
                }
            }
            (*rn).parent = y;
            if y == nil {
                (*tree).root = rn;
            } else if mln_event_rbtree_fd_cmp((*rn).data, (*y).data) < 0 as libc::c_int {
                (*y).left = rn;
            } else {
                (*y).right = rn;
            }
            (*rn).right = nil;
            (*rn).left = (*rn).right;
            (*rn).set_color(M_RB_RED as libc::c_int as mln_u32_t);
            rbtree_insert_fixup(tree, rn);
            if (*tree).min == nil {
                (*tree).min = rn;
            } else if mln_event_rbtree_fd_cmp((*rn).data, (*(*tree).min).data)
                < 0 as libc::c_int
            {
                (*tree).min = rn;
            }
            (*tree).nr_node = ((*tree).nr_node).wrapping_add(1);
            (*tree).nr_node;
            mln_rbtree_chain_add(&mut (*tree).head, &mut (*tree).tail, rn);
            //compile_error!("Function call expression is not supposed to be used")
        });
        ev_fd_wait_chain_add(
            &mut (*event).ev_fd_wait_head,
            &mut (*event).ev_fd_wait_tail,
            ed,
        );
        if mln_event_fd_timeout_set(event, ed, timeout_ms) < 0 as libc::c_int {
            free(ed as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    } else {
        if ((*ed).data.fd).is_clear() != 0 {
            let mut in_process: mln_u32_t = ((*ed).data.fd).in_process();
            memset(
                &mut (*ed).data.fd as *mut mln_event_fd_t as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<mln_event_fd_t>() as libc::c_ulong,
            );
            ((*ed).data.fd).set_in_process(in_process);
            (*ed).data.fd.fd = fd;
            (*ed).flag = 0 as libc::c_int as mln_u32_t;
        } else {
            (*ed).flag = 0 as libc::c_int as mln_u32_t;
            ((*ed).data.fd).set_rd_oneshot(0 as libc::c_int as mln_u32_t);
            ((*ed).data.fd).set_wr_oneshot(0 as libc::c_int as mln_u32_t);
            ((*ed).data.fd).set_err_oneshot(0 as libc::c_int as mln_u32_t);
            (*ed).data.fd.fd = fd;
        }
        if mln_event_fd_timeout_set(event, ed, timeout_ms) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return mln_event_fd_append_set(
        event,
        ed,
        fd,
        flag,
        -(2 as libc::c_int),
        data,
        fd_handler,
        other_mark,
    );
}
#[inline]
unsafe extern "C" fn mln_event_fd_append_set(
    mut event: *mut mln_event_t,
    mut ed: *mut mln_event_desc_t,
    mut fd: libc::c_int,
    mut flag: mln_u32_t,
    mut timeout_ms: libc::c_int,
    mut data: *mut libc::c_void,
    mut fd_handler: ev_fd_handler,
    mut other_mark: libc::c_int,
) -> libc::c_int {
    if mln_event_fd_timeout_set(event, ed, timeout_ms) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut oneshot: libc::c_int = if flag & 0x8 as libc::c_int as mln_u32_t != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut mask: libc::c_int = 0 as libc::c_int;
    if (*ed).flag & 0x1 as libc::c_int as mln_u32_t != 0 {
        mask |= 0x1 as libc::c_int;
    }
    if (*ed).flag & 0x2 as libc::c_int as mln_u32_t != 0 {
        mask |= 0x2 as libc::c_int;
    }
    if (*ed).flag & 0x4 as libc::c_int as mln_u32_t != 0 {
        mask |= 0x4 as libc::c_int;
    }
    if flag & 0x1 as libc::c_int as mln_u32_t != 0 {
        (*ed).flag |= 0x1 as libc::c_int as mln_u32_t;
        (*ed).data.fd.rcv_data = data;
        (*ed).data.fd.rcv_handler = fd_handler;
        if oneshot != 0 {
            ((*ed).data.fd).set_rd_oneshot(1 as libc::c_int as mln_u32_t);
        }
        mask |= 0x1 as libc::c_int;
    }
    if flag & 0x2 as libc::c_int as mln_u32_t != 0 {
        (*ed).flag |= 0x2 as libc::c_int as mln_u32_t;
        (*ed).data.fd.snd_data = data;
        (*ed).data.fd.snd_handler = fd_handler;
        if oneshot != 0 {
            ((*ed).data.fd).set_wr_oneshot(1 as libc::c_int as mln_u32_t);
        }
        mask |= 0x2 as libc::c_int;
    }
    if flag & 0x4 as libc::c_int as mln_u32_t != 0 {
        (*ed).flag |= 0x4 as libc::c_int as mln_u32_t;
        (*ed).data.fd.err_data = data;
        (*ed).data.fd.err_handler = fd_handler;
        if oneshot != 0 {
            ((*ed).data.fd).set_err_oneshot(1 as libc::c_int as mln_u32_t);
        }
        mask |= 0x4 as libc::c_int;
    }
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    memset(
        &mut ev as *mut epoll_event as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<epoll_event>() as libc::c_ulong,
    );
    match mask {
        1 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLONESHOT as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev.events = EPOLLIN as libc::c_int as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLONESHOT as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev.events = EPOLLIN as libc::c_int as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        2 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLOUT as libc::c_int | EPOLLONESHOT as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev.events = EPOLLOUT as libc::c_int as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLOUT as libc::c_int | EPOLLONESHOT as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev.events = EPOLLOUT as libc::c_int as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        3 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                        | EPOLLONESHOT as libc::c_int) as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                    | EPOLLONESHOT as libc::c_int) as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        4 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLERR as libc::c_int | EPOLLONESHOT as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev.events = EPOLLERR as libc::c_int as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLERR as libc::c_int | EPOLLONESHOT as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev.events = EPOLLERR as libc::c_int as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        5 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLERR as libc::c_int
                        | EPOLLONESHOT as libc::c_int) as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLERR as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLERR as libc::c_int
                    | EPOLLONESHOT as libc::c_int) as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLERR as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        6 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLOUT as libc::c_int | EPOLLERR as libc::c_int
                        | EPOLLONESHOT as libc::c_int) as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev
                        .events = (EPOLLOUT as libc::c_int | EPOLLERR as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLOUT as libc::c_int | EPOLLERR as libc::c_int
                    | EPOLLONESHOT as libc::c_int) as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev
                    .events = (EPOLLOUT as libc::c_int | EPOLLERR as libc::c_int)
                    as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        7 => {
            if other_mark != 0 {
                if oneshot != 0 {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                        | EPOLLERR as libc::c_int | EPOLLONESHOT as libc::c_int)
                        as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                } else {
                    ev
                        .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                        | EPOLLERR as libc::c_int) as uint32_t;
                    ev.data.ptr = ed as *mut libc::c_void;
                    epoll_ctl((*event).epollfd, 3 as libc::c_int, fd, &mut ev);
                }
            } else if oneshot != 0 {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                    | EPOLLERR as libc::c_int | EPOLLONESHOT as libc::c_int) as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            } else {
                ev
                    .events = (EPOLLIN as libc::c_int | EPOLLOUT as libc::c_int
                    | EPOLLERR as libc::c_int) as uint32_t;
                ev.data.ptr = ed as *mut libc::c_void;
                epoll_ctl((*event).epollfd, 1 as libc::c_int, fd, &mut ev);
            }
        }
        _ => return 0 as libc::c_int,
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_event_fd_timeout_set(
    mut ev: *mut mln_event_t,
    mut ed: *mut mln_event_desc_t,
    mut timeout_ms: libc::c_int,
) -> libc::c_int {
    if timeout_ms == -(2 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut ef: *mut mln_event_fd_t = &mut (*ed).data.fd;
    if timeout_ms == -(1 as libc::c_int) {
        if !((*ef).timeout_node).is_null() {
            ({
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                    }
                    let mut cp: fheap_copy = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        fheap_copy,
                    >(
                        Some(
                            mln_event_fd_timeout_copy
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                    );
                    if cp.is_none() {
                        cp = (*(*ev).ev_fd_timeout_heap).copy;
                    }
                    let mut r: libc::c_int = 0 as libc::c_int;
                    if cmp
                        .expect(
                            "non-null function pointer",
                        )((*(*ef).timeout_node).key, (*(*ev).ev_fd_timeout_heap).min_val)
                        == 0
                    {
                        r = -(1 as libc::c_int);
                    } else {
                        cp
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*ef).timeout_node).key,
                            (*(*ev).ev_fd_timeout_heap).min_val,
                        );
                        let mut y: *mut mln_fheap_node_t = (*(*ef).timeout_node).parent;
                        if !y.is_null()
                            && cmp
                                .expect(
                                    "non-null function pointer",
                                )((*(*ef).timeout_node).key, (*y).key) == 0
                        {
                            mln_fheap_cut(
                                (*ev).ev_fd_timeout_heap,
                                (*ef).timeout_node,
                                y,
                            );
                            mln_fheap_cascading_cut((*ev).ev_fd_timeout_heap, y);
                        }
                        if (*ef).timeout_node != (*(*ev).ev_fd_timeout_heap).min
                            && cmp
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*ef).timeout_node).key,
                                (*(*(*ev).ev_fd_timeout_heap).min).key,
                            ) == 0
                        {
                            (*(*ev).ev_fd_timeout_heap).min = (*ef).timeout_node;
                        }
                    }
                    r
                });
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                    }
                    let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap).min;
                    if !z.is_null() {
                        let mut child: *mut mln_fheap_node_t = 0
                            as *mut mln_fheap_node_t;
                        loop {
                            child = mln_fheap_remove_child(&mut (*z).child);
                            if child.is_null() {
                                break;
                            }
                            mln_fheap_add_child(
                                &mut (*(*ev).ev_fd_timeout_heap).root_list,
                                child,
                            );
                            (*child).parent = 0 as *mut mln_fheap_node_s;
                        }
                        let mut right: *mut mln_fheap_node_t = (*z).right;
                        mln_fheap_del_child(
                            &mut (*(*ev).ev_fd_timeout_heap).root_list,
                            z,
                        );
                        if z == right {
                            (*(*ev).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                        } else {
                            (*(*ev).ev_fd_timeout_heap).min = right;
                            mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                        }
                        (*(*ev).ev_fd_timeout_heap)
                            .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                        (*(*ev).ev_fd_timeout_heap).num;
                    }
                    z
                });
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                    }
                    let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap).min;
                    if !z.is_null() {
                        let mut child: *mut mln_fheap_node_t = 0
                            as *mut mln_fheap_node_t;
                        loop {
                            child = mln_fheap_remove_child(&mut (*z).child);
                            if child.is_null() {
                                break;
                            }
                            mln_fheap_add_child(
                                &mut (*(*ev).ev_fd_timeout_heap).root_list,
                                child,
                            );
                            (*child).parent = 0 as *mut mln_fheap_node_s;
                        }
                        let mut right: *mut mln_fheap_node_t = (*z).right;
                        mln_fheap_del_child(
                            &mut (*(*ev).ev_fd_timeout_heap).root_list,
                            z,
                        );
                        if z == right {
                            (*(*ev).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                        } else {
                            (*(*ev).ev_fd_timeout_heap).min = right;
                            mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                        }
                        (*(*ev).ev_fd_timeout_heap)
                            .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                        (*(*ev).ev_fd_timeout_heap).num;
                    }
                    z
                })
            });
            let mut f: fheap_key_free = ::core::mem::transmute::<
                *mut libc::c_void,
                fheap_key_free,
            >(0 as *mut libc::c_void);
            if f.is_none() {
                f = (*(*ev).ev_fd_timeout_heap).key_free;
            }
            if !((*ef).timeout_node).is_null() {
                if f.is_some() && !((*(*ef).timeout_node).key).is_null() {
                    f.expect("non-null function pointer")((*(*ef).timeout_node).key);
                }
                if (*(*ef).timeout_node).nofree() == 0 {
                    if !((*(*ev).ev_fd_timeout_heap).pool).is_null() {
                        ((*(*ev).ev_fd_timeout_heap).pool_free)
                            .expect(
                                "non-null function pointer",
                            )((*ef).timeout_node as *mut libc::c_void);
                    } else {
                        free((*ef).timeout_node as *mut libc::c_void);
                    }
                }
            }
            (*ef).timeout_node = 0 as *mut mln_fheap_node_t;
            (*ef).end_us = 0 as libc::c_int as mln_u64_t;
        }
        return 0 as libc::c_int;
    }
    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memset(
        &mut tv as *mut timeval as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    if ((*ef).timeout_node).is_null() {
        (*ef)
            .end_us = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec
            + (timeout_ms * 1000 as libc::c_int) as libc::c_long) as mln_u64_t;
        fn_0 = mln_fheap_node_new((*ev).ev_fd_timeout_heap, ed as *mut libc::c_void);
        if fn_0.is_null() {
            return -(1 as libc::c_int);
        }
        (*ef).timeout_node = fn_0;
        ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fd_timeout_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*ev).ev_fd_timeout_heap).cmp;
            }
            mln_fheap_add_child(&mut (*(*ev).ev_fd_timeout_heap).root_list, fn_0);
            (*fn_0).parent = 0 as *mut mln_fheap_node_s;
            if ((*(*ev).ev_fd_timeout_heap).min).is_null() {
                (*(*ev).ev_fd_timeout_heap).min = fn_0;
            } else if cmp
                .expect(
                    "non-null function pointer",
                )((*fn_0).key, (*(*(*ev).ev_fd_timeout_heap).min).key) == 0
            {
                (*(*ev).ev_fd_timeout_heap).min = fn_0;
            }
            (*(*ev).ev_fd_timeout_heap)
                .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_add(1);
            (*(*ev).ev_fd_timeout_heap).num;
            (*(*ev).ev_fd_timeout_heap).num
        });
    } else {
        fn_0 = (*ef).timeout_node;
        ({
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                }
                let mut cp: fheap_copy = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                    fheap_copy,
                >(
                    Some(
                        mln_event_fd_timeout_copy
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                );
                if cp.is_none() {
                    cp = (*(*ev).ev_fd_timeout_heap).copy;
                }
                let mut r: libc::c_int = 0 as libc::c_int;
                if cmp
                    .expect(
                        "non-null function pointer",
                    )((*fn_0).key, (*(*ev).ev_fd_timeout_heap).min_val) == 0
                {
                    r = -(1 as libc::c_int);
                } else {
                    cp
                        .expect(
                            "non-null function pointer",
                        )((*fn_0).key, (*(*ev).ev_fd_timeout_heap).min_val);
                    let mut y: *mut mln_fheap_node_t = (*fn_0).parent;
                    if !y.is_null()
                        && cmp.expect("non-null function pointer")((*fn_0).key, (*y).key)
                            == 0
                    {
                        mln_fheap_cut((*ev).ev_fd_timeout_heap, fn_0, y);
                        mln_fheap_cascading_cut((*ev).ev_fd_timeout_heap, y);
                    }
                    if fn_0 != (*(*ev).ev_fd_timeout_heap).min
                        && cmp
                            .expect(
                                "non-null function pointer",
                            )((*fn_0).key, (*(*(*ev).ev_fd_timeout_heap).min).key) == 0
                    {
                        (*(*ev).ev_fd_timeout_heap).min = fn_0;
                    }
                }
                r
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*ev).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(&mut (*(*ev).ev_fd_timeout_heap).root_list, z);
                    if z == right {
                        (*(*ev).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*ev).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                    }
                    (*(*ev).ev_fd_timeout_heap)
                        .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*ev).ev_fd_timeout_heap).num;
                }
                z
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*ev).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*ev).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*ev).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(&mut (*(*ev).ev_fd_timeout_heap).root_list, z);
                    if z == right {
                        (*(*ev).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*ev).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*ev).ev_fd_timeout_heap, cmp);
                    }
                    (*(*ev).ev_fd_timeout_heap)
                        .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*ev).ev_fd_timeout_heap).num;
                }
                z
            })
        });
        (*ef)
            .end_us = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec
            + (timeout_ms * 1000 as libc::c_int) as libc::c_long) as mln_u64_t;
        ({
            let mut cmp: fheap_cmp = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
                fheap_cmp,
            >(
                Some(
                    mln_event_fd_timeout_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if cmp.is_none() {
                cmp = (*(*ev).ev_fd_timeout_heap).cmp;
            }
            mln_fheap_add_child(&mut (*(*ev).ev_fd_timeout_heap).root_list, fn_0);
            (*fn_0).parent = 0 as *mut mln_fheap_node_s;
            if ((*(*ev).ev_fd_timeout_heap).min).is_null() {
                (*(*ev).ev_fd_timeout_heap).min = fn_0;
            } else if cmp
                .expect(
                    "non-null function pointer",
                )((*fn_0).key, (*(*(*ev).ev_fd_timeout_heap).min).key) == 0
            {
                (*(*ev).ev_fd_timeout_heap).min = fn_0;
            }
            (*(*ev).ev_fd_timeout_heap)
                .num = ((*(*ev).ev_fd_timeout_heap).num).wrapping_add(1);
            (*(*ev).ev_fd_timeout_heap).num;
            (*(*ev).ev_fd_timeout_heap).num
        });
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_event_fd_clr_set(
    mut event: *mut mln_event_t,
    mut fd: libc::c_int,
) {
    let mut tmp: mln_event_desc_t = mln_event_desc_t {
        prev: 0 as *mut mln_event_desc_s,
        next: 0 as *mut mln_event_desc_s,
        act_prev: 0 as *mut mln_event_desc_s,
        act_next: 0 as *mut mln_event_desc_s,
        type_0: M_EV_FD,
        flag: 0,
        data: C2RustUnnamed {
            tm: mln_event_tm_t {
                data: 0 as *mut libc::c_void,
                handler: None,
                end_tm: 0,
            },
        },
    };
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    memset(
        &mut tmp as *mut mln_event_desc_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mln_event_desc_t>() as libc::c_ulong,
    );
    tmp.type_0 = M_EV_FD;
    tmp.data.fd.fd = fd;
    let mut rn: *mut mln_rbtree_node_t = 0 as *mut mln_rbtree_node_t;
    rn = ({
        let mut tree: *mut mln_rbtree_t = (*event).ev_fd_tree;
        let mut ret_node: *mut mln_rbtree_node_t = (*tree).root;
        let mut ret: libc::c_int = 0;
        while ret_node != &mut (*tree).nil as *mut mln_rbtree_node_t
            && {
                ret = mln_event_rbtree_fd_cmp(
                    &mut tmp as *mut mln_event_desc_t as *const libc::c_void,
                    (*ret_node).data,
                );
                ret != 0 as libc::c_int
            }
        {
            if ret < 0 as libc::c_int {
                ret_node = (*ret_node).left;
            } else {
                ret_node = (*ret_node).right;
            }
        }
        ret_node
    });
    if rn == &mut (*(*event).ev_fd_tree).nil as *mut mln_rbtree_node_t {
        return;
    }
    ed = (*rn).data as *mut mln_event_desc_t;
    if !((*ed).data.fd.timeout_node).is_null() {
        ({
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut cp: fheap_copy = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                    fheap_copy,
                >(
                    Some(
                        mln_event_fd_timeout_copy
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                );
                if cp.is_none() {
                    cp = (*(*event).ev_fd_timeout_heap).copy;
                }
                let mut r: libc::c_int = 0 as libc::c_int;
                if cmp
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*ed).data.fd.timeout_node).key,
                    (*(*event).ev_fd_timeout_heap).min_val,
                ) == 0
                {
                    r = -(1 as libc::c_int);
                } else {
                    cp
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*ed).data.fd.timeout_node).key,
                        (*(*event).ev_fd_timeout_heap).min_val,
                    );
                    let mut y: *mut mln_fheap_node_t = (*(*ed).data.fd.timeout_node)
                        .parent;
                    if !y.is_null()
                        && cmp
                            .expect(
                                "non-null function pointer",
                            )((*(*ed).data.fd.timeout_node).key, (*y).key) == 0
                    {
                        mln_fheap_cut(
                            (*event).ev_fd_timeout_heap,
                            (*ed).data.fd.timeout_node,
                            y,
                        );
                        mln_fheap_cascading_cut((*event).ev_fd_timeout_heap, y);
                    }
                    if (*ed).data.fd.timeout_node != (*(*event).ev_fd_timeout_heap).min
                        && cmp
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*ed).data.fd.timeout_node).key,
                            (*(*(*event).ev_fd_timeout_heap).min).key,
                        ) == 0
                    {
                        (*(*event).ev_fd_timeout_heap).min = (*ed).data.fd.timeout_node;
                    }
                }
                r
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(
                        &mut (*(*event).ev_fd_timeout_heap).root_list,
                        z,
                    );
                    if z == right {
                        (*(*event).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*event).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                    }
                    (*(*event).ev_fd_timeout_heap)
                        .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*event).ev_fd_timeout_heap).num;
                }
                z
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(
                        &mut (*(*event).ev_fd_timeout_heap).root_list,
                        z,
                    );
                    if z == right {
                        (*(*event).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*event).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                    }
                    (*(*event).ev_fd_timeout_heap)
                        .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*event).ev_fd_timeout_heap).num;
                }
                z
            })
        });
        let mut f: fheap_key_free = ::core::mem::transmute::<
            *mut libc::c_void,
            fheap_key_free,
        >(0 as *mut libc::c_void);
        if f.is_none() {
            f = (*(*event).ev_fd_timeout_heap).key_free;
        }
        if !((*ed).data.fd.timeout_node).is_null() {
            if f.is_some() && !((*(*ed).data.fd.timeout_node).key).is_null() {
                f.expect("non-null function pointer")((*(*ed).data.fd.timeout_node).key);
            }
            if (*(*ed).data.fd.timeout_node).nofree() == 0 {
                if !((*(*event).ev_fd_timeout_heap).pool).is_null() {
                    ((*(*event).ev_fd_timeout_heap).pool_free)
                        .expect(
                            "non-null function pointer",
                        )((*ed).data.fd.timeout_node as *mut libc::c_void);
                } else {
                    free((*ed).data.fd.timeout_node as *mut libc::c_void);
                }
            }
        }
        (*ed).data.fd.timeout_node = 0 as *mut mln_fheap_node_t;
        (*ed).data.fd.end_us = 0 as libc::c_int as mln_u64_t;
    }
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    memset(
        &mut ev as *mut epoll_event as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<epoll_event>() as libc::c_ulong,
    );
    ev.data.ptr = ed as *mut libc::c_void;
    epoll_ctl((*event).epollfd, 2 as libc::c_int, fd, &mut ev);
    if ((*ed).data.fd).in_process() != 0 {
        ((*ed).data.fd).set_is_clear(1 as libc::c_int as mln_u32_t);
        return;
    }
    mln_rbtree_delete((*event).ev_fd_tree, rn);
    mln_rbtree_node_free((*event).ev_fd_tree, rn);
    if ((*ed).data.fd).in_active() != 0 {
        ev_fd_active_chain_del(
            &mut (*event).ev_fd_active_head,
            &mut (*event).ev_fd_active_tail,
            ed,
        );
        (*ed).data.fd.active_flag = 0 as libc::c_int as mln_u32_t;
        ((*ed).data.fd).set_in_active(0 as libc::c_int as mln_u32_t);
    }
    ev_fd_wait_chain_del(
        &mut (*event).ev_fd_wait_head,
        &mut (*event).ev_fd_wait_tail,
        ed,
    );
    mln_event_desc_free(ed as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_callback_set(
    mut ev: *mut mln_event_t,
    mut dc: dispatch_callback,
    mut dc_data: *mut libc::c_void,
) {
    pthread_mutex_lock(&mut (*ev).cb_lock);
    (*ev).callback = dc;
    (*ev).callback_data = dc_data;
    pthread_mutex_unlock(&mut (*ev).cb_lock);
}
#[inline]
unsafe extern "C" fn mln_event_fd_nonblock_set(mut fd: libc::c_int) {
    let mut flg: libc::c_int = 0;
    flg = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
    flg = fcntl(fd, 4 as libc::c_int, flg | 0o4000 as libc::c_int);
}
#[inline]
unsafe extern "C" fn mln_event_fd_block_set(mut fd: libc::c_int) {
    let mut flg: libc::c_int = 0;
    flg = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
    flg = fcntl(fd, 4 as libc::c_int, flg & !(0o4000 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn mln_event_dispatch(mut event: *mut mln_event_t) {
    let mut mod_event: __uint32_t = 0;
    let mut nfds: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut oneshot: libc::c_int = 0;
    let mut other_oneshot: libc::c_int = 0;
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    let mut events: [epoll_event; 1024] = [epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    }; 1024];
    let mut ev: *mut epoll_event = 0 as *mut epoll_event;
    let mut mod_ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    loop {
        if pthread_mutex_trylock(&mut (*event).cb_lock) == 0 {
            let mut cb: dispatch_callback = (*event).callback;
            let mut data: *mut libc::c_void = (*event).callback_data;
            if cb.is_some() {
                pthread_mutex_unlock(&mut (*event).cb_lock);
                cb.expect("non-null function pointer")(event, data);
            } else {
                pthread_mutex_unlock(&mut (*event).cb_lock);
            }
        }
        if (*event).is_break() != 0 {
            return;
        }
        mln_event_timer_process(event);
        if (*event).is_break() != 0 {
            return;
        }
        mln_event_active_fd_process(event);
        if (*event).is_break() != 0 {
            return;
        }
        mln_event_fd_timeout_process(event);
        if (*event).is_break() != 0 {
            return;
        }
        mln_event_timer_process(event);
        if (*event).is_break() != 0 {
            return;
        }
        if pthread_mutex_trylock(&mut (*event).fd_lock) != 0 {
            epoll_wait(
                (*event).unusedfd,
                events.as_mut_ptr(),
                1024 as libc::c_int,
                3 as libc::c_int,
            );
        } else {
            nfds = epoll_wait(
                (*event).epollfd,
                events.as_mut_ptr(),
                1024 as libc::c_int,
                7 as libc::c_int,
            );
            if nfds < 0 as libc::c_int {
                if *__errno_location() == 4 as libc::c_int {
                    pthread_mutex_unlock(&mut (*event).fd_lock);
                    continue;
                }
            } else if nfds == 0 as libc::c_int {
                pthread_mutex_unlock(&mut (*event).fd_lock);
                epoll_wait(
                    (*event).unusedfd,
                    events.as_mut_ptr(),
                    1024 as libc::c_int,
                    3 as libc::c_int,
                );
                continue;
            }
            n = 0 as libc::c_int;
            while n < nfds {
                mod_event = 0 as libc::c_int as __uint32_t;
                oneshot = 0 as libc::c_int;
                other_oneshot = 0 as libc::c_int;
                ev = &mut *events.as_mut_ptr().offset(n as isize) as *mut epoll_event;
                ed = (*ev).data.ptr as *mut mln_event_desc_t;
                if !(((*ed).data.fd).is_clear() != 0) {
                    if (*ev).events & EPOLLIN as libc::c_int as libc::c_uint != 0 {
                        if ((*ed).data.fd).rd_oneshot() != 0 {
                            if ((*ed).data.fd).in_active() as libc::c_int != 0
                                || ((*ed).data.fd).in_process() as libc::c_int != 0
                            {
                                if (*ed).data.fd.active_flag
                                    & 0x1 as libc::c_int as mln_u32_t == 0
                                {
                                    mln_event_fd_append_set(
                                        event,
                                        ed,
                                        (*ed).data.fd.fd,
                                        0x1 as libc::c_int as mln_u32_t
                                            | 0x8 as libc::c_int as mln_u32_t,
                                        -(2 as libc::c_int),
                                        (*ed).data.fd.rcv_data,
                                        (*ed).data.fd.rcv_handler,
                                        1 as libc::c_int,
                                    );
                                }
                            } else {
                                ((*ed).data.fd)
                                    .set_rd_oneshot(0 as libc::c_int as mln_u32_t);
                                oneshot = 1 as libc::c_int;
                                (*ed).flag &= !(0x1 as libc::c_int as mln_u32_t);
                            }
                        }
                        if ((*ed).data.fd).in_active() == 0
                            && ((*ed).data.fd).in_process() == 0
                        {
                            (*ed).data.fd.active_flag |= 0x1 as libc::c_int as mln_u32_t;
                        }
                    }
                    if (*ev).events & EPOLLOUT as libc::c_int as libc::c_uint != 0 {
                        if ((*ed).data.fd).wr_oneshot() != 0 {
                            if ((*ed).data.fd).in_active() as libc::c_int != 0
                                || ((*ed).data.fd).in_process() as libc::c_int != 0
                            {
                                if (*ed).data.fd.active_flag
                                    & 0x2 as libc::c_int as mln_u32_t == 0
                                {
                                    mln_event_fd_append_set(
                                        event,
                                        ed,
                                        (*ed).data.fd.fd,
                                        0x2 as libc::c_int as mln_u32_t
                                            | 0x8 as libc::c_int as mln_u32_t,
                                        -(2 as libc::c_int),
                                        (*ed).data.fd.snd_data,
                                        (*ed).data.fd.snd_handler,
                                        1 as libc::c_int,
                                    );
                                }
                            } else {
                                ((*ed).data.fd)
                                    .set_wr_oneshot(0 as libc::c_int as mln_u32_t);
                                oneshot = 1 as libc::c_int;
                                (*ed).flag &= !(0x2 as libc::c_int as mln_u32_t);
                            }
                        }
                        if ((*ed).data.fd).in_active() == 0
                            && ((*ed).data.fd).in_process() == 0
                        {
                            (*ed).data.fd.active_flag |= 0x2 as libc::c_int as mln_u32_t;
                        }
                    }
                    if (*ev).events & EPOLLERR as libc::c_int as libc::c_uint != 0 {
                        if ((*ed).data.fd).err_oneshot() != 0 {
                            if ((*ed).data.fd).in_active() as libc::c_int != 0
                                || ((*ed).data.fd).in_process() as libc::c_int != 0
                            {
                                if (*ed).data.fd.active_flag
                                    & 0x4 as libc::c_int as mln_u32_t == 0
                                {
                                    mln_event_fd_append_set(
                                        event,
                                        ed,
                                        (*ed).data.fd.fd,
                                        0x4 as libc::c_int as mln_u32_t
                                            | 0x8 as libc::c_int as mln_u32_t,
                                        -(2 as libc::c_int),
                                        (*ed).data.fd.err_data,
                                        (*ed).data.fd.err_handler,
                                        1 as libc::c_int,
                                    );
                                }
                            } else {
                                ((*ed).data.fd)
                                    .set_err_oneshot(0 as libc::c_int as mln_u32_t);
                                oneshot = 1 as libc::c_int;
                                (*ed).flag &= !(0x4 as libc::c_int as mln_u32_t);
                            }
                        }
                        if ((*ed).data.fd).in_active() == 0
                            && ((*ed).data.fd).in_process() == 0
                        {
                            (*ed).data.fd.active_flag |= 0x4 as libc::c_int as mln_u32_t;
                        }
                    }
                    if !(((*ed).data.fd).in_active() as libc::c_int != 0
                        || ((*ed).data.fd).in_process() as libc::c_int != 0)
                    {
                        ev_fd_active_chain_add(
                            &mut (*event).ev_fd_active_head,
                            &mut (*event).ev_fd_active_tail,
                            ed,
                        );
                        ((*ed).data.fd).set_in_active(1 as libc::c_int as mln_u32_t);
                        if oneshot != 0 {
                            if (*ed).flag & 0x1 as libc::c_int as mln_u32_t != 0 {
                                mod_event |= EPOLLIN as libc::c_int as libc::c_uint;
                            }
                            if (*ed).flag & 0x2 as libc::c_int as mln_u32_t != 0 {
                                mod_event |= EPOLLOUT as libc::c_int as libc::c_uint;
                            }
                            if (*ed).flag & 0x4 as libc::c_int as mln_u32_t != 0 {
                                mod_event |= EPOLLERR as libc::c_int as libc::c_uint;
                            }
                            if ((*ed).data.fd).rd_oneshot() as libc::c_int != 0
                                || ((*ed).data.fd).wr_oneshot() as libc::c_int != 0
                                || ((*ed).data.fd).err_oneshot() as libc::c_int != 0
                            {
                                other_oneshot = 1 as libc::c_int;
                            }
                            memset(
                                &mut mod_ev as *mut epoll_event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<epoll_event>() as libc::c_ulong,
                            );
                            if other_oneshot != 0 {
                                mod_ev
                                    .events = mod_event
                                    | EPOLLONESHOT as libc::c_int as libc::c_uint;
                                mod_ev.data.ptr = ed as *mut libc::c_void;
                                epoll_ctl(
                                    (*event).epollfd,
                                    3 as libc::c_int,
                                    (*ed).data.fd.fd,
                                    &mut mod_ev,
                                );
                            } else {
                                mod_ev.events = mod_event;
                                mod_ev.data.ptr = ed as *mut libc::c_void;
                                epoll_ctl(
                                    (*event).epollfd,
                                    3 as libc::c_int,
                                    (*ed).data.fd.fd,
                                    &mut mod_ev,
                                );
                            }
                        }
                    }
                }
                n += 1;
                n;
            }
            pthread_mutex_unlock(&mut (*event).fd_lock);
        }
    };
}
#[inline]
unsafe extern "C" fn mln_event_active_fd_process(mut event: *mut mln_event_t) {
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    let mut ef: *mut mln_event_fd_t = 0 as *mut mln_event_fd_t;
    let mut h: ev_fd_handler = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fd: libc::c_int = 0;
    loop {
        if pthread_mutex_trylock(&mut (*event).fd_lock) != 0 {
            return;
        }
        ed = (*event).ev_fd_active_head;
        if ed.is_null() {
            break;
        }
        ev_fd_active_chain_del(
            &mut (*event).ev_fd_active_head,
            &mut (*event).ev_fd_active_tail,
            ed,
        );
        ef = &mut (*ed).data.fd;
        if !((*ef).timeout_node).is_null() {
            ({
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*event).ev_fd_timeout_heap).cmp;
                    }
                    let mut cp: fheap_copy = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        fheap_copy,
                    >(
                        Some(
                            mln_event_fd_timeout_copy
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                    );
                    if cp.is_none() {
                        cp = (*(*event).ev_fd_timeout_heap).copy;
                    }
                    let mut r: libc::c_int = 0 as libc::c_int;
                    if cmp
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*ef).timeout_node).key,
                        (*(*event).ev_fd_timeout_heap).min_val,
                    ) == 0
                    {
                        r = -(1 as libc::c_int);
                    } else {
                        cp
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*ef).timeout_node).key,
                            (*(*event).ev_fd_timeout_heap).min_val,
                        );
                        let mut y: *mut mln_fheap_node_t = (*(*ef).timeout_node).parent;
                        if !y.is_null()
                            && cmp
                                .expect(
                                    "non-null function pointer",
                                )((*(*ef).timeout_node).key, (*y).key) == 0
                        {
                            mln_fheap_cut(
                                (*event).ev_fd_timeout_heap,
                                (*ef).timeout_node,
                                y,
                            );
                            mln_fheap_cascading_cut((*event).ev_fd_timeout_heap, y);
                        }
                        if (*ef).timeout_node != (*(*event).ev_fd_timeout_heap).min
                            && cmp
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*ef).timeout_node).key,
                                (*(*(*event).ev_fd_timeout_heap).min).key,
                            ) == 0
                        {
                            (*(*event).ev_fd_timeout_heap).min = (*ef).timeout_node;
                        }
                    }
                    r
                });
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*event).ev_fd_timeout_heap).cmp;
                    }
                    let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap)
                        .min;
                    if !z.is_null() {
                        let mut child: *mut mln_fheap_node_t = 0
                            as *mut mln_fheap_node_t;
                        loop {
                            child = mln_fheap_remove_child(&mut (*z).child);
                            if child.is_null() {
                                break;
                            }
                            mln_fheap_add_child(
                                &mut (*(*event).ev_fd_timeout_heap).root_list,
                                child,
                            );
                            (*child).parent = 0 as *mut mln_fheap_node_s;
                        }
                        let mut right: *mut mln_fheap_node_t = (*z).right;
                        mln_fheap_del_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            z,
                        );
                        if z == right {
                            (*(*event).ev_fd_timeout_heap)
                                .min = 0 as *mut mln_fheap_node_t;
                        } else {
                            (*(*event).ev_fd_timeout_heap).min = right;
                            mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                        }
                        (*(*event).ev_fd_timeout_heap)
                            .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                        (*(*event).ev_fd_timeout_heap).num;
                    }
                    z
                });
                ({
                    let mut cmp: fheap_cmp = ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                        >,
                        fheap_cmp,
                    >(
                        Some(
                            mln_event_fd_timeout_cmp
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ),
                    );
                    if cmp.is_none() {
                        cmp = (*(*event).ev_fd_timeout_heap).cmp;
                    }
                    let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap)
                        .min;
                    if !z.is_null() {
                        let mut child: *mut mln_fheap_node_t = 0
                            as *mut mln_fheap_node_t;
                        loop {
                            child = mln_fheap_remove_child(&mut (*z).child);
                            if child.is_null() {
                                break;
                            }
                            mln_fheap_add_child(
                                &mut (*(*event).ev_fd_timeout_heap).root_list,
                                child,
                            );
                            (*child).parent = 0 as *mut mln_fheap_node_s;
                        }
                        let mut right: *mut mln_fheap_node_t = (*z).right;
                        mln_fheap_del_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            z,
                        );
                        if z == right {
                            (*(*event).ev_fd_timeout_heap)
                                .min = 0 as *mut mln_fheap_node_t;
                        } else {
                            (*(*event).ev_fd_timeout_heap).min = right;
                            mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                        }
                        (*(*event).ev_fd_timeout_heap)
                            .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                        (*(*event).ev_fd_timeout_heap).num;
                    }
                    z
                })
            });
            let mut f: fheap_key_free = ::core::mem::transmute::<
                *mut libc::c_void,
                fheap_key_free,
            >(0 as *mut libc::c_void);
            if f.is_none() {
                f = (*(*event).ev_fd_timeout_heap).key_free;
            }
            if !((*ef).timeout_node).is_null() {
                if f.is_some() && !((*(*ef).timeout_node).key).is_null() {
                    f.expect("non-null function pointer")((*(*ef).timeout_node).key);
                }
                if (*(*ef).timeout_node).nofree() == 0 {
                    if !((*(*event).ev_fd_timeout_heap).pool).is_null() {
                        ((*(*event).ev_fd_timeout_heap).pool_free)
                            .expect(
                                "non-null function pointer",
                            )((*ef).timeout_node as *mut libc::c_void);
                    } else {
                        free((*ef).timeout_node as *mut libc::c_void);
                    }
                }
            }
            (*ef).timeout_node = 0 as *mut mln_fheap_node_t;
            (*ef).end_us = 0 as libc::c_int as mln_u64_t;
        }
        (*ef).set_in_active(0 as libc::c_int as mln_u32_t);
        (*ef).set_in_process(1 as libc::c_int as mln_u32_t);
        if (*ef).is_clear() as libc::c_int != 0
            || (*event).is_break() as libc::c_int != 0
        {
            (*ef).active_flag = 0 as libc::c_int as mln_u32_t;
        }
        if (*ef).active_flag & 0x1 as libc::c_int as mln_u32_t != 0 {
            if ((*ef).rcv_handler).is_some() {
                h = (*ef).rcv_handler;
                data = (*ef).rcv_data;
                fd = (*ef).fd;
                pthread_mutex_unlock(&mut (*event).fd_lock);
                h.expect("non-null function pointer")(event, fd, data);
                pthread_mutex_lock(&mut (*event).fd_lock);
            }
            (*ef).active_flag &= !(0x1 as libc::c_int as mln_u32_t);
        }
        if (*ef).is_clear() as libc::c_int != 0
            || (*event).is_break() as libc::c_int != 0
        {
            (*ef).active_flag = 0 as libc::c_int as mln_u32_t;
        }
        if (*ef).active_flag & 0x2 as libc::c_int as mln_u32_t != 0 {
            if ((*ef).snd_handler).is_some() {
                h = (*ef).snd_handler;
                data = (*ef).snd_data;
                fd = (*ef).fd;
                pthread_mutex_unlock(&mut (*event).fd_lock);
                h.expect("non-null function pointer")(event, fd, data);
                pthread_mutex_lock(&mut (*event).fd_lock);
            }
            (*ef).active_flag &= !(0x2 as libc::c_int as mln_u32_t);
        }
        if (*ef).is_clear() as libc::c_int != 0
            || (*event).is_break() as libc::c_int != 0
        {
            (*ef).active_flag = 0 as libc::c_int as mln_u32_t;
        }
        if (*ef).active_flag & 0x4 as libc::c_int as mln_u32_t != 0 {
            if ((*ef).err_handler).is_some() {
                h = (*ef).err_handler;
                data = (*ef).err_data;
                fd = (*ef).fd;
                pthread_mutex_unlock(&mut (*event).fd_lock);
                h.expect("non-null function pointer")(event, fd, data);
                pthread_mutex_lock(&mut (*event).fd_lock);
            }
            (*ef).active_flag &= !(0x4 as libc::c_int as mln_u32_t);
        }
        (*ef).set_in_process(0 as libc::c_int as mln_u32_t);
        if (*ef).is_clear() != 0 {
            mln_event_fd_clr_set(event, (*ef).fd);
        }
        pthread_mutex_unlock(&mut (*event).fd_lock);
        if (*event).is_break() != 0 {
            return;
        }
    }
    pthread_mutex_unlock(&mut (*event).fd_lock);
}
#[inline]
unsafe extern "C" fn mln_event_fd_timeout_process(mut event: *mut mln_event_t) {
    let mut now: mln_u64_t = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    now = (tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec) as mln_u64_t;
    let mut ed: *mut mln_event_desc_t = 0 as *mut mln_event_desc_t;
    let mut fn_0: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
    let mut ef: *mut mln_event_fd_t = 0 as *mut mln_event_fd_t;
    let mut h: ev_fd_handler = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fd: libc::c_int = 0;
    loop {
        if pthread_mutex_trylock(&mut (*event).fd_lock) != 0 {
            return;
        }
        fn_0 = (*(*event).ev_fd_timeout_heap).min;
        if fn_0.is_null() {
            pthread_mutex_unlock(&mut (*event).fd_lock);
            return;
        }
        ed = (*fn_0).key as *mut mln_event_desc_t;
        ef = &mut (*ed).data.fd;
        if (*ef).in_active() != 0 {
            ev_fd_active_chain_del(
                &mut (*event).ev_fd_active_head,
                &mut (*event).ev_fd_active_tail,
                ed,
            );
            (*ef).set_in_active(0 as libc::c_int as mln_u32_t);
        }
        if (*ef).end_us > now {
            pthread_mutex_unlock(&mut (*event).fd_lock);
            return;
        }
        (*ef).set_in_process(1 as libc::c_int as mln_u32_t);
        ({
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut cp: fheap_copy = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                    fheap_copy,
                >(
                    Some(
                        mln_event_fd_timeout_copy
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                );
                if cp.is_none() {
                    cp = (*(*event).ev_fd_timeout_heap).copy;
                }
                let mut r: libc::c_int = 0 as libc::c_int;
                if cmp
                    .expect(
                        "non-null function pointer",
                    )((*fn_0).key, (*(*event).ev_fd_timeout_heap).min_val) == 0
                {
                    r = -(1 as libc::c_int);
                } else {
                    cp
                        .expect(
                            "non-null function pointer",
                        )((*fn_0).key, (*(*event).ev_fd_timeout_heap).min_val);
                    let mut y: *mut mln_fheap_node_t = (*fn_0).parent;
                    if !y.is_null()
                        && cmp.expect("non-null function pointer")((*fn_0).key, (*y).key)
                            == 0
                    {
                        mln_fheap_cut((*event).ev_fd_timeout_heap, fn_0, y);
                        mln_fheap_cascading_cut((*event).ev_fd_timeout_heap, y);
                    }
                    if fn_0 != (*(*event).ev_fd_timeout_heap).min
                        && cmp
                            .expect(
                                "non-null function pointer",
                            )((*fn_0).key, (*(*(*event).ev_fd_timeout_heap).min).key)
                            == 0
                    {
                        (*(*event).ev_fd_timeout_heap).min = fn_0;
                    }
                }
                r
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(
                        &mut (*(*event).ev_fd_timeout_heap).root_list,
                        z,
                    );
                    if z == right {
                        (*(*event).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*event).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                    }
                    (*(*event).ev_fd_timeout_heap)
                        .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*event).ev_fd_timeout_heap).num;
                }
                z
            });
            ({
                let mut cmp: fheap_cmp = ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    fheap_cmp,
                >(
                    Some(
                        mln_event_fd_timeout_cmp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if cmp.is_none() {
                    cmp = (*(*event).ev_fd_timeout_heap).cmp;
                }
                let mut z: *mut mln_fheap_node_t = (*(*event).ev_fd_timeout_heap).min;
                if !z.is_null() {
                    let mut child: *mut mln_fheap_node_t = 0 as *mut mln_fheap_node_t;
                    loop {
                        child = mln_fheap_remove_child(&mut (*z).child);
                        if child.is_null() {
                            break;
                        }
                        mln_fheap_add_child(
                            &mut (*(*event).ev_fd_timeout_heap).root_list,
                            child,
                        );
                        (*child).parent = 0 as *mut mln_fheap_node_s;
                    }
                    let mut right: *mut mln_fheap_node_t = (*z).right;
                    mln_fheap_del_child(
                        &mut (*(*event).ev_fd_timeout_heap).root_list,
                        z,
                    );
                    if z == right {
                        (*(*event).ev_fd_timeout_heap).min = 0 as *mut mln_fheap_node_t;
                    } else {
                        (*(*event).ev_fd_timeout_heap).min = right;
                        mln_fheap_consolidate((*event).ev_fd_timeout_heap, cmp);
                    }
                    (*(*event).ev_fd_timeout_heap)
                        .num = ((*(*event).ev_fd_timeout_heap).num).wrapping_sub(1);
                    (*(*event).ev_fd_timeout_heap).num;
                }
                z
            })
        });
        let mut f: fheap_key_free = ::core::mem::transmute::<
            *mut libc::c_void,
            fheap_key_free,
        >(0 as *mut libc::c_void);
        if f.is_none() {
            f = (*(*event).ev_fd_timeout_heap).key_free;
        }
        if !fn_0.is_null() {
            if f.is_some() && !((*fn_0).key).is_null() {
                f.expect("non-null function pointer")((*fn_0).key);
            }
            if (*fn_0).nofree() == 0 {
                if !((*(*event).ev_fd_timeout_heap).pool).is_null() {
                    ((*(*event).ev_fd_timeout_heap).pool_free)
                        .expect("non-null function pointer")(fn_0 as *mut libc::c_void);
                } else {
                    free(fn_0 as *mut libc::c_void);
                }
            }
        }
        (*ed).data.fd.timeout_node = 0 as *mut mln_fheap_node_t;
        if ((*ed).data.fd.timeout_handler).is_some() {
            h = (*ed).data.fd.timeout_handler;
            fd = (*ed).data.fd.fd;
            data = (*ed).data.fd.timeout_data;
            pthread_mutex_unlock(&mut (*event).fd_lock);
            h.expect("non-null function pointer")(event, fd, data);
            pthread_mutex_lock(&mut (*event).fd_lock);
        }
        (*ef).set_in_process(0 as libc::c_int as mln_u32_t);
        if (*ef).is_clear() != 0 {
            mln_event_fd_clr_set(event, (*ef).fd);
        }
        pthread_mutex_unlock(&mut (*event).fd_lock);
        if (*event).is_break() != 0 {
            return;
        }
    };
}
unsafe extern "C" fn mln_event_rbtree_fd_cmp(
    mut k1: *const libc::c_void,
    mut k2: *const libc::c_void,
) -> libc::c_int {
    let mut ed1: *mut mln_event_desc_t = k1 as *mut mln_event_desc_t;
    let mut ed2: *mut mln_event_desc_t = k2 as *mut mln_event_desc_t;
    return (*ed1).data.fd.fd - (*ed2).data.fd.fd;
}
#[inline]
unsafe extern "C" fn mln_event_fd_timeout_cmp(
    mut k1: *const libc::c_void,
    mut k2: *const libc::c_void,
) -> libc::c_int {
    let mut ed1: *mut mln_event_desc_t = k1 as *mut mln_event_desc_t;
    let mut ed2: *mut mln_event_desc_t = k2 as *mut mln_event_desc_t;
    if (*ed1).data.fd.end_us < (*ed2).data.fd.end_us {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_event_fd_timeout_copy(
    mut k1: *mut libc::c_void,
    mut k2: *mut libc::c_void,
) {
    let mut ed1: *mut mln_event_desc_t = k1 as *mut mln_event_desc_t;
    let mut ed2: *mut mln_event_desc_t = k2 as *mut mln_event_desc_t;
    (*ed1).data.fd.end_us = (*ed2).data.fd.end_us;
}
#[inline]
unsafe extern "C" fn mln_event_fheap_timer_cmp(
    mut k1: *const libc::c_void,
    mut k2: *const libc::c_void,
) -> libc::c_int {
    let mut ed1: *mut mln_event_desc_t = k1 as *mut mln_event_desc_t;
    let mut ed2: *mut mln_event_desc_t = k2 as *mut mln_event_desc_t;
    if (*ed1).data.tm.end_tm < (*ed2).data.tm.end_tm {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mln_event_fheap_timer_copy(
    mut k1: *mut libc::c_void,
    mut k2: *mut libc::c_void,
) {
    let mut ed1: *mut mln_event_desc_t = k1 as *mut mln_event_desc_t;
    let mut ed2: *mut mln_event_desc_t = k2 as *mut mln_event_desc_t;
    (*ed1).data.tm.end_tm = (*ed2).data.tm.end_tm;
}
#[inline]
unsafe extern "C" fn ev_fd_wait_chain_del(
    mut head: *mut *mut mln_event_desc_t,
    mut tail: *mut *mut mln_event_desc_t,
    mut node: *mut mln_event_desc_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_event_desc_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_event_desc_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_event_desc_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_event_desc_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn ev_fd_wait_chain_add(
    mut head: *mut *mut mln_event_desc_t,
    mut tail: *mut *mut mln_event_desc_t,
    mut node: *mut mln_event_desc_t,
) {
    (*node).next = 0 as *mut mln_event_desc_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn ev_fd_active_chain_add(
    mut head: *mut *mut mln_event_desc_t,
    mut tail: *mut *mut mln_event_desc_t,
    mut node: *mut mln_event_desc_t,
) {
    (*node).act_next = 0 as *mut mln_event_desc_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).act_next = node;
    }
    (*node).act_prev = *tail;
    *tail = node;
}
#[inline]
unsafe extern "C" fn ev_fd_active_chain_del(
    mut head: *mut *mut mln_event_desc_t,
    mut tail: *mut *mut mln_event_desc_t,
    mut node: *mut mln_event_desc_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_event_desc_t;
            *head = *tail;
        } else {
            *head = (*node).act_next;
            (**head).act_prev = 0 as *mut mln_event_desc_s;
        }
    } else if *tail == node {
        *tail = (*node).act_prev;
        (**tail).act_next = 0 as *mut mln_event_desc_s;
    } else {
        (*(*node).act_prev).act_next = (*node).act_next;
        (*(*node).act_next).act_prev = (*node).act_prev;
    }
    (*node).act_next = 0 as *mut mln_event_desc_s;
    (*node).act_prev = (*node).act_next;
}
