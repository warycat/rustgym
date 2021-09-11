use bindgen::builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    Config::new("../XNNPACK")
        .always_configure(false)
        .very_verbose(true)
        .build();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_path.join("lib").display());
    println!("cargo:rustc-link-lib=static=XNNPACK");
    println!("cargo:rustc-link-lib=static=pthreadpool");
    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-lib=static=clog");
    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(&format!("-I/{}", out_path.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
