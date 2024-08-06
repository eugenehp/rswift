//! Apple RealityFoundation — RealityKit foundation types from Rust.
//!
//! **Platform support:** macOS 15+, iOS 18+, visionOS 2+.
//!
//! ```ignore
//! assert!(realityfoundation::is_available());
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

apple_sys_helpers::apple_framework!(c"realityfoundation_available"; "macos", "ios", "xros");
