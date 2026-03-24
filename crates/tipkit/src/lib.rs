//! Apple TipKit — contextual tips and hints from Rust.
//!
//! **Platform:** macOS 14+, iOS 17+, tvOS 17+, watchOS 10+.
//!
//! ```ignore
//! tipkit::configure(tipkit::DisplayFrequency::Daily);
//! // Tips are shown via SwiftUI TipView — this crate manages the data store.
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" {
    fn tipkit_swift_available() -> bool;
    fn tipkit_configure(display_frequency: isize);
    fn tipkit_reset_datastore();
    fn tipkit_show_all_tips();
    fn tipkit_hide_all_tips();
}

pub fn is_available() -> bool { unsafe { tipkit_swift_available() } }

/// Display frequency for tips.
#[derive(Debug, Clone, Copy)]
pub enum DisplayFrequency {
    Immediate = 0,
    Hourly = 1,
    Daily = 2,
    Weekly = 3,
    Monthly = 4,
}

/// Configure the TipKit system with a display frequency.
pub fn configure(frequency: DisplayFrequency) {
    unsafe { tipkit_configure(frequency as isize); }
}

/// Reset the tip data store (all tips become eligible again).
pub fn reset_datastore() {
    unsafe { tipkit_reset_datastore(); }
}

/// Show all tips regardless of eligibility (for testing).
pub fn show_all_tips_for_testing() {
    unsafe { tipkit_show_all_tips(); }
}

/// Hide all tips (for testing).
pub fn hide_all_tips_for_testing() {
    unsafe { tipkit_hide_all_tips(); }
}
