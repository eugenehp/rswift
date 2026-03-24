//! `AVAudioEnvironmentNode` — 3D audio environment with reverb and distance attenuation.

use apple_objc_sys::*;
use crate::*;
use crate::node::impl_node_methods;

/// Distance attenuation model.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceAttenuationModel {
    Exponential = 1,
    Inverse = 2,
    Linear = 3,
}

/// Environment output type.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentOutputType {
    Auto = 0,
    Headphones = 1,
    BuiltInSpeakers = 2,
    ExternalSpeakers = 3,
}

/// 3D audio environment node.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioenvironmentnode>
pub struct AudioEnvironmentNode { h: Id }

unsafe impl Send for AudioEnvironmentNode {}

impl AudioEnvironmentNode {
    /// Create a new environment node.
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioEnvironmentNode\0"), new] };
        Self { h }
    }

    pub fn as_raw(&self) -> Id { self.h }

    /// Output volume.
    pub fn output_volume(&self) -> f32 { unsafe { msg_send_f32(self.h, b"outputVolume\0") } }

    /// Set output volume.
    pub fn set_output_volume(&self, vol: f32) {
        unsafe {
            let sel = sel_registerName(b"setOutputVolume:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, vol);
        }
    }

    /// Next available input bus.
    pub fn next_available_input_bus(&self) -> AudioNodeBus {
        unsafe { msg_send_usize(self.h, b"nextAvailableInputBus\0") }
    }

    /// Output type.
    pub fn output_type(&self) -> EnvironmentOutputType {
        let v = unsafe { msg_send_isize(self.h, b"outputType\0") };
        match v {
            1 => EnvironmentOutputType::Headphones,
            2 => EnvironmentOutputType::BuiltInSpeakers,
            3 => EnvironmentOutputType::ExternalSpeakers,
            _ => EnvironmentOutputType::Auto,
        }
    }

    /// Set output type.
    pub fn set_output_type(&self, ot: EnvironmentOutputType) {
        unsafe {
            let sel = sel_registerName(b"setOutputType:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, ot as isize);
        }
    }
}

impl Default for AudioEnvironmentNode { fn default() -> Self { Self::new() } }

impl_node_methods!(AudioEnvironmentNode);
