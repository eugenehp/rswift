//! `AVAudioInputNode` and `AVAudioOutputNode` — I/O nodes.

use apple_objc_sys::*;
use crate::*;
use crate::node::impl_node_methods;
use std::fmt;

/// The engine's input (microphone) node.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioinputnode>
pub struct AudioInputNode { h: Id }

unsafe impl Send for AudioInputNode {}

impl AudioInputNode {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// Whether voice processing is enabled.
    pub fn is_voice_processing_enabled(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"isVoiceProcessingEnabled\0") }
    }

    /// Enable or disable voice processing.
    pub fn set_voice_processing_enabled(&self, enabled: bool) -> Result<(), String> {
        unsafe {
            let sel = sel_registerName(b"setVoiceProcessingEnabled:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, bool, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, enabled, &mut err);
            if ok { Ok(()) } else {
                let desc = if !err.is_null() {
                    let d = msg_send_id(err, b"localizedDescription\0");
                    nsstring_to_string(d).unwrap_or_else(|| "unknown error".into())
                } else { "unknown error".into() };
                Err(desc)
            }
        }
    }

    /// Whether voice processing AGC is enabled.
    pub fn is_voice_processing_agc_enabled(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"isVoiceProcessingAGCEnabled\0") }
    }

    /// Enable or disable voice processing AGC.
    pub fn set_voice_processing_agc_enabled(&self, enabled: bool) {
        unsafe { msg_send_set_bool(self.h, b"setVoiceProcessingAGCEnabled:\0", enabled); }
    }

    /// Whether voice processing input muting is enabled.
    pub fn is_voice_processing_input_muted(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"isVoiceProcessingInputMuted\0") }
    }

    /// Mute or unmute voice processing input.
    pub fn set_voice_processing_input_muted(&self, muted: bool) {
        unsafe { msg_send_set_bool(self.h, b"setVoiceProcessingInputMuted:\0", muted); }
    }
}

impl_node_methods!(AudioInputNode);

impl fmt::Display for AudioInputNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "AVAudioInputNode {{ VP: {}, AGC: {} }}",
            self.is_voice_processing_enabled(),
            self.is_voice_processing_agc_enabled(),
        )
    }
}

impl fmt::Debug for AudioInputNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioInputNode")
            .field("voice_processing", &self.is_voice_processing_enabled())
            .field("voice_processing_agc", &self.is_voice_processing_agc_enabled())
            .field("voice_processing_input_muted", &self.is_voice_processing_input_muted())
            .finish()
    }
}

// ── AVAudioOutputNode ───────────────────────────────────────────────────────

/// The engine's output (speaker) node.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiooutputnode>
pub struct AudioOutputNode { h: Id }

unsafe impl Send for AudioOutputNode {}

impl AudioOutputNode {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
}

impl_node_methods!(AudioOutputNode);

impl fmt::Display for AudioOutputNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AVAudioOutputNode")
    }
}

impl fmt::Debug for AudioOutputNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioOutputNode").finish()
    }
}
