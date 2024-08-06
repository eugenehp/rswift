//! Apple Intents — Siri intents and shortcuts from Rust.
//!
//! **Platform support:** macOS 11+, iOS 10+, tvOS 14+, watchOS 3.2+.
//!
//! ```ignore
//! assert!(intents::is_available());
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

apple_sys_helpers::apple_framework!(c"intents_available"; "macos", "ios", "tvos", "watchos");
