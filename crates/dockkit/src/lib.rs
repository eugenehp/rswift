//! Apple DockKit — motorized stand control from Rust.
//!
//! **Platform support:** iOS 17+.
//!
//! Wraps DockKit for controlling motorized camera stands and tracking.
//!
//! ```ignore
//! assert!(dockkit::is_available());
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

apple_sys_helpers::apple_framework!(c"dockkit_available"; "ios");
