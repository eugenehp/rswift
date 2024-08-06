//! Apple LiveCommunicationKit — live calling from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+.
//!
//! Wraps LiveCommunicationKit for VoIP calling with modern async API.
//!
//! ```ignore
//! assert!(livecommunicationkit::is_available());
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

apple_sys_helpers::apple_framework!(c"livecommunicationkit_available"; "macos", "ios");
