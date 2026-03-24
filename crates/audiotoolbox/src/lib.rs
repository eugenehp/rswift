//! Apple AudioToolbox — system sounds and audio services from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 2+, tvOS 9+.
//!
//! Links AudioToolbox directly — no Swift bridge needed.
//!
//! # Quick start
//!
//! ```ignore
//! // Play a built-in system sound
//! audiotoolbox::play_system_sound(audiotoolbox::sounds::GLASS);
//!
//! // Play a custom sound file
//! let sound = audiotoolbox::SystemSound::new("/System/Library/Sounds/Ping.aiff").unwrap();
//! sound.play();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

/// AudioToolbox is always available on Apple platforms.
pub fn is_available() -> bool { true }

// ── Raw AudioToolbox C symbols ──────────────────────────────────────────────

type SystemSoundID = u32;
type OSStatus = i32;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn AudioServicesPlaySystemSound(sound_id: SystemSoundID);
    fn AudioServicesPlayAlertSound(sound_id: SystemSoundID);
    fn AudioServicesCreateSystemSoundID(url: *const core::ffi::c_void, out: *mut SystemSoundID) -> OSStatus;
    fn AudioServicesDisposeSystemSoundID(sound_id: SystemSoundID) -> OSStatus;

    // CoreFoundation helpers for URL construction
    fn CFStringCreateWithBytes(
        alloc: *const core::ffi::c_void,
        bytes: *const u8, num_bytes: isize,
        encoding: u32, is_external: bool,
    ) -> *const core::ffi::c_void;
    fn CFURLCreateWithFileSystemPath(
        alloc: *const core::ffi::c_void,
        path: *const core::ffi::c_void,
        style: isize, is_dir: bool,
    ) -> *const core::ffi::c_void;
    fn CFRelease(cf: *const core::ffi::c_void);
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;
const K_CF_URL_POSIX_PATH_STYLE: isize = 0;

/// Play a system sound by ID.
pub fn play_system_sound(sound_id: u32) {
    unsafe { AudioServicesPlaySystemSound(sound_id) }
}

/// Play a system sound as an alert (may include vibration on iOS).
pub fn play_alert_sound(sound_id: u32) {
    unsafe { AudioServicesPlayAlertSound(sound_id) }
}

/// A handle to a loaded system sound file.
pub struct SystemSound {
    id: u32,
}

impl SystemSound {
    /// Load a sound from a file path.
    pub fn new(path: &str) -> Result<Self, i32> {
        unsafe {
            let cf_str = CFStringCreateWithBytes(
                core::ptr::null(), path.as_ptr(), path.len() as isize,
                K_CF_STRING_ENCODING_UTF8, false,
            );
            if cf_str.is_null() { return Err(-1); }
            let cf_url = CFURLCreateWithFileSystemPath(
                core::ptr::null(), cf_str, K_CF_URL_POSIX_PATH_STYLE, false,
            );
            CFRelease(cf_str);
            if cf_url.is_null() { return Err(-1); }
            let mut id: SystemSoundID = 0;
            let status = AudioServicesCreateSystemSoundID(cf_url, &mut id);
            CFRelease(cf_url);
            if status == 0 { Ok(Self { id }) } else { Err(status) }
        }
    }

    /// Play the sound.
    pub fn play(&self) {
        unsafe { AudioServicesPlaySystemSound(self.id) }
    }

    /// Play as an alert sound.
    pub fn play_alert(&self) {
        unsafe { AudioServicesPlayAlertSound(self.id) }
    }

    /// Sound ID.
    pub fn id(&self) -> u32 { self.id }
}

impl Drop for SystemSound {
    fn drop(&mut self) {
        unsafe { AudioServicesDisposeSystemSoundID(self.id); }
    }
}

/// Well-known system sound IDs.
pub mod sounds {
    // macOS system sounds
    pub const GLASS: u32 = 1000;
    pub const BASSO: u32 = 1001;
    pub const BLOW: u32 = 1002;
    pub const BOTTLE: u32 = 1003;
    pub const FROG: u32 = 1004;
    pub const FUNK: u32 = 1005;
    pub const HERO: u32 = 1006;
    pub const MORSE: u32 = 1007;
    pub const PING: u32 = 1008;
    pub const POP: u32 = 1009;
    pub const PURR: u32 = 1010;
    pub const SOSUMI: u32 = 1011;
    pub const SUBMARINE: u32 = 1012;
    pub const TINK: u32 = 1013;

    // iOS well-known sounds
    pub const NEW_MAIL: u32 = 1000;
    pub const MAIL_SENT: u32 = 1001;
    pub const SMS_RECEIVED: u32 = 1003;
    pub const CALENDAR_ALERT: u32 = 1005;
    pub const LOCK: u32 = 1006;
    pub const KEY_PRESS: u32 = 1104;
    pub const PHOTO_SHUTTER: u32 = 1108;

    /// Vibrate (iOS only).
    pub const VIBRATE: u32 = 4095;
}
