use swift_runtime::debug;

#[test]
fn test_allocation_pool_size() {
    // May be None on pre-6.3 OS runtimes — just check it doesn't panic
    if let Some(size) = debug::allocation_pool_size() {
        assert!(size > 0);
    }
}

#[test]
fn test_metadata_allocator_page_size() {
    if let Some(size) = debug::metadata_allocator_page_size() {
        assert!(size > 0);
        assert!(size.is_power_of_two());
    }
}

#[test]
fn test_task_slab_allocator_enabled() {
    // Just verify it's callable
    let _ = debug::task_slab_allocator_enabled();
}
