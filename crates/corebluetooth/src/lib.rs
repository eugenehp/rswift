//! Apple Core Bluetooth — BLE central/peripheral from Rust.
//!
//! **Platform:** macOS 10.10+, iOS 5+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! ```ignore
//! println!("BLE auth: {:?}", corebluetooth::authorization());
//! let central = corebluetooth::CentralManager::new();
//! println!("State: {:?}", central.state());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── Authorization ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authorization { NotDetermined = 0, Restricted = 1, Denied = 2, Allowed = 3 }

impl From<isize> for Authorization {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Restricted, 2=>Self::Denied, 3=>Self::Allowed, _=>Self::NotDetermined }
    }
}

/// Current Bluetooth authorization status.
pub fn authorization() -> Authorization {
    unsafe { Authorization::from(msg_send_t![isize; class!(b"CBManager\0"), authorization]) }
}

// ── Manager state ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerState {
    Unknown = 0, Resetting = 1, Unsupported = 2,
    Unauthorized = 3, PoweredOff = 4, PoweredOn = 5,
}

impl From<isize> for ManagerState {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Resetting, 2=>Self::Unsupported, 3=>Self::Unauthorized,
                  4=>Self::PoweredOff, 5=>Self::PoweredOn, _=>Self::Unknown }
    }
}

// ── CBUUID ──────────────────────────────────────────────────────────────────

/// A Bluetooth UUID (wraps `CBUUID`).
pub struct Uuid { inner: Id }

impl Uuid {
    /// Create from a UUID string (e.g. `"180D"` for Heart Rate Service).
    pub fn from_string(s: &str) -> Self {
        unsafe {
            let ns = nsstring(s);
            let u: Id = msg_send![class!(b"CBUUID\0"), UUIDWithString: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(u as CFTypeRef);
            Self { inner: u }
        }
    }

    /// The UUID string.
    pub fn string(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, UUIDString]).unwrap_or_default() }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Uuid { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

impl Clone for Uuid {
    fn clone(&self) -> Self {
        unsafe { CFRetain(self.inner as CFTypeRef); }
        Self { inner: self.inner }
    }
}

/// Well-known Bluetooth service UUIDs.
pub mod services {
    pub const HEART_RATE: &str = "180D";
    pub const BATTERY: &str = "180F";
    pub const DEVICE_INFORMATION: &str = "180A";
    pub const BLOOD_PRESSURE: &str = "1810";
    pub const HEALTH_THERMOMETER: &str = "1809";
    pub const CYCLING_SPEED_CADENCE: &str = "1816";
    pub const RUNNING_SPEED_CADENCE: &str = "1814";
    pub const GENERIC_ACCESS: &str = "1800";
    pub const GENERIC_ATTRIBUTE: &str = "1801";
}

// ── CentralManager ──────────────────────────────────────────────────────────

/// Wraps `CBCentralManager` for scanning and connecting to peripherals.
///
/// Note: delegate-based callbacks require ObjC block support. This provides
/// the synchronous query API; for full scan/connect flow, use as_ptr() and
/// set delegates from your ObjC/Swift layer.
pub struct CentralManager { inner: Id }

impl CentralManager {
    /// Create a new central manager (no delegate).
    pub fn new() -> Self {
        unsafe {
            let mgr: Id = msg_send![class!(b"CBCentralManager\0"), alloc];
            let mgr = msg_send![mgr, initWithDelegate: NIL, queue: NIL];
            Self { inner: mgr }
        }
    }

    /// Current Bluetooth state.
    pub fn state(&self) -> ManagerState {
        unsafe { ManagerState::from(msg_send_t![isize; self.inner, state]) }
    }

    /// Whether the central manager is scanning.
    pub fn is_scanning(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isScanning] }
    }

    /// Start scanning for peripherals advertising the given service UUIDs.
    /// Pass empty slice to scan for all peripherals.
    pub fn scan_for_peripherals(&self, service_uuids: &[Uuid]) {
        unsafe {
            let arr = if service_uuids.is_empty() {
                NIL
            } else {
                let ptrs: Vec<Id> = service_uuids.iter().map(|u| u.inner).collect();
                let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
                let f: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
                    core::mem::transmute(objc_msgSend as *const ());
                f(class!(b"NSArray\0") as Id, sel, ptrs.as_ptr(), ptrs.len())
            };
            msg_send_void![self.inner, scanForPeripheralsWithServices: arr, options: NIL];
        }
    }

    /// Stop scanning.
    pub fn stop_scan(&self) {
        unsafe { msg_send_void![self.inner, stopScan]; }
    }

    /// Connect to a peripheral (by raw pointer from delegate callback).
    pub fn connect(&self, peripheral: Id) {
        unsafe { msg_send_void![self.inner, connectPeripheral: peripheral, options: NIL]; }
    }

    /// Cancel a pending or active connection.
    pub fn cancel_connection(&self, peripheral: Id) {
        unsafe { msg_send_void![self.inner, cancelPeripheralConnection: peripheral]; }
    }

    /// List of currently connected peripherals for the given service UUIDs.
    pub fn retrieve_connected(&self, service_uuids: &[Uuid]) -> Vec<Id> {
        unsafe {
            let ptrs: Vec<Id> = service_uuids.iter().map(|u| u.inner).collect();
            let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(class!(b"NSArray\0") as Id, sel, ptrs.as_ptr(), ptrs.len());
            let results: Id = msg_send![self.inner,
                retrieveConnectedPeripheralsWithServices: arr];
            if results.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; results, count];
            (0..count).map(|i| msg_send![results, objectAtIndex: i]).collect()
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for CentralManager { fn default() -> Self { Self::new() } }
impl Drop for CentralManager {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── PeripheralManager ───────────────────────────────────────────────────────

/// Wraps `CBPeripheralManager` for advertising services.
pub struct PeripheralManager { inner: Id }

impl PeripheralManager {
    pub fn new() -> Self {
        unsafe {
            let mgr: Id = msg_send![class!(b"CBPeripheralManager\0"), alloc];
            let mgr = msg_send![mgr, initWithDelegate: NIL, queue: NIL];
            Self { inner: mgr }
        }
    }

    pub fn state(&self) -> ManagerState {
        unsafe { ManagerState::from(msg_send_t![isize; self.inner, state]) }
    }

    pub fn is_advertising(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAdvertising] }
    }

    pub fn stop_advertising(&self) {
        unsafe { msg_send_void![self.inner, stopAdvertising]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for PeripheralManager { fn default() -> Self { Self::new() } }
impl Drop for PeripheralManager {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization() { let _ = authorization(); }

    #[test]
    fn test_uuid() {
        let u = Uuid::from_string("180D");
        assert_eq!(u.string(), "180D");
    }

    #[test]
    fn test_uuid_clone() {
        let u = Uuid::from_string("180F");
        let u2 = u.clone();
        assert_eq!(u2.string(), "180F");
    }

    #[test]
    fn test_central_manager() {
        let cm = CentralManager::new();
        let _ = cm.state();
        assert!(!cm.is_scanning());
    }
}
