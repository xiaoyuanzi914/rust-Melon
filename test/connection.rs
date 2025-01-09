#[macro_use]
extern crate c2rust_bitfields;

mod mln_connection;  // 导入 mln_connection.rs 文件
mod mln_alloc_bindings; // 导入 mln_alloc_bindings.rs 文件

use crate::mln_connection::{mln_tcp_conn_t, mln_tcp_conn_init, mln_tcp_conn_pool_get, mln_tcp_conn_append, mln_tcp_conn_send, mln_tcp_conn_recv, mln_tcp_conn_head, mln_tcp_conn_destroy};
use crate::mln_alloc_bindings::{mln_alloc_t, mln_alloc_m, mln_alloc_init, mln_chain_t, mln_chain_new, mln_buf_t, mln_buf_new};

use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::os::unix::io::RawFd;
use std::io::{self, Write};
use std::cmp::min;

const M_C_SEND: u32 = 1;
const M_C_FINISH: u32 = 2;
const M_C_TYPE_MEMORY: u32 = 3;
const M_C_RECV: u32 = 4;
const M_C_ERROR: u32 = 5;

fn main() {
    unsafe {
        // 创建 Unix 套接字
        let mut fds: [RawFd; 2] = [-1, -1];
        assert!(socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) == 0);

        // 初始化连接
        let mut conn1: mln_tcp_conn_t = mem::zeroed();
        let mut conn2: mln_tcp_conn_t = mem::zeroed();
        assert!(mln_tcp_conn_init(&mut conn1, fds[0]) == 0);
        assert!(mln_tcp_conn_init(&mut conn2, fds[1]) == 0);

        // 获取连接池
        let pool: *mut mln_alloc_t = mln_tcp_conn_pool_get(&conn1);

        // 分配内存
        let buf = mln_alloc_m(pool, 10) as *mut u8;
        assert!(!buf.is_null());
        for i in 0..9 {
            *buf.offset(i as isize) = b'a';
        }
        *buf.offset(9) = 0; // Null terminate

        // 创建链表和缓冲区
        let c = mln_chain_new(pool);
        let b = mln_buf_new(pool);
        (*c).buf = b;
        (*b).left_pos = (*b).pos = (*b).start = buf;
        (*b).last = (*b).end = buf.offset(10);
        (*b).in_memory = 1;
        (*b).last_buf = 1;
        (*b).last_in_chain = 1;

        // 将缓冲区附加到连接
        mln_tcp_conn_append(&mut conn1, c, M_C_SEND);

        // 发送数据
        assert!(mln_tcp_conn_send(&mut conn1) == M_C_FINISH);

        // 接收数据
        assert!(mln_tcp_conn_recv(&mut conn2, M_C_TYPE_MEMORY) != M_C_ERROR);

        // 验证接收数据是否正确
        let received_buf = (*mln_tcp_conn_head(&mut conn2, M_C_RECV)).buf.start;
        assert!(memcmp(received_buf, buf, 10) == 0);

        // 销毁连接并关闭文件描述符
        mln_tcp_conn_destroy(&mut conn1);
        mln_tcp_conn_destroy(&mut conn2);
        libc::close(fds[0]);
        libc::close(fds[1]);
    }
}

extern "C" {
    fn socketpair(domain: libc::c_int, socket_type: libc::c_int, protocol: libc::c_int, sv: *mut RawFd) -> libc::c_int;
    fn memcmp(s1: *const libc::c_void, s2: *const libc::c_void, n: usize) -> libc::c_int;
}
