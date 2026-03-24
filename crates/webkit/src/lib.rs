//! Apple WebKit — web views from Rust.
//!
//! **Platform:** macOS 10.10+, iOS 8+, visionOS 1+.
//!
//! ```ignore
//! let config = webkit::WebViewConfiguration::new();
//! config.set_allows_inline_playback(true);
//! let webview = webkit::WebView::new([0.0, 0.0, 800.0, 600.0], &config);
//! webview.load_url("https://www.rust-lang.org");
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

// ── Configuration ───────────────────────────────────────────────────────────

/// Wraps `WKWebViewConfiguration`.
pub struct WebViewConfiguration { inner: Id }

impl WebViewConfiguration {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"WKWebViewConfiguration\0"), new] } }
    }

    /// Enable inline media playback (iOS).
    pub fn set_allows_inline_playback(&self, allow: bool) {
        unsafe { msg_send_void![self.inner, setAllowsInlineMediaPlayback: allow as u8]; }
    }

    /// Set whether JavaScript is enabled (default: true).
    pub fn set_javascript_enabled(&self, enabled: bool) {
        unsafe {
            let prefs: Id = msg_send![self.inner, defaultWebpagePreferences];
            msg_send_void![prefs, setAllowsContentJavaScript: enabled as u8];
        }
    }

    /// The website data store (for cookies, caches).
    pub fn data_store(&self) -> DataStore {
        DataStore { inner: unsafe { msg_send![self.inner, websiteDataStore] } }
    }

    /// Set the data store (e.g. for private browsing).
    pub fn set_data_store(&self, store: &DataStore) {
        unsafe { msg_send_void![self.inner, setWebsiteDataStore: store.inner]; }
    }

    /// User content controller for injecting scripts.
    pub fn user_content_controller(&self) -> UserContentController {
        UserContentController { inner: unsafe { msg_send![self.inner, userContentController] } }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for WebViewConfiguration { fn default() -> Self { Self::new() } }
impl Drop for WebViewConfiguration {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

// ── DataStore ───────────────────────────────────────────────────────────────

/// Wraps `WKWebsiteDataStore`.
pub struct DataStore { inner: Id }

impl DataStore {
    /// The default (persistent) data store.
    pub fn default() -> Self {
        Self { inner: unsafe { msg_send![class!(b"WKWebsiteDataStore\0"), defaultDataStore] } }
    }

    /// A non-persistent (private browsing) data store.
    pub fn non_persistent() -> Self {
        Self { inner: unsafe { msg_send![class!(b"WKWebsiteDataStore\0"), nonPersistentDataStore] } }
    }

    /// Whether this is a persistent store.
    pub fn is_persistent(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isPersistent] }
    }

    /// The HTTP cookie store.
    pub fn cookie_store(&self) -> HttpCookieStore {
        HttpCookieStore { inner: unsafe { msg_send![self.inner, httpCookieStore] } }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── HttpCookieStore ─────────────────────────────────────────────────────────

/// Wraps `WKHTTPCookieStore`.
pub struct HttpCookieStore { inner: Id }

impl HttpCookieStore {
    /// Delete all cookies (fire-and-forget).
    pub fn delete_all_cookies(&self) {
        unsafe {
            let types: Id = msg_send![class!(b"NSSet\0"), setWithObject:
                nsstring("WKWebsiteDataTypeCookies")];
            let store = DataStore::default();
            msg_send_void![store.inner, removeDataOfTypes: types,
                modifiedSince: msg_send![class!(b"NSDate\0"), distantPast],
                completionHandler: NIL];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── UserContentController ───────────────────────────────────────────────────

/// Wraps `WKUserContentController`.
pub struct UserContentController { inner: Id }

impl UserContentController {
    /// Add a user script (JavaScript) to be injected.
    pub fn add_user_script(&self, source: &str, inject_at_start: bool, for_main_frame_only: bool) {
        unsafe {
            let ns = nsstring(source);
            let when: isize = if inject_at_start { 0 } else { 1 }; // WKUserScriptInjectionTime
            let sel = sel_registerName(b"initWithSource:injectionTime:forMainFrameOnly:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, isize, bool) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let script = msg_send![class!(b"WKUserScript\0"), alloc];
            let script = f(script, sel, ns, when, for_main_frame_only);
            CFRelease(ns as CFTypeRef);
            msg_send_void![self.inner, addUserScript: script];
        }
    }

    /// Remove all user scripts.
    pub fn remove_all_scripts(&self) {
        unsafe { msg_send_void![self.inner, removeAllUserScripts]; }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

// ── WebView ─────────────────────────────────────────────────────────────────

/// Wraps `WKWebView`.
pub struct WebView { inner: Id }

#[repr(C)]
struct NSRect { x: f64, y: f64, w: f64, h: f64 }

impl WebView {
    /// Create a web view with the given frame `[x, y, width, height]` and configuration.
    pub fn new(frame: [f64; 4], config: &WebViewConfiguration) -> Self {
        unsafe {
            let r = NSRect { x: frame[0], y: frame[1], w: frame[2], h: frame[3] };
            let sel = sel_registerName(b"initWithFrame:configuration:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, NSRect, Id) -> Id =
                core::mem::transmute(objc_msgSend as *const ());
            let alloc: Id = msg_send![class!(b"WKWebView\0"), alloc];
            Self { inner: f(alloc, sel, r, config.inner) }
        }
    }

    /// Load a URL string.
    pub fn load_url(&self, url: &str) {
        unsafe {
            let ns = nsstring(url);
            let nsurl: Id = msg_send![class!(b"NSURL\0"), URLWithString: ns];
            let req: Id = msg_send![class!(b"NSURLRequest\0"), requestWithURL: nsurl];
            msg_send_void![self.inner, loadRequest: req];
            CFRelease(ns as CFTypeRef);
        }
    }

    /// Load an HTML string.
    pub fn load_html(&self, html: &str) {
        unsafe {
            let h = nsstring(html);
            msg_send_void![self.inner, loadHTMLString: h, baseURL: NIL];
            CFRelease(h as CFTypeRef);
        }
    }

    /// Navigate back.
    pub fn go_back(&self) { unsafe { msg_send_void![self.inner, goBack]; } }
    /// Navigate forward.
    pub fn go_forward(&self) { unsafe { msg_send_void![self.inner, goForward]; } }
    /// Reload the page.
    pub fn reload(&self) { unsafe { msg_send_void![self.inner, reload]; } }
    /// Stop loading.
    pub fn stop_loading(&self) { unsafe { msg_send_void![self.inner, stopLoading]; } }

    /// Whether the web view can navigate back.
    pub fn can_go_back(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, canGoBack] }
    }
    /// Whether the web view can navigate forward.
    pub fn can_go_forward(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, canGoForward] }
    }
    /// Whether the web view is currently loading.
    pub fn is_loading(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isLoading] }
    }
    /// Current page title.
    pub fn title(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, title]) }
    }
    /// Current URL as string.
    pub fn url(&self) -> Option<String> {
        unsafe {
            let url: Id = msg_send![self.inner, URL];
            if url.is_null() { return None; }
            nsstring_to_string(msg_send![url, absoluteString])
        }
    }
    /// Estimated loading progress (0.0 – 1.0).
    pub fn estimated_progress(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(b"estimatedProgress\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel)
        }
    }

    /// Set custom user agent string.
    pub fn set_custom_user_agent(&self, ua: &str) {
        unsafe {
            let ns = nsstring(ua);
            msg_send_void![self.inner, setCustomUserAgent: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    /// Get the current custom user agent.
    pub fn custom_user_agent(&self) -> Option<String> {
        unsafe { nsstring_to_string(msg_send![self.inner, customUserAgent]) }
    }

    /// Set whether the page allows magnification (zoom).
    pub fn set_allows_magnification(&self, allow: bool) {
        unsafe { msg_send_void![self.inner, setAllowsMagnification: allow as u8]; }
    }

    /// Current magnification level.
    pub fn magnification(&self) -> f64 {
        unsafe {
            let sel = sel_registerName(b"magnification\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel)
        }
    }

    /// Set magnification level.
    pub fn set_magnification(&self, mag: f64) {
        unsafe {
            let sel = sel_registerName(b"setMagnification:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, mag);
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for WebView {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let cfg = WebViewConfiguration::new();
        cfg.set_allows_inline_playback(true);
        cfg.set_javascript_enabled(true);
    }

    #[test]
    fn test_data_store() {
        let ds = DataStore::default();
        assert!(ds.is_persistent());
        let np = DataStore::non_persistent();
        assert!(!np.is_persistent());
    }

    #[test]
    #[ignore] // WKWebView requires an app context with main runloop
    fn test_create_webview() {
        let cfg = WebViewConfiguration::new();
        let wv = WebView::new([0.0, 0.0, 100.0, 100.0], &cfg);
        assert!(!wv.is_loading());
        assert!(wv.can_go_back() == false);
    }

    #[test]
    #[ignore] // WKWebView requires an app context
    fn test_load_html() {
        let cfg = WebViewConfiguration::new();
        let wv = WebView::new([0.0, 0.0, 100.0, 100.0], &cfg);
        wv.load_html("<h1>Hello</h1>");
    }

    #[test]
    #[ignore] // WKWebView requires an app context
    fn test_user_content() {
        let cfg = WebViewConfiguration::new();
        let uc = cfg.user_content_controller();
        uc.add_user_script("console.log('injected');", true, true);
        uc.remove_all_scripts();
    }
}
