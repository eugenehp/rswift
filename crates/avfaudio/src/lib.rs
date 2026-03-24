#![allow(unsafe_op_in_unsafe_fn, dead_code)]
//! Apple AVFAudio — complete Rust bindings for audio playback, recording,
//! session management, and the audio engine graph.
//!
//! Pure Rust ObjC dispatch — no `.m` thunks, no `cc` build step.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use avfaudio::prelude::*;
//!
//! // ── Configure the audio session ─────────────────────────
//! let session = AudioSession::shared();
//! session.configure()
//!     .category(Category::Playback)
//!     .mode(Mode::Default)
//!     .options(CategoryOptions::MIX_WITH_OTHERS)
//!     .activate()?;
//!
//! // ── Play a file ─────────────────────────────────────────
//! let player = AudioPlayer::open("song.mp3")?;
//! player.play();
//! println!("{}", player);            // "▶ song.mp3 — 3:42 @ 100%"
//!
//! // ── Engine graph ────────────────────────────────────────
//! let engine = AudioEngine::new();
//! let player_node = AudioPlayerNode::new();
//! engine.attach(&player_node);
//! engine.connect(&player_node, &engine.main_mixer_node(), None);
//! engine.start()?;
//!
//! // ── Text-to-speech ──────────────────────────────────────
//! avfaudio::speak("Hello from Rust!");
//! ```
//!
//! # Modules
//!
//! | Module | Apple Class(es) |
//! |--------|-----------------|
//! | [`session`] | `AVAudioSession` — category, mode, activation, routing |
//! | [`engine`] | `AVAudioEngine` — real-time audio graph |
//! | [`player`] | `AVAudioPlayer` — simple file playback |
//! | [`player_node`] | `AVAudioPlayerNode` — scheduled buffer/file playback |
//! | [`recorder`] | `AVAudioRecorder` — audio recording |
//! | [`format`] | `AVAudioFormat` — audio format description |
//! | [`file`] | `AVAudioFile` — reading/writing audio files |
//! | [`buffer`] | `AVAudioPCMBuffer`, `AVAudioCompressedBuffer` |
//! | [`time`] | `AVAudioTime` — host/sample time |
//! | [`node`] | `AVAudioNode` — base node |
//! | [`mixer_node`] | `AVAudioMixerNode` — mixing |
//! | [`io_node`] | `AVAudioInputNode`, `AVAudioOutputNode` |
//! | [`channel_layout`] | `AVAudioChannelLayout` |
//! | [`connection_point`] | `AVAudioConnectionPoint` |
//! | [`converter`] | `AVAudioConverter` |
//! | [`environment_node`] | `AVAudioEnvironmentNode` |
//! | [`sink_node`] | `AVAudioSinkNode` |
//! | [`source_node`] | `AVAudioSourceNode` |
//! | [`unit`] | `AVAudioUnit` and effect subclasses |
//! | [`sequencer`] | `AVAudioSequencer` |
//! | [`midi_player`] | `AVMIDIPlayer` |
//! | [`speech`] | `AVSpeechSynthesizer`, `AVSpeechUtterance` |
//! | [`application`] | `AVAudioApplication` |
//! | [`ffi`] | Raw ObjC selector constants |
//!
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;
use anyhow::anyhow;
#[allow(unused_imports)] use anyhow::{bail, ensure, Context as _};

pub mod ffi;
pub mod error;
pub mod prelude;

// ── Sub-modules ─────────────────────────────────────────────────────────────
pub mod session;
pub mod engine;
pub mod player;
pub mod player_node;
pub mod recorder;
pub mod format;
pub mod file;
pub mod buffer;
pub mod time;
pub mod node;
pub mod mixer_node;
pub mod io_node;
pub mod channel_layout;
pub mod connection_point;
pub mod converter;
pub mod environment_node;
pub mod sink_node;
pub mod source_node;
pub mod unit;
pub mod sequencer;
pub mod midi_player;
pub mod speech;
pub mod application;
pub mod settings;
pub mod types;

// ── Re-exports ──────────────────────────────────────────────────────────────
pub use error::AudioResult;
pub use session::{AudioSession, Category, CategoryOptions, Mode};
pub use engine::AudioEngine;
pub use player::AudioPlayer;
pub use player_node::AudioPlayerNode;
pub use recorder::AudioRecorder;
pub use format::{AudioFormat, AudioCommonFormat};
pub use file::AudioFile;
pub use buffer::{AudioPCMBuffer, AudioCompressedBuffer};
pub use time::AudioTime;
pub use node::AudioNode;
pub use mixer_node::AudioMixerNode;
pub use io_node::{AudioInputNode, AudioOutputNode};
pub use channel_layout::AudioChannelLayout;
pub use connection_point::AudioConnectionPoint;
pub use converter::AudioConverter;
pub use environment_node::AudioEnvironmentNode;
pub use sink_node::AudioSinkNode;
pub use source_node::AudioSourceNode;
pub use unit::*;
pub use sequencer::AudioSequencer;
pub use midi_player::MIDIPlayer;
pub use speech::*;
pub use application::AudioApplication;
pub use types::*;

// ═══════════════════════════════════════════════════════════════════════════
//  Top-level convenience functions
// ═══════════════════════════════════════════════════════════════════════════

/// Returns `true` — AVFAudio is always available on Apple platforms.
pub fn is_available() -> bool { true }

/// Speak the given text using the default voice.
///
/// ```rust,ignore
/// avfaudio::speak("Turn left in 500 meters");
/// ```
pub fn speak(text: &str) {
    let synth = SpeechSynthesizer::new();
    let utt = SpeechUtterance::new(text);
    synth.speak(&utt);
    // Synthesizer must stay alive while speaking; spin briefly.
    while synth.is_speaking() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// Obtain the shared audio session (shorthand for `AudioSession::shared()`).
pub fn audio_session() -> AudioSession { AudioSession::shared() }

// ═══════════════════════════════════════════════════════════════════════════
//  Internal ObjC dispatch helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Retain an ObjC object (increment refcount). Returns the same pointer.
#[inline]
pub(crate) unsafe fn retain(obj: Id) -> Id {
    if !obj.is_null() { CFRetain(obj as CFTypeRef); }
    obj
}

/// Release an ObjC object (decrement refcount).
#[inline]
pub(crate) unsafe fn release(obj: Id) {
    if !obj.is_null() { CFRelease(obj as CFTypeRef); }
}

#[inline]
pub(crate) unsafe fn msg_send_f64(obj: Id, sel_name: &[u8]) -> f64 {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> f64 =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_f32(obj: Id, sel_name: &[u8]) -> f32 {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> f32 =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_isize(obj: Id, sel_name: &[u8]) -> isize {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> isize =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_usize(obj: Id, sel_name: &[u8]) -> usize {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> usize =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_bool(obj: Id, sel_name: &[u8]) -> bool {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> bool =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_id(obj: Id, sel_name: &[u8]) -> Id {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> Id =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel)
}

#[inline]
pub(crate) unsafe fn msg_send_void(obj: Id, sel_name: &[u8]) {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel);
}

#[inline]
pub(crate) unsafe fn msg_send_void_id(obj: Id, sel_name: &[u8], arg: Id) {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel, Id) =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel, arg);
}

#[inline]
pub(crate) unsafe fn msg_send_set_bool(obj: Id, sel_name: &[u8], val: bool) {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel, bool) =
        core::mem::transmute(objc_msgSend as *const ());
    f(obj, sel, val);
}

/// DRY: call `[obj sel:arg error:&err]` → `anyhow::Result<()>`.
macro_rules! objc_try {
    // (obj, "sel:error:", arg_id)
    ($obj:expr, $sel:expr, $arg:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            apple_objc_sys::Id, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $arg, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // (obj, "sel:val:error:", bool_val)
    (bool $obj:expr, $sel:expr, $val:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            bool, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $val, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // (obj, "sel:val:error:", f64_val)
    (f64 $obj:expr, $sel:expr, $val:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            f64, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $val, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // (obj, "sel:val:error:", f32_val)
    (f32 $obj:expr, $sel:expr, $val:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            f32, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $val, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // (obj, "sel:val:error:", isize_val)
    (isize $obj:expr, $sel:expr, $val:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            isize, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $val, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // (obj, "sel:val:error:", usize_val)
    (usize $obj:expr, $sel:expr, $val:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel,
            usize, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, $val, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};

    // no-arg: (obj, "startAndReturnError:")
    (noarg $obj:expr, $sel:expr) => {{
        let sel = apple_objc_sys::sel_registerName($sel.as_ptr());
        let mut err: apple_objc_sys::Id = apple_objc_sys::NIL;
        let f: unsafe extern "C" fn(
            apple_objc_sys::Id, apple_objc_sys::Sel, *mut apple_objc_sys::Id,
        ) -> bool = core::mem::transmute(apple_objc_sys::objc_msgSend as *const ());
        let ok = f($obj, sel, &mut err);
        if ok { Ok(()) } else { Err($crate::anyhow!("ObjC call failed")) }
    }};
}

pub(crate) use objc_try;
