use bindgen::builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst: PathBuf = Config::new("../XNNPACK")
        .build_target("XNNPACK")
        .very_verbose(true)
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=XNNPACK");
    println!("cargo:rustc-link-search=native={}/build/pthreadpool", dst.display());
    println!("cargo:rustc-link-lib=static=pthreadpool");
    println!("cargo:rustc-link-search=native={}/build/cpuinfo", dst.display());
    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-search=native={}/build/clog", dst.display());
    println!("cargo:rustc-link-lib=static=clog");
    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(&format!("-I/{}/include", dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
