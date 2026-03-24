//! `AVAudioRecorder` — audio recording to file.

use apple_objc_sys::*;
use crate::*;
use crate::AudioResult;
use std::fmt;

/// Audio recorder.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiorecorder>
pub struct AudioRecorder { h: Id }

unsafe impl Send for AudioRecorder {}

impl AudioRecorder {
    /// Create a recorder targeting `path` with the given settings `NSDictionary*`.
    ///
    /// For convenience, use `AudioRecorder::with_url_settings(url_id, settings_id)`.
    pub fn from_raw_url_settings(url: Id, settings: Id) -> Option<Self> {
        unsafe {
            let r = msg_send![class!(b"AVAudioRecorder\0"), alloc];
            let r = msg_send![r, initWithURL: url, settings: settings, error: NIL];
            if r.is_null() { None } else { Some(Self { h: r }) }
        }
    }

    /// Create a recorder targeting `path` with settings (typed error).
    pub fn new(url: Id, settings: Id) -> AudioResult<Self> {
        unsafe {
            let r = msg_send![class!(b"AVAudioRecorder\0"), alloc];
            let mut err: Id = NIL;
            let sel = sel_registerName(b"initWithURL:settings:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(r, sel, url, settings, &mut err);
            if h.is_null() {
                Err(crate::anyhow!("failed to create audio recorder"))
            } else {
                Ok(Self { h })
            }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    // ── Recording control ───────────────────────────────────────────────

    /// Prepare to record (create the file, allocate buffers).
    pub fn prepare_to_record(&self) -> bool { unsafe { msg_send_bool(self.h, b"prepareToRecord\0") } }

    /// Start recording.
    pub fn record(&self) -> bool { unsafe { msg_send_bool(self.h, b"record\0") } }

    /// Record for a maximum duration (seconds).
    pub fn record_for_duration(&self, duration: f64) -> bool {
        unsafe {
            let sel = sel_registerName(b"recordForDuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, duration)
        }
    }

    /// Record starting at a specific time.
    pub fn record_at_time(&self, time: f64) -> bool {
        unsafe {
            let sel = sel_registerName(b"recordAtTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, time)
        }
    }

    /// Pause recording.
    pub fn pause(&self) { unsafe { msg_send_void(self.h, b"pause\0"); } }

    /// Stop recording.
    pub fn stop(&self) { unsafe { msg_send_void(self.h, b"stop\0"); } }

    /// Whether the recorder is currently recording.
    pub fn is_recording(&self) -> bool { unsafe { msg_send_bool(self.h, b"isRecording\0") } }

    /// Delete the recorded file.
    pub fn delete_recording(&self) -> bool { unsafe { msg_send_bool(self.h, b"deleteRecording\0") } }

    // ── Properties ──────────────────────────────────────────────────────

    /// Current recording time.
    pub fn current_time(&self) -> f64 { unsafe { msg_send_f64(self.h, b"currentTime\0") } }

    /// URL of the recording.
    pub fn url(&self) -> Option<String> {
        unsafe {
            let u = msg_send_id(self.h, b"url\0");
            if u.is_null() { return None; }
            let s = msg_send_id(u, b"absoluteString\0");
            nsstring_to_string(s)
        }
    }

    // ── Metering ────────────────────────────────────────────────────────

    pub fn is_metering_enabled(&self) -> bool { unsafe { msg_send_bool(self.h, b"isMeteringEnabled\0") } }

    pub fn set_metering_enabled(&self, enabled: bool) {
        unsafe { msg_send_set_bool(self.h, b"setMeteringEnabled:\0", enabled); }
    }

    pub fn update_meters(&self) { unsafe { msg_send_void(self.h, b"updateMeters\0"); } }

    pub fn peak_power_for_channel(&self, channel: usize) -> f32 {
        unsafe {
            let sel = sel_registerName(b"peakPowerForChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> f32 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, channel)
        }
    }

    pub fn average_power_for_channel(&self, channel: usize) -> f32 {
        unsafe {
            let sel = sel_registerName(b"averagePowerForChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> f32 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, channel)
        }
    }
}

impl Drop for AudioRecorder { fn drop(&mut self) { unsafe { release(self.h); } } }

impl fmt::Display for AudioRecorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.is_recording() { "⏹" } else { "◯" };
        let dur = self.current_time();
        let min = (dur / 60.0) as u32;
        let sec = (dur % 60.0) as u32;
        let url = self.url().unwrap_or_else(|| "<unknown>".into());
        let short = url.rsplit('/').next().unwrap_or(&url);
        write!(f, "{icon} {short} @ {min}:{sec:02}")
    }
}

impl fmt::Debug for AudioRecorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioRecorder")
            .field("recording", &self.is_recording())
            .field("current_time", &self.current_time())
            .field("url", &self.url())
            .finish()
    }
}
