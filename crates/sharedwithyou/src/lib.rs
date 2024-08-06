//! Apple SharedWithYou — Messages collaboration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+.
//!
//! Wraps SharedWithYou for surfacing content shared via Messages.
//!
//! ```ignore
//! assert!(sharedwithyou::is_available());
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

apple_sys_helpers::apple_framework!(c"sharedwithyou_available"; "macos", "ios", "tvos", "xros");
