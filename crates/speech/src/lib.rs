//! Apple Speech — speech recognition from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 10+, visionOS 1+.
//!
//! Wraps Speech framework for on-device and server-based speech recognition.
//!
//! ```ignore
//! assert!(speech::is_available());
//! ```

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

apple_sys_helpers::apple_framework!(c"speech_available"; "macos", "ios", "xros");
