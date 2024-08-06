//! Apple Network framework — modern networking from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 12+, tvOS 12+, visionOS 1+, watchOS 6+.
//!
//! Wraps Network.framework for TCP/UDP/QUIC connections, listeners, and path monitoring.
//!
//! ```ignore
//! assert!(network::is_available());
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

apple_sys_helpers::apple_framework!(c"network_available");
