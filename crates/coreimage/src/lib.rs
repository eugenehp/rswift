//! Apple Core Image — GPU-accelerated image processing from Rust.
//!
//! **Platform:** macOS 10.4+, iOS 5+, tvOS 9+.
//!
//! ```ignore
//! let img = coreimage::Image::from_file("/path/to/photo.jpg").unwrap();
//! let blurred = img.apply("CIGaussianBlur", &[("inputRadius", &10.0f64)]);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── CIImage ─────────────────────────────────────────────────────────────────

/// A Core Image image (immutable pixel data on GPU).
pub struct Image { inner: Id }

impl Image {
    /// Create from a file path.
    pub fn from_file(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let img: Id = msg_send![class!(b"CIImage\0"), imageWithContentsOfURL: url];
            if img.is_null() { None } else { CFRetain(img as CFTypeRef); Some(Self { inner: img }) }
        }
    }

    /// Create from raw RGBA bytes.
    pub fn from_rgba(data: &[u8], width: usize, height: usize) -> Self {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let sel = sel_registerName(b"imageWithBitmapData:bytesPerRow:size:format:colorSpace:\0".as_ptr());
            // CIFormat for RGBA8 = 24 (kCIFormatRGBA8)
            let f: unsafe extern "C" fn(Id, Sel, Id, usize, [f64;2], i32, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let size = [width as f64, height as f64];
            let img = f(class!(b"CIImage\0") as Id, sel, nsdata, width * 4, size, 24, NIL);
            CFRetain(img as CFTypeRef);
            Self { inner: img }
        }
    }

    /// Image extent: (x, y, width, height).
    pub fn extent(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let sel = sel_registerName(b"extent\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f64; 4] =
                core::mem::transmute(objc_msgSend as *const ());
            let r = f(self.inner, sel);
            (r[0], r[1], r[2], r[3])
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        unsafe { CFRetain(self.inner as CFTypeRef); }
        Self { inner: self.inner }
    }
}

impl Drop for Image {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── CIFilter ────────────────────────────────────────────────────────────────

/// A Core Image filter.
pub struct Filter { inner: Id }

impl Filter {
    /// Create a filter by name (e.g. `"CIGaussianBlur"`).
    pub fn new(name: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(name);
            let f: Id = msg_send![class!(b"CIFilter\0"), filterWithName: ns];
            CFRelease(ns as CFTypeRef);
            if f.is_null() { None } else { CFRetain(f as CFTypeRef); Some(Self { inner: f }) }
        }
    }

    /// Set the input image.
    pub fn set_input_image(&self, image: &Image) {
        unsafe {
            let key = nsstring("inputImage");
            msg_send_void![self.inner, setValue: image.inner, forKey: key];
            CFRelease(key as CFTypeRef);
        }
    }

    /// Set a float parameter.
    pub fn set_float(&self, key: &str, value: f64) {
        unsafe {
            let k = nsstring(key);
            // Wrap f64 in NSNumber
            let sel = sel_registerName(b"numberWithDouble:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let num = f(class!(b"NSNumber\0") as Id, sel, value);
            msg_send_void![self.inner, setValue: num, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    /// Set a string parameter.
    pub fn set_string(&self, key: &str, value: &str) {
        unsafe {
            let k = nsstring(key);
            let v = nsstring(value);
            msg_send_void![self.inner, setValue: v, forKey: k];
            CFRelease(k as CFTypeRef);
            CFRelease(v as CFTypeRef);
        }
    }

    /// Get the output image.
    pub fn output_image(&self) -> Option<Image> {
        unsafe {
            let img: Id = msg_send![self.inner, outputImage];
            if img.is_null() { None } else { CFRetain(img as CFTypeRef); Some(Image { inner: img }) }
        }
    }

    /// Reset all parameters to defaults.
    pub fn set_defaults(&self) {
        unsafe { msg_send_void![self.inner, setDefaults]; }
    }

    /// List input parameter keys.
    pub fn input_keys(&self) -> Vec<String> {
        unsafe {
            let arr: Id = msg_send![self.inner, inputKeys];
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
                .collect()
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Filter {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── CIContext ───────────────────────────────────────────────────────────────

/// A Core Image rendering context.
pub struct Context { inner: Id }

impl Context {
    /// Create a default context (uses GPU when available).
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"CIContext\0"), context] } }
    }

    /// Render a CIImage to RGBA8 pixel data.
    pub fn render_to_rgba(&self, image: &Image) -> Vec<u8> {
        let (_, _, w, h) = image.extent();
        let w = w as usize;
        let h = h as usize;
        let mut buf = vec![0u8; w * h * 4];
        unsafe {
            let sel = sel_registerName(b"render:toBitmap:rowBytes:bounds:format:colorSpace:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, *mut u8, usize, [f64;4], i32, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            let bounds = [0.0, 0.0, w as f64, h as f64];
            f(self.inner, sel, image.inner, buf.as_mut_ptr(), w * 4, bounds, 24, NIL);
        }
        buf
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for Context { fn default() -> Self { Self::new() } }

// ── Convenience ─────────────────────────────────────────────────────────────

/// List all available CIFilter names.
pub fn filter_names() -> Vec<String> {
    unsafe {
        let arr: Id = msg_send![class!(b"CIFilter\0"), filterNamesInCategories: NIL];
        if arr.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; arr, count];
        (0..count)
            .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
            .collect()
    }
}

/// Filter names in a specific category.
pub fn filter_names_in_category(category: &str) -> Vec<String> {
    unsafe {
        let ns = nsstring(category);
        let arr_cat: Id = msg_send![class!(b"NSArray\0"), arrayWithObject: ns];
        let arr: Id = msg_send![class!(b"CIFilter\0"), filterNamesInCategories: arr_cat];
        CFRelease(ns as CFTypeRef);
        if arr.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; arr, count];
        (0..count)
            .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_names() {
        let names = filter_names();
        assert!(!names.is_empty());
        assert!(names.iter().any(|n| n.contains("Blur")));
    }

    #[test]
    fn test_create_filter() {
        let f = Filter::new("CIGaussianBlur").unwrap();
        let keys = f.input_keys();
        assert!(keys.contains(&"inputRadius".to_string()));
    }

    #[test]
    fn test_filter_chain() {
        // Create a 4x4 red image
        let red = vec![255u8, 0, 0, 255].repeat(16);
        let img = Image::from_rgba(&red, 4, 4);
        let (_, _, w, h) = img.extent();
        assert_eq!(w as usize, 4);
        assert_eq!(h as usize, 4);

        // Apply color invert
        let f = Filter::new("CIColorInvert").unwrap();
        f.set_input_image(&img);
        let out = f.output_image().unwrap();
        assert!(out.extent().2 > 0.0);
    }

    #[test]
    fn test_blur_category() {
        let blurs = filter_names_in_category("CICategoryBlur");
        assert!(blurs.iter().any(|n| n == "CIGaussianBlur"));
    }
}
