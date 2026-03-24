//! Apple MetalKit — Metal rendering helpers from Rust.
//!
//! **Platform:** macOS 10.11+, iOS 9+, tvOS 9+, visionOS 1+.
//!
//! ```ignore
//! let device = metal::Device::system_default().unwrap();
//! let loader = metalkit::TextureLoader::new(&device);
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {

    unsafe { !class!(b"MTKView\0").is_null() }
}

/// Wraps `MTKTextureLoader` for loading textures from files/data.
pub struct TextureLoader { inner: Id }

impl TextureLoader {
    /// Create a texture loader for a Metal device.
    pub fn new(device_ptr: Id) -> Self {
        unsafe {
            let tl: Id = msg_send![class!(b"MTKTextureLoader\0"), alloc];
            let tl = msg_send![tl, initWithDevice: device_ptr];
            Self { inner: tl }
        }
    }

    /// Load a texture from a file path (synchronous).
    pub fn from_file(&self, path: &str) -> Result<Id, String> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let mut error: Id = NIL;
            let sel = sel_registerName(b"newTextureWithContentsOfURL:options:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let tex = f(self.inner, sel, url, NIL, &mut error);
            if tex.is_null() {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Load failed".into())
                } else { "Load failed".into() };
                Err(desc)
            } else { Ok(tex) }
        }
    }

    /// Load a texture from raw data (synchronous).
    pub fn from_data(&self, data: &[u8]) -> Result<Id, String> {
        unsafe {
            let nsdata: Id = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
            let mut error: Id = NIL;
            let sel = sel_registerName(b"newTextureWithData:options:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let tex = f(self.inner, sel, nsdata, NIL, &mut error);
            if tex.is_null() {
                Err("Failed to load texture from data".into())
            } else { Ok(tex) }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for TextureLoader { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Wraps `MTKMesh` for loading 3D meshes via Model I/O.
pub struct Mesh;

impl Mesh {
    /// Load meshes from a Model I/O asset (returns raw MTKMesh pointers).
    pub fn from_asset(asset_ptr: Id, device_ptr: Id) -> Vec<Id> {
        unsafe {
            let mut error: Id = NIL;
            let sel = sel_registerName(b"newMeshesFromAsset:device:sourceMeshes:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id, Id, *mut Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(class!(b"MTKMesh\0") as Id, sel, asset_ptr, device_ptr, NIL, &mut error);
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count).map(|i| msg_send![arr, objectAtIndex: i]).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available() { assert!(is_available()); }
}
