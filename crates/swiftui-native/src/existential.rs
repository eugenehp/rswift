//! `any View` existential container — pure Rust, no Swift code.
//!
//! Layout (arm64, 64-bit):
//!   [0..24]  inline value buffer (3 words) — types ≤24 bytes stored inline
//!            OR [0..8] = heap box pointer for types >24 bytes
//!   [24..32] type metadata pointer
//!   [32..40] View protocol witness table pointer

use core::ffi::c_void;
use swift_runtime_sys::SwiftABI::get_value_witness_table;

/// A heap-allocated `any View` existential container.
///
/// This is the native alternative to AnyView — constructed entirely from Rust
/// using the Swift runtime's value witness tables and metadata.
pub struct ViewExistential {
    /// Heap-allocated 40-byte existential container.
    buf: *mut u8,
}

const CONTAINER_SIZE: usize = 40;
const INLINE_SIZE: usize = 24;

impl ViewExistential {
    /// Create a new existential from a concrete view value.
    ///
    /// # Safety
    /// - `value` must point to a valid instance of the type described by `metadata`
    /// - `metadata` must be a valid Swift type metadata pointer
    /// - `view_wt` must be a valid View protocol witness table for the type
    pub unsafe fn new(
        value: *const c_void,
        value_size: usize,
        metadata: *const c_void,
        view_wt: *const c_void,
    ) -> Self {
        let buf = libc::malloc(CONTAINER_SIZE) as *mut u8;
        assert!(!buf.is_null(), "Failed to allocate existential container");
        core::ptr::write_bytes(buf, 0, CONTAINER_SIZE);

        if value_size <= INLINE_SIZE {
            // Inline: copy value directly into buffer
            core::ptr::copy_nonoverlapping(value as *const u8, buf, value_size);
        } else {
            // Box: allocate and copy, store box pointer in first word
            let vwt = &*get_value_witness_table(metadata);
            let heap_buf = libc::malloc(value_size);
            assert!(!heap_buf.is_null());
            // Use VWT initializeWithCopy for proper reference counting
            (vwt.initialize_with_copy)(heap_buf, value as *mut c_void, metadata);
            *(buf as *mut *mut c_void) = heap_buf;
        }

        // Write metadata pointer at offset 24
        *((buf.add(24)) as *mut *const c_void) = metadata;
        // Write witness table at offset 32
        *((buf.add(32)) as *mut *const c_void) = view_wt;

        Self { buf }
    }

    /// Create from a zero-size type (EmptyView, Divider).
    pub unsafe fn from_zero_size(metadata: *const c_void, view_wt: *const c_void) -> Self {
        Self::new(core::ptr::null(), 0, metadata, view_wt)
    }

    /// Get the type metadata pointer.
    pub fn metadata(&self) -> *const c_void {
        unsafe { *((self.buf.add(24)) as *const *const c_void) }
    }

    /// Get the View witness table pointer.
    pub fn witness_table(&self) -> *const c_void {
        unsafe { *((self.buf.add(32)) as *const *const c_void) }
    }

    /// Get pointer to the value (inline buffer start).
    pub fn value_ptr(&self) -> *const c_void {
        let vwt = unsafe { &*get_value_witness_table(self.metadata()) };
        if vwt.size <= INLINE_SIZE {
            self.buf as *const c_void
        } else {
            unsafe { *(self.buf as *const *const c_void) }
        }
    }

    /// Get the raw existential container pointer (40 bytes).
    pub fn as_raw(&self) -> *const c_void {
        self.buf as *const c_void
    }
}

impl Drop for ViewExistential {
    fn drop(&mut self) {
        if !self.buf.is_null() {
            let metadata = self.metadata();
            if !metadata.is_null() {
                let vwt = unsafe { &*get_value_witness_table(metadata) };
                if vwt.size <= INLINE_SIZE {
                    // Destroy inline value
                    unsafe { (vwt.destroy)(self.buf as *mut c_void, metadata) };
                } else {
                    // Destroy boxed value and free box
                    let box_ptr = unsafe { *(self.buf as *mut *mut c_void) };
                    if !box_ptr.is_null() {
                        unsafe { (vwt.destroy)(box_ptr, metadata) };
                        unsafe { libc::free(box_ptr) };
                    }
                }
            }
            unsafe { libc::free(self.buf as *mut c_void) };
            self.buf = core::ptr::null_mut();
        }
    }
}

impl Clone for ViewExistential {
    fn clone(&self) -> Self {
        let metadata = self.metadata();
        let view_wt = self.witness_table();
        let vwt = unsafe { &*get_value_witness_table(metadata) };

        let new_buf = unsafe { libc::malloc(CONTAINER_SIZE) as *mut u8 };
        assert!(!new_buf.is_null());
        unsafe { core::ptr::write_bytes(new_buf, 0, CONTAINER_SIZE) };

        if vwt.size <= INLINE_SIZE {
            // Copy inline value using VWT
            unsafe {
                (vwt.initialize_with_copy)(
                    new_buf as *mut c_void,
                    self.buf as *mut c_void,
                    metadata,
                );
            }
        } else {
            // Allocate new box and copy
            let new_box = unsafe { libc::malloc(vwt.size) };
            assert!(!new_box.is_null());
            let old_box = unsafe { *(self.buf as *const *mut c_void) };
            unsafe { (vwt.initialize_with_copy)(new_box, old_box, metadata) };
            unsafe { *(new_buf as *mut *mut c_void) = new_box };
        }

        // Copy metadata + witness table
        unsafe {
            *((new_buf.add(24)) as *mut *const c_void) = metadata;
            *((new_buf.add(32)) as *mut *const c_void) = view_wt;
        }

        Self { buf: new_buf }
    }
}

unsafe impl Send for ViewExistential {}
