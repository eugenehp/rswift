//! `AVAudioFormat` — audio format description.

use apple_objc_sys::*;
use crate::*;
use std::fmt;

/// Common audio sample formats.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCommonFormat {
    Other = 0,
    PCMFloat32 = 1,
    PCMFloat64 = 2,
    PCMInt16 = 3,
    PCMInt32 = 4,
}

/// An audio format description.
///
/// Wraps `AVAudioFormat`. Instances are immutable.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudioformat>
pub struct AudioFormat {
    pub(crate) h: Id,
    pub(crate) owned: bool,
}

unsafe impl Send for AudioFormat {}
unsafe impl Sync for AudioFormat {}

impl AudioFormat {
    /// Wrap an existing `AVAudioFormat*` (does **not** retain — caller manages lifetime).
    pub(crate) fn from_raw(h: Id) -> Self { Self { h, owned: false } }

    /// Wrap and take ownership (will release on drop).
    pub(crate) fn from_raw_owned(h: Id) -> Self { Self { h, owned: true } }

    pub fn as_raw(&self) -> Id { self.h }

    /// Standard deinterleaved float format.
    pub fn standard(sample_rate: f64, channels: AudioChannelCount) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVAudioFormat\0"), alloc];
            let sel = sel_registerName(b"initStandardFormatWithSampleRate:channels:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, u32) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, sample_rate, channels);
            if h.is_null() { None } else { Some(Self { h, owned: true }) }
        }
    }

    /// Common format with sample rate, channels, and interleaving.
    pub fn with_common_format(
        format: AudioCommonFormat, sample_rate: f64,
        channels: AudioChannelCount, interleaved: bool,
    ) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVAudioFormat\0"), alloc];
            let sel = sel_registerName(b"initWithCommonFormat:sampleRate:channels:interleaved:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize, f64, u32, bool) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, format as usize, sample_rate, channels, interleaved);
            if h.is_null() { None } else { Some(Self { h, owned: true }) }
        }
    }

    /// Initialize from a settings dictionary (see `settings` module).
    pub fn with_settings(settings: Id) -> Option<Self> {
        unsafe {
            let obj = msg_send![class!(b"AVAudioFormat\0"), alloc];
            let h = msg_send![obj, initWithSettings: settings];
            if h.is_null() { None } else { Some(Self { h, owned: true }) }
        }
    }

    // ── Properties ──────────────────────────────────────────────────────

    /// Whether the format is standard deinterleaved float.
    pub fn is_standard(&self) -> bool { unsafe { msg_send_bool(self.h, b"isStandard\0") } }

    /// The common format type.
    pub fn common_format(&self) -> AudioCommonFormat {
        let v = unsafe { msg_send_usize(self.h, b"commonFormat\0") };
        match v {
            1 => AudioCommonFormat::PCMFloat32,
            2 => AudioCommonFormat::PCMFloat64,
            3 => AudioCommonFormat::PCMInt16,
            4 => AudioCommonFormat::PCMInt32,
            _ => AudioCommonFormat::Other,
        }
    }

    /// Number of channels.
    pub fn channel_count(&self) -> AudioChannelCount {
        unsafe { msg_send_usize(self.h, b"channelCount\0") as AudioChannelCount }
    }

    /// Sample rate in Hz.
    pub fn sample_rate(&self) -> f64 { unsafe { msg_send_f64(self.h, b"sampleRate\0") } }

    /// Whether samples are interleaved.
    pub fn is_interleaved(&self) -> bool { unsafe { msg_send_bool(self.h, b"isInterleaved\0") } }

    /// Get the channel layout (if any).
    pub fn channel_layout(&self) -> Option<AudioChannelLayout> {
        unsafe {
            let h = msg_send_id(self.h, b"channelLayout\0");
            if h.is_null() { None } else { Some(AudioChannelLayout::from_raw(h)) }
        }
    }

    /// Check equality with another format.
    pub fn is_equal(&self, other: &AudioFormat) -> bool {
        unsafe { msg_send_t![bool; self.h, isEqual: other.h] }
    }
}

impl Clone for AudioFormat {
    fn clone(&self) -> Self {
        unsafe { retain(self.h); }
        Self { h: self.h, owned: true }
    }
}

impl Drop for AudioFormat {
    fn drop(&mut self) {
        if self.owned { unsafe { release(self.h); } }
    }
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_name = match self.common_format() {
            AudioCommonFormat::PCMFloat32 => "PCM-f32",
            AudioCommonFormat::PCMFloat64 => "PCM-f64",
            AudioCommonFormat::PCMInt16 => "PCM-i16",
            AudioCommonFormat::PCMInt32 => "PCM-i32",
            AudioCommonFormat::Other => "other",
        };
        let layout = if self.is_interleaved() { "interleaved" } else { "planar" };
        write!(
            f, "{fmt_name} {:.0}Hz {}ch {layout}",
            self.sample_rate(), self.channel_count()
        )
    }
}

impl fmt::Debug for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioFormat")
            .field("format", &self.common_format())
            .field("sample_rate", &self.sample_rate())
            .field("channels", &self.channel_count())
            .field("interleaved", &self.is_interleaved())
            .field("standard", &self.is_standard())
            .finish()
    }
}
