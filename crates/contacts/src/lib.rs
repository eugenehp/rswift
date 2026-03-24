//! Apple Contacts — address book access from Rust.
//!
//! **Platform:** macOS 10.11+, iOS 9+, watchOS 2+.
//!
//! ```ignore
//! println!("Auth: {:?}", contacts::authorization_status());
//! let all = contacts::fetch_all();
//! for c in &all { println!("{} {}", c.given_name, c.family_name); }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationStatus { NotDetermined = 0, Restricted = 1, Denied = 2, Authorized = 3 }
impl From<isize> for AuthorizationStatus {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Restricted, 2=>Self::Denied, 3=>Self::Authorized, _=>Self::NotDetermined }
    }
}

pub fn authorization_status() -> AuthorizationStatus {
    unsafe {
        let sel = sel_registerName(b"authorizationStatusForEntityType:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, isize) -> isize =
            core::mem::transmute(objc_msgSend as *const ());
        AuthorizationStatus::from(f(class!(b"CNContactStore\0") as Id, sel, 0))
    }
}

/// A contact record.
#[derive(Debug, Clone, Default)]
pub struct Contact {
    pub given_name: String,
    pub family_name: String,
    pub organization: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// Fetch all contacts (requires authorization).
pub fn fetch_all() -> Vec<Contact> {
    unsafe {
        let store: Id = msg_send![class!(b"CNContactStore\0"), new];

        // Keys to fetch
        let keys_raw = [
            nsstring("givenName"),
            nsstring("familyName"),
            nsstring("organizationName"),
            nsstring("emailAddresses"),
            nsstring("phoneNumbers"),
        ];
        let sel_arr = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
        let f_arr: unsafe extern "C" fn(Id, Sel, *const Id, usize) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let keys = f_arr(class!(b"NSArray\0") as Id, sel_arr, keys_raw.as_ptr(), keys_raw.len());

        // Fetch request
        let request: Id = msg_send![class!(b"CNContactFetchRequest\0"), alloc];
        let request = msg_send![request, initWithKeysToFetch: keys];

        // We can't easily use enumerateContactsWithFetchRequest: (needs blocks).
        // Use predicate-based fetch instead.
        let container_id: Id = msg_send![store, defaultContainerIdentifier];
        if container_id.is_null() {
            for k in &keys_raw { CFRelease(*k as CFTypeRef); }
            CFRelease(store as CFTypeRef);
            return vec![];
        }

        let pred: Id = msg_send![class!(b"CNContact\0"),
            predicateForContactsInContainerWithIdentifier: container_id];
        let contacts: Id = msg_send![store,
            unifiedContactsMatchingPredicate: pred, keysToFetch: keys, error: NIL];

        let mut result = Vec::new();
        if !contacts.is_null() {
            let count: usize = msg_send_t![usize; contacts, count];
            for i in 0..count {
                let c: Id = msg_send![contacts, objectAtIndex: i];
                let given = nsstring_to_string(msg_send![c, givenName]).unwrap_or_default();
                let family = nsstring_to_string(msg_send![c, familyName]).unwrap_or_default();
                let org = nsstring_to_string(msg_send![c, organizationName]).unwrap_or_default();

                // First email
                let emails: Id = msg_send![c, emailAddresses];
                let email = if !emails.is_null() && msg_send_t![usize; emails, count] > 0 {
                    let lv: Id = msg_send![emails, objectAtIndex: 0usize];
                    nsstring_to_string(msg_send![lv, value])
                } else { None };

                // First phone
                let phones: Id = msg_send![c, phoneNumbers];
                let phone = if !phones.is_null() && msg_send_t![usize; phones, count] > 0 {
                    let lv: Id = msg_send![phones, objectAtIndex: 0usize];
                    let pn: Id = msg_send![lv, value];
                    nsstring_to_string(msg_send![pn, stringValue])
                } else { None };

                result.push(Contact { given_name: given, family_name: family, organization: org, email, phone });
            }
        }

        for k in &keys_raw { CFRelease(*k as CFTypeRef); }
        let _ = request;
        CFRelease(store as CFTypeRef);
        result
    }
}

/// Count contacts in the default container.
pub fn count() -> isize {
    unsafe {
        let store: Id = msg_send![class!(b"CNContactStore\0"), new];
        let key = nsstring("givenName");
        let keys = msg_send![class!(b"NSArray\0"), arrayWithObject: key];
        CFRelease(key as CFTypeRef);

        let container_id: Id = msg_send![store, defaultContainerIdentifier];
        if container_id.is_null() { CFRelease(store as CFTypeRef); return -1; }

        let pred: Id = msg_send![class!(b"CNContact\0"),
            predicateForContactsInContainerWithIdentifier: container_id];
        let contacts: Id = msg_send![store,
            unifiedContactsMatchingPredicate: pred, keysToFetch: keys, error: NIL];
        let c = if contacts.is_null() { -1 } else { msg_send_t![isize; contacts, count] };
        CFRelease(store as CFTypeRef);
        c
    }
}
