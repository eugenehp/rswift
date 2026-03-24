//! ObjC selector constants for AVKit.
#![allow(dead_code)]

// ── AVPlayerView (2 methods, 1 properties) ──
pub mod a_v_player_view {
    pub const CLASS: &[u8] = b"AVPlayerView\0";
    pub const SEL_PLAYER: &[u8] = b"player\0";
    pub const SEL_SET_PLAYER: &[u8] = b"setPlayer:\0";
    pub const SEL_SELECT_SPEED: &[u8] = b"selectSpeed:\0";
    pub const SEL_SET_MAGNIFICATION: &[u8] = b"setMagnification:centeredAtPoint:\0";
}

// ── AVPlayerViewController (0 methods, 0 properties) ──
pub mod a_v_player_view_controller {
}

// ── AVPictureInPictureController (1 methods, 0 properties) ──
pub mod a_v_picture_in_picture_controller {
    pub const SEL_INVALIDATE_PLAYBACK_STATE: &[u8] = b"invalidatePlaybackState\0";
}

// ── AVRoutePickerView (2 methods, 1 properties) ──
pub mod a_v_route_picker_view {
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_ROUTE_PICKER_BUTTON_COLOR_FOR_STATE: &[u8] = b"routePickerButtonColorForState:\0";
    pub const SEL_SET_ROUTE_PICKER_BUTTON_COLOR: &[u8] = b"setRoutePickerButtonColor:forState:\0";
}

// Total: 9 selector constants
