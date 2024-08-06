//! Apple AccessorySetupKit — accessory pairing from Rust.
//!
//! **Platform support:** macOS 15+, iOS 18+.
//!
//! Wraps AccessorySetupKit for discovering and pairing Bluetooth/Wi-Fi accessories.
//!
//! ```ignore
//! assert!(accessorysetupkit::is_available());
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

apple_sys_helpers::apple_framework!(c"accessorysetupkit_available"; "macos", "ios");
