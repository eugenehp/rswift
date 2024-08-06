//! Apple CloudKit — iCloud database from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps CloudKit for iCloud public/private database, records, and subscriptions.
//!
//! ```ignore
//! assert!(cloudkit::is_available());
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

apple_sys_helpers::apple_framework!(c"cloudkit_available");
