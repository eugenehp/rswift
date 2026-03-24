//! ObjC selector constants for LinkPresentation.
#![allow(dead_code)]

// ── LPMetadataProvider (3 methods, 2 properties) ──
pub mod l_p_metadata_provider {
    pub const CLASS: &[u8] = b"LPMetadataProvider\0";
    pub const SEL_SHOULD_FETCH_SUBRESOURCES: &[u8] = b"shouldFetchSubresources\0";
    pub const SEL_SET_SHOULD_FETCH_SUBRESOURCES: &[u8] = b"setShouldFetchSubresources:\0";
    pub const SEL_TIMEOUT: &[u8] = b"timeout\0";
    pub const SEL_SET_TIMEOUT: &[u8] = b"setTimeout:\0";
    pub const SEL_CANCEL: &[u8] = b"cancel\0";
}

// ── LPLinkMetadata (0 methods, 7 properties) ──
pub mod l_p_link_metadata {
    pub const SEL_ORIGINAL_U_R_L: &[u8] = b"originalURL\0";
    pub const SEL_SET_ORIGINAL_U_R_L: &[u8] = b"setOriginalURL:\0";
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:\0";
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
    pub const SEL_ICON_PROVIDER: &[u8] = b"iconProvider\0";
    pub const SEL_SET_ICON_PROVIDER: &[u8] = b"setIconProvider:\0";
    pub const SEL_IMAGE_PROVIDER: &[u8] = b"imageProvider\0";
    pub const SEL_SET_IMAGE_PROVIDER: &[u8] = b"setImageProvider:\0";
    pub const SEL_VIDEO_PROVIDER: &[u8] = b"videoProvider\0";
    pub const SEL_SET_VIDEO_PROVIDER: &[u8] = b"setVideoProvider:\0";
    pub const SEL_REMOTE_VIDEO_U_R_L: &[u8] = b"remoteVideoURL\0";
    pub const SEL_SET_REMOTE_VIDEO_U_R_L: &[u8] = b"setRemoteVideoURL:\0";
}

// ── LPLinkView (1 methods, 1 properties) ──
pub mod l_p_link_view {
    pub const SEL_METADATA: &[u8] = b"metadata\0";
    pub const SEL_SET_METADATA: &[u8] = b"setMetadata:\0";
    pub const SEL_ENCODE_WITH_CODER: &[u8] = b"encodeWithCoder:\0";
}

// Total: 24 selector constants
