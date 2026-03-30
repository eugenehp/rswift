//! # SwiftUI Native — pure Rust, zero Swift source files
//!
//! Constructs SwiftUI views entirely from Rust using:
//! - `dlsym` to resolve symbols from `SwiftUI.framework`
//! - arm64 inline assembly for Swift calling convention
//! - `swift-runtime-sys` for retain/release and string creation
//!
//! ```text
//! ┌────────────┐    dlsym     ┌──────────────────────┐
//! │  Rust code │ ──────────►  │ SwiftUI.framework    │
//! │  (arm64    │   Swift CC   │ (Text, Color, etc.)  │
//! │   asm)     │ ──────────►  │                      │
//! └────────────┘              └──────────────────────┘
//! ```
//!
//! # Quick start
//! ```ignore
//! use swiftui_native::views::*;
//!
//! let t = text("Hello from pure Rust!");
//! let padded = padding(&t, 20.0);
//! show_window(&padded, "Native", 400.0, 300.0);
//! ```
//!
//! # Platform
//!
//! macOS aarch64 only. Requires Swift 6.3+ runtime.

pub mod abi;
pub mod resolve;
pub mod views;
pub mod window;

// Keep old modules for backward compat
pub mod existential;
#[doc(hidden)]
pub mod handle;

pub use views::ViewHandle;
