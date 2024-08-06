//! Apple Matter — smart home connectivity from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+, tvOS 16+.
//!
//! Wraps Matter/MatterSupport for commissioning and controlling Matter smart home devices.
//!
//! ```ignore
//! assert!(matter::is_available());
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

apple_sys_helpers::apple_framework!(c"matter_available"; "macos", "ios", "tvos");
