use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type mln_uauto_t = libc::c_ulong;
pub type queue_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type queue_iterate_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_queue_t {
    pub head: *mut *mut libc::c_void,
    pub tail: *mut *mut libc::c_void,
    pub queue: *mut *mut libc::c_void,
    pub qlen: mln_uauto_t,
    pub nr_element: mln_uauto_t,
    pub free_handler: queue_free,
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_init(
    mut qlen: mln_uauto_t,
    mut free_handler: queue_free,
) -> *mut mln_queue_t {
    let mut q: *mut mln_queue_t = malloc(
        ::core::mem::size_of::<mln_queue_t>() as libc::c_ulong,
    ) as *mut mln_queue_t;
    if q.is_null() {
        return 0 as *mut mln_queue_t;
    }
    (*q).qlen = qlen;
    (*q).nr_element = 0 as libc::c_int as mln_uauto_t;
    (*q).free_handler = free_handler;
    (*q)
        .queue = calloc(
        (*q).qlen,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    if ((*q).queue).is_null() {
        free(q as *mut libc::c_void);
        return 0 as *mut mln_queue_t;
    }
    (*q).tail = (*q).queue;
    (*q).head = (*q).tail;
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_destroy(mut q: *mut mln_queue_t) {
    if q.is_null() {
        return;
    }
    if ((*q).free_handler).is_some() {
        while (*q).nr_element != 0 {
            ((*q).free_handler).expect("non-null function pointer")(*(*q).head);
            (*q).head = ((*q).head).offset(1);
            if (*q).head >= ((*q).queue).offset((*q).qlen as isize) {
                (*q).head = (*q).queue;
            }
            (*q).nr_element = ((*q).nr_element).wrapping_sub(1);
            (*q).nr_element;
        }
    }
    if !((*q).queue).is_null() {
        free((*q).queue as *mut libc::c_void);
    }
    free(q as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_append(
    mut q: *mut mln_queue_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if (*q).nr_element >= (*q).qlen {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*q).tail;
    (*q).tail = ((*q).tail).offset(1);
    *fresh0 = data;
    if (*q).tail == ((*q).queue).offset((*q).qlen as isize) {
        (*q).tail = (*q).queue;
    }
    (*q).nr_element = ((*q).nr_element).wrapping_add(1);
    (*q).nr_element;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_get(mut q: *mut mln_queue_t) -> *mut libc::c_void {
    if (*q).nr_element == 0 {
        return 0 as *mut libc::c_void;
    }
    return *(*q).head;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_remove(mut q: *mut mln_queue_t) {
    if (*q).nr_element == 0 {
        return;
    }
    (*q).head = ((*q).head).offset(1);
    if (*q).head >= ((*q).queue).offset((*q).qlen as isize) {
        (*q).head = (*q).queue;
    }
    (*q).nr_element = ((*q).nr_element).wrapping_sub(1);
    (*q).nr_element;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_search(
    mut q: *mut mln_queue_t,
    mut index: mln_uauto_t,
) -> *mut libc::c_void {
    if index >= (*q).nr_element {
        return 0 as *mut libc::c_void;
    }
    let mut ptr: *mut *mut libc::c_void = ((*q).head).offset(index as isize);
    if ptr >= ((*q).queue).offset((*q).qlen as isize) {
        ptr = ((*q).queue)
            .offset(
                ptr.offset_from(((*q).queue).offset((*q).qlen as isize)) as libc::c_long
                    as isize,
            );
    }
    return *ptr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_iterate(
    mut q: *mut mln_queue_t,
    mut handler: queue_iterate_handler,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut scan: *mut *mut libc::c_void = (*q).head;
    let mut i: mln_uauto_t = 0 as libc::c_int as mln_uauto_t;
    while i < (*q).nr_element {
        if handler.is_some() {
            if handler.expect("non-null function pointer")(*scan, udata)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        }
        scan = scan.offset(1);
        if scan >= ((*q).queue).offset((*q).qlen as isize) {
            scan = (*q).queue;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mln_queue_free_index(
    mut q: *mut mln_queue_t,
    mut index: mln_uauto_t,
) {
    if index >= (*q).nr_element {
        return;
    }
    let mut pos: *mut *mut libc::c_void = ((*q).head).offset(index as isize);
    if pos >= ((*q).queue).offset((*q).qlen as isize) {
        pos = ((*q).queue)
            .offset(
                pos.offset_from(((*q).queue).offset((*q).qlen as isize)) as libc::c_long
                    as isize,
            );
    }
    let mut save: *mut libc::c_void = *pos;
    let mut next: *mut *mut libc::c_void = pos;
    let mut i: mln_uauto_t = 0;
    let mut end: mln_uauto_t = ((*q).nr_element).wrapping_sub(index);
    i = 0 as libc::c_int as mln_uauto_t;
    while i < end {
        next = next.offset(1);
        if next >= ((*q).queue).offset((*q).qlen as isize) {
            next = (*q).queue;
        }
        let fresh1 = pos;
        pos = pos.offset(1);
        *fresh1 = *next;
        if pos >= ((*q).queue).offset((*q).qlen as isize) {
            pos = (*q).queue;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*q).tail = pos;
    (*q).nr_element = ((*q).nr_element).wrapping_sub(1);
    (*q).nr_element;
    if ((*q).free_handler).is_some() {
        ((*q).free_handler).expect("non-null function pointer")(save);
    }
}
