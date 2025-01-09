use ::libc;
#[no_mangle]
pub unsafe extern "C" fn spin_lock(mut lock: *mut libc::c_void) {
    let mut l: *mut libc::c_long = lock as *mut libc::c_long;
    while !(::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        l,
        0 as libc::c_int as libc::c_long,
        1 as libc::c_int as libc::c_long,
    ))
        .1
    {}
}
#[no_mangle]
pub unsafe extern "C" fn spin_unlock(mut lock: *mut libc::c_void) {
    let mut l: *mut libc::c_long = lock as *mut libc::c_long;
    (::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        l,
        1 as libc::c_int as libc::c_long,
        0 as libc::c_int as libc::c_long,
    ))
        .1;
}
#[no_mangle]
pub unsafe extern "C" fn spin_trylock(mut lock: *mut libc::c_void) -> libc::c_int {
    let mut l: *mut libc::c_long = lock as *mut libc::c_long;
    return !(::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        l,
        0 as libc::c_int as libc::c_long,
        1 as libc::c_int as libc::c_long,
    ))
        .1 as libc::c_int;
}
