//! Apple CryptoTokenKit — smart card and token access from Rust.
//!
//! **Platform:** macOS 10.10+, iOS 10+.
//!
//! ```ignore
//! let mgr = cryptotokenkit::SmartCardSlotManager::shared();
//! let slots = mgr.slot_names();
//! for s in &slots { println!("Slot: {s}"); }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"TKSmartCardSlotManager\0").is_null() }
}

/// Wraps `TKSmartCardSlotManager`.
pub struct SmartCardSlotManager { inner: Id }

impl SmartCardSlotManager {
    /// Shared slot manager.
    pub fn shared() -> Option<Self> {
        unsafe {
            let mgr: Id = msg_send![class!(b"TKSmartCardSlotManager\0"), defaultManager];
            if mgr.is_null() { None } else { Some(Self { inner: mgr }) }
        }
    }

    /// List available slot names.
    pub fn slot_names(&self) -> Vec<String> {
        unsafe {
            let arr: Id = msg_send![self.inner, slotNames];
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
                .collect()
        }
    }
}

/// Smart card ATR (Answer To Reset) info.
pub struct SmartCardATR { inner: Id }

impl SmartCardATR {
    /// Parse ATR from raw bytes.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let atr: Id = msg_send![class!(b"TKSmartCardATR\0"), alloc];
            let atr = msg_send![atr, initWithBytes: nsdata];
            if atr.is_null() { None } else { Some(Self { inner: atr }) }
        }
    }

    /// Number of interface groups.
    pub fn interface_group_count(&self) -> usize {
        unsafe {
            let arr: Id = msg_send![self.inner, interfaceGroups];
            if arr.is_null() { 0 } else { msg_send_t![usize; arr, count] }
        }
    }

    /// Whether the card supports a specific protocol (T=0: 1, T=1: 2).
    pub fn has_protocol(&self, protocol: usize) -> bool {
        unsafe {
            let arr: Id = msg_send![self.inner, protocols];
            if arr.is_null() { return false; }
            let count: usize = msg_send_t![usize; arr, count];
            for i in 0..count {
                let val: Id = msg_send![arr, objectAtIndex: i];
                let v: usize = msg_send_t![usize; val, unsignedIntegerValue];
                if v == protocol { return true; }
            }
            false
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for SmartCardATR { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_manager() {
        // May be None if no smart card subsystem
        if let Some(mgr) = SmartCardSlotManager::shared() {
            let slots = mgr.slot_names();
            println!("Smart card slots: {:?}", slots);
        }
    }
}
