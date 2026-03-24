#![allow(dead_code)]
//! Apple GroupActivities — SharePlay from Rust.
//!
//! **Platform:** macOS 13+, iOS 15+, tvOS 15+.
//!
//! SharePlay's `GroupActivity` protocol requires Swift conformance.
//! This crate bridges the eligibility check and session state.
//!
//! ```ignore
//! if groupactivities::is_eligible_for_group_session() {
//!     println!("SharePlay is available");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" {
    fn ga_swift_available() -> bool;
    fn ga_is_eligible_for_group_session() -> bool;
    fn ga_group_state_is_eligible() -> bool;
}

pub fn is_available() -> bool { unsafe { ga_swift_available() } }

/// Whether the device is currently eligible for a group session (FaceTime call active).
pub fn is_eligible_for_group_session() -> bool {
    unsafe { ga_group_state_is_eligible() }
}
