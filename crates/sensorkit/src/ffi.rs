//! ObjC selector constants for SensorKit.
#![allow(dead_code)]

// ── SRSensorReader (5 methods, 3 properties) ──
pub mod s_r_sensor_reader {
    pub const CLASS: &[u8] = b"SRSensorReader\0";
    pub const SEL_AUTHORIZATION_STATUS: &[u8] = b"authorizationStatus\0";
    pub const SEL_SET_AUTHORIZATION_STATUS: &[u8] = b"setAuthorizationStatus:\0";
    pub const SEL_SENSOR: &[u8] = b"sensor\0";
    pub const SEL_SET_SENSOR: &[u8] = b"setSensor:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_START_RECORDING: &[u8] = b"startRecording\0";
    pub const SEL_STOP_RECORDING: &[u8] = b"stopRecording\0";
    pub const SEL_FETCH_DEVICES: &[u8] = b"fetchDevices\0";
    pub const SEL_FETCH: &[u8] = b"fetch:\0";
}

// ── SRFetchRequest (0 methods, 3 properties) ──
pub mod s_r_fetch_request {
    pub const SEL_FROM: &[u8] = b"from\0";
    pub const SEL_SET_FROM: &[u8] = b"setFrom:\0";
    pub const SEL_TO: &[u8] = b"to\0";
    pub const SEL_SET_TO: &[u8] = b"setTo:\0";
    pub const SEL_DEVICE: &[u8] = b"device\0";
    pub const SEL_SET_DEVICE: &[u8] = b"setDevice:\0";
}

// ── SRDevice (0 methods, 5 properties) ──
pub mod s_r_device {
    pub const SEL_CURRENT_DEVICE: &[u8] = b"currentDevice\0";
    pub const SEL_SET_CURRENT_DEVICE: &[u8] = b"setCurrentDevice:\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_MODEL: &[u8] = b"model\0";
    pub const SEL_SET_MODEL: &[u8] = b"setModel:\0";
    pub const SEL_SYSTEM_NAME: &[u8] = b"systemName\0";
    pub const SEL_SET_SYSTEM_NAME: &[u8] = b"setSystemName:\0";
    pub const SEL_SYSTEM_VERSION: &[u8] = b"systemVersion\0";
    pub const SEL_SET_SYSTEM_VERSION: &[u8] = b"setSystemVersion:\0";
}

// Total: 27 selector constants
