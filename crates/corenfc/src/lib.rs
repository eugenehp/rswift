//! Apple CoreNFC — NFC tag reading/writing from Rust.
//!
//! **Platform:** iOS 11+ (reading), iOS 13+ (tag reader).
//! Not available on macOS — `is_available()` returns false on Mac.
//!
//! ```ignore
//! if corenfc::is_available() && corenfc::is_reading_available() {
//!     println!("NFC reading supported on this device");
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" {
    fn corenfc_swift_avail() -> bool;
    fn corenfc_reading_available() -> bool;
}

pub fn is_available() -> bool { unsafe { corenfc_swift_avail() } }

/// Whether NDEF reading is available on this device.
pub fn is_reading_available() -> bool {
    unsafe { corenfc_reading_available() }
}

/// NFC tag types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// ISO 14443 Type A (e.g. MIFARE)
    Iso7816 = 1,
    /// FeliCa (Sony)
    FeliCa = 2,
    /// ISO 15693 (vicinity cards)
    Iso15693 = 3,
    /// MIFARE family
    MiFare = 4,
}

/// NDEF record type name format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeNameFormat {
    Empty = 0,
    WellKnown = 1,
    Media = 2,
    AbsoluteURI = 3,
    External = 4,
    Unknown = 5,
    Unchanged = 6,
}

/// Create an NDEF text payload.
pub fn create_text_payload(text: &str, locale: &str) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe {
        extern "C" {
            fn corenfc_create_text_payload(
                text: *const u8, text_len: usize,
                locale: *const u8, locale_len: usize,
                buf: *mut u8, buf_len: usize,
            ) -> isize;
        }
        corenfc_create_text_payload(
            text.as_ptr(), text.len(),
            locale.as_ptr(), locale.len(),
            buf.as_mut_ptr(), buf.len(),
        )
    };
    if len < 0 { None } else { buf.truncate(len as usize); Some(buf) }
}

/// Create an NDEF URI payload.
pub fn create_uri_payload(url: &str) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe {
        extern "C" {
            fn corenfc_create_uri_payload(
                url: *const u8, url_len: usize,
                buf: *mut u8, buf_len: usize,
            ) -> isize;
        }
        corenfc_create_uri_payload(url.as_ptr(), url.len(), buf.as_mut_ptr(), buf.len())
    };
    if len < 0 { None } else { buf.truncate(len as usize); Some(buf) }
}
