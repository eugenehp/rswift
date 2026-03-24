use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let swift_dir = manifest.join("swift");

    println!("cargo:rerun-if-changed=swift/WinitSwiftBridge.swift");
    println!("cargo:rerun-if-changed=swift/VisionOSApp.swift");

    // 1) Compile the main bridge dylib into OUT_DIR
    let bridge_src = swift_dir.join("WinitSwiftBridge.swift");
    let dylib = out_dir.join("libWinitSwift.dylib");

    if bridge_src.exists() {
        println!("cargo:warning=Compiling WinitSwiftBridge.swift...");
        let status = Command::new("swiftc")
            .args([
                "-emit-library",
                "-o", dylib.to_str().unwrap(),
                bridge_src.to_str().unwrap(),
                "-framework", "Foundation",
                "-framework", "QuartzCore",
                "-framework", "Metal",
                "-framework", "CoreGraphics",
                "-framework", "AppKit",
                "-framework", "CoreHaptics",
                "-Xlinker", "-install_name",
                "-Xlinker", "@rpath/libWinitSwift.dylib",
                "-O",
            ])
            .status()
            .expect("Failed to run swiftc");

        assert!(status.success(), "swiftc bridge compilation failed");
    }

    // 2) Compile VisionOS app shim object into OUT_DIR
    let app_src = swift_dir.join("VisionOSApp.swift");
    let app_obj = out_dir.join("VisionOSApp.o");

    if app_src.exists() {
        println!("cargo:warning=Compiling VisionOSApp.swift...");

        let mut args = vec![
            "-parse-as-library".to_string(),
            "-emit-object".to_string(),
            "-O".to_string(),
            "-o".to_string(), app_obj.to_str().unwrap().to_string(),
            app_src.to_str().unwrap().to_string(),
            "-framework".to_string(), "MetalKit".to_string(),
        ];

        let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        if os == "xros" {
            if let Some(sdk) = xros_sdk_path() {
                args.insert(0, "-sdk".to_string());
                args.insert(1, sdk);
                args.insert(2, "-target".to_string());
                args.insert(3, "arm64-apple-xros2.0".to_string());
            }
        }

        let status = Command::new("swiftc")
            .args(&args)
            .status()
            .expect("Failed to compile VisionOSApp.swift");

        if !status.success() {
            println!("cargo:warning=VisionOSApp.swift compilation failed (non-fatal)");
        } else {
            let lib_path = out_dir.join("libVisionOSApp.a");
            let ar_status = Command::new("ar")
                .args(["rcs", lib_path.to_str().unwrap(), app_obj.to_str().unwrap()])
                .status();
            if let Ok(s) = ar_status {
                if !s.success() {
                    println!("cargo:warning=ar failed to create static library");
                }
            }
        }
    }

    // 3) Link outputs from OUT_DIR
    let app_a = out_dir.join("libVisionOSApp.a");
    if app_a.exists() {
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=VisionOSApp");

        println!("cargo:rustc-link-lib=framework=SwiftUI");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=dylib=swiftCore");
        println!("cargo:rustc-link-search=native=/usr/lib/swift");
    }

    if dylib.exists() {
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=dylib=WinitSwift");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
    }

    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
}

#[allow(dead_code)]
fn is_newer(a: &Path, b: &Path) -> bool {
    let a_time = std::fs::metadata(a).and_then(|m| m.modified()).ok();
    let b_time = std::fs::metadata(b).and_then(|m| m.modified()).ok();
    match (a_time, b_time) {
        (Some(a), Some(b)) => a > b,
        _ => true,
    }
}

fn xros_sdk_path() -> Option<String> {
    let p = "/Applications/Xcode.app/Contents/Developer/Platforms/XROS.platform/Developer/SDKs/XROS.sdk";
    if Path::new(p).exists() { Some(p.to_string()) } else { None }
}
