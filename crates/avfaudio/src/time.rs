//! `AVAudioTime` — represent a moment in time.

use apple_objc_sys::*;
use crate::*;
use std::fmt;

/// A moment in time (host time, sample time, or both).
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiotime>
pub struct AudioTime { h: Id }

unsafe impl Send for AudioTime {}
unsafe impl Sync for AudioTime {}

impl AudioTime {
    pub fn as_raw(&self) -> Id { self.h }

    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }

    /// Create from a host time.
    pub fn with_host_time(host_time: u64) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioTime\0"), alloc];
            let sel = sel_registerName(b"initWithHostTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { h: f(obj, sel, host_time) }
        }
    }

    /// Create from a sample time at a given rate.
    pub fn with_sample_time(sample_time: AudioFramePosition, sample_rate: f64) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioTime\0"), alloc];
            let sel = sel_registerName(b"initWithSampleTime:atRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { h: f(obj, sel, sample_time, sample_rate) }
        }
    }

    /// Create from both host time and sample time.
    pub fn with_host_and_sample_time(host_time: u64, sample_time: AudioFramePosition, sample_rate: f64) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioTime\0"), alloc];
            let sel = sel_registerName(b"initWithHostTime:sampleTime:atRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u64, i64, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { h: f(obj, sel, host_time, sample_time, sample_rate) }
        }
    }

    // ── Class methods ───────────────────────────────────────────────────

    /// Convert seconds to host time ticks.
    pub fn host_time_for_seconds(seconds: f64) -> u64 {
        unsafe {
            let cls = class!(b"AVAudioTime\0") as Id;
            let sel = sel_registerName(b"hostTimeForSeconds:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> u64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(cls, sel, seconds)
        }
    }

    /// Convert host time ticks to seconds.
    pub fn seconds_for_host_time(host_time: u64) -> f64 {
        unsafe {
            let cls = class!(b"AVAudioTime\0") as Id;
            let sel = sel_registerName(b"secondsForHostTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u64) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(cls, sel, host_time)
        }
    }

    // ── Properties ──────────────────────────────────────────────────────

    /// Whether `host_time` is valid.
    pub fn is_host_time_valid(&self) -> bool { unsafe { msg_send_bool(self.h, b"isHostTimeValid\0") } }

    /// The host time.
    pub fn host_time(&self) -> u64 {
        unsafe {
            let sel = sel_registerName(b"hostTime\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> u64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel)
        }
    }

    /// Whether `sample_time` and `sample_rate` are valid.
    pub fn is_sample_time_valid(&self) -> bool { unsafe { msg_send_bool(self.h, b"isSampleTimeValid\0") } }

    /// The sample time.
    pub fn sample_time(&self) -> AudioFramePosition {
        unsafe {
            let sel = sel_registerName(b"sampleTime\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel)
        }
    }

    /// The sample rate.
    pub fn sample_rate(&self) -> f64 { unsafe { msg_send_f64(self.h, b"sampleRate\0") } }

    /// Extrapolate time from an anchor.
    pub fn extrapolate_from_anchor(&self, anchor: &AudioTime) -> Option<AudioTime> {
        unsafe {
            let h = msg_send![self.h, extrapolateTimeFromAnchor: anchor.h];
            if h.is_null() { None } else { Some(AudioTime { h }) }
        }
    }
}

impl Drop for AudioTime { fn drop(&mut self) { unsafe { release(self.h); } } }

impl fmt::Display for AudioTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_sample_time_valid() {
            let samples = self.sample_time();
            let rate = self.sample_rate();
            let secs = samples as f64 / rate;
            write!(f, "{:.3}s (#{} @ {:.0}Hz)", secs, samples, rate)
        } else if self.is_host_time_valid() {
            write!(f, "host-time={}", self.host_time())
        } else {
            write!(f, "<invalid>")
        }
    }
}

impl fmt::Debug for AudioTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioTime")
            .field("host_time_valid", &self.is_host_time_valid())
            .field("host_time", &self.host_time())
            .field("sample_time_valid", &self.is_sample_time_valid())
            .field("sample_time", &self.sample_time())
            .field("sample_rate", &self.sample_rate())
            .finish()
    }
}
