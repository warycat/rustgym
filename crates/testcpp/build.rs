use cc;
use glob::glob;

fn main() {
    cc::Build::new()
        .compiler("clang++")
        .cpp(true)
        .flag("-std=c++11")
        .flag("-stdlib=libc++")
        .files(glob("src/*.cpp").expect("entries").filter_map(|x| x.ok()))
        .compile("libfoo.a");
}
