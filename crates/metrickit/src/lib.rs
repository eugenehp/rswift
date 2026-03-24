//! Apple MetricKit — app diagnostics and metrics from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// MXMetricManager shared instance.
pub fn shared_manager_ptr() -> Id {
    unsafe { msg_send![class!(b"MXMetricManager\0"), sharedManager] }
}

