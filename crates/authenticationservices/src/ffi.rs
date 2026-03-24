//! ObjC selector constants for AuthenticationServices.
#![allow(dead_code)]

// ── ASAuthorizationAppleIDProvider (2 methods, 0 properties) ──
pub mod a_s_authorization_apple_i_d_provider {
    pub const CLASS: &[u8] = b"ASAuthorizationAppleIDProvider\0";
    pub const SEL_CREATE_REQUEST: &[u8] = b"createRequest\0";
}

// ── ASAuthorizationPasswordProvider (1 methods, 0 properties) ──
pub mod a_s_authorization_password_provider {
}

// ── ASAuthorizationController (4 methods, 2 properties) ──
pub mod a_s_authorization_controller {
    pub const SEL_AUTHORIZATION_REQUESTS: &[u8] = b"authorizationRequests\0";
    pub const SEL_SET_AUTHORIZATION_REQUESTS: &[u8] = b"setAuthorizationRequests:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_PERFORM_REQUESTS: &[u8] = b"performRequests\0";
    pub const SEL_PERFORM_AUTO_FILL_ASSISTED_REQUESTS: &[u8] = b"performAutoFillAssistedRequests\0";
    pub const SEL_PERFORM_REQUESTS_WITH_OPTIONS: &[u8] = b"performRequestsWithOptions:\0";
    pub const SEL_CANCEL: &[u8] = b"cancel\0";
}

// ── ASWebAuthenticationSession (2 methods, 0 properties) ──
pub mod a_s_web_authentication_session {
    pub const SEL_START: &[u8] = b"start\0";
}

// Total: 13 selector constants
