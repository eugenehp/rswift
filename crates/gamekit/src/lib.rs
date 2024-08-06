//! Apple GameKit — Game Center from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 4+, tvOS 9+, visionOS 1+.
//!
//! Wraps GameKit for leaderboards, achievements, matchmaking, and Game Center.
//!
//! ```ignore
//! assert!(gamekit::is_available());
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

apple_sys_helpers::apple_framework!(c"gamekit_available"; "macos", "ios", "tvos", "xros");
