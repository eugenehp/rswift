//! Rust target triple → Clang triple / SDK name conversion.

use anyhow::{anyhow, Result};
use std::{borrow::Cow, fmt, str::FromStr};

/// CPU architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    X86,
    X86_64,
    Arm,
    Arm64,
    Arm64_32,
}

impl FromStr for Architecture {
    type Err = anyhow::Error;

    fn from_str(target: &str) -> Result<Self> {
        match target.to_lowercase().as_str() {
            "x86" | "i386" | "i686" => Ok(Self::X86),
            "x86_64" => Ok(Self::X86_64),
            "arm" | "armv7" | "armv7s" | "armv7k" => Ok(Self::Arm),
            "aarch64" | "arm64" => Ok(Self::Arm64),
            "arm64_32" => Ok(Self::Arm64_32),
            _ => Err(anyhow!("Unsupported architecture: {target}")),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::X86 => Cow::from("x86"),
            Self::X86_64 => Cow::from("x86_64"),
            Self::Arm => Cow::from("arm"),
            Self::Arm64 => Cow::from("arm64"),
            Self::Arm64_32 => Cow::from("arm64_32"),
        };
        write!(f, "{s}")
    }
}

/// Rust target triple → Clang target triple conversion.
pub struct Triple;

impl Triple {
    /// Convert a Rust target triple to a Clang-compatible target triple.
    ///
    /// Returns the input unchanged if no mapping is found.
    ///
    /// ```
    /// use apple_platforms::triple::Triple;
    /// assert_eq!(Triple::target_to_clang_target("aarch64-apple-darwin"), "arm64-apple-macosx");
    /// assert_eq!(Triple::target_to_clang_target("aarch64-apple-visionos"), "arm64-apple-xros");
    /// assert_eq!(Triple::target_to_clang_target("aarch64-apple-ios-sim"), "arm64-apple-ios-simulator");
    /// ```
    pub fn target_to_clang_target(target: &str) -> &str {
        match target {
            // macOS
            "aarch64-apple-darwin"          => "arm64-apple-macosx",
            "x86_64-apple-darwin"           => "x86_64-apple-macosx",

            // iOS
            "aarch64-apple-ios"             => "arm64-apple-ios",
            "aarch64-apple-ios-sim"         => "arm64-apple-ios-simulator",
            "x86_64-apple-ios"              => "x86_64-apple-ios-simulator",
            "armv7-apple-ios"               => "armv7-apple-ios",
            "armv7s-apple-ios"              => "armv7s-apple-ios",

            // Mac Catalyst
            "aarch64-apple-ios-macabi"      => "arm64-apple-ios-macabi",
            "x86_64-apple-ios-macabi"       => "x86_64-apple-ios-macabi",

            // tvOS
            "aarch64-apple-tvos"            => "arm64-apple-tvos",
            "aarch64-apple-tvos-sim"        => "arm64-apple-tvos-simulator",
            "x86_64-apple-tvos"             => "x86_64-apple-tvos-simulator",

            // watchOS
            "aarch64-apple-watchos"         => "arm64-apple-watchos",
            "aarch64-apple-watchos-sim"     => "arm64-apple-watchos-simulator",
            "x86_64-apple-watchos-sim"      => "x86_64-apple-watchos-simulator",
            "armv7k-apple-watchos"          => "armv7k-apple-watchos",
            "arm64_32-apple-watchos"        => "arm64_32-apple-watchos",

            // visionOS (xrOS)
            "aarch64-apple-visionos"        => "arm64-apple-xros",
            "aarch64-apple-visionos-sim"    => "arm64-apple-xros-simulator",

            // DriverKit
            "aarch64-apple-driverkit"       => "arm64-apple-driverkit",
            "x86_64-apple-driverkit"        => "x86_64-apple-driverkit",

            // Unknown — return as-is
            _ => target,
        }
    }
}

/// Rust target triple → Apple SDK name conversion.
pub struct SDK;

impl SDK {
    /// Convert a Rust target triple to the corresponding Apple SDK name.
    ///
    /// The SDK name can be passed to `xcrun --sdk <name>`.
    ///
    /// ```
    /// use apple_platforms::triple::SDK;
    /// assert_eq!(SDK::target_to_sdk("aarch64-apple-darwin").unwrap(), "macosx");
    /// assert_eq!(SDK::target_to_sdk("aarch64-apple-ios").unwrap(), "iphoneos");
    /// assert_eq!(SDK::target_to_sdk("aarch64-apple-ios-sim").unwrap(), "iphonesimulator");
    /// ```
    pub fn target_to_sdk(target: &str) -> Result<&str> {
        match target {
            // macOS
            "aarch64-apple-darwin" | "x86_64-apple-darwin" => Ok("macosx"),

            // iOS device
            "aarch64-apple-ios" | "armv7-apple-ios" | "armv7s-apple-ios" => Ok("iphoneos"),

            // iOS simulator
            "aarch64-apple-ios-sim" | "x86_64-apple-ios" | "i386-apple-ios" => {
                Ok("iphonesimulator")
            }

            // Mac Catalyst (uses macOS SDK)
            "aarch64-apple-ios-macabi" | "x86_64-apple-ios-macabi" => Ok("macosx"),

            // tvOS
            "aarch64-apple-tvos" => Ok("appletvos"),
            "aarch64-apple-tvos-sim" | "x86_64-apple-tvos" => Ok("appletvsimulator"),

            // watchOS
            "aarch64-apple-watchos" | "armv7k-apple-watchos" | "arm64_32-apple-watchos" => {
                Ok("watchos")
            }
            "aarch64-apple-watchos-sim" | "x86_64-apple-watchos-sim" => Ok("watchsimulator"),

            // visionOS
            "aarch64-apple-visionos" => Ok("xros"),
            "aarch64-apple-visionos-sim" => Ok("xrsimulator"),

            // DriverKit
            "aarch64-apple-driverkit" | "x86_64-apple-driverkit" => Ok("driverkit"),

            _ => Err(anyhow!("Unsupported target for SDK: {target}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Architecture ────────────────────────────────────────────────────────

    #[test]
    fn parse_architectures() {
        assert_eq!("aarch64".parse::<Architecture>().unwrap(), Architecture::Arm64);
        assert_eq!("x86_64".parse::<Architecture>().unwrap(), Architecture::X86_64);
        assert_eq!("armv7".parse::<Architecture>().unwrap(), Architecture::Arm);
        assert_eq!("arm64_32".parse::<Architecture>().unwrap(), Architecture::Arm64_32);
        assert!("riscv64".parse::<Architecture>().is_err());
    }

    // ── Triple ──────────────────────────────────────────────────────────────

    #[test]
    fn macos_targets() {
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-darwin"), "arm64-apple-macosx");
        assert_eq!(Triple::target_to_clang_target("x86_64-apple-darwin"), "x86_64-apple-macosx");
    }

    #[test]
    fn ios_targets() {
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-ios"), "arm64-apple-ios");
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-ios-sim"), "arm64-apple-ios-simulator");
    }

    #[test]
    fn visionos_targets() {
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-visionos"), "arm64-apple-xros");
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-visionos-sim"), "arm64-apple-xros-simulator");
    }

    #[test]
    fn catalyst_targets() {
        assert_eq!(Triple::target_to_clang_target("aarch64-apple-ios-macabi"), "arm64-apple-ios-macabi");
    }

    #[test]
    fn unknown_passthrough() {
        assert_eq!(Triple::target_to_clang_target("x86_64-unknown-linux-gnu"), "x86_64-unknown-linux-gnu");
    }

    // ── SDK ─────────────────────────────────────────────────────────────────

    #[test]
    fn sdk_macos() {
        assert_eq!(SDK::target_to_sdk("aarch64-apple-darwin").unwrap(), "macosx");
        assert_eq!(SDK::target_to_sdk("x86_64-apple-darwin").unwrap(), "macosx");
    }

    #[test]
    fn sdk_ios() {
        assert_eq!(SDK::target_to_sdk("aarch64-apple-ios").unwrap(), "iphoneos");
        assert_eq!(SDK::target_to_sdk("aarch64-apple-ios-sim").unwrap(), "iphonesimulator");
    }

    #[test]
    fn sdk_visionos() {
        assert_eq!(SDK::target_to_sdk("aarch64-apple-visionos").unwrap(), "xros");
        assert_eq!(SDK::target_to_sdk("aarch64-apple-visionos-sim").unwrap(), "xrsimulator");
    }

    #[test]
    fn sdk_tvos() {
        assert_eq!(SDK::target_to_sdk("aarch64-apple-tvos").unwrap(), "appletvos");
        assert_eq!(SDK::target_to_sdk("aarch64-apple-tvos-sim").unwrap(), "appletvsimulator");
    }

    #[test]
    fn sdk_watchos() {
        assert_eq!(SDK::target_to_sdk("aarch64-apple-watchos").unwrap(), "watchos");
        assert_eq!(SDK::target_to_sdk("aarch64-apple-watchos-sim").unwrap(), "watchsimulator");
    }

    #[test]
    fn sdk_catalyst() {
        // Mac Catalyst uses the macOS SDK
        assert_eq!(SDK::target_to_sdk("aarch64-apple-ios-macabi").unwrap(), "macosx");
    }

    #[test]
    fn sdk_unknown_fails() {
        assert!(SDK::target_to_sdk("x86_64-unknown-linux-gnu").is_err());
    }
}
