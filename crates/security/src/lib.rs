#![allow(unsafe_op_in_unsafe_fn)]
//! Apple Security — Keychain access and secure random from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! Links Security framework directly — no Swift bridge needed.
//! SecItem* and SecRandomCopyBytes are pure C APIs.
//!
//! # Quick start
//!
//! ```ignore
//! security::Keychain::set("com.myapp", "user@email.com", b"s3cret").unwrap();
//! let password = security::Keychain::get("com.myapp", "user@email.com").unwrap();
//! assert_eq!(password, b"s3cret");
//!
//! let bytes = security::random_bytes(32).unwrap();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

/// Security is always available on Apple platforms.
/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── CoreFoundation types (opaque) ───────────────────────────────────────────
type CFTypeRef = *const c_void;
type CFDictionaryRef = *const c_void;
type CFStringRef = *const c_void;
type CFDataRef = *const c_void;
type CFBooleanRef = *const c_void;
type CFAllocatorRef = *const c_void;
type OSStatus = i32;

#[allow(non_snake_case, non_upper_case_globals)]
unsafe extern "C" {
    // CoreFoundation
    fn CFRelease(cf: CFTypeRef);
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef, bytes: *const u8, num_bytes: isize,
        encoding: u32, is_external: bool,
    ) -> CFStringRef;
    fn CFDataCreate(alloc: CFAllocatorRef, bytes: *const u8, length: isize) -> CFDataRef;
    fn CFDataGetLength(data: CFDataRef) -> isize;
    fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
    fn CFDictionaryCreate(
        alloc: CFAllocatorRef,
        keys: *const CFTypeRef, values: *const CFTypeRef, num_values: isize,
        key_callbacks: *const c_void, value_callbacks: *const c_void,
    ) -> CFDictionaryRef;

    // kCFTypeDictionaryKeyCallBacks / kCFTypeDictionaryValueCallBacks
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;
    static kCFBooleanTrue: CFBooleanRef;
    static kCFBooleanFalse: CFBooleanRef;

    // Security
    static kSecClass: CFStringRef;
    static kSecClassGenericPassword: CFStringRef;
    static kSecAttrService: CFStringRef;
    static kSecAttrAccount: CFStringRef;
    static kSecValueData: CFStringRef;
    static kSecReturnData: CFStringRef;
    static kSecMatchLimit: CFStringRef;
    static kSecMatchLimitOne: CFStringRef;

    fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;
    fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    fn SecRandomCopyBytes(rnd: CFTypeRef, count: usize, bytes: *mut u8) -> i32;
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;
const ERR_SEC_SUCCESS: OSStatus = 0;

// ── Internal helpers ────────────────────────────────────────────────────────

unsafe fn cf_string(s: &str) -> CFStringRef {
    CFStringCreateWithBytes(
        core::ptr::null(), s.as_ptr(), s.len() as isize,
        K_CF_STRING_ENCODING_UTF8, false,
    )
}

unsafe fn cf_data(d: &[u8]) -> CFDataRef {
    CFDataCreate(core::ptr::null(), d.as_ptr(), d.len() as isize)
}

unsafe fn make_dict(keys: &[CFTypeRef], vals: &[CFTypeRef]) -> CFDictionaryRef {
    CFDictionaryCreate(
        core::ptr::null(),
        keys.as_ptr(), vals.as_ptr(), keys.len() as isize,
        &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
        &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
    )
}

/// Keychain access for storing secrets (wraps `SecItem*`).
pub struct Keychain;

impl Keychain {
    /// Store data in the Keychain. Overwrites existing entries.
    pub fn set(service: &str, account: &str, data: &[u8]) -> Result<(), i32> {
        unsafe {
            let svc = cf_string(service);
            let acct = cf_string(account);
            let val = cf_data(data);

            // Delete existing
            let del_keys = [kSecClass, kSecAttrService, kSecAttrAccount];
            let del_vals = [kSecClassGenericPassword, svc as CFTypeRef, acct as CFTypeRef];
            let del_q = make_dict(&del_keys, &del_vals);
            SecItemDelete(del_q);
            CFRelease(del_q);

            // Add new
            let add_keys = [kSecClass, kSecAttrService, kSecAttrAccount, kSecValueData];
            let add_vals = [kSecClassGenericPassword, svc as CFTypeRef, acct as CFTypeRef, val as CFTypeRef];
            let add_q = make_dict(&add_keys, &add_vals);
            let status = SecItemAdd(add_q, core::ptr::null_mut());

            CFRelease(add_q);
            CFRelease(val);
            CFRelease(acct);
            CFRelease(svc);

            if status == ERR_SEC_SUCCESS { Ok(()) } else { Err(status) }
        }
    }

    /// Read data from the Keychain.
    pub fn get(service: &str, account: &str) -> Option<Vec<u8>> {
        unsafe {
            let svc = cf_string(service);
            let acct = cf_string(account);

            let keys = [kSecClass, kSecAttrService, kSecAttrAccount, kSecReturnData, kSecMatchLimit];
            let vals = [kSecClassGenericPassword, svc as CFTypeRef, acct as CFTypeRef,
                        kCFBooleanTrue as CFTypeRef, kSecMatchLimitOne as CFTypeRef];
            let query = make_dict(&keys, &vals);

            let mut result: CFTypeRef = core::ptr::null();
            let status = SecItemCopyMatching(query, &mut result);

            CFRelease(query);
            CFRelease(acct);
            CFRelease(svc);

            if status != ERR_SEC_SUCCESS || result.is_null() { return None; }

            let data_ref = result as CFDataRef;
            let len = CFDataGetLength(data_ref) as usize;
            let ptr = CFDataGetBytePtr(data_ref);
            let bytes = core::slice::from_raw_parts(ptr, len).to_vec();
            CFRelease(result);
            Some(bytes)
        }
    }

    /// Read a string from the Keychain.
    pub fn get_string(service: &str, account: &str) -> Option<String> {
        Self::get(service, account).and_then(|b| String::from_utf8(b).ok())
    }

    /// Delete an entry from the Keychain.
    pub fn delete(service: &str, account: &str) -> Result<(), i32> {
        unsafe {
            let svc = cf_string(service);
            let acct = cf_string(account);

            let keys = [kSecClass, kSecAttrService, kSecAttrAccount];
            let vals = [kSecClassGenericPassword, svc as CFTypeRef, acct as CFTypeRef];
            let query = make_dict(&keys, &vals);
            let status = SecItemDelete(query);

            CFRelease(query);
            CFRelease(acct);
            CFRelease(svc);

            if status == ERR_SEC_SUCCESS { Ok(()) } else { Err(status) }
        }
    }

    /// Check if an entry exists in the Keychain.
    pub fn exists(service: &str, account: &str) -> bool {
        unsafe {
            let svc = cf_string(service);
            let acct = cf_string(account);

            let keys = [kSecClass, kSecAttrService, kSecAttrAccount, kSecReturnData];
            let vals = [kSecClassGenericPassword, svc as CFTypeRef, acct as CFTypeRef,
                        kCFBooleanFalse as CFTypeRef];
            let query = make_dict(&keys, &vals);
            let status = SecItemCopyMatching(query, core::ptr::null_mut());

            CFRelease(query);
            CFRelease(acct);
            CFRelease(svc);

            status == ERR_SEC_SUCCESS
        }
    }
}

/// Generate cryptographically secure random bytes.
pub fn random_bytes(count: usize) -> Result<Vec<u8>, i32> {
    let mut buf = vec![0u8; count];
    // kSecRandomDefault = NULL
    let s = unsafe { SecRandomCopyBytes(core::ptr::null(), count, buf.as_mut_ptr()) };
    if s == 0 { Ok(buf) } else { Err(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes_length() {
        let b = random_bytes(32).unwrap();
        assert_eq!(b.len(), 32);
    }

    #[test]
    fn test_random_bytes_unique() {
        let a = random_bytes(16).unwrap();
        let b = random_bytes(16).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn test_random_bytes_zero() {
        let b = random_bytes(0).unwrap();
        assert_eq!(b.len(), 0);
    }
}
