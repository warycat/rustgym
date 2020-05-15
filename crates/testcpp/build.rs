extern crate pkg_config;
use cc::Build;
use glob::glob;

fn main() {
    let library = pkg_config::probe_library("libgvc").unwrap();
    let include_path: String = library.include_paths[0].to_str().unwrap().to_string();
    Build::new()
        .compiler("clang++")
        .cpp(true)
        .include(include_path)
        .cpp_link_stdlib("c++")
        .files(glob("src/*.cpp").expect("entries").filter_map(|x| x.ok()))
        .compile("libfoo.a");
}
