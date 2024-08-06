//! Apple AdSupport — advertising identifier from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 6+, tvOS 9+, visionOS 1+.
//!
//! Wraps AdSupport for reading the IDFA advertising identifier.
//!
//! ```ignore
//! assert!(adsupport::is_available());
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

apple_sys_helpers::apple_framework!(c"adsupport_available"; "macos", "ios", "tvos", "xros");
