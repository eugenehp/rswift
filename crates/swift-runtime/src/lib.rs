//! Safe Rust interface to the Swift Runtime.
//!
//! This crate provides safe, ergonomic wrappers around the raw FFI bindings
//! in `swift-runtime-sys`.

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

pub use swift_runtime_sys as sys;

pub mod concurrency;
pub mod debug;
pub mod demangle;
pub mod metadata;
pub mod retain;
pub mod string;
pub mod types;
