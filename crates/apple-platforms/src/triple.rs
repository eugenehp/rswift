use anyhow::{anyhow, Result};
use std::{borrow::Cow, fmt, str::FromStr};

#[derive(Debug)]
pub enum Architecture {
    X86,
    X86_64,
    Arm,
    Arm64,
}

impl FromStr for Architecture {
    type Err = anyhow::Error;

    fn from_str(target: &str) -> Result<Self> {
        match target.to_lowercase().as_str() {
            "x86" => Ok(Architecture::X86),
            "x86_64" => Ok(Architecture::X86_64),
            "arm" => Ok(Architecture::Arm),
            "aarch64" => Ok(Architecture::Arm64),
            _ => Err(anyhow!("Unsupported architecture: {target}")),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Architecture::X86 => Cow::from("x86"),
            Architecture::X86_64 => Cow::from("x64"),
            Architecture::Arm => Cow::from("arm"),
            Architecture::Arm64 => Cow::from("arm64"),
        };
        write!(f, "{}", str)
    }
}

pub struct Triple;

impl Triple {
    /**
     * Convert rust target to clang compatible target.
     * https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def#L123-L138
     * Note: `visionos` is still called `xros` in clang, though it's `visionos` in Xcode and rust.
     */
    pub fn target_to_clang_target(target: &str) -> &str {
        // TODO: implement using Architecture and Platform.tapi_target
        match target {
            "aarch64-apple-ios-macabi" => "arm64-apple-maccatalyst",
            "aarch64-apple-ios" => "arm64-apple-ios",
            "aarch64-apple-ios-sim" => "aarch64-apple-ios-simulator",
            "aarch64-apple-tvos" => "arm64-apple-tvos",
            "aarch64-apple-tvos-sim" => "aarch-apple-tvos-simulator",
            "aarch64-apple-watchos-sim" => "aarch64-apple-watchos-simulator",
            "aarch64-apple-driverkit" => "arm64-apple-driverkit",
            "aarch64-apple-visionos" => "arm64-apple-xros",
            "aarch64-apple-visionos-sim" => "aarch64-apple-xros-simulator",
            "aarch64-apple-darwin" => "arm64-apple-darwin",
            // "aarch64-apple-watchos" => "",
            // "arm64_32-apple-watchos" => "",
            // "armv7k-apple-watchos" => "",
            // "aarch64-apple-watchos-sim" => "",
            // "x86_64-apple-watchos-sim" => "",
            // "aarch64-apple-tvos" => "",
            // "aarch64-apple-tvos-sim" => "",
            // "x86_64-apple-tvos" => "",
            _ => target,
        }
    }
}

pub struct SDK;

impl SDK {
    pub fn target_to_sdk(target: &str) -> Result<&str> {
        match target {
            "aarch64-apple-darwin" | "x86_64-apple-darwin" => Ok("macosx"),
            "x86_64-apple-ios" | "i386-apple-ios" | "aarch64-apple-ios-sim" => {
                Ok("iphonesimulator")
            }
            "aarch64-apple-ios" | "armv7-apple-ios" | "armv7s-apple-ios" => Ok("iphoneos"),
            "aarch64-apple-visionos-sim" => Ok("xrsimulator"),
            "aarch64-apple-visionos" => Ok("xros"),
            "aarch64-apple-tvos-sim" | "x86_64-apple-tvos" => Ok("appletvsimulator"),
            "aarch64-apple-tvos" => Ok("appletvos"),
            "aarch64-apple-watchos" | "armv7k-apple-watchos" | "arm64_32-apple-watchos" => {
                Ok("watchos")
            }

            "aarch64-apple-watchos-sim" | "x86_64-apple-watchos-sim" => Ok("watchsimulator"),
            _ => Err(anyhow!("Unsupported target for sdk: {target}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_clang() {
        let clang_target = Triple::target_to_clang_target("aarch64-apple-visionos");
        assert_eq!(clang_target, "arm64-apple-xros")
    }
}
