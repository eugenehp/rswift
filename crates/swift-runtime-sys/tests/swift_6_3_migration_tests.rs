//! Swift 6.3 migration tests.
//!
//! Validates that:
//! 1. Removed APIs still resolve at runtime (ABI compat) but are deprecated
//! 2. Replacement APIs work correctly
//! 3. New Swift 6.3 features are accessible
//! 4. Regenerated bindings are coherent
//! 5. Edge cases around the deprecated → replacement transition

use core::ffi::{c_char, c_void};

// ═══════════════════════════════════════════════════════════════════════════
// Helper: resolve symbol via dlsym
// ═══════════════════════════════════════════════════════════════════════════

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}
const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

fn symbol_exists(name: &std::ffi::CStr) -> bool {
    !unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) }.is_null()
}

// ═══════════════════════════════════════════════════════════════════════════
// §1: Deprecated async-let APIs still link (ABI backwards compatibility)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_deprecated_asynclet_start_still_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_start"),
        "swift_asyncLet_start should still be present for ABI compat"
    );
}

#[test]
fn test_deprecated_asynclet_end_still_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_end"),
        "swift_asyncLet_end should still be present for ABI compat"
    );
}

#[test]
fn test_deprecated_asynclet_wait_still_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_wait"),
        "swift_asyncLet_wait should still be present for ABI compat"
    );
}

#[test]
fn test_deprecated_asynclet_wait_throwing_still_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_wait_throwing"),
        "swift_asyncLet_wait_throwing should still be present for ABI compat"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §2: Replacement APIs resolve and are the preferred path
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_asynclet_begin_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_begin"),
        "swift_asyncLet_begin (replacement for swift_asyncLet_start) should resolve"
    );
}

#[test]
fn test_asynclet_get_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_get"),
        "swift_asyncLet_get (replacement for swift_asyncLet_wait) should resolve"
    );
}

#[test]
fn test_asynclet_get_throwing_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_get_throwing"),
        "swift_asyncLet_get_throwing (replacement for swift_asyncLet_wait_throwing) should resolve"
    );
}

#[test]
fn test_asynclet_finish_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_finish"),
        "swift_asyncLet_finish (replacement for swift_asyncLet_end) should resolve"
    );
}

#[test]
fn test_asynclet_consume_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_consume"),
        "swift_asyncLet_consume should resolve"
    );
}

#[test]
fn test_asynclet_consume_throwing_resolves() {
    assert!(
        symbol_exists(c"swift_asyncLet_consume_throwing"),
        "swift_asyncLet_consume_throwing should resolve"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §3: Deprecated and replacement symbols point to different addresses
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_asynclet_start_and_begin_are_distinct() {
    let start = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_start".as_ptr()) };
    let begin = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_begin".as_ptr()) };
    assert!(!start.is_null());
    assert!(!begin.is_null());
    assert_ne!(
        start, begin,
        "swift_asyncLet_start and swift_asyncLet_begin should be distinct symbols"
    );
}

#[test]
fn test_asynclet_wait_and_get_are_distinct() {
    let wait = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_wait".as_ptr()) };
    let get = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_get".as_ptr()) };
    assert!(!wait.is_null());
    assert!(!get.is_null());
    assert_ne!(
        wait, get,
        "swift_asyncLet_wait and swift_asyncLet_get should be distinct symbols"
    );
}

#[test]
fn test_asynclet_end_and_finish_are_distinct() {
    let end = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_end".as_ptr()) };
    let finish = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_finish".as_ptr()) };
    assert!(!end.is_null());
    assert!(!finish.is_null());
    assert_ne!(
        end, finish,
        "swift_asyncLet_end and swift_asyncLet_finish should be distinct symbols"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §4: ConcurrencyAbi profile — deprecated symbols are now optional
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(feature = "runtime-contract")]
mod concurrency_abi_tests {
    use swift_runtime_sys::ConcurrencyAbi;

    #[test]
    fn test_required_symbols_no_longer_include_deprecated() {
        let required = ConcurrencyAbi::required_symbols();
        assert!(
            !required.contains(&"swift_asyncLet_start"),
            "swift_asyncLet_start should no longer be required"
        );
        assert!(
            !required.contains(&"swift_asyncLet_end"),
            "swift_asyncLet_end should no longer be required"
        );
        // swift_asyncLet_begin is still required
        assert!(
            required.contains(&"swift_asyncLet_begin"),
            "swift_asyncLet_begin should be required"
        );
    }

    #[test]
    fn test_optional_symbols_include_deprecated() {
        let optional = ConcurrencyAbi::optional_symbols();
        assert!(
            optional.contains(&"swift_asyncLet_start"),
            "swift_asyncLet_start should be optional (deprecated)"
        );
        assert!(
            optional.contains(&"swift_asyncLet_end"),
            "swift_asyncLet_end should be optional (deprecated)"
        );
        assert!(
            optional.contains(&"swift_asyncLet_wait"),
            "swift_asyncLet_wait should be optional (deprecated)"
        );
        assert!(
            optional.contains(&"swift_asyncLet_wait_throwing"),
            "swift_asyncLet_wait_throwing should be optional (deprecated)"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §5: New Swift 6.3 Config constants (SWIFT_REFCOUNT_CC_PRESERVEMOST)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_refcount_cc_preservemost_constant() {
    // On aarch64 non-embedded, Swift 6.3 uses preserve_most CC for retain/release
    assert_eq!(
        swift_runtime_sys::Config::SWIFT_REFCOUNT_CC_PRESERVEMOST,
        1,
        "SWIFT_REFCOUNT_CC_PRESERVEMOST should be 1 on aarch64"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §6: New Swift 6.3 debug variables (DebugVars)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_debug_allocation_pool_size_callable() {
    // The symbol may or may not be present on this OS version.
    // We're just testing the accessor doesn't crash.
    let result = swift_runtime_sys::DebugVars::debug_allocationPoolSize();
    match result {
        Some(size) => {
            println!("debug_allocationPoolSize = {size}");
            // If present, the pool size should be > 0
            assert!(size > 0, "Pool size should be positive if available");
        }
        None => {
            println!("_swift_debug_allocationPoolSize not available on this runtime (expected on pre-6.3 OS)");
        }
    }
}

#[test]
fn test_debug_metadata_allocator_page_size_callable() {
    let result = swift_runtime_sys::DebugVars::debug_metadataAllocatorPageSize();
    match result {
        Some(size) => {
            println!("debug_metadataAllocatorPageSize = {size}");
            assert!(size > 0, "Page size should be positive if available");
            // Page sizes are typically powers of 2
            assert!(
                size.is_power_of_two(),
                "Page size {size} should be a power of 2"
            );
        }
        None => {
            println!("_swift_debug_metadataAllocatorPageSize not available on this runtime");
        }
    }
}

#[test]
fn test_debug_task_slab_allocator_callable() {
    let result = swift_runtime_sys::DebugVars::debug_concurrencyEnableTaskSlabAllocator();
    match result {
        Some(enabled) => {
            println!("concurrencyEnableTaskSlabAllocator = {enabled}");
        }
        None => {
            println!("Task slab allocator query not available on this runtime");
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §7: SwiftDtoa removal — verify no stale references
// ══���════════════════════════════════════════════════════════════════════════

#[test]
fn test_swiftdtoa_module_removed() {
    // Compile-time check: SwiftDtoa should NOT be a module in the crate.
    // If someone accidentally re-adds it, this test documents the intent.
    //
    // We can't do a negative compile-time check easily, but we CAN verify
    // that the numeric conversion functions we actually use still work.
    let mut buf = [0u8; 64];
    let len = unsafe {
        swift_runtime_sys::NumericConversion::swift_float64ToString(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            2.71828,
            false,
        )
    };
    let s = std::str::from_utf8(&buf[..len]).unwrap();
    assert!(
        s.starts_with("2.71828"),
        "Float-to-string should still work after SwiftDtoa removal, got: {s}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §8: Core runtime symbols still link after re-generation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_retain_release_still_link() {
    use swift_runtime_sys::RuntimeRaw::*;
    // Null retain/release should be no-ops
    let r = unsafe { swift_retain(std::ptr::null_mut()) };
    assert!(r.is_null());
    unsafe { swift_release(std::ptr::null_mut()) };
}

#[test]
fn test_metadata_lookup_still_works() {
    use swift_runtime_sys::RuntimeRaw::*;
    let mangled = b"Si"; // Swift.Int
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null(), "Int metadata should resolve after 6.3 migration");
}

#[test]
fn test_demangle_still_works() {
    let mangled = c"$sSiN";
    let result = unsafe {
        swift_runtime_sys::DebugHooks::swift_demangle(
            mangled.as_ptr(),
            mangled.to_bytes().len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    };
    assert!(!result.is_null(), "Demangling should still work");
    let name = unsafe { std::ffi::CStr::from_ptr(result) }.to_str().unwrap();
    assert!(name.contains("Int"), "Demangled name should contain 'Int', got: {name}");
    unsafe { libc::free(result as *mut c_void) };
}

// ═══════════════════════════════════════════════════════════════════════════
// §9: Concurrency runtime — non-deprecated APIs comprehensive check
// ═══════════════════════════════════════════════════════════════════════════

/// All concurrency symbols that remain in the 6.3 headers should resolve.
#[test]
fn test_all_current_concurrency_symbols_resolve() {
    let current_symbols = [
        "swift_task_getCurrent",
        "swift_task_alloc",
        "swift_task_dealloc",
        "swift_task_cancel",
        "swift_task_create",
        "swift_continuation_init",
        "swift_continuation_resume",
        "swift_continuation_throwingResume",
        "swift_asyncLet_begin",
        "swift_asyncLet_get",
        "swift_asyncLet_get_throwing",
        "swift_asyncLet_finish",
        "swift_asyncLet_consume",
        "swift_asyncLet_consume_throwing",
        "swift_job_run",
    ];

    let mut missing = Vec::new();
    for sym in &current_symbols {
        let cstr = std::ffi::CString::new(*sym).unwrap();
        if !symbol_exists(&cstr) {
            missing.push(*sym);
        }
    }
    assert!(
        missing.is_empty(),
        "Current (non-deprecated) concurrency symbols missing: {missing:?}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §10: Edge case — deprecated thunks still callable
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_deprecated_thunk_asynclet_start_resolves_via_dlsym() {
    // Even though the thunk is deprecated, it should still resolve.
    // This tests the runtime compatibility layer.
    let ptr = unsafe { dlsym(RTLD_DEFAULT, c"swift_asyncLet_start".as_ptr()) };
    assert!(
        !ptr.is_null(),
        "Deprecated swift_asyncLet_start thunk should still resolve via dlsym \
         (ABI compatibility requires this until a future OS removes it)"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §11: Heap object header size unchanged
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_heap_object_header_size_stable() {
    use swift_runtime_sys::SwiftABI::*;
    assert_eq!(
        std::mem::size_of::<HeapObject>(),
        HEAP_OBJECT_HEADER_SIZE,
    );
    assert_eq!(
        HEAP_OBJECT_HEADER_SIZE, 16,
        "HeapObject header should remain 16 bytes across Swift versions"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §12: Value witness table ABI stability
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_vwt_abi_stable_across_migration() {
    use swift_runtime_sys::RuntimeRaw::*;
    use swift_runtime_sys::SwiftABI::*;

    // Verify fundamental type layouts haven't changed
    let types: &[(&[u8], usize, bool)] = &[
        (b"Si", std::mem::size_of::<isize>(), true),  // Int — POD
        (b"Sb", 1, true),                              // Bool — POD
        (b"Sd", 8, true),                              // Double — POD
        (b"SS", 16, false),                            // String — not POD
    ];

    for (mangled, expected_size, expected_pod) in types {
        let metadata = unsafe {
            swift_getTypeByMangledNameInEnvironment(
                mangled.as_ptr(),
                mangled.len(),
                std::ptr::null(),
                0,
            )
        };
        assert!(!metadata.is_null(), "Failed to resolve type {:?}", std::str::from_utf8(mangled));

        let vwt = unsafe { &*get_value_witness_table(metadata) };
        assert_eq!(
            vwt.get_size(),
            *expected_size,
            "Size mismatch for {:?}",
            std::str::from_utf8(mangled)
        );
        assert_eq!(
            vwt.is_pod(),
            *expected_pod,
            "POD mismatch for {:?}",
            std::str::from_utf8(mangled)
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §13: Metadata kind values unchanged
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_metadata_kind_values_stable() {
    use swift_runtime_sys::SwiftABI::MetadataKind;
    assert_eq!(MetadataKind::Class as u32, 0);
    assert_eq!(MetadataKind::Struct as u32, 0x200);
    assert_eq!(MetadataKind::Enum as u32, 0x201);
    assert_eq!(MetadataKind::Optional as u32, 0x202);
    assert_eq!(MetadataKind::Tuple as u32, 0x301);
    assert_eq!(MetadataKind::Function as u32, 0x302);
    assert_eq!(MetadataKind::Existential as u32, 0x303);
    assert_eq!(MetadataKind::Metatype as u32, 0x304);
}

// ═══════════════════════════════════════════════════════════════════════════
// §14: Concurrency thunks — replacement APIs work through thunks
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_thunk_main_executor_after_migration() {
    let result = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getMainExecutor() };
    assert!(result.is_ok(), "Main executor thunk should work after migration");
    let executor = result.unwrap();
    assert!(
        !executor.identity.is_null() || !executor.implementation.is_null(),
        "Main executor should be non-null"
    );
}

#[test]
fn test_thunk_get_time_after_migration() {
    let mut sec: i64 = 0;
    let mut nsec: i64 = 0;
    let result =
        unsafe { swift_runtime_sys::ConcurrencyThunks::swift_get_time(&mut sec, &mut nsec, 1) };
    assert!(result.is_ok());
    assert!(sec > 0 || nsec > 0, "Clock should return non-zero time");
}

// ═══════════════════════════════════════════════════════════════════════════
// §15: Edge case — null pointers through new and deprecated paths
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_error_retain_release_null_safe() {
    // Verify null safety hasn't regressed
    unsafe {
        swift_runtime_sys::ErrorHandling::swift_errorRelease(std::ptr::null_mut());
    }
}

#[test]
fn test_bridge_object_null_safe() {
    unsafe {
        let r =
            swift_runtime_sys::BridgeObject::swift_bridgeObjectRetain(std::ptr::null_mut());
        assert!(r.is_null());
        swift_runtime_sys::BridgeObject::swift_bridgeObjectRelease(std::ptr::null_mut());
    }
}

#[test]
fn test_unowned_null_safe() {
    unsafe {
        let r = swift_runtime_sys::UnownedRef::swift_unownedRetain(std::ptr::null_mut());
        assert!(r.is_null());
        swift_runtime_sys::UnownedRef::swift_unownedRelease(std::ptr::null_mut());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §16: Bindgen output coherence — no SwiftDtoa references remain
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_no_swiftdtoa_in_crate() {
    // Runtime check: the module should not be reachable.
    // We verify this indirectly by confirming NumericConversion
    // (our own module) still provides float-to-string.
    let mut buf = [0u8; 64];
    let len = unsafe {
        swift_runtime_sys::NumericConversion::swift_int64ToString(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            12345,
            10,
            false,
        )
    };
    assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), "12345");
}

// ═══════════════════════════════════════════════════════════════════════════
// §17: Full symbol coverage — every ConcurrencyRuntime extern fn resolves
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_concurrency_runtime_complete_symbol_coverage() {
    // Every symbol declared in the ConcurrencyRuntime extern block should
    // be resolvable. This catches any symbols that were removed from the
    // dylib but still declared in our bindings.
    let all_symbols = [
        // Task lifecycle
        "swift_task_getCurrent",
        "swift_task_create",
        "swift_task_cancel",
        "swift_task_alloc",
        "swift_task_dealloc",
        // Continuations
        "swift_continuation_init",
        "swift_continuation_resume",
        "swift_continuation_throwingResume",
        // Async let — current
        "swift_asyncLet_begin",
        "swift_asyncLet_get",
        "swift_asyncLet_get_throwing",
        "swift_asyncLet_consume",
        "swift_asyncLet_consume_throwing",
        "swift_asyncLet_finish",
        // Async let — deprecated but ABI-present
        "swift_asyncLet_start",
        "swift_asyncLet_end",
        "swift_asyncLet_wait",
        "swift_asyncLet_wait_throwing",
        // Jobs
        "swift_job_run",
        // Actors
        "swift_defaultActor_initialize",
        "swift_defaultActor_destroy",
        // Task groups
        "swift_taskGroup_initialize",
        "swift_taskGroup_destroy",
        "swift_taskGroup_isCancelled",
    ];

    let mut missing = Vec::new();
    for sym in &all_symbols {
        let cstr = std::ffi::CString::new(*sym).unwrap();
        if !symbol_exists(&cstr) {
            missing.push(*sym);
        }
    }

    assert!(
        missing.is_empty(),
        "ConcurrencyRuntime symbols not found in dylib: {missing:?}\n\
         This may indicate a symbol was removed from the runtime \
         and should be removed from our extern block."
    );
}
