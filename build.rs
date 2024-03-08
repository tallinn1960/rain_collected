#![allow(missing_docs)]

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/trap.cpp")
        .flag_if_supported("-std=c++20")
        .compile("libtrap.a");
    println!("cargo:rerun-if-changed=src/trap.cpp");
}
