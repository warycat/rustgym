use bindgen::builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    Config::new("../XNNPACK").always_configure(false).build();
    let bindings = builder()
        .header("wrapper.h")
        .size_t_is_usize(true)
        .clang_arg(&format!("-I/{}", out_path.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=XNNPACK");
    println!("cargo:rustc-link-lib=static=pthreadpool");
    println!("cargo:rustc-link-lib=static=clog");
    println!("cargo:rustc-link-lib=static=cpuinfo");
}
