//! Apple BackgroundTasks — background work scheduling from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 7+.
//!
//! Wraps BackgroundTasks for scheduling app refresh and processing tasks.
//!
//! ```ignore
//! assert!(backgroundtasks::is_available());
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

apple_sys_helpers::apple_framework!(c"backgroundtasks_available");
