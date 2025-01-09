use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
    fn pthread_setconcurrency(__level: libc::c_int) -> libc::c_int;
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn pthread_atfork(
        __prepare: Option::<unsafe extern "C" fn() -> ()>,
        __parent: Option::<unsafe extern "C" fn() -> ()>,
        __child: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
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
pub union pthread_condattr_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_CREATE_DETACHED: C2RustUnnamed_0 = 1;
pub const PTHREAD_CREATE_JOINABLE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut libc::c_void; 4],
}
pub type mln_u32_t = libc::c_uint;
pub type mln_u64_t = libc::c_ulong;
pub type mln_size_t = size_t;
pub type mln_sptr_t = libc::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_thread_pool_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub attr: pthread_attr_t,
    pub res_chain_head: *mut mln_thread_pool_resource_t,
    pub res_chain_tail: *mut mln_thread_pool_resource_t,
    pub child_head: *mut mln_thread_pool_member_t,
    pub child_tail: *mut mln_thread_pool_member_t,
    pub max: mln_u32_t,
    pub idle: mln_u32_t,
    pub counter: mln_u32_t,
    #[bitfield(name = "quit", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "mln_u32_t", bits = "1..=31")]
    pub quit_padding: [u8; 4],
    pub cond_timeout: mln_u64_t,
    pub n_res: mln_size_t,
    pub process_handler: mln_thread_process,
    pub free_handler: mln_thread_data_free,
}
pub type mln_thread_data_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type mln_thread_process = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type mln_thread_pool_member_t = mln_thread_pool_member_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_thread_pool_member_s {
    pub data: *mut libc::c_void,
    pub pool: *mut mln_thread_pool_t,
    #[bitfield(name = "idle", ty = "mln_u32_t", bits = "0..=0")]
    #[bitfield(name = "locked", ty = "mln_u32_t", bits = "1..=1")]
    #[bitfield(name = "forked", ty = "mln_u32_t", bits = "2..=2")]
    #[bitfield(name = "child", ty = "mln_u32_t", bits = "3..=3")]
    pub idle_locked_forked_child: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub prev: *mut mln_thread_pool_member_s,
    pub next: *mut mln_thread_pool_member_s,
}
pub type mln_thread_pool_t = mln_thread_pool_s;
pub type mln_thread_pool_resource_t = mln_thread_pool_resource_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_pool_resource_s {
    pub data: *mut libc::c_void,
    pub next: *mut mln_thread_pool_resource_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_pool_attr {
    pub main_data: *mut libc::c_void,
    pub child_process_handler: mln_thread_process,
    pub main_process_handler: mln_thread_process,
    pub free_handler: mln_thread_data_free,
    pub cond_timeout: mln_u64_t,
    pub max: mln_u32_t,
    pub concurrency: mln_u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_thread_pool_info {
    pub max_num: mln_u32_t,
    pub idle_num: mln_u32_t,
    pub cur_num: mln_u32_t,
    pub res_num: mln_size_t,
}
#[no_mangle]
#[thread_local]
pub static mut m_thread_pool_self: *mut mln_thread_pool_member_t = 0
    as *const mln_thread_pool_member_t as *mut mln_thread_pool_member_t;
#[inline]
unsafe extern "C" fn mln_thread_pool_member_new(
    mut tpool: *mut mln_thread_pool_t,
    mut child: mln_u32_t,
) -> *mut mln_thread_pool_member_t {
    let mut tpm: *mut mln_thread_pool_member_t = 0 as *mut mln_thread_pool_member_t;
    tpm = malloc(::core::mem::size_of::<mln_thread_pool_member_t>() as libc::c_ulong)
        as *mut mln_thread_pool_member_t;
    if tpm.is_null() {
        return 0 as *mut mln_thread_pool_member_t;
    }
    (*tpm).data = 0 as *mut libc::c_void;
    (*tpm).pool = tpool;
    (*tpm).set_idle(1 as libc::c_int as mln_u32_t);
    (*tpm).set_locked(0 as libc::c_int as mln_u32_t);
    (*tpm).set_forked(0 as libc::c_int as mln_u32_t);
    (*tpm).set_child(child);
    (*tpm).next = 0 as *mut mln_thread_pool_member_s;
    (*tpm).prev = (*tpm).next;
    return tpm;
}
#[inline]
unsafe extern "C" fn mln_thread_pool_member_free(
    mut member: *mut mln_thread_pool_member_t,
) {
    if member.is_null() {
        return;
    }
    if !((*member).data).is_null() && ((*(*member).pool).free_handler).is_some() {
        ((*(*member).pool).free_handler)
            .expect("non-null function pointer")((*member).data);
    }
    free(member as *mut libc::c_void);
}
unsafe extern "C" fn mln_thread_pool_member_join(
    mut tp: *mut mln_thread_pool_t,
    mut child: mln_u32_t,
) -> *mut mln_thread_pool_member_t {
    let mut tpm: *mut mln_thread_pool_member_t = 0 as *mut mln_thread_pool_member_t;
    tpm = mln_thread_pool_member_new(tp, child);
    if tpm.is_null() {
        return 0 as *mut mln_thread_pool_member_t;
    }
    (*tp).counter = ((*tp).counter).wrapping_add(1);
    (*tp).counter;
    (*tp).idle = ((*tp).idle).wrapping_add(1);
    (*tp).idle;
    mln_child_chain_add(&mut (*tp).child_head, &mut (*tp).child_tail, tpm);
    return tpm;
}
unsafe extern "C" fn mln_thread_pool_member_exit(mut arg: *mut libc::c_void) {
    let mut tpm: *mut mln_thread_pool_member_t = arg as *mut mln_thread_pool_member_t;
    let mut forked: mln_u32_t = (*tpm).forked();
    let mut child: mln_u32_t = (*tpm).child();
    let mut tpool: *mut mln_thread_pool_t = (*tpm).pool;
    if (*tpm).locked() as libc::c_int == 0 as libc::c_int {
        (*tpm).set_locked(1 as libc::c_int as mln_u32_t);
        pthread_mutex_lock(&mut (*tpool).mutex);
    }
    mln_child_chain_del(&mut (*tpool).child_head, &mut (*tpool).child_tail, tpm);
    (*tpool).counter = ((*tpool).counter).wrapping_sub(1);
    (*tpool).counter;
    if (*tpm).idle() != 0 {
        (*tpool).idle = ((*tpool).idle).wrapping_sub(1);
        (*tpool).idle;
    }
    pthread_mutex_unlock(&mut (*tpool).mutex);
    mln_thread_pool_member_free(tpm);
    if forked != 0 && child != 0 {
        mln_thread_pool_free(tpool);
    }
}
unsafe extern "C" fn mln_thread_pool_prepare() {
    if m_thread_pool_self.is_null() {
        return;
    }
    if (*m_thread_pool_self).locked() == 0 {
        pthread_mutex_lock(&mut (*(*m_thread_pool_self).pool).mutex);
    }
}
unsafe extern "C" fn mln_thread_pool_parent() {
    if m_thread_pool_self.is_null() {
        return;
    }
    if (*m_thread_pool_self).locked() == 0 {
        pthread_mutex_unlock(&mut (*(*m_thread_pool_self).pool).mutex);
    }
}
unsafe extern "C" fn mln_thread_pool_child() {
    if m_thread_pool_self.is_null() {
        return;
    }
    let mut tpool: *mut mln_thread_pool_t = (*m_thread_pool_self).pool;
    if (*m_thread_pool_self).locked() == 0 {
        pthread_mutex_unlock(&mut (*tpool).mutex);
    }
    (*m_thread_pool_self).set_forked(1 as libc::c_int as mln_u32_t);
    let mut tpm: *mut mln_thread_pool_member_t = (*tpool).child_head;
    let mut fr: *mut mln_thread_pool_member_t = 0 as *mut mln_thread_pool_member_t;
    while !tpm.is_null() {
        fr = tpm;
        tpm = (*tpm).next;
        if fr == m_thread_pool_self {
            continue;
        }
        mln_thread_pool_member_exit(fr as *mut libc::c_void);
    }
}
unsafe extern "C" fn mln_thread_pool_new(
    mut tpattr: *mut mln_thread_pool_attr,
    mut err: *mut libc::c_int,
) -> *mut mln_thread_pool_t {
    let mut rc: libc::c_int = 0;
    let mut tp: *mut mln_thread_pool_t = 0 as *mut mln_thread_pool_t;
    tp = malloc(::core::mem::size_of::<mln_thread_pool_t>() as libc::c_ulong)
        as *mut mln_thread_pool_t;
    if tp.is_null() {
        *err = 12 as libc::c_int;
        return 0 as *mut mln_thread_pool_t;
    }
    rc = pthread_mutex_init(&mut (*tp).mutex, 0 as *const pthread_mutexattr_t);
    if rc != 0 as libc::c_int {
        free(tp as *mut libc::c_void);
        *err = rc;
        return 0 as *mut mln_thread_pool_t;
    }
    rc = pthread_cond_init(&mut (*tp).cond, 0 as *const pthread_condattr_t);
    if rc != 0 as libc::c_int {
        pthread_mutex_destroy(&mut (*tp).mutex);
        free(tp as *mut libc::c_void);
        *err = rc;
        return 0 as *mut mln_thread_pool_t;
    }
    rc = pthread_attr_init(&mut (*tp).attr);
    if rc != 0 as libc::c_int {
        pthread_cond_destroy(&mut (*tp).cond);
        pthread_mutex_destroy(&mut (*tp).mutex);
        free(tp as *mut libc::c_void);
        *err = rc;
        return 0 as *mut mln_thread_pool_t;
    }
    rc = pthread_attr_setdetachstate(
        &mut (*tp).attr,
        PTHREAD_CREATE_DETACHED as libc::c_int,
    );
    if rc != 0 as libc::c_int {
        pthread_attr_destroy(&mut (*tp).attr);
        pthread_cond_destroy(&mut (*tp).cond);
        pthread_mutex_destroy(&mut (*tp).mutex);
        free(tp as *mut libc::c_void);
        *err = rc;
        return 0 as *mut mln_thread_pool_t;
    }
    (*tp).res_chain_tail = 0 as *mut mln_thread_pool_resource_t;
    (*tp).res_chain_head = (*tp).res_chain_tail;
    (*tp).child_tail = 0 as *mut mln_thread_pool_member_t;
    (*tp).child_head = (*tp).child_tail;
    (*tp).counter = 0 as libc::c_int as mln_u32_t;
    (*tp).idle = (*tp).counter;
    (*tp).set_quit(0 as libc::c_int as mln_u32_t);
    (*tp).cond_timeout = (*tpattr).cond_timeout;
    (*tp).n_res = 0 as libc::c_int as mln_size_t;
    (*tp).process_handler = (*tpattr).child_process_handler;
    (*tp).free_handler = (*tpattr).free_handler;
    (*tp).max = (*tpattr).max;
    if (*tpattr).concurrency != 0 {
        pthread_setconcurrency((*tpattr).concurrency as libc::c_int);
    }
    rc = pthread_atfork(
        Some(mln_thread_pool_prepare as unsafe extern "C" fn() -> ()),
        Some(mln_thread_pool_parent as unsafe extern "C" fn() -> ()),
        Some(mln_thread_pool_child as unsafe extern "C" fn() -> ()),
    );
    if rc != 0 as libc::c_int {
        pthread_attr_destroy(&mut (*tp).attr);
        pthread_cond_destroy(&mut (*tp).cond);
        pthread_mutex_destroy(&mut (*tp).mutex);
        free(tp as *mut libc::c_void);
        *err = rc;
        return 0 as *mut mln_thread_pool_t;
    }
    m_thread_pool_self = mln_thread_pool_member_join(tp, 0 as libc::c_int as mln_u32_t);
    if m_thread_pool_self.is_null() {
        pthread_attr_destroy(&mut (*tp).attr);
        pthread_cond_destroy(&mut (*tp).cond);
        pthread_mutex_destroy(&mut (*tp).mutex);
        free(tp as *mut libc::c_void);
        *err = 12 as libc::c_int;
        return 0 as *mut mln_thread_pool_t;
    }
    return tp;
}
unsafe extern "C" fn mln_thread_pool_free(mut tp: *mut mln_thread_pool_t) {
    if tp.is_null() {
        return;
    }
    m_thread_pool_self = 0 as *mut mln_thread_pool_member_t;
    let mut tpr: *mut mln_thread_pool_resource_t = 0 as *mut mln_thread_pool_resource_t;
    loop {
        tpr = (*tp).res_chain_head;
        if tpr.is_null() {
            break;
        }
        (*tp).res_chain_head = (*(*tp).res_chain_head).next;
        if ((*tp).free_handler).is_some() {
            ((*tp).free_handler).expect("non-null function pointer")((*tpr).data);
        }
        free(tpr as *mut libc::c_void);
    }
    pthread_mutex_destroy(&mut (*tp).mutex);
    pthread_cond_destroy(&mut (*tp).cond);
    pthread_attr_destroy(&mut (*tp).attr);
    free(tp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_pool_resource_add(
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut tpr: *mut mln_thread_pool_resource_t = 0 as *mut mln_thread_pool_resource_t;
    let mut tpool: *mut mln_thread_pool_t = (*m_thread_pool_self).pool;
    tpr = malloc(::core::mem::size_of::<mln_thread_pool_resource_t>() as libc::c_ulong)
        as *mut mln_thread_pool_resource_t;
    if tpr.is_null() {
        return 12 as libc::c_int;
    }
    (*tpr).data = data;
    (*tpr).next = 0 as *mut mln_thread_pool_resource_s;
    (*m_thread_pool_self).set_locked(1 as libc::c_int as mln_u32_t);
    pthread_mutex_lock(&mut (*tpool).mutex);
    if ((*tpool).res_chain_head).is_null() {
        (*tpool).res_chain_tail = tpr;
        (*tpool).res_chain_head = (*tpool).res_chain_tail;
    } else {
        (*(*tpool).res_chain_tail).next = tpr;
        (*tpool).res_chain_tail = tpr;
    }
    (*tpool).n_res = ((*tpool).n_res).wrapping_add(1);
    (*tpool).n_res;
    if (*tpool).idle <= 1 as libc::c_int as libc::c_uint
        && (*tpool).counter
            < ((*tpool).max).wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        let mut rc: libc::c_int = 0;
        let mut threadid: pthread_t = 0;
        let mut tpm: *mut mln_thread_pool_member_t = 0 as *mut mln_thread_pool_member_t;
        tpm = mln_thread_pool_member_join(tpool, 1 as libc::c_int as mln_u32_t);
        if tpm.is_null() {
            pthread_mutex_unlock(&mut (*tpool).mutex);
            (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
            return 12 as libc::c_int;
        }
        rc = pthread_create(
            &mut threadid,
            &mut (*tpool).attr,
            Some(
                child_thread_launcher
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tpm as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            pthread_mutex_unlock(&mut (*tpool).mutex);
            (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
            return rc;
        }
    }
    pthread_cond_signal(&mut (*tpool).cond);
    pthread_mutex_unlock(&mut (*tpool).mutex);
    (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mln_thread_pool_resource_remove() -> *mut libc::c_void {
    let mut tpr: *mut mln_thread_pool_resource_t = 0 as *mut mln_thread_pool_resource_t;
    let mut tpool: *mut mln_thread_pool_t = (*m_thread_pool_self).pool;
    loop {
        tpr = (*tpool).res_chain_head;
        if tpr.is_null() {
            return 0 as *mut libc::c_void;
        }
        (*tpool).res_chain_head = (*(*tpool).res_chain_head).next;
        if ((*tpool).res_chain_head).is_null() {
            (*tpool).res_chain_tail = 0 as *mut mln_thread_pool_resource_t;
        }
        (*tpool).n_res = ((*tpool).n_res).wrapping_sub(1);
        (*tpool).n_res;
        (*m_thread_pool_self).data = (*tpr).data;
        free(tpr as *mut libc::c_void);
        if !((*m_thread_pool_self).data).is_null() {
            break;
        }
    }
    (*m_thread_pool_self).set_idle(0 as libc::c_int as mln_u32_t);
    (*tpool).idle = ((*tpool).idle).wrapping_sub(1);
    (*tpool).idle;
    return (*m_thread_pool_self).data;
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_pool_run(
    mut tpattr: *mut mln_thread_pool_attr,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut tpool: *mut mln_thread_pool_t = 0 as *mut mln_thread_pool_t;
    if ((*tpattr).child_process_handler).is_none()
        || ((*tpattr).main_process_handler).is_none()
    {
        return 22 as libc::c_int;
    }
    tpool = mln_thread_pool_new(tpattr, &mut rc);
    if tpool.is_null() {
        return rc;
    }
    rc = ((*tpattr).main_process_handler)
        .expect("non-null function pointer")((*tpattr).main_data);
    (*tpool).set_quit(1 as libc::c_int as mln_u32_t);
    loop {
        (*m_thread_pool_self).set_locked(1 as libc::c_int as mln_u32_t);
        pthread_mutex_lock(&mut (*tpool).mutex);
        if (*tpool).counter <= 1 as libc::c_int as libc::c_uint {
            pthread_mutex_unlock(&mut (*tpool).mutex);
            (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
            break;
        } else {
            pthread_cond_broadcast(&mut (*tpool).cond);
            pthread_mutex_unlock(&mut (*tpool).mutex);
            (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
            usleep(50000 as libc::c_int as __useconds_t);
        }
    }
    mln_thread_pool_member_exit(m_thread_pool_self as *mut libc::c_void);
    m_thread_pool_self = 0 as *mut mln_thread_pool_member_t;
    mln_thread_pool_free(tpool);
    return rc;
}
unsafe extern "C" fn child_thread_launcher(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut rc: mln_sptr_t = 0 as libc::c_int as mln_sptr_t;
    let mut forked: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = Some(
        mln_thread_pool_member_exit as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    let mut __cancel_arg: *mut libc::c_void = arg;
    let mut __not_first_call: libc::c_int = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0 as libc::c_int,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut timeout: mln_u32_t = 0 as libc::c_int as mln_u32_t;
    let mut tpm: *mut mln_thread_pool_member_t = arg as *mut mln_thread_pool_member_t;
    let mut tpool: *mut mln_thread_pool_t = (*tpm).pool;
    m_thread_pool_self = tpm;
    's_54: loop {
        (*tpm).set_locked(1 as libc::c_int as mln_u32_t);
        pthread_mutex_lock(&mut (*tpool).mutex);
        loop {
            if (*tpm).idle() as libc::c_int <= 0 as libc::c_int {
                (*tpm).set_idle(1 as libc::c_int as mln_u32_t);
                (*tpool).idle = ((*tpool).idle).wrapping_add(1);
                (*tpool).idle;
            }
            if (*tpool).quit() != 0 {
                break 's_54;
            }
            if !(mln_thread_pool_resource_remove()).is_null() {
                break;
            }
            if timeout != 0 {
                break 's_54;
            }
            ts
                .tv_sec = (time(0 as *mut time_t) as libc::c_ulong)
                .wrapping_add(
                    ((*tpool).cond_timeout)
                        .wrapping_div(1000 as libc::c_int as libc::c_ulong),
                ) as __time_t;
            ts
                .tv_nsec = ((*tpool).cond_timeout)
                .wrapping_rem(1000 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
                as __syscall_slong_t;
            rc = pthread_cond_timedwait(&mut (*tpool).cond, &mut (*tpool).mutex, &mut ts)
                as mln_sptr_t;
            if rc != 0 as libc::c_int as libc::c_long {
                if !(rc == 110 as libc::c_int as libc::c_long) {
                    break 's_54;
                }
                timeout = 1 as libc::c_int as mln_u32_t;
            } else {
                timeout = 0 as libc::c_int as mln_u32_t;
            }
        }
        pthread_mutex_unlock(&mut (*tpool).mutex);
        (*tpm).set_locked(0 as libc::c_int as mln_u32_t);
        timeout = 0 as libc::c_int as mln_u32_t;
        rc = ((*tpool).process_handler).expect("non-null function pointer")((*tpm).data)
            as mln_sptr_t;
        (*tpm).data = 0 as *mut libc::c_void;
    }
    forked = (*m_thread_pool_self).forked();
    __pthread_unregister_cancel(&mut __cancel_buf);
    __cancel_routine.expect("non-null function pointer")(__cancel_arg);
    m_thread_pool_self = 0 as *mut mln_thread_pool_member_t;
    if forked != 0 {
        exit(rc as libc::c_int);
    }
    return rc as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_quit() {
    let mut tpool: *mut mln_thread_pool_t = (*m_thread_pool_self).pool;
    (*m_thread_pool_self).set_locked(1 as libc::c_int as mln_u32_t);
    pthread_mutex_lock(&mut (*tpool).mutex);
    (*tpool).set_quit(1 as libc::c_int as mln_u32_t);
    pthread_mutex_unlock(&mut (*tpool).mutex);
    (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mln_thread_resource_info(mut info: *mut mln_thread_pool_info) {
    if info.is_null() {
        return;
    }
    let mut tpool: *mut mln_thread_pool_t = (*m_thread_pool_self).pool;
    (*m_thread_pool_self).set_locked(1 as libc::c_int as mln_u32_t);
    pthread_mutex_lock(&mut (*tpool).mutex);
    (*info).max_num = (*tpool).max;
    (*info).idle_num = (*tpool).idle;
    (*info).cur_num = (*tpool).counter;
    (*info).res_num = (*tpool).n_res;
    pthread_mutex_unlock(&mut (*tpool).mutex);
    (*m_thread_pool_self).set_locked(0 as libc::c_int as mln_u32_t);
}
#[inline]
unsafe extern "C" fn mln_child_chain_del(
    mut head: *mut *mut mln_thread_pool_member_t,
    mut tail: *mut *mut mln_thread_pool_member_t,
    mut node: *mut mln_thread_pool_member_t,
) {
    if *head == node {
        if *tail == node {
            *tail = 0 as *mut mln_thread_pool_member_t;
            *head = *tail;
        } else {
            *head = (*node).next;
            (**head).prev = 0 as *mut mln_thread_pool_member_s;
        }
    } else if *tail == node {
        *tail = (*node).prev;
        (**tail).next = 0 as *mut mln_thread_pool_member_s;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut mln_thread_pool_member_s;
    (*node).prev = (*node).next;
}
#[inline]
unsafe extern "C" fn mln_child_chain_add(
    mut head: *mut *mut mln_thread_pool_member_t,
    mut tail: *mut *mut mln_thread_pool_member_t,
    mut node: *mut mln_thread_pool_member_t,
) {
    (*node).next = 0 as *mut mln_thread_pool_member_s;
    if (*head).is_null() {
        *head = node;
    } else {
        (**tail).next = node;
    }
    (*node).prev = *tail;
    *tail = node;
}
