//! Apple Accessibility — accessibility content and charts from Rust.
//!
//! **Platform:** macOS 12+, iOS 15+, tvOS 15+.
//!
//! ```ignore
//! let content = accessibility::CustomContent::new("Price", "$9.99");
//! let content2 = accessibility::CustomContent::important("Rating", "4.5 stars");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"AXCustomContent\0").is_null() }
}

/// Importance level for custom accessibility content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Importance { Default = 0, High = 1 }

/// Wraps `AXCustomContent`.
pub struct CustomContent { inner: Id }

impl CustomContent {
    /// Create with default importance.
    pub fn new(label: &str, value: &str) -> Self {
        unsafe {
            let l = nsstring(label);
            let v = nsstring(value);
            let c: Id = msg_send![class!(b"AXCustomContent\0"), customContentWithLabel: l, value: v];
            CFRelease(l as CFTypeRef);
            CFRelease(v as CFTypeRef);
            CFRetain(c as CFTypeRef);
            Self { inner: c }
        }
    }

    /// Create with high importance (always spoken by VoiceOver).
    pub fn important(label: &str, value: &str) -> Self {
        let c = Self::new(label, value);
        unsafe {
            let sel = sel_registerName(b"setImportance:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(c.inner, sel, Importance::High as isize);
        }
        c
    }

    /// The label text.
    pub fn label(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, label]).unwrap_or_default() }
    }

    /// The value text.
    pub fn value(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, value]).unwrap_or_default() }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for CustomContent { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Chart descriptor for accessible charts (wraps `AXChartDescriptor`).
pub struct ChartDescriptor { inner: Id }

impl ChartDescriptor {
    /// Create a new chart descriptor.
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"AXChartDescriptor\0"), new] } }
    }

    /// Set the chart title.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send_void![self.inner, setTitle: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    /// Set the summary.
    pub fn set_summary(&self, summary: &str) {
        unsafe {
            let ns = nsstring(summary);
            msg_send_void![self.inner, setSummary: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for ChartDescriptor { fn default() -> Self { Self::new() } }
impl Drop for ChartDescriptor { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_content() {
        let c = CustomContent::new("Price", "$9.99");
        assert_eq!(c.label(), "Price");
        assert_eq!(c.value(), "$9.99");
    }

    #[test]
    fn test_important_content() {
        let c = CustomContent::important("Alert", "Low battery");
        assert_eq!(c.label(), "Alert");
    }
}
