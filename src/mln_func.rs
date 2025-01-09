use ::libc;
pub type mln_func_entry_cb_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
        ...
    ) -> libc::c_int,
>;
pub type mln_func_exit_cb_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
        ...
    ) -> (),
>;
#[no_mangle]
pub static mut mln_func_entry: mln_func_entry_cb_t = None;
#[no_mangle]
pub static mut mln_func_exit: mln_func_exit_cb_t = None;
#[no_mangle]
pub unsafe extern "C" fn mln_func_entry_callback_set(mut cb: mln_func_entry_cb_t) {
    mln_func_entry = cb;
}
#[no_mangle]
pub unsafe extern "C" fn mln_func_entry_callback_get() -> mln_func_entry_cb_t {
    return mln_func_entry;
}
#[no_mangle]
pub unsafe extern "C" fn mln_func_exit_callback_set(mut cb: mln_func_exit_cb_t) {
    mln_func_exit = cb;
}
#[no_mangle]
pub unsafe extern "C" fn mln_func_exit_callback_get() -> mln_func_exit_cb_t {
    return mln_func_exit;
}
