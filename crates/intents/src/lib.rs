//! Apple Intents — SiriKit intents and shortcuts from Rust.
//!
//! **Platform:** macOS 11+, iOS 10+, watchOS 3.2+.
//!
//! ```ignore
//! let interaction = intents::Interaction::from_intent_ptr(intent_ptr);
//! println!("Direction: {:?}", interaction.direction());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool {
    unsafe { !class!(b"INInteraction\0").is_null() }
}

/// Interaction direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionDirection { Unspecified = 0, Outgoing = 1, Incoming = 2 }
impl From<isize> for InteractionDirection {
    fn from(v: isize) -> Self { match v { 1=>Self::Outgoing, 2=>Self::Incoming, _=>Self::Unspecified } }
}

/// Wraps `INInteraction`.
pub struct Interaction { inner: Id }

impl Interaction {
    /// Wrap an existing INInteraction pointer.
    pub fn from_ptr(ptr: Id) -> Self {
        unsafe { CFRetain(ptr as CFTypeRef); }
        Self { inner: ptr }
    }

    /// The direction of the interaction.
    pub fn direction(&self) -> InteractionDirection {
        unsafe { InteractionDirection::from(msg_send_t![isize; self.inner, intentHandlingStatus]) }
    }

    /// The interaction identifier.
    pub fn identifier(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, identifier]) }
    }

    /// Group identifier.
    pub fn group_identifier(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, groupIdentifier]) }
    }

    /// Delete all donated interactions.
    pub fn delete_all() {
        unsafe { msg_send_void![class!(b"INInteraction\0"), deleteAllInteractionsWithCompletion: NIL]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for Interaction { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }

/// Wraps `INPerson`.
pub struct Person { inner: Id }

impl Person {
    pub fn display_name(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, displayName]) }
    }
    pub fn person_handle_value(&self) -> Option<String> {
        unsafe {
            let handle: Id = msg_send![self.inner, personHandle];
            if handle.is_null() { return None; }
            nsstring_to_string(msg_send![handle, value])
        }
    }
}

/// Wraps `INImage`.
pub struct IntentImage { inner: Id }

impl IntentImage {
    /// Create from a system image name (SF Symbols).
    pub fn system_image(name: &str) -> Self {
        unsafe {
            let ns = nsstring(name);
            let img: Id = msg_send![class!(b"INImage\0"), systemImageNamed: ns];
            CFRelease(ns as CFTypeRef);
            CFRetain(img as CFTypeRef);
            Self { inner: img }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for IntentImage { fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } } }
