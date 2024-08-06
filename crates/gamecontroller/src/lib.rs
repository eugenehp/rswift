//! Apple GameController — controller input from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 7+, tvOS 9+, visionOS 1+.
//!
//! Wraps GameController for MFi gamepads, keyboard, mouse, and racing wheel input.
//!
//! ```ignore
//! assert!(gamecontroller::is_available());
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

apple_sys_helpers::apple_framework!(c"gamecontroller_available"; "macos", "ios", "tvos", "xros");
