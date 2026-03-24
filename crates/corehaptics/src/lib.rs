#![allow(unsafe_op_in_unsafe_fn)]
//! Apple Core Haptics — haptic feedback engine from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 13+, visionOS 1+.
//!
//! ```ignore
//! if corehaptics::supports_haptics() {
//!     let engine = corehaptics::Engine::new().unwrap();
//!     engine.start().unwrap();
//!     // Play a transient tap
//!     engine.play_transient(1.0, 0.5).unwrap();
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Whether the device supports haptics.
pub fn supports_haptics() -> bool {
    unsafe {
        let cap = msg_send![class!(b"CHHapticEngine\0"), capabilitiesForHardware];
        if cap.is_null() { return false; }
        msg_send_t![bool; cap, supportsHaptics]
    }
}

/// Whether the device supports audio haptics.
pub fn supports_audio() -> bool {
    unsafe {
        let cap = msg_send![class!(b"CHHapticEngine\0"), capabilitiesForHardware];
        if cap.is_null() { return false; }
        msg_send_t![bool; cap, supportsAudio]
    }
}

/// Wraps `CHHapticEngine`.
pub struct Engine { inner: Id }

impl Engine {
    /// Create a new haptic engine.
    pub fn new() -> Result<Self, String> {
        unsafe {
            let mut error: Id = NIL;
            let e: Id = msg_send![class!(b"CHHapticEngine\0"), alloc];
            let sel = sel_registerName(b"initAndReturnError:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let e = f(e, sel, &mut error);
            if e.is_null() {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Unknown error".into())
                } else { "Failed to create engine".into() };
                Err(desc)
            } else {
                Ok(Self { inner: e })
            }
        }
    }

    /// Start the engine.
    pub fn start(&self) -> Result<(), String> {
        unsafe {
            let mut error: Id = NIL;
            let sel = sel_registerName(b"startAndReturnError:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            if f(self.inner, sel, &mut error) {
                Ok(())
            } else {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Start failed".into())
                } else { "Start failed".into() };
                Err(desc)
            }
        }
    }

    /// Stop the engine.
    pub fn stop(&self) {
        unsafe { msg_send_void![self.inner, stopWithCompletionHandler: NIL]; }
    }

    /// Play a transient (tap) haptic event.
    ///
    /// - `intensity`: 0.0 – 1.0
    /// - `sharpness`: 0.0 – 1.0
    pub fn play_transient(&self, intensity: f32, sharpness: f32) -> Result<(), String> {
        unsafe {
            // Build CHHapticEvent
            let i_param = make_event_param(b"HapticIntensity\0", intensity);
            let s_param = make_event_param(b"HapticSharpness\0", sharpness);
            if i_param.is_null() || s_param.is_null() { return Err("Failed to create params".into()); }

            let sel_arr = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
            let f_arr: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let params = [i_param, s_param];
            let arr = f_arr(class!(b"NSArray\0") as Id, sel_arr, params.as_ptr(), 2);

            // CHHapticEventTypeHapticTransient = 1
            let sel_ev = sel_registerName(b"initWithEventType:parameters:relativeTime:\0".as_ptr());
            let f_ev: unsafe extern "C" fn(Id, Sel, isize, Id, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let event = msg_send![class!(b"CHHapticEvent\0"), alloc];
            let event = f_ev(event, sel_ev, 1, arr, 0.0);

            let events = f_arr(class!(b"NSArray\0") as Id, sel_arr, &event as *const Id, 1);

            // Create pattern
            let pattern: Id = msg_send![class!(b"CHHapticPattern\0"), alloc];
            let sel_pat = sel_registerName(b"initWithEvents:parameters:error:\0".as_ptr());
            let f_pat: unsafe extern "C" fn(Id, Sel, Id, Id, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let empty_arr = f_arr(class!(b"NSArray\0") as Id, sel_arr, core::ptr::null(), 0);
            let pattern = f_pat(pattern, sel_pat, events, empty_arr, NIL);

            if pattern.is_null() { return Err("Failed to create pattern".into()); }

            // Create player and start
            let mut error: Id = NIL;
            let sel_player = sel_registerName(b"createPlayerWithPattern:error:\0".as_ptr());
            let f_player: unsafe extern "C" fn(Id, Sel, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let player = f_player(self.inner, sel_player, pattern, &mut error);

            if player.is_null() {
                return Err("Failed to create player".into());
            }

            let sel_start = sel_registerName(b"startAtTime:error:\0".as_ptr());
            let f_start: unsafe extern "C" fn(Id, Sel, f64, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            // 0.0 = immediately
            if f_start(player, sel_start, 0.0, &mut error) {
                Ok(())
            } else {
                Err("Failed to start player".into())
            }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Engine { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

unsafe fn make_event_param(param_id: &[u8], value: f32) -> Id {
    let ns = nsstring(core::str::from_utf8(&param_id[..param_id.len()-1]).unwrap());
    let sel = sel_registerName(b"initWithParameterID:value:\0".as_ptr());
    let f: unsafe extern "C" fn(Id, Sel, Id, f32) -> Id =
        core::mem::transmute(objc_msgSend as *const ());
    let p = msg_send![class!(b"CHHapticEventParameter\0"), alloc];
    let p = f(p, sel, ns, value);
    CFRelease(ns as CFTypeRef);
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities() {
        let h = supports_haptics();
        let a = supports_audio();
        println!("Haptics: {h}, Audio: {a}");
    }
}
