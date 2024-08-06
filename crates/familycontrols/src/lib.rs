//! Apple FamilyControls — parental controls from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+.
//!
//! Wraps FamilyControls for requesting Screen Time authorization and app restrictions.
//!
//! ```ignore
//! assert!(familycontrols::is_available());
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

apple_sys_helpers::apple_framework!(c"familycontrols_available"; "macos", "ios");
