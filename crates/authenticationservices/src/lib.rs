//! Apple AuthenticationServices — Sign in with Apple from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 12+, tvOS 16+, visionOS 1+, watchOS 6+.
//!
//! Wraps AuthenticationServices for Sign in with Apple, passkeys, and web authentication.
//!
//! ```ignore
//! assert!(authenticationservices::is_available());
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

apple_sys_helpers::apple_framework!(c"authenticationservices_available");
