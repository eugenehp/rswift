//! Apple Vision framework — image analysis and computer vision from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 11+, tvOS 11+, visionOS 1+.
//!
//! Wraps Vision for face detection, text recognition, image classification, and more.
//!
//! ```ignore
//! assert!(vision::is_available());
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

apple_sys_helpers::apple_framework!(c"vision_available"; "macos", "ios", "tvos", "xros");
