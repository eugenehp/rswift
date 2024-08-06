//! Apple AutomaticAssessmentConfiguration — exam lockdown from Rust.
//!
//! **Platform support:** macOS 10.15.4+, iOS 13.4+.
//!
//! ```ignore
//! assert!(automaticassessmentconfiguration::is_available());
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

apple_sys_helpers::apple_framework!(c"automaticassessmentconfiguration_available"; "macos", "ios");
