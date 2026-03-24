//! ObjC selector constants for PhotosUI.
#![allow(dead_code)]

// ── PHPickerViewController (6 methods, 0 properties) ──
pub mod p_h_picker_view_controller {
    pub const CLASS: &[u8] = b"PHPickerViewController\0";
    pub const SEL_UPDATE_PICKER_USING_CONFIGURATION: &[u8] = b"updatePickerUsingConfiguration:\0";
    pub const SEL_DESELECT_ASSETS_WITH_IDENTIFIERS: &[u8] = b"deselectAssetsWithIdentifiers:\0";
    pub const SEL_MOVE_ASSET_WITH_IDENTIFIER: &[u8] = b"moveAssetWithIdentifier:afterAssetWithIdentifier:\0";
    pub const SEL_SCROLL_TO_INITIAL_POSITION: &[u8] = b"scrollToInitialPosition\0";
    pub const SEL_ZOOM_IN: &[u8] = b"zoomIn\0";
    pub const SEL_ZOOM_OUT: &[u8] = b"zoomOut\0";
}

// ── PHPickerConfiguration (0 methods, 0 properties) ──
pub mod p_h_picker_configuration {
}

// ── PHPickerResult (0 methods, 0 properties) ──
pub mod p_h_picker_result {
}

// ── PHPickerFilter (4 methods, 2 properties) ──
pub mod p_h_picker_filter {
    pub const SEL_VIDEOS_FILTER: &[u8] = b"videosFilter\0";
    pub const SEL_SET_VIDEOS_FILTER: &[u8] = b"setVideosFilter:\0";
    pub const SEL_LIVE_PHOTOS_FILTER: &[u8] = b"livePhotosFilter\0";
    pub const SEL_SET_LIVE_PHOTOS_FILTER: &[u8] = b"setLivePhotosFilter:\0";
    pub const SEL_PLAYBACK_STYLE_FILTER: &[u8] = b"playbackStyleFilter:\0";
    pub const SEL_ANY_FILTER_MATCHING_SUBFILTERS: &[u8] = b"anyFilterMatchingSubfilters:\0";
    pub const SEL_ALL_FILTER_MATCHING_SUBFILTERS: &[u8] = b"allFilterMatchingSubfilters:\0";
    pub const SEL_NOT_FILTER_OF_SUBFILTER: &[u8] = b"notFilterOfSubfilter:\0";
}

// Total: 14 selector constants
