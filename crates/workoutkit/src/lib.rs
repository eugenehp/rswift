//! Apple WorkoutKit — workout composition from Rust.
//!
//! **Platform support:** iOS 17+, watchOS 10+.
//!
//! Wraps WorkoutKit for building custom workout plans and intervals.
//!
//! ```ignore
//! assert!(workoutkit::is_available());
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

apple_sys_helpers::apple_framework!(c"workoutkit_available"; "ios", "watchos");
