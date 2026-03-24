//! Apple PushKit — VoIP push notifications from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 8+.
//!
//! ```ignore
//! let types = vec!["PKPushTypeVoIP"];
//! // In real use, create a PKPushRegistry with delegate for push handling
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"PKPushRegistry\0").is_null() }
}

/// Push type constants.
pub mod push_type {
    pub const VOIP: &str = "PKPushTypeVoIP";
    pub const COMPLICATION: &str = "PKPushTypeComplication";
    pub const FILE_PROVIDER: &str = "PKPushTypeFileProvider";
}

/// Wraps `PKPushRegistry`.
pub struct PushRegistry { inner: Id }

impl PushRegistry {
    /// Create a registry on the main queue.
    pub fn new() -> Self {
        unsafe {
            extern "C" { fn dispatch_get_main_queue() -> Id; }
            let r: Id = msg_send![class!(b"PKPushRegistry\0"), alloc];
            let r = msg_send![r, initWithQueue: dispatch_get_main_queue()];
            Self { inner: r }
        }
    }

    /// Set desired push types (e.g. `&["PKPushTypeVoIP"]`).
    pub fn set_desired_push_types(&self, types: &[&str]) {
        unsafe {
            let ns_strs: Vec<Id> = types.iter().map(|t| nsstring(t)).collect();
            let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(class!(b"NSArray\0") as Id, sel, ns_strs.as_ptr(), ns_strs.len());
            let set: Id = msg_send![class!(b"NSSet\0"), setWithArray: arr];
            msg_send_void![self.inner, setDesiredPushTypes: set];
            for s in &ns_strs { CFRelease(*s as CFTypeRef); }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for PushRegistry { fn default() -> Self { Self::new() } }
impl Drop for PushRegistry { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }
