//! `AVAudioSinkNode` — node that receives audio data via a block.

use apple_objc_sys::*;
use crate::node::impl_node_methods;

#[allow(unused_imports)]
use crate::*;

/// A sink node that receives audio via a render block.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosinknode>
pub struct AudioSinkNode { h: Id }

unsafe impl Send for AudioSinkNode {}

impl AudioSinkNode {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
}

impl_node_methods!(AudioSinkNode);
