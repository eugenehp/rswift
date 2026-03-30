//! View handle — retained AnyView pointer from the compiled Swift shim.

use core::ffi::c_void;

unsafe extern "C" {
    fn native_retain(ptr: *mut c_void);
    fn native_release(ptr: *mut c_void);
}

/// An opaque handle to a SwiftUI AnyView, backed by a retained Swift object.
pub struct ViewHandle(*mut c_void);

impl ViewHandle {
    /// Wrap a raw pointer returned by a native_* function.
    ///
    /// # Safety
    /// `ptr` must be a retained AnyView-as-AnyObject pointer from the shim.
    pub(crate) unsafe fn from_raw(ptr: *mut c_void) -> Self {
        debug_assert!(!ptr.is_null());
        Self(ptr)
    }

    /// Get the raw pointer (for passing back to shim functions).
    pub fn as_raw(&self) -> *mut c_void {
        self.0
    }
}

impl Clone for ViewHandle {
    fn clone(&self) -> Self {
        unsafe { native_retain(self.0) };
        Self(self.0)
    }
}

impl Drop for ViewHandle {
    fn drop(&mut self) {
        unsafe { native_release(self.0) };
    }
}

unsafe impl Send for ViewHandle {}
