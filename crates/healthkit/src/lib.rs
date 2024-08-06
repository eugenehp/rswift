//! Apple HealthKit — health and fitness data from Rust.
//!
//! **Platform support:** macOS 13+, iOS 8+, visionOS 1+, watchOS 2+.
//!
//! Wraps HealthKit for reading/writing health samples, workouts, and statistics.
//!
//! ```ignore
//! assert!(healthkit::is_available());
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

apple_sys_helpers::apple_framework!(c"healthkit_available"; "macos", "ios", "xros", "watchos");
