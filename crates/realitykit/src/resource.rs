//! Resource handles — EnvironmentResource, TextureResource.
//! Re-exported from [`material`] and [`entity`] for convenience.

use core::ffi::c_void;

/// An HDR environment cube map for image-based lighting.
///
/// Create with [`EnvironmentResource::load`] or [`EnvironmentResource::load_named`],
/// then pass to [`crate::Entity::image_based_light`].
pub struct EnvironmentResource {
    pub(crate) ptr: *mut c_void,
}
unsafe impl Send for EnvironmentResource {}
unsafe impl Sync for EnvironmentResource {}

impl Clone for EnvironmentResource {
    fn clone(&self) -> Self {
        unsafe { realitykit_sys::rk_retain(self.ptr) };
        EnvironmentResource { ptr: self.ptr }
    }
}
impl Drop for EnvironmentResource {
    fn drop(&mut self) { unsafe { realitykit_sys::rk_release(self.ptr) }; }
}

impl EnvironmentResource {
    /// Load from a `.hdr`, `.exr`, or `.skybox` file path.
    pub fn load(path: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_environment_resource_load(
                path.as_ptr(), path.len(), err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(EnvironmentResource { ptr: p })
        }
    }

    /// Load from the main bundle by resource name.
    pub fn load_named(name: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe {
            realitykit_sys::rk_environment_resource_load_named(
                name.as_ptr(), name.len(), err.as_mut_ptr(), err.len(),
            )
        };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(EnvironmentResource { ptr: p })
        }
    }
}
