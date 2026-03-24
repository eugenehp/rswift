//! `AVAudioEngine` — the real-time audio processing graph.

use apple_objc_sys::*;
use crate::*;
use crate::AudioResult;
use std::fmt;

/// Manual rendering mode.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManualRenderingMode {
    Offline = 0,
    Realtime = 1,
}

/// Manual rendering status.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManualRenderingStatus {
    Error = -1,
    Success = 0,
    InsufficientDataFromInputNode = 1,
    CannotDoInCurrentContext = 2,
}

/// The audio processing graph.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioengine>
pub struct AudioEngine { h: Id }

unsafe impl Send for AudioEngine {}

impl AudioEngine {
    /// Create a new audio engine.
    pub fn new() -> Self {
        let h = unsafe { msg_send![class!(b"AVAudioEngine\0"), new] };
        Self { h }
    }

    pub fn as_raw(&self) -> Id { self.h }

    // ── Node access ─────────────────────────────────────────────────────

    /// The singleton output node.
    pub fn output_node(&self) -> AudioOutputNode {
        let h = unsafe { msg_send_id(self.h, b"outputNode\0") };
        AudioOutputNode::from_raw(h)
    }

    /// The singleton input node.
    pub fn input_node(&self) -> AudioInputNode {
        let h = unsafe { msg_send_id(self.h, b"inputNode\0") };
        AudioInputNode::from_raw(h)
    }

    /// The main mixer node (created on demand).
    pub fn main_mixer_node(&self) -> AudioMixerNode {
        let h = unsafe { msg_send_id(self.h, b"mainMixerNode\0") };
        AudioMixerNode::from_raw(h)
    }

    // ── Attach / detach ─────────────────────────────────────────────────

    /// Attach a node to the engine.
    pub fn attach_node(&self, node: &dyn AsRawNode) {
        unsafe { msg_send_void_id(self.h, b"attachNode:\0", node.as_raw_node()); }
    }

    /// Detach a node from the engine.
    pub fn detach_node(&self, node: &dyn AsRawNode) {
        unsafe { msg_send_void_id(self.h, b"detachNode:\0", node.as_raw_node()); }
    }

    // ── Connect / disconnect ────────────────────────────────────────────

    /// Connect two nodes (bus 0 → bus 0 or next available mixer input).
    pub fn connect(&self, from: &dyn AsRawNode, to: &dyn AsRawNode, format: Option<&AudioFormat>) {
        unsafe {
            let sel = sel_registerName(b"connect:to:format:\0".as_ptr());
            let fmt = format.map_or(NIL, |f| f.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, from.as_raw_node(), to.as_raw_node(), fmt);
        }
    }

    /// Connect two nodes on specific busses.
    pub fn connect_from_bus_to_bus(
        &self, from: &dyn AsRawNode, to: &dyn AsRawNode,
        from_bus: AudioNodeBus, to_bus: AudioNodeBus,
        format: Option<&AudioFormat>,
    ) {
        unsafe {
            let sel = sel_registerName(b"connect:to:fromBus:toBus:format:\0".as_ptr());
            let fmt = format.map_or(NIL, |f| f.as_raw());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, usize, usize, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, from.as_raw_node(), to.as_raw_node(), from_bus, to_bus, fmt);
        }
    }

    /// Disconnect all input connections on a node.
    pub fn disconnect_node_input(&self, node: &dyn AsRawNode) {
        unsafe { msg_send_void_id(self.h, b"disconnectNodeInput:\0", node.as_raw_node()); }
    }

    /// Disconnect a specific input bus.
    pub fn disconnect_node_input_bus(&self, node: &dyn AsRawNode, bus: AudioNodeBus) {
        unsafe {
            let sel = sel_registerName(b"disconnectNodeInput:bus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, usize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, node.as_raw_node(), bus);
        }
    }

    /// Disconnect all output connections on a node.
    pub fn disconnect_node_output(&self, node: &dyn AsRawNode) {
        unsafe { msg_send_void_id(self.h, b"disconnectNodeOutput:\0", node.as_raw_node()); }
    }

    /// Disconnect a specific output bus.
    pub fn disconnect_node_output_bus(&self, node: &dyn AsRawNode, bus: AudioNodeBus) {
        unsafe {
            let sel = sel_registerName(b"disconnectNodeOutput:bus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, usize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, node.as_raw_node(), bus);
        }
    }

    // ── Lifecycle ───────────────────────────────────────────────────────

    /// Prepare the engine for starting (preallocate resources).
    pub fn prepare(&self) { unsafe { msg_send_void(self.h, b"prepare\0"); } }

    /// Start the engine. Returns `true` on success.
    pub fn start(&self) -> bool {
        unsafe { msg_send_t![bool; self.h, startAndReturnError: NIL] }
    }

    /// Start the engine, returning a typed error on failure.
    pub fn try_start(&self) -> AudioResult {
        unsafe { objc_try!(noarg self.h, b"startAndReturnError:\0") }
    }

    /// Alias: start returning `AudioResult`.
    pub fn start_and_return_error(&self) -> AudioResult { self.try_start() }

    /// Pause the engine (keeps resources allocated).
    pub fn pause(&self) { unsafe { msg_send_void(self.h, b"pause\0"); } }

    /// Reset all nodes.
    pub fn reset(&self) { unsafe { msg_send_void(self.h, b"reset\0"); } }

    /// Stop the engine (releases resources).
    pub fn stop(&self) { unsafe { msg_send_void(self.h, b"stop\0"); } }

    // ── State ───────────────────────────────────────────────────────────

    /// Whether the engine is currently running.
    pub fn is_running(&self) -> bool { unsafe { msg_send_bool(self.h, b"isRunning\0") } }

    /// Whether auto-shutdown is enabled.
    pub fn is_auto_shutdown_enabled(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"isAutoShutdownEnabled\0") }
    }

    /// Enable/disable auto-shutdown.
    pub fn set_auto_shutdown_enabled(&self, enabled: bool) {
        unsafe { msg_send_set_bool(self.h, b"setAutoShutdownEnabled:\0", enabled); }
    }

    // ── Manual rendering ────────────────────────────────────────────────

    /// Whether the engine is in manual rendering mode.
    pub fn is_in_manual_rendering_mode(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"isInManualRenderingMode\0") }
    }

    /// The current manual rendering mode.
    pub fn manual_rendering_mode(&self) -> ManualRenderingMode {
        let v = unsafe { msg_send_isize(self.h, b"manualRenderingMode\0") };
        if v == 1 { ManualRenderingMode::Realtime } else { ManualRenderingMode::Offline }
    }

    /// Maximum frame count for manual rendering.
    pub fn manual_rendering_maximum_frame_count(&self) -> AudioFrameCount {
        unsafe { msg_send_usize(self.h, b"manualRenderingMaximumFrameCount\0") as AudioFrameCount }
    }

    /// Current sample time in manual rendering mode.
    pub fn manual_rendering_sample_time(&self) -> AudioFramePosition {
        unsafe {
            let sel = sel_registerName(b"manualRenderingSampleTime\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel)
        }
    }

    /// Disable manual rendering (switch back to device rendering).
    pub fn disable_manual_rendering_mode(&self) {
        unsafe { msg_send_void(self.h, b"disableManualRenderingMode\0"); }
    }

    // ── Connection queries ──────────────────────────────────────────────

    /// Get the connection point for a node's input bus.
    pub fn input_connection_point_for_node(&self, node: &dyn AsRawNode, bus: AudioNodeBus) -> Option<AudioConnectionPoint> {
        unsafe {
            let sel = sel_registerName(b"inputConnectionPointForNode:inputBus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let cp = f(self.h, sel, node.as_raw_node(), bus);
            if cp.is_null() { None } else { Some(AudioConnectionPoint::from_raw(cp)) }
        }
    }

    // ── Convenience from old API ────────────────────────────────────────

    /// Get the output format (sample rate, channel count) — convenience method.
    pub fn output_format(&self) -> (f64, u32) {
        unsafe {
            let node: Id = msg_send_id(self.h, b"outputNode\0");
            let sel = sel_registerName(b"outputFormatForBus:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let fmt = f(node, sel, 0);
            let sr = msg_send_f64(fmt, b"sampleRate\0");
            let ch: u32 = msg_send_t![u32; fmt, channelCount];
            (sr, ch)
        }
    }

    // ── Ergonomic aliases ──────────────────────────────────────────────

    /// Attach a node (alias for `attach_node`).
    pub fn attach(&self, node: &dyn AsRawNode) { self.attach_node(node); }

    /// Detach a node (alias for `detach_node`).
    pub fn detach(&self, node: &dyn AsRawNode) { self.detach_node(node); }
}

impl Default for AudioEngine { fn default() -> Self { Self::new() } }
impl Drop for AudioEngine { fn drop(&mut self) { unsafe { release(self.h); } } }

impl fmt::Display for AudioEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sr, ch) = self.output_format();
        let state = if self.is_running() { "running" } else { "stopped" };
        write!(f, "AVAudioEngine {{ {state}, {sr} Hz, {ch}ch }}")
    }
}

impl fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (sr, ch) = self.output_format();
        f.debug_struct("AudioEngine")
            .field("running", &self.is_running())
            .field("sample_rate", &sr)
            .field("channels", &ch)
            .finish()
    }
}

/// Trait for types that wrap an `AVAudioNode*`.
pub trait AsRawNode {
    fn as_raw_node(&self) -> Id;
}
