//! Apple platform metadata, target-triple conversion, and SDK name resolution.
//!
//! This crate provides a typed Rust representation of every Apple platform
//! defined in [LLVM's `MachO.def`][llvm-macho] — macOS, iOS, tvOS, watchOS,
//! visionOS, Mac Catalyst, DriverKit, BridgeOS, and their simulator variants —
//! together with utilities for converting Rust target triples to the strings
//! that Apple's toolchain expects.
//!
//! It is primarily intended for use in `build.rs` scripts that invoke `swiftc`,
//! `xcrun`, or `clang`.
//!
//! [llvm-macho]: https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def
//!
//! # Quick start
//!
//! ```
//! use apple_platforms::{ApplePlatform, triple};
//!
//! // Parse a Rust target triple into a typed platform value
//! let p = ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap();
//! assert_eq!(p, ApplePlatform::IOSSimulator);
//!
//! // SDK name for `xcrun --sdk <name>`
//! assert_eq!(p.sdk(),    Some("iphonesimulator"));
//!
//! // Navigate sim ↔ device
//! assert_eq!(p.device(), ApplePlatform::IOS);
//! assert_eq!(ApplePlatform::IOS.simulator(), Some(ApplePlatform::IOSSimulator));
//!
//! // Access full LLVM-sourced metadata
//! let meta = p.metadata();
//! assert_eq!(meta.marketing,   "iOS Simulator");
//! assert_eq!(meta.tapi_target, "ios-simulator");
//!
//! // String-only path — useful when you only need strings
//! assert_eq!(triple::to_clang("aarch64-apple-darwin"), Some("arm64-apple-macosx"));
//! assert_eq!(triple::to_sdk("aarch64-apple-ios"),      Some("iphoneos"));
//! ```
//!
//! # Modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`platform`] | [`ApplePlatform`] enum, [`Platform`] metadata struct, [`ParsePlatformError`] |
//! | [`triple`]   | [`triple::to_clang`], [`triple::to_sdk`] free functions, [`triple::Architecture`] |
//!
//! # Typical `build.rs` pattern
//!
//! ```rust,ignore
//! fn main() {
//!     let target = std::env::var("TARGET").unwrap();
//!
//!     let Some(clang_triple) = apple_platforms::triple::to_clang(&target) else {
//!         return; // not an Apple target
//!     };
//!
//!     let platform = apple_platforms::ApplePlatform::from_rust_triple(&target).unwrap();
//!     let sdk_name  = platform.sdk().unwrap_or("macosx");
//!
//!     // Pass to xcrun / swiftc / clang …
//! }
//! ```

pub mod platform;
pub mod triple;

// ── Top-level re-exports ─────────────────────────────────────────────────────

pub use platform::{ApplePlatform, ParsePlatformError, Platform};

// ── Prelude ───────────────────────────────────────────────────────────────────

/// Convenience re-exports for glob import (`use apple_platforms::prelude::*`).
///
/// Brings in the types you'll need most often:
///
/// ```
/// use apple_platforms::prelude::*;
///
/// let p = ApplePlatform::MacOS;
/// assert_eq!(p.sdk(), Some("macosx"));
/// assert_eq!(to_sdk("aarch64-apple-darwin"), Some("macosx"));
/// ```
pub mod prelude {
    pub use super::platform::{ApplePlatform, ParsePlatformError, Platform};
    pub use super::triple::{to_clang, to_sdk, Architecture};
}
