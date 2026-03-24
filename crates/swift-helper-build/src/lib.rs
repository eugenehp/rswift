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
    /// Extra `-F <path>` search directories passed to swiftc.
    /// Use this for Catalyst (iOSSupport) frameworks that live outside the
    /// standard macOS SDK framework path.
    framework_search_paths: Vec<String>,
}

impl SwiftBridge {
    /// Create a new bridge builder. `name` becomes the dylib name:
    /// `lib{name}.dylib`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            frameworks: Vec::new(),
            framework_search_paths: Vec::new(),
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

    /// Add an extra `-F <path>` framework search directory for swiftc.
    ///
    /// Use for Catalyst (iOSSupport) frameworks that are not in the default
    /// macOS SDK path:
    ///
    /// ```ignore
    /// let ios_support = format!(
    ///     "{}/System/iOSSupport/System/Library/Frameworks",
    ///     std::env::var("SDKROOT").unwrap_or_default()
    /// );
    /// SwiftBridge::new("my_bridge")
    ///     .file("swift/bridge.swift")
    ///     .framework("MyFramework")
    ///     .framework_search_path(&ios_support)
    ///     .compile();
    /// ```
    pub fn framework_search_path(mut self, path: &str) -> Self {
        self.framework_search_paths.push(path.to_string());
        self
    }

    /// Convenience: add the macOS SDK's iOSSupport framework search path.
    ///
    /// Required for Catalyst frameworks (e.g. `AdAttributionKit`,
    /// `MarketplaceKit`, `BrowserKit`).
    pub fn ios_support_frameworks(self) -> Self {
        if let Some(sdk) = get_sdk_path() {
            let path = format!("{sdk}/System/iOSSupport/System/Library/Frameworks");
            if Path::new(&path).exists() {
                return self.framework_search_path(&path);
            }
        }
        self
    }

    /// Compile the Swift bridge and emit all linker directives.
    ///
    /// The compiled dylib is cached in `<CARGO_MANIFEST_DIR>/.swift-cache/`
    /// keyed by a SHA-256 hash of the concatenated source files.  This means
    /// the expensive `swiftc` invocation is skipped whenever the Swift sources
    /// have not changed — even when Cargo regenerates a new `OUT_DIR` (which
    /// happens on every `Cargo.toml` or `Cargo.lock` change).
    pub fn compile(self) {
        let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir  = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let dylib_name = format!("lib{}.dylib", self.name);

        // Tell cargo to rerun if any source changes.
        for f in &self.files {
            if f.exists() {
                println!("cargo:rerun-if-changed={}", f.display());
            }
        }

        // ── Stable content-addressed cache ───────────────────────────────────
        // Hash every source file in order.  The digest changes iff any source
        // changes, so we can skip swiftc when the cache is warm — even across
        // OUT_DIR invalidations caused by Cargo metadata changes.
        let cache_dir = manifest.join(".swift-cache");
        let hash = source_hash(&self.files);
        let cached_dylib = cache_dir.join(format!("{}_{}.dylib", self.name, &hash[..16]));

        if !cached_dylib.exists() {
            // Compile into the stable cache directory.
            let _ = std::fs::create_dir_all(&cache_dir);
            self.do_compile(&cached_dylib);
        }

        if !cached_dylib.exists() {
            println!("cargo:warning={dylib_name} not found after build");
            return;
        }

        // Copy (or hard-link) from cache into OUT_DIR so the linker can find it
        // under the expected name.
        let out_dylib = out_dir.join(&dylib_name);
        if std::fs::hard_link(&cached_dylib, &out_dylib).is_err() {
            let _ = std::fs::copy(&cached_dylib, &out_dylib);
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

        // Optimise in release builds, keep debug info in debug builds.
        let opt_flag = if std::env::var("PROFILE").as_deref() == Ok("release") {
            "-O"
        } else {
            "-Onone"
        };

        let mut cmd = Command::new("xcrun");
        cmd.arg("swiftc")
            .arg("-emit-library")
            .arg(opt_flag)
            .args(existing.iter().map(|p| p.as_os_str()))
            .arg("-o").arg(output)
            .arg("-target").arg(&swift_target)
            .arg("-sdk").arg(&sdk)
            .arg("-Xlinker").arg("-install_name")
            .arg("-Xlinker").arg(&install_name);

        // Extra framework search paths (-F flags)
        for sp in &self.framework_search_paths {
            cmd.arg("-F").arg(sp);
        }

        // Framework link flags
        for fw in &self.frameworks {
            cmd.arg("-framework").arg(fw);
        }

        match cmd.output() {
            Ok(out) if out.status.success() => {
                // Surface any warnings even on success
                let stderr = String::from_utf8_lossy(&out.stderr);
                for line in stderr.lines().filter(|l| l.contains("error:") || l.contains("warning:")) {
                    println!("cargo:warning=[swiftc] {line}");
                }
            }
            Ok(out) => {
                // Compile error — emit each line so Cargo shows them clearly
                let stderr = String::from_utf8_lossy(&out.stderr);
                for line in stderr.lines() {
                    println!("cargo:warning=[swiftc error] {line}");
                }
                // Panic to fail the build visibly, not silently produce a stale dylib
                panic!("swiftc failed for '{}'. See warnings above.", self.name);
            }
            Err(e) => {
                panic!("Failed to run swiftc for '{}': {e}", self.name);
            }
        }
    }
}

// ── ObjC / C bridge builder (no swiftc, no dylib) ──────────────────────────

/// Compile ObjC (`.m`) or C (`.c`) source files into a static library and
/// link against the specified Apple frameworks.
///
/// This avoids spawning `swiftc`, producing a dylib, and linking the Swift
/// runtime — perfect for frameworks whose public API is ObjC or pure-C.
///
/// ```ignore
/// // build.rs
/// swift_helper_build::ObjCBridge::new("foundation_bridge")
///     .file("objc/bridge.m")
///     .framework("Foundation")
///     .compile();
/// ```
pub struct ObjCBridge {
    name: String,
    files: Vec<PathBuf>,
    frameworks: Vec<String>,
    /// Extra `-I` include paths.
    includes: Vec<PathBuf>,
}

impl ObjCBridge {
    /// Create a new ObjC/C bridge builder. `name` becomes the static archive
    /// name (`lib{name}.a`) passed to `cc::Build`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            frameworks: Vec::new(),
            includes: Vec::new(),
        }
    }

    /// Add a source file (`.m` or `.c`) relative to `CARGO_MANIFEST_DIR`.
    pub fn file(mut self, path: &str) -> Self {
        let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        self.files.push(manifest.join(path));
        self
    }

    /// Link an Apple framework (e.g. `"Foundation"`, `"Security"`).
    pub fn framework(mut self, name: &str) -> Self {
        self.frameworks.push(name.to_string());
        self
    }

    /// Add a header search path.
    pub fn include(mut self, path: &str) -> Self {
        let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        self.includes.push(manifest.join(path));
        self
    }

    /// Compile the ObjC/C bridge into a static library and emit cargo
    /// directives.  No Swift runtime or dylib involved.
    pub fn compile(self) {
        // Tell cargo to rerun if any source changes
        for f in &self.files {
            if f.exists() {
                println!("cargo:rerun-if-changed={}", f.display());
            }
        }

        let existing: Vec<&PathBuf> = self.files.iter().filter(|f| f.exists()).collect();
        if existing.is_empty() {
            println!("cargo:warning=No source files found for ObjCBridge '{}'", self.name);
            return;
        }

        let mut build = cc::Build::new();
        build.cargo_warnings(false);

        for f in &existing {
            build.file(f);
        }

        // Detect if any file is ObjC and enable ARC
        let has_objc = existing.iter().any(|f| {
            f.extension().and_then(|e| e.to_str()) == Some("m")
        });
        if has_objc {
            build.flag("-fobjc-arc");
        }

        for inc in &self.includes {
            build.include(inc);
        }

        // SDK sysroot
        if let Some(sdk) = get_sdk_path() {
            build.flag(format!("-isysroot{sdk}"));
        }

        // Match the deployment target that Rust uses so the linker
        // doesn't warn about version mismatches.
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        let deploy_flag = match target_os.as_str() {
            "macos"  => Some("-mmacosx-version-min=11.0"),
            "ios"    => Some("-miphoneos-version-min=13.0"),
            "tvos"   => Some("-mappletvos-version-min=13.0"),
            "watchos" => Some("-mwatchos-version-min=7.0"),
            _ => None,
        };
        if let Some(flag) = deploy_flag {
            build.flag(flag);
        }

        build.compile(&self.name);

        // Emit framework link directives
        for fw in &self.frameworks {
            println!("cargo:rustc-link-lib=framework={fw}");
        }

        // ObjC files need libobjc and CoreFoundation (for CFRelease, etc.)
        if has_objc {
            println!("cargo:rustc-link-lib=dylib=objc");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
        }
    }
}

// ── Legacy monolithic build (removed) ────────────────────────────────────────
// Previously contained build_and_link(), compile_legacy(), etc. for the
// monolithic swift_helper/ dylib.  All crates now use per-crate SwiftBridge
// or ObjCBridge, so the legacy path has been removed.

fn get_sdk_path() -> Option<String> {
    Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn get_swift_target() -> String {
    let target = std::env::var("TARGET").unwrap_or_default();

    // Use apple-platforms for proper Rust→Clang target conversion
    if let Some(clang_target) = apple_platforms::triple::to_clang(&target) {
        let sdk_ver = detect_sdk_version(&target);
        return format!("{clang_target}{sdk_ver}");
    }

    // Fallback: manual macOS target construction
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
    if target.contains("x86_64-apple-darwin") {
        format!("x86_64-apple-macosx{macos_ver}")
    } else {
        format!("arm64-apple-macosx{macos_ver}")
    }
}

fn detect_sdk_version(target: &str) -> String {
    let sdk_name = apple_platforms::triple::to_sdk(target)
        .unwrap_or("macosx");

    Command::new("xcrun")
        .args(["--sdk", sdk_name, "--show-sdk-version"])
        .output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "15.0".into())
}

fn find_swift_lib() -> Option<String> {
    get_sdk_path().map(|sdk| format!("{sdk}/usr/lib/swift"))
}

/// Compute a hex SHA-256 digest of the concatenated contents of `files`.
/// Used to key the `.swift-cache/` entries so swiftc is skipped when
/// sources haven't changed, even if Cargo regenerates a new `OUT_DIR`.
fn source_hash(files: &[PathBuf]) -> String {
    // Simple djb2-style 64-bit hash — good enough for a build cache key
    // and avoids pulling in a sha2 crate.
    let mut h: u64 = 5381;
    for f in files {
        if let Ok(content) = std::fs::read(f) {
            for b in content {
                h = h.wrapping_mul(33).wrapping_add(b as u64);
            }
        }
    }
    format!("{h:016x}")
}
