//! Apple Core Graphics — 2D drawing from Rust.
//!
//! **Platform support:** macOS 10.0+, iOS 2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Graphics (Quartz 2D) for paths, colors, images, and PDF generation.
//!
//! ```ignore
//! assert!(coregraphics::is_available());
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

apple_sys_helpers::apple_framework!(c"coregraphics_available");
