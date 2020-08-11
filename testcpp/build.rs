extern crate pkg_config;
use cc::Build;
use glob::glob;
use std::env;

fn main() {
    let stdlib = if "macos".to_string() == env::var("CARGO_CFG_TARGET_OS").unwrap() {
        "c++"
    } else {
        "stdc++"
    };
    Build::new()
        .cpp(true)
        .cpp_link_stdlib(stdlib)
        .files(glob("src/*.cpp").expect("entries").filter_map(|x| x.ok()))
        .compile("libfoo.a");
}
