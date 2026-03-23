//! Apple VideoSubscriberAccount — TV provider authentication from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 10+, tvOS 10+.
//!
//! Wraps VideoSubscriberAccount for TV provider single sign-on and
//! subscriber verification. The Swift helper is compiled and linked
//! automatically via `build.rs` — no manual setup needed.
//!
//! # Quick start
//!
//! ```ignore
//! if videosubscriberaccount::is_available() {
//!     println!("✅ TV provider authentication is available");
//! }
//! ```
//!
//! # Use cases
//!
//! - **Single sign-on** with cable/satellite TV providers
//! - **Subscriber verification** for premium content
//! - **TV provider picker** UI integration

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

apple_sys_helpers::apple_framework!(c"videosubscriberaccount_available"; "macos", "ios", "tvos");
