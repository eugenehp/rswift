//! Apple GameSave — game save management from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+, tvOS 26+.
//!
//! ```ignore
//! assert!(gamesave::is_available());
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

apple_sys_helpers::apple_framework!(c"gamesave_available"; "macos", "ios", "tvos");
