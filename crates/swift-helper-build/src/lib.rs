//! Build-time helper: compile and link the Swift helper dylib.
//!
//! Use in your crate's `build.rs`:
//!
//! ```ignore
//! fn main() {
//!     swift_helper_build::build_and_link();
//! }
//! ```
//!
//! This will:
//! 1. Find the `swift_helper/` directory relative to the workspace root
//! 2. Compile `libSwiftUIHelper.dylib` if any Swift source is newer than the dylib
//! 3. Emit `cargo:rustc-link-lib=dylib=SwiftUIHelper` and search/rpath directives
//!
//! After this, all `@_cdecl` symbols from the Swift helper are available as
//! normal `extern "C"` functions — no `dlopen`/`dlsym` needed.

use std::path::{Path, PathBuf};
use std::process::Command;

/// All Swift source files that make up the helper dylib.
const SWIFT_SOURCES: &[&str] = &[
    "SwiftUIHelper.swift",
    "SnapshotHelper.swift",
    "Platform.swift",
    "AppHost.swift",
    "RealityKitHelper.swift",
    "DataHelper.swift",
    "FrameworkHelpers.swift",
    "ChartsHelper.swift",
    "FoundationModelsHelper.swift",
    "FoundationBridge.swift",
    "SecurityBridge.swift",
    "CoreAnimationBridge.swift",
    "AccelerateBridge.swift",
    "CoreMediaBridge.swift",
    "AudioToolboxBridge.swift",
];

/// Compile the Swift helper (if needed) and emit linker directives.
///
/// Call this from `build.rs`. It handles everything:
/// - Finding the `swift_helper/` directory
/// - Incremental compilation (skips if dylib is up to date)
/// - Emitting `cargo:rustc-link-lib`, `cargo:rustc-link-search`, and rpath
pub fn build_and_link() {
    let helper_dir = find_helper_dir();

    let Some(dir) = helper_dir else {
        println!("cargo:warning=swift_helper/ directory not found — Swift symbols will be unresolved at link time");
        return;
    };

    let dylib = dir.join("libSwiftUIHelper.dylib");

    // Tell cargo to rerun if any Swift source changes
    for src in SWIFT_SOURCES {
        let path = dir.join(src);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Check if rebuild is needed
    if needs_rebuild(&dir, &dylib) {
        compile_helper(&dir);
    } else {
        // Ensure install name is correct even for pre-existing dylibs
        fix_install_name(&dylib);
    }

    if !dylib.exists() {
        println!("cargo:warning=libSwiftUIHelper.dylib not found after build attempt");
        return;
    }

    // Emit linker directives
    let dir_str = dir.canonicalize().unwrap_or(dir.clone());
    println!("cargo:rustc-link-search=native={}", dir_str.display());
    println!("cargo:rustc-link-lib=dylib=SwiftUIHelper");
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{}",
        dir_str.display()
    );
    // Also add Swift runtime rpath
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    // Link Swift runtime and system frameworks needed by the helper
    let swift_lib = find_swift_lib();
    if let Some(ref sl) = swift_lib {
        println!("cargo:rustc-link-search=native={sl}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{sl}");
    }
    println!("cargo:rustc-link-lib=dylib=swiftCore");
}

/// Like [`build_and_link`], but also links the specified Apple framework.
///
/// ```ignore
/// swift_helper_build::build_and_link_with_framework("AVFAudio");
/// ```
pub fn build_and_link_with_framework(framework: &str) {
    build_and_link();
    println!("cargo:rustc-link-lib=framework={framework}");
}

// ── Internals ───────────────────────────────────────────────────────────────

fn find_helper_dir() -> Option<PathBuf> {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let candidates = [
        manifest.join("../../swift_helper"),   // from crates/<name>/
        manifest.join("../swift_helper"),       // one level up
        manifest.join("swift_helper"),          // workspace root
        PathBuf::from("swift_helper"),          // cwd
    ];

    for c in &candidates {
        if c.join("SwiftUIHelper.swift").exists() {
            return Some(c.clone());
        }
    }
    None
}

fn needs_rebuild(dir: &Path, dylib: &Path) -> bool {
    if !dylib.exists() {
        return true;
    }

    let dylib_modified = std::fs::metadata(dylib)
        .and_then(|m| m.modified())
        .ok();

    SWIFT_SOURCES.iter().any(|src| {
        let src_path = dir.join(src);
        if !src_path.exists() {
            return false;
        }
        let src_modified = std::fs::metadata(&src_path)
            .and_then(|m| m.modified())
            .ok();
        match (src_modified, dylib_modified) {
            (Some(s), Some(d)) => s > d,
            _ => true,
        }
    })
}

fn compile_helper(dir: &Path) {
    let sdk = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    let Some(sdk) = sdk else {
        println!("cargo:warning=No macOS SDK found, skipping Swift helper compilation");
        return;
    };

    let macos_ver = detect_macos_version();
    let target = std::env::var("TARGET").unwrap_or_default();
    let swift_target = if target.contains("x86_64-apple-darwin") {
        format!("x86_64-apple-macosx{macos_ver}")
    } else {
        format!("arm64-apple-macosx{macos_ver}")
    };

    let source_paths: Vec<PathBuf> = SWIFT_SOURCES
        .iter()
        .map(|s| dir.join(s))
        .filter(|p| p.exists())
        .collect();

    if source_paths.is_empty() {
        println!("cargo:warning=No Swift source files found in {}", dir.display());
        return;
    }

    let output = dir.join("libSwiftUIHelper.dylib");

    println!(
        "cargo:warning=Compiling Swift helper ({} sources) → {}",
        source_paths.len(),
        output.display()
    );

    let result = Command::new("xcrun")
        .arg("swiftc")
        .arg("-emit-library")
        .args(source_paths.iter().map(|p| p.as_os_str()))
        .arg("-o")
        .arg(&output)
        .arg("-target")
        .arg(&swift_target)
        .arg("-sdk")
        .arg(&sdk)
        // Set the install name so dyld uses @rpath to find the dylib
        .arg("-Xlinker")
        .arg("-install_name")
        .arg("-Xlinker")
        .arg("@rpath/libSwiftUIHelper.dylib")
        .output();

    match result {
        Ok(out) if out.status.success() => {
            println!("cargo:warning=Swift helper compiled successfully");
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("cargo:warning=Swift compilation failed: {stderr}");
        }
        Err(e) => {
            println!("cargo:warning=Failed to run swiftc: {e}");
        }
    }

    // Also fix install name on existing dylib if it wasn't built with @rpath
    fix_install_name(&output);
}

/// Fix the install name of an existing dylib to use @rpath.
fn fix_install_name(dylib: &Path) {
    if !dylib.exists() {
        return;
    }

    // Check current install name
    let output = Command::new("otool")
        .arg("-D")
        .arg(dylib)
        .output();

    let needs_fix = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            // Second line is the install name
            stdout.lines().nth(1)
                .map(|name| !name.contains("@rpath"))
                .unwrap_or(false)
        }
        Err(_) => false,
    };

    if needs_fix {
        let _ = Command::new("install_name_tool")
            .arg("-id")
            .arg("@rpath/libSwiftUIHelper.dylib")
            .arg(dylib)
            .output();
    }
}

fn detect_macos_version() -> String {
    std::env::var("MACOS_VERSION").ok().unwrap_or_else(|| {
        Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-version"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| {
                let v = s.trim();
                if let Some(dot) = v.find('.') {
                    format!("{}.0", &v[..dot])
                } else {
                    format!("{v}.0")
                }
            })
            .unwrap_or_else(|| "15.0".to_string())
    })
}

fn find_swift_lib() -> Option<String> {
    Command::new("xcrun")
        .args(["--show-sdk-path", "--sdk", "macosx"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| {
            let sdk = s.trim();
            format!("{sdk}/usr/lib/swift")
        })
}
