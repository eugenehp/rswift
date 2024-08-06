//! Apple UniformTypeIdentifiers — UTI system from Rust.
//!
//! **Platform support:** macOS 11+, iOS 14+, tvOS 14+, visionOS 1+, watchOS 7+.
//!
//! Wraps UniformTypeIdentifiers for declaring and querying file types.
//!
//! ```ignore
//! assert!(uniformtypeidentifiers::is_available());
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

apple_sys_helpers::apple_framework!(c"uniformtypeidentifiers_available");
