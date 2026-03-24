//! `AVAudioPlayerNode` — scheduled playback in an audio graph.

use apple_objc_sys::*;
use crate::*;
use crate::node::impl_node_methods;

// Buffer scheduling options.
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PlayerNodeBufferOptions: usize {
        /// Loop the buffer.
        const LOOPS = 1;
        /// Interrupt any currently-playing buffer.
        const INTERRUPTS = 2;
        /// Interrupt at the end of the current loop.
        const INTERRUPTS_AT_LOOP = 4;
    }
}

/// Completion callback type.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerNodeCompletionCallbackType {
    /// Called when the data has been consumed (rendered).
    DataConsumed = 0,
    /// Called when the data has been rendered to the output.
    DataRendered = 1,
    /// Called when playback has completed.
    DataPlayedBack = 2,
}

/// A player node for scheduling buffers and files in the audio graph.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioplayernode>
pub struct AudioPlayerNode { h: Id }

unsafe impl Send for AudioPlayerNode {}

impl AudioPlayerNode {
    /// Create a new player node.
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioPlayerNode\0"), new] };
        Self { h }
    }

    pub fn as_raw(&self) -> Id { self.h }

    // ── Schedule methods ────────────────────────────────────────────────

    /// Schedule a buffer for playback (no completion handler).
    pub fn schedule_buffer(&self, buffer: &AudioPCMBuffer) {
        unsafe {
            let sel = sel_registerName(b"scheduleBuffer:completionHandler:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, buffer.as_raw(), NIL);
        }
    }

    /// Schedule a buffer with options and time.
    pub fn schedule_buffer_at_time(
        &self, buffer: &AudioPCMBuffer, when: Option<&AudioTime>,
        options: PlayerNodeBufferOptions,
    ) {
        unsafe {
            let sel = sel_registerName(b"scheduleBuffer:atTime:options:completionHandler:\0".as_ptr());
            let t = when.map_or(NIL, |t| t.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, usize, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, buffer.as_raw(), t, options.bits(), NIL);
        }
    }

    /// Schedule an entire file for playback.
    pub fn schedule_file(&self, file: &AudioFile, when: Option<&AudioTime>) {
        unsafe {
            let sel = sel_registerName(b"scheduleFile:atTime:completionHandler:\0".as_ptr());
            let t = when.map_or(NIL, |t| t.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, file.as_raw(), t, NIL);
        }
    }

    /// Schedule a segment of a file.
    pub fn schedule_segment(
        &self, file: &AudioFile, start_frame: AudioFramePosition,
        frame_count: AudioFrameCount, when: Option<&AudioTime>,
    ) {
        unsafe {
            let sel = sel_registerName(
                b"scheduleSegment:startingFrame:frameCount:atTime:completionHandler:\0".as_ptr()
            );
            let t = when.map_or(NIL, |t| t.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id, i64, u32, Id, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, file.as_raw(), start_frame, frame_count, t, NIL);
        }
    }

    // ── Playback control ────────────────────────────────────────────────

    /// Start playback immediately.
    pub fn play(&self) { unsafe { msg_send_void(self.h, b"play\0"); } }

    /// Start playback at a specific time.
    pub fn play_at_time(&self, when: Option<&AudioTime>) {
        unsafe {
            let sel = sel_registerName(b"playAtTime:\0".as_ptr());
            let t = when.map_or(NIL, |t| t.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, t);
        }
    }

    /// Pause playback.
    pub fn pause(&self) { unsafe { msg_send_void(self.h, b"pause\0"); } }

    /// Stop playback and clear scheduled events.
    pub fn stop(&self) { unsafe { msg_send_void(self.h, b"stop\0"); } }

    /// Whether the node is playing.
    pub fn is_playing(&self) -> bool { unsafe { msg_send_bool(self.h, b"isPlaying\0") } }

    // ── Time conversion ─────────────────────────────────────────────────

    /// Prepare with a frame count.
    pub fn prepare_with_frame_count(&self, frame_count: AudioFrameCount) {
        unsafe {
            let sel = sel_registerName(b"prepareWithFrameCount:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u32) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, frame_count);
        }
    }

    /// Convert node time to player time.
    pub fn player_time_for_node_time(&self, node_time: &AudioTime) -> Option<AudioTime> {
        unsafe {
            let h = msg_send![self.h, playerTimeForNodeTime: node_time.as_raw()];
            if h.is_null() { None } else { Some(AudioTime::from_raw(h)) }
        }
    }

    /// Convert player time to node time.
    pub fn node_time_for_player_time(&self, player_time: &AudioTime) -> Option<AudioTime> {
        unsafe {
            let h = msg_send![self.h, nodeTimeForPlayerTime: player_time.as_raw()];
            if h.is_null() { None } else { Some(AudioTime::from_raw(h)) }
        }
    }
}

impl Default for AudioPlayerNode { fn default() -> Self { Self::new() } }
impl Drop for AudioPlayerNode { fn drop(&mut self) { unsafe { release(self.h); } } }

impl_node_methods!(AudioPlayerNode);
