//! Apple Core Text — text layout and font handling from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 3.2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Text for advanced text layout, font enumeration, and glyph rendering.
//!
//! ```ignore
//! assert!(coretext::is_available());
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

apple_sys_helpers::apple_framework!(c"coretext_available");
