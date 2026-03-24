//! ObjC selector constants for CoreMotion.
#![allow(dead_code)]

// ── CMMotionManager (15 methods, 0 properties) ──
pub mod c_m_motion_manager {
    pub const CLASS: &[u8] = b"CMMotionManager\0";
    pub const SEL_START_ACCELEROMETER_UPDATES: &[u8] = b"startAccelerometerUpdates\0";
    pub const SEL_START_ACCELEROMETER_UPDATES_TO_QUEUE: &[u8] = b"startAccelerometerUpdatesToQueue:withHandler:\0";
    pub const SEL_STOP_ACCELEROMETER_UPDATES: &[u8] = b"stopAccelerometerUpdates\0";
    pub const SEL_START_GYRO_UPDATES: &[u8] = b"startGyroUpdates\0";
    pub const SEL_START_GYRO_UPDATES_TO_QUEUE: &[u8] = b"startGyroUpdatesToQueue:withHandler:\0";
    pub const SEL_STOP_GYRO_UPDATES: &[u8] = b"stopGyroUpdates\0";
    pub const SEL_START_MAGNETOMETER_UPDATES_TO_QUEUE: &[u8] = b"startMagnetometerUpdatesToQueue:withHandler:COREMOTION_EXPORT\0";
    pub const SEL_START_DEVICE_MOTION_UPDATES: &[u8] = b"startDeviceMotionUpdates\0";
    pub const SEL_START_DEVICE_MOTION_UPDATES_TO_QUEUE: &[u8] = b"startDeviceMotionUpdatesToQueue:withHandler:\0";
    pub const SEL_START_DEVICE_MOTION_UPDATES_USING_REFERENCE_FRAME: &[u8] = b"startDeviceMotionUpdatesUsingReferenceFrame:COREMOTION_EXPORT\0";
    pub const SEL_STOP_DEVICE_MOTION_UPDATES: &[u8] = b"stopDeviceMotionUpdates\0";
}

// ── CMPedometer (12 methods, 0 properties) ──
pub mod c_m_pedometer {
    pub const SEL_IS_STEP_COUNTING_AVAILABLE: &[u8] = b"isStepCountingAvailable\0";
    pub const SEL_IS_DISTANCE_AVAILABLE: &[u8] = b"isDistanceAvailable\0";
    pub const SEL_IS_FLOOR_COUNTING_AVAILABLE: &[u8] = b"isFloorCountingAvailable\0";
    pub const SEL_QUERY_PEDOMETER_DATA_FROM_DATE: &[u8] = b"queryPedometerDataFromDate:toDate:withHandler:\0";
    pub const SEL_START_PEDOMETER_UPDATES_FROM_DATE: &[u8] = b"startPedometerUpdatesFromDate:withHandler:\0";
    pub const SEL_STOP_PEDOMETER_UPDATES: &[u8] = b"stopPedometerUpdates\0";
    pub const SEL_START_PEDOMETER_EVENT_UPDATES_WITH_HANDLER: &[u8] = b"startPedometerEventUpdatesWithHandler:COREMOTION_EXPORT\0";
}

// ── CMAltimeter (7 methods, 0 properties) ──
pub mod c_m_altimeter {
    pub const SEL_IS_RELATIVE_ALTITUDE_AVAILABLE: &[u8] = b"isRelativeAltitudeAvailable\0";
    pub const SEL_AUTHORIZATION_STATUS: &[u8] = b"authorizationStatus\0";
    pub const SEL_START_RELATIVE_ALTITUDE_UPDATES_TO_QUEUE: &[u8] = b"startRelativeAltitudeUpdatesToQueue:withHandler:\0";
    pub const SEL_STOP_RELATIVE_ALTITUDE_UPDATES: &[u8] = b"stopRelativeAltitudeUpdates\0";
    pub const SEL_IS_ABSOLUTE_ALTITUDE_AVAILABLE: &[u8] = b"isAbsoluteAltitudeAvailable\0";
    pub const SEL_START_ABSOLUTE_ALTITUDE_UPDATES_TO_QUEUE: &[u8] = b"startAbsoluteAltitudeUpdatesToQueue:withHandler:\0";
    pub const SEL_STOP_ABSOLUTE_ALTITUDE_UPDATES: &[u8] = b"stopAbsoluteAltitudeUpdates\0";
}

// ── CMMotionActivityManager (5 methods, 0 properties) ──
pub mod c_m_motion_activity_manager {
    pub const SEL_IS_ACTIVITY_AVAILABLE: &[u8] = b"isActivityAvailable\0";
    pub const SEL_QUERY_ACTIVITY_STARTING_FROM_DATE: &[u8] = b"queryActivityStartingFromDate:toDate:toQueue:withHandler:\0";
    pub const SEL_START_ACTIVITY_UPDATES_TO_QUEUE: &[u8] = b"startActivityUpdatesToQueue:withHandler:\0";
    pub const SEL_STOP_ACTIVITY_UPDATES: &[u8] = b"stopActivityUpdates\0";
}

// ── CMDeviceMotion (0 methods, 4 properties) ──
pub mod c_m_device_motion {
    pub const SEL_ATTITUDE: &[u8] = b"attitude\0";
    pub const SEL_SET_ATTITUDE: &[u8] = b"setAttitude:\0";
    pub const SEL_ROTATION_RATE: &[u8] = b"rotationRate\0";
    pub const SEL_SET_ROTATION_RATE: &[u8] = b"setRotationRate:\0";
    pub const SEL_GRAVITY: &[u8] = b"gravity\0";
    pub const SEL_SET_GRAVITY: &[u8] = b"setGravity:\0";
    pub const SEL_USER_ACCELERATION: &[u8] = b"userAcceleration\0";
    pub const SEL_SET_USER_ACCELERATION: &[u8] = b"setUserAcceleration:\0";
}

// ── CMAccelerometerData (0 methods, 1 properties) ──
pub mod c_m_accelerometer_data {
    pub const SEL_ACCELERATION: &[u8] = b"acceleration\0";
    pub const SEL_SET_ACCELERATION: &[u8] = b"setAcceleration:\0";
}

// Total: 48 selector constants
