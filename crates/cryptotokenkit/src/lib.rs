//! Apple CryptoTokenKit — smart cards and crypto tokens from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 13+, tvOS 14+, watchOS 7+.
//!
//! ```ignore
//! assert!(cryptotokenkit::is_available());
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

apple_sys_helpers::apple_framework!(c"cryptotokenkit_available"; "macos", "ios", "tvos", "watchos");
