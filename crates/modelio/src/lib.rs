//! Apple Model I/O — 3D model import/export from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, tvOS 9+, visionOS 1+.
//!
//! Wraps Model I/O for loading 3D assets (USD, OBJ, etc.) and voxelization.
//!
//! ```ignore
//! assert!(modelio::is_available());
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

apple_sys_helpers::apple_framework!(c"modelio_available"; "macos", "ios", "tvos", "xros");
