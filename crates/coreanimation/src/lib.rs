#![allow(unsafe_op_in_unsafe_fn)]
//! Apple CoreAnimation — animation timing and transactions from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 2+, tvOS 9+, visionOS 1+.
//!
//! Links QuartzCore directly — CACurrentMediaTime is pure C,
//! CATransaction uses ObjC message dispatch via libobjc.
//!
//! # Quick start
//!
//! ```ignore
//! let t = coreanimation::current_media_time();
//!
//! coreanimation::Transaction::begin();
//! coreanimation::Transaction::set_duration(0.3);
//! coreanimation::Transaction::commit();
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

/// CoreAnimation is always available on Apple platforms.
/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── Pure C symbol ───────────────────────────────────────────────────────────

#[allow(non_snake_case)]
unsafe extern "C" {
    fn CACurrentMediaTime() -> f64;
}

// ── ObjC runtime (always linked on Apple) ───────────────────────────────────

unsafe extern "C" {
    fn objc_getClass(name: *const u8) -> *const c_void;
    fn sel_registerName(name: *const u8) -> *const c_void;
    fn objc_msgSend(receiver: *const c_void, sel: *const c_void, ...) -> *const c_void;
}

macro_rules! cls {
    ($name:literal) => { objc_getClass(concat!($name, "\0").as_ptr()) };
}
macro_rules! sel {
    ($name:literal) => { sel_registerName(concat!($name, "\0").as_ptr()) };
}

// Type aliases for objc_msgSend casts — calling convention varies by return type
type MsgSendVoid = unsafe extern "C" fn(*const c_void, *const c_void);
type MsgSendVoidF64 = unsafe extern "C" fn(*const c_void, *const c_void, f64);
type MsgSendVoidBool = unsafe extern "C" fn(*const c_void, *const c_void, bool);
type MsgSendVoidId = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void);

/// High-precision media time (seconds since boot, monotonic).
pub fn current_media_time() -> f64 {
    unsafe { CACurrentMediaTime() }
}

/// Animation timing functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingFunction {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Default = 4,
}

/// Explicit animation transaction (wraps `CATransaction` via ObjC).
pub struct Transaction;

impl Transaction {
    /// Begin an explicit animation transaction.
    pub fn begin() {
        unsafe {
            let f: MsgSendVoid = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("begin"));
        }
    }

    /// Commit the current transaction.
    pub fn commit() {
        unsafe {
            let f: MsgSendVoid = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("commit"));
        }
    }

    /// Flush pending transactions immediately.
    pub fn flush() {
        unsafe {
            let f: MsgSendVoid = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("flush"));
        }
    }

    /// Set the animation duration for the current transaction.
    pub fn set_duration(seconds: f64) {
        unsafe {
            let f: MsgSendVoidF64 = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("setAnimationDuration:"), seconds);
        }
    }

    /// Disable implicit animations in the current transaction.
    pub fn set_disable_actions(disable: bool) {
        unsafe {
            let f: MsgSendVoidBool = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("setDisableActions:"), disable);
        }
    }

    /// Set the timing function for the current transaction.
    pub fn set_timing(timing: TimingFunction) {
        unsafe {
            let name_str = match timing {
                TimingFunction::Linear => "kCAMediaTimingFunctionLinear\0",
                TimingFunction::EaseIn => "kCAMediaTimingFunctionEaseIn\0",
                TimingFunction::EaseOut => "kCAMediaTimingFunctionEaseOut\0",
                TimingFunction::EaseInEaseOut => "kCAMediaTimingFunctionEaseInEaseOut\0",
                TimingFunction::Default => "kCAMediaTimingFunctionDefault\0",
            };
            // [CAMediaTimingFunction functionWithName:name]
            let timing_cls = cls!("CAMediaTimingFunction");
            // The name constants are NSString globals — we use the string form
            // via +[CAMediaTimingFunction functionWithName:]
            // The actual constant names are e.g. kCAMediaTimingFunctionLinear as NSString
            // We look them up as symbols
            let name_sel = match timing {
                TimingFunction::Linear => get_timing_constant(b"kCAMediaTimingFunctionLinear\0"),
                TimingFunction::EaseIn => get_timing_constant(b"kCAMediaTimingFunctionEaseIn\0"),
                TimingFunction::EaseOut => get_timing_constant(b"kCAMediaTimingFunctionEaseOut\0"),
                TimingFunction::EaseInEaseOut => get_timing_constant(b"kCAMediaTimingFunctionEaseInEaseOut\0"),
                TimingFunction::Default => get_timing_constant(b"kCAMediaTimingFunctionDefault\0"),
            };
            let _ = name_str; // used for the match arm above
            if name_sel.is_null() { return; }
            // +[CAMediaTimingFunction functionWithName:]
            let tf = objc_msgSend(timing_cls, sel!("functionWithName:"), name_sel);
            // +[CATransaction setAnimationTimingFunction:]
            let f: MsgSendVoidId = core::mem::transmute(objc_msgSend as *const c_void);
            f(cls!("CATransaction"), sel!("setAnimationTimingFunction:"), tf);
        }
    }
}

/// Look up a QuartzCore NSString constant by symbol name via dlsym.
unsafe fn get_timing_constant(name: &[u8]) -> *const c_void {
    extern "C" { fn dlsym(handle: *mut c_void, symbol: *const u8) -> *mut c_void; }
    let ptr = dlsym((-2isize) as *mut c_void, name.as_ptr()); // RTLD_DEFAULT
    if ptr.is_null() { return core::ptr::null(); }
    // The symbol is a pointer to an NSString (CFStringRef)
    *(ptr as *const *const c_void)
}
