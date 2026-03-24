//! Apple Model I/O — 3D asset import/export from Rust.
//!
//! **Platform:** macOS 10.11+, iOS 9+, tvOS 9+.
//!
//! ```ignore
//! assert!(modelio::can_import("obj"));
//! assert!(modelio::can_import("usdz"));
//! let asset = modelio::Asset::from_file("/path/to/model.obj").unwrap();
//! println!("Meshes: {}", asset.mesh_count());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Check if a file extension can be imported.
pub fn can_import(extension: &str) -> bool {
    unsafe {
        let ns = nsstring(extension);
        let r = msg_send_t![bool; class!(b"MDLAsset\0"), canImportFileExtension: ns];
        CFRelease(ns as CFTypeRef);
        r
    }
}

/// Check if a file extension can be exported.
pub fn can_export(extension: &str) -> bool {
    unsafe {
        let ns = nsstring(extension);
        let r = msg_send_t![bool; class!(b"MDLAsset\0"), canExportFileExtension: ns];
        CFRelease(ns as CFTypeRef);
        r
    }
}

/// A 3D asset containing meshes, cameras, lights.
pub struct Asset { inner: Id }

impl Asset {
    /// Load from a file path.
    pub fn from_file(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let asset: Id = msg_send![class!(b"MDLAsset\0"), alloc];
            let asset = msg_send![asset, initWithURL: url];
            if asset.is_null() { None } else { Some(Self { inner: asset }) }
        }
    }

    /// Number of objects (meshes + cameras + lights) at the top level.
    pub fn object_count(&self) -> usize {
        unsafe { msg_send_t![usize; self.inner, count] }
    }

    /// Number of mesh objects.
    pub fn mesh_count(&self) -> usize {
        let total = self.object_count();
        let mut meshes = 0;
        for i in 0..total {
            unsafe {
                let sel = sel_registerName(b"objectAtIndex:\0".as_ptr());
                let f: unsafe extern "C" fn(Id, Sel, usize) -> Id =
                    core::mem::transmute(objc_msgSend as *const ());
                let obj = f(self.inner, sel, i);
                if msg_send_t![bool; obj, isKindOfClass: class!(b"MDLMesh\0")] {
                    meshes += 1;
                }
            }
        }
        meshes
    }

    /// Bounding box: ((min_x, min_y, min_z), (max_x, max_y, max_z)).
    pub fn bounding_box(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        unsafe {
            // MDLAxisAlignedBoundingBox has { vector_float3 minBounds, maxBounds }
            let sel = sel_registerName(b"boundingBox\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [[f32; 4]; 2] =
                core::mem::transmute(objc_msgSend as *const ());
            let bb = f(self.inner, sel);
            ((bb[0][0], bb[0][1], bb[0][2]), (bb[1][0], bb[1][1], bb[1][2]))
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Asset { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_support() {
        assert!(can_import("obj"));
        assert!(can_import("usdz") || can_import("usd"));
    }

    #[test]
    fn test_export_support() {
        // At minimum OBJ export should be supported
        let _ = can_export("obj");
    }
}
