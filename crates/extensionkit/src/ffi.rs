//! ObjC selector constants for ExtensionKit.
#![allow(dead_code)]

// ── EXAppExtensionBrowserViewController (0 methods, 0 properties) ──
pub mod e_x_app_extension_browser_view_controller {
    pub const CLASS: &[u8] = b"EXAppExtensionBrowserViewController\0";
}

// ── EXHostViewController (1 methods, 2 properties) ──
pub mod e_x_host_view_controller {
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_PLACEHOLDER_VIEW: &[u8] = b"placeholderView\0";
    pub const SEL_SET_PLACEHOLDER_VIEW: &[u8] = b"setPlaceholderView:\0";
    pub const SEL_MAKE_X_P_C_CONNECTION_WITH_ERROR: &[u8] = b"makeXPCConnectionWithError:\0";
}

// Total: 5 selector constants
