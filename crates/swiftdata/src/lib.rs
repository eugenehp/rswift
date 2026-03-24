//! Apple SwiftData — persistent models from Rust.
//!
//! **Platform:** macOS 14+, iOS 17+, tvOS 17+, watchOS 10+.
//!
//! SwiftData's `@Model` macro and `ModelContainer`/`ModelContext` are
//! Swift-only. This crate bridges the core lifecycle: create containers,
//! contexts, save, rollback.
//!
//! ```ignore
//! if swiftdata::is_available() {
//!     let container = swiftdata::ModelContainer::in_memory().unwrap();
//!     let ctx = container.new_context();
//!     // insert models via Swift bridge...
//!     ctx.save().unwrap();
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

unsafe extern "C" { fn swiftdata_swift_available() -> bool; }
pub fn is_available() -> bool { unsafe { swiftdata_swift_available() } }

unsafe extern "C" {
    fn swiftdata_container_create(
        schemas: *const u8, schemas_len: usize,
        in_memory: bool,
        url: *const u8, url_len: usize,
    ) -> *mut c_void;
    fn swiftdata_container_release(ptr: *mut c_void);
    fn swiftdata_context_create(container: *mut c_void) -> *mut c_void;
    fn swiftdata_context_save(ctx: *mut c_void) -> bool;
    fn swiftdata_context_has_changes(ctx: *mut c_void) -> bool;
    fn swiftdata_context_rollback(ctx: *mut c_void);
    fn swiftdata_context_release(ctx: *mut c_void);
}

/// A SwiftData model container (wraps `ModelContainer`).
pub struct ModelContainer { ptr: *mut c_void }

impl ModelContainer {
    /// Create an in-memory container (no persistence).
    pub fn in_memory() -> Option<Self> {
        let ptr = unsafe {
            swiftdata_container_create(
                b"".as_ptr(), 0, true, b"".as_ptr(), 0,
            )
        };
        if ptr.is_null() { None } else { Some(Self { ptr }) }
    }

    /// Create a persistent container at a file URL.
    pub fn at_url(url: &str) -> Option<Self> {
        let ptr = unsafe {
            swiftdata_container_create(
                b"".as_ptr(), 0, false, url.as_ptr(), url.len(),
            )
        };
        if ptr.is_null() { None } else { Some(Self { ptr }) }
    }

    /// Create a new `ModelContext` for this container.
    pub fn new_context(&self) -> ModelContext {
        let ptr = unsafe { swiftdata_context_create(self.ptr) };
        ModelContext { ptr }
    }

    pub fn as_ptr(&self) -> *mut c_void { self.ptr }
}

impl Drop for ModelContainer {
    fn drop(&mut self) { unsafe { swiftdata_container_release(self.ptr); } }
}

/// A SwiftData model context (wraps `ModelContext`).
pub struct ModelContext { ptr: *mut c_void }

impl ModelContext {
    /// Save pending changes.
    pub fn save(&self) -> Result<(), &'static str> {
        if unsafe { swiftdata_context_save(self.ptr) } { Ok(()) }
        else { Err("SwiftData save failed") }
    }

    /// Whether there are unsaved changes.
    pub fn has_changes(&self) -> bool {
        unsafe { swiftdata_context_has_changes(self.ptr) }
    }

    /// Rollback unsaved changes.
    pub fn rollback(&self) {
        unsafe { swiftdata_context_rollback(self.ptr); }
    }

    pub fn as_ptr(&self) -> *mut c_void { self.ptr }
}

impl Drop for ModelContext {
    fn drop(&mut self) { unsafe { swiftdata_context_release(self.ptr); } }
}
