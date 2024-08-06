//! Apple NetworkExtension — VPN and content filtering from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 8+, tvOS 17+, visionOS 1+.
//!
//! Wraps NetworkExtension for VPN, DNS proxy, and content filter providers.
//!
//! ```ignore
//! assert!(networkextension::is_available());
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

apple_sys_helpers::apple_framework!(c"networkextension_available"; "macos", "ios", "tvos", "xros");
