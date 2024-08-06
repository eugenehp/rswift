//! Apple PhotosUI — photo picker from Rust.
//!
//! **Platform support:** macOS 13+, iOS 14+, visionOS 1+, watchOS 9+.
//!
//! Wraps PhotosUI for PHPickerViewController and editing extensions.
//!
//! ```ignore
//! assert!(photosui::is_available());
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

apple_sys_helpers::apple_framework!(c"photosui_available"; "macos", "ios", "xros", "watchos");
