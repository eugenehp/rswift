//! Apple AudioToolbox — system sounds and audio services from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 2+, tvOS 9+.
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

apple_sys_helpers::apple_framework!(c"audiotoolbox_available");

unsafe extern "C" {
    fn audiotoolbox_play_system_sound(id: u32);
    fn audiotoolbox_play_alert_sound(id: u32);
    fn audiotoolbox_create_system_sound(p: *const u8, l: usize, out: *mut u32) -> i32;
    fn audiotoolbox_dispose_system_sound(id: u32) -> i32;
}

/// Play a system sound by ID.
pub fn play_system_sound(sound_id: u32) {
    unsafe { audiotoolbox_play_system_sound(sound_id) }
}

/// Play a system sound as an alert (may include vibration on iOS).
pub fn play_alert_sound(sound_id: u32) {
    unsafe { audiotoolbox_play_alert_sound(sound_id) }
}

/// A handle to a loaded system sound file.
pub struct SystemSound {
    id: u32,
}

impl SystemSound {
    /// Load a sound from a file path.
    pub fn new(path: &str) -> Result<Self, i32> {
        let mut id = 0u32;
        let status = unsafe { audiotoolbox_create_system_sound(path.as_ptr(), path.len(), &mut id) };
        if status == 0 { Ok(Self { id }) } else { Err(status) }
    }

    /// Play the sound.
    pub fn play(&self) {
        unsafe { audiotoolbox_play_system_sound(self.id) }
    }

    /// Play as an alert sound.
    pub fn play_alert(&self) {
        unsafe { audiotoolbox_play_alert_sound(self.id) }
    }

    /// Sound ID.
    pub fn id(&self) -> u32 { self.id }
}

impl Drop for SystemSound {
    fn drop(&mut self) {
        unsafe { audiotoolbox_dispose_system_sound(self.id); }
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
