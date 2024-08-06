//! Apple CallKit — VoIP call integration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 10+, watchOS 9+.
//!
//! Wraps CallKit for VoIP call UI, call directory, and blocking/identification.
//!
//! ```ignore
//! assert!(callkit::is_available());
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

apple_sys_helpers::apple_framework!(c"callkit_available"; "macos", "ios", "watchos");
