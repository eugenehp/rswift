//! Apple UserNotifications — push and local notifications from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 10+, tvOS 10+, visionOS 1+, watchOS 3+.
//!
//! Wraps UserNotifications for scheduling local notifications and handling remote push.
//!
//! ```ignore
//! assert!(usernotifications::is_available());
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

apple_sys_helpers::apple_framework!(c"usernotifications_available");
