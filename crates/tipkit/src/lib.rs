//! Apple TipKit — in-app tips and hints from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+, watchOS 10+.
//!
//! Wraps TipKit for displaying contextual tips, feature discovery, and onboarding hints.
//!
//! ```ignore
//! assert!(tipkit::is_available());
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

apple_sys_helpers::apple_framework!(c"tipkit_available");
