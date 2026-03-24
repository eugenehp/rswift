//! Apple NaturalLanguage — tokenization, language detection, NLP from Rust.
//!
//! **Platform:** macOS 10.14+, iOS 12+, tvOS 12+, watchOS 5+.
//!
//! ```ignore
//! let lang = naturallanguage::detect_language("Bonjour le monde");
//! assert_eq!(lang, Some("fr".into()));
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Detect the dominant language of a text string.
///
/// Returns an ISO 639-1 language code (e.g. `"en"`, `"fr"`, `"ja"`),
/// or `None` if the language cannot be determined.
pub fn detect_language(text: &str) -> Option<String> {
    unsafe {
        let recognizer: Id = msg_send![class!(b"NLLanguageRecognizer\0"), new];
        let ns = nsstring(text);
        msg_send_void![recognizer, processString: ns];
        CFRelease(ns as CFTypeRef);
        let lang: Id = msg_send![recognizer, dominantLanguage];
        let result = nsstring_to_string(lang);
        CFRelease(recognizer as CFTypeRef);
        result
    }
}

/// Tokenize text into words.
pub fn tokenize_words(text: &str) -> Vec<String> {
    tokenize(text, 0) // NLTokenUnitWord = 0
}

/// Tokenize text into sentences.
pub fn tokenize_sentences(text: &str) -> Vec<String> {
    tokenize(text, 1) // NLTokenUnitSentence = 1
}

fn tokenize(text: &str, unit: isize) -> Vec<String> {
    unsafe {
        let sel = sel_registerName(b"initWithUnit:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, isize) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let tok = msg_send![class!(b"NLTokenizer\0"), alloc];
        let tok = f(tok, sel, unit);
        let ns = nsstring(text);
        msg_send_void![tok, setString: ns];

        // Get full range
        let text_len = text.len();
        // NSRange { location, length }
        #[repr(C)]
        struct NSRange { location: usize, length: usize }
        let range = NSRange { location: 0, length: text_len };

        let mut tokens = Vec::new();

        // enumerateTokensInRange:usingBlock: requires a block — use tokensForRange: instead
        let sel2 = sel_registerName(b"tokensForRange:\0".as_ptr());
        let f2: unsafe extern "C" fn(Id, Sel, NSRange) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let arr = f2(tok, sel2, range);

        if !arr.is_null() {
            let count: usize = msg_send_t![usize; arr, count];
            for i in 0..count {
                // Each element is an NSValue wrapping NSRange
                let val: Id = msg_send![arr, objectAtIndex: i];
                let sel_rv = sel_registerName(b"rangeValue\0".as_ptr());
                let f_rv: unsafe extern "C" fn(Id, Sel) -> NSRange =
                    core::mem::transmute(objc_msgSend as *const ());
                let r = f_rv(val, sel_rv);
                if r.location + r.length <= text_len {
                    tokens.push(text[r.location..r.location + r.length].to_string());
                }
            }
        }

        CFRelease(ns as CFTypeRef);
        CFRelease(tok as CFTypeRef);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_english() {
        let lang = detect_language("Hello, how are you today?");
        assert_eq!(lang, Some("en".into()));
    }

    #[test]
    fn test_detect_french() {
        let lang = detect_language("Bonjour le monde, comment allez-vous?");
        assert_eq!(lang, Some("fr".into()));
    }

    #[test]
    fn test_tokenize_words() {
        let words = tokenize_words("Hello world");
        assert_eq!(words, vec!["Hello", "world"]);
    }

    #[test]
    fn test_tokenize_sentences() {
        let sents = tokenize_sentences("Hello world. How are you?");
        assert_eq!(sents.len(), 2);
    }
}
