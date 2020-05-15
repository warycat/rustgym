use cc::Build;
use glob::glob;

fn main() {
    println!("cargo:rustc-link-lib=gvc");
    println!("cargo:rustc-link-lib=cgraph");
    println!("cargo:rustc-link-lib=cdt");
    Build::new()
        .compiler("clang++")
        .cpp(true)
        .flag("-I/usr/local/include/graphviz")
        .flag("-std=c++11")
        .flag("-stdlib=libc++")
        .files(glob("src/*.cpp").expect("entries").filter_map(|x| x.ok()))
        .compile("libfoo.a");
}
