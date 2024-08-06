//! Apple GroupActivities — SharePlay from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+.
//!
//! Wraps GroupActivities for SharePlay sessions and shared experiences.
//!
//! ```ignore
//! assert!(groupactivities::is_available());
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

apple_sys_helpers::apple_framework!(c"groupactivities_available"; "macos", "ios", "tvos", "xros");
