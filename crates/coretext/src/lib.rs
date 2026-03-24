#![allow(unsafe_op_in_unsafe_fn)]
//! Apple Core Text — font handling and text layout from Rust.
//!
//! **Platform:** macOS 10.5+, iOS 3.2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Pure C framework — all symbols linked directly.
//!
//! ```ignore
//! let font = coretext::Font::with_name("Helvetica", 14.0).unwrap();
//! println!("Family: {}", font.family_name().unwrap());
//! println!("Ascent: {:.1}", font.ascent());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_snake_case)]

use core::ffi::c_void;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


type CTFontRef = *mut c_void;
type CFStringRef = *const c_void;
type CFAllocatorRef = *const c_void;
type CGFloat = f64;

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

unsafe extern "C" {
    fn CFRelease(cf: *const c_void);
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef, bytes: *const u8, num_bytes: isize,
        encoding: u32, is_external: bool,
    ) -> CFStringRef;
    fn CFStringGetCStringPtr(s: CFStringRef, encoding: u32) -> *const u8;
    fn CFStringGetLength(s: CFStringRef) -> isize;
    fn CFStringGetCString(s: CFStringRef, buf: *mut u8, size: isize, enc: u32) -> bool;

    fn CTFontCreateWithName(name: CFStringRef, size: CGFloat, matrix: *const c_void) -> CTFontRef;
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> u32;
    fn CTFontGetGlyphCount(font: CTFontRef) -> isize;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
}

unsafe fn cfstring_to_string(s: CFStringRef) -> Option<String> {
    if s.is_null() { return None; }
    let cptr = CFStringGetCStringPtr(s, K_CF_STRING_ENCODING_UTF8);
    if !cptr.is_null() {
        let cstr = core::ffi::CStr::from_ptr(cptr as *const core::ffi::c_char);
        return Some(cstr.to_string_lossy().into_owned());
    }
    let len = CFStringGetLength(s);
    let buf_size = len * 4 + 1;
    let mut buf = vec![0u8; buf_size as usize];
    if CFStringGetCString(s, buf.as_mut_ptr(), buf_size, K_CF_STRING_ENCODING_UTF8) {
        let cstr = core::ffi::CStr::from_ptr(buf.as_ptr() as *const core::ffi::c_char);
        Some(cstr.to_string_lossy().into_owned())
    } else { None }
}

/// A Core Text font.
pub struct Font(CTFontRef);

impl Font {
    /// Create a font by name and size.
    pub fn with_name(name: &str, size: f64) -> Option<Self> {
        unsafe {
            let cf = CFStringCreateWithBytes(
                core::ptr::null(), name.as_ptr(), name.len() as isize,
                K_CF_STRING_ENCODING_UTF8, false,
            );
            let font = CTFontCreateWithName(cf, size, core::ptr::null());
            CFRelease(cf);
            if font.is_null() { None } else { Some(Self(font)) }
        }
    }

    pub fn size(&self) -> f64 { unsafe { CTFontGetSize(self.0) } }
    pub fn ascent(&self) -> f64 { unsafe { CTFontGetAscent(self.0) } }
    pub fn descent(&self) -> f64 { unsafe { CTFontGetDescent(self.0) } }
    pub fn leading(&self) -> f64 { unsafe { CTFontGetLeading(self.0) } }
    pub fn units_per_em(&self) -> u32 { unsafe { CTFontGetUnitsPerEm(self.0) } }
    pub fn glyph_count(&self) -> usize { unsafe { CTFontGetGlyphCount(self.0) as usize } }

    pub fn family_name(&self) -> Option<String> {
        unsafe { let s = CTFontCopyFamilyName(self.0); let r = cfstring_to_string(s); CFRelease(s); r }
    }
    pub fn full_name(&self) -> Option<String> {
        unsafe { let s = CTFontCopyFullName(self.0); let r = cfstring_to_string(s); CFRelease(s); r }
    }
    pub fn postscript_name(&self) -> Option<String> {
        unsafe { let s = CTFontCopyPostScriptName(self.0); let r = cfstring_to_string(s); CFRelease(s); r }
    }
    pub fn display_name(&self) -> Option<String> {
        unsafe { let s = CTFontCopyDisplayName(self.0); let r = cfstring_to_string(s); CFRelease(s); r }
    }

    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}

impl Drop for Font { fn drop(&mut self) { unsafe { CFRelease(self.0 as *const c_void); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_font() {
        let font = Font::with_name("Helvetica", 14.0).unwrap();
        assert_eq!(font.family_name(), Some("Helvetica".into()));
        assert!((font.size() - 14.0).abs() < 0.01);
        assert!(font.ascent() > 0.0);
        assert!(font.glyph_count() > 0);
    }

    #[test]
    fn test_system_font() {
        // .AppleSystemUIFont or similar
        let font = Font::with_name(".AppleSystemUIFont", 12.0);
        assert!(font.is_some());
    }

    #[test]
    fn test_font_names() {
        let font = Font::with_name("Times New Roman", 16.0).unwrap();
        assert!(font.full_name().is_some());
        assert!(font.postscript_name().is_some());
    }
}
