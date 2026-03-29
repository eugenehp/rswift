#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime debug variables.
//! These are global variables used by debugging tools. Access via dlsym.

use core::ffi::c_void;

unsafe extern "C" {
    /// Pointer to the metadata allocation pool.
    pub static swift_debug_allocationPoolPointer: *const c_void;

    /// Pointer to the metadata allocation backtrace list.
    pub static swift_debug_metadataAllocationBacktraceList: *const c_void;

    /// Whether metadata allocation iteration is enabled.
    pub static swift_debug_metadataAllocationIterationEnabled: bool;

    /// The multi-payload enum pointer spare bits mask.
    pub static swift_debug_multiPayloadEnumPointerSpareBitsMask: usize;

    /// Pointer to the protocol conformance state.
    pub static swift_debug_protocolConformanceStatePointer: *const c_void;
}

// ── New in Swift 6.3 ────────────────────────────────────────────────────
// These debug variables were added in Swift 6.3 (Debug.h).  They are SPI
// and may not be present on older OS runtimes, so resolve them via dlsym.

use core::ffi::c_char;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

/// Size of the metadata allocation pool (added in Swift 6.3).
/// Returns `None` if the symbol is not available on this runtime.
pub fn debug_allocationPoolSize() -> Option<usize> {
    let ptr =
        unsafe { dlsym(RTLD_DEFAULT, c"_swift_debug_allocationPoolSize".as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { *(ptr as *const usize) })
    }
}

/// Size of the pages the metadata allocator allocates (added in Swift 6.3).
/// Useful for filtering candidate metadata pages when examining the heap.
/// Returns `None` if the symbol is not available on this runtime.
pub fn debug_metadataAllocatorPageSize() -> Option<usize> {
    let ptr = unsafe {
        dlsym(
            RTLD_DEFAULT,
            c"_swift_debug_metadataAllocatorPageSize".as_ptr(),
        )
    };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { *(ptr as *const usize) })
    }
}

/// Whether the concurrency task slab allocator is enabled (Swift 6.3+).
/// Controlled by SWIFT_DEBUG_ENABLE_TASK_SLAB_ALLOCATOR env var.
/// Returns `None` if the symbol is not available on this runtime.
pub fn debug_concurrencyEnableTaskSlabAllocator() -> Option<bool> {
    // This is a function, not a variable
    let ptr = unsafe {
        dlsym(
            RTLD_DEFAULT,
            c"swift_runtime_environment_concurrencyEnableTaskSlabAllocator".as_ptr(),
        )
    };
    if ptr.is_null() {
        None
    } else {
        type F = unsafe extern "C" fn() -> bool;
        Some(unsafe { core::mem::transmute::<_, F>(ptr)() })
    }
}
