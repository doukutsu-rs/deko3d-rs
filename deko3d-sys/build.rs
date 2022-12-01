extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search={}/libnx/lib", env::var("DEVKITPRO").unwrap());

    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=static=deko3dd");
    } else {
        println!("cargo:rustc-link-lib=static=deko3d");
    }
    println!("cargo:rustc-link-lib=nx");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("--sysroot={}/devkitA64/aarch64-none-elf", env::var("DEVKITPRO").unwrap()))
        .clang_arg(format!("-I{}/libnx/include", env::var("DEVKITPRO").unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
