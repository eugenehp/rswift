//! Apple Speech — speech recognition from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 10+, visionOS 1+.
//!
//! ```ignore
//! println!("Auth: {:?}", speech::authorization_status());
//! let recognizer = speech::Recognizer::new("en-US");
//! println!("Available: {}", recognizer.is_available());
//! println!("On-device: {}", recognizer.supports_on_device());
//! println!("Supported locales: {:?}", speech::supported_locales());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Denied = 1, Restricted = 2, Authorized = 3 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Denied, 2=>Self::Restricted, 3=>Self::Authorized, _=>Self::NotDetermined }
    }
}

/// Current speech recognition authorization status.
pub fn authorization_status() -> AuthorizationStatus {
    unsafe {
        AuthorizationStatus::from(msg_send_t![isize; class!(b"SFSpeechRecognizer\0"), authorizationStatus])
    }
}

/// List of supported locale identifiers for speech recognition.
pub fn supported_locales() -> Vec<String> {
    unsafe {
        let set: Id = msg_send![class!(b"SFSpeechRecognizer\0"), supportedLocales];
        if set.is_null() { return vec![]; }
        let arr: Id = msg_send![set, allObjects];
        let count: usize = msg_send_t![usize; arr, count];
        (0..count).filter_map(|i| {
            let locale: Id = msg_send![arr, objectAtIndex: i];
            nsstring_to_string(msg_send![locale, localeIdentifier])
        }).collect()
    }
}

/// Wraps `SFSpeechRecognizer`.
pub struct Recognizer { inner: Id }

impl Recognizer {
    /// Create a recognizer for a specific locale (e.g. `"en-US"`, `"ja-JP"`).
    pub fn new(locale_id: &str) -> Self {
        unsafe {
            let ns = nsstring(locale_id);
            let loc: Id = msg_send![class!(b"NSLocale\0"), localeWithLocaleIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            let r: Id = msg_send![class!(b"SFSpeechRecognizer\0"), alloc];
            let r = msg_send![r, initWithLocale: loc];
            Self { inner: r }
        }
    }

    /// Create a recognizer for the default locale.
    pub fn default() -> Self {
        Self { inner: unsafe { msg_send![class!(b"SFSpeechRecognizer\0"), new] } }
    }

    /// Whether speech recognition is currently available.
    pub fn is_available(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAvailable] }
    }

    /// Whether on-device recognition is supported (no network needed).
    pub fn supports_on_device(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, supportsOnDeviceRecognition] }
    }

    /// The locale identifier this recognizer uses.
    pub fn locale(&self) -> String {
        unsafe {
            let loc: Id = msg_send![self.inner, locale];
            nsstring_to_string(msg_send![loc, localeIdentifier]).unwrap_or_default()
        }
    }

    /// Default task hint for recognition.
    pub fn default_task_hint(&self) -> TaskHint {
        unsafe { TaskHint::from(msg_send_t![isize; self.inner, defaultTaskHint]) }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Recognizer {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

/// Hint about the type of speech to expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskHint {
    Unspecified = 0,
    Dictation = 1,
    Search = 2,
    Confirmation = 3,
}
impl From<isize> for TaskHint {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Dictation, 2=>Self::Search, 3=>Self::Confirmation, _=>Self::Unspecified }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_status() { let _ = authorization_status(); }

    #[test]
    fn test_supported_locales() {
        let locales = supported_locales();
        assert!(!locales.is_empty(), "Should support at least one locale");
        // en-US should always be there
        assert!(locales.iter().any(|l| l.starts_with("en")),
            "English should be supported: {:?}", locales);
    }

    #[test]
    fn test_recognizer() {
        let r = Recognizer::new("en-US");
        assert!(r.locale().starts_with("en"));
        let _ = r.is_available();
        let _ = r.supports_on_device();
    }

    #[test]
    fn test_default_recognizer() {
        let r = Recognizer::default();
        assert!(!r.locale().is_empty());
    }
}
