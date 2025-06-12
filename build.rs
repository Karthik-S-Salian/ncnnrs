use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let ncnn_dir = env::var("NCNN_DIR")
        .expect("ERROR: please set NCNN_DIR (e.g. export NCNN_DIR=/path/to/ncnn)");
    let ncnn_path = PathBuf::from(&ncnn_dir);

    let include_path = ncnn_path.join("include/ncnn");
    let lib_path = ncnn_path.join("lib");

    let c_api_path = include_path.join("c_api.h");
    if !c_api_path.exists() {
        panic!(
            "ERROR: Missing c_api.h at {}. Make sure NCNN_DIR is set to the root of your NCNN build.",
            c_api_path.display()
        );
    }

    if !lib_path.exists() {
        panic!(
            "ERROR: Missing lib/ directory at {}. Make sure NCNN was compiled.",
            lib_path.display()
        );
    }

    // Link NCNN and dependencies
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=ncnn");
    println!("cargo:rustc-link-lib=static=SPIRV");
    println!("cargo:rustc-link-lib=static=OSDependent");
    println!("cargo:rustc-link-lib=static=MachineIndependent");
    println!("cargo:rustc-link-lib=static=glslang");
    println!("cargo:rustc-link-lib=gomp");
    // TODO: may need to change this to c++ on clang setups
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Rebuild triggers
    println!("cargo:rerun-if-env-changed=NCNN_DIR");

    // Bindgen
    let bindings = bindgen::Builder::default()
        .header(c_api_path.to_str().unwrap())
        .header(include_path.join("gpu.h").to_str().unwrap())
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .wrap_unsafe_ops(true) // To avoid rust 2024 unsafe warnings
        .generate()
        .expect("Unable to generate bindings with bindgen");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings to OUT_DIR");
}
