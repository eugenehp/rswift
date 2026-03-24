//! `AVAudioNode` — base class for audio graph nodes.

use apple_objc_sys::*;
use crate::*;
use crate::engine::AsRawNode;

/// Base audio node.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudionode>
pub struct AudioNode { h: Id }

unsafe impl Send for AudioNode {}

impl AudioNode {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// Reset the node's processing state.
    pub fn reset(&self) { unsafe { msg_send_void(self.h, b"reset\0"); } }

    /// Input format for a given bus.
    pub fn input_format_for_bus(&self, bus: AudioNodeBus) -> AudioFormat {
        unsafe {
            let sel = sel_registerName(b"inputFormatForBus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            AudioFormat::from_raw(f(self.h, sel, bus))
        }
    }

    /// Output format for a given bus.
    pub fn output_format_for_bus(&self, bus: AudioNodeBus) -> AudioFormat {
        unsafe {
            let sel = sel_registerName(b"outputFormatForBus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            AudioFormat::from_raw(f(self.h, sel, bus))
        }
    }

    /// Number of input busses.
    pub fn number_of_inputs(&self) -> usize { unsafe { msg_send_usize(self.h, b"numberOfInputs\0") } }

    /// Number of output busses.
    pub fn number_of_outputs(&self) -> usize { unsafe { msg_send_usize(self.h, b"numberOfOutputs\0") } }

    /// The last render time.
    pub fn last_render_time(&self) -> Option<AudioTime> {
        unsafe {
            let h = msg_send_id(self.h, b"lastRenderTime\0");
            if h.is_null() { None } else { Some(AudioTime::from_raw(h)) }
        }
    }

    /// Processing latency in seconds.
    pub fn latency(&self) -> f64 { unsafe { msg_send_f64(self.h, b"latency\0") } }

    /// Output presentation latency in seconds.
    pub fn output_presentation_latency(&self) -> f64 {
        unsafe { msg_send_f64(self.h, b"outputPresentationLatency\0") }
    }

    /// Remove a tap on the given output bus.
    pub fn remove_tap_on_bus(&self, bus: AudioNodeBus) {
        unsafe {
            let sel = sel_registerName(b"removeTapOnBus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, bus);
        }
    }
}

impl AsRawNode for AudioNode {
    fn as_raw_node(&self) -> Id { self.h }
}

/// Shared node methods implemented via a macro for all node wrapper types.
macro_rules! impl_node_methods {
    ($ty:ty) => {
        impl $ty {
            /// Reset the node.
            pub fn node_reset(&self) { unsafe { crate::msg_send_void(self.h, b"reset\0"); } }

            /// Input format for a bus.
            pub fn input_format_for_bus(&self, bus: crate::AudioNodeBus) -> crate::AudioFormat {
                unsafe {
                    let sel = apple_objc_sys::sel_registerName(b"inputFormatForBus:\0".as_ptr());
                    let f: unsafe extern "C" fn(apple_objc_sys::Id, apple_objc_sys::Sel, usize) -> apple_objc_sys::Id =
                        core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
                    crate::AudioFormat::from_raw(f(self.h, sel, bus))
                }
            }

            /// Output format for a bus.
            pub fn output_format_for_bus(&self, bus: crate::AudioNodeBus) -> crate::AudioFormat {
                unsafe {
                    let sel = apple_objc_sys::sel_registerName(b"outputFormatForBus:\0".as_ptr());
                    let f: unsafe extern "C" fn(apple_objc_sys::Id, apple_objc_sys::Sel, usize) -> apple_objc_sys::Id =
                        core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
                    crate::AudioFormat::from_raw(f(self.h, sel, bus))
                }
            }

            /// Number of input busses.
            pub fn number_of_inputs(&self) -> usize { unsafe { crate::msg_send_usize(self.h, b"numberOfInputs\0") } }

            /// Number of output busses.
            pub fn number_of_outputs(&self) -> usize { unsafe { crate::msg_send_usize(self.h, b"numberOfOutputs\0") } }

            /// Last render time.
            pub fn last_render_time(&self) -> Option<crate::AudioTime> {
                unsafe {
                    let h = crate::msg_send_id(self.h, b"lastRenderTime\0");
                    if h.is_null() { None } else { Some(crate::AudioTime::from_raw(h)) }
                }
            }

            /// Processing latency.
            pub fn node_latency(&self) -> f64 { unsafe { crate::msg_send_f64(self.h, b"latency\0") } }

            /// Output presentation latency.
            pub fn output_presentation_latency(&self) -> f64 {
                unsafe { crate::msg_send_f64(self.h, b"outputPresentationLatency\0") }
            }
        }

        impl crate::engine::AsRawNode for $ty {
            fn as_raw_node(&self) -> apple_objc_sys::Id { self.h }
        }
    };
}

pub(crate) use impl_node_methods;
