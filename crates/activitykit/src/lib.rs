//! Apple ActivityKit — Live Activities and Dynamic Island from Rust.
//!
//! **Platform support:** iOS 16.1+ only (not available on macOS, tvOS, or visionOS).
//!
//! ```ignore
//! assert!(activitykit::is_available());
//! ```
//!
//! Note: Creating Live Activities requires defining ActivityAttributes
//! via the @available Swift protocol, which needs compiler macro support.
//! This crate provides availability checking. Full Live Activity support
//! requires a Swift extension target in the app bundle.

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

apple_sys_helpers::apple_framework!(c"activitykit_available"; "ios");
