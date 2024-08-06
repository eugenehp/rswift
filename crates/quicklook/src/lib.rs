//! Apple QuickLook — file previews from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 4+, visionOS 1+.
//!
//! Wraps QuickLook for previewing documents, images, and 3D models.
//!
//! ```ignore
//! assert!(quicklook::is_available());
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

apple_sys_helpers::apple_framework!(c"quicklook_available"; "macos", "ios", "xros");
