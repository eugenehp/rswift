//! Apple Security — Keychain access and secure random from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! # Quick start
//!
//! ```ignore
//! // Store a password
//! security::Keychain::set("com.myapp", "user@email.com", b"s3cret").unwrap();
//!
//! // Read it back
//! let password = security::Keychain::get("com.myapp", "user@email.com").unwrap();
//! assert_eq!(password, b"s3cret");
//!
//! // Secure random bytes
//! let bytes = security::random_bytes(32).unwrap();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"security_available");

unsafe extern "C" {
    fn security_keychain_set(s: *const u8, sl: usize, a: *const u8, al: usize, d: *const u8, dl: usize) -> i32;
    fn security_keychain_get(s: *const u8, sl: usize, a: *const u8, al: usize, buf: *mut u8, bl: usize) -> isize;
    fn security_keychain_delete(s: *const u8, sl: usize, a: *const u8, al: usize) -> i32;
    fn security_keychain_exists(s: *const u8, sl: usize, a: *const u8, al: usize) -> bool;
    fn security_random_bytes(buf: *mut u8, count: usize) -> i32;
}

/// Keychain access for storing secrets (wraps `SecItem*`).
pub struct Keychain;

impl Keychain {
    /// Store data in the Keychain. Overwrites existing entries.
    ///
    /// Returns `Ok(())` on success, `Err(status)` on failure.
    pub fn set(service: &str, account: &str, data: &[u8]) -> Result<(), i32> {
        let s = unsafe {
            security_keychain_set(
                service.as_ptr(), service.len(),
                account.as_ptr(), account.len(),
                data.as_ptr(), data.len(),
            )
        };
        if s == 0 { Ok(()) } else { Err(s) }
    }

    /// Read data from the Keychain.
    pub fn get(service: &str, account: &str) -> Option<Vec<u8>> {
        let mut buf = vec![0u8; 8192];
        let len = unsafe {
            security_keychain_get(
                service.as_ptr(), service.len(),
                account.as_ptr(), account.len(),
                buf.as_mut_ptr(), buf.len(),
            )
        };
        if len < 0 { None } else { buf.truncate(len as usize); Some(buf) }
    }

    /// Read a string from the Keychain.
    pub fn get_string(service: &str, account: &str) -> Option<String> {
        Self::get(service, account).and_then(|b| String::from_utf8(b).ok())
    }

    /// Delete an entry from the Keychain.
    pub fn delete(service: &str, account: &str) -> Result<(), i32> {
        let s = unsafe {
            security_keychain_delete(service.as_ptr(), service.len(), account.as_ptr(), account.len())
        };
        if s == 0 { Ok(()) } else { Err(s) }
    }

    /// Check if an entry exists in the Keychain.
    pub fn exists(service: &str, account: &str) -> bool {
        unsafe { security_keychain_exists(service.as_ptr(), service.len(), account.as_ptr(), account.len()) }
    }
}

/// Generate cryptographically secure random bytes.
pub fn random_bytes(count: usize) -> Result<Vec<u8>, i32> {
    let mut buf = vec![0u8; count];
    let s = unsafe { security_random_bytes(buf.as_mut_ptr(), count) };
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
