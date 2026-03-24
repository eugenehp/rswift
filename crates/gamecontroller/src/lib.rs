#![allow(unsafe_op_in_unsafe_fn)]
//! Apple GameController — game controller input from Rust.
//!
//! **Platform:** macOS 10.9+, iOS 7+, tvOS 9+, visionOS 1+.
//!
//! ```ignore
//! let controllers = gamecontroller::connected();
//! for c in &controllers {
//!     println!("{} — battery: {:.0}%", c.vendor_name(), c.battery_level() * 100.0);
//!     if let Some(gp) = c.extended_gamepad() {
//!         println!("  A={} B={} LT={:.2}", gp.button_a, gp.button_b, gp.left_trigger);
//!     }
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── Controller ──────────────────────────────────────────────────────────────

/// A connected game controller.
pub struct Controller { inner: Id }

/// List all currently connected controllers.
pub fn connected() -> Vec<Controller> {
    unsafe {
        let arr: Id = msg_send![class!(b"GCController\0"), controllers];
        if arr.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; arr, count];
        (0..count).map(|i| {
            let c: Id = msg_send![arr, objectAtIndex: i];
            CFRetain(c as CFTypeRef);
            Controller { inner: c }
        }).collect()
    }
}

/// Number of connected controllers.
pub fn connected_count() -> usize {
    unsafe {
        let arr: Id = msg_send![class!(b"GCController\0"), controllers];
        if arr.is_null() { 0 } else { msg_send_t![usize; arr, count] }
    }
}

impl Controller {
    /// Vendor name (e.g. "Xbox Wireless Controller", "DualSense").
    pub fn vendor_name(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, vendorName]).unwrap_or_default() }
    }

    /// Product category.
    pub fn product_category(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, productCategory]).unwrap_or_default() }
    }

    /// Whether this is attached to the device (e.g. MFi controller physically connected).
    pub fn is_attached_to_device(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAttachedToDevice] }
    }

    /// Battery level (0.0 – 1.0), or -1.0 if unavailable.
    pub fn battery_level(&self) -> f32 {
        unsafe {
            let battery: Id = msg_send![self.inner, battery];
            if battery.is_null() { return -1.0; }
            let sel = sel_registerName(b"batteryLevel\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f32 =
                core::mem::transmute(objc_msgSend as *const ());
            f(battery, sel)
        }
    }

    /// Battery state: 0=unknown, 1=discharging, 2=charging, 3=full.
    pub fn battery_state(&self) -> i32 {
        unsafe {
            let battery: Id = msg_send![self.inner, battery];
            if battery.is_null() { return 0; }
            msg_send_t![i32; battery, batteryState]
        }
    }

    /// Whether this controller has an extended gamepad profile.
    pub fn has_extended_gamepad(&self) -> bool {
        unsafe { !msg_send![self.inner, extendedGamepad].is_null() }
    }

    /// Whether this controller has a micro gamepad profile (Apple TV remote).
    pub fn has_micro_gamepad(&self) -> bool {
        unsafe { !msg_send![self.inner, microGamepad].is_null() }
    }

    /// Snapshot of the extended gamepad state.
    pub fn extended_gamepad(&self) -> Option<GamepadState> {
        unsafe {
            let gp: Id = msg_send![self.inner, extendedGamepad];
            if gp.is_null() { return None; }
            Some(read_gamepad(gp))
        }
    }

    /// Controller player index (0–3, or -1 if unset).
    pub fn player_index(&self) -> i32 {
        unsafe { msg_send_t![i32; self.inner, playerIndex] }
    }

    /// Set player index.
    pub fn set_player_index(&self, index: i32) {
        unsafe {
            let sel = sel_registerName(b"setPlayerIndex:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, index);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Controller { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

// ── Gamepad state ───────────────────────────────────────────────────────────

/// Snapshot of an extended gamepad's input state.
#[derive(Debug, Clone, Copy, Default)]
pub struct GamepadState {
    // Face buttons (0.0 or 1.0, or pressure-sensitive 0.0–1.0)
    pub button_a: f32,
    pub button_b: f32,
    pub button_x: f32,
    pub button_y: f32,

    // Shoulder buttons
    pub left_shoulder: f32,
    pub right_shoulder: f32,

    // Triggers
    pub left_trigger: f32,
    pub right_trigger: f32,

    // D-pad
    pub dpad_up: f32,
    pub dpad_down: f32,
    pub dpad_left: f32,
    pub dpad_right: f32,

    // Thumbstick axes (-1.0 to 1.0)
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,

    // Thumbstick buttons (L3/R3)
    pub left_stick_button: f32,
    pub right_stick_button: f32,

    // Menu buttons
    pub button_menu: f32,
    pub button_options: f32,
    pub button_home: f32,
}

unsafe fn read_button(element: Id) -> f32 {
    if element.is_null() { return 0.0; }
    let sel = sel_registerName(b"value\0".as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> f32 =
        core::mem::transmute(objc_msgSend as *const ());
    f(element, sel)
}

unsafe fn read_axis(element: Id, axis: &[u8]) -> f32 {
    if element.is_null() { return 0.0; }
    let sel = sel_registerName(axis.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> f32 =
        core::mem::transmute(objc_msgSend as *const ());
    f(element, sel)
}

unsafe fn read_gamepad(gp: Id) -> GamepadState {
    let mut s = GamepadState::default();

    s.button_a = read_button(msg_send![gp, buttonA]);
    s.button_b = read_button(msg_send![gp, buttonB]);
    s.button_x = read_button(msg_send![gp, buttonX]);
    s.button_y = read_button(msg_send![gp, buttonY]);
    s.left_shoulder = read_button(msg_send![gp, leftShoulder]);
    s.right_shoulder = read_button(msg_send![gp, rightShoulder]);
    s.left_trigger = read_button(msg_send![gp, leftTrigger]);
    s.right_trigger = read_button(msg_send![gp, rightTrigger]);

    let dpad: Id = msg_send![gp, dpad];
    s.dpad_up = read_button(msg_send![dpad, up]);
    s.dpad_down = read_button(msg_send![dpad, down]);
    s.dpad_left = read_button(msg_send![dpad, left]);
    s.dpad_right = read_button(msg_send![dpad, right]);

    let ls: Id = msg_send![gp, leftThumbstick];
    s.left_stick_x = read_axis(ls, b"xAxis\0");
    s.left_stick_y = read_axis(ls, b"yAxis\0");

    let rs: Id = msg_send![gp, rightThumbstick];
    s.right_stick_x = read_axis(rs, b"xAxis\0");
    s.right_stick_y = read_axis(rs, b"yAxis\0");

    s.left_stick_button = read_button(msg_send![gp, leftThumbstickButton]);
    s.right_stick_button = read_button(msg_send![gp, rightThumbstickButton]);
    s.button_menu = read_button(msg_send![gp, buttonMenu]);
    s.button_options = read_button(msg_send![gp, buttonOptions]);
    s.button_home = read_button(msg_send![gp, buttonHome]);

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected() {
        let c = connected();
        println!("Connected controllers: {}", c.len());
        for ctrl in &c {
            println!("  {} ({})", ctrl.vendor_name(), ctrl.product_category());
        }
    }

    #[test]
    fn test_connected_count() {
        let _ = connected_count();
    }

    #[test]
    fn test_gamepad_state_default() {
        let s = GamepadState::default();
        assert_eq!(s.button_a, 0.0);
        assert_eq!(s.left_stick_x, 0.0);
    }
}
