#![allow(unsafe_op_in_unsafe_fn)]
//! Apple Photos — photo library access from Rust.
//!
//! **Platform:** macOS 10.13+, iOS 8+, tvOS 10+, visionOS 1+.
//!
//! ```ignore
//! println!("Auth: {:?}", photos::authorization_status(photos::AccessLevel::ReadWrite));
//! let assets = photos::fetch_assets(None, 10);
//! for a in &assets { println!("{} {}×{}", a.local_identifier, a.pixel_width, a.pixel_height); }
//! let albums = photos::fetch_albums();
//! for a in &albums { println!("{}: {} items", a.title, a.count); }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Restricted = 1, Denied = 2, Authorized = 3, Limited = 4 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Restricted, 2=>Self::Denied, 3=>Self::Authorized, 4=>Self::Limited, _=>Self::NotDetermined }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AccessLevel { AddOnly = 1, ReadWrite = 2 }

/// Check photo library authorization status.
pub fn authorization_status(level: AccessLevel) -> AuthorizationStatus {
    unsafe {
        let sel = sel_registerName(b"authorizationStatusForAccessLevel:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, isize) -> isize =
            core::mem::transmute(objc_msgSend as *const ());
        AuthorizationStatus::from(f(class!(b"PHPhotoLibrary\0") as Id, sel, level as isize))
    }
}

// ── Media type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType { Unknown = 0, Image = 1, Video = 2, Audio = 3 }
impl From<isize> for MediaType {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Image, 2=>Self::Video, 3=>Self::Audio, _=>Self::Unknown }
    }
}

// ── Asset info ──────────────────────────────────────────────────────────────

/// Summary of a photo/video asset.
#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub local_identifier: String,
    pub media_type: MediaType,
    pub pixel_width: usize,
    pub pixel_height: usize,
    /// Creation date as Unix timestamp (seconds since 1970).
    pub creation_date: Option<f64>,
    /// Duration in seconds (0 for photos).
    pub duration: f64,
    pub is_favorite: bool,
    pub is_hidden: bool,
}

/// Fetch recent assets from the photo library.
///
/// - `media_type`: `None` for all, or filter by type.
/// - `limit`: maximum number of results (0 = unlimited).
///
/// Requires read authorization.
pub fn fetch_assets(media_type: Option<MediaType>, limit: usize) -> Vec<AssetInfo> {
    unsafe {
        let opts: Id = msg_send![class!(b"PHFetchOptions\0"), new];

        if limit > 0 {
            let sel = sel_registerName(b"setFetchLimit:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(opts, sel, limit);
        }

        // Sort by creation date descending
        let key = nsstring("creationDate");
        let sd: Id = msg_send![class!(b"NSSortDescriptor\0"), alloc];
        let sd = msg_send![sd, initWithKey: key, ascending: 0u8];
        let arr = msg_send![class!(b"NSArray\0"), arrayWithObject: sd];
        msg_send_void![opts, setSortDescriptors: arr];
        CFRelease(key as CFTypeRef);

        // Filter by media type
        if let Some(mt) = media_type {
            let fmt = nsstring(&format!("mediaType == {}", mt as isize));
            let pred: Id = msg_send![class!(b"NSPredicate\0"), predicateWithFormat: fmt];
            msg_send_void![opts, setPredicate: pred];
            CFRelease(fmt as CFTypeRef);
        }

        let result: Id = msg_send![class!(b"PHAsset\0"), fetchAssetsWithOptions: opts];
        CFRelease(opts as CFTypeRef);

        if result.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; result, count];

        let mut assets = Vec::with_capacity(count);
        for i in 0..count {
            let a: Id = msg_send![result, objectAtIndex: i];
            assets.push(read_asset(a));
        }
        assets
    }
}

unsafe fn read_asset(a: Id) -> AssetInfo {
    let local_id = nsstring_to_string(msg_send![a, localIdentifier]).unwrap_or_default();
    let media_type = MediaType::from(msg_send_t![isize; a, mediaType]);
    let pixel_width: usize = msg_send_t![usize; a, pixelWidth];
    let pixel_height: usize = msg_send_t![usize; a, pixelHeight];
    let is_favorite: bool = msg_send_t![bool; a, isFavorite];
    let is_hidden: bool = msg_send_t![bool; a, isHidden];

    let dur_sel = sel_registerName(b"duration\0".as_ptr());
    let dur_f: unsafe extern "C" fn(Id, Sel) -> f64 =
        core::mem::transmute(objc_msgSend as *const ());
    let duration = dur_f(a, dur_sel);

    let date: Id = msg_send![a, creationDate];
    let creation_date = if date.is_null() { None } else {
        let ts_sel = sel_registerName(b"timeIntervalSince1970\0".as_ptr());
        let ts_f: unsafe extern "C" fn(Id, Sel) -> f64 =
            core::mem::transmute(objc_msgSend as *const ());
        Some(ts_f(date, ts_sel))
    };

    AssetInfo { local_identifier: local_id, media_type, pixel_width, pixel_height,
                creation_date, duration, is_favorite, is_hidden }
}

/// Number of assets in the library.
pub fn asset_count() -> usize {
    unsafe {
        let opts: Id = msg_send![class!(b"PHFetchOptions\0"), new];
        let result: Id = msg_send![class!(b"PHAsset\0"), fetchAssetsWithOptions: opts];
        CFRelease(opts as CFTypeRef);
        if result.is_null() { 0 } else { msg_send_t![usize; result, count] }
    }
}

// ── Albums ──────────────────────────────────────────────────────────────────

/// Summary of a photo album / smart album.
#[derive(Debug, Clone)]
pub struct AlbumInfo {
    pub local_identifier: String,
    pub title: String,
    pub count: usize,
}

/// Fetch user-created albums.
pub fn fetch_albums() -> Vec<AlbumInfo> {
    fetch_collections(1, 2) // PHAssetCollectionTypeAlbum, PHAssetCollectionSubtypeAlbumRegular
}

/// Fetch smart albums (Recents, Favorites, Screenshots, etc.).
pub fn fetch_smart_albums() -> Vec<AlbumInfo> {
    fetch_collections(2, 200) // PHAssetCollectionTypeSmartAlbum, any subtype
}

fn fetch_collections(coll_type: isize, subtype: isize) -> Vec<AlbumInfo> {
    unsafe {
        let sel = sel_registerName(b"fetchAssetCollectionsWithType:subtype:options:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, isize, isize, Id) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let result = f(class!(b"PHAssetCollection\0") as Id, sel, coll_type, subtype, NIL);
        if result.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; result, count];

        let mut albums = Vec::with_capacity(count);
        for i in 0..count {
            let c: Id = msg_send![result, objectAtIndex: i];
            let local_id = nsstring_to_string(msg_send![c, localIdentifier]).unwrap_or_default();
            let title = nsstring_to_string(msg_send![c, localizedTitle]).unwrap_or_default();
            let est: usize = msg_send_t![usize; c, estimatedAssetCount];
            albums.push(AlbumInfo { local_identifier: local_id, title, count: est });
        }
        albums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_status() {
        let _ = authorization_status(AccessLevel::ReadWrite);
    }

    #[test]
    fn test_media_type() {
        assert_eq!(MediaType::from(1), MediaType::Image);
        assert_eq!(MediaType::from(2), MediaType::Video);
    }
}
