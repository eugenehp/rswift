//! ObjC selector constants for GameController.
#![allow(dead_code)]

// ── GCController (2 methods, 0 properties) ──
pub mod g_c_controller {
    pub const CLASS: &[u8] = b"GCController\0";
    pub const SEL_CONTROLLERS: &[u8] = b"controllers\0";
    pub const SEL_SUPPORTS_H_I_D_DEVICE: &[u8] = b"supportsHIDDevice:\0";
}

// ── GCExtendedGamepad (2 methods, 13 properties) ──
pub mod g_c_extended_gamepad {
    pub const SEL_CONTROLLER: &[u8] = b"controller\0";
    pub const SEL_SET_CONTROLLER: &[u8] = b"setController:\0";
    pub const SEL_VALUE_CHANGED_HANDLER: &[u8] = b"valueChangedHandler\0";
    pub const SEL_SET_VALUE_CHANGED_HANDLER: &[u8] = b"setValueChangedHandler:\0";
    pub const SEL_DPAD: &[u8] = b"dpad\0";
    pub const SEL_SET_DPAD: &[u8] = b"setDpad:\0";
    pub const SEL_BUTTON_A: &[u8] = b"buttonA\0";
    pub const SEL_SET_BUTTON_A: &[u8] = b"setButtonA:\0";
    pub const SEL_BUTTON_B: &[u8] = b"buttonB\0";
    pub const SEL_SET_BUTTON_B: &[u8] = b"setButtonB:\0";
    pub const SEL_BUTTON_X: &[u8] = b"buttonX\0";
    pub const SEL_SET_BUTTON_X: &[u8] = b"setButtonX:\0";
    pub const SEL_BUTTON_Y: &[u8] = b"buttonY\0";
    pub const SEL_SET_BUTTON_Y: &[u8] = b"setButtonY:\0";
    pub const SEL_LEFT_THUMBSTICK: &[u8] = b"leftThumbstick\0";
    pub const SEL_SET_LEFT_THUMBSTICK: &[u8] = b"setLeftThumbstick:\0";
    pub const SEL_RIGHT_THUMBSTICK: &[u8] = b"rightThumbstick\0";
    pub const SEL_SET_RIGHT_THUMBSTICK: &[u8] = b"setRightThumbstick:\0";
    pub const SEL_LEFT_SHOULDER: &[u8] = b"leftShoulder\0";
    pub const SEL_SET_LEFT_SHOULDER: &[u8] = b"setLeftShoulder:\0";
    pub const SEL_RIGHT_SHOULDER: &[u8] = b"rightShoulder\0";
    pub const SEL_SET_RIGHT_SHOULDER: &[u8] = b"setRightShoulder:\0";
    pub const SEL_LEFT_TRIGGER: &[u8] = b"leftTrigger\0";
    pub const SEL_SET_LEFT_TRIGGER: &[u8] = b"setLeftTrigger:\0";
    pub const SEL_RIGHT_TRIGGER: &[u8] = b"rightTrigger\0";
    pub const SEL_SET_RIGHT_TRIGGER: &[u8] = b"setRightTrigger:\0";
    pub const SEL_SAVE_SNAPSHOT: &[u8] = b"saveSnapshot\0";
    pub const SEL_SET_STATE_FROM_EXTENDED_GAMEPAD: &[u8] = b"setStateFromExtendedGamepad:\0";
}

// ── GCMicroGamepad (2 methods, 7 properties) ──
pub mod g_c_micro_gamepad {
    pub const SEL_REPORTS_ABSOLUTE_DPAD_VALUES: &[u8] = b"reportsAbsoluteDpadValues\0";
    pub const SEL_SET_REPORTS_ABSOLUTE_DPAD_VALUES: &[u8] = b"setReportsAbsoluteDpadValues:\0";
    pub const SEL_ALLOWS_ROTATION: &[u8] = b"allowsRotation\0";
    pub const SEL_SET_ALLOWS_ROTATION: &[u8] = b"setAllowsRotation:\0";
    pub const SEL_SET_STATE_FROM_MICRO_GAMEPAD: &[u8] = b"setStateFromMicroGamepad:\0";
}

// ── GCMotion (6 methods, 6 properties) ──
pub mod g_c_motion {
    pub const SEL_GRAVITY: &[u8] = b"gravity\0";
    pub const SEL_SET_GRAVITY: &[u8] = b"setGravity:\0";
    pub const SEL_USER_ACCELERATION: &[u8] = b"userAcceleration\0";
    pub const SEL_SET_USER_ACCELERATION: &[u8] = b"setUserAcceleration:\0";
    pub const SEL_ATTITUDE: &[u8] = b"attitude\0";
    pub const SEL_SET_ATTITUDE: &[u8] = b"setAttitude:\0";
    pub const SEL_ROTATION_RATE: &[u8] = b"rotationRate\0";
    pub const SEL_SET_ROTATION_RATE: &[u8] = b"setRotationRate:\0";
    pub const SEL_SET_ACCELERATION: &[u8] = b"setAcceleration:\0";
    pub const SEL_SET_STATE_FROM_MOTION: &[u8] = b"setStateFromMotion:\0";
}

// Total: 64 selector constants
