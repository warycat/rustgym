extern crate pkg_config;
use cc::Build;
use glob::glob;
use std::env;

fn main() {
    let library = pkg_config::probe_library("libgvc").unwrap();
    let include_path: String = library.include_paths[0].to_str().unwrap().to_string();
    let stdlib = if "macos".to_string() == env::var("CARGO_CFG_TARGET_OS").unwrap() {
        "c++"
    } else {
        "stdc++"
    };
    Build::new()
        .compiler("clang++")
        .cpp(true)
        .include(include_path)
        .cpp_link_stdlib(stdlib)
        .files(glob("src/*.cpp").expect("entries").filter_map(|x| x.ok()))
        .compile("libfoo.a");
}
