//! Apple Vision — image analysis, OCR, face detection from Rust.
//!
//! **Platform:** macOS 10.13+, iOS 11+, tvOS 11+, visionOS 1+.
//!
//! ```ignore
//! let request = vision::TextRecognitionRequest::new();
//! println!("Supported languages: {:?}", request.supported_languages());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Wraps `VNRecognizeTextRequest` for OCR.
pub struct TextRecognitionRequest { inner: Id }

impl TextRecognitionRequest {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"VNRecognizeTextRequest\0"), new] } }
    }

    /// List of supported recognition languages.
    pub fn supported_languages(&self) -> Vec<String> {
        unsafe {
            let sel = sel_registerName(b"supportedRecognitionLanguagesAndReturnError:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(self.inner, sel, NIL);
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
                .collect()
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for TextRecognitionRequest { fn default() -> Self { Self::new() } }
impl Drop for TextRecognitionRequest { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Wraps `VNDetectFaceRectanglesRequest`.
pub struct FaceDetectionRequest { inner: Id }

impl FaceDetectionRequest {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"VNDetectFaceRectanglesRequest\0"), new] } }
    }
    pub fn as_ptr(&self) -> Id { self.inner }
}
impl Default for FaceDetectionRequest { fn default() -> Self { Self::new() } }
impl Drop for FaceDetectionRequest { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Wraps `VNDetectBarcodesRequest`.
pub struct BarcodeDetectionRequest { inner: Id }

impl BarcodeDetectionRequest {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"VNDetectBarcodesRequest\0"), new] } }
    }
    pub fn as_ptr(&self) -> Id { self.inner }
}
impl Default for BarcodeDetectionRequest { fn default() -> Self { Self::new() } }
impl Drop for BarcodeDetectionRequest { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_text_request() {
        let req = TextRecognitionRequest::new();
        let langs = req.supported_languages();
        assert!(!langs.is_empty(), "Should support at least one language");
    }
}
