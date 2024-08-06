//! Apple CompositorServices — visionOS rendering from Rust.
//!
//! **Platform support:** visionOS 1+.
//!
//! Wraps CompositorServices for low-level visionOS rendering with Metal.
//!
//! ```ignore
//! assert!(compositorservices::is_available());
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

apple_sys_helpers::apple_framework!(c"compositorservices_available"; "xros");
