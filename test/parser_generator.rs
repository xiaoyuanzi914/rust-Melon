#[macro_use]
extern crate c2rust_bitfields;

mod mln_log;
mod mln_lex;
mod mln_alloc_bindings;
mod mln_parser_generator;

use std::ptr;
use std::mem;
use std::assert;

use crate::mln_lex::{mln_lex_init_with_hooks, mln_lex_destroy, mln_lex_t, mln_lex_attr};
use crate::mln_alloc_bindings::{mln_alloc_init, mln_alloc_destroy, mln_alloc_t};
use crate::mln_parser_generator::{MLN_DECLARE_PARSER_GENERATOR, MLN_DEFINE_PARSER_GENERATOR, test_parser_generate, test_parse, test_pg_data_free};

MLN_DECLARE_PARSER_GENERATOR!(static, test, TEST);
MLN_DEFINE_PARSER_GENERATOR!(static, test, TEST);

static PROD_TBL: &[(&str, Option<fn() -> ()>)] = &[
    ("start: stm TEST_TK_EOF", None),
    ("stm: exp TEST_TK_SEMIC stm", None),
    ("stm: ", None),
    ("exp: TEST_TK_ID addsub", None),
    ("exp: TEST_TK_DEC addsub", None),
    ("addsub: TEST_TK_PLUS exp", None),
    ("addsub: TEST_TK_SUB exp", None),
    ("addsub: ", None),
];

fn main() {
    let mut lex: Option<mln_lex_t> = None;
    let mut lattr = mln_lex_attr {
        pool: std::ptr::null_mut(),
        keywords: std::ptr::null_mut(),
        hooks: std::ptr::null_mut(),
        preprocess: 1,
        type_: M_INPUT_T_BUF,
        data: &"a + 1;\nb + a;\nc - b;\n".to_string(),
        env: std::ptr::null_mut(),
    };

    let pool = unsafe { mln_alloc_init(std::ptr::null_mut()) };
    assert!(pool != std::ptr::null_mut(), "Memory allocation failed");

    lattr.pool = pool;

    let hooks: mln_lex_hooks_t = unsafe { std::mem::zeroed() };
    lattr.hooks = &hooks;

    unsafe {
        mln_lex_init_with_hooks("test", lex, &lattr);
    }

    assert!(lex.is_some(), "Lex initialization failed");

    let ptr = unsafe {
        test_parser_generate(
            PROD_TBL.as_ptr() as *const mln_production_t,
            PROD_TBL.len() as u64,
            std::ptr::null_mut(),
        )
    };

    assert!(ptr != std::ptr::null_mut(), "Parser generation failed");

    let pattr = mln_parse_attr {
        pool,
        prod_tbl: PROD_TBL.as_ptr(),
        lex: lex.unwrap(),
        pg_data: ptr,
        udata: std::ptr::null_mut(),
    };

    let ast = unsafe { test_parse(&pattr) };
    assert!(ast.is_null(), "AST generation failed: should be NULL");

    unsafe {
        mln_lex_destroy(lex.unwrap());
        test_pg_data_free(ptr);
        mln_alloc_destroy(pool);
    }
}
