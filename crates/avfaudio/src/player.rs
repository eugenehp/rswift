//! `AVAudioPlayer` — simple file-based audio playback.

use apple_objc_sys::*;
use crate::*;
use crate::AudioResult;
use std::fmt;

/// Simple audio player for files.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioplayer>
pub struct AudioPlayer { h: Id }

unsafe impl Send for AudioPlayer {}

impl AudioPlayer {
    /// Open an audio file. Returns a typed error if the file can't be opened.
    ///
    /// ```rust,ignore
    /// let player = AudioPlayer::open("song.mp3")?;
    /// player.play();
    /// ```
    pub fn open(path: &str) -> AudioResult<Self> {
        Self::new(path).ok_or_else(|| crate::anyhow!("failed to open audio file: {}", path))
    }

    /// Create a player for the file at `path`. Returns `None` if the file can't be opened.
    pub fn new(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let p = msg_send![class!(b"AVAudioPlayer\0"), alloc];
            let p = msg_send![p, initWithContentsOfURL: url, error: NIL];
            if p.is_null() { None } else { Some(Self { h: p }) }
        }
    }

    /// Create a player from `NSData`.
    pub fn from_data(data: Id) -> Option<Self> {
        unsafe {
            let p = msg_send![class!(b"AVAudioPlayer\0"), alloc];
            let p = msg_send![p, initWithData: data, error: NIL];
            if p.is_null() { None } else { Some(Self { h: p }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    // ── Playback control ────────────────────────────────────────────────

    /// Prepare to play (preload buffers).
    pub fn prepare_to_play(&self) -> bool { unsafe { msg_send_bool(self.h, b"prepareToPlay\0") } }

    /// Start playback. Returns `true` on success.
    pub fn play(&self) -> bool { unsafe { msg_send_t![bool; self.h, play] } }

    /// Play at a specific time.
    pub fn play_at_time(&self, time: f64) -> bool {
        unsafe {
            let sel = sel_registerName(b"playAtTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, time)
        }
    }

    /// Pause playback.
    pub fn pause(&self) { unsafe { msg_send_void![self.h, pause]; } }

    /// Stop playback.
    pub fn stop(&self) { unsafe { msg_send_void![self.h, stop]; } }

    /// Whether the player is currently playing.
    pub fn is_playing(&self) -> bool { unsafe { msg_send_bool(self.h, b"isPlaying\0") } }

    // ── Properties ──────────────────────────────────────────────────────

    /// Duration of the audio in seconds.
    pub fn duration(&self) -> f64 { unsafe { msg_send_f64(self.h, b"duration\0") } }

    /// Number of audio channels.
    pub fn number_of_channels(&self) -> usize { unsafe { msg_send_usize(self.h, b"numberOfChannels\0") } }

    /// Current playback time in seconds.
    pub fn current_time(&self) -> f64 { unsafe { msg_send_f64(self.h, b"currentTime\0") } }

    /// Seek to a specific time.
    pub fn set_current_time(&self, t: f64) {
        unsafe {
            let sel = sel_registerName(b"setCurrentTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, t);
        }
    }

    /// Get playback volume [0.0, 1.0].
    pub fn volume(&self) -> f32 { unsafe { msg_send_f32(self.h, b"volume\0") } }

    /// Set playback volume [0.0, 1.0].
    pub fn set_volume(&self, v: f32) { unsafe { msg_send_void![self.h, setVolume: v]; } }

    /// Set volume with fade duration.
    pub fn set_volume_fade(&self, v: f32, duration: f64) {
        unsafe {
            let sel = sel_registerName(b"setVolume:fadeDuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, v, duration);
        }
    }

    /// Set number of loops (-1 = infinite, 0 = no loop, N = loop N times).
    pub fn set_loops(&self, n: isize) { unsafe { msg_send_void![self.h, setNumberOfLoops: n]; } }

    /// Get number of loops.
    pub fn number_of_loops(&self) -> isize { unsafe { msg_send_isize(self.h, b"numberOfLoops\0") } }

    // ── Metering ────────────────────────────────────────────────────────

    /// Whether metering is enabled.
    pub fn is_metering_enabled(&self) -> bool { unsafe { msg_send_bool(self.h, b"isMeteringEnabled\0") } }

    /// Enable or disable metering.
    pub fn set_metering_enabled(&self, enabled: bool) {
        unsafe { msg_send_set_bool(self.h, b"setMeteringEnabled:\0", enabled); }
    }

    /// Update the metering data.
    pub fn update_meters(&self) { unsafe { msg_send_void(self.h, b"updateMeters\0"); } }

    /// Peak power for a channel in decibels.
    pub fn peak_power_for_channel(&self, channel: usize) -> f32 {
        unsafe {
            let sel = sel_registerName(b"peakPowerForChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> f32 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, channel)
        }
    }

    /// Average power for a channel in decibels.
    pub fn average_power_for_channel(&self, channel: usize) -> f32 {
        unsafe {
            let sel = sel_registerName(b"averagePowerForChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> f32 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, channel)
        }
    }

    // ── URL / Data ──────────────────────────────────────────────────────

    /// The URL of the file being played (if any).
    pub fn url(&self) -> Option<String> {
        unsafe {
            let u = msg_send_id(self.h, b"url\0");
            if u.is_null() { return None; }
            let s = msg_send_id(u, b"absoluteString\0");
            nsstring_to_string(s)
        }
    }
}

impl Drop for AudioPlayer { fn drop(&mut self) { unsafe { release(self.h); } } }

impl fmt::Display for AudioPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = if self.is_playing() { "▶" } else { "⏸" };
        let dur = self.duration();
        let min = (dur / 60.0) as u32;
        let sec = (dur % 60.0) as u32;
        let vol = (self.volume() * 100.0) as u32;
        let name = self.url().unwrap_or_else(|| "<data>".into());
        // Extract just the filename from the URL
        let short = name.rsplit('/').next().unwrap_or(&name);
        write!(f, "{icon} {short} — {min}:{sec:02} @ {vol}%")
    }
}

impl fmt::Debug for AudioPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioPlayer")
            .field("playing", &self.is_playing())
            .field("duration", &self.duration())
            .field("current_time", &self.current_time())
            .field("volume", &self.volume())
            .field("channels", &self.number_of_channels())
            .finish()
    }
}
