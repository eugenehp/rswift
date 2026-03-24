//! Apple Translation — text translation from Rust.
//!
//! **Platform:** macOS 14+, iOS 17+.
//!
//! ```ignore
//! if translation::is_available() {
//!     let langs = translation::supported_language_codes();
//!     println!("Languages: {:?}", &langs[..5]);
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#[allow(unused_imports)] use core::ffi::c_void;

unsafe extern "C" {
    fn translation_available() -> bool;
}

pub fn is_available() -> bool {
    unsafe { translation_available() }
}

/// Get list of ISO language codes supported for translation.
pub fn supported_language_codes() -> Vec<String> {
    let mut buf = vec![0u8; 8192];
    let len = unsafe {
        extern "C" {
            fn translation_supported_languages(buf: *mut u8, len: usize) -> usize;
        }
        translation_supported_languages(buf.as_mut_ptr(), buf.len())
    };
    if len == 0 { return vec![]; }
    String::from_utf8_lossy(&buf[..len.min(buf.len())])
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available() {
        assert!(is_available());
    }

    #[test]
    fn test_languages() {
        let langs = supported_language_codes();
        assert!(langs.len() > 50, "Should have many language codes, got {}", langs.len());
    }
}
