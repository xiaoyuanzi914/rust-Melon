use ::libc;
pub type mln_path_type_t = libc::c_uint;
pub const m_p_melang_dylib: mln_path_type_t = 7;
pub const m_p_melang_lib: mln_path_type_t = 6;
pub const m_p_null: mln_path_type_t = 5;
pub const m_p_log: mln_path_type_t = 4;
pub const m_p_pid: mln_path_type_t = 3;
pub const m_p_tmpfile: mln_path_type_t = 2;
pub const m_p_conf: mln_path_type_t = 1;
pub const m_p_install: mln_path_type_t = 0;
pub type mln_path_hook_t = Option::<unsafe extern "C" fn() -> *mut libc::c_char>;
static mut install_path: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<&[u8; 17], &mut [libc::c_char; 17]>(b"/usr/local/melon\0")
};
static mut conf_path: [libc::c_char; 33] = unsafe {
    *::core::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"/usr/local/melon/conf/melon.conf\0")
};
static mut tmpfile_path: [libc::c_char; 21] = unsafe {
    *::core::mem::transmute::<
        &[u8; 21],
        &mut [libc::c_char; 21],
    >(b"/usr/local/melon/tmp\0")
};
static mut pid_path: [libc::c_char; 32] = unsafe {
    *::core::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"/usr/local/melon/logs/melon.pid\0")
};
static mut log_path: [libc::c_char; 32] = unsafe {
    *::core::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"/usr/local/melon/logs/melon.log\0")
};
static mut null_path: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"/dev/null\0")
};
static mut melang_lib_path: [libc::c_char; 22] = unsafe {
    *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"/usr/local/lib/melang\0")
};
static mut melang_dylib_path: [libc::c_char; 30] = unsafe {
    *::core::mem::transmute::<
        &[u8; 30],
        &mut [libc::c_char; 30],
    >(b"/usr/local/lib/melang_dynamic\0")
};
static mut _install_path: mln_path_hook_t = None;
static mut _null_path: mln_path_hook_t = None;
static mut _melang_lib_path: mln_path_hook_t = None;
static mut _melang_dylib_path: mln_path_hook_t = None;
static mut _conf_path: mln_path_hook_t = None;
static mut _tmpfile_path: mln_path_hook_t = None;
static mut _pid_path: mln_path_hook_t = None;
static mut _log_path: mln_path_hook_t = None;
#[no_mangle]
pub unsafe extern "C" fn mln_path_hook_set(
    mut type_0: mln_path_type_t,
    mut hook: mln_path_hook_t,
) {
    match type_0 as libc::c_uint {
        0 => {
            _install_path = hook;
        }
        1 => {
            _conf_path = hook;
        }
        2 => {
            _tmpfile_path = hook;
        }
        3 => {
            _pid_path = hook;
        }
        4 => {
            _log_path = hook;
        }
        5 => {
            _null_path = hook;
        }
        6 => {
            _melang_lib_path = hook;
        }
        7 => {
            _melang_dylib_path = hook;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path() -> *mut libc::c_char {
    return if _install_path.is_none() {
        install_path.as_mut_ptr()
    } else {
        _install_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_conf() -> *mut libc::c_char {
    return if _conf_path.is_none() {
        conf_path.as_mut_ptr()
    } else {
        _conf_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_tmpfile() -> *mut libc::c_char {
    return if _tmpfile_path.is_none() {
        tmpfile_path.as_mut_ptr()
    } else {
        _tmpfile_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_pid() -> *mut libc::c_char {
    return if _pid_path.is_none() {
        pid_path.as_mut_ptr()
    } else {
        _pid_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_log() -> *mut libc::c_char {
    return if _log_path.is_none() {
        log_path.as_mut_ptr()
    } else {
        _log_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_null() -> *mut libc::c_char {
    return if _null_path.is_none() {
        null_path.as_mut_ptr()
    } else {
        _null_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_melang_lib() -> *mut libc::c_char {
    return if _melang_lib_path.is_none() {
        melang_lib_path.as_mut_ptr()
    } else {
        _melang_lib_path.expect("non-null function pointer")()
    };
}
#[no_mangle]
pub unsafe extern "C" fn mln_path_melang_dylib() -> *mut libc::c_char {
    return if _melang_dylib_path.is_none() {
        melang_dylib_path.as_mut_ptr()
    } else {
        _melang_dylib_path.expect("non-null function pointer")()
    };
}
