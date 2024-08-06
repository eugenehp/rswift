//! Apple PassKit — Wallet and Apple Pay from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 6+, visionOS 1+, watchOS 2+.
//!
//! Wraps PassKit for Apple Pay, Wallet passes, and payment sheets.
//!
//! ```ignore
//! assert!(passkit::is_available());
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

apple_sys_helpers::apple_framework!(c"passkit_available"; "macos", "ios", "xros", "watchos");
