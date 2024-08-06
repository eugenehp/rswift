//! Apple SceneKit — 3D rendering from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps SceneKit for 3D scene graphs, physics, and rendering.
//!
//! ```ignore
//! assert!(scenekit::is_available());
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

apple_sys_helpers::apple_framework!(c"scenekit_available");
