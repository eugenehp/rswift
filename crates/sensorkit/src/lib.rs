//! Apple SensorKit — research sensor data from Rust.
//!
//! **Platform support:** iOS 14+.
//!
//! Wraps SensorKit for ambient light, accelerometer, and keyboard metrics (research use).
//!
//! ```ignore
//! assert!(sensorkit::is_available());
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

apple_sys_helpers::apple_framework!(c"sensorkit_available"; "ios");
