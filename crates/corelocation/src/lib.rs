//! Apple Core Location — GPS and location services from Rust.
//!
//! **Platform support:** macOS 10.6+, iOS 2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Location for GPS, geofencing, beacon ranging, and heading updates.
//!
//! ```ignore
//! assert!(corelocation::is_available());
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

apple_sys_helpers::apple_framework!(c"corelocation_available");
