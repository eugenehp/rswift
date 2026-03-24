//! Apple ReplayKit — screen recording and broadcast from Rust.
//!
//! **Platform:** macOS 11+, iOS 9+, tvOS 10+.
//!
//! ```ignore
//! let recorder = replaykit::ScreenRecorder::shared();
//! println!("Available: {}", recorder.is_available());
//! println!("Recording: {}", recorder.is_recording());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Wraps `RPScreenRecorder`.
pub struct ScreenRecorder { inner: Id }

impl ScreenRecorder {
    /// The shared screen recorder.
    pub fn shared() -> Self {
        Self { inner: unsafe { msg_send![class!(b"RPScreenRecorder\0"), sharedRecorder] } }
    }

    /// Whether recording is available on this device.
    pub fn is_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAvailable] }
    }

    /// Whether a recording is currently in progress.
    pub fn is_recording(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isRecording] }
    }

    /// Whether the microphone is enabled for recording.
    pub fn is_microphone_enabled(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isMicrophoneEnabled] }
    }

    /// Enable/disable microphone capture.
    pub fn set_microphone_enabled(&self, enabled: bool) {
        unsafe { msg_send_void![self.inner, setMicrophoneEnabled: enabled as u8]; }
    }

    /// Whether the camera is enabled for recording.
    pub fn is_camera_enabled(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isCameraEnabled] }
    }

    /// Enable/disable camera capture.
    pub fn set_camera_enabled(&self, enabled: bool) {
        unsafe { msg_send_void![self.inner, setCameraEnabled: enabled as u8]; }
    }

    /// Stop the current recording discarding the result.
    pub fn discard_recording(&self) {
        unsafe { msg_send_void![self.inner, discardRecordingWithHandler: NIL]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder() {
        let r = ScreenRecorder::shared();
        let _ = r.is_available();
        assert!(!r.is_recording());
    }
}
