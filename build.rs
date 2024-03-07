#![allow(missing_docs)]

use cmake::Config;

fn main() {
    let dst = Config::new("trap").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=trap");
}
