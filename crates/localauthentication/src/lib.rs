//! Apple LocalAuthentication — biometric auth from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 8+, visionOS 1+, watchOS 3+.
//!
//! Wraps LocalAuthentication for Face ID, Touch ID, and password authentication.
//!
//! ```ignore
//! assert!(localauthentication::is_available());
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

apple_sys_helpers::apple_framework!(c"localauthentication_available"; "macos", "ios", "xros", "watchos");
