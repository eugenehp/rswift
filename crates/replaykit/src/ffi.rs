//! ObjC selector constants for ReplayKit.
#![allow(dead_code)]

// ── RPScreenRecorder (11 methods, 3 properties) ──
pub mod r_p_screen_recorder {
    pub const CLASS: &[u8] = b"RPScreenRecorder\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_AVAILABLE: &[u8] = b"available\0";
    pub const SEL_SET_AVAILABLE: &[u8] = b"setAvailable:\0";
    pub const SEL_RECORDING: &[u8] = b"recording\0";
    pub const SEL_SET_RECORDING: &[u8] = b"setRecording:\0";
    pub const SEL_SHARED_RECORDER: &[u8] = b"sharedRecorder\0";
}

// ── RPBroadcastController (4 methods, 5 properties) ──
pub mod r_p_broadcast_controller {
    pub const SEL_BROADCASTING: &[u8] = b"broadcasting\0";
    pub const SEL_SET_BROADCASTING: &[u8] = b"setBroadcasting:\0";
    pub const SEL_PAUSED: &[u8] = b"paused\0";
    pub const SEL_SET_PAUSED: &[u8] = b"setPaused:\0";
    pub const SEL_BROADCAST_U_R_L: &[u8] = b"broadcastURL\0";
    pub const SEL_SET_BROADCAST_U_R_L: &[u8] = b"setBroadcastURL:\0";
    pub const SEL_SERVICE_INFO: &[u8] = b"serviceInfo\0";
    pub const SEL_SET_SERVICE_INFO: &[u8] = b"setServiceInfo:\0";
    pub const SEL_PAUSE_BROADCAST: &[u8] = b"pauseBroadcast\0";
    pub const SEL_RESUME_BROADCAST: &[u8] = b"resumeBroadcast\0";
}

// ── RPBroadcastActivityViewController (2 methods, 1 properties) ──
pub mod r_p_broadcast_activity_view_controller {
}

// Total: 35 selector constants
