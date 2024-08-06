//! Apple CarKey — digital car keys from Rust.
//!
//! **Platform support:** macOS 13.3+, iOS 16.4+, watchOS 9.4+.
//!
//! ```ignore
//! assert!(carkey::is_available());
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

apple_sys_helpers::apple_framework!(c"carkey_available"; "macos", "ios", "watchos");
