//! Apple VisionKit — document scanning and visual lookup from Rust.
//!
//! **Platform support:** macOS 13+, iOS 13+, visionOS 1+.
//!
//! Wraps VisionKit for document camera, data scanner, and Live Text.
//!
//! ```ignore
//! assert!(visionkit::is_available());
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

apple_sys_helpers::apple_framework!(c"visionkit_available"; "macos", "ios", "xros");
