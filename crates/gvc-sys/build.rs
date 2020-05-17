extern crate pkg_config;

fn main() {
    pkg_config::probe_library("libgvc").unwrap();
    // let library = pkg_config::probe_library("libgvc").unwrap();
    // let include_path = library.include_paths[0].to_str().unwrap();
    // let gvc_path: String = format!("{}/gvc.h", include_path);
    // let bindings = bindgen::Builder::default()
    //     .header(gvc_path)
    //     .blacklist_function("*l")
    //     .blacklist_function("nexttoward")
    //     .blacklist_function("nexttowardf")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file("src/bindings.rs")
    //     .expect("Couldn't write bindings!");
}
