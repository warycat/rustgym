use cc;

fn main() {
    cc::Build::new()
        .compiler("clang++")
        .cpp(true)
        .flag("-std=c++11")
        .flag("-stdlib=libc++")
        .file("src/_278_first_bad_version.cpp")
        .compile("libfoo.a");
}
