//! Apple SafariServices — in-app browser from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 7+, visionOS 1+.
//!
//! Wraps SafariServices for SFSafariViewController, content blockers, and web extensions.
//!
//! ```ignore
//! assert!(safariservices::is_available());
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

apple_sys_helpers::apple_framework!(c"safariservices_available"; "macos", "ios", "xros");
