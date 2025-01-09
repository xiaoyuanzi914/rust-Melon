#[macro_use]
extern crate c2rust_bitfields;

mod mln_lex;
mod mln_log;
mod mln_string;
mod mln_alloc_bindings;

use std::ptr;
use std::ffi::{CStr, CString};
use libc::{memset, MLN_ERR, MLN_EOF};

use crate::mln_lex::{mln_lex_t, mln_lex_init_with_hooks, mln_lex_getchar, mln_lex_putchar, mln_lex_error_set, mln_lex_result_clean, mln_lex_destroy, mln_lex_hooks_t};
use crate::mln_string::{mln_string, mln_string_free};
use crate::mln_alloc_bindings::*; // 引用 mln_alloc_bindings 中的内容
use crate::mln_log::{mln_log, debug};

// Define token types and struct
MLN_DEFINE_TOKEN_TYPE_AND_STRUCT!(static, mln_test, TEST, TEST_TK_ON, TEST_TK_OFF, TEST_TK_STRING);
MLN_DEFINE_TOKEN!(static, mln_test, TEST, (TEST_TK_ON, "TEST_TK_ON"), (TEST_TK_OFF, "TEST_TK_OFF"), (TEST_TK_STRING, "TEST_TK_STRING"));

static mut KEYWORDS: [mln_string; 3] = [
    mln_string("on"),
    mln_string("off"),
    mln_string(""),
];

unsafe extern "C" fn mln_get_char(lex: *mut mln_lex_t, c: char) -> i32 {
    if c == '\\' {
        let n = mln_lex_getchar(lex);
        if n == MLN_ERR {
            return -1;
        }
        match n {
            '"' | '\'' => {
                if mln_lex_putchar(lex, n) == MLN_ERR {
                    return -1;
                }
            }
            'n' => {
                if mln_lex_putchar(lex, '\n') == MLN_ERR {
                    return -1;
                }
            }
            't' => {
                if mln_lex_putchar(lex, '\t') == MLN_ERR {
                    return -1;
                }
            }
            'b' => {
                if mln_lex_putchar(lex, '\u{08}') == MLN_ERR {
                    return -1;
                }
            }
            'a' => {
                if mln_lex_putchar(lex, '\u{07}') == MLN_ERR {
                    return -1;
                }
            }
            'f' => {
                if mln_lex_putchar(lex, '\u{0C}') == MLN_ERR {
                    return -1;
                }
            }
            'r' => {
                if mln_lex_putchar(lex, '\r') == MLN_ERR {
                    return -1;
                }
            }
            'v' => {
                if mln_lex_putchar(lex, '\u{0B}') == MLN_ERR {
                    return -1;
                }
            }
            '\\' => {
                if mln_lex_putchar(lex, '\\') == MLN_ERR {
                    return -1;
                }
            }
            _ => {
                mln_lex_error_set(lex, MLN_LEX_EINVCHAR);
                return -1;
            }
        }
    } else {
        if mln_lex_putchar(lex, c) == MLN_ERR {
            return -1;
        }
    }
    return 0;
}

unsafe extern "C" fn mln_test_dblq_handler(lex: *mut mln_lex_t, data: *mut libc::c_void) -> *mut mln_test_struct_t {
    mln_lex_result_clean(lex);
    let mut c;
    loop {
        c = mln_lex_getchar(lex);
        if c == MLN_ERR {
            return ptr::null_mut();
        }
        if c == MLN_EOF {
            mln_lex_error_set(lex, MLN_LEX_EINVEOF);
            return ptr::null_mut();
        }
        if c == '\"' {
            break;
        }
        if mln_get_char(lex, c) < 0 {
            return ptr::null_mut();
        }
    }
    mln_test_new(lex, TEST_TK_STRING)
}

fn main() {
    let ini = mln_string("a = b; c = d");
    let mut lex: *mut mln_lex_t = ptr::null_mut();
    let mut lattr: mln_lex_attr = unsafe { std::mem::zeroed() };
    let mut ts: *mut mln_test_struct_t = ptr::null_mut();
    let mut hooks: mln_lex_hooks_t = unsafe { std::mem::zeroed() };

    unsafe {
        hooks.dblq_handler = Some(mln_test_dblq_handler);

        lattr.pool = mln_alloc_init(ptr::null_mut()); // 使用 mln_alloc_init 来初始化内存池
        lattr.keywords = KEYWORDS.as_ptr();
        lattr.hooks = &hooks;
        lattr.preprocess = 1;
        lattr.padding = 0;
        lattr.type_ = M_INPUT_T_BUF;
        lattr.data = &ini;
        lattr.env = ptr::null_mut();

        mln_lex_init_with_hooks(mln_test, lex, &lattr);
    }

    if lex.is_null() {
        eprintln!("Failed to initialize lexer");
        return;
    }

    loop {
        unsafe {
            ts = mln_test_token(lex);
        }
        if ts.is_null() || unsafe { (*ts).type_ == TEST_TK_EOF } {
            unsafe {
                mln_test_free(ts);
            }
            break;
        }
        unsafe {
            mln_log(debug, "line:{} text:[{}] type:{}\n", (*ts).line, (*ts).text, (*ts).type_);
            mln_test_free(ts);
        }
    }

    unsafe {
        mln_lex_destroy(lex);
        mln_alloc_destroy(lattr.pool); // 使用 mln_alloc_destroy 来销毁内存池
    }
}
