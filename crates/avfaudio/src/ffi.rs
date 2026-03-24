//! ObjC selector constants for AVFAudio.
#![allow(dead_code)]

// ── AVAudioEngine (27 methods, 4 properties) ──
pub mod a_v_audio_engine {
    pub const CLASS: &[u8] = b"AVAudioEngine\0";
    pub const SEL_MUSIC_SEQUENCE: &[u8] = b"musicSequence\0";
    pub const SEL_SET_MUSIC_SEQUENCE: &[u8] = b"setMusicSequence:\0";
    pub const SEL_OUTPUT_NODE: &[u8] = b"outputNode\0";
    pub const SEL_SET_OUTPUT_NODE: &[u8] = b"setOutputNode:\0";
    pub const SEL_MAIN_MIXER_NODE: &[u8] = b"mainMixerNode\0";
    pub const SEL_SET_MAIN_MIXER_NODE: &[u8] = b"setMainMixerNode:\0";
    pub const SEL_RUNNING: &[u8] = b"running\0";
    pub const SEL_SET_RUNNING: &[u8] = b"setRunning:\0";
    pub const SEL_ATTACH_NODE: &[u8] = b"attachNode:\0";
    pub const SEL_DETACH_NODE: &[u8] = b"detachNode:\0";
    pub const SEL_CONNECT: &[u8] = b"connect:to:fromBus:toBus:format:\0";
    pub const SEL_DISCONNECT_NODE_INPUT: &[u8] = b"disconnectNodeInput:bus:\0";
    pub const SEL_DISCONNECT_NODE_OUTPUT: &[u8] = b"disconnectNodeOutput:bus:\0";
    pub const SEL_PREPARE: &[u8] = b"prepare\0";
    pub const SEL_START_AND_RETURN_ERROR: &[u8] = b"startAndReturnError:\0";
    pub const SEL_PAUSE: &[u8] = b"pause\0";
    pub const SEL_RESET: &[u8] = b"reset\0";
    pub const SEL_STOP: &[u8] = b"stop\0";
    pub const SEL_INPUT_CONNECTION_POINT_FOR_NODE: &[u8] = b"inputConnectionPointForNode:inputBus:\0";
    pub const SEL_OUTPUT_CONNECTION_POINTS_FOR_NODE: &[u8] = b"outputConnectionPointsForNode:outputBus:\0";
    pub const SEL_ENABLE_MANUAL_RENDERING_MODE: &[u8] = b"enableManualRenderingMode:format:maximumFrameCount:error:\0";
    pub const SEL_DISABLE_MANUAL_RENDERING_MODE: &[u8] = b"disableManualRenderingMode\0";
    pub const SEL_RENDER_OFFLINE: &[u8] = b"renderOffline:toBuffer:error:\0";
    pub const SEL_CONNECT_M_I_D_I: &[u8] = b"connectMIDI:to:format:block:\0";
    pub const SEL_DISCONNECT_M_I_D_I: &[u8] = b"disconnectMIDI:from:\0";
    pub const SEL_DISCONNECT_M_I_D_I_INPUT: &[u8] = b"disconnectMIDIInput:\0";
    pub const SEL_DISCONNECT_M_I_D_I_OUTPUT: &[u8] = b"disconnectMIDIOutput:\0";
}

// ── AVAudioPlayer (9 methods, 8 properties) ──
pub mod a_v_audio_player {
    pub const SEL_PLAYING: &[u8] = b"playing\0";
    pub const SEL_SET_PLAYING: &[u8] = b"setPlaying:\0";
    pub const SEL_NUMBER_OF_CHANNELS: &[u8] = b"numberOfChannels\0";
    pub const SEL_SET_NUMBER_OF_CHANNELS: &[u8] = b"setNumberOfChannels:\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_URL: &[u8] = b"url\0";
    pub const SEL_SET_URL: &[u8] = b"setUrl:\0";
    pub const SEL_DATA: &[u8] = b"data\0";
    pub const SEL_SET_DATA: &[u8] = b"setData:\0";
    pub const SEL_METERING_ENABLED: &[u8] = b"meteringEnabled\0";
    pub const SEL_SET_METERING_ENABLED: &[u8] = b"setMeteringEnabled:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
    pub const SEL_PREPARE_TO_PLAY: &[u8] = b"prepareToPlay\0";
    pub const SEL_PLAY: &[u8] = b"play\0";
    pub const SEL_PLAY_AT_TIME: &[u8] = b"playAtTime:\0";
    pub const SEL_SET_VOLUME: &[u8] = b"setVolume:fadeDuration:\0";
    pub const SEL_UPDATE_METERS: &[u8] = b"updateMeters\0";
    pub const SEL_PEAK_POWER_FOR_CHANNEL: &[u8] = b"peakPowerForChannel:\0";
    pub const SEL_AVERAGE_POWER_FOR_CHANNEL: &[u8] = b"averagePowerForChannel:\0";
}

// ── AVAudioRecorder (11 methods, 6 properties) ──
pub mod a_v_audio_recorder {
    pub const SEL_RECORDING: &[u8] = b"recording\0";
    pub const SEL_SET_RECORDING: &[u8] = b"setRecording:\0";
    pub const SEL_SETTINGS: &[u8] = b"settings\0";
    pub const SEL_SET_SETTINGS: &[u8] = b"setSettings:\0";
    pub const SEL_CURRENT_TIME: &[u8] = b"currentTime\0";
    pub const SEL_SET_CURRENT_TIME: &[u8] = b"setCurrentTime:\0";
    pub const SEL_PREPARE_TO_RECORD: &[u8] = b"prepareToRecord\0";
    pub const SEL_RECORD: &[u8] = b"record\0";
    pub const SEL_RECORD_AT_TIME: &[u8] = b"recordAtTime:\0";
    pub const SEL_RECORD_FOR_DURATION: &[u8] = b"recordForDuration:\0";
    pub const SEL_DELETE_RECORDING: &[u8] = b"deleteRecording\0";
}

// ── AVAudioSession (2 methods, 1 properties) ──
pub mod a_v_audio_session {
    pub const SEL_T_V_O_S_P_R_O_H_I_B_I_T_E_D: &[u8] = b"__TVOS_PROHIBITED\0";
    pub const SEL_SET_T_V_O_S_P_R_O_H_I_B_I_T_E_D: &[u8] = b"set__TVOS_PROHIBITED:\0";
    pub const SEL_SET_ACTIVE: &[u8] = b"setActive:withFlags:error:\0";
    pub const SEL_SET_PREFERRED_HARDWARE_SAMPLE_RATE: &[u8] = b"setPreferredHardwareSampleRate:error:\0";
}

// ── AVAudioPlayerNode (15 methods, 1 properties) ──
pub mod a_v_audio_player_node {
    pub const SEL_SCHEDULE_BUFFER: &[u8] = b"scheduleBuffer:completionHandler:\0";
    pub const SEL_SCHEDULE_FILE: &[u8] = b"scheduleFile:atTime:completionHandler:\0";
    pub const SEL_SCHEDULE_SEGMENT: &[u8] = b"scheduleSegment:startingFrame:frameCount:atTime:completionHandler:\0";
    pub const SEL_PREPARE_WITH_FRAME_COUNT: &[u8] = b"prepareWithFrameCount:\0";
    pub const SEL_NODE_TIME_FOR_PLAYER_TIME: &[u8] = b"nodeTimeForPlayerTime:\0";
    pub const SEL_PLAYER_TIME_FOR_NODE_TIME: &[u8] = b"playerTimeForNodeTime:\0";
}

// ── AVAudioFormat (1 methods, 8 properties) ──
pub mod a_v_audio_format {
    pub const SEL_STANDARD: &[u8] = b"standard\0";
    pub const SEL_SET_STANDARD: &[u8] = b"setStandard:\0";
    pub const SEL_COMMON_FORMAT: &[u8] = b"commonFormat\0";
    pub const SEL_SET_COMMON_FORMAT: &[u8] = b"setCommonFormat:\0";
    pub const SEL_CHANNEL_COUNT: &[u8] = b"channelCount\0";
    pub const SEL_SET_CHANNEL_COUNT: &[u8] = b"setChannelCount:\0";
    pub const SEL_SAMPLE_RATE: &[u8] = b"sampleRate\0";
    pub const SEL_SET_SAMPLE_RATE: &[u8] = b"setSampleRate:\0";
    pub const SEL_INTERLEAVED: &[u8] = b"interleaved\0";
    pub const SEL_SET_INTERLEAVED: &[u8] = b"setInterleaved:\0";
    pub const SEL_STREAM_DESCRIPTION: &[u8] = b"streamDescription\0";
    pub const SEL_SET_STREAM_DESCRIPTION: &[u8] = b"setStreamDescription:\0";
    pub const SEL_CHANNEL_LAYOUT: &[u8] = b"channelLayout\0";
    pub const SEL_SET_CHANNEL_LAYOUT: &[u8] = b"setChannelLayout:\0";
    pub const SEL_IS_EQUAL: &[u8] = b"isEqual:\0";
}

// ── AVAudioFile (4 methods, 5 properties) ──
pub mod a_v_audio_file {
    pub const SEL_FILE_FORMAT: &[u8] = b"fileFormat\0";
    pub const SEL_SET_FILE_FORMAT: &[u8] = b"setFileFormat:\0";
    pub const SEL_PROCESSING_FORMAT: &[u8] = b"processingFormat\0";
    pub const SEL_SET_PROCESSING_FORMAT: &[u8] = b"setProcessingFormat:\0";
    pub const SEL_LENGTH: &[u8] = b"length\0";
    pub const SEL_SET_LENGTH: &[u8] = b"setLength:\0";
    pub const SEL_FRAME_POSITION: &[u8] = b"framePosition\0";
    pub const SEL_SET_FRAME_POSITION: &[u8] = b"setFramePosition:\0";
    pub const SEL_CLOSE: &[u8] = b"close\0";
    pub const SEL_READ_INTO_BUFFER: &[u8] = b"readIntoBuffer:error:\0";
    pub const SEL_WRITE_FROM_BUFFER: &[u8] = b"writeFromBuffer:error:\0";
}

// Total: 120 selector constants
