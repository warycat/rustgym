use std::env;
use std::path::PathBuf;
use std::process::Command;

struct TargetArgs {
    file: String,
    sdk: String,
}

fn get_target_args() -> TargetArgs {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");
    if target_os.contains("macos") {
        return TargetArgs {
            file: "src/macos.m".to_string(),
            sdk: "macosx".to_string(),
        };
    }
    if target_os.contains("ios") {
        return TargetArgs {
            file: "src/ios.m".to_string(),
            sdk: "iphoneos".to_string(),
        };
    }
    panic!();
}

fn main() {
    println!("cargo:rerun-if-changed=shaders.metal");
    println!("cargo:rerun-if-changed=metaldnn.m");

    let target_args = get_target_args();
    cc::Build::new()
        .file("src/metaldnn.m")
        .file(target_args.file.as_str())
        .compile("metaldnn");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_air = out_dir.join("shaders.air");
    let out_lib = out_dir.join("shaders.metallib");

    let output = Command::new("xcrun")
        .arg("--sdk")
        .arg(target_args.sdk.as_str())
        .arg("metal")
        .args(&["-c", "src/shaders.metal"])
        .args(&["-o", &format!("{}", out_air.display())])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if !output.status.success() {
        panic!(
            r#"
stdout: {}
stderr: {}
"#,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        );
    }

    Command::new("xcrun")
        .arg("--sdk")
        .arg(target_args.sdk.as_str())
        .arg("metallib")
        .arg(out_air)
        .args(&["-o", &format!("{}", out_lib.display())])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
