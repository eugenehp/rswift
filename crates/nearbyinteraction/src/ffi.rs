//! ObjC selector constants for NearbyInteraction.
#![allow(dead_code)]

// ── NISession (5 methods, 4 properties) ──
pub mod n_i_session {
    pub const CLASS: &[u8] = b"NISession\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_DELEGATE_QUEUE: &[u8] = b"delegateQueue\0";
    pub const SEL_SET_DELEGATE_QUEUE: &[u8] = b"setDelegateQueue:\0";
    pub const SEL_DISCOVERY_TOKEN: &[u8] = b"discoveryToken\0";
    pub const SEL_SET_DISCOVERY_TOKEN: &[u8] = b"setDiscoveryToken:\0";
    pub const SEL_CONFIGURATION: &[u8] = b"configuration\0";
    pub const SEL_SET_CONFIGURATION: &[u8] = b"setConfiguration:\0";
    pub const SEL_RUN_WITH_CONFIGURATION: &[u8] = b"runWithConfiguration:\0";
    pub const SEL_PAUSE: &[u8] = b"pause\0";
    pub const SEL_INVALIDATE: &[u8] = b"invalidate\0";
    pub const SEL_SET_A_R_SESSION: &[u8] = b"setARSession:\0";
    pub const SEL_WORLD_TRANSFORM_FOR_OBJECT: &[u8] = b"worldTransformForObject:\0";
}

// ── NINearbyObject (0 methods, 2 properties) ──
pub mod n_i_nearby_object {
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
}

// ── NIDiscoveryToken (0 methods, 0 properties) ──
pub mod n_i_discovery_token {
}

// Total: 17 selector constants
