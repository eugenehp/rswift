//! Apple ScreenCaptureKit — screen recording and display capture from Rust.
//!
//! **Platform:** macOS 12.3+.
//!
//! ```ignore
//! let displays = screencapturekit::available_displays();
//! for d in &displays {
//!     println!("Display {}: {}×{}", d.display_id, d.width, d.height);
//! }
//! let windows = screencapturekit::available_windows();
//! for w in &windows {
//!     println!("Window: {:?} ({})", w.title, w.owning_app);
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

pub fn is_available() -> bool {
    unsafe { !class!(b"SCShareableContent\0").is_null() }
}

/// Info about a capturable display.
#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub display_id: u32,
    pub width: usize,
    pub height: usize,
}

/// Info about a capturable window.
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub window_id: u32,
    pub title: Option<String>,
    pub owning_app: String,
    pub is_on_screen: bool,
    pub width: usize,
    pub height: usize,
}

/// Synchronously fetch available displays.
/// Note: SCShareableContent normally uses async completion handlers.
/// This uses the synchronous ObjC API if available, or returns empty.
pub fn available_displays() -> Vec<DisplayInfo> {
    // SCShareableContent requires async block-based API.
    // For a sync fallback, use CGGetActiveDisplayList (CoreGraphics C API).
    unsafe {
        #[allow(non_snake_case)]
        extern "C" {
            fn CGGetActiveDisplayList(max: u32, displays: *mut u32, count: *mut u32) -> i32;
            fn CGDisplayPixelsWide(display: u32) -> usize;
            fn CGDisplayPixelsHigh(display: u32) -> usize;
        }
        let mut count = 0u32;
        CGGetActiveDisplayList(0, core::ptr::null_mut(), &mut count);
        if count == 0 { return vec![]; }
        let mut ids = vec![0u32; count as usize];
        CGGetActiveDisplayList(count, ids.as_mut_ptr(), &mut count);
        ids.iter().map(|&id| DisplayInfo {
            display_id: id,
            width: CGDisplayPixelsWide(id),
            height: CGDisplayPixelsHigh(id),
        }).collect()
    }
}

/// Fetch available windows via CGWindowListCopyWindowInfo (synchronous).
pub fn available_windows() -> Vec<WindowInfo> {
    unsafe {
        extern "C" {
            fn CGWindowListCopyWindowInfo(option: u32, rel: u32) -> Id;
        }
        // kCGWindowListOptionOnScreenOnly = 1, kCGNullWindowID = 0
        let list = CGWindowListCopyWindowInfo(1, 0);
        if list.is_null() { return vec![]; }
        let count: usize = msg_send_t![usize; list, count];
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let info: Id = msg_send![list, objectAtIndex: i];
            let wid_key = nsstring("kCGWindowNumber");
            let name_key = nsstring("kCGWindowName");
            let app_key = nsstring("kCGWindowOwnerName");
            let on_key = nsstring("kCGWindowIsOnscreen");
            let w_key = nsstring("kCGWindowBounds");

            let wid_val: Id = msg_send![info, objectForKey: wid_key];
            let window_id = if !wid_val.is_null() {
                msg_send_t![u32; wid_val, unsignedIntValue]
            } else { 0 };

            let title = {
                let v: Id = msg_send![info, objectForKey: name_key];
                nsstring_to_string(v)
            };
            let owning_app = {
                let v: Id = msg_send![info, objectForKey: app_key];
                nsstring_to_string(v).unwrap_or_default()
            };
            let is_on_screen = {
                let v: Id = msg_send![info, objectForKey: on_key];
                if !v.is_null() { msg_send_t![bool; v, boolValue] } else { false }
            };

            // Bounds dict → width/height
            let bounds: Id = msg_send![info, objectForKey: w_key];
            let (width, height) = if !bounds.is_null() {
                let w_k = nsstring("Width");
                let h_k = nsstring("Height");
                let wv: Id = msg_send![bounds, objectForKey: w_k];
                let hv: Id = msg_send![bounds, objectForKey: h_k];
                CFRelease(w_k as CFTypeRef);
                CFRelease(h_k as CFTypeRef);
                let w = if !wv.is_null() { msg_send_t![usize; wv, integerValue] } else { 0 };
                let h = if !hv.is_null() { msg_send_t![usize; hv, integerValue] } else { 0 };
                (w, h)
            } else { (0, 0) };

            for k in [wid_key, name_key, app_key, on_key, w_key] {
                CFRelease(k as CFTypeRef);
            }
            result.push(WindowInfo { window_id, title, owning_app, is_on_screen, width, height });
        }
        CFRelease(list as CFTypeRef);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displays() {
        let displays = available_displays();
        assert!(!displays.is_empty(), "Should have at least one display");
        let d = &displays[0];
        assert!(d.width > 0);
        assert!(d.height > 0);
        println!("Primary: {}×{}", d.width, d.height);
    }

    #[test]
    fn test_windows() {
        let windows = available_windows();
        assert!(!windows.is_empty(), "Should have at least one window");
        // Find at least one named window
        let named = windows.iter().filter(|w| w.title.is_some()).count();
        println!("Total windows: {}, named: {}", windows.len(), named);
    }
}
