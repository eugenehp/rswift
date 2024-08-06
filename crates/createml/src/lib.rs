//! Apple Create ML — train machine learning models from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 15+.
//!
//! Wraps Create ML for training image classifiers, text classifiers, and more.
//!
//! ```ignore
//! assert!(createml::is_available());
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

apple_sys_helpers::apple_framework!(c"createml_available"; "macos", "ios");
