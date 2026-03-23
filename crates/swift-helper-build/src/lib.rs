//! Build-time helper: compile Swift bridges and link them.
//!
//! ## Per-crate compilation (preferred)
//!
//! Each crate compiles only its own Swift bridge file:
//!
//! ```ignore
//! // build.rs
//! fn main() {
//!     swift_helper_build::SwiftBridge::new("MyBridge")
//!         .file("swift/bridge.swift")
//!         .framework("AVFAudio")
//!         .compile();
//! }
//! ```
//!
//! ## Legacy monolithic compilation
//!
//! ```ignore
//! fn main() {
//!     swift_helper_build::build_and_link();
//! }
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

// ── Per-crate Swift bridge builder ──────────────────────────────────────────

/// Compile a per-crate Swift bridge into a small dylib and link it.
///
/// ```ignore
/// swift_helper_build::SwiftBridge::new("avfaudio_bridge")
///     .file("swift/bridge.swift")
///     .framework("AVFAudio")
///     .compile();
/// ```
pub struct SwiftBridge {
    name: String,
    files: Vec<PathBuf>,
    frameworks: Vec<String>,
}

impl SwiftBridge {
    /// Create a new bridge builder. `name` becomes the dylib name:
    /// `lib{name}.dylib`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            frameworks: Vec::new(),
        }
    }

    /// Add a Swift source file (relative to `CARGO_MANIFEST_DIR`).
    pub fn file(mut self, path: &str) -> Self {
        let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        self.files.push(manifest.join(path));
        self
    }

    /// Link an Apple framework (e.g. `"AVFAudio"`, `"Security"`).
    pub fn framework(mut self, name: &str) -> Self {
        self.frameworks.push(name.to_string());
        self
    }

    /// Compile the Swift bridge and emit all linker directives.
    pub fn compile(self) {
        let _manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let dylib_name = format!("lib{}.dylib", self.name);
        let dylib_path = out_dir.join(&dylib_name);

        // Tell cargo to rerun if any source changes
        for f in &self.files {
            if f.exists() {
                println!("cargo:rerun-if-changed={}", f.display());
            }
        }

        // Check if rebuild needed
        let needs_build = if dylib_path.exists() {
            let dylib_mod = std::fs::metadata(&dylib_path)
                .and_then(|m| m.modified()).ok();
            self.files.iter().any(|f| {
                let src_mod = std::fs::metadata(f)
                    .and_then(|m| m.modified()).ok();
                match (src_mod, dylib_mod) {
                    (Some(s), Some(d)) => s > d,
                    _ => true,
                }
            })
        } else {
            true
        };

        if needs_build {
            self.do_compile(&dylib_path);
        }

        if !dylib_path.exists() {
            println!("cargo:warning={dylib_name} not found after build");
            return;
        }

        // Emit linker directives
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=dylib={}", self.name);
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // Swift runtime
        if let Some(sl) = find_swift_lib() {
            println!("cargo:rustc-link-search=native={sl}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{sl}");
        }
        println!("cargo:rustc-link-lib=dylib=swiftCore");

        // Frameworks
        for fw in &self.frameworks {
            println!("cargo:rustc-link-lib=framework={fw}");
        }
    }

    fn do_compile(&self, output: &Path) {
        let sdk = match get_sdk_path() {
            Some(s) => s,
            None => {
                println!("cargo:warning=No macOS SDK found");
                return;
            }
        };

        let swift_target = get_swift_target();
        let install_name = format!("@rpath/lib{}.dylib", self.name);

        let existing: Vec<&PathBuf> = self.files.iter().filter(|f| f.exists()).collect();
        if existing.is_empty() {
            println!("cargo:warning=No Swift source files found for {}", self.name);
            return;
        }

        let mut cmd = Command::new("xcrun");
        cmd.arg("swiftc")
            .arg("-emit-library")
            .args(existing.iter().map(|p| p.as_os_str()))
            .arg("-o").arg(output)
            .arg("-target").arg(&swift_target)
            .arg("-sdk").arg(&sdk)
            .arg("-Xlinker").arg("-install_name")
            .arg("-Xlinker").arg(&install_name);

        // Add framework link flags
        for fw in &self.frameworks {
            cmd.arg("-framework").arg(fw);
        }

        match cmd.output() {
            Ok(out) if out.status.success() => {}
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                println!("cargo:warning=Swift compile failed for {}: {stderr}", self.name);
            }
            Err(e) => {
                println!("cargo:warning=Failed to run swiftc: {e}");
            }
        }
    }
}

// ── Legacy monolithic build ─────────────────────────────────────────────────

/// All Swift source files that make up the monolithic helper dylib.
/// Used by crates that haven't migrated to per-crate bridges yet.
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
];

/// Legacy: compile the monolithic Swift helper and link it.
///
/// Prefer [`SwiftBridge`] for new crates.
pub fn build_and_link() {
    let helper_dir = find_helper_dir();

    let Some(dir) = helper_dir else {
        println!("cargo:warning=swift_helper/ directory not found");
        return;
    };

    let dylib = dir.join("libSwiftUIHelper.dylib");

    for src in SWIFT_SOURCES {
        let path = dir.join(src);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    if needs_rebuild_legacy(&dir, &dylib) {
        compile_legacy(&dir);
    } else {
        fix_install_name(&dylib, "libSwiftUIHelper.dylib");
    }

    if !dylib.exists() {
        println!("cargo:warning=libSwiftUIHelper.dylib not found");
        return;
    }

    let dir_str = dir.canonicalize().unwrap_or(dir.clone());
    println!("cargo:rustc-link-search=native={}", dir_str.display());
    println!("cargo:rustc-link-lib=dylib=SwiftUIHelper");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dir_str.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Some(sl) = find_swift_lib() {
        println!("cargo:rustc-link-search=native={sl}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{sl}");
    }
    println!("cargo:rustc-link-lib=dylib=swiftCore");
}

// ── Shared utilities ────────────────────────────────────────────────────────

fn find_helper_dir() -> Option<PathBuf> {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    for c in [
        manifest.join("../../swift_helper"),
        manifest.join("../swift_helper"),
        manifest.join("swift_helper"),
    ] {
        if c.join("SwiftUIHelper.swift").exists() {
            return Some(c);
        }
    }
    None
}

fn needs_rebuild_legacy(dir: &Path, dylib: &Path) -> bool {
    if !dylib.exists() { return true; }
    let dylib_mod = std::fs::metadata(dylib).and_then(|m| m.modified()).ok();
    SWIFT_SOURCES.iter().any(|src| {
        let p = dir.join(src);
        if !p.exists() { return false; }
        let s = std::fs::metadata(&p).and_then(|m| m.modified()).ok();
        matches!((s, dylib_mod), (Some(s), Some(d)) if s > d)
    })
}

fn compile_legacy(dir: &Path) {
    let Some(sdk) = get_sdk_path() else {
        println!("cargo:warning=No macOS SDK found");
        return;
    };

    let swift_target = get_swift_target();
    let sources: Vec<PathBuf> = SWIFT_SOURCES.iter()
        .map(|s| dir.join(s)).filter(|p| p.exists()).collect();

    if sources.is_empty() { return; }
    let output = dir.join("libSwiftUIHelper.dylib");

    let result = Command::new("xcrun")
        .arg("swiftc").arg("-emit-library")
        .args(sources.iter().map(|p| p.as_os_str()))
        .arg("-o").arg(&output)
        .arg("-target").arg(&swift_target)
        .arg("-sdk").arg(&sdk)
        .arg("-Xlinker").arg("-install_name")
        .arg("-Xlinker").arg("@rpath/libSwiftUIHelper.dylib")
        .output();

    match result {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            println!("cargo:warning=Swift compile failed: {}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => println!("cargo:warning=swiftc failed: {e}"),
    }
}

fn fix_install_name(dylib: &Path, name: &str) {
    if !dylib.exists() { return; }
    if let Ok(out) = Command::new("otool").arg("-D").arg(dylib).output() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            if !line.contains("@rpath") {
                let _ = Command::new("install_name_tool")
                    .arg("-id").arg(format!("@rpath/{name}"))
                    .arg(dylib).output();
            }
        }
    }
}

fn get_sdk_path() -> Option<String> {
    Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn get_swift_target() -> String {
    let macos_ver = std::env::var("MACOS_VERSION").ok().unwrap_or_else(|| {
        Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-version"])
            .output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| {
                let v = s.trim();
                v.find('.').map(|d| format!("{}.0", &v[..d])).unwrap_or(format!("{v}.0"))
            })
            .unwrap_or("15.0".into())
    });
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("x86_64-apple-darwin") {
        format!("x86_64-apple-macosx{macos_ver}")
    } else {
        format!("arm64-apple-macosx{macos_ver}")
    }
}

fn find_swift_lib() -> Option<String> {
    get_sdk_path().map(|sdk| format!("{sdk}/usr/lib/swift"))
}
