//! Apple Image Playground — AI image generation from Rust.
//!
//! **Platform support:** macOS 15.2+, iOS 18.2+.
//!
//! Wraps Image Playground for on-device AI image generation.
//!
//! ```ignore
//! assert!(imageplayground::is_available());
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

apple_sys_helpers::apple_framework!(c"imageplayground_available"; "macos", "ios");
