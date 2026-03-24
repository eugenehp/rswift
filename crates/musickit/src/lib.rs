#![allow(unsafe_op_in_unsafe_fn)]
//! Apple MusicKit — Apple Music catalog and playback from Rust.
//!
//! **Platform:** macOS 12+, iOS 15+, tvOS 15+, watchOS 8+.
//!
//! ```ignore
//! println!("Auth: {:?}", musickit::authorization_status());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

unsafe extern "C" {
    fn musickit_available() -> bool;
}

pub fn is_available() -> bool { unsafe { musickit_available() } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Denied = 1, Restricted = 2, Authorized = 3 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Denied, 2=>Self::Restricted, 3=>Self::Authorized, _=>Self::NotDetermined }
    }
}

/// Current MusicKit authorization status.
pub fn authorization_status() -> AuthorizationStatus {
    unsafe {
        extern "C" { fn musickit_authorization_status() -> isize; }
        AuthorizationStatus::from(musickit_authorization_status())
    }
}

/// A song search result.
#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: f64,
}

/// Search the Apple Music catalog (async via callback).
pub fn search_catalog<F: FnOnce(Result<Vec<Song>, ()>) + Send + 'static>(
    query: &str,
    callback: F,
) {
    unsafe extern "C" fn trampoline(
        json_ptr: *const u8, json_len: usize, success: bool, ud: *mut c_void,
    ) {
        let cb: Box<Box<dyn FnOnce(Result<Vec<Song>, ()>) + Send>> =
            Box::from_raw(ud as *mut _);
        if success && json_len > 0 && !json_ptr.is_null() {
            let json = String::from_utf8_lossy(
                core::slice::from_raw_parts(json_ptr, json_len)
            ).into_owned();
            let mut songs = Vec::new();
            for entry in json.trim_matches(|c| c == '[' || c == ']').split("},{") {
                let entry = entry.trim_matches(|c| c == '{' || c == '}');
                let mut title = String::new();
                let mut artist = String::new();
                let mut duration = 0.0;
                for field in entry.split(',') {
                    let parts: Vec<&str> = field.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().trim_matches('"');
                        let val = parts[1].trim().trim_matches('"');
                        match key {
                            "title" => title = val.to_string(),
                            "artist" => artist = val.to_string(),
                            "duration" => duration = val.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                }
                if !title.is_empty() {
                    songs.push(Song { title, artist, duration });
                }
            }
            cb(Ok(songs));
        } else {
            cb(Err(()));
        }
    }

    let cb: Box<Box<dyn FnOnce(Result<Vec<Song>, ()>) + Send>> =
        Box::new(Box::new(callback));
    let ud = Box::into_raw(cb) as *mut c_void;

    unsafe {
        extern "C" {
            fn musickit_search_catalog(
                query: *const u8, query_len: usize,
                cb: unsafe extern "C" fn(*const u8, usize, bool, *mut c_void),
                ud: *mut c_void,
            );
        }
        musickit_search_catalog(query.as_ptr(), query.len(), trampoline, ud);
    }
}
