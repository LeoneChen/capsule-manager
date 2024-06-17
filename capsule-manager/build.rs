fn main() {
    if cfg!(feature = "trace_cmp") {
        let lib_dir = std::env::var("LIB_DIR").unwrap();
        println!("cargo::rustc-link-lib=static:+whole-archive=trace_cmp_rt");
        println!("cargo::rustc-link-lib=fmt");
        println!("cargo::rustc-link-search=native={}", lib_dir);
    }
}
