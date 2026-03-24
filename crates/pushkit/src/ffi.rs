//! ObjC selector constants for PushKit.
#![allow(dead_code)]

// ── PKPushRegistry (1 methods, 2 properties) ──
pub mod p_k_push_registry {
    pub const CLASS: &[u8] = b"PKPushRegistry\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_DESIRED_PUSH_TYPES: &[u8] = b"desiredPushTypes\0";
    pub const SEL_SET_DESIRED_PUSH_TYPES: &[u8] = b"setDesiredPushTypes:\0";
    pub const SEL_PUSH_TOKEN_FOR_TYPE: &[u8] = b"pushTokenForType:\0";
}

// ── PKPushCredentials (0 methods, 2 properties) ──
pub mod p_k_push_credentials {
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_TOKEN: &[u8] = b"token\0";
    pub const SEL_SET_TOKEN: &[u8] = b"setToken:\0";
}

// ── PKPushPayload (0 methods, 2 properties) ──
pub mod p_k_push_payload {
    pub const SEL_DICTIONARY_PAYLOAD: &[u8] = b"dictionaryPayload\0";
    pub const SEL_SET_DICTIONARY_PAYLOAD: &[u8] = b"setDictionaryPayload:\0";
}

// Total: 13 selector constants
