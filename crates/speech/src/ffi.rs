//! ObjC selector constants for Speech.
#![allow(dead_code)]

// ── SFSpeechRecognizer (5 methods, 5 properties) ──
pub mod s_f_speech_recognizer {
    pub const CLASS: &[u8] = b"SFSpeechRecognizer\0";
    pub const SEL_AVAILABLE: &[u8] = b"available\0";
    pub const SEL_SET_AVAILABLE: &[u8] = b"setAvailable:\0";
    pub const SEL_LOCALE: &[u8] = b"locale\0";
    pub const SEL_SET_LOCALE: &[u8] = b"setLocale:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_DEFAULT_TASK_HINT: &[u8] = b"defaultTaskHint\0";
    pub const SEL_SET_DEFAULT_TASK_HINT: &[u8] = b"setDefaultTaskHint:\0";
    pub const SEL_QUEUE: &[u8] = b"queue\0";
    pub const SEL_SET_QUEUE: &[u8] = b"setQueue:\0";
    pub const SEL_SUPPORTED_LOCALES: &[u8] = b"supportedLocales\0";
    pub const SEL_AUTHORIZATION_STATUS: &[u8] = b"authorizationStatus\0";
}

// ── SFSpeechAudioBufferRecognitionRequest (3 methods, 1 properties) ──
pub mod s_f_speech_audio_buffer_recognition_request {
    pub const SEL_NATIVE_AUDIO_FORMAT: &[u8] = b"nativeAudioFormat\0";
    pub const SEL_SET_NATIVE_AUDIO_FORMAT: &[u8] = b"setNativeAudioFormat:\0";
    pub const SEL_APPEND_AUDIO_P_C_M_BUFFER: &[u8] = b"appendAudioPCMBuffer:\0";
    pub const SEL_APPEND_AUDIO_SAMPLE_BUFFER: &[u8] = b"appendAudioSampleBuffer:\0";
    pub const SEL_END_AUDIO: &[u8] = b"endAudio\0";
}

// ── SFSpeechRecognitionResult (0 methods, 3 properties) ──
pub mod s_f_speech_recognition_result {
    pub const SEL_BEST_TRANSCRIPTION: &[u8] = b"bestTranscription\0";
    pub const SEL_SET_BEST_TRANSCRIPTION: &[u8] = b"setBestTranscription:\0";
    pub const SEL_TRANSCRIPTIONS: &[u8] = b"transcriptions\0";
    pub const SEL_SET_TRANSCRIPTIONS: &[u8] = b"setTranscriptions:\0";
    pub const SEL_FINAL: &[u8] = b"final\0";
    pub const SEL_SET_FINAL: &[u8] = b"setFinal:\0";
}

// ── SFSpeechRecognitionTask (2 methods, 4 properties) ──
pub mod s_f_speech_recognition_task {
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_SET_STATE: &[u8] = b"setState:\0";
    pub const SEL_FINISHING: &[u8] = b"finishing\0";
    pub const SEL_SET_FINISHING: &[u8] = b"setFinishing:\0";
    pub const SEL_CANCELLED: &[u8] = b"cancelled\0";
    pub const SEL_SET_CANCELLED: &[u8] = b"setCancelled:\0";
    pub const SEL_ERROR: &[u8] = b"error\0";
    pub const SEL_SET_ERROR: &[u8] = b"setError:\0";
    pub const SEL_FINISH: &[u8] = b"finish\0";
    pub const SEL_CANCEL: &[u8] = b"cancel\0";
}

// ── SFTranscription (0 methods, 2 properties) ──
pub mod s_f_transcription {
    pub const SEL_FORMATTED_STRING: &[u8] = b"formattedString\0";
    pub const SEL_SET_FORMATTED_STRING: &[u8] = b"setFormattedString:\0";
    pub const SEL_SEGMENTS: &[u8] = b"segments\0";
    pub const SEL_SET_SEGMENTS: &[u8] = b"setSegments:\0";
}

// ── SFTranscriptionSegment (0 methods, 6 properties) ──
pub mod s_f_transcription_segment {
    pub const SEL_SUBSTRING: &[u8] = b"substring\0";
    pub const SEL_SET_SUBSTRING: &[u8] = b"setSubstring:\0";
    pub const SEL_SUBSTRING_RANGE: &[u8] = b"substringRange\0";
    pub const SEL_SET_SUBSTRING_RANGE: &[u8] = b"setSubstringRange:\0";
    pub const SEL_TIMESTAMP: &[u8] = b"timestamp\0";
    pub const SEL_SET_TIMESTAMP: &[u8] = b"setTimestamp:\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_CONFIDENCE: &[u8] = b"confidence\0";
    pub const SEL_SET_CONFIDENCE: &[u8] = b"setConfidence:\0";
    pub const SEL_ALTERNATIVE_SUBSTRINGS: &[u8] = b"alternativeSubstrings\0";
    pub const SEL_SET_ALTERNATIVE_SUBSTRINGS: &[u8] = b"setAlternativeSubstrings:\0";
}

// Total: 51 selector constants
