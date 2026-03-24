//! `AVAudioSequencer` — MIDI sequencer integrated with the audio engine.

use apple_objc_sys::*;
use crate::*;

// MIDI sequence load options.
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MusicSequenceLoadOptions: usize {
        const PRESERVE_TRACKS = 0;
        const CHANNELS_TO_TRACKS = 1;
    }
}

/// A MIDI sequencer.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosequencer>
pub struct AudioSequencer { h: Id }

unsafe impl Send for AudioSequencer {}

impl AudioSequencer {
    /// Create a sequencer attached to the given engine.
    pub fn new(engine: &AudioEngine) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioSequencer\0"), alloc];
            let h = msg_send![obj, initWithAudioEngine: engine.as_raw()];
            Self { h }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Load from a URL.
    pub fn load(&self, url: Id, options: MusicSequenceLoadOptions) -> Result<(), String> {
        unsafe {
            let sel = sel_registerName(b"loadFromURL:options:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, Id, usize, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, url, options.bits(), &mut err);
            if ok { Ok(()) } else {
                let desc = if !err.is_null() {
                    let d = msg_send_id(err, b"localizedDescription\0");
                    nsstring_to_string(d).unwrap_or_else(|| "unknown error".into())
                } else { "unknown error".into() };
                Err(desc)
            }
        }
    }

    /// Prepare to play.
    pub fn prepare_to_play(&self) { unsafe { msg_send_void(self.h, b"prepareToPlay\0"); } }

    /// Start playback.
    pub fn start(&self) -> Result<(), String> {
        unsafe {
            let mut err: Id = NIL;
            let sel = sel_registerName(b"startAndReturnError:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, &mut err);
            if ok { Ok(()) } else {
                let desc = if !err.is_null() {
                    let d = msg_send_id(err, b"localizedDescription\0");
                    nsstring_to_string(d).unwrap_or_else(|| "unknown error".into())
                } else { "unknown error".into() };
                Err(desc)
            }
        }
    }

    /// Stop playback.
    pub fn stop(&self) { unsafe { msg_send_void(self.h, b"stop\0"); } }

    /// Whether the sequencer is playing.
    pub fn is_playing(&self) -> bool { unsafe { msg_send_bool(self.h, b"isPlaying\0") } }

    /// The playback rate (1.0 = normal).
    pub fn rate(&self) -> f32 { unsafe { msg_send_f32(self.h, b"rate\0") } }

    /// Set the playback rate.
    pub fn set_rate(&self, rate: f32) {
        unsafe {
            let sel = sel_registerName(b"setRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, rate);
        }
    }

    /// Current playback position in beats.
    pub fn current_position_in_beats(&self) -> f64 {
        unsafe { msg_send_f64(self.h, b"currentPositionInBeats\0") }
    }

    /// Set the current position in beats.
    pub fn set_current_position_in_beats(&self, beats: f64) {
        unsafe {
            let sel = sel_registerName(b"setCurrentPositionInBeats:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, beats);
        }
    }

    /// Current playback position in seconds.
    pub fn current_position_in_seconds(&self) -> f64 {
        unsafe { msg_send_f64(self.h, b"currentPositionInSeconds\0") }
    }

    /// Set the current position in seconds.
    pub fn set_current_position_in_seconds(&self, seconds: f64) {
        unsafe {
            let sel = sel_registerName(b"setCurrentPositionInSeconds:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, seconds);
        }
    }

    /// Tempo in BPM.
    pub fn tempo_in_bpm(&self) -> f64 {
        unsafe { msg_send_f64(self.h, b"tempoInBeatsPerMinute\0") }
    }
}

impl Drop for AudioSequencer { fn drop(&mut self) { unsafe { release(self.h); } } }
