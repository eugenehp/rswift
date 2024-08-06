//! Apple Core Spotlight — search indexing from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 9+, visionOS 1+.
//!
//! Wraps Core Spotlight for indexing app content for system search.
//!
//! ```ignore
//! assert!(corespotlight::is_available());
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

apple_sys_helpers::apple_framework!(c"corespotlight_available"; "macos", "ios", "xros");
