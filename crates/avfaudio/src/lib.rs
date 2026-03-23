//! Apple AVFAudio — audio playback, recording, and processing from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 7+.
//!
//! Wraps AVFAudio for audio engine, players, recorders, and audio sessions.
//! The Swift helper is compiled and linked automatically via `build.rs` —
//! no manual `dlopen` or `load_helper()` needed.
//!
//! # Quick start
//!
//! ```ignore
//! assert!(avfaudio::is_available());
//!
//! // Create an audio engine and query output format
//! let engine = avfaudio::AudioEngine::new();
//! let (sample_rate, channels) = engine.output_format();
//! println!("Output: {sample_rate} Hz, {channels} ch");
//!
//! // Play an audio file
//! if let Some(player) = avfaudio::AudioPlayer::new("/path/to/sound.wav") {
//!     player.set_volume(0.8);
//!     player.play();
//!     println!("Duration: {:.1}s", player.duration());
//! }
//! ```

//!
//! ## Citation
//!
//! ```bibtex
//! @software{rswift,
//!   author       = {Eugene Hauptmann},
//!   title        = {rswift},
//!   year         = {2025},
//!   url          = {https://github.com/eugenehp/rswift},
//!   note         = {Build native Apple apps from Rust}
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

apple_sys_helpers::apple_framework!(c"avfaudio_available");

// ── FFI declarations (linked via build.rs → libSwiftUIHelper.dylib) ─────────

unsafe extern "C" {
    fn avfaudio_engine_create() -> *mut c_void;
    fn avfaudio_engine_start(ptr: *mut c_void) -> bool;
    fn avfaudio_engine_stop(ptr: *mut c_void);
    fn avfaudio_engine_is_running(ptr: *mut c_void) -> bool;
    fn avfaudio_engine_release(ptr: *mut c_void);
    fn avfaudio_engine_output_format(
        ptr: *mut c_void,
        sample_rate: *mut f64,
        channels: *mut u32,
    );

    fn avfaudio_player_create(path: *const u8, len: usize) -> *mut c_void;
    fn avfaudio_player_play(ptr: *mut c_void) -> bool;
    fn avfaudio_player_pause(ptr: *mut c_void);
    fn avfaudio_player_stop(ptr: *mut c_void);
    fn avfaudio_player_is_playing(ptr: *mut c_void) -> bool;
    fn avfaudio_player_duration(ptr: *mut c_void) -> f64;
    fn avfaudio_player_current_time(ptr: *mut c_void) -> f64;
    fn avfaudio_player_set_current_time(ptr: *mut c_void, time: f64);
    fn avfaudio_player_set_volume(ptr: *mut c_void, volume: f32);
    fn avfaudio_player_volume(ptr: *mut c_void) -> f32;
    fn avfaudio_player_set_loops(ptr: *mut c_void, loops: isize);
    fn avfaudio_player_release(ptr: *mut c_void);
}

// ── AudioEngine ─────────────────────────────────────────────────────────────

/// An audio engine for real-time audio processing.
///
/// Wraps `AVAudioEngine`. Create, start, stop, and query the output format.
pub struct AudioEngine {
    handle: *mut c_void,
}

impl AudioEngine {
    /// Create a new audio engine.
    pub fn new() -> Self {
        let h = unsafe { avfaudio_engine_create() };
        assert!(!h.is_null(), "Failed to create AVAudioEngine");
        Self { handle: h }
    }

    /// Start the audio engine. Returns `true` on success.
    pub fn start(&self) -> bool {
        unsafe { avfaudio_engine_start(self.handle) }
    }

    /// Stop the audio engine.
    pub fn stop(&self) {
        unsafe { avfaudio_engine_stop(self.handle) }
    }

    /// Whether the engine is currently running.
    pub fn is_running(&self) -> bool {
        unsafe { avfaudio_engine_is_running(self.handle) }
    }

    /// Query the output node's format: `(sample_rate_hz, channel_count)`.
    pub fn output_format(&self) -> (f64, u32) {
        let mut sr = 0.0f64;
        let mut ch = 0u32;
        unsafe { avfaudio_engine_output_format(self.handle, &mut sr, &mut ch) }
        (sr, ch)
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        unsafe { avfaudio_engine_release(self.handle) }
    }
}

// ── AudioPlayer ─────────────────────────────────────────────────────────────

/// A simple audio file player.
///
/// Wraps `AVAudioPlayer`. Load a file, control playback, volume, and looping.
///
/// Supported formats: WAV, MP3, AAC, AIFF, CAF, M4A.
pub struct AudioPlayer {
    handle: *mut c_void,
}

impl AudioPlayer {
    /// Create a player for the audio file at `path`.
    ///
    /// Returns `None` if the file cannot be opened.
    pub fn new(path: &str) -> Option<Self> {
        let h = unsafe { avfaudio_player_create(path.as_ptr(), path.len()) };
        if h.is_null() { None } else { Some(Self { handle: h }) }
    }

    /// Start playback. Returns `true` on success.
    pub fn play(&self) -> bool {
        unsafe { avfaudio_player_play(self.handle) }
    }

    /// Pause playback.
    pub fn pause(&self) {
        unsafe { avfaudio_player_pause(self.handle) }
    }

    /// Stop playback and reset to the beginning.
    pub fn stop(&self) {
        unsafe { avfaudio_player_stop(self.handle) }
    }

    /// Whether audio is currently playing.
    pub fn is_playing(&self) -> bool {
        unsafe { avfaudio_player_is_playing(self.handle) }
    }

    /// Total duration in seconds.
    pub fn duration(&self) -> f64 {
        unsafe { avfaudio_player_duration(self.handle) }
    }

    /// Current playback position in seconds.
    pub fn current_time(&self) -> f64 {
        unsafe { avfaudio_player_current_time(self.handle) }
    }

    /// Seek to a position in seconds.
    pub fn set_current_time(&self, time: f64) {
        unsafe { avfaudio_player_set_current_time(self.handle, time) }
    }

    /// Set playback volume (0.0 – 1.0).
    pub fn set_volume(&self, volume: f32) {
        unsafe { avfaudio_player_set_volume(self.handle, volume) }
    }

    /// Current playback volume (0.0 – 1.0).
    pub fn volume(&self) -> f32 {
        unsafe { avfaudio_player_volume(self.handle) }
    }

    /// Set the number of loops.
    ///
    /// - `0` = play once (default)
    /// - `-1` = loop forever
    /// - `N` = play N + 1 times total
    pub fn set_loops(&self, loops: isize) {
        unsafe { avfaudio_player_set_loops(self.handle, loops) }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        unsafe { avfaudio_player_release(self.handle) }
    }
}
