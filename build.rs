// build.rs
use bindgen;

fn main() {
    // 告诉 Cargo 重新编译时，依赖于 "include/mln_alloc.h" 文件
    println!("cargo:rerun-if-changed=include/mln_alloc.h");

    // 生成 FFI 绑定
    let bindings = bindgen::Builder::default()
        .header("include/mln_alloc.h")
        .generate()
        .expect("Unable to generate bindings");

    // 将生成的绑定代码写入到 src 目录下
    bindings
        .write_to_file("mln_alloc_bindings.rs")
        .expect("Couldn't write bindings!");
}
