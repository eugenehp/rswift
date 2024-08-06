//! Apple Cinematic — cinematic video processing from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+.
//!
//! Wraps Cinematic for processing Cinematic mode video and adjusting depth of field.
//!
//! ```ignore
//! assert!(cinematic::is_available());
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

apple_sys_helpers::apple_framework!(c"cinematic_available"; "macos", "ios");
