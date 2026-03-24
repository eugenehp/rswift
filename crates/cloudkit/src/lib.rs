//! Apple CloudKit — iCloud database from Rust.
//!
//! **Platform:** macOS 10.10+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! ```ignore
//! let container = cloudkit::Container::default();
//! let db = container.public_database();
//! let record = cloudkit::Record::new("Note");
//! record.set_string("title", "Hello from Rust");
//! record.set_double("score", 42.0);
//! db.save(&record); // async via completion handler
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── Account status ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    CouldNotDetermine = 0, Available = 1, Restricted = 2,
    NoAccount = 3, TemporarilyUnavailable = 4,
}
impl From<isize> for AccountStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Available, 2=>Self::Restricted, 3=>Self::NoAccount,
                  4=>Self::TemporarilyUnavailable, _=>Self::CouldNotDetermine }
    }
}

// ── Container ───────────────────────────────────────────────────────────────

/// Wraps `CKContainer`.
pub struct Container { inner: Id }

impl Container {
    pub fn default() -> Self {
        Self { inner: unsafe { msg_send![class!(b"CKContainer\0"), defaultContainer] } }
    }

    pub fn with_identifier(id: &str) -> Self {
        unsafe {
            let ns = nsstring(id);
            let c = msg_send![class!(b"CKContainer\0"), containerWithIdentifier: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: c }
        }
    }

    pub fn identifier(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, containerIdentifier]) }
    }

    pub fn public_database(&self) -> Database {
        Database { inner: unsafe { msg_send![self.inner, publicCloudDatabase] } }
    }
    pub fn private_database(&self) -> Database {
        Database { inner: unsafe { msg_send![self.inner, privateCloudDatabase] } }
    }
    pub fn shared_database(&self) -> Database {
        Database { inner: unsafe { msg_send![self.inner, sharedCloudDatabase] } }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── Database ────────────────────────────────────────────────────────────────

/// Wraps `CKDatabase` (public, private, or shared).
pub struct Database { inner: Id }

impl Database {
    /// Save a record (fire-and-forget — completion handler is nil).
    pub fn save(&self, record: &Record) {
        unsafe {
            msg_send_void![self.inner, saveRecord: record.inner, completionHandler: NIL];
        }
    }

    /// Delete a record by its record ID (fire-and-forget).
    pub fn delete(&self, record_id: &RecordId) {
        unsafe {
            msg_send_void![self.inner, deleteRecordWithID: record_id.inner, completionHandler: NIL];
        }
    }

    /// Fetch a record by ID (fire-and-forget, result via completion).
    pub fn fetch(&self, record_id: &RecordId) {
        unsafe {
            msg_send_void![self.inner, fetchRecordWithID: record_id.inner, completionHandler: NIL];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── RecordId ────────────────────────────────────────────────────────────────

/// Wraps `CKRecord.ID`.
pub struct RecordId { inner: Id }

impl RecordId {
    /// Create a record ID with a unique name.
    pub fn new(name: &str) -> Self {
        unsafe {
            let ns = nsstring(name);
            let rid: Id = msg_send![class!(b"CKRecordID\0"), alloc];
            let rid = msg_send![rid, initWithRecordName: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: rid }
        }
    }

    /// The record name.
    pub fn name(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, recordName]).unwrap_or_default() }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for RecordId {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── Record ──────────────────────────────────────────────────────────────────

/// Wraps `CKRecord`.
pub struct Record { inner: Id }

impl Record {
    /// Create a new record with the given record type name.
    pub fn new(record_type: &str) -> Self {
        unsafe {
            let ns = nsstring(record_type);
            let r: Id = msg_send![class!(b"CKRecord\0"), alloc];
            let r = msg_send![r, initWithRecordType: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: r }
        }
    }

    /// Create a record with a specific ID.
    pub fn with_id(record_type: &str, id: &RecordId) -> Self {
        unsafe {
            let ns = nsstring(record_type);
            let sel = sel_registerName(b"initWithRecordType:recordID:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let r = msg_send![class!(b"CKRecord\0"), alloc];
            let r = f(r, sel, ns, id.inner);
            CFRelease(ns as CFTypeRef);
            Self { inner: r }
        }
    }

    /// The record type name.
    pub fn record_type(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, recordType]).unwrap_or_default() }
    }

    /// The record ID.
    pub fn record_id(&self) -> RecordId {
        let inner = unsafe { msg_send![self.inner, recordID] };
        unsafe { CFRetain(inner as CFTypeRef); }
        RecordId { inner }
    }

    /// Creation date as Unix timestamp.
    pub fn creation_date(&self) -> Option<f64> {
        unsafe {
            let date: Id = msg_send![self.inner, creationDate];
            if date.is_null() { return None; }
            let sel = sel_registerName(b"timeIntervalSince1970\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(date, sel))
        }
    }

    /// Modification date as Unix timestamp.
    pub fn modification_date(&self) -> Option<f64> {
        unsafe {
            let date: Id = msg_send![self.inner, modificationDate];
            if date.is_null() { return None; }
            let sel = sel_registerName(b"timeIntervalSince1970\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(date, sel))
        }
    }

    // ── Field setters ───────────────────────────────────────────────────

    /// Set a string field.
    pub fn set_string(&self, key: &str, value: &str) {
        unsafe {
            let k = nsstring(key);
            let v = nsstring(value);
            msg_send_void![self.inner, setObject: v, forKey: k];
            CFRelease(k as CFTypeRef);
            CFRelease(v as CFTypeRef);
        }
    }

    /// Set a double field.
    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            let k = nsstring(key);
            let sel = sel_registerName(b"numberWithDouble:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let num = f(class!(b"NSNumber\0") as Id, sel, value);
            msg_send_void![self.inner, setObject: num, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    /// Set an integer field.
    pub fn set_int(&self, key: &str, value: i64) {
        unsafe {
            let k = nsstring(key);
            let sel = sel_registerName(b"numberWithLongLong:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let num = f(class!(b"NSNumber\0") as Id, sel, value);
            msg_send_void![self.inner, setObject: num, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    /// Set a data (bytes) field.
    pub fn set_data(&self, key: &str, value: &[u8]) {
        unsafe {
            let k = nsstring(key);
            let d: Id = msg_send![class!(b"NSData\0"), dataWithBytes: value.as_ptr(), length: value.len()];
            msg_send_void![self.inner, setObject: d, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    // ── Field getters ───────────────────────────────────────────────────

    /// Get a string field.
    pub fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, objectForKey: k];
            CFRelease(k as CFTypeRef);
            nsstring_to_string(v)
        }
    }

    /// Get a double field.
    pub fn get_double(&self, key: &str) -> Option<f64> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, objectForKey: k];
            CFRelease(k as CFTypeRef);
            if v.is_null() { return None; }
            let sel = sel_registerName(b"doubleValue\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(v, sel))
        }
    }

    /// Get an integer field.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        unsafe {
            let k = nsstring(key);
            let v: Id = msg_send![self.inner, objectForKey: k];
            CFRelease(k as CFTypeRef);
            if v.is_null() { return None; }
            let sel = sel_registerName(b"longLongValue\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> i64 =
                core::mem::transmute(objc_msgSend as *const ());
            Some(f(v, sel))
        }
    }

    /// List all keys set on this record.
    pub fn all_keys(&self) -> Vec<String> {
        unsafe {
            let arr: Id = msg_send![self.inner, allKeys];
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
                .collect()
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Record {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── RecordZone ──────────────────────────────────────────────────────────────

/// Wraps `CKRecordZone`.
pub struct RecordZone { inner: Id }

impl RecordZone {
    /// The default zone.
    pub fn default() -> Self {
        Self { inner: unsafe { msg_send![class!(b"CKRecordZone\0"), defaultRecordZone] } }
    }

    /// Create a custom zone.
    pub fn new(name: &str) -> Self {
        unsafe {
            let ns = nsstring(name);
            let z: Id = msg_send![class!(b"CKRecordZone\0"), alloc];
            let z = msg_send![z, initWithZoneName: ns];
            CFRelease(ns as CFTypeRef);
            Self { inner: z }
        }
    }

    /// Zone name.
    pub fn name(&self) -> String {
        unsafe {
            let zid: Id = msg_send![self.inner, zoneID];
            nsstring_to_string(msg_send![zid, zoneName]).unwrap_or_default()
        }
    }
}

impl Drop for RecordZone {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_crud() {
        let r = Record::new("TestNote");
        assert_eq!(r.record_type(), "TestNote");

        r.set_string("title", "Hello");
        r.set_double("score", 99.5);
        r.set_int("count", 42);
        r.set_data("payload", b"binary data");

        assert_eq!(r.get_string("title"), Some("Hello".into()));
        assert_eq!(r.get_double("score"), Some(99.5));
        assert_eq!(r.get_int("count"), Some(42));

        let keys = r.all_keys();
        assert!(keys.contains(&"title".into()));
        assert!(keys.contains(&"score".into()));
    }

    #[test]
    fn test_record_id() {
        let rid = RecordId::new("test-uuid-123");
        assert_eq!(rid.name(), "test-uuid-123");
    }

    #[test]
    fn test_record_zone() {
        let z = RecordZone::default();
        assert_eq!(z.name(), "_defaultZone");

        let custom = RecordZone::new("MyZone");
        assert_eq!(custom.name(), "MyZone");
    }

    #[test]
    #[ignore] // Requires CloudKit entitlement
    fn test_container() {
        let c = Container::default();
        let _ = c.identifier();
    }
}
