//! Apple LinkPresentation — URL previews from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, visionOS 1+.
//!
//! Wraps LinkPresentation for fetching rich URL metadata and preview views.
//!
//! ```ignore
//! assert!(linkpresentation::is_available());
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

apple_sys_helpers::apple_framework!(c"linkpresentation_available"; "macos", "ios", "xros");
