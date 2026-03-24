//! Apple MediaPlayer — system music playback and Now Playing from Rust.
//!
//! **Platform:** macOS 10.12+, iOS 3+, tvOS 14+.
//!
//! ```ignore
//! let player = mediaplayer::SystemMusicPlayer::shared();
//! println!("State: {:?}", player.playback_state());
//! player.play();
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped = 0, Playing = 1, Paused = 2,
    Interrupted = 3, SeekingForward = 4, SeekingBackward = 5,
}
impl From<isize> for PlaybackState {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Playing, 2=>Self::Paused, 3=>Self::Interrupted,
                  4=>Self::SeekingForward, 5=>Self::SeekingBackward, _=>Self::Stopped }
    }
}

/// System music player (wraps `MPMusicPlayerController`).
pub struct SystemMusicPlayer { inner: Id }

impl SystemMusicPlayer {
    /// The system-wide music player.
    pub fn shared() -> Self {
        Self { inner: unsafe { msg_send![class!(b"MPMusicPlayerController\0"), systemMusicPlayer] } }
    }

    /// The application's own music player.
    pub fn application_player() -> Self {
        Self { inner: unsafe { msg_send![class!(b"MPMusicPlayerController\0"), applicationMusicPlayer] } }
    }

    pub fn play(&self) { unsafe { msg_send_void![self.inner, play]; } }
    pub fn pause(&self) { unsafe { msg_send_void![self.inner, pause]; } }
    pub fn stop(&self) { unsafe { msg_send_void![self.inner, stop]; } }
    pub fn skip_to_next(&self) { unsafe { msg_send_void![self.inner, skipToNextItem]; } }
    pub fn skip_to_previous(&self) { unsafe { msg_send_void![self.inner, skipToPreviousItem]; } }
    pub fn skip_to_beginning(&self) { unsafe { msg_send_void![self.inner, skipToBeginning]; } }

    pub fn playback_state(&self) -> PlaybackState {
        unsafe { PlaybackState::from(msg_send_t![isize; self.inner, playbackState]) }
    }

    /// Current playback time in seconds.
    pub fn current_playback_time(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(b"currentPlaybackTime\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel)
        }
    }

    /// Set current playback time.
    pub fn set_current_playback_time(&self, time: f64) {
        unsafe {
            let sel = sel_registerName(b"setCurrentPlaybackTime:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, time);
        }
    }

    /// Now playing item title.
    pub fn now_playing_title(&self) -> Option<String> {
        unsafe {
            let item: Id = msg_send![self.inner, nowPlayingItem];
            if item.is_null() { return None; }
            let key = nsstring("title");
            let val = msg_send![item, valueForProperty: key];
            CFRelease(key as CFTypeRef);
            nsstring_to_string(val)
        }
    }

    /// Now playing item artist.
    pub fn now_playing_artist(&self) -> Option<String> {
        unsafe {
            let item: Id = msg_send![self.inner, nowPlayingItem];
            if item.is_null() { return None; }
            let key = nsstring("artist");
            let val = msg_send![item, valueForProperty: key];
            CFRelease(key as CFTypeRef);
            nsstring_to_string(val)
        }
    }

    /// Now playing item album.
    pub fn now_playing_album(&self) -> Option<String> {
        unsafe {
            let item: Id = msg_send![self.inner, nowPlayingItem];
            if item.is_null() { return None; }
            let key = nsstring("albumTitle");
            let val = msg_send![item, valueForProperty: key];
            CFRelease(key as CFTypeRef);
            nsstring_to_string(val)
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

/// Now Playing Info Center — for updating the lock screen / control center.
pub struct NowPlayingInfoCenter;

impl NowPlayingInfoCenter {
    /// Get the shared info center.
    pub fn shared_ptr() -> Id {
        unsafe { msg_send![class!(b"MPNowPlayingInfoCenter\0"), defaultCenter] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_player() {
        let p = SystemMusicPlayer::shared();
        let _ = p.playback_state();
        let _ = p.now_playing_title();
    }
}
