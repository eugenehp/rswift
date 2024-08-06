//! Apple Photos — photo library access from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 8+, tvOS 10+, visionOS 1+.
//!
//! Wraps Photos for fetching, caching, and editing photo assets and albums.
//!
//! ```ignore
//! assert!(photos::is_available());
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

apple_sys_helpers::apple_framework!(c"photos_available"; "macos", "ios", "tvos", "xros");
