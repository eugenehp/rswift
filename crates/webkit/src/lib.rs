//! Apple WebKit — web views from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 8+, visionOS 1+.
//!
//! Wraps WebKit/WKWebView for embedding web content, navigation, and JavaScript evaluation.
//!
//! ```ignore
//! assert!(webkit::is_available());
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

apple_sys_helpers::apple_framework!(c"webkit_available"; "macos", "ios", "xros");
