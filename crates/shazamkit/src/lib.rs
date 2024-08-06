//! Apple ShazamKit — music recognition from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+, watchOS 8+.
//!
//! Wraps ShazamKit for identifying songs from audio and building custom catalogs.
//!
//! ```ignore
//! assert!(shazamkit::is_available());
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

apple_sys_helpers::apple_framework!(c"shazamkit_available");
