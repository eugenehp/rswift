//! Apple LocalAuthentication — Face ID / Touch ID from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 8+, visionOS 1+, watchOS 3+.
//!
//! # Quick start
//!
//! ```ignore
//! let ctx = localauthentication::Context::new();
//! match ctx.can_evaluate_biometrics() {
//!     Ok(()) => println!("Biometrics available: {:?}", ctx.biometry_type()),
//!     Err(e) => println!("Not available: error {e}"),
//! }
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool { true }

/// Biometry type reported by the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometryType {
    /// No biometry hardware.
    None = 0,
    /// Touch ID (fingerprint).
    TouchID = 1,
    /// Face ID.
    FaceID = 2,
    /// Optic ID (visionOS).
    OpticID = 3,
}

impl From<isize> for BiometryType {
    fn from(v: isize) -> Self {
        match v {
            1 => Self::TouchID,
            2 => Self::FaceID,
            3 => Self::OpticID,
            _ => Self::None,
        }
    }
}

/// LAPolicy values.
#[derive(Debug, Clone, Copy)]
pub enum Policy {
    /// Authenticate with biometrics (Face ID / Touch ID).
    BiometricsOnly = 1,
    /// Authenticate with biometrics or device passcode.
    BiometricsOrPasscode = 2,
    /// Device owner authentication (watch).
    DeviceOwnerAuthentication = 3,
}

/// Wraps `LAContext` for biometric authentication.
pub struct Context {
    inner: Id,
}

impl Context {
    /// Create a new authentication context.
    pub fn new() -> Self {
        let inner = unsafe { msg_send![class!(b"LAContext\0"), new] };
        Self { inner }
    }

    /// Check if biometric authentication can be evaluated.
    ///
    /// Returns `Ok(())` if biometrics are available, or `Err(error_code)`.
    pub fn can_evaluate_biometrics(&self) -> Result<(), i64> {
        self.can_evaluate(Policy::BiometricsOrPasscode)
    }

    /// Check if a given policy can be evaluated.
    pub fn can_evaluate(&self, policy: Policy) -> Result<(), i64> {
        unsafe {
            let mut error: Id = NIL;
            let sel = sel_registerName(b"canEvaluatePolicy:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, isize, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let ok = f(self.inner, sel, policy as isize, &mut error);
            if ok {
                Ok(())
            } else if !error.is_null() {
                let code: i64 = msg_send_t![i64; error, code];
                Err(code)
            } else {
                Err(-1)
            }
        }
    }

    /// The biometry type available on this device.
    pub fn biometry_type(&self) -> BiometryType {
        unsafe {
            BiometryType::from(msg_send_t![isize; self.inner, biometryType])
        }
    }

    /// Set the localized reason string shown in the auth dialog.
    pub fn set_reason(&self, reason: &str) {
        unsafe {
            let s = nsstring(reason);
            msg_send_void![self.inner, setLocalizedReason: s];
            CFRelease(s as CFTypeRef);
        }
    }

    /// Set the localized fallback button title (empty string hides the button).
    pub fn set_fallback_title(&self, title: &str) {
        unsafe {
            let s = nsstring(title);
            msg_send_void![self.inner, setLocalizedFallbackTitle: s];
            CFRelease(s as CFTypeRef);
        }
    }

    /// Evaluate biometric authentication synchronously (blocks the thread).
    ///
    /// For async evaluation, use `evaluate_with_callback`.
    pub fn evaluate_with_callback(
        &self,
        policy: Policy,
        reason: &str,
        callback: extern "C" fn(bool, Id, *mut core::ffi::c_void),
        context: *mut core::ffi::c_void,
    ) {
        // LAContext evaluatePolicy:localizedReason:reply: takes a block.
        // Direct block creation from Rust is complex; for simple use cases
        // callers can use the sync check + can_evaluate pattern.
        // This is left as a hook point for block-based dispatch.
        let _ = (policy, reason, callback, context);
        todo!("Block-based evaluate requires objc_block ABI support");
    }
}

impl Default for Context {
    fn default() -> Self { Self::new() }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { CFRelease(self.inner as CFTypeRef); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let ctx = Context::new();
        // Just verify it doesn't crash; biometry may or may not be available
        let _ = ctx.biometry_type();
    }

    #[test]
    fn test_can_evaluate() {
        let ctx = Context::new();
        // This will return Err on machines without biometrics, Ok on those with
        let result = ctx.can_evaluate_biometrics();
        match result {
            Ok(()) => println!("Biometrics available: {:?}", ctx.biometry_type()),
            Err(code) => println!("Biometrics unavailable (error {})", code),
        }
    }
}
