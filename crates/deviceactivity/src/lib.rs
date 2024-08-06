//! Apple DeviceActivity — Screen Time monitoring from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+.
//!
//! Wraps DeviceActivity for monitoring app and website usage (Screen Time API).
//!
//! ```ignore
//! assert!(deviceactivity::is_available());
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

apple_sys_helpers::apple_framework!(c"deviceactivity_available"; "macos", "ios");
