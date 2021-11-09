extern crate bindgen;

use std::env;
use std::path::PathBuf;

const IOS_ROOT: &str = "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS15.0.sdk";

const MACOS_ROOT: &str = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";

fn get_clang_args() -> Vec<&'static str> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");
    if target_os.contains("macos") {
        return vec!["-x", "objective-c", "-fblocks", "-isysroot", MACOS_ROOT];
    }
    if target_os.contains("ios") {
        return vec!["-x", "objective-c", "-fblocks", "-isysroot", IOS_ROOT];
    }
    panic!();
}

fn main() {
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    let clang_args = get_clang_args();
    let bindings = bindgen::Builder::default()
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        .block_extern_crate(true)
        .generate_block(true)
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .blocklist_item("timezone")
        .blocklist_item("id")
        .blocklist_item("FndrOpaqueInfo")
        .blocklist_item("HFSCatalogFolder")
        .blocklist_item("HFSPlusCatalogFolder")
        .blocklist_item("HFSCatalogFile")
        .blocklist_item("HFSPlusCatalogFile")
        .no_copy("objc_object")
        .no_debug("objc_object")
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
