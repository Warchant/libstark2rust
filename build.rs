use std::env;
use std::path::Path;

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();

    cxx_build::bridge("src/glue.rs")
        .include(Path::new(&root).join("src"))
        .include(Path::new(&root).join("libSTARK/libstark/src"))
        .include(Path::new(&root).join("libSTARK/algebra/FFT/src"))
        .include(Path::new(&root).join("libSTARK/algebra/algebralib/headers"))
        .flag_if_supported("-w")
        .flag_if_supported("-std=c++11")
        .file("src/glue.cpp")
        .compile("glue");

    let libdir = Path::new(&root).join("libSTARK/bin");

    // add algebralib
    println!("cargo:rustc-link-search=native={}/algebralib", libdir.display());
    println!("cargo:rustc-link-lib=static=algebralib");
    println!("cargo:rerun-if-changed={}/algebralib/algebralib.a", libdir.display());

    // add fft
    println!("cargo:rustc-link-search=native={}/fft", libdir.display());
    println!("cargo:rustc-link-lib=static=FFT");
    println!("cargo:rerun-if-changed={}/fft/libFFT.a", libdir.display());

    // add libstark
    println!("cargo:rustc-link-search=native={}/libstark", libdir.display());
    println!("cargo:rustc-link-lib=static=stark");
    println!("cargo:rerun-if-changed={}/libstark/libstark.a", libdir.display());

    println!("cargo:rustc-link-arg=-fopenmp");

    // rerun if changed
    println!("cargo:rerun-if-changed={}/src/glue.cpp", root);
    println!("cargo:rerun-if-changed={}/src/glue.hpp", root);
    println!("cargo:rerun-if-changed={}/build.rs", root);
}
