//! Apple SwiftData — modern persistence framework from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+, watchOS 10+.
//!
//! Wraps SwiftData for declarative data modeling and persistence.
//!
//! Note: The existing `swift-data` crate provides UserDefaults. This crate wraps the SwiftData framework.
//!
//! ```ignore
//! assert!(swiftdata::is_available());
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

apple_sys_helpers::apple_framework!(c"swiftdata_available");
