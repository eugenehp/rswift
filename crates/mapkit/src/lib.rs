//! Apple MapKit — maps and directions from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 3+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps MapKit for map views, annotations, overlays, and directions.
//!
//! ```ignore
//! assert!(mapkit::is_available());
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

apple_sys_helpers::apple_framework!(c"mapkit_available");
