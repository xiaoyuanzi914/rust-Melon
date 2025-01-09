#[macro_use]
extern crate c2rust_bitfields;

mod mln_conf;  // 导入 mln_conf.rs 文件

use crate::mln_conf::{mln_conf_t, mln_conf_load, mln_conf, mln_conf_domain_t, mln_conf_cmd_t, mln_conf_item_t};

fn main() {
    unsafe {
        // 加载配置
        if mln_conf_load() < 0 {
            eprintln!("Load configuration failed.");
            return;
        }

        let cf = mln_conf();
        let cd = (*cf).search(cf, "main\0".as_ptr() as *const i8);
        let cc = (*cd).search(cd, "framework\0".as_ptr() as *const i8);
        if cc.is_null() {
            eprintln!("framework not found.");
            return;
        }

        let ci = (*cc).search(cc, 1);
        match (*ci).type_ {
            CONF_BOOL => {
                if (*ci).val.b == 0 {
                    println!("framework off");
                }
            }
            CONF_STR => {
                println!("framework {}", (*ci).val.s.data);
            }
            _ => {
                eprintln!("Invalid framework value.");
            }
        }

        // 释放配置
        mln_conf_free();
    }
}
