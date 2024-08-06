//! Apple PushKit — VoIP and complication push notifications from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 8+, visionOS 1+, watchOS 6+.
//!
//! Wraps PushKit for receiving VoIP pushes and complication updates.
//!
//! ```ignore
//! assert!(pushkit::is_available());
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

apple_sys_helpers::apple_framework!(c"pushkit_available"; "macos", "ios", "xros", "watchos");
