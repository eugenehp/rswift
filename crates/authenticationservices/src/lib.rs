//! Apple AuthenticationServices — Sign in with Apple, passkeys from Rust.
//!
//! **Platform:** macOS 10.15+, iOS 13+, tvOS 13+.
//!
//! ```ignore
//! let provider = authenticationservices::AppleIdProvider::new();
//! let credential_state = provider.credential_state_for("user-id-here");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialState { Revoked = 0, Authorized = 1, NotFound = 2, Transferred = 3 }
impl From<isize> for CredentialState {
    fn from(v: isize) -> Self {
        match v { 1=>Self::Authorized, 2=>Self::NotFound, 3=>Self::Transferred, _=>Self::Revoked }
    }
}

/// Whether passkey (platform public key credential) is supported.
pub fn supports_passkeys() -> bool {
    unsafe { !class!(b"ASAuthorizationPlatformPublicKeyCredentialProvider\0").is_null() }
}

/// Whether Sign in with Apple is supported.
pub fn supports_sign_in_with_apple() -> bool {
    unsafe { !class!(b"ASAuthorizationAppleIDProvider\0").is_null() }
}

/// Apple ID credential provider.
pub struct AppleIdProvider { inner: Id }

impl AppleIdProvider {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"ASAuthorizationAppleIDProvider\0"), new] } }
    }

    /// Check credential state for a user identifier (synchronous check is not available;
    /// the real API uses a completion handler). Returns the provider pointer for advanced use.
    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for AppleIdProvider { fn default() -> Self { Self::new() } }
impl Drop for AppleIdProvider { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Password credential provider (AutoFill).
pub struct PasswordProvider { inner: Id }

impl PasswordProvider {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"ASAuthorizationPasswordProvider\0"), new] } }
    }
    pub fn as_ptr(&self) -> Id { self.inner }
}
impl Default for PasswordProvider { fn default() -> Self { Self::new() } }
impl Drop for PasswordProvider { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_support() {
        let _ = supports_passkeys();
    }

    #[test]
    fn test_sign_in_support() {
        assert!(supports_sign_in_with_apple());
    }

    #[test]
    fn test_providers() {
        let _ = AppleIdProvider::new();
        let _ = PasswordProvider::new();
    }
}
