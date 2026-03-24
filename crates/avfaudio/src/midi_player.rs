//! `AVMIDIPlayer` — play MIDI files using a sound bank.

use apple_objc_sys::*;
use crate::*;

/// A simple MIDI file player.
///
/// <https://developer.apple.com/documentation/avfaudio/avmidiplayer>
pub struct MIDIPlayer { h: Id }

unsafe impl Send for MIDIPlayer {}

impl MIDIPlayer {
    /// Create a MIDI player from a URL and sound bank URL.
    pub fn new(url: Id, sound_bank_url: Id) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVMIDIPlayer\0"), alloc];
            let h = msg_send![obj, initWithContentsOfURL: url, soundBankURL: sound_bank_url, error: NIL];
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Prepare to play.
    pub fn prepare_to_play(&self) { unsafe { msg_send_void(self.h, b"prepareToPlay\0"); } }

    /// Start playback (no completion handler).
    pub fn play(&self) {
        unsafe {
            let sel = sel_registerName(b"play:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, NIL);
        }
    }

    /// Stop playback.
    pub fn stop(&self) { unsafe { msg_send_void(self.h, b"stop\0"); } }

    /// Whether the player is playing.
    pub fn is_playing(&self) -> bool { unsafe { msg_send_bool(self.h, b"isPlaying\0") } }

    /// Duration of the MIDI file.
    pub fn duration(&self) -> f64 { unsafe { msg_send_f64(self.h, b"duration\0") } }

    /// Current playback position.
    pub fn current_position(&self) -> f64 { unsafe { msg_send_f64(self.h, b"currentPosition\0") } }

    /// Set the playback position.
    pub fn set_current_position(&self, pos: f64) {
        unsafe {
            let sel = sel_registerName(b"setCurrentPosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, pos);
        }
    }

    /// Playback rate (1.0 = normal).
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
}

impl Drop for MIDIPlayer { fn drop(&mut self) { unsafe { release(self.h); } } }
