//! ObjC selector constants for Intents.
#![allow(dead_code)]

// ── INInteraction (4 methods, 7 properties) ──
pub mod i_n_interaction {
    pub const CLASS: &[u8] = b"INInteraction\0";
    pub const SEL_INTENT: &[u8] = b"intent\0";
    pub const SEL_SET_INTENT: &[u8] = b"setIntent:\0";
    pub const SEL_INTENT_RESPONSE: &[u8] = b"intentResponse\0";
    pub const SEL_SET_INTENT_RESPONSE: &[u8] = b"setIntentResponse:\0";
    pub const SEL_INTENT_HANDLING_STATUS: &[u8] = b"intentHandlingStatus\0";
    pub const SEL_SET_INTENT_HANDLING_STATUS: &[u8] = b"setIntentHandlingStatus:\0";
    pub const SEL_DIRECTION: &[u8] = b"direction\0";
    pub const SEL_SET_DIRECTION: &[u8] = b"setDirection:\0";
    pub const SEL_DATE_INTERVAL: &[u8] = b"dateInterval\0";
    pub const SEL_SET_DATE_INTERVAL: &[u8] = b"setDateInterval:\0";
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_SET_IDENTIFIER: &[u8] = b"setIdentifier:\0";
    pub const SEL_GROUP_IDENTIFIER: &[u8] = b"groupIdentifier\0";
    pub const SEL_SET_GROUP_IDENTIFIER: &[u8] = b"setGroupIdentifier:\0";
}

// ── INIntent (3 methods, 1 properties) ──
pub mod i_n_intent {
    pub const SEL_SET_IMAGE: &[u8] = b"setImage:forParameterNamed:\0";
    pub const SEL_IMAGE_FOR_PARAMETER_NAMED: &[u8] = b"imageForParameterNamed:\0";
    pub const SEL_KEY_IMAGE: &[u8] = b"keyImage\0";
}

// ── INIntentResponse (0 methods, 1 properties) ──
pub mod i_n_intent_response {
    pub const SEL_USER_ACTIVITY: &[u8] = b"userActivity\0";
    pub const SEL_SET_USER_ACTIVITY: &[u8] = b"setUserActivity:\0";
}

// ── INPerson (0 methods, 0 properties) ──
pub mod i_n_person {
}

// ── INImage (5 methods, 0 properties) ──
pub mod i_n_image {
    pub const SEL_IMAGE_NAMED: &[u8] = b"imageNamed:\0";
    pub const SEL_SYSTEM_IMAGE_NAMED: &[u8] = b"systemImageNamed:\0";
    pub const SEL_IMAGE_WITH_IMAGE_DATA: &[u8] = b"imageWithImageData:\0";
    pub const SEL_IMAGE_WITH_U_R_L: &[u8] = b"imageWithURL:\0";
}

// ── INParameter (4 methods, 2 properties) ──
pub mod i_n_parameter {
    pub const SEL_PARAMETER_CLASS: &[u8] = b"parameterClass\0";
    pub const SEL_SET_PARAMETER_CLASS: &[u8] = b"setParameterClass:\0";
    pub const SEL_PARAMETER_KEY_PATH: &[u8] = b"parameterKeyPath\0";
    pub const SEL_SET_PARAMETER_KEY_PATH: &[u8] = b"setParameterKeyPath:\0";
    pub const SEL_PARAMETER_FOR_CLASS: &[u8] = b"parameterForClass:keyPath:\0";
    pub const SEL_IS_EQUAL_TO_PARAMETER: &[u8] = b"isEqualToParameter:\0";
    pub const SEL_SET_INDEX: &[u8] = b"setIndex:forSubKeyPath:\0";
    pub const SEL_INDEX_FOR_SUB_KEY_PATH: &[u8] = b"indexForSubKeyPath:\0";
}

// Total: 37 selector constants
