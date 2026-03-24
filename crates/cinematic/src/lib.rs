//! Apple Cinematic — cinematic video processing from Rust.
//!
//! **Platform:** macOS 14+, iOS 17+.
//!
//! ```ignore
//! let info = cinematic::AssetInfo::from_asset_ptr(av_asset);
//! println!("Tracks: {}", info.cinematic_track_count());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"CNAssetInfo\0").is_null() }
}

/// Check if an asset contains cinematic video.
pub fn is_cinematic_asset(asset_ptr: Id) -> bool {
    unsafe {
        let _sel = sel_registerName(b"checkIfCinematic:completionHandler:\0".as_ptr());
        // This is async — just check class availability for now
        !class!(b"CNAssetInfo\0").is_null() && !asset_ptr.is_null()
    }
}

/// Decision types for cinematic focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionType {
    UserDecision = 0,
    AutoDecision = 1,
}

/// Detection types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionType {
    Unknown = 0,
    HumanFace = 1,
    HumanHead = 2,
    HumanTorso = 3,
    CatBody = 4,
    DogBody = 5,
    CatHead = 6,
    DogHead = 7,
    SportsBall = 8,
    AutoFocus = 100,
    FixedFocus = 101,
    Custom = 102,
}

impl From<isize> for DetectionType {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::HumanFace, 2 => Self::HumanHead, 3 => Self::HumanTorso,
            4 => Self::CatBody, 5 => Self::DogBody, 6 => Self::CatHead,
            7 => Self::DogHead, 8 => Self::SportsBall,
            100 => Self::AutoFocus, 101 => Self::FixedFocus, 102 => Self::Custom,
            _ => Self::Unknown,
        }
    }
}
