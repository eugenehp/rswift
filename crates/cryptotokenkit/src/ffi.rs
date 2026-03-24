//! ObjC selector constants for CryptoTokenKit.
#![allow(dead_code)]

// ── TKSmartCard (5 methods, 4 properties) ──
pub mod t_k_smart_card {
    pub const CLASS: &[u8] = b"TKSmartCard\0";
    pub const SEL_SLOT: &[u8] = b"slot\0";
    pub const SEL_SET_SLOT: &[u8] = b"setSlot:\0";
    pub const SEL_VALID: &[u8] = b"valid\0";
    pub const SEL_SET_VALID: &[u8] = b"setValid:\0";
    pub const SEL_CURRENT_PROTOCOL: &[u8] = b"currentProtocol\0";
    pub const SEL_SET_CURRENT_PROTOCOL: &[u8] = b"setCurrentProtocol:\0";
    pub const SEL_CONTEXT: &[u8] = b"context\0";
    pub const SEL_SET_CONTEXT: &[u8] = b"setContext:\0";
    pub const SEL_END_SESSION: &[u8] = b"endSession\0";
    pub const SEL_USER_INTERACTION_FOR_SECURE_P_I_N_VERIFICATION_WITH_P_I_N_FORMAT: &[u8] = b"userInteractionForSecurePINVerificationWithPINFormat:APDU:PINByteOffset:\0";
    pub const SEL_USER_INTERACTION_FOR_SECURE_P_I_N_CHANGE_WITH_P_I_N_FORMAT: &[u8] = b"userInteractionForSecurePINChangeWithPINFormat:APDU:currentPINByteOffset:newPINByteOffset:\0";
}

// ── TKSmartCardATR (2 methods, 3 properties) ──
pub mod t_k_smart_card_a_t_r {
    pub const SEL_BYTES: &[u8] = b"bytes\0";
    pub const SEL_SET_BYTES: &[u8] = b"setBytes:\0";
    pub const SEL_PROTOCOLS: &[u8] = b"protocols\0";
    pub const SEL_SET_PROTOCOLS: &[u8] = b"setProtocols:\0";
    pub const SEL_HISTORICAL_BYTES: &[u8] = b"historicalBytes\0";
    pub const SEL_SET_HISTORICAL_BYTES: &[u8] = b"setHistoricalBytes:\0";
    pub const SEL_INTERFACE_GROUP_AT_INDEX: &[u8] = b"interfaceGroupAtIndex:\0";
    pub const SEL_INTERFACE_GROUP_FOR_PROTOCOL: &[u8] = b"interfaceGroupForProtocol:\0";
}

// ── TKSmartCardSlot (1 methods, 5 properties) ──
pub mod t_k_smart_card_slot {
    pub const SEL_STATE: &[u8] = b"state\0";
    pub const SEL_SET_STATE: &[u8] = b"setState:\0";
    pub const SEL_A_T_R: &[u8] = b"ATR\0";
    pub const SEL_SET_A_T_R: &[u8] = b"setATR:\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_MAX_INPUT_LENGTH: &[u8] = b"maxInputLength\0";
    pub const SEL_SET_MAX_INPUT_LENGTH: &[u8] = b"setMaxInputLength:\0";
    pub const SEL_MAX_OUTPUT_LENGTH: &[u8] = b"maxOutputLength\0";
    pub const SEL_SET_MAX_OUTPUT_LENGTH: &[u8] = b"setMaxOutputLength:\0";
    pub const SEL_MAKE_SMART_CARD: &[u8] = b"makeSmartCard\0";
}

// ── TKSmartCardSlotManager (4 methods, 2 properties) ──
pub mod t_k_smart_card_slot_manager {
    pub const SEL_DEFAULT_MANAGER: &[u8] = b"defaultManager\0";
    pub const SEL_SET_DEFAULT_MANAGER: &[u8] = b"setDefaultManager:\0";
    pub const SEL_SLOT_NAMES: &[u8] = b"slotNames\0";
    pub const SEL_SET_SLOT_NAMES: &[u8] = b"setSlotNames:\0";
    pub const SEL_SLOT_NAMED: &[u8] = b"slotNamed:\0";
    pub const SEL_IS_N_F_C_SUPPORTED: &[u8] = b"isNFCSupported\0";
}

// ── TKToken (0 methods, 3 properties) ──
pub mod t_k_token {
    pub const SEL_TOKEN_DRIVER: &[u8] = b"tokenDriver\0";
    pub const SEL_SET_TOKEN_DRIVER: &[u8] = b"setTokenDriver:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_KEYCHAIN_CONTENTS: &[u8] = b"keychainContents\0";
    pub const SEL_SET_KEYCHAIN_CONTENTS: &[u8] = b"setKeychainContents:\0";
}

// ── TKTokenSession (0 methods, 2 properties) ──
pub mod t_k_token_session {
    pub const SEL_TOKEN: &[u8] = b"token\0";
    pub const SEL_SET_TOKEN: &[u8] = b"setToken:\0";
}

// Total: 50 selector constants
