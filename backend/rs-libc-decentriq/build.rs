use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let p_c = ["src", "c"].iter().collect::<PathBuf>();
    let p_s = ["src", "asm", &arch].iter().collect::<PathBuf>();

    let extension_filter = |ext| {
        move |f: Result<DirEntry, _>| {
            let f = f.unwrap();
            if f.file_type().unwrap().is_file() {
                let path = f.path();
                if path.extension().and_then(OsStr::to_str) == Some(ext) {
                    return Some(path);
                }
            }
            None
        }
    };

    let i_c = read_dir(p_c).unwrap().filter_map(extension_filter("c"));
    let i_s = read_dir(p_s).unwrap().filter_map(extension_filter("S"));

    let mut build = cc::Build::new();
    for path in i_c.chain(i_s) {
        build.file(path);
    }

    let name = if env::var("TARGET").unwrap() == "x86_64-fortanix-unknown-sgx" {
        "libc-decentriq.a"
    } else {
        "librsc-decentriq.a"
    };

    build
        .define(
            "weak_alias(old,new)",
            Some("extern __typeof(old) new __attribute__((alias(#old)))"),
            )
        .flag("-U_FORTIFY_SOURCE")
        .define("_FORTIFY_SOURCE", Some("0"))
        .define("__NO_STRING_INLINES", None)
        .define("__NO_MATH_INLINES", None)
        .flag("-ffreestanding")
        .warnings(false)
        .compile(name);
    println!("cargo:lib_dir={}", out_dir);
}
