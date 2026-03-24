//! `AVAudioMixerNode` — mixes multiple inputs to a single output.

use apple_objc_sys::*;
use crate::*;
use crate::node::impl_node_methods;
use std::fmt;

/// A mixer node.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiomixernode>
pub struct AudioMixerNode { h: Id }

unsafe impl Send for AudioMixerNode {}

impl AudioMixerNode {
    /// Create a new mixer node.
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioMixerNode\0"), new] };
        Self { h }
    }

    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// The mixer's output volume [0.0, 1.0].
    pub fn output_volume(&self) -> f32 { unsafe { msg_send_f32(self.h, b"outputVolume\0") } }

    /// Set the output volume.
    pub fn set_output_volume(&self, vol: f32) {
        unsafe {
            let sel = sel_registerName(b"setOutputVolume:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, vol);
        }
    }

    /// The next available input bus.
    pub fn next_available_input_bus(&self) -> AudioNodeBus {
        unsafe { msg_send_usize(self.h, b"nextAvailableInputBus\0") }
    }
}

impl Default for AudioMixerNode { fn default() -> Self { Self::new() } }

impl_node_methods!(AudioMixerNode);

impl fmt::Display for AudioMixerNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vol_pct = (self.output_volume() * 100.0) as u32;
        write!(f, "AVAudioMixerNode {{ vol: {}%, next_bus: {} }}", vol_pct, self.next_available_input_bus())
    }
}

impl fmt::Debug for AudioMixerNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioMixerNode")
            .field("output_volume", &self.output_volume())
            .field("next_available_input_bus", &self.next_available_input_bus())
            .finish()
    }
}
