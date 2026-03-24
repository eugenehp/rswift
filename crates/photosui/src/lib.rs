//! Apple PhotosUI — photo picker from Rust.
//!
//! **Platform:** macOS 13+, iOS 14+.
//!
//! ```ignore
//! let config = photosui::PickerConfiguration::new();
//! config.set_selection_limit(5);
//! config.set_filter(photosui::PickerFilter::Images);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"PHPickerViewController\0").is_null() }
}

/// PHPickerFilter presets.
#[derive(Debug, Clone, Copy)]
pub enum PickerFilter { Images, Videos, LivePhotos, Screenshots, Bursts }

/// Wraps `PHPickerConfiguration`.
pub struct PickerConfiguration { inner: Id }

impl PickerConfiguration {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"PHPickerConfiguration\0"), new] } }
    }

    /// Set maximum selection (0 = unlimited).
    pub fn set_selection_limit(&self, limit: isize) {
        unsafe {
            let sel = sel_registerName(b"setSelectionLimit:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, limit);
        }
    }

    /// Set the media filter.
    pub fn set_filter(&self, filter: PickerFilter) {
        unsafe {
            let filter_obj = match filter {
                PickerFilter::Images => msg_send![class!(b"PHPickerFilter\0"), imagesFilter],
                PickerFilter::Videos => msg_send![class!(b"PHPickerFilter\0"), videosFilter],
                PickerFilter::LivePhotos => msg_send![class!(b"PHPickerFilter\0"), livePhotosFilter],
                PickerFilter::Screenshots => msg_send![class!(b"PHPickerFilter\0"), screenshotsFilter],
                PickerFilter::Bursts => msg_send![class!(b"PHPickerFilter\0"), burstsFilter],
            };
            msg_send_void![self.inner, setFilter: filter_obj];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for PickerConfiguration { fn default() -> Self { Self::new() } }
impl Drop for PickerConfiguration { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config() {
        let c = PickerConfiguration::new();
        c.set_selection_limit(3);
    }
}
