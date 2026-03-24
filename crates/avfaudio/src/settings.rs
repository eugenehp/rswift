//! Audio settings keys — used with `AVAudioRecorder`, `AVAudioFormat`, etc.
//!
//! These correspond to the `AVAudioSettings.h` / `AVFormatIDKey`, etc.

/// Keys for audio settings dictionaries.
pub mod keys {
    pub const FORMAT_ID: &str = "AVFormatIDKey";
    pub const SAMPLE_RATE: &str = "AVSampleRateKey";
    pub const NUMBER_OF_CHANNELS: &str = "AVNumberOfChannelsKey";
    pub const LINEAR_PCM_BIT_DEPTH: &str = "AVLinearPCMBitDepthKey";
    pub const LINEAR_PCM_IS_FLOAT: &str = "AVLinearPCMIsFloatKey";
    pub const LINEAR_PCM_IS_BIG_ENDIAN: &str = "AVLinearPCMIsBigEndianKey";
    pub const LINEAR_PCM_IS_NON_INTERLEAVED: &str = "AVLinearPCMIsNonInterleaved";
    pub const ENCODER_AUDIO_QUALITY: &str = "AVEncoderAudioQualityKey";
    pub const ENCODER_BIT_RATE: &str = "AVEncoderBitRateKey";
    pub const ENCODER_BIT_RATE_PER_CHANNEL: &str = "AVEncoderBitRatePerChannelKey";
    pub const ENCODER_BIT_DEPTH_HINT: &str = "AVEncoderBitDepthHintKey";
    pub const CHANNEL_LAYOUT: &str = "AVChannelLayoutKey";
}

/// Common `kAudioFormat*` FourCC values.
pub mod format_id {
    pub const LINEAR_PCM: u32 = 0x6C70636D; // 'lpcm'
    pub const APPLE_LOSSLESS: u32 = 0x616C6163; // 'alac'
    pub const AAC: u32 = 0x61616320; // 'aac '
    pub const MPEG4_AAC: u32 = AAC;
    pub const MPEG_LAYER3: u32 = 0x2E6D7033; // '.mp3'
    pub const APPLE_IMA4: u32 = 0x696D6134; // 'ima4'
    pub const ALAW: u32 = 0x616C6177; // 'alaw'
    pub const ULAW: u32 = 0x756C6177; // 'ulaw'
    pub const FLAC: u32 = 0x666C6163; // 'flac'
    pub const OPUS: u32 = 0x6F707573; // 'opus'
}

/// Audio quality levels for `AVEncoderAudioQualityKey`.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioQuality {
    Min = 0,
    Low = 0x20,
    Medium = 0x40,
    High = 0x60,
    Max = 0x7F,
}
