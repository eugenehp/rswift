//! Apple MetalKit — Metal utilities from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, tvOS 9+, visionOS 1+.
//!
//! Wraps MetalKit for MTKView, texture loading, and model I/O integration.
//!
//! ```ignore
//! assert!(metalkit::is_available());
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

apple_sys_helpers::apple_framework!(c"metalkit_available"; "macos", "ios", "tvos", "xros");
