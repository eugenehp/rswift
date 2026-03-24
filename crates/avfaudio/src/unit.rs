//! `AVAudioUnit` and its subclasses — effects, EQ, reverb, delay, distortion,
//! generators, samplers, time effects, time pitch, varispeed.

use apple_objc_sys::*;
use crate::*;
use crate::node::impl_node_methods;

// ── AVAudioUnit (base) ─────────────────────────────────────────────────────

/// Base class for audio units in the processing graph.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiounit>
pub struct AudioUnit { h: Id }
unsafe impl Send for AudioUnit {}
impl AudioUnit {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
    /// The audio unit's name.
    pub fn name(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send_id(self.h, b"name\0")) }
    }
    /// The manufacturer name.
    pub fn manufacturer_name(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send_id(self.h, b"manufacturerName\0")) }
    }
}
impl_node_methods!(AudioUnit);

// ── AVAudioUnitEffect ───────────────────────────────────────────────────────

/// An effect audio unit.
pub struct AudioUnitEffect { h: Id }
unsafe impl Send for AudioUnitEffect {}
impl AudioUnitEffect {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
    /// Whether the effect is bypassed.
    pub fn bypass(&self) -> bool { unsafe { msg_send_bool(self.h, b"bypass\0") } }
    pub fn set_bypass(&self, bypass: bool) { unsafe { msg_send_set_bool(self.h, b"setBypass:\0", bypass); } }
}
impl_node_methods!(AudioUnitEffect);

// ── AVAudioUnitReverb ───────────────────────────────────────────────────────

/// Reverb effect.
pub struct AudioUnitReverb { h: Id }
unsafe impl Send for AudioUnitReverb {}
impl AudioUnitReverb {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitReverb\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn wet_dry_mix(&self) -> f32 { unsafe { msg_send_f32(self.h, b"wetDryMix\0") } }
    pub fn set_wet_dry_mix(&self, mix: f32) {
        unsafe {
            let sel = sel_registerName(b"setWetDryMix:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, mix);
        }
    }
}
impl Default for AudioUnitReverb { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitReverb { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitReverb);

// ── AVAudioUnitDelay ────────────────────────────────────────────────────────

/// Delay effect.
pub struct AudioUnitDelay { h: Id }
unsafe impl Send for AudioUnitDelay {}
impl AudioUnitDelay {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitDelay\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn delay_time(&self) -> f64 { unsafe { msg_send_f64(self.h, b"delayTime\0") } }
    pub fn set_delay_time(&self, t: f64) {
        unsafe {
            let sel = sel_registerName(b"setDelayTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, t);
        }
    }
    pub fn feedback(&self) -> f32 { unsafe { msg_send_f32(self.h, b"feedback\0") } }
    pub fn set_feedback(&self, fb: f32) {
        unsafe {
            let sel = sel_registerName(b"setFeedback:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, fb);
        }
    }
    pub fn low_pass_cutoff(&self) -> f32 { unsafe { msg_send_f32(self.h, b"lowPassCutoff\0") } }
    pub fn set_low_pass_cutoff(&self, freq: f32) {
        unsafe {
            let sel = sel_registerName(b"setLowPassCutoff:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, freq);
        }
    }
    pub fn wet_dry_mix(&self) -> f32 { unsafe { msg_send_f32(self.h, b"wetDryMix\0") } }
    pub fn set_wet_dry_mix(&self, mix: f32) {
        unsafe {
            let sel = sel_registerName(b"setWetDryMix:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, mix);
        }
    }
}
impl Default for AudioUnitDelay { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitDelay { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitDelay);

// ── AVAudioUnitDistortion ───────────────────────────────────────────────────

/// Distortion effect.
pub struct AudioUnitDistortion { h: Id }
unsafe impl Send for AudioUnitDistortion {}
impl AudioUnitDistortion {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitDistortion\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn pre_gain(&self) -> f32 { unsafe { msg_send_f32(self.h, b"preGain\0") } }
    pub fn set_pre_gain(&self, gain: f32) {
        unsafe {
            let sel = sel_registerName(b"setPreGain:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, gain);
        }
    }
    pub fn wet_dry_mix(&self) -> f32 { unsafe { msg_send_f32(self.h, b"wetDryMix\0") } }
    pub fn set_wet_dry_mix(&self, mix: f32) {
        unsafe {
            let sel = sel_registerName(b"setWetDryMix:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, mix);
        }
    }
}
impl Default for AudioUnitDistortion { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitDistortion { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitDistortion);

// ── AVAudioUnitEQ ───────────────────────────────────────────────────────────

/// Parametric equalizer.
pub struct AudioUnitEQ { h: Id }
unsafe impl Send for AudioUnitEQ {}
impl AudioUnitEQ {
    /// Create an EQ with the given number of bands.
    pub fn new(number_of_bands: usize) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioUnitEQ\0"), alloc];
            let sel = sel_registerName(b"initWithNumberOfBands:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { h: f(obj, sel, number_of_bands) }
        }
    }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn global_gain(&self) -> f32 { unsafe { msg_send_f32(self.h, b"globalGain\0") } }
    pub fn set_global_gain(&self, gain: f32) {
        unsafe {
            let sel = sel_registerName(b"setGlobalGain:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, gain);
        }
    }
}
impl Drop for AudioUnitEQ { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitEQ);

// ── AVAudioUnitTimePitch ────────────────────────────────────────────────────

/// Time pitch effect (change pitch without changing tempo).
pub struct AudioUnitTimePitch { h: Id }
unsafe impl Send for AudioUnitTimePitch {}
impl AudioUnitTimePitch {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitTimePitch\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    /// Pitch shift in cents (-2400 to 2400).
    pub fn pitch(&self) -> f32 { unsafe { msg_send_f32(self.h, b"pitch\0") } }
    pub fn set_pitch(&self, cents: f32) {
        unsafe {
            let sel = sel_registerName(b"setPitch:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, cents);
        }
    }
    /// Playback rate (0.25 to 4.0).
    pub fn rate(&self) -> f32 { unsafe { msg_send_f32(self.h, b"rate\0") } }
    pub fn set_rate(&self, rate: f32) {
        unsafe {
            let sel = sel_registerName(b"setRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, rate);
        }
    }
    /// Overlap (3.0 to 32.0).
    pub fn overlap(&self) -> f32 { unsafe { msg_send_f32(self.h, b"overlap\0") } }
    pub fn set_overlap(&self, overlap: f32) {
        unsafe {
            let sel = sel_registerName(b"setOverlap:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, overlap);
        }
    }
}
impl Default for AudioUnitTimePitch { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitTimePitch { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitTimePitch);

// ── AVAudioUnitVarispeed ────────────────────────────────────────────────────

/// Varispeed effect (change tempo and pitch together).
pub struct AudioUnitVarispeed { h: Id }
unsafe impl Send for AudioUnitVarispeed {}
impl AudioUnitVarispeed {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitVarispeed\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn rate(&self) -> f32 { unsafe { msg_send_f32(self.h, b"rate\0") } }
    pub fn set_rate(&self, rate: f32) {
        unsafe {
            let sel = sel_registerName(b"setRate:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, rate);
        }
    }
}
impl Default for AudioUnitVarispeed { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitVarispeed { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitVarispeed);

// ── AVAudioUnitGenerator ────────────────────────────────────────────────────

/// An audio unit that generates audio (instrument/tone generator).
pub struct AudioUnitGenerator { h: Id }
unsafe impl Send for AudioUnitGenerator {}
impl AudioUnitGenerator {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
    pub fn bypass(&self) -> bool { unsafe { msg_send_bool(self.h, b"bypass\0") } }
    pub fn set_bypass(&self, bypass: bool) { unsafe { msg_send_set_bool(self.h, b"setBypass:\0", bypass); } }
}
impl_node_methods!(AudioUnitGenerator);

// ── AVAudioUnitSampler ──────────────────────────────────────────────────────

/// A sampler audio unit (loads sound banks / audio files as instruments).
pub struct AudioUnitSampler { h: Id }
unsafe impl Send for AudioUnitSampler {}
impl AudioUnitSampler {
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioUnitSampler\0"), new] };
        Self { h }
    }
    pub fn as_raw(&self) -> Id { self.h }
    /// Master gain in decibels.
    pub fn master_gain(&self) -> f32 { unsafe { msg_send_f32(self.h, b"masterGain\0") } }
    pub fn set_master_gain(&self, gain: f32) {
        unsafe {
            let sel = sel_registerName(b"setMasterGain:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, gain);
        }
    }
    /// Stereo pan (-1 left, 0 center, +1 right).
    pub fn stereo_pan(&self) -> f32 { unsafe { msg_send_f32(self.h, b"stereoPan\0") } }
    pub fn set_stereo_pan(&self, pan: f32) {
        unsafe {
            let sel = sel_registerName(b"setStereoPan:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, pan);
        }
    }
    /// Global tuning in cents.
    pub fn global_tuning(&self) -> f32 { unsafe { msg_send_f32(self.h, b"globalTuning\0") } }
    pub fn set_global_tuning(&self, tuning: f32) {
        unsafe {
            let sel = sel_registerName(b"setGlobalTuning:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, tuning);
        }
    }
}
impl Default for AudioUnitSampler { fn default() -> Self { Self::new() } }
impl Drop for AudioUnitSampler { fn drop(&mut self) { unsafe { release(self.h); } } }
impl_node_methods!(AudioUnitSampler);

// ── AVAudioUnitMIDIInstrument ───────────────────────────────────────────────

/// A MIDI instrument audio unit.
pub struct AudioUnitMIDIInstrument { h: Id }
unsafe impl Send for AudioUnitMIDIInstrument {}
impl AudioUnitMIDIInstrument {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// Send a MIDI note-on.
    pub fn start_note(&self, note: u8, velocity: u8, channel: u8) {
        unsafe {
            let sel = sel_registerName(b"startNote:withVelocity:onChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u8, u8, u8) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, note, velocity, channel);
        }
    }

    /// Send a MIDI note-off.
    pub fn stop_note(&self, note: u8, channel: u8) {
        unsafe {
            let sel = sel_registerName(b"stopNote:onChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u8, u8) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, note, channel);
        }
    }

    /// Send a MIDI controller change.
    pub fn send_controller(&self, controller: u8, value: u8, channel: u8) {
        unsafe {
            let sel = sel_registerName(b"sendController:withValue:onChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u8, u8, u8) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, controller, value, channel);
        }
    }

    /// Send a MIDI program change.
    pub fn send_program_change(&self, program: u8, channel: u8) {
        unsafe {
            let sel = sel_registerName(b"sendProgramChange:onChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u8, u8) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, program, channel);
        }
    }

    /// Send a MIDI pitch bend.
    pub fn send_pitch_bend(&self, pitch_bend: u16, channel: u8) {
        unsafe {
            let sel = sel_registerName(b"sendPitchBend:onChannel:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u16, u8) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, pitch_bend, channel);
        }
    }
}
impl_node_methods!(AudioUnitMIDIInstrument);
