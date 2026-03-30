//! Tests for pure-Rust SwiftUI view construction.

use swiftui_native::views;

#[test]
fn test_text_creates_valid_existential() {
    let view = views::text("Hello from pure Rust!");
    assert!(!view.metadata().is_null());
    assert!(!view.witness_table().is_null());
    assert!(!view.value_ptr().is_null());
}

#[test]
fn test_empty_view() {
    let view = views::empty_view();
    assert!(!view.metadata().is_null());
    assert!(!view.witness_table().is_null());
}

#[test]
fn test_divider() {
    let view = views::divider();
    assert!(!view.metadata().is_null());
    assert!(!view.witness_table().is_null());
}

#[test]
fn test_system_image() {
    let view = views::system_image("star.fill");
    assert!(!view.metadata().is_null());
    assert!(!view.witness_table().is_null());
}

#[test]
fn test_text_clone() {
    let v1 = views::text("clone me");
    let v2 = v1.clone();
    assert_eq!(v1.metadata(), v2.metadata());
    assert_eq!(v1.witness_table(), v2.witness_table());
}

#[test]
fn test_text_metadata_is_text() {
    let view = views::text("check type");
    let name = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeName(view.metadata(), true)
    };
    assert!(name.is_ok());
    let (name, _) = name.unwrap();
    assert!(
        name.contains("Text"),
        "Expected Text metadata, got: {name}"
    );
}

#[test]
fn test_divider_metadata_is_divider() {
    let view = views::divider();
    let name = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeName(view.metadata(), true)
    };
    assert!(name.is_ok());
    let (name, _) = name.unwrap();
    assert!(
        name.contains("Divider"),
        "Expected Divider metadata, got: {name}"
    );
}

#[test]
fn test_image_metadata_is_image() {
    let view = views::system_image("gear");
    let name = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeName(view.metadata(), true)
    };
    assert!(name.is_ok());
    let (name, _) = name.unwrap();
    assert!(
        name.contains("Image"),
        "Expected Image metadata, got: {name}"
    );
}

#[test]
fn test_many_texts_no_leak() {
    for i in 0..500 {
        let v = views::text(&format!("item {i}"));
        drop(v);
    }
}

#[test]
fn test_witness_table_matches_conformance() {
    // Verify the cached WT matches what swift_conformsToProtocol returns
    use core::ffi::{c_char, c_void};
    unsafe extern "C" {
        fn dlsym(h: *mut c_void, s: *const c_char) -> *mut c_void;
    }
    let conforms_fn = unsafe {
        dlsym(
            -2isize as *mut c_void,
            c"swift_conformsToProtocol".as_ptr(),
        )
    };
    if conforms_fn.is_null() {
        return;
    }
    type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    let conforms: ConformsFn = unsafe { core::mem::transmute(conforms_fn) };

    let view = views::text("test");
    let proto = swiftui_native::resolve::view_protocol();
    let runtime_wt = unsafe { conforms(view.metadata(), proto) };
    assert_eq!(
        runtime_wt, view.witness_table(),
        "Cached WT should match runtime conformance lookup"
    );
}
