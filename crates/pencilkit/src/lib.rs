//! Apple PencilKit — Apple Pencil drawing from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 13+, visionOS 1+.
//!
//! ```ignore
//! let drawing = pencilkit::Drawing::new();
//! println!("Empty: {} strokes", drawing.stroke_count());
//! let data = drawing.to_data();
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"PKCanvasView\0").is_null() }
}

/// Wraps `PKDrawing`.
pub struct Drawing { inner: Id }

impl Drawing {
    /// Create an empty drawing.
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"PKDrawing\0"), new] } }
    }

    /// Load from serialized data.
    pub fn from_data(data: &[u8]) -> Option<Self> {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let d: Id = msg_send![class!(b"PKDrawing\0"), alloc];
            let d = msg_send![d, initWithData: nsdata, error: NIL];
            if d.is_null() { None } else { Some(Self { inner: d }) }
        }
    }

    /// Serialize to data bytes.
    pub fn to_data(&self) -> Vec<u8> {
        unsafe {
            let nsdata: Id = msg_send![self.inner, dataRepresentation];
            if nsdata.is_null() { return vec![]; }
            let len = CFDataGetLength(nsdata as CFDataRef) as usize;
            let ptr = CFDataGetBytePtr(nsdata as CFDataRef);
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    /// Number of strokes.
    pub fn stroke_count(&self) -> usize {
        unsafe {
            let strokes: Id = msg_send![self.inner, strokes];
            if strokes.is_null() { 0 } else { msg_send_t![usize; strokes, count] }
        }
    }

    /// Bounding rect: (x, y, width, height).
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let sel = sel_registerName(b"bounds\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f64; 4] =
                core::mem::transmute(objc_msgSend as *const ());
            let r = f(self.inner, sel);
            (r[0], r[1], r[2], r[3])
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Drawing { fn default() -> Self { Self::new() } }
impl Drop for Drawing { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Available ink types.
pub mod ink_types {
    pub const PEN: &str = "com.apple.ink.pen";
    pub const PENCIL: &str = "com.apple.ink.pencil";
    pub const MARKER: &str = "com.apple.ink.marker";
    pub const MONOLINE: &str = "com.apple.ink.monoline";
    pub const FOUNTAIN_PEN: &str = "com.apple.ink.fountainPen";
    pub const WATERCOLOR: &str = "com.apple.ink.watercolor";
    pub const CRAYON: &str = "com.apple.ink.crayon";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_drawing() {
        let d = Drawing::new();
        assert_eq!(d.stroke_count(), 0);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let d = Drawing::new();
        let data = d.to_data();
        assert!(!data.is_empty(), "Empty drawing should still serialize");
        let d2 = Drawing::from_data(&data);
        assert!(d2.is_some());
    }
}
