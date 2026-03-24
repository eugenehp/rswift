//! `AVSpeechSynthesizer` and `AVSpeechUtterance` — text-to-speech.

use apple_objc_sys::*;
use crate::*;

// ── AVSpeechUtterance ───────────────────────────────────────────────────────

/// A chunk of text to be spoken.
///
/// <https://developer.apple.com/documentation/avfaudio/avspeechutterance>
pub struct SpeechUtterance { h: Id }

unsafe impl Send for SpeechUtterance {}

impl SpeechUtterance {
    /// Create an utterance with the given text.
    pub fn new(text: &str) -> Self {
        unsafe {
            let ns = nsstring(text);
            let obj = msg_send![class!(b"AVSpeechUtterance\0"), alloc];
            let h = msg_send![obj, initWithString: ns];
            CFRelease(ns as CFTypeRef);
            Self { h }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Speaking rate [0.0, 1.0] where 0.5 is default.
    pub fn rate(&self) -> f32 { unsafe { msg_send_f32(self.h, b"rate\0") } }
    pub fn set_rate(&self, rate: f32) {
        unsafe {
            let sel = sel_registerName(b"setRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, rate);
        }
    }

    /// Pitch multiplier [0.5, 2.0] where 1.0 is default.
    pub fn pitch_multiplier(&self) -> f32 { unsafe { msg_send_f32(self.h, b"pitchMultiplier\0") } }
    pub fn set_pitch_multiplier(&self, pitch: f32) {
        unsafe {
            let sel = sel_registerName(b"setPitchMultiplier:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, pitch);
        }
    }

    /// Volume [0.0, 1.0].
    pub fn volume(&self) -> f32 { unsafe { msg_send_f32(self.h, b"volume\0") } }
    pub fn set_volume(&self, vol: f32) {
        unsafe {
            let sel = sel_registerName(b"setVolume:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, vol);
        }
    }

    /// Pre-utterance delay in seconds.
    pub fn pre_utterance_delay(&self) -> f64 { unsafe { msg_send_f64(self.h, b"preUtteranceDelay\0") } }
    pub fn set_pre_utterance_delay(&self, delay: f64) {
        unsafe {
            let sel = sel_registerName(b"setPreUtteranceDelay:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, delay);
        }
    }

    /// Post-utterance delay in seconds.
    pub fn post_utterance_delay(&self) -> f64 { unsafe { msg_send_f64(self.h, b"postUtteranceDelay\0") } }
    pub fn set_post_utterance_delay(&self, delay: f64) {
        unsafe {
            let sel = sel_registerName(b"setPostUtteranceDelay:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, delay);
        }
    }
}

impl Drop for SpeechUtterance { fn drop(&mut self) { unsafe { release(self.h); } } }

// ── AVSpeechSynthesisVoice ──────────────────────────────────────────────────

/// A voice for speech synthesis.
pub struct SpeechSynthesisVoice { h: Id }

unsafe impl Send for SpeechSynthesisVoice {}

impl SpeechSynthesisVoice {
    /// Get a voice for the given language code (e.g. "en-US").
    pub fn with_language(language: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(language);
            let h = msg_send![class!(b"AVSpeechSynthesisVoice\0"), voiceWithLanguage: ns];
            CFRelease(ns as CFTypeRef);
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    /// Get a voice by identifier.
    pub fn with_identifier(identifier: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(identifier);
            let h = msg_send![class!(b"AVSpeechSynthesisVoice\0"), voiceWithIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// The voice's language.
    pub fn language(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send_id(self.h, b"language\0")) }
    }

    /// The voice's name.
    pub fn name(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send_id(self.h, b"name\0")) }
    }

    /// The voice's identifier.
    pub fn identifier(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send_id(self.h, b"identifier\0")) }
    }
}

// ── AVSpeechSynthesizer ────────────────────────────────────────────────────

/// Text-to-speech synthesizer.
///
/// <https://developer.apple.com/documentation/avfaudio/avspeechsynthesizer>
pub struct SpeechSynthesizer { h: Id }

unsafe impl Send for SpeechSynthesizer {}

impl SpeechSynthesizer {
    /// Create a new synthesizer.
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVSpeechSynthesizer\0"), new] };
        Self { h }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Speak an utterance.
    pub fn speak(&self, utterance: &SpeechUtterance) {
        unsafe { msg_send_void_id(self.h, b"speakUtterance:\0", utterance.as_raw()); }
    }

    /// Stop speaking.
    pub fn stop_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe {
            let sel = sel_registerName(b"stopSpeakingAtBoundary:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, boundary as isize)
        }
    }

    /// Pause speaking.
    pub fn pause_speaking(&self, boundary: SpeechBoundary) -> bool {
        unsafe {
            let sel = sel_registerName(b"pauseSpeakingAtBoundary:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, boundary as isize)
        }
    }

    /// Continue speaking after a pause.
    pub fn continue_speaking(&self) -> bool { unsafe { msg_send_bool(self.h, b"continueSpeaking\0") } }

    /// Whether the synthesizer is speaking.
    pub fn is_speaking(&self) -> bool { unsafe { msg_send_bool(self.h, b"isSpeaking\0") } }

    /// Whether the synthesizer is paused.
    pub fn is_paused(&self) -> bool { unsafe { msg_send_bool(self.h, b"isPaused\0") } }
}

impl Default for SpeechSynthesizer { fn default() -> Self { Self::new() } }
impl Drop for SpeechSynthesizer { fn drop(&mut self) { unsafe { release(self.h); } } }

/// Boundary for stopping/pausing speech.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechBoundary {
    Immediate = 0,
    Word = 1,
}
