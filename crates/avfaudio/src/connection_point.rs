//! `AVAudioConnectionPoint` — a connection to a node on a specific bus.

use apple_objc_sys::*;
use crate::*;

/// A connection point (node + bus).
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioconnectionpoint>
pub struct AudioConnectionPoint { h: Id }

unsafe impl Send for AudioConnectionPoint {}

impl AudioConnectionPoint {
    pub(crate) fn from_raw(h: Id) -> Self { Self { h } }
    pub fn as_raw(&self) -> Id { self.h }

    /// The bus number.
    pub fn bus(&self) -> AudioNodeBus { unsafe { msg_send_usize(self.h, b"bus\0") } }

    /// The node (raw `AVAudioNode*`).
    pub fn node_raw(&self) -> Id { unsafe { msg_send_id(self.h, b"node\0") } }
}
