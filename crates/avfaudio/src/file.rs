//! `AVAudioFile` — reading and writing audio files.

use apple_objc_sys::*;
use crate::*;

/// An audio file for reading or writing.
///
/// <https://developer.apple.com/documentation/avfaudio/avaudiofile>
pub struct AudioFile { h: Id }

unsafe impl Send for AudioFile {}

impl AudioFile {
    /// Open a file for reading (standard deinterleaved float processing format).
    pub fn open_for_reading(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let obj = msg_send![class!(b"AVAudioFile\0"), alloc];
            let h = msg_send![obj, initForReading: url, error: NIL];
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    /// Open a file for reading with a specific processing format.
    pub fn open_for_reading_with_format(
        path: &str, format: AudioCommonFormat, interleaved: bool,
    ) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let obj = msg_send![class!(b"AVAudioFile\0"), alloc];
            let sel = sel_registerName(b"initForReading:commonFormat:interleaved:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, usize, bool, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let h = f(obj, sel, url, format as usize, interleaved, NIL);
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    /// Open/create a file for writing with the given settings dictionary.
    pub fn open_for_writing(path: &str, settings: Id) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let obj = msg_send![class!(b"AVAudioFile\0"), alloc];
            let h = msg_send![obj, initForWriting: url, settings: settings, error: NIL];
            if h.is_null() { None } else { Some(Self { h }) }
        }
    }

    pub fn as_raw(&self) -> Id { self.h }

    // ── I/O ─────────────────────────────────────────────────────────────

    /// Read into a PCM buffer. Returns `true` on success.
    pub fn read_into_buffer(&self, buffer: &AudioPCMBuffer) -> bool {
        unsafe { msg_send_t![bool; self.h, readIntoBuffer: buffer.as_raw(), error: NIL] }
    }

    /// Read a specific number of frames into a buffer.
    pub fn read_into_buffer_frame_count(&self, buffer: &AudioPCMBuffer, frames: AudioFrameCount) -> bool {
        unsafe {
            let sel = sel_registerName(b"readIntoBuffer:frameCount:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, u32, Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, buffer.as_raw(), frames, NIL)
        }
    }

    /// Write from a PCM buffer.
    pub fn write_from_buffer(&self, buffer: &AudioPCMBuffer) -> bool {
        unsafe { msg_send_t![bool; self.h, writeFromBuffer: buffer.as_raw(), error: NIL] }
    }

    /// Close the file.
    pub fn close(&self) { unsafe { msg_send_void(self.h, b"close\0"); } }

    // ── Properties ──────────────────────────────────────────────────────

    /// Whether the file is open.
    pub fn is_open(&self) -> bool { unsafe { msg_send_bool(self.h, b"isOpen\0") } }

    /// The on-disk format.
    pub fn file_format(&self) -> AudioFormat {
        let h = unsafe { msg_send_id(self.h, b"fileFormat\0") };
        AudioFormat::from_raw(h)
    }

    /// The processing format (what you read/write with).
    pub fn processing_format(&self) -> AudioFormat {
        let h = unsafe { msg_send_id(self.h, b"processingFormat\0") };
        AudioFormat::from_raw(h)
    }

    /// Total length in sample frames.
    pub fn length(&self) -> AudioFramePosition {
        unsafe {
            let sel = sel_registerName(b"length\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel)
        }
    }

    /// Current read/write position.
    pub fn frame_position(&self) -> AudioFramePosition {
        unsafe {
            let sel = sel_registerName(b"framePosition\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel)
        }
    }

    /// Set the read/write position (seek).
    pub fn set_frame_position(&self, pos: AudioFramePosition) {
        unsafe {
            let sel = sel_registerName(b"setFramePosition:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.h, sel, pos);
        }
    }

    /// The file URL.
    pub fn url(&self) -> Option<String> {
        unsafe {
            let u = msg_send_id(self.h, b"url\0");
            if u.is_null() { return None; }
            let s = msg_send_id(u, b"absoluteString\0");
            nsstring_to_string(s)
        }
    }
}

impl Drop for AudioFile { fn drop(&mut self) { unsafe { release(self.h); } } }
