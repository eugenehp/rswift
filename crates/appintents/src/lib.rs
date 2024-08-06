//! Apple AppIntents — Siri shortcuts and Spotlight integration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! ```ignore
//! assert!(appintents::is_available());
//! ```
//!
//! Note: Defining actual App Intents requires the Swift @AppIntent protocol
//! which needs compiler macro support. This crate provides availability
//! checking and will be extended as the bridge generator supports protocols.

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

apple_sys_helpers::apple_framework!(c"appintents_available");
