//! Apple Core Data — object graph persistence from Rust.
//!
//! **Platform:** macOS 10.4+, iOS 3+, tvOS 9+, watchOS 2+.
//!
//! ```ignore
//! let container = coredata::PersistentContainer::new("MyModel");
//! container.load_stores();
//! let ctx = container.view_context();
//! let entity = ctx.insert_new("Person");
//! entity.set_string("name", "Alice");
//! entity.set_int("age", 30);
//! ctx.save().unwrap();
//! let people = ctx.fetch("Person");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── PersistentContainer ─────────────────────────────────────────────────────

/// Wraps `NSPersistentContainer`.
pub struct PersistentContainer { inner: Id }

impl PersistentContainer {
    /// Create a container with the given data model name.
    pub fn new(name: &str) -> Self {
        unsafe {
            let ns = nsstring(name);
            let c = msg_send![class!(b"NSPersistentContainer\0"), persistentContainerWithName: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(c as CFTypeRef);
            Self { inner: c }
        }
    }

    /// Load the persistent stores (fire-and-forget).
    pub fn load_stores(&self) {
        unsafe {
            msg_send_void![self.inner, loadPersistentStoresWithCompletionHandler: NIL];
        }
    }

    /// The view context (main-queue managed object context).
    pub fn view_context(&self) -> ManagedObjectContext {
        let inner = unsafe { msg_send![self.inner, viewContext] };
        ManagedObjectContext { inner, owned: false }
    }

    /// Create a new background context.
    pub fn new_background_context(&self) -> ManagedObjectContext {
        let inner = unsafe { msg_send![self.inner, newBackgroundContext] };
        ManagedObjectContext { inner, owned: true }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for PersistentContainer {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── ManagedObjectContext ────────────────────────────────────────────────────

/// Wraps `NSManagedObjectContext`.
pub struct ManagedObjectContext { inner: Id, owned: bool }

impl ManagedObjectContext {
    /// Insert a new entity by name. Returns a managed object.
    pub fn insert_new(&self, entity_name: &str) -> ManagedObject {
        unsafe {
            let ns = nsstring(entity_name);
            let sel = sel_registerName(b"insertNewObjectForEntityForName:inManagedObjectContext:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let obj = f(class!(b"NSEntityDescription\0") as Id, sel, ns, self.inner);
            CFRelease(ns as CFTypeRef);
            ManagedObject { inner: obj }
        }
    }

    /// Save the context.
    pub fn save(&self) -> Result<(), String> {
        unsafe {
            let mut error: Id = NIL;
            let sel = sel_registerName(b"save:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            if f(self.inner, sel, &mut error) {
                Ok(())
            } else if !error.is_null() {
                let desc = nsstring_to_string(msg_send![error, localizedDescription]);
                Err(desc.unwrap_or_else(|| "Unknown error".into()))
            } else {
                Err("Save failed".into())
            }
        }
    }

    /// Check if the context has unsaved changes.
    pub fn has_changes(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, hasChanges] }
    }

    /// Rollback unsaved changes.
    pub fn rollback(&self) {
        unsafe { msg_send_void![self.inner, rollback]; }
    }

    /// Reset the context (discard all objects).
    pub fn reset(&self) {
        unsafe { msg_send_void![self.inner, reset]; }
    }

    /// Execute a fetch request and return results.
    pub fn fetch(&self, entity_name: &str) -> Vec<ManagedObject> {
        self.fetch_with_predicate(entity_name, None, None)
    }

    /// Fetch with optional predicate and sort key.
    pub fn fetch_with_predicate(
        &self,
        entity_name: &str,
        predicate_format: Option<&str>,
        sort_key: Option<&str>,
    ) -> Vec<ManagedObject> {
        unsafe {
            let ns = nsstring(entity_name);
            let req: Id = msg_send![class!(b"NSFetchRequest\0"), fetchRequestWithEntityName: ns];
            CFRelease(ns as CFTypeRef);

            // Set predicate if provided
            if let Some(fmt) = predicate_format {
                let fmt_ns = nsstring(fmt);
                let pred: Id = msg_send![class!(b"NSPredicate\0"), predicateWithFormat: fmt_ns];
                msg_send_void![req, setPredicate: pred];
                CFRelease(fmt_ns as CFTypeRef);
            }

            // Set sort descriptor if provided
            if let Some(key) = sort_key {
                let key_ns = nsstring(key);
                let sd: Id = msg_send![class!(b"NSSortDescriptor\0"), alloc];
                let sd = msg_send![sd, initWithKey: key_ns, ascending: 1u8];
                let arr = msg_send![class!(b"NSArray\0"), arrayWithObject: sd];
                msg_send_void![req, setSortDescriptors: arr];
                CFRelease(key_ns as CFTypeRef);
            }

            // Execute
            let results: Id = msg_send![self.inner, executeFetchRequest: req, error: NIL];
            if results.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; results, count];
            (0..count)
                .map(|i| ManagedObject { inner: msg_send![results, objectAtIndex: i] })
                .collect()
        }
    }

    /// Count of entities matching a fetch request.
    pub fn count(&self, entity_name: &str) -> usize {
        unsafe {
            let ns = nsstring(entity_name);
            let req: Id = msg_send![class!(b"NSFetchRequest\0"), fetchRequestWithEntityName: ns];
            CFRelease(ns as CFTypeRef);
            let sel = sel_registerName(b"countForFetchRequest:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id) -> usize =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, req, NIL)
        }
    }

    /// Delete a managed object.
    pub fn delete(&self, object: &ManagedObject) {
        unsafe { msg_send_void![self.inner, deleteObject: object.inner]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for ManagedObjectContext {
    fn drop(&mut self) {
        if self.owned { unsafe { CFRelease(self.inner as CFTypeRef); } }
    }
}

// ── ManagedObject ───────────────────────────────────────────────────────────

/// Wraps `NSManagedObject`.
pub struct ManagedObject { inner: Id }

impl ManagedObject {
    /// Entity name.
    pub fn entity_name(&self) -> String {
        unsafe {
            let entity: Id = msg_send![self.inner, entity];
            nsstring_to_string(msg_send![entity, name]).unwrap_or_default()
        }
    }

    /// Set a string value for a key.
    pub fn set_string(&self, key: &str, value: &str) {
        unsafe {
            let k = nsstring(key);
            let v = nsstring(value);
            msg_send_void![self.inner, setValue: v, forKey: k];
            CFRelease(k as CFTypeRef);
            CFRelease(v as CFTypeRef);
        }
    }

    /// Get a string value for a key.
    pub fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, valueForKey: k];
            CFRelease(k as CFTypeRef);
            nsstring_to_string(v)
        }
    }

    /// Set an integer value.
    pub fn set_int(&self, key: &str, value: i64) {
        unsafe {
            let k = nsstring(key);
            let sel = sel_registerName(b"numberWithLongLong:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let num = f(class!(b"NSNumber\0") as Id, sel, value);
            msg_send_void![self.inner, setValue: num, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    /// Get an integer value.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, valueForKey: k];
            CFRelease(k as CFTypeRef);
            if v.is_null() { return None; }
            let sel = sel_registerName(b"longLongValue\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(v, sel))
        }
    }

    /// Set a double value.
    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            let k = nsstring(key);
            let sel = sel_registerName(b"numberWithDouble:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let num = f(class!(b"NSNumber\0") as Id, sel, value);
            msg_send_void![self.inner, setValue: num, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    /// Get a double value.
    pub fn get_double(&self, key: &str) -> Option<f64> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, valueForKey: k];
            CFRelease(k as CFTypeRef);
            if v.is_null() { return None; }
            let sel = sel_registerName(b"doubleValue\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(v, sel))
        }
    }

    /// Set a bool value.
    pub fn set_bool(&self, key: &str, value: bool) {
        self.set_int(key, value as i64);
    }

    /// Get a bool value.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get_int(key).map(|v| v != 0)
    }

    /// Whether this object has been deleted from its context.
    pub fn is_deleted(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isDeleted] }
    }

    /// Whether this object has unsaved changes.
    pub fn has_changes(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, hasChanges] }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}
