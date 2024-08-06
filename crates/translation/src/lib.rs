//! Apple Translation framework — on-device text translation from Rust.
//!
//! **Platform support:** macOS 15+, iOS 18+ (not available on tvOS, watchOS, or visionOS).
//!
//! Wraps Apple's Translation framework for privacy-preserving,
//! on-device translation. Available on macOS 15+ and iOS 18+.
//!
//! ```ignore
//! use translation::*;
//!
//! assert!(is_available());
//! ```
//!
//! Note: Full translation requires async session management which
//! is bridged through the SwiftUI `.translationPresentation()` modifier.

//!
//! ## Citation
//!
//! ```bibtex
//! @software{rswift,
//!   author       = {Eugene Hauptmann},
//!   title        = {rswift},
//!   year         = {2025},
//!   url          = {https://github.com/eugenehp/rswift},
//!   note         = {Build native Apple apps from Rust}
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"translation_available"; "macos", "ios");
