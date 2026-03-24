//! `AVAudioChannelLayout` — channel layout description.

use apple_objc_sys::*;
use crate::*;

/// An audio channel layout.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiochannellayout>
pub struct AudioChannelLayout { h: Id }

unsafe impl Send for AudioChannelLayout {}
unsafe impl Sync for AudioChannelLayout {}

impl AudioChannelLayout {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// Create a layout with a specific layout tag.
    pub fn with_layout_tag(tag: u32) -> Self {
        unsafe {
            let obj = msg_send![class!(b"AVAudioChannelLayout\0"), alloc];
            let sel = sel_registerName(b"initWithLayoutTag:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u32) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            Self { h: f(obj, sel, tag) }
        }
    }

    /// The layout tag.
    pub fn layout_tag(&self) -> u32 {
        unsafe { msg_send_usize(self.h, b"layoutTag\0") as u32 }
    }

    /// The channel count.
    pub fn channel_count(&self) -> AudioChannelCount {
        unsafe { msg_send_usize(self.h, b"channelCount\0") as AudioChannelCount }
    }
}
