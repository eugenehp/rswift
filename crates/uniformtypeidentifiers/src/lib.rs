//! Apple UniformTypeIdentifiers — file type identification from Rust.
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
use apple_objc_sys::*;
pub fn is_available() -> bool { true }

/// Look up a UTType by filename extension.
pub fn type_for_extension(ext: &str) -> Option<String> {
    unsafe {
        let ns = nsstring(ext);
        let ut: Id = msg_send![class!(b"UTType\0"), typeWithFilenameExtension: ns];
        CFRelease(ns as CFTypeRef);
        if ut.is_null() { return None; }
        nsstring_to_string(msg_send![ut, identifier])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_png() {
        let t = type_for_extension("png");
        assert_eq!(t, Some("public.png".into()));
    }
    #[test]
    fn test_pdf() {
        let t = type_for_extension("pdf");
        assert_eq!(t, Some("com.adobe.pdf".into()));
    }
}

