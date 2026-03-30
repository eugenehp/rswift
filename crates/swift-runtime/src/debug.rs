//! Runtime debug variable accessors.
//!
//! These provide access to Swift runtime debugging facilities.
//! Some variables are only available on Swift 6.3+ runtimes.

/// Metadata allocation pool size (Swift 6.3+).
/// Returns `None` on older runtimes.
pub fn allocation_pool_size() -> Option<usize> {
    swift_runtime_sys::DebugVars::debug_allocationPoolSize()
}

/// Metadata allocator page size (Swift 6.3+).
/// Returns `None` on older runtimes.
pub fn metadata_allocator_page_size() -> Option<usize> {
    swift_runtime_sys::DebugVars::debug_metadataAllocatorPageSize()
}

/// Whether the concurrency task slab allocator is enabled (Swift 6.3+).
/// Returns `None` on older runtimes.
pub fn task_slab_allocator_enabled() -> Option<bool> {
    swift_runtime_sys::DebugVars::debug_concurrencyEnableTaskSlabAllocator()
}
