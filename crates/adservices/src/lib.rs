//! Apple AdServices — ad attribution from Rust.
//!
//! **Platform support:** macOS 14+, iOS 14.3+.
//!
//! Wraps AdServices for Apple Search Ads attribution tokens.
//!
//! ```ignore
//! assert!(adservices::is_available());
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

apple_sys_helpers::apple_framework!(c"adservices_available"; "macos", "ios");
