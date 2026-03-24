//! ObjC selector constants for AVFoundation classes.
//! Use with `apple_objc_sys::sel_registerName(SEL.as_ptr())`.

#![allow(dead_code)]

pub mod player {
    pub const CLASS: &[u8] = b"AVPlayer\0";
    pub const SEL_PLAYER_WITH_URL: &[u8] = b"playerWithURL:\0";
    pub const SEL_PLAYER_WITH_PLAYER_ITEM: &[u8] = b"playerWithPlayerItem:\0";
    pub const SEL_PLAY: &[u8] = b"play\0";
    pub const SEL_PAUSE: &[u8] = b"pause\0";
    pub const SEL_RATE: &[u8] = b"rate\0";
    pub const SEL_SET_RATE: &[u8] = b"setRate:\0";
    pub const SEL_VOLUME: &[u8] = b"volume\0";
    pub const SEL_SET_VOLUME: &[u8] = b"setVolume:\0";
    pub const SEL_IS_MUTED: &[u8] = b"isMuted\0";
    pub const SEL_SET_MUTED: &[u8] = b"setMuted:\0";
    pub const SEL_CURRENT_TIME: &[u8] = b"currentTime\0";
    pub const SEL_SEEK_TO_TIME: &[u8] = b"seekToTime:\0";
    pub const SEL_SEEK_TO_TIME_TOLERANCES: &[u8] = b"seekToTime:toleranceBefore:toleranceAfter:\0";
    pub const SEL_STATUS: &[u8] = b"status\0";
    pub const SEL_CURRENT_ITEM: &[u8] = b"currentItem\0";
    pub const SEL_REPLACE_CURRENT_ITEM: &[u8] = b"replaceCurrentItemWithPlayerItem:\0";
    pub const SEL_ACTION_AT_ITEM_END: &[u8] = b"actionAtItemEnd\0";
    pub const SEL_SET_ACTION_AT_ITEM_END: &[u8] = b"setActionAtItemEnd:\0";
    pub const SEL_ALLOWS_EXTERNAL_PLAYBACK: &[u8] = b"allowsExternalPlayback\0";
    pub const SEL_IS_EXTERNAL_PLAYBACK_ACTIVE: &[u8] = b"isExternalPlaybackActive\0";
    pub const SEL_REASONABLE_TIME_FOR_PLAYBACK: &[u8] = b"reasonForWaitingToPlay\0";
    pub const SEL_TIME_CONTROL_STATUS: &[u8] = b"timeControlStatus\0";
    pub const SEL_PLAYBACK_COORDINATOR: &[u8] = b"playbackCoordinator\0";
    pub const SEL_DEFAULT_RATE: &[u8] = b"defaultRate\0";
    pub const SEL_SET_DEFAULT_RATE: &[u8] = b"setDefaultRate:\0";
}

pub mod player_item {
    pub const CLASS: &[u8] = b"AVPlayerItem\0";
    pub const SEL_PLAYER_ITEM_WITH_URL: &[u8] = b"playerItemWithURL:\0";
    pub const SEL_PLAYER_ITEM_WITH_ASSET: &[u8] = b"playerItemWithAsset:\0";
    pub const SEL_STATUS: &[u8] = b"status\0";
    pub const SEL_ERROR: &[u8] = b"error\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_CURRENT_TIME: &[u8] = b"currentTime\0";
    pub const SEL_FORWARD_PLAYBACK_END_TIME: &[u8] = b"forwardPlaybackEndTime\0";
    pub const SEL_SET_FORWARD_PLAYBACK_END_TIME: &[u8] = b"setForwardPlaybackEndTime:\0";
    pub const SEL_REVERSE_PLAYBACK_END_TIME: &[u8] = b"reversePlaybackEndTime\0";
    pub const SEL_LOADED_TIME_RANGES: &[u8] = b"loadedTimeRanges\0";
    pub const SEL_IS_PLAYBACK_LIKELY_TO_KEEP_UP: &[u8] = b"isPlaybackLikelyToKeepUp\0";
    pub const SEL_IS_PLAYBACK_BUFFER_EMPTY: &[u8] = b"isPlaybackBufferEmpty\0";
    pub const SEL_IS_PLAYBACK_BUFFER_FULL: &[u8] = b"isPlaybackBufferFull\0";
    pub const SEL_CAN_PLAY_REVERSE: &[u8] = b"canPlayReverse\0";
    pub const SEL_CAN_PLAY_FAST_FORWARD: &[u8] = b"canPlayFastForward\0";
    pub const SEL_CAN_PLAY_SLOW_FORWARD: &[u8] = b"canPlaySlowForward\0";
    pub const SEL_ASSET: &[u8] = b"asset\0";
    pub const SEL_TRACKS: &[u8] = b"tracks\0";
    pub const SEL_PREFERRED_PEAK_BIT_RATE: &[u8] = b"preferredPeakBitRate\0";
    pub const SEL_SET_PREFERRED_PEAK_BIT_RATE: &[u8] = b"setPreferredPeakBitRate:\0";
    pub const SEL_PRESENTATION_SIZE: &[u8] = b"presentationSize\0";
}

pub mod asset {
    pub const CLASS: &[u8] = b"AVAsset\0";
    pub const SEL_ASSET_WITH_URL: &[u8] = b"assetWithURL:\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_PREFERRED_RATE: &[u8] = b"preferredRate\0";
    pub const SEL_PREFERRED_VOLUME: &[u8] = b"preferredVolume\0";
    pub const SEL_PREFERRED_TRANSFORM: &[u8] = b"preferredTransform\0";
    pub const SEL_NATURAL_SIZE: &[u8] = b"naturalSize\0";
    pub const SEL_TRACKS: &[u8] = b"tracks\0";
    pub const SEL_METADATA: &[u8] = b"metadata\0";
    pub const SEL_CREATION_DATE: &[u8] = b"creationDate\0";
    pub const SEL_IS_PLAYABLE: &[u8] = b"isPlayable\0";
    pub const SEL_IS_EXPORTABLE: &[u8] = b"isExportable\0";
    pub const SEL_IS_READABLE: &[u8] = b"isReadable\0";
    pub const SEL_IS_COMPOSABLE: &[u8] = b"isComposable\0";
    pub const SEL_HAS_PROTECTED_CONTENT: &[u8] = b"hasProtectedContent\0";
}

pub mod url_asset {
    pub const CLASS: &[u8] = b"AVURLAsset\0";
    pub const SEL_URL_ASSET_WITH_URL_OPTIONS: &[u8] = b"URLAssetWithURL:options:\0";
    pub const SEL_URL: &[u8] = b"URL\0";
    pub const SEL_RESOURCE_LOADER: &[u8] = b"resourceLoader\0";
}

pub mod capture_session {
    pub const CLASS: &[u8] = b"AVCaptureSession\0";
    pub const SEL_CAN_ADD_INPUT: &[u8] = b"canAddInput:\0";
    pub const SEL_ADD_INPUT: &[u8] = b"addInput:\0";
    pub const SEL_REMOVE_INPUT: &[u8] = b"removeInput:\0";
    pub const SEL_CAN_ADD_OUTPUT: &[u8] = b"canAddOutput:\0";
    pub const SEL_ADD_OUTPUT: &[u8] = b"addOutput:\0";
    pub const SEL_REMOVE_OUTPUT: &[u8] = b"removeOutput:\0";
    pub const SEL_START_RUNNING: &[u8] = b"startRunning\0";
    pub const SEL_STOP_RUNNING: &[u8] = b"stopRunning\0";
    pub const SEL_IS_RUNNING: &[u8] = b"isRunning\0";
    pub const SEL_BEGIN_CONFIGURATION: &[u8] = b"beginConfiguration\0";
    pub const SEL_COMMIT_CONFIGURATION: &[u8] = b"commitConfiguration\0";
    pub const SEL_SESSION_PRESET: &[u8] = b"sessionPreset\0";
    pub const SEL_SET_SESSION_PRESET: &[u8] = b"setSessionPreset:\0";
    pub const SEL_INPUTS: &[u8] = b"inputs\0";
    pub const SEL_OUTPUTS: &[u8] = b"outputs\0";
    pub const SEL_IS_INTERRUPTED: &[u8] = b"isInterrupted\0";
    pub const SEL_IS_MULTITASKING_CAMERA_ACCESS_SUPPORTED: &[u8] = b"isMultitaskingCameraAccessSupported\0";
}

pub mod capture_device {
    pub const CLASS: &[u8] = b"AVCaptureDevice\0";
    pub const SEL_DEFAULT_DEVICE_WITH_MEDIA_TYPE: &[u8] = b"defaultDeviceWithMediaType:\0";
    pub const SEL_DEVICES_WITH_MEDIA_TYPE: &[u8] = b"devicesWithMediaType:\0";
    pub const SEL_UNIQUE_ID: &[u8] = b"uniqueID\0";
    pub const SEL_MODEL_ID: &[u8] = b"modelID\0";
    pub const SEL_LOCALIZED_NAME: &[u8] = b"localizedName\0";
    pub const SEL_MANUFACTURER: &[u8] = b"manufacturer\0";
    pub const SEL_POSITION: &[u8] = b"position\0";
    pub const SEL_IS_CONNECTED: &[u8] = b"isConnected\0";
    pub const SEL_HAS_MEDIA_TYPE: &[u8] = b"hasMediaType:\0";
    pub const SEL_LOCK_FOR_CONFIGURATION: &[u8] = b"lockForConfiguration:\0";
    pub const SEL_UNLOCK_FOR_CONFIGURATION: &[u8] = b"unlockForConfiguration\0";
    pub const SEL_ACTIVE_FORMAT: &[u8] = b"activeFormat\0";
    pub const SEL_FORMATS: &[u8] = b"formats\0";
    pub const SEL_HAS_TORCH: &[u8] = b"hasTorch\0";
    pub const SEL_IS_TORCH_AVAILABLE: &[u8] = b"isTorchAvailable\0";
    pub const SEL_HAS_FLASH: &[u8] = b"hasFlash\0";
    pub const SEL_IS_FLASH_AVAILABLE: &[u8] = b"isFlashAvailable\0";
    pub const SEL_AUTHORIZATION_STATUS_FOR_MEDIA_TYPE: &[u8] = b"authorizationStatusForMediaType:\0";
    pub const SEL_REQUEST_ACCESS_FOR_MEDIA_TYPE: &[u8] = b"requestAccessForMediaType:completionHandler:\0";
}

pub mod speech_synthesizer {
    pub const CLASS: &[u8] = b"AVSpeechSynthesizer\0";
    pub const SEL_SPEAK_UTTERANCE: &[u8] = b"speakUtterance:\0";
    pub const SEL_STOP_SPEAKING_AT_BOUNDARY: &[u8] = b"stopSpeakingAtBoundary:\0";
    pub const SEL_PAUSE_SPEAKING_AT_BOUNDARY: &[u8] = b"pauseSpeakingAtBoundary:\0";
    pub const SEL_CONTINUE_SPEAKING: &[u8] = b"continueSpeaking\0";
    pub const SEL_IS_SPEAKING: &[u8] = b"isSpeaking\0";
    pub const SEL_IS_PAUSED: &[u8] = b"isPaused\0";
}

pub mod speech_utterance {
    pub const CLASS: &[u8] = b"AVSpeechUtterance\0";
    pub const SEL_SPEECH_UTTERANCE_WITH_STRING: &[u8] = b"speechUtteranceWithString:\0";
    pub const SEL_RATE: &[u8] = b"rate\0";
    pub const SEL_SET_RATE: &[u8] = b"setRate:\0";
    pub const SEL_PITCH_MULTIPLIER: &[u8] = b"pitchMultiplier\0";
    pub const SEL_SET_PITCH_MULTIPLIER: &[u8] = b"setPitchMultiplier:\0";
    pub const SEL_VOLUME: &[u8] = b"volume\0";
    pub const SEL_SET_VOLUME: &[u8] = b"setVolume:\0";
    pub const SEL_VOICE: &[u8] = b"voice\0";
    pub const SEL_SET_VOICE: &[u8] = b"setVoice:\0";
    pub const SEL_PRE_UTTERANCE_DELAY: &[u8] = b"preUtteranceDelay\0";
    pub const SEL_SET_PRE_UTTERANCE_DELAY: &[u8] = b"setPreUtteranceDelay:\0";
    pub const SEL_POST_UTTERANCE_DELAY: &[u8] = b"postUtteranceDelay\0";
    pub const SEL_SET_POST_UTTERANCE_DELAY: &[u8] = b"setPostUtteranceDelay:\0";
}

pub mod speech_voice {
    pub const CLASS: &[u8] = b"AVSpeechSynthesisVoice\0";
    pub const SEL_VOICE_WITH_LANGUAGE: &[u8] = b"voiceWithLanguage:\0";
    pub const SEL_VOICE_WITH_IDENTIFIER: &[u8] = b"voiceWithIdentifier:\0";
    pub const SEL_SPEECH_VOICES: &[u8] = b"speechVoices\0";
    pub const SEL_LANGUAGE: &[u8] = b"language\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_QUALITY: &[u8] = b"quality\0";
}

pub mod export_session {
    pub const CLASS: &[u8] = b"AVAssetExportSession\0";
    pub const SEL_EXPORT_SESSION_WITH_ASSET_PRESET: &[u8] = b"exportSessionWithAsset:presetName:\0";
    pub const SEL_ALL_EXPORT_PRESETS: &[u8] = b"allExportPresets\0";
    pub const SEL_EXPORT_PRESETS_COMPATIBLE_WITH_ASSET: &[u8] = b"exportPresetsCompatibleWithAsset:\0";
    pub const SEL_STATUS: &[u8] = b"status\0";
    pub const SEL_ERROR: &[u8] = b"error\0";
    pub const SEL_OUTPUT_URL: &[u8] = b"outputURL\0";
    pub const SEL_SET_OUTPUT_URL: &[u8] = b"setOutputURL:\0";
    pub const SEL_OUTPUT_FILE_TYPE: &[u8] = b"outputFileType\0";
    pub const SEL_SET_OUTPUT_FILE_TYPE: &[u8] = b"setOutputFileType:\0";
    pub const SEL_PROGRESS: &[u8] = b"progress\0";
    pub const SEL_EXPORT_ASYNCHRONOUSLY_WITH_COMPLETION_HANDLER: &[u8] = b"exportAsynchronouslyWithCompletionHandler:\0";
    pub const SEL_CANCEL_EXPORT: &[u8] = b"cancelExport\0";
    pub const SEL_ESTIMATED_OUTPUT_FILE_LENGTH: &[u8] = b"estimatedOutputFileLength\0";
}

/// Common media type constants (use as NSString* via nsstring()).
pub mod media_types {
    pub const VIDEO: &str = "vide";
    pub const AUDIO: &str = "soun";
    pub const TEXT: &str = "text";
    pub const CLOSED_CAPTION: &str = "clcp";
    pub const SUBTITLE: &str = "sbtl";
    pub const TIMECODE: &str = "tmcd";
    pub const METADATA: &str = "meta";
    pub const MUXED: &str = "muxx";
}

/// Export preset names.
pub mod presets {
    pub const LOW_QUALITY: &str = "AVAssetExportPresetLowQuality";
    pub const MEDIUM_QUALITY: &str = "AVAssetExportPresetMediumQuality";
    pub const HIGHEST_QUALITY: &str = "AVAssetExportPresetHighestQuality";
    pub const H264_640X480: &str = "AVAssetExportPreset640x480";
    pub const H264_960X540: &str = "AVAssetExportPreset960x540";
    pub const H264_1280X720: &str = "AVAssetExportPreset1280x720";
    pub const H264_1920X1080: &str = "AVAssetExportPreset1920x1080";
    pub const H264_3840X2160: &str = "AVAssetExportPreset3840x2160";
    pub const APPLE_M4A: &str = "AVAssetExportPresetAppleM4A";
    pub const PASSTHROUGH: &str = "AVAssetExportPresetPassthrough";
    pub const HEVC_1920X1080: &str = "AVAssetExportPresetHEVC1920x1080";
    pub const HEVC_3840X2160: &str = "AVAssetExportPresetHEVC3840x2160";
}
