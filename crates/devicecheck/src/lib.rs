//! Apple DeviceCheck — device attestation from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 11+, tvOS 11+, visionOS 1+, watchOS 9+.
//!
//! Wraps DeviceCheck and App Attest for device-level fraud prevention.
//!
//! ```ignore
//! assert!(devicecheck::is_available());
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

apple_sys_helpers::apple_framework!(c"devicecheck_available");
