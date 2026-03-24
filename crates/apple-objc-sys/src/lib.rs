#![allow(unsafe_op_in_unsafe_fn)]
//! Minimal ObjC runtime FFI for Apple framework crates.
//!
//! Provides the raw ObjC runtime symbols (`objc_msgSend`, `objc_getClass`,
//! `sel_registerName`) plus ergonomic helper macros so framework crates can
//! call ObjC methods directly from Rust — no `.m` files, no `cc` build step.
//!
//! # Example
//!
//! ```ignore
//! use apple_objc_sys::*;
//!
//! unsafe {
//!     let cls = class!(b"NSProcessInfo");
//!     let info: Id = msg_send![cls, processInfo];
//!     let count: isize = msg_send![info, processorCount];
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_camel_case_types)]

use core::ffi::c_void;

// ── Types ───────────────────────────────────────────────────────────────────

/// Opaque ObjC object pointer.
pub type Id = *mut c_void;
/// Opaque ObjC class pointer.
pub type Class = *const c_void;
/// Opaque ObjC selector pointer.
pub type Sel = *const c_void;
/// NULL object.
pub const NIL: Id = core::ptr::null_mut();

// ── ObjC runtime ────────────────────────────────────────────────────────────

unsafe extern "C" {
    pub fn objc_getClass(name: *const u8) -> Class;
    pub fn sel_registerName(name: *const u8) -> Sel;
    pub fn objc_msgSend(receiver: Id, sel: Sel, ...) -> Id;
    /// `objc_msgSend` variant for stret on x86_64 (structs > 16 bytes).
    #[cfg(target_arch = "x86_64")]
    pub fn objc_msgSend_stret(stret: *mut c_void, receiver: Id, sel: Sel, ...);
}

// ── CoreFoundation helpers ──────────────────────────────────────────────────

pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFIndex = isize;

pub const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;
pub const K_CF_ALLOCATOR_DEFAULT: CFAllocatorRef = core::ptr::null();

unsafe extern "C" {
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef, bytes: *const u8, num_bytes: CFIndex,
        encoding: u32, is_external: bool,
    ) -> CFStringRef;
    pub fn CFStringGetLength(s: CFStringRef) -> CFIndex;
    pub fn CFStringGetCStringPtr(s: CFStringRef, encoding: u32) -> *const u8;
    pub fn CFStringGetCString(
        s: CFStringRef, buffer: *mut u8, buffer_size: CFIndex, encoding: u32,
    ) -> bool;
    pub fn CFDataCreate(alloc: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
    pub fn CFDictionaryCreate(
        alloc: CFAllocatorRef,
        keys: *const CFTypeRef, values: *const CFTypeRef, num_values: CFIndex,
        key_cbs: *const c_void, value_cbs: *const c_void,
    ) -> CFDictionaryRef;

    pub static kCFTypeDictionaryKeyCallBacks: c_void;
    pub static kCFTypeDictionaryValueCallBacks: c_void;
    pub static kCFBooleanTrue: CFTypeRef;
    pub static kCFBooleanFalse: CFTypeRef;
}

// ── dlsym ───────────────────────────────────────────────────────────────────

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const u8) -> *mut c_void;
}

/// Look up a global symbol by name via `dlsym(RTLD_DEFAULT, name)`.
#[inline]
pub unsafe fn dlsym_global(name: &[u8]) -> *const c_void {
    dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void
}

/// Read a global `NSString*` / `CFStringRef` constant by symbol name.
/// Many Apple framework constants (e.g. `kSecClass`) are pointers to
/// `CFStringRef` — this dereferences the pointer.
#[inline]
pub unsafe fn global_string_const(name: &[u8]) -> CFStringRef {
    let ptr = dlsym_global(name);
    if ptr.is_null() { return core::ptr::null(); }
    *(ptr as *const CFStringRef)
}

// ── Helper: NSString ↔ Rust ─────────────────────────────────────────────────

/// Create an autoreleased `NSString` from a Rust `&str`.
///
/// Uses `CFStringCreateWithBytes` which returns a toll-free-bridged
/// `NSString*`. Caller must manage the refcount (or let autorelease handle it).
#[inline]
pub unsafe fn nsstring(s: &str) -> Id {
    CFStringCreateWithBytes(
        K_CF_ALLOCATOR_DEFAULT,
        s.as_ptr(), s.len() as CFIndex,
        K_CF_STRING_ENCODING_UTF8, false,
    ) as Id
}

/// Read an `NSString*` (toll-free bridged `CFStringRef`) into a Rust `String`.
pub unsafe fn nsstring_to_string(s: Id) -> Option<String> {
    if s.is_null() { return None; }
    let cfstr = s as CFStringRef;
    // Fast path: direct C pointer
    let cptr = CFStringGetCStringPtr(cfstr, K_CF_STRING_ENCODING_UTF8);
    if !cptr.is_null() {
        let cstr = core::ffi::CStr::from_ptr(cptr as *const core::ffi::c_char);
        return Some(cstr.to_string_lossy().into_owned());
    }
    // Slow path: copy into buffer
    let len = CFStringGetLength(cfstr);
    let buf_size = len * 4 + 1; // worst case UTF-8
    let mut buf = vec![0u8; buf_size as usize];
    if CFStringGetCString(cfstr, buf.as_mut_ptr(), buf_size, K_CF_STRING_ENCODING_UTF8) {
        let cstr = core::ffi::CStr::from_ptr(buf.as_ptr() as *const core::ffi::c_char);
        Some(cstr.to_string_lossy().into_owned())
    } else {
        None
    }
}

/// Write an `NSString*` into a `(buf, len)` pair, returning bytes written or -1.
pub unsafe fn nsstring_to_buf(s: Id, buf: *mut u8, buf_len: usize) -> isize {
    if s.is_null() { return -1; }
    match nsstring_to_string(s) {
        Some(string) => {
            let bytes = string.as_bytes();
            let n = bytes.len().min(buf_len);
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, n);
            n as isize
        }
        None => -1,
    }
}

/// Build a `CFDictionaryRef` from parallel key/value slices.
#[inline]
pub unsafe fn cfdict(keys: &[CFTypeRef], vals: &[CFTypeRef]) -> CFDictionaryRef {
    CFDictionaryCreate(
        K_CF_ALLOCATOR_DEFAULT,
        keys.as_ptr(), vals.as_ptr(), keys.len() as CFIndex,
        &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
        &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
    )
}

// ── Macros ──────────────────────────────────────────────────────────────────

/// Look up an ObjC class by name (null-terminated byte string).
///
/// ```ignore
/// let cls = class!(b"NSString\0");
/// ```
#[macro_export]
macro_rules! class {
    ($name:expr) => {
        $crate::objc_getClass($name.as_ptr())
    };
}

/// Register / look up an ObjC selector (null-terminated byte string).
///
/// ```ignore
/// let sel = sel!(b"init\0");
/// ```
#[macro_export]
macro_rules! sel {
    ($name:expr) => {
        $crate::sel_registerName($name.as_ptr())
    };
}

/// Send an ObjC message. Returns `Id` by default.
///
/// ```ignore
/// // No args
/// let obj: Id = msg_send![receiver, init];
/// // With args
/// let s: Id = msg_send![cls, stringWithUTF8String: ptr];
/// ```
#[macro_export]
macro_rules! msg_send {
    // No arguments → returns Id
    [$obj:expr, $sel:ident] => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $crate::sel!(concat!(stringify!($sel), "\0").as_bytes()))
    }};

    // With arguments → returns Id
    [$obj:expr, $($sel:ident : $arg:expr),+ $(,)?] => {{
        // Build selector string at compile time
        let sel_name = concat!($(stringify!($sel), ":",)+ "\0");
        let sel = $crate::sel_registerName(sel_name.as_ptr());
        // We transmute to the right variadic-free signature
        msg_send!(@call $obj, sel, $($arg),+)
    }};

    // Internal: call with 1 arg
    (@call $obj:expr, $sel:expr, $a1:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1)
    }};
    // 2 args
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2)
    }};
    // 3 args
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3)
    }};
    // 4 args
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3, $a4)
    }};
    // 5 args
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _, _, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3, $a4, $a5)
    }};
    // 6 args
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _, _, _, _) -> $crate::Id =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3, $a4, $a5, $a6)
    }};
}

/// Send an ObjC message that returns a typed non-Id value (bool, isize, f64, etc.).
///
/// ```ignore
/// let count: isize = msg_send_t![info, processorCount];
/// let ok: bool = msg_send_t![mgr, fileExistsAtPath: path];
/// ```
#[macro_export]
macro_rules! msg_send_t {
    // Return type $t, no args
    [$t:ty; $obj:expr, $sel:ident] => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel) -> $t =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $crate::sel!(concat!(stringify!($sel), "\0").as_bytes()))
    }};
    // Return type $t, with args
    [$t:ty; $obj:expr, $($sel:ident : $arg:expr),+ $(,)?] => {{
        let sel_name = concat!($(stringify!($sel), ":",)+ "\0");
        let sel = $crate::sel_registerName(sel_name.as_ptr());
        msg_send_t!(@call $t; $obj, sel, $($arg),+)
    }};

    (@call $t:ty; $obj:expr, $sel:expr, $a1:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _) -> $t =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1)
    }};
    (@call $t:ty; $obj:expr, $sel:expr, $a1:expr, $a2:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _) -> $t =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2)
    }};
    (@call $t:ty; $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _) -> $t =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3)
    }};
    (@call $t:ty; $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _, _) -> $t =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3, $a4)
    }};
}

/// Void-returning ObjC message send.
#[macro_export]
macro_rules! msg_send_void {
    [$obj:expr, $sel:ident] => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel) =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $crate::sel!(concat!(stringify!($sel), "\0").as_bytes()))
    }};
    [$obj:expr, $($sel:ident : $arg:expr),+ $(,)?] => {{
        let sel_name = concat!($(stringify!($sel), ":",)+ "\0");
        let sel = $crate::sel_registerName(sel_name.as_ptr());
        msg_send_void!(@call $obj, sel, $($arg),+)
    }};

    (@call $obj:expr, $sel:expr, $a1:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _) =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1)
    }};
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _) =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2)
    }};
    (@call $obj:expr, $sel:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        let f: unsafe extern "C" fn($crate::Id, $crate::Sel, _, _, _) =
            core::mem::transmute($crate::objc_msgSend as *const ());
        f($obj as $crate::Id, $sel, $a1, $a2, $a3)
    }};
}
