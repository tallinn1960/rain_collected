use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/root.zig");
    let compiler = "zig";

    Command::new(compiler)
        .args(&["build", "--release=fast"])
        .output()
        .expect("Failed to compile Zig lib");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("zig-out/lib").display());
    println!("cargo:rustc-link-lib=static=trap_zig");
}