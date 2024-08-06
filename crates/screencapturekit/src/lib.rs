//! Apple ScreenCaptureKit — screen recording from Rust.
//!
//! **Platform support:** macOS 12.3+.
//!
//! Wraps ScreenCaptureKit for capturing screen content, windows, and apps.
//!
//! ```ignore
//! assert!(screencapturekit::is_available());
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

apple_sys_helpers::apple_framework!(c"screencapturekit_available"; "macos");
