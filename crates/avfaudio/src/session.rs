//! `AVAudioSession` — configure audio behavior for your app.
//!
//! # Builder pattern
//!
//! ```rust,ignore
//! use avfaudio::prelude::*;
//!
//! AudioSession::shared()
//!     .configure()
//!     .category(Category::PlayAndRecord)
//!     .mode(Mode::VoiceChat)
//!     .options(CategoryOptions::ALLOW_BLUETOOTH_HFP | CategoryOptions::DEFAULT_TO_SPEAKER)
//!     .preferred_sample_rate(48_000.0)
//!     .preferred_io_buffer_duration(0.005)
//!     .activate()?;
//! ```

use apple_objc_sys::*;
use crate::{msg_send_id, msg_send_f64, msg_send_f32, msg_send_bool, msg_send_isize, msg_send_usize};
use crate::AudioResult;
use crate::objc_try;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
//  Enums
// ═══════════════════════════════════════════════════════════════════════════

/// Audio session category.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosessioncategory>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    /// Background sounds (rain, engine noise). Mixes with other audio.
    Ambient,
    /// Background sounds. Other music **stops** playing.
    SoloAmbient,
    /// Music/media playback.
    Playback,
    /// Audio recording.
    Record,
    /// Simultaneous recording and playback.
    PlayAndRecord,
    /// Multiple routes simultaneously.
    MultiRoute,
}

impl Category {
    pub(crate) unsafe fn as_id(self) -> Id {
        let sym: &[u8] = match self {
            Self::Ambient => b"AVAudioSessionCategoryAmbient\0",
            Self::SoloAmbient => b"AVAudioSessionCategorySoloAmbient\0",
            Self::Playback => b"AVAudioSessionCategoryPlayback\0",
            Self::Record => b"AVAudioSessionCategoryRecord\0",
            Self::PlayAndRecord => b"AVAudioSessionCategoryPlayAndRecord\0",
            Self::MultiRoute => b"AVAudioSessionCategoryMultiRoute\0",
        };
        global_string_const(sym) as Id
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ambient => "Ambient",
            Self::SoloAmbient => "SoloAmbient",
            Self::Playback => "Playback",
            Self::Record => "Record",
            Self::PlayAndRecord => "PlayAndRecord",
            Self::MultiRoute => "MultiRoute",
        })
    }
}

// ── CategoryOptions ─────────────────────────────────────────────────────────

bitflags::bitflags! {
    /// Options that customize the behavior of an audio session category.
    ///
    /// <https://developer.apple.com/documentation/avfaudio/avaudiosession/categoryoptions>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CategoryOptions: u64 {
        const MIX_WITH_OTHERS = 0x1;
        const DUCK_OTHERS = 0x2;
        const ALLOW_BLUETOOTH_HFP = 0x4;
        const DEFAULT_TO_SPEAKER = 0x8;
        const INTERRUPT_SPOKEN_AUDIO_AND_MIX_WITH_OTHERS = 0x11;
        const ALLOW_BLUETOOTH_A2DP = 0x20;
        const ALLOW_AIR_PLAY = 0x40;
        const OVERRIDE_MUTED_MICROPHONE_INTERRUPTION = 0x80;
    }
}

// ── Mode ────────────────────────────────────────────────────────────────────

/// Audio session mode.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosessionmode>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Default,
    VoiceChat,
    GameChat,
    VideoRecording,
    Measurement,
    MoviePlayback,
    VideoChat,
    SpokenAudio,
    VoicePrompt,
}

impl Mode {
    pub(crate) unsafe fn as_id(self) -> Id {
        let sym: &[u8] = match self {
            Self::Default => b"AVAudioSessionModeDefault\0",
            Self::VoiceChat => b"AVAudioSessionModeVoiceChat\0",
            Self::GameChat => b"AVAudioSessionModeGameChat\0",
            Self::VideoRecording => b"AVAudioSessionModeVideoRecording\0",
            Self::Measurement => b"AVAudioSessionModeMeasurement\0",
            Self::MoviePlayback => b"AVAudioSessionModeMoviePlayback\0",
            Self::VideoChat => b"AVAudioSessionModeVideoChat\0",
            Self::SpokenAudio => b"AVAudioSessionModeSpokenAudio\0",
            Self::VoicePrompt => b"AVAudioSessionModeVoicePrompt\0",
        };
        global_string_const(sym) as Id
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Route sharing policy.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteSharingPolicy {
    Default = 0, LongFormAudio = 1, Independent = 2, LongFormVideo = 3,
}

bitflags::bitflags! {
    /// Options for `set_active`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SetActiveOptions: u64 {
        const NOTIFY_OTHERS_ON_DEACTIVATION = 1;
    }
}

/// Override the output audio port.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortOverride { None = 0, Speaker = 0x73706B72 }

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionType { Ended = 0, Began = 1 }

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteChangeReason {
    Unknown = 0, NewDeviceAvailable = 1, OldDeviceUnavailable = 2,
    CategoryChange = 3, Override = 4, WakeFromSleep = 6,
    NoSuitableRouteForCategory = 7, RouteConfigurationChange = 8,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOType { NotSpecified = 0, Aggregated = 1 }

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptStyle {
    None = 0x6E6F6E65, Short = 0x73687274, Normal = 0x6E726D6C,
}

#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingMode {
    NotApplicable = 0, MonoStereo = 1, Surround = 2,
    SpatialAudio = 3, DolbyAudio = 4, DolbyAtmos = 5,
}

// ═══════════════════════════════════════════════════════════════════════════
//  AudioSession
// ═══════════════════════════════════════════════════════════════════════════

/// The shared audio session for your app.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiosession>
pub struct AudioSession { h: Id }

unsafe impl Send for AudioSession {}
unsafe impl Sync for AudioSession {}

impl AudioSession {
    /// Obtain the shared audio session singleton.
    pub fn shared() -> Self {
        unsafe {
            let cls = class!(b"AVAudioSession\0");
            let h: Id = msg_send_id(cls as Id, b"sharedInstance\0");
            Self { h }
        }
    }

    /// Alias kept for backward-compat with the published crate.
    pub fn shared_instance() -> Self { Self::shared() }

    /// Start a fluent configuration builder.
    pub fn configure(&self) -> SessionBuilder<'_> { SessionBuilder::new(self) }

    pub fn as_raw(&self) -> Id { self.h }

    // ── Category ────────────────────────────────────────────────────────

    /// Set just the category.
    pub fn set_category(&self, category: Category) -> AudioResult {
        unsafe { objc_try!(self.h, b"setCategory:error:\0", category.as_id()) }
    }

    /// Set category + options.
    pub fn set_category_with_options(&self, category: Category, options: CategoryOptions) -> AudioResult {
        unsafe {
            let sel = sel_registerName(b"setCategory:withOptions:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, Id, u64, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, category.as_id(), options.bits(), &mut err);
            if ok { Ok(()) } else { Err(crate::anyhow!("failed to set audio session category with options")) }
        }
    }

    /// Set category + mode + options.
    pub fn set_category_mode_options(&self, category: Category, mode: Mode, options: CategoryOptions) -> AudioResult {
        unsafe {
            let sel = sel_registerName(b"setCategory:mode:options:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, u64, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, category.as_id(), mode.as_id(), options.bits(), &mut err);
            if ok { Ok(()) } else { Err(crate::anyhow!("AVAudioSession error")) }
        }
    }

    /// Set category + mode + policy + options.
    pub fn set_category_mode_policy_options(
        &self, category: Category, mode: Mode,
        policy: RouteSharingPolicy, options: CategoryOptions,
    ) -> AudioResult {
        unsafe {
            let sel = sel_registerName(b"setCategory:mode:routeSharingPolicy:options:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, usize, u64, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, category.as_id(), mode.as_id(), policy as usize, options.bits(), &mut err);
            if ok { Ok(()) } else { Err(crate::anyhow!("AVAudioSession error")) }
        }
    }

    pub fn route_sharing_policy(&self) -> RouteSharingPolicy {
        match unsafe { msg_send_usize(self.h, b"routeSharingPolicy\0") } {
            1 => RouteSharingPolicy::LongFormAudio, 2 => RouteSharingPolicy::Independent,
            3 => RouteSharingPolicy::LongFormVideo, _ => RouteSharingPolicy::Default,
        }
    }

    // ── Mode ────────────────────────────────────────────────────────────

    pub fn set_mode(&self, mode: Mode) -> AudioResult {
        unsafe { objc_try!(self.h, b"setMode:error:\0", mode.as_id()) }
    }

    // ── Activation ──────────────────────────────────────────────────────

    pub fn set_active(&self, active: bool) -> AudioResult {
        unsafe { objc_try!(bool self.h, b"setActive:error:\0", active) }
    }

    pub fn set_active_with_options(&self, active: bool, options: SetActiveOptions) -> AudioResult {
        unsafe {
            let sel = sel_registerName(b"setActive:withOptions:error:\0".as_ptr());
            let mut err: Id = NIL;
            let f: unsafe extern "C" fn(Id, Sel, bool, u64, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.h, sel, active, options.bits(), &mut err);
            if ok { Ok(()) } else { Err(crate::anyhow!("AVAudioSession error")) }
        }
    }

    /// Activate the session.
    pub fn activate(&self) -> AudioResult { self.set_active(true) }
    /// Deactivate the session.
    pub fn deactivate(&self) -> AudioResult { self.set_active(false) }

    // ── Routing ─────────────────────────────────────────────────────────

    pub fn override_output_audio_port(&self, port: PortOverride) -> AudioResult {
        unsafe { objc_try!(usize self.h, b"overrideOutputAudioPort:error:\0", port as usize) }
    }

    pub fn set_aggregated_io_preference(&self, io_type: IOType) -> AudioResult {
        unsafe { objc_try!(usize self.h, b"setAggregatedIOPreference:error:\0", io_type as usize) }
    }

    // ── Hardware state (read-only) ──────────────────────────────────────

    pub fn sample_rate(&self) -> f64 { unsafe { msg_send_f64(self.h, b"sampleRate\0") } }
    pub fn input_number_of_channels(&self) -> isize { unsafe { msg_send_isize(self.h, b"inputNumberOfChannels\0") } }
    pub fn output_number_of_channels(&self) -> isize { unsafe { msg_send_isize(self.h, b"outputNumberOfChannels\0") } }
    pub fn input_latency(&self) -> std::time::Duration { std::time::Duration::from_secs_f64(unsafe { msg_send_f64(self.h, b"inputLatency\0") }) }
    pub fn output_latency(&self) -> std::time::Duration { std::time::Duration::from_secs_f64(unsafe { msg_send_f64(self.h, b"outputLatency\0") }) }
    pub fn io_buffer_duration(&self) -> std::time::Duration { std::time::Duration::from_secs_f64(unsafe { msg_send_f64(self.h, b"IOBufferDuration\0") }) }
    pub fn maximum_input_number_of_channels(&self) -> isize { unsafe { msg_send_isize(self.h, b"maximumInputNumberOfChannels\0") } }
    pub fn maximum_output_number_of_channels(&self) -> isize { unsafe { msg_send_isize(self.h, b"maximumOutputNumberOfChannels\0") } }

    // ── Hardware preferences ────────────────────────────────────────────

    pub fn set_preferred_sample_rate(&self, rate: f64) -> AudioResult {
        unsafe { objc_try!(f64 self.h, b"setPreferredSampleRate:error:\0", rate) }
    }
    pub fn preferred_sample_rate(&self) -> f64 { unsafe { msg_send_f64(self.h, b"preferredSampleRate\0") } }

    pub fn set_preferred_io_buffer_duration(&self, dur: std::time::Duration) -> AudioResult {
        unsafe { objc_try!(f64 self.h, b"setPreferredIOBufferDuration:error:\0", dur.as_secs_f64()) }
    }
    pub fn preferred_io_buffer_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f64(unsafe { msg_send_f64(self.h, b"preferredIOBufferDuration\0") })
    }

    pub fn set_preferred_input_number_of_channels(&self, n: isize) -> AudioResult {
        unsafe { objc_try!(isize self.h, b"setPreferredInputNumberOfChannels:error:\0", n) }
    }
    pub fn set_preferred_output_number_of_channels(&self, n: isize) -> AudioResult {
        unsafe { objc_try!(isize self.h, b"setPreferredOutputNumberOfChannels:error:\0", n) }
    }

    // ── Input gain ──────────────────────────────────────────────────────

    pub fn is_input_gain_settable(&self) -> bool { unsafe { msg_send_bool(self.h, b"isInputGainSettable\0") } }
    pub fn input_gain(&self) -> f32 { unsafe { msg_send_f32(self.h, b"inputGain\0") } }
    pub fn set_input_gain(&self, gain: f32) -> AudioResult {
        unsafe { objc_try!(f32 self.h, b"setInputGain:error:\0", gain) }
    }
    pub fn is_input_available(&self) -> bool { unsafe { msg_send_bool(self.h, b"isInputAvailable\0") } }

    // ── Observation ─────────────────────────────────────────────────────

    pub fn is_other_audio_playing(&self) -> bool { unsafe { msg_send_bool(self.h, b"isOtherAudioPlaying\0") } }
    pub fn secondary_audio_should_be_silenced_hint(&self) -> bool { unsafe { msg_send_bool(self.h, b"secondaryAudioShouldBeSilencedHint\0") } }
    pub fn output_volume(&self) -> f32 { unsafe { msg_send_f32(self.h, b"outputVolume\0") } }

    pub fn prompt_style(&self) -> PromptStyle {
        match unsafe { msg_send_usize(self.h, b"promptStyle\0") } {
            0x6E6F6E65 => PromptStyle::None, 0x73687274 => PromptStyle::Short,
            _ => PromptStyle::Normal,
        }
    }

    // ── Misc boolean prefs ──────────────────────────────────────────────

    pub fn set_allow_haptics_and_system_sounds_during_recording(&self, v: bool) -> AudioResult {
        unsafe { objc_try!(bool self.h, b"setAllowHapticsAndSystemSoundsDuringRecording:error:\0", v) }
    }
    pub fn allow_haptics_and_system_sounds_during_recording(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"allowHapticsAndSystemSoundsDuringRecording\0") }
    }

    pub fn set_prefers_no_interruptions_from_system_alerts(&self, v: bool) -> AudioResult {
        unsafe { objc_try!(bool self.h, b"setPrefersNoInterruptionsFromSystemAlerts:error:\0", v) }
    }
    pub fn prefers_no_interruptions_from_system_alerts(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"prefersNoInterruptionsFromSystemAlerts\0") }
    }

    pub fn set_supports_multichannel_content(&self, v: bool) -> AudioResult {
        unsafe { objc_try!(bool self.h, b"setSupportsMultichannelContent:error:\0", v) }
    }
    pub fn supports_multichannel_content(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"supportsMultichannelContent\0") }
    }

    pub fn set_prefers_interruption_on_route_disconnect(&self, v: bool) -> AudioResult {
        unsafe { objc_try!(bool self.h, b"setPrefersInterruptionOnRouteDisconnect:error:\0", v) }
    }
    pub fn prefers_interruption_on_route_disconnect(&self) -> bool {
        unsafe { msg_send_bool(self.h, b"prefersInterruptionOnRouteDisconnect\0") }
    }
}

impl fmt::Display for AudioSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "AVAudioSession {{ rate: {} Hz, in: {}ch, out: {}ch, vol: {:.0}% }}",
            self.sample_rate(),
            self.input_number_of_channels(),
            self.output_number_of_channels(),
            self.output_volume() * 100.0,
        )
    }
}

impl fmt::Debug for AudioSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioSession")
            .field("sample_rate", &self.sample_rate())
            .field("input_channels", &self.input_number_of_channels())
            .field("output_channels", &self.output_number_of_channels())
            .field("output_volume", &self.output_volume())
            .finish()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  SessionBuilder — fluent configuration
// ═══════════════════════════════════════════════════════════════════════════

/// Fluent builder for configuring an `AudioSession`.
///
/// ```rust,ignore
/// AudioSession::shared()
///     .configure()
///     .category(Category::Playback)
///     .mode(Mode::Default)
///     .options(CategoryOptions::MIX_WITH_OTHERS)
///     .preferred_sample_rate(48_000.0)
///     .activate()?;
/// ```
pub struct SessionBuilder<'a> {
    session: &'a AudioSession,
    category: Option<Category>,
    mode: Option<Mode>,
    options: CategoryOptions,
    policy: Option<RouteSharingPolicy>,
    pref_sample_rate: Option<f64>,
    pref_io_buf_dur: Option<std::time::Duration>,
}

impl<'a> SessionBuilder<'a> {
    fn new(session: &'a AudioSession) -> Self {
        Self {
            session, category: None, mode: None,
            options: CategoryOptions::empty(), policy: None,
            pref_sample_rate: None, pref_io_buf_dur: None,
        }
    }

    pub fn category(mut self, c: Category) -> Self { self.category = Some(c); self }
    pub fn mode(mut self, m: Mode) -> Self { self.mode = Some(m); self }
    pub fn options(mut self, o: CategoryOptions) -> Self { self.options = o; self }
    pub fn policy(mut self, p: RouteSharingPolicy) -> Self { self.policy = Some(p); self }
    pub fn preferred_sample_rate(mut self, hz: f64) -> Self { self.pref_sample_rate = Some(hz); self }
    pub fn preferred_io_buffer_duration(mut self, dur: std::time::Duration) -> Self { self.pref_io_buf_dur = Some(dur); self }

    /// Apply all settings **without** activating the session.
    pub fn apply(&self) -> AudioResult {
        let s = self.session;
        if let Some(cat) = self.category {
            match (self.mode, self.policy) {
                (Some(m), Some(p)) => s.set_category_mode_policy_options(cat, m, p, self.options)?,
                (Some(m), None) => s.set_category_mode_options(cat, m, self.options)?,
                (None, _) if !self.options.is_empty() => s.set_category_with_options(cat, self.options)?,
                (None, _) => s.set_category(cat)?,
            }
        }
        if let Some(hz) = self.pref_sample_rate { s.set_preferred_sample_rate(hz)?; }
        if let Some(d) = self.pref_io_buf_dur { s.set_preferred_io_buffer_duration(d)?; }
        Ok(())
    }

    /// Apply all settings **and** activate the session.
    pub fn activate(self) -> AudioResult {
        self.apply()?;
        self.session.activate()
    }
}
