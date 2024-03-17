#![allow(missing_docs)]
use cmake::Config;

fn main() {
    // Note that a static c++ library won't work here
    // as cargo bench doesn't link with the c++ standard library.
    let dst = Config::new("trap_cpp")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .generator("Ninja Multi-Config")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=trap_cpp");

    // But we can do static linking with swift.
    let dst = Config::new("trap_swift")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .generator("Ninja Multi-Config")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=trap_swift");
}
