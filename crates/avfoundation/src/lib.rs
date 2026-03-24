//! Apple AVFoundation — media capture, playback, and speech from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! ```ignore
//! let player = avfoundation::Player::new("https://example.com/video.mp4");
//! player.play();
//!
//! let synth = avfoundation::SpeechSynthesizer::new();
//! synth.speak("Hello from Rust!");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Complete ObjC selector constants for AVFoundation classes.
pub mod ffi;

pub fn is_available() -> bool { true }

// ── CMTime C imports ────────────────────────────────────────────────────────
#[repr(C)] #[derive(Clone, Copy)]
struct RawCMTime { value: i64, timescale: i32, flags: u32, epoch: i64 }
#[allow(non_snake_case)]
unsafe extern "C" {
    fn CMTimeGetSeconds(time: RawCMTime) -> f64;
    fn CMTimeMakeWithSeconds(seconds: f64, preferredTimescale: i32) -> RawCMTime;
}

fn make_url(s: &str) -> Id {
    unsafe {
        let ns = nsstring(s);
        let url = if s.starts_with("http://") || s.starts_with("https://") {
            msg_send![class!(b"NSURL\0"), URLWithString: ns]
        } else {
            msg_send![class!(b"NSURL\0"), fileURLWithPath: ns]
        };
        CFRelease(ns as CFTypeRef);
        url
    }
}

// ── Player status ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus { Unknown = 0, ReadyToPlay = 1, Failed = 2 }
impl From<isize> for PlayerStatus {
    fn from(v: isize) -> Self { match v { 1=>Self::ReadyToPlay, 2=>Self::Failed, _=>Self::Unknown } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeControlStatus { Paused = 0, WaitingToPlay = 1, Playing = 2 }
impl From<isize> for TimeControlStatus {
    fn from(v: isize) -> Self { match v { 1=>Self::WaitingToPlay, 2=>Self::Playing, _=>Self::Paused } }
}

// ── Player ──────────────────────────────────────────────────────────────────

pub struct Player { h: Id }

impl Player {
    pub fn new(url: &str) -> Self {
        let u = make_url(url);
        let h = unsafe { msg_send![class!(b"AVPlayer\0"), playerWithURL: u] };
        unsafe { CFRetain(h as CFTypeRef); }
        Self { h }
    }

    pub fn with_item(item: &PlayerItem) -> Self {
        let h = unsafe { msg_send![class!(b"AVPlayer\0"), playerWithPlayerItem: item.h] };
        unsafe { CFRetain(h as CFTypeRef); }
        Self { h }
    }

    pub fn play(&self) { unsafe { msg_send_void![self.h, play]; } }
    pub fn pause(&self) { unsafe { msg_send_void![self.h, pause]; } }
    pub fn rate(&self) -> f32 { unsafe { msg_send_t![f32; self.h, rate] } }
    pub fn set_rate(&self, r: f32) { unsafe { msg_send_void![self.h, setRate: r]; } }
    pub fn volume(&self) -> f32 { unsafe { msg_send_t![f32; self.h, volume] } }
    pub fn set_volume(&self, v: f32) { unsafe { msg_send_void![self.h, setVolume: v]; } }
    pub fn is_muted(&self) -> bool { unsafe { msg_send_t![bool; self.h, isMuted] } }
    pub fn set_muted(&self, m: bool) { unsafe { msg_send_void![self.h, setMuted: m as u8]; } }
    pub fn status(&self) -> PlayerStatus { PlayerStatus::from(unsafe { msg_send_t![isize; self.h, status] }) }

    pub fn time_control_status(&self) -> TimeControlStatus {
        TimeControlStatus::from(unsafe { msg_send_t![isize; self.h, timeControlStatus] })
    }

    pub fn current_time(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(ffi::player::SEL_CURRENT_TIME.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> RawCMTime = core::mem::transmute(objc_msgSend as *const ());
            CMTimeGetSeconds(f(self.h, sel))
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            let item: Id = msg_send![self.h, currentItem];
            if item.is_null() { return 0.0; }
            let sel = sel_registerName(ffi::player_item::SEL_DURATION.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> RawCMTime = core::mem::transmute(objc_msgSend as *const ());
            let dur = f(item, sel);
            if dur.flags & 1 != 0 && dur.flags & 16 == 0 { CMTimeGetSeconds(dur) } else { 0.0 }
        }
    }

    pub fn seek(&self, seconds: f64) {
        unsafe {
            let t = CMTimeMakeWithSeconds(seconds, 600);
            let sel = sel_registerName(ffi::player::SEL_SEEK_TO_TIME.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, RawCMTime) = core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, t);
        }
    }

    pub fn replace_current_item(&self, item: &PlayerItem) {
        unsafe { msg_send_void![self.h, replaceCurrentItemWithPlayerItem: item.h]; }
    }

    pub fn allows_external_playback(&self) -> bool {
        unsafe { msg_send_t![bool; self.h, allowsExternalPlayback] }
    }

    pub fn is_external_playback_active(&self) -> bool {
        unsafe { msg_send_t![bool; self.h, isExternalPlaybackActive] }
    }

    pub fn as_ptr(&self) -> Id { self.h }
}
impl Drop for Player { fn drop(&mut self) { unsafe { CFRelease(self.h as CFTypeRef); } } }

// ── PlayerItem ──────────────────────────────────────────────────────────────

pub struct PlayerItem { h: Id }

impl PlayerItem {
    pub fn new(url: &str) -> Self {
        let u = make_url(url);
        let h = unsafe { msg_send![class!(b"AVPlayerItem\0"), playerItemWithURL: u] };
        unsafe { CFRetain(h as CFTypeRef); }
        Self { h }
    }

    pub fn with_asset(asset: &Asset) -> Self {
        let h = unsafe { msg_send![class!(b"AVPlayerItem\0"), playerItemWithAsset: asset.h] };
        unsafe { CFRetain(h as CFTypeRef); }
        Self { h }
    }

    pub fn status(&self) -> PlayerStatus { PlayerStatus::from(unsafe { msg_send_t![isize; self.h, status] }) }

    pub fn duration(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(ffi::player_item::SEL_DURATION.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> RawCMTime = core::mem::transmute(objc_msgSend as *const ());
            let d = f(self.h, sel);
            if d.flags & 1 != 0 && d.flags & 16 == 0 { CMTimeGetSeconds(d) } else { 0.0 }
        }
    }

    pub fn is_playback_likely_to_keep_up(&self) -> bool { unsafe { msg_send_t![bool; self.h, isPlaybackLikelyToKeepUp] } }
    pub fn is_playback_buffer_empty(&self) -> bool { unsafe { msg_send_t![bool; self.h, isPlaybackBufferEmpty] } }
    pub fn is_playback_buffer_full(&self) -> bool { unsafe { msg_send_t![bool; self.h, isPlaybackBufferFull] } }
    pub fn can_play_reverse(&self) -> bool { unsafe { msg_send_t![bool; self.h, canPlayReverse] } }
    pub fn can_play_fast_forward(&self) -> bool { unsafe { msg_send_t![bool; self.h, canPlayFastForward] } }

    pub fn as_ptr(&self) -> Id { self.h }
}
impl Drop for PlayerItem { fn drop(&mut self) { unsafe { CFRelease(self.h as CFTypeRef); } } }

// ── Asset ───────────────────────────────────────────────────────────────────

pub struct Asset { h: Id }

impl Asset {
    pub fn new(url: &str) -> Self {
        let u = make_url(url);
        let h = unsafe { msg_send![class!(b"AVURLAsset\0"), URLAssetWithURL: u, options: NIL] };
        unsafe { CFRetain(h as CFTypeRef); }
        Self { h }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(ffi::asset::SEL_DURATION.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> RawCMTime = core::mem::transmute(objc_msgSend as *const ());
            CMTimeGetSeconds(f(self.h, sel))
        }
    }

    pub fn is_playable(&self) -> bool { unsafe { msg_send_t![bool; self.h, isPlayable] } }
    pub fn is_exportable(&self) -> bool { unsafe { msg_send_t![bool; self.h, isExportable] } }
    pub fn is_readable(&self) -> bool { unsafe { msg_send_t![bool; self.h, isReadable] } }
    pub fn has_protected_content(&self) -> bool { unsafe { msg_send_t![bool; self.h, hasProtectedContent] } }

    pub fn track_count(&self) -> usize {
        unsafe {
            let tracks: Id = msg_send![self.h, tracks];
            if tracks.is_null() { 0 } else { msg_send_t![usize; tracks, count] }
        }
    }

    pub fn as_ptr(&self) -> Id { self.h }
}
impl Drop for Asset { fn drop(&mut self) { unsafe { CFRelease(self.h as CFTypeRef); } } }

// ── Speech ──────────────────────────────────────────────────────────────────

/// Text-to-speech synthesizer.
pub struct SpeechSynthesizer { h: Id }

impl SpeechSynthesizer {
    pub fn new() -> Self {
        Self { h: unsafe { msg_send![class!(b"AVSpeechSynthesizer\0"), new] } }
    }

    /// Speak a string with default voice and rate.
    pub fn speak(&self, text: &str) {
        unsafe {
            let ns = nsstring(text);
            let utt: Id = msg_send![class!(b"AVSpeechUtterance\0"), speechUtteranceWithString: ns];
            CFRelease(ns as CFTypeRef);
            msg_send_void![self.h, speakUtterance: utt];
        }
    }

    /// Speak an utterance with custom settings.
    pub fn speak_utterance(&self, utterance: &SpeechUtterance) {
        unsafe { msg_send_void![self.h, speakUtterance: utterance.h]; }
    }

    pub fn is_speaking(&self) -> bool { unsafe { msg_send_t![bool; self.h, isSpeaking] } }
    pub fn is_paused(&self) -> bool { unsafe { msg_send_t![bool; self.h, isPaused] } }

    /// Stop immediately (0) or at word boundary (1).
    pub fn stop(&self, boundary: isize) {
        unsafe {
            let sel = sel_registerName(ffi::speech_synthesizer::SEL_STOP_SPEAKING_AT_BOUNDARY.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> bool = core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, boundary);
        }
    }

    pub fn pause(&self, boundary: isize) {
        unsafe {
            let sel = sel_registerName(ffi::speech_synthesizer::SEL_PAUSE_SPEAKING_AT_BOUNDARY.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> bool = core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, boundary);
        }
    }

    pub fn continue_speaking(&self) -> bool {
        unsafe { msg_send_t![bool; self.h, continueSpeaking] }
    }

    pub fn as_ptr(&self) -> Id { self.h }
}
impl Default for SpeechSynthesizer { fn default() -> Self { Self::new() } }
impl Drop for SpeechSynthesizer { fn drop(&mut self) { unsafe { CFRelease(self.h as CFTypeRef); } } }

/// Speech boundary constants.
pub mod speech_boundary {
    pub const IMMEDIATE: isize = 0;
    pub const WORD: isize = 1;
}

/// A speech utterance with configurable voice, rate, pitch, volume.
pub struct SpeechUtterance { h: Id }

impl SpeechUtterance {
    pub fn new(text: &str) -> Self {
        unsafe {
            let ns = nsstring(text);
            let h: Id = msg_send![class!(b"AVSpeechUtterance\0"), speechUtteranceWithString: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(h as CFTypeRef);
            Self { h }
        }
    }

    pub fn set_rate(&self, rate: f32) { unsafe { msg_send_void![self.h, setRate: rate]; } }
    pub fn set_pitch_multiplier(&self, pitch: f32) { unsafe { msg_send_void![self.h, setPitchMultiplier: pitch]; } }
    pub fn set_volume(&self, vol: f32) { unsafe { msg_send_void![self.h, setVolume: vol]; } }

    pub fn set_voice_language(&self, lang: &str) {
        unsafe {
            let ns = nsstring(lang);
            let voice: Id = msg_send![class!(b"AVSpeechSynthesisVoice\0"), voiceWithLanguage: ns];
            CFRelease(ns as CFTypeRef);
            if !voice.is_null() {
                msg_send_void![self.h, setVoice: voice];
            }
        }
    }

    pub fn set_pre_utterance_delay(&self, seconds: f64) {
        unsafe {
            let sel = sel_registerName(ffi::speech_utterance::SEL_SET_PRE_UTTERANCE_DELAY.as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) = core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, seconds);
        }
    }

    pub fn as_ptr(&self) -> Id { self.h }
}
impl Drop for SpeechUtterance { fn drop(&mut self) { unsafe { CFRelease(self.h as CFTypeRef); } } }

/// List available speech voices.
pub fn speech_voices() -> Vec<(String, String)> {
    unsafe {
        let arr: Id = msg_send![class!(b"AVSpeechSynthesisVoice\0"), speechVoices];
        if arr.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; arr, count];
        (0..count).filter_map(|i| {
            let v: Id = msg_send![arr, objectAtIndex: i];
            let name = nsstring_to_string(msg_send![v, name])?;
            let lang = nsstring_to_string(msg_send![v, language])?;
            Some((name, lang))
        }).collect()
    }
}

/// List available export presets for an asset.
pub fn export_presets_for_asset(asset: &Asset) -> Vec<String> {
    unsafe {
        let arr: Id = msg_send![class!(b"AVAssetExportSession\0"), exportPresetsCompatibleWithAsset: asset.h];
        if arr.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; arr, count];
        (0..count).filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i])).collect()
    }
}

/// Camera/microphone authorization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Restricted = 1, Denied = 2, Authorized = 3 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self { match v { 1=>Self::Restricted, 2=>Self::Denied, 3=>Self::Authorized, _=>Self::NotDetermined } }
}

/// Check camera authorization.
pub fn camera_authorization() -> AuthorizationStatus {
    unsafe {
        let vide = nsstring(ffi::media_types::VIDEO);
        let sel = sel_registerName(ffi::capture_device::SEL_AUTHORIZATION_STATUS_FOR_MEDIA_TYPE.as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, Id) -> isize = core::mem::transmute(objc_msgSend as *const ());
        let s = f(class!(b"AVCaptureDevice\0") as Id, sel, vide);
        CFRelease(vide as CFTypeRef);
        AuthorizationStatus::from(s)
    }
}

/// Check microphone authorization.
pub fn microphone_authorization() -> AuthorizationStatus {
    unsafe {
        let soun = nsstring(ffi::media_types::AUDIO);
        let sel = sel_registerName(ffi::capture_device::SEL_AUTHORIZATION_STATUS_FOR_MEDIA_TYPE.as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, Id) -> isize = core::mem::transmute(objc_msgSend as *const ());
        let s = f(class!(b"AVCaptureDevice\0") as Id, sel, soun);
        CFRelease(soun as CFTypeRef);
        AuthorizationStatus::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_voices() {
        let voices = speech_voices();
        assert!(!voices.is_empty(), "Should have at least one speech voice");
        assert!(voices.iter().any(|(_, lang)| lang.starts_with("en")));
    }

    #[test]
    fn test_camera_auth() {
        let _ = camera_authorization();
    }

    #[test]
    fn test_speech_utterance() {
        let utt = SpeechUtterance::new("Hello");
        utt.set_rate(0.5);
        utt.set_pitch_multiplier(1.2);
        utt.set_volume(0.8);
        utt.set_voice_language("en-US");
    }
}
