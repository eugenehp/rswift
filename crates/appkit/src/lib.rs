//! Apple AppKit/UIKit — app lifecycle, clipboard, screen, alerts from Rust.
//! Pure Rust ObjC dispatch — no `.m` thunks.

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


pub struct Screen;
impl Screen {
    #[cfg(target_os = "macos")]
    pub fn main_size() -> (f64, f64) {
        unsafe {
            let s: Id = msg_send![class!(b"NSScreen\0"), mainScreen];
            if s.is_null() { return (0.0, 0.0); }
            // NSScreen.frame returns NSRect { origin: NSPoint, size: NSSize }
            // On ARM64, NSRect is returned in registers (4 f64s)
            let sel = sel_registerName(b"frame\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> [f64; 4] =
                core::mem::transmute(objc_msgSend as *const ());
            let r = f(s, sel);
            (r[2], r[3]) // width, height
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn main_size() -> (f64, f64) { (0.0, 0.0) }

    #[cfg(target_os = "macos")]
    pub fn count() -> usize {
        unsafe {
            let arr: Id = msg_send![class!(b"NSScreen\0"), screens];
            msg_send_t![usize; arr, count]
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn count() -> usize { 0 }

    #[cfg(target_os = "macos")]
    pub fn scale() -> f64 {
        unsafe {
            let s: Id = msg_send![class!(b"NSScreen\0"), mainScreen];
            if s.is_null() { return 1.0; }
            let sel = sel_registerName(b"backingScaleFactor\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(s, sel)
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn scale() -> f64 { 1.0 }
}

pub struct Clipboard;
impl Clipboard {
    #[cfg(target_os = "macos")]
    pub fn get() -> Option<String> {
        unsafe {
            let pb: Id = msg_send![class!(b"NSPasteboard\0"), generalPasteboard];
            let nstype = nsstring("public.utf8-plain-text");
            let s = msg_send![pb, stringForType: nstype];
            CFRelease(nstype as CFTypeRef);
            nsstring_to_string(s)
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn get() -> Option<String> { None }

    #[cfg(target_os = "macos")]
    pub fn set(text: &str) {
        unsafe {
            let pb: Id = msg_send![class!(b"NSPasteboard\0"), generalPasteboard];
            msg_send_void![pb, clearContents];
            let s = nsstring(text);
            let nstype = nsstring("public.utf8-plain-text");
            msg_send_void![pb, setString: s, forType: nstype];
            CFRelease(s as CFTypeRef);
            CFRelease(nstype as CFTypeRef);
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn set(_text: &str) {}
}

pub struct App;
impl App {
    #[cfg(target_os = "macos")]
    pub fn is_running() -> bool {
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            if app.is_null() { return false; }
            msg_send_t![bool; app, isRunning]
        }
    }
    #[cfg(not(target_os = "macos"))]
    pub fn is_running() -> bool { false }

    pub fn activate() {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            msg_send_void![app, activateIgnoringOtherApps: 1u8];
        }
    }
    pub fn terminate() {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            msg_send_void![app, terminate: NIL];
        }
    }
    pub fn hide() {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            msg_send_void![app, hide: NIL];
        }
    }
    pub fn unhide() {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            msg_send_void![app, unhideWithoutActivation];
        }
    }
    pub fn window_count() -> usize {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            if app.is_null() { return 0; }
            let wins: Id = msg_send![app, windows];
            msg_send_t![usize; wins, count]
        }
        #[cfg(not(target_os = "macos"))]
        { 0 }
    }
    pub fn key_window_title() -> Option<String> {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
            let win: Id = msg_send![app, keyWindow];
            if win.is_null() { return None; }
            nsstring_to_string(msg_send![win, title])
        }
        #[cfg(not(target_os = "macos"))]
        { None }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AlertStyle { Warning = 0, Informational = 1, Critical = 2 }

#[cfg(target_os = "macos")]
pub fn show_alert(title: &str, message: &str, style: AlertStyle) -> bool {
    unsafe {
        let alert: Id = msg_send![class!(b"NSAlert\0"), new];
        let t = nsstring(title);
        let m = nsstring(message);
        msg_send_void![alert, setMessageText: t];
        msg_send_void![alert, setInformativeText: m];
        msg_send_void![alert, setAlertStyle: style as isize];
        let ok = nsstring("OK");
        let cancel = nsstring("Cancel");
        msg_send_void![alert, addButtonWithTitle: ok];
        msg_send_void![alert, addButtonWithTitle: cancel];
        CFRelease(t as CFTypeRef);
        CFRelease(m as CFTypeRef);
        CFRelease(ok as CFTypeRef);
        CFRelease(cancel as CFTypeRef);
        let r: isize = msg_send_t![isize; alert, runModal];
        r == 1000 // NSAlertFirstButtonReturn
    }
}
#[cfg(not(target_os = "macos"))]
pub fn show_alert(_title: &str, _message: &str, _style: AlertStyle) -> bool { false }

#[cfg(target_os = "macos")]
pub fn open_url(url: &str) -> bool {
    unsafe {
        let ns = nsstring(url);
        let u: Id = msg_send![class!(b"NSURL\0"), URLWithString: ns];
        CFRelease(ns as CFTypeRef);
        if u.is_null() { return false; }
        let ws: Id = msg_send![class!(b"NSWorkspace\0"), sharedWorkspace];
        msg_send_t![bool; ws, openURL: u]
    }
}
#[cfg(not(target_os = "macos"))]
pub fn open_url(_url: &str) -> bool { false }

#[cfg(target_os = "macos")]
pub fn reveal_in_finder(path: &str) {
    unsafe {
        let p = nsstring(path);
        let empty = nsstring("");
        let ws: Id = msg_send![class!(b"NSWorkspace\0"), sharedWorkspace];
        msg_send_void![ws, selectFile: p, inFileViewerRootedAtPath: empty];
        CFRelease(p as CFTypeRef);
        CFRelease(empty as CFTypeRef);
    }
}
#[cfg(not(target_os = "macos"))]
pub fn reveal_in_finder(_path: &str) {}

#[cfg(target_os = "macos")]
pub fn is_dark_mode() -> bool {
    unsafe {
        let app: Id = msg_send![class!(b"NSApplication\0"), sharedApplication];
        if app.is_null() {
            let defs: Id = msg_send![class!(b"NSUserDefaults\0"), standardUserDefaults];
            let key = nsstring("AppleInterfaceStyle");
            let val = msg_send![defs, stringForKey: key];
            CFRelease(key as CFTypeRef);
            return nsstring_to_string(val).map(|s| s.to_lowercase() == "dark").unwrap_or(false);
        }
        let appearance: Id = msg_send![app, effectiveAppearance];
        let dark = nsstring("NSAppearanceNameDarkAqua");
        let aqua = nsstring("NSAppearanceNameAqua");
        // arrayWithObjects:count: takes a C array, not variadic
        let objs = [dark as CFTypeRef, aqua as CFTypeRef];
        let sel = sel_registerName(b"arrayWithObjects:count:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, *const CFTypeRef, usize) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let arr = f(class!(b"NSArray\0") as Id, sel, objs.as_ptr(), 2);
        let best = msg_send![appearance, bestMatchFromAppearancesWithNames: arr];
        let eq: bool = msg_send_t![bool; best, isEqualToString: dark];
        CFRelease(dark as CFTypeRef);
        CFRelease(aqua as CFTypeRef);
        eq
    }
}
#[cfg(not(target_os = "macos"))]
pub fn is_dark_mode() -> bool { false }
