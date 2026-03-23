//! Apple AppKit/UIKit — app lifecycle, clipboard, screen, alerts from Rust.
//!
//! **Platform support:** macOS (AppKit), iOS/tvOS/visionOS (UIKit).
//!
//! Provides a cross-platform API that maps to AppKit on macOS and UIKit on iOS.
//!
//! # Quick start
//!
//! ```ignore
//! // Screen info
//! let (w, h) = appkit::Screen::main_size();
//! println!("Screen: {w}×{h} @{:.0}x", appkit::Screen::scale());
//!
//! // Clipboard
//! appkit::Clipboard::set("Hello from Rust!");
//! println!("{}", appkit::Clipboard::get().unwrap_or_default());
//!
//! // Open URL
//! appkit::open_url("https://github.com/eugenehp/rswift");
//!
//! // Dark mode
//! println!("Dark mode: {}", appkit::is_dark_mode());
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"appkit_available");

unsafe extern "C" {
    // Screen
    fn appkit_main_screen_size(w: *mut f64, h: *mut f64);
    fn appkit_screen_count() -> isize;
    fn appkit_main_screen_scale() -> f64;

    // Clipboard
    fn appkit_clipboard_get_string(buf: *mut u8, bl: usize) -> isize;
    fn appkit_clipboard_set_string(ptr: *const u8, len: usize);

    // App control
    fn appkit_app_is_running() -> bool;
    fn appkit_app_activate();
    fn appkit_app_terminate();
    fn appkit_app_hide();
    fn appkit_app_unhide();

    // Alerts
    fn appkit_show_alert(t: *const u8, tl: usize, m: *const u8, ml: usize, style: isize) -> isize;

    // URL
    fn appkit_open_url(u: *const u8, ul: usize) -> bool;
    fn appkit_reveal_in_finder(p: *const u8, pl: usize);

    // Dark mode
    fn appkit_is_dark_mode() -> bool;

    // Window
    fn appkit_key_window_title(buf: *mut u8, bl: usize) -> isize;
    fn appkit_window_count() -> isize;
}

/// Screen information.
pub struct Screen;

impl Screen {
    /// Main screen size in points: `(width, height)`.
    pub fn main_size() -> (f64, f64) {
        let (mut w, mut h) = (0.0f64, 0.0f64);
        unsafe { appkit_main_screen_size(&mut w, &mut h) }
        (w, h)
    }

    /// Number of connected screens.
    pub fn count() -> usize {
        unsafe { appkit_screen_count() as usize }
    }

    /// Main screen backing scale factor (1.0 or 2.0 for Retina).
    pub fn scale() -> f64 {
        unsafe { appkit_main_screen_scale() }
    }
}

/// System clipboard (pasteboard).
pub struct Clipboard;

impl Clipboard {
    /// Get the current clipboard string.
    pub fn get() -> Option<String> {
        let mut buf = vec![0u8; 65536];
        let len = unsafe { appkit_clipboard_get_string(buf.as_mut_ptr(), buf.len()) };
        if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len.min(buf.len() as isize) as usize]).into()) }
    }

    /// Set the clipboard to a string.
    pub fn set(text: &str) {
        unsafe { appkit_clipboard_set_string(text.as_ptr(), text.len()) }
    }
}

/// Application lifecycle control.
pub struct App;

impl App {
    /// Whether the application's run loop is active.
    pub fn is_running() -> bool {
        unsafe { appkit_app_is_running() }
    }

    /// Activate the application (bring to front).
    pub fn activate() {
        unsafe { appkit_app_activate() }
    }

    /// Terminate the application.
    pub fn terminate() {
        unsafe { appkit_app_terminate() }
    }

    /// Hide the application.
    pub fn hide() {
        unsafe { appkit_app_hide() }
    }

    /// Unhide the application.
    pub fn unhide() {
        unsafe { appkit_app_unhide() }
    }

    /// Number of open windows.
    pub fn window_count() -> usize {
        unsafe { appkit_window_count() as usize }
    }

    /// Title of the key (focused) window.
    pub fn key_window_title() -> Option<String> {
        let mut buf = vec![0u8; 4096];
        let len = unsafe { appkit_key_window_title(buf.as_mut_ptr(), buf.len()) };
        if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into()) }
    }
}

/// Alert dialog style.
#[derive(Debug, Clone, Copy)]
pub enum AlertStyle {
    Warning = 0,
    Informational = 1,
    Critical = 2,
}

/// Show a modal alert dialog. Returns `true` if OK was clicked.
pub fn show_alert(title: &str, message: &str, style: AlertStyle) -> bool {
    let r = unsafe {
        appkit_show_alert(title.as_ptr(), title.len(), message.as_ptr(), message.len(), style as isize)
    };
    r == 1
}

/// Open a URL in the default browser/handler.
pub fn open_url(url: &str) -> bool {
    unsafe { appkit_open_url(url.as_ptr(), url.len()) }
}

/// Reveal a file in Finder (macOS only).
pub fn reveal_in_finder(path: &str) {
    unsafe { appkit_reveal_in_finder(path.as_ptr(), path.len()) }
}

/// Whether the system is in dark mode.
pub fn is_dark_mode() -> bool {
    unsafe { appkit_is_dark_mode() }
}
