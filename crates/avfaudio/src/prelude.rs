//! Convenience re-exports for common usage.
//!
//! ```rust,ignore
//! use avfaudio::prelude::*;
//! ```

pub use crate::error::AudioResult;
pub use anyhow::{anyhow, bail, ensure, Context};
pub use crate::session::{AudioSession, Category, CategoryOptions, Mode, RouteSharingPolicy};
pub use crate::engine::AudioEngine;
pub use crate::player::AudioPlayer;
pub use crate::player_node::AudioPlayerNode;
pub use crate::recorder::AudioRecorder;
pub use crate::format::{AudioFormat, AudioCommonFormat};
pub use crate::file::AudioFile;
pub use crate::buffer::AudioPCMBuffer;
pub use crate::time::AudioTime;
pub use crate::mixer_node::AudioMixerNode;
pub use crate::io_node::{AudioInputNode, AudioOutputNode};
pub use crate::converter::AudioConverter;
pub use crate::speech::{SpeechSynthesizer, SpeechUtterance, SpeechBoundary};
pub use crate::types::*;
