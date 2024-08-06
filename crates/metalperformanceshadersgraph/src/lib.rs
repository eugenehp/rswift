//! Apple MPS Graph — GPU machine learning graph operations from Rust.
//!
//! **Platform support:** macOS 11+, iOS 14+, tvOS 14+.
//!
//! ```ignore
//! assert!(metalperformanceshadersgraph::is_available());
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

apple_sys_helpers::apple_framework!(c"metalperformanceshadersgraph_available"; "macos", "ios", "tvos");
