//! Apple CoreAnimation — animation timing and transactions from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 2+, tvOS 9+, visionOS 1+.
//!
//! # Quick start
//!
//! ```ignore
//! // High-precision media time
//! let t = coreanimation::current_media_time();
//!
//! // Animate with explicit transactions
//! coreanimation::Transaction::begin();
//! coreanimation::Transaction::set_duration(0.3);
//! // ... modify layers ...
//! coreanimation::Transaction::commit();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"coreanimation_available");

unsafe extern "C" {
    fn coreanimation_current_media_time() -> f64;
    fn coreanimation_transaction_begin();
    fn coreanimation_transaction_commit();
    fn coreanimation_transaction_flush();
    fn coreanimation_transaction_set_duration(d: f64);
    fn coreanimation_transaction_set_disable_actions(d: bool);
    fn coreanimation_transaction_set_timing(t: isize);
}

/// High-precision media time (seconds since boot, monotonic).
pub fn current_media_time() -> f64 {
    unsafe { coreanimation_current_media_time() }
}

/// Animation timing functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingFunction {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Default = 4,
}

/// Explicit animation transaction (wraps `CATransaction`).
pub struct Transaction;

impl Transaction {
    /// Begin an explicit animation transaction.
    pub fn begin() { unsafe { coreanimation_transaction_begin() } }

    /// Commit the current transaction.
    pub fn commit() { unsafe { coreanimation_transaction_commit() } }

    /// Flush pending transactions immediately.
    pub fn flush() { unsafe { coreanimation_transaction_flush() } }

    /// Set the animation duration for the current transaction.
    pub fn set_duration(seconds: f64) {
        unsafe { coreanimation_transaction_set_duration(seconds) }
    }

    /// Disable implicit animations in the current transaction.
    pub fn set_disable_actions(disable: bool) {
        unsafe { coreanimation_transaction_set_disable_actions(disable) }
    }

    /// Set the timing function for the current transaction.
    pub fn set_timing(timing: TimingFunction) {
        unsafe { coreanimation_transaction_set_timing(timing as isize) }
    }
}
