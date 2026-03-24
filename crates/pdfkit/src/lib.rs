//! Apple PDFKit — PDF display and text extraction from Rust.
//!
//! **Platform:** macOS 10.4+, iOS 11+, visionOS 1+.
//!
//! ```ignore
//! let doc = pdfkit::Document::from_path("/path/to/file.pdf").unwrap();
//! println!("Pages: {}", doc.page_count());
//! if let Some(text) = doc.page_text(0) { println!("{text}"); }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// A PDF document.
pub struct Document { inner: Id }

impl Document {
    /// Open a PDF from a file path.
    pub fn from_path(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let doc: Id = msg_send![class!(b"PDFDocument\0"), alloc];
            let doc = msg_send![doc, initWithURL: url];
            if doc.is_null() { None } else { Some(Self { inner: doc }) }
        }
    }

    /// Open a PDF from raw bytes.
    pub fn from_data(data: &[u8]) -> Option<Self> {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let doc: Id = msg_send![class!(b"PDFDocument\0"), alloc];
            let doc = msg_send![doc, initWithData: nsdata];
            if doc.is_null() { None } else { Some(Self { inner: doc }) }
        }
    }

    /// Number of pages.
    pub fn page_count(&self) -> usize {
        unsafe { msg_send_t![usize; self.inner, pageCount] }
    }

    /// Get a page by index.
    pub fn page(&self, index: usize) -> Option<Page> {
        unsafe {
            let sel = sel_registerName(b"pageAtIndex:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let p = f(self.inner, sel, index);
            if p.is_null() { None } else { Some(Page { inner: p }) }
        }
    }

    /// Extract text from a specific page.
    pub fn page_text(&self, index: usize) -> Option<String> {
        self.page(index).and_then(|p| p.text())
    }

    /// Extract all text from the entire document.
    pub fn all_text(&self) -> String {
        let mut result = String::new();
        for i in 0..self.page_count() {
            if let Some(text) = self.page_text(i) {
                if !result.is_empty() { result.push('\n'); }
                result.push_str(&text);
            }
        }
        result
    }

    /// Whether the document is encrypted.
    pub fn is_encrypted(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isEncrypted] }
    }

    /// Whether the document is locked (needs password).
    pub fn is_locked(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isLocked] }
    }

    /// Try to unlock with a password.
    pub fn unlock(&self, password: &str) -> bool {
        unsafe {
            let ns = nsstring(password);
            let r = msg_send_t![bool; self.inner, unlockWithPassword: ns];
            CFRelease(ns as CFTypeRef);
            r
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Document {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

/// A single PDF page.
pub struct Page { inner: Id }

impl Page {
    /// Extract the text content of this page.
    pub fn text(&self) -> Option<String> {
        unsafe {
            let s: Id = msg_send![self.inner, string];
            nsstring_to_string(s)
        }
    }

    /// Page label (e.g. "i", "ii", "1", "2").
    pub fn label(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, label]) }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}
