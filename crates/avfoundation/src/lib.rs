//! Apple AVFoundation — media capture, playback, and editing from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! Wraps AVFoundation for media playback, asset inspection, and more.
//! The Swift helper is compiled and linked automatically via `build.rs` —
//! no manual `dlopen` or `load_helper()` needed.
//!
//! # Quick start — play a video/audio file
//!
//! ```ignore
//! assert!(avfoundation::is_available());
//!
//! let player = avfoundation::Player::new("https://example.com/video.mp4");
//! player.play();
//! println!("Rate: {}", player.rate());
//! println!("Duration: {:.1}s", player.duration());
//! ```
//!
//! # Quick start — inspect a media asset
//!
//! ```ignore
//! let asset = avfoundation::Asset::new("/path/to/movie.mov");
//! println!("Duration: {:.1}s", asset.duration());
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

apple_sys_helpers::apple_framework!(c"avfoundation_available"; "macos", "ios", "tvos", "xros");

// ── FFI declarations (linked via build.rs → libSwiftUIHelper.dylib) ─────────

unsafe extern "C" {
    fn avfoundation_player_create(url: *const u8, len: usize) -> *mut c_void;
    fn avfoundation_player_play(ptr: *mut c_void);
    fn avfoundation_player_pause(ptr: *mut c_void);
    fn avfoundation_player_rate(ptr: *mut c_void) -> f32;
    fn avfoundation_player_set_rate(ptr: *mut c_void, rate: f32);
    fn avfoundation_player_set_volume(ptr: *mut c_void, volume: f32);
    fn avfoundation_player_volume(ptr: *mut c_void) -> f32;
    fn avfoundation_player_is_muted(ptr: *mut c_void) -> bool;
    fn avfoundation_player_set_muted(ptr: *mut c_void, muted: bool);
    fn avfoundation_player_current_time(ptr: *mut c_void) -> f64;
    fn avfoundation_player_duration(ptr: *mut c_void) -> f64;
    fn avfoundation_player_seek(ptr: *mut c_void, seconds: f64);
    fn avfoundation_player_status(ptr: *mut c_void) -> isize;
    fn avfoundation_player_release(ptr: *mut c_void);

    fn avfoundation_asset_create(url: *const u8, len: usize) -> *mut c_void;
    fn avfoundation_asset_duration(ptr: *mut c_void) -> f64;
    fn avfoundation_asset_release(ptr: *mut c_void);
}

// ── Player status ───────────────────────────────────────────────────────────

/// AVPlayer status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    /// Status is not yet known.
    Unknown = 0,
    /// The player is ready to play.
    ReadyToPlay = 1,
    /// The player encountered an error.
    Failed = 2,
}

impl From<isize> for PlayerStatus {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::ReadyToPlay,
            2 => Self::Failed,
            _ => Self::Unknown,
        }
    }
}

// ── Player ──────────────────────────────────────────────────────────────────

/// A media player for audio and video content.
///
/// Wraps `AVPlayer`. Supports local files and remote URLs (HTTP/HTTPS, HLS).
///
/// # Example
///
/// ```ignore
/// let player = avfoundation::Player::new("/path/to/video.mp4");
/// player.set_volume(0.8);
/// player.play();
/// std::thread::sleep(std::time::Duration::from_secs(2));
/// println!("Position: {:.1}s / {:.1}s", player.current_time(), player.duration());
/// player.pause();
/// ```
pub struct Player {
    handle: *mut c_void,
}

impl Player {
    /// Create a player for the given URL (file path or HTTP URL).
    pub fn new(url: &str) -> Self {
        let h = unsafe { avfoundation_player_create(url.as_ptr(), url.len()) };
        assert!(!h.is_null(), "Failed to create AVPlayer");
        Self { handle: h }
    }

    /// Start or resume playback.
    pub fn play(&self) {
        unsafe { avfoundation_player_play(self.handle) }
    }

    /// Pause playback.
    pub fn pause(&self) {
        unsafe { avfoundation_player_pause(self.handle) }
    }

    /// Current playback rate (0.0 = paused, 1.0 = normal).
    pub fn rate(&self) -> f32 {
        unsafe { avfoundation_player_rate(self.handle) }
    }

    /// Set the playback rate. Use 0.0 to pause, 1.0 for normal, 2.0 for 2x, etc.
    pub fn set_rate(&self, rate: f32) {
        unsafe { avfoundation_player_set_rate(self.handle, rate) }
    }

    /// Current volume (0.0 – 1.0).
    pub fn volume(&self) -> f32 {
        unsafe { avfoundation_player_volume(self.handle) }
    }

    /// Set volume (0.0 – 1.0).
    pub fn set_volume(&self, volume: f32) {
        unsafe { avfoundation_player_set_volume(self.handle, volume) }
    }

    /// Whether audio output is muted.
    pub fn is_muted(&self) -> bool {
        unsafe { avfoundation_player_is_muted(self.handle) }
    }

    /// Mute or unmute audio output.
    pub fn set_muted(&self, muted: bool) {
        unsafe { avfoundation_player_set_muted(self.handle, muted) }
    }

    /// Current playback position in seconds.
    pub fn current_time(&self) -> f64 {
        unsafe { avfoundation_player_current_time(self.handle) }
    }

    /// Total duration in seconds (0.0 if unknown or live stream).
    pub fn duration(&self) -> f64 {
        unsafe { avfoundation_player_duration(self.handle) }
    }

    /// Seek to a position in seconds.
    pub fn seek(&self, seconds: f64) {
        unsafe { avfoundation_player_seek(self.handle, seconds) }
    }

    /// Current player status.
    pub fn status(&self) -> PlayerStatus {
        let s = unsafe { avfoundation_player_status(self.handle) };
        PlayerStatus::from(s)
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe { avfoundation_player_release(self.handle) }
    }
}

// ── Asset ───────────────────────────────────────────────────────────────────

/// A media asset for inspecting file metadata.
///
/// Wraps `AVURLAsset`. Use to query duration and other properties.
///
/// # Example
///
/// ```ignore
/// let asset = avfoundation::Asset::new("/path/to/movie.mov");
/// println!("Duration: {:.1}s", asset.duration());
/// ```
pub struct Asset {
    handle: *mut c_void,
}

impl Asset {
    /// Create an asset from a file path or URL string.
    pub fn new(url: &str) -> Self {
        let h = unsafe { avfoundation_asset_create(url.as_ptr(), url.len()) };
        assert!(!h.is_null(), "Failed to create AVURLAsset");
        Self { handle: h }
    }

    /// Total duration in seconds.
    pub fn duration(&self) -> f64 {
        unsafe { avfoundation_asset_duration(self.handle) }
    }
}

impl Drop for Asset {
    fn drop(&mut self) {
        unsafe { avfoundation_asset_release(self.handle) }
    }
}
