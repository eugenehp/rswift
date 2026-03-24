//! Apple EventKit — calendar and reminders from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 4+, visionOS 1+, watchOS 2+.
//!
//! # Quick start
//!
//! ```ignore
//! let store = eventkit::EventStore::new();
//! println!("Calendar auth: {:?}", eventkit::authorization_status());
//! println!("Calendars: {}", store.calendar_count());
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// EKAuthorizationStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    /// Full access (iOS 17+ / macOS 14+) or legacy authorized.
    FullAccess = 3,
    /// Write-only access (iOS 17+).
    WriteOnly = 4,
}

impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::FullAccess,
            4 => Self::WriteOnly,
            _ => Self::NotDetermined,
        }
    }
}

/// EKEntityType
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Event = 0,
    Reminder = 1,
}

/// Check authorization status for events or reminders.
pub fn authorization_status(entity: EntityType) -> AuthorizationStatus {
    unsafe {
        let sel = sel_registerName(b"authorizationStatusForEntityType:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, isize) -> isize =
            core::mem::transmute(objc_msgSend as *const ());
        AuthorizationStatus::from(f(class!(b"EKEventStore\0") as Id, sel, entity as isize))
    }
}

/// Wraps `EKEventStore`.
pub struct EventStore {
    inner: Id,
}

impl EventStore {
    /// Create a new event store.
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"EKEventStore\0"), new] } }
    }

    /// Default calendar for new events (may be nil).
    pub fn default_calendar_title(&self) -> Option<String> {
        unsafe {
            let cal: Id = msg_send![self.inner, defaultCalendarForNewEvents];
            if cal.is_null() { return None; }
            nsstring_to_string(msg_send![cal, title])
        }
    }

    /// Number of calendars for events.
    pub fn calendar_count(&self) -> usize {
        unsafe {
            // calendarsForEntityType: 0 = events
            let sel = sel_registerName(b"calendarsForEntityType:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(self.inner, sel, 0);
            if arr.is_null() { return 0; }
            msg_send_t![usize; arr, count]
        }
    }

    /// List all calendar titles for events.
    pub fn calendar_titles(&self) -> Vec<String> {
        unsafe {
            let sel = sel_registerName(b"calendarsForEntityType:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(self.inner, sel, 0);
            if arr.is_null() { return vec![]; }
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| {
                    let cal: Id = msg_send![arr, objectAtIndex: i];
                    nsstring_to_string(msg_send![cal, title])
                })
                .collect()
        }
    }

    /// Number of reminder lists.
    pub fn reminder_list_count(&self) -> usize {
        unsafe {
            let sel = sel_registerName(b"calendarsForEntityType:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let arr = f(self.inner, sel, 1); // 1 = reminders
            if arr.is_null() { return 0; }
            msg_send_t![usize; arr, count]
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for EventStore { fn default() -> Self { Self::new() } }
impl Drop for EventStore { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_status() {
        // Should not crash; returns some valid status
        let s = authorization_status(EntityType::Event);
        println!("Event auth: {:?}", s);
    }

    #[test]
    fn test_create_store() {
        let store = EventStore::new();
        let _ = store.default_calendar_title();
    }
}
