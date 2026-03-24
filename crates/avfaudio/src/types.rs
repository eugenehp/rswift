//! Core audio type aliases used throughout AVFAudio.

/// A number of audio sample frames.
pub type AudioFrameCount = u32;

/// A position in an audio file or stream, in sample frames.
pub type AudioFramePosition = i64;

/// A count of audio packets.
pub type AudioPacketCount = u32;

/// A count of audio channels.
pub type AudioChannelCount = u32;

/// A bus on an audio node.
pub type AudioNodeBus = usize;
