//! Apple Core Motion — accelerometer, gyroscope, pedometer from Rust.
//!
//! **Platform support:** iOS 4+, visionOS 1+, watchOS 2+.
//! Note: CoreMotion is NOT available on macOS (use IOKit instead).
//!
//! # Quick start
//!
//! ```ignore
//! let mm = coremotion::MotionManager::new();
//! if mm.is_accelerometer_available() {
//!     println!("Accelerometer available");
//! }
//! if coremotion::Pedometer::is_step_counting_available() {
//!     println!("Step counting available");
//! }
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {

    // CoreMotion exists on iOS/watchOS/visionOS, not macOS
    cfg!(any(target_os = "ios", target_os = "watchos", target_os = "xros"))
}

/// Wraps `CMMotionManager`.
pub struct MotionManager {
    inner: Id,
}

impl MotionManager {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"CMMotionManager\0"), new] } }
    }

    pub fn is_accelerometer_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAccelerometerAvailable] }
    }
    pub fn is_gyro_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isGyroAvailable] }
    }
    pub fn is_magnetometer_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isMagnetometerAvailable] }
    }
    pub fn is_device_motion_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isDeviceMotionAvailable] }
    }

    pub fn is_accelerometer_active(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAccelerometerActive] }
    }
    pub fn is_gyro_active(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isGyroActive] }
    }

    /// Set accelerometer update interval in seconds.
    pub fn set_accelerometer_interval(&self, seconds: f64) {
        unsafe {
            let sel = sel_registerName(b"setAccelerometerUpdateInterval:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, seconds);
        }
    }

    /// Start accelerometer updates (poll via `accelerometer_data()`).
    pub fn start_accelerometer_updates(&self) {
        unsafe { msg_send_void![self.inner, startAccelerometerUpdates]; }
    }

    /// Stop accelerometer updates.
    pub fn stop_accelerometer_updates(&self) {
        unsafe { msg_send_void![self.inner, stopAccelerometerUpdates]; }
    }

    /// Read latest accelerometer data: `(x, y, z)` in G.
    /// Returns `None` if not active or no data yet.
    pub fn accelerometer_data(&self) -> Option<(f64, f64, f64)> {
        unsafe {
            let data: Id = msg_send![self.inner, accelerometerData];
            if data.is_null() { return None; }
            let _accel: Id = msg_send![data, acceleration];
            // CMAcceleration is a struct { double x, y, z } returned by value
            // Actually, `acceleration` returns CMAcceleration by value.
            // On ARM64, 3 doubles fit in registers.
            let sel = sel_registerName(b"acceleration\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f64; 3] =
                core::mem::transmute(objc_msgSend as *const ());
            let a = f(data, sel);
            Some((a[0], a[1], a[2]))
        }
    }

    /// Set gyro update interval in seconds.
    pub fn set_gyro_interval(&self, seconds: f64) {
        unsafe {
            let sel = sel_registerName(b"setGyroUpdateInterval:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, seconds);
        }
    }

    pub fn start_gyro_updates(&self) {
        unsafe { msg_send_void![self.inner, startGyroUpdates]; }
    }
    pub fn stop_gyro_updates(&self) {
        unsafe { msg_send_void![self.inner, stopGyroUpdates]; }
    }

    /// Read latest gyro data: `(x, y, z)` in radians/sec.
    pub fn gyro_data(&self) -> Option<(f64, f64, f64)> {
        unsafe {
            let data: Id = msg_send![self.inner, gyroData];
            if data.is_null() { return None; }
            let sel = sel_registerName(b"rotationRate\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f64; 3] =
                core::mem::transmute(objc_msgSend as *const ());
            let r = f(data, sel);
            Some((r[0], r[1], r[2]))
        }
    }
}

impl Default for MotionManager { fn default() -> Self { Self::new() } }
impl Drop for MotionManager { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Pedometer (step counting).
pub struct Pedometer;

impl Pedometer {
    pub fn is_step_counting_available() -> bool {
        unsafe { msg_send_t![bool; class!(b"CMPedometer\0"), isStepCountingAvailable] }
    }
    pub fn is_distance_available() -> bool {
        unsafe { msg_send_t![bool; class!(b"CMPedometer\0"), isDistanceAvailable] }
    }
    pub fn is_floor_counting_available() -> bool {
        unsafe { msg_send_t![bool; class!(b"CMPedometer\0"), isFloorCountingAvailable] }
    }
    pub fn is_pace_available() -> bool {
        unsafe { msg_send_t![bool; class!(b"CMPedometer\0"), isPaceAvailable] }
    }
}

/// Altimeter.
pub struct Altimeter;

impl Altimeter {
    pub fn is_relative_altitude_available() -> bool {
        unsafe { msg_send_t![bool; class!(b"CMAltimeter\0"), isRelativeAltitudeAvailable] }
    }
}
