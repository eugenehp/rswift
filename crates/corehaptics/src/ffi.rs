//! ObjC selector constants for CoreHaptics.
#![allow(dead_code)]

// ── CHHapticEngine (11 methods, 8 properties) ──
pub mod c_h_haptic_engine {
    pub const CLASS: &[u8] = b"CHHapticEngine\0";
    pub const SEL_CURRENT_TIME: &[u8] = b"currentTime\0";
    pub const SEL_SET_CURRENT_TIME: &[u8] = b"setCurrentTime:\0";
    pub const SEL_STOPPED_HANDLER: &[u8] = b"stoppedHandler\0";
    pub const SEL_SET_STOPPED_HANDLER: &[u8] = b"setStoppedHandler:\0";
    pub const SEL_RESET_HANDLER: &[u8] = b"resetHandler\0";
    pub const SEL_SET_RESET_HANDLER: &[u8] = b"setResetHandler:\0";
    pub const SEL_PLAYS_HAPTICS_ONLY: &[u8] = b"playsHapticsOnly\0";
    pub const SEL_SET_PLAYS_HAPTICS_ONLY: &[u8] = b"setPlaysHapticsOnly:\0";
    pub const SEL_IS_MUTED_FOR_AUDIO: &[u8] = b"isMutedForAudio\0";
    pub const SEL_SET_IS_MUTED_FOR_AUDIO: &[u8] = b"setIsMutedForAudio:\0";
    pub const SEL_IS_MUTED_FOR_HAPTICS: &[u8] = b"isMutedForHaptics\0";
    pub const SEL_SET_IS_MUTED_FOR_HAPTICS: &[u8] = b"setIsMutedForHaptics:\0";
    pub const SEL_AUTO_SHUTDOWN_ENABLED: &[u8] = b"autoShutdownEnabled\0";
    pub const SEL_SET_AUTO_SHUTDOWN_ENABLED: &[u8] = b"setAutoShutdownEnabled:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
    pub const SEL_CAPABILITIES_FOR_HARDWARE: &[u8] = b"capabilitiesForHardware\0";
    pub const SEL_START_WITH_COMPLETION_HANDLER: &[u8] = b"startWithCompletionHandler:\0";
    pub const SEL_START_AND_RETURN_ERROR: &[u8] = b"startAndReturnError:\0";
    pub const SEL_STOP_WITH_COMPLETION_HANDLER: &[u8] = b"stopWithCompletionHandler:\0";
    pub const SEL_NOTIFY_WHEN_PLAYERS_FINISHED: &[u8] = b"notifyWhenPlayersFinished:\0";
    pub const SEL_CREATE_PLAYER_WITH_PATTERN: &[u8] = b"createPlayerWithPattern:error:\0";
    pub const SEL_CREATE_ADVANCED_PLAYER_WITH_PATTERN: &[u8] = b"createAdvancedPlayerWithPattern:error:\0";
    pub const SEL_REGISTER_AUDIO_RESOURCE: &[u8] = b"registerAudioResource:options:error:\0";
    pub const SEL_UNREGISTER_AUDIO_RESOURCE: &[u8] = b"unregisterAudioResource:error:\0";
    pub const SEL_PLAY_PATTERN_FROM_U_R_L: &[u8] = b"playPatternFromURL:error:\0";
    pub const SEL_PLAY_PATTERN_FROM_DATA: &[u8] = b"playPatternFromData:error:\0";
}

// ── CHHapticPattern (1 methods, 1 properties) ──
pub mod c_h_haptic_pattern {
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_EXPORT_DICTIONARY_AND_RETURN_ERROR: &[u8] = b"exportDictionaryAndReturnError:\0";
}

// ── CHHapticEvent (0 methods, 4 properties) ──
pub mod c_h_haptic_event {
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_EVENT_PARAMETERS: &[u8] = b"eventParameters\0";
    pub const SEL_SET_EVENT_PARAMETERS: &[u8] = b"setEventParameters:\0";
    pub const SEL_RELATIVE_TIME: &[u8] = b"relativeTime\0";
    pub const SEL_SET_RELATIVE_TIME: &[u8] = b"setRelativeTime:\0";
}

// ── CHHapticEventParameter (0 methods, 2 properties) ──
pub mod c_h_haptic_event_parameter {
    pub const SEL_PARAMETER_I_D: &[u8] = b"parameterID\0";
    pub const SEL_SET_PARAMETER_I_D: &[u8] = b"setParameterID:\0";
    pub const SEL_VALUE: &[u8] = b"value\0";
    pub const SEL_SET_VALUE: &[u8] = b"setValue:\0";
}

// Total: 42 selector constants
