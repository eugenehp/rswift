//! Apple HomeKit — smart home control from Rust.
//!
//! **Platform support:** iOS 8+, watchOS 2+, tvOS 10+, visionOS 1+.
//!
//! ```ignore
//! assert!(homekit::is_available());
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

apple_sys_helpers::apple_framework!(c"homekit_available"; "ios", "watchos", "tvos", "xros");
