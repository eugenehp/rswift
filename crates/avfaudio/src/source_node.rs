//! `AVAudioSourceNode` — node that provides audio data via a block.

use apple_objc_sys::*;
use crate::node::impl_node_methods;

#[allow(unused_imports)]
use crate::*;

/// A source node that generates audio via a render block.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosourcenode>
pub struct AudioSourceNode { h: Id }

unsafe impl Send for AudioSourceNode {}

impl AudioSourceNode {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }
}

impl_node_methods!(AudioSourceNode);
