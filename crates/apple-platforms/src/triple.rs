//! Rust target triple → Clang triple / SDK name conversion.
//!
//! # Overview
//!
//! Apple's toolchain (Clang, `swiftc`, `xcrun`) uses a slightly different
//! target-triple vocabulary than Rust's `rustc`.  The two free functions in
//! this module bridge that gap:
//!
//! ```
//! use apple_platforms::triple;
//!
//! // Rust → Clang
//! assert_eq!(triple::to_clang("aarch64-apple-darwin"),      Some("arm64-apple-macosx"));
//! assert_eq!(triple::to_clang("aarch64-apple-ios-sim"),     Some("arm64-apple-ios-simulator"));
//! assert_eq!(triple::to_clang("x86_64-unknown-linux-gnu"),  None); // not an Apple target
//!
//! // Rust → SDK name (for `xcrun --sdk <name>`)
//! assert_eq!(triple::to_sdk("aarch64-apple-darwin"),        Some("macosx"));
//! assert_eq!(triple::to_sdk("aarch64-apple-ios-sim"),       Some("iphonesimulator"));
//! assert_eq!(triple::to_sdk("x86_64-unknown-linux-gnu"),    None);
//! ```
//!
//! Both functions return `None` for unknown triples instead of passing the
//! input through unchanged, making unrecognised targets explicit.
//!
//! # Higher-level API
//!
//! If you need more than just strings, parse the triple into an
//! [`ApplePlatform`](crate::platform::ApplePlatform) and use its methods:
//!
//! ```
//! use apple_platforms::platform::ApplePlatform;
//!
//! let platform = ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap();
//! assert_eq!(platform, ApplePlatform::IOSSimulator);
//! assert_eq!(platform.sdk(), Some("iphonesimulator"));
//! assert_eq!(platform.device(), ApplePlatform::IOS);
//! ```

use std::{borrow::Cow, fmt, str::FromStr};

// ── Architecture ──────────────────────────────────────────────────────────────

/// CPU architecture extracted from a target triple.
///
/// Parsed from the first component of a Rust or Clang target triple string
/// (`"aarch64"`, `"x86_64"`, `"armv7"`, etc.).
///
/// ```
/// use apple_platforms::triple::Architecture;
///
/// assert_eq!("aarch64".parse::<Architecture>().unwrap(), Architecture::Arm64);
/// assert_eq!("armv7k".parse::<Architecture>().unwrap(), Architecture::Arm);
/// assert!("riscv64".parse::<Architecture>().is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    /// 32-bit x86 (`x86`, `i386`, `i686`).
    X86,
    /// 64-bit x86 (`x86_64`).
    X86_64,
    /// 32-bit ARM (`arm`, `armv7`, `armv7s`, `armv7k`).
    Arm,
    /// 64-bit ARM / Apple Silicon (`aarch64`, `arm64`).
    Arm64,
    /// watchOS ILP32 64-bit-pointer ARM (`arm64_32`).
    Arm64_32,
}

impl FromStr for Architecture {
    type Err = UnknownArchitecture;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x86" | "i386" | "i686"          => Ok(Self::X86),
            "x86_64"                          => Ok(Self::X86_64),
            "arm" | "armv7" | "armv7s" | "armv7k" => Ok(Self::Arm),
            "aarch64" | "arm64"               => Ok(Self::Arm64),
            "arm64_32"                        => Ok(Self::Arm64_32),
            _ => Err(UnknownArchitecture(s.to_string())),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: Cow<'static, str> = match self {
            Self::X86      => Cow::Borrowed("x86"),
            Self::X86_64   => Cow::Borrowed("x86_64"),
            Self::Arm      => Cow::Borrowed("arm"),
            Self::Arm64    => Cow::Borrowed("arm64"),
            Self::Arm64_32 => Cow::Borrowed("arm64_32"),
        };
        write!(f, "{s}")
    }
}

/// Error returned when parsing an architecture string fails.
///
/// The inner `String` is the unrecognised input.
///
/// ```
/// use apple_platforms::triple::{Architecture, UnknownArchitecture};
///
/// let err: UnknownArchitecture = "riscv64".parse::<Architecture>().unwrap_err();
/// assert!(err.to_string().contains("riscv64"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownArchitecture(pub String);

impl fmt::Display for UnknownArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported architecture: {:?}", self.0)
    }
}

impl std::error::Error for UnknownArchitecture {}

// ── to_clang ──────────────────────────────────────────────────────────────────

/// Convert a Rust target triple to the equivalent Clang-compatible triple.
///
/// Returns `None` for triples that are not recognised Apple targets (e.g.
/// Linux triples) rather than silently returning the input unchanged.
///
/// ```
/// use apple_platforms::triple;
///
/// assert_eq!(triple::to_clang("aarch64-apple-darwin"),    Some("arm64-apple-macosx"));
/// assert_eq!(triple::to_clang("aarch64-apple-visionos"),  Some("arm64-apple-xros"));
/// assert_eq!(triple::to_clang("aarch64-apple-ios-sim"),   Some("arm64-apple-ios-simulator"));
/// assert_eq!(triple::to_clang("x86_64-unknown-linux-gnu"), None);
/// ```
pub fn to_clang(target: &str) -> Option<&'static str> {
    match target {
        // macOS
        "aarch64-apple-darwin"         => Some("arm64-apple-macosx"),
        "x86_64-apple-darwin"          => Some("x86_64-apple-macosx"),

        // iOS device
        "aarch64-apple-ios"            => Some("arm64-apple-ios"),
        "armv7-apple-ios"              => Some("armv7-apple-ios"),
        "armv7s-apple-ios"             => Some("armv7s-apple-ios"),

        // iOS simulator  (x86_64-apple-ios is historically the sim triple)
        "aarch64-apple-ios-sim"        => Some("arm64-apple-ios-simulator"),
        "x86_64-apple-ios"             => Some("x86_64-apple-ios-simulator"),

        // Mac Catalyst
        "aarch64-apple-ios-macabi"     => Some("arm64-apple-ios-macabi"),
        "x86_64-apple-ios-macabi"      => Some("x86_64-apple-ios-macabi"),

        // tvOS
        "aarch64-apple-tvos"           => Some("arm64-apple-tvos"),
        "aarch64-apple-tvos-sim"       => Some("arm64-apple-tvos-simulator"),
        "x86_64-apple-tvos"            => Some("x86_64-apple-tvos-simulator"),

        // watchOS
        "aarch64-apple-watchos"        => Some("arm64-apple-watchos"),
        "armv7k-apple-watchos"         => Some("armv7k-apple-watchos"),
        "arm64_32-apple-watchos"       => Some("arm64_32-apple-watchos"),
        "aarch64-apple-watchos-sim"    => Some("arm64-apple-watchos-simulator"),
        "x86_64-apple-watchos-sim"     => Some("x86_64-apple-watchos-simulator"),

        // visionOS (xrOS)
        "aarch64-apple-visionos"       => Some("arm64-apple-xros"),
        "aarch64-apple-visionos-sim"   => Some("arm64-apple-xros-simulator"),

        // DriverKit
        "aarch64-apple-driverkit"      => Some("arm64-apple-driverkit"),
        "x86_64-apple-driverkit"       => Some("x86_64-apple-driverkit"),

        _ => None,
    }
}

// ── to_sdk ────────────────────────────────────────────────────────────────────

/// Convert a Rust target triple to the Apple SDK name for `xcrun --sdk <name>`.
///
/// Returns `None` for unrecognised or non-Apple triples.
///
/// Note: Mac Catalyst targets use the macOS SDK (`"macosx"`) with the
/// `macabi` ABI environment.
///
/// ```
/// use apple_platforms::triple;
///
/// assert_eq!(triple::to_sdk("aarch64-apple-darwin"),    Some("macosx"));
/// assert_eq!(triple::to_sdk("aarch64-apple-ios"),       Some("iphoneos"));
/// assert_eq!(triple::to_sdk("aarch64-apple-ios-sim"),   Some("iphonesimulator"));
/// assert_eq!(triple::to_sdk("x86_64-unknown-linux-gnu"), None);
/// ```
pub fn to_sdk(target: &str) -> Option<&'static str> {
    // Delegate through ApplePlatform so the mapping lives in one place.
    crate::platform::ApplePlatform::from_rust_triple(target)
        .ok()
        .and_then(|p| p.sdk())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Architecture ─────────────────────────────────────────────────────────

    #[test]
    fn parse_architectures() {
        assert_eq!("aarch64".parse::<Architecture>().unwrap(), Architecture::Arm64);
        assert_eq!("x86_64".parse::<Architecture>().unwrap(), Architecture::X86_64);
        assert_eq!("armv7".parse::<Architecture>().unwrap(), Architecture::Arm);
        assert_eq!("arm64_32".parse::<Architecture>().unwrap(), Architecture::Arm64_32);
        let err = "riscv64".parse::<Architecture>().unwrap_err();
        assert!(err.to_string().contains("riscv64"));
    }

    #[test]
    fn architecture_display() {
        assert_eq!(format!("{}", Architecture::Arm64),   "arm64");
        assert_eq!(format!("{}", Architecture::X86_64),  "x86_64");
        assert_eq!(format!("{}", Architecture::Arm64_32), "arm64_32");
    }

    // ── to_clang ──────────────────────────────────────────────────────────────

    #[test]
    fn clang_macos() {
        assert_eq!(to_clang("aarch64-apple-darwin"), Some("arm64-apple-macosx"));
        assert_eq!(to_clang("x86_64-apple-darwin"),  Some("x86_64-apple-macosx"));
    }

    #[test]
    fn clang_ios() {
        assert_eq!(to_clang("aarch64-apple-ios"),     Some("arm64-apple-ios"));
        assert_eq!(to_clang("aarch64-apple-ios-sim"), Some("arm64-apple-ios-simulator"));
        assert_eq!(to_clang("x86_64-apple-ios"),      Some("x86_64-apple-ios-simulator"));
    }

    #[test]
    fn clang_visionos() {
        assert_eq!(to_clang("aarch64-apple-visionos"),     Some("arm64-apple-xros"));
        assert_eq!(to_clang("aarch64-apple-visionos-sim"), Some("arm64-apple-xros-simulator"));
    }

    #[test]
    fn clang_catalyst() {
        assert_eq!(to_clang("aarch64-apple-ios-macabi"), Some("arm64-apple-ios-macabi"));
    }

    #[test]
    fn clang_watchos() {
        assert_eq!(to_clang("aarch64-apple-watchos"),     Some("arm64-apple-watchos"));
        assert_eq!(to_clang("armv7k-apple-watchos"),      Some("armv7k-apple-watchos"));
        assert_eq!(to_clang("arm64_32-apple-watchos"),    Some("arm64_32-apple-watchos"));
        assert_eq!(to_clang("aarch64-apple-watchos-sim"), Some("arm64-apple-watchos-simulator"));
    }

    #[test]
    fn clang_driverkit() {
        assert_eq!(to_clang("aarch64-apple-driverkit"), Some("arm64-apple-driverkit"));
        assert_eq!(to_clang("x86_64-apple-driverkit"),  Some("x86_64-apple-driverkit"));
    }

    #[test]
    fn clang_unknown_is_none() {
        assert_eq!(to_clang("x86_64-unknown-linux-gnu"), None);
        assert_eq!(to_clang("aarch64-unknown-linux-musl"), None);
        assert_eq!(to_clang(""), None);
    }

    // ── to_sdk ────────────────────────────────────────────────────────────────

    #[test]
    fn sdk_macos() {
        assert_eq!(to_sdk("aarch64-apple-darwin"), Some("macosx"));
        assert_eq!(to_sdk("x86_64-apple-darwin"),  Some("macosx"));
    }

    #[test]
    fn sdk_ios() {
        assert_eq!(to_sdk("aarch64-apple-ios"),     Some("iphoneos"));
        assert_eq!(to_sdk("aarch64-apple-ios-sim"), Some("iphonesimulator"));
        assert_eq!(to_sdk("x86_64-apple-ios"),      Some("iphonesimulator"));
    }

    #[test]
    fn sdk_visionos() {
        assert_eq!(to_sdk("aarch64-apple-visionos"),     Some("xros"));
        assert_eq!(to_sdk("aarch64-apple-visionos-sim"), Some("xrsimulator"));
    }

    #[test]
    fn sdk_tvos() {
        assert_eq!(to_sdk("aarch64-apple-tvos"),     Some("appletvos"));
        assert_eq!(to_sdk("aarch64-apple-tvos-sim"), Some("appletvsimulator"));
    }

    #[test]
    fn sdk_watchos() {
        assert_eq!(to_sdk("aarch64-apple-watchos"),     Some("watchos"));
        assert_eq!(to_sdk("aarch64-apple-watchos-sim"), Some("watchsimulator"));
    }

    #[test]
    fn sdk_catalyst() {
        // Mac Catalyst uses the macOS SDK
        assert_eq!(to_sdk("aarch64-apple-ios-macabi"), Some("macosx"));
    }

    #[test]
    fn sdk_unknown_is_none() {
        assert_eq!(to_sdk("x86_64-unknown-linux-gnu"), None);
        assert_eq!(to_sdk(""), None);
    }
}
