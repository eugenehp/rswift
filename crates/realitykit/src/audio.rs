//! Audio resources and playback controllers.


use core::ffi::c_void;

/// Spatial or non-spatial input mode for audio resources.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioInputMode {
    /// 3-D spatialised audio (default).
    Spatial    = 0,
    /// Flat stereo / mono (background music, UI sounds).
    NonSpatial = 1,
}

/// A loaded `AudioFileResource`.
pub struct AudioResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AudioResource {}
unsafe impl Sync for AudioResource {}
impl Clone for AudioResource {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; AudioResource { ptr: self.ptr } }
}
impl Drop for AudioResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl AudioResource {
    /// Load from an absolute or relative file path.
    pub fn load(path: &str, mode: AudioInputMode) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_audio_resource_load(path.as_ptr(), path.len(), mode as i32, err.as_mut_ptr(), err.len())
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(AudioResource { ptr: p })
        }
    }

    /// Load from the main bundle by name.
    pub fn load_named(name: &str, mode: AudioInputMode) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_audio_resource_load_named(name.as_ptr(), name.len(), mode as i32, err.as_mut_ptr(), err.len())
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(AudioResource { ptr: p })
        }
    }
}

/// A group of audio files that can be played as a single resource
/// (`AudioFileGroupResource`).
///
/// Loaded from a Reality Composer Pro scene bundle.
pub struct AudioGroupResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AudioGroupResource {}
unsafe impl Sync for AudioGroupResource {}
impl Clone for AudioGroupResource {
    fn clone(&self) -> Self { unsafe { realitykit_sys::rk_retain(self.ptr) }; AudioGroupResource { ptr: self.ptr } }
}
impl Drop for AudioGroupResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl AudioGroupResource {
    /// Load an `AudioFileGroupResource` by bundle scene name.
    pub fn load_named(name: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_audio_group_load_named(
                name.as_ptr(), name.len(), err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(AudioGroupResource { ptr: p })
        }
    }
}

/// Controls a running audio playback via `AudioPlaybackController`.
pub struct AudioController {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for AudioController {}
unsafe impl Sync for AudioController {}
impl Drop for AudioController {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl AudioController {
    pub fn pause(&self)  { unsafe { realitykit_sys::rk_audio_controller_pause(self.ptr) }; }
    pub fn resume(&self) { unsafe { realitykit_sys::rk_audio_controller_resume(self.ptr) }; }
    pub fn stop(&self)   { unsafe { realitykit_sys::rk_audio_controller_stop(self.ptr) }; }
    pub fn is_playing(&self) -> bool { unsafe { realitykit_sys::rk_audio_controller_is_playing(self.ptr) } }

    /// Get gain in decibels (negative = quieter).
    pub fn gain(&self) -> f64 { unsafe { realitykit_sys::rk_audio_controller_get_gain(self.ptr) } }
    /// Set gain in decibels.
    pub fn set_gain(&self, db: f64) { unsafe { realitykit_sys::rk_audio_controller_set_gain(self.ptr, db) }; }

    /// Playback speed multiplier (1.0 = normal).
    pub fn speed(&self) -> f64 { unsafe { realitykit_sys::rk_audio_controller_get_speed(self.ptr) } }
    pub fn set_speed(&self, v: f64) { unsafe { realitykit_sys::rk_audio_controller_set_speed(self.ptr, v) }; }
}
