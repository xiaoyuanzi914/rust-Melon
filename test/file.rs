#[macro_use]
extern crate c2rust_bitfields;

mod mln_file;
mod mln_log;

use std::ptr;
use std::fs::{OpenOptions};
use std::io::{Write};
use std::os::unix::io::{AsRawFd};

use crate::mln_file::{mln_fileset_t, mln_file_t, mln_fileset_init, mln_file_open, mln_fileset_destroy};

fn main() {
    unsafe {
        let mut fset: *mut mln_fileset_t;
        let mut file: *mut mln_file_t;

        fset = mln_fileset_init(100);
        assert!(!fset.is_null());

        // 模拟文件写操作
        let file_path = "/tmp/aaa";
        let fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path);

        let mut fd = fd.expect("Failed to open file");
        let n = fd.write_all(b"Hello").unwrap();
        assert_eq!(n, 5);
        fd.sync_all().unwrap();

        // 打开文件进行读取
        file = mln_file_open(fset, file_path);
        assert!(!file.is_null());

        // 打印文件路径和大小
        println!("filename: {} size: {}", (*file).file_path.data, (*file).size);

        mln_fileset_destroy(fset);
    }
}
