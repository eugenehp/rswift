//! Apple ReplayKit — screen recording and broadcasting from Rust.
//!
//! **Platform support:** macOS 11+, iOS 9+, tvOS 10+.
//!
//! Wraps ReplayKit for in-app screen recording and live broadcasting.
//!
//! ```ignore
//! assert!(replaykit::is_available());
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

apple_sys_helpers::apple_framework!(c"replaykit_available"; "macos", "ios", "tvos");
