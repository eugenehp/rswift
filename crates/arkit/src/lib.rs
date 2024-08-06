//! Apple ARKit — augmented reality from Rust.
//!
//! **Platform support:** iOS 11+, visionOS 1+.
//!
//! Wraps ARKit for world tracking, plane detection, face tracking, and body tracking.
//!
//! ```ignore
//! assert!(arkit::is_available());
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

apple_sys_helpers::apple_framework!(c"arkit_available"; "ios", "xros");
