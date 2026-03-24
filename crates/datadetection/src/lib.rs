//! Apple DataDetection — detect dates, links, addresses in text from Rust.
//!
//! **Platform:** macOS 14+, iOS 17+.
//!
//! ```ignore
//! let results = datadetection::detect("Call me at 555-1234 or email bob@test.com");
//! for r in &results { println!("{}: {}", r.start, r.text); }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

unsafe extern "C" {
    fn datadetection_swift_available() -> bool;
}

pub fn is_available() -> bool { unsafe { datadetection_swift_available() } }

/// A detected data item in text.
#[derive(Debug, Clone)]
pub struct Detection {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

/// Detect data patterns (dates, links, phone numbers, addresses) in text.
pub fn detect(text: &str) -> Vec<Detection> {
    let mut buf = vec![0u8; 65536];
    let len = unsafe {
        extern "C" {
            fn datadetection_detect(
                text: *const u8, text_len: usize,
                buf: *mut u8, buf_len: usize,
            ) -> isize;
        }
        datadetection_detect(text.as_ptr(), text.len(), buf.as_mut_ptr(), buf.len())
    };
    if len <= 0 { return vec![]; }
    
    // Simple JSON array parse
    let json = String::from_utf8_lossy(&buf[..len as usize]);
    let mut results = Vec::new();
    for entry in json.trim_matches(|c| c == '[' || c == ']').split("},{") {
        let entry = entry.trim_matches(|c| c == '{' || c == '}');
        let mut start = 0usize;
        let mut end = 0usize;
        let mut matched_text = String::new();
        for field in entry.split(',') {
            let parts: Vec<&str> = field.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches('"');
                let val = parts[1].trim().trim_matches('"');
                match key {
                    "start" => start = val.parse().unwrap_or(0),
                    "end" => end = val.parse().unwrap_or(0),
                    "text" => matched_text = val.to_string(),
                    _ => {}
                }
            }
        }
        if !matched_text.is_empty() {
            results.push(Detection { start, end, text: matched_text });
        }
    }
    results
}
