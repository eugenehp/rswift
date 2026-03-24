#![allow(dead_code)]
//! Apple Network.framework — modern networking from Rust.
//!
//! **Platform:** macOS 10.14+, iOS 12+, tvOS 12+, watchOS 5+.
//!
//! Pure C framework (`nw_*` symbols). Replaces BSD sockets for TCP/UDP/QUIC.
//!
//! ```ignore
//! let endpoint = network::Endpoint::host("example.com", "443");
//! let params = network::Parameters::tls();
//! let conn = network::Connection::new(&endpoint, &params);
//! conn.start();
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_snake_case)]

use core::ffi::c_void;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// Opaque OS object types (refcounted via nw_retain/nw_release but also
// toll-free bridged to NSObject, so CFRelease works).
type NwEndpointT = *mut c_void;
type NwParametersT = *mut c_void;
type NwConnectionT = *mut c_void;
type NwListenerT = *mut c_void;
type NwPathMonitorT = *mut c_void;
type DispatchQueueT = *mut c_void;

unsafe extern "C" {
    // Endpoints
    fn nw_endpoint_create_host(hostname: *const u8, port: *const u8) -> NwEndpointT;
    fn nw_endpoint_get_hostname(ep: NwEndpointT) -> *const u8;
    fn nw_endpoint_copy_port_string(ep: NwEndpointT) -> *mut u8;

    // Parameters
    fn nw_parameters_create_secure_tcp(
        tls_config: *const c_void,
        tcp_config: *const c_void,
    ) -> NwParametersT;
    fn nw_parameters_create_secure_udp(
        dtls_config: *const c_void,
        udp_config: *const c_void,
    ) -> NwParametersT;

    // Connection
    fn nw_connection_create(endpoint: NwEndpointT, params: NwParametersT) -> NwConnectionT;
    fn nw_connection_set_queue(conn: NwConnectionT, queue: DispatchQueueT);
    fn nw_connection_start(conn: NwConnectionT);
    fn nw_connection_cancel(conn: NwConnectionT);
    fn nw_connection_force_cancel(conn: NwConnectionT);
    fn nw_connection_copy_endpoint(conn: NwConnectionT) -> NwEndpointT;
    fn nw_connection_copy_description(conn: NwConnectionT) -> *mut u8;

    // Listener
    fn nw_listener_create(params: NwParametersT) -> NwListenerT;
    fn nw_listener_set_queue(listener: NwListenerT, queue: DispatchQueueT);
    fn nw_listener_start(listener: NwListenerT);
    fn nw_listener_cancel(listener: NwListenerT);

    // Path monitor
    fn nw_path_monitor_create() -> NwPathMonitorT;
    fn nw_path_monitor_set_queue(monitor: NwPathMonitorT, queue: DispatchQueueT);
    fn nw_path_monitor_start(monitor: NwPathMonitorT);
    fn nw_path_monitor_cancel(monitor: NwPathMonitorT);

    // GCD queue
    fn dispatch_get_main_queue() -> DispatchQueueT;

    // nw objects use OS_OBJECT refcounting
    fn nw_release(obj: *mut c_void);
    fn nw_retain(obj: *mut c_void) -> *mut c_void;

    fn free(ptr: *mut c_void);
}

// ── Endpoint ────────────────────────────────────────────────────────────────

/// A network endpoint (host + port).
pub struct Endpoint(NwEndpointT);

impl Endpoint {
    /// Create a host endpoint. Both must be null-terminated-safe ASCII.
    pub fn host(hostname: &str, port: &str) -> Self {
        let mut h = Vec::with_capacity(hostname.len() + 1);
        h.extend_from_slice(hostname.as_bytes());
        h.push(0);
        let mut p = Vec::with_capacity(port.len() + 1);
        p.extend_from_slice(port.as_bytes());
        p.push(0);
        Self(unsafe { nw_endpoint_create_host(h.as_ptr(), p.as_ptr()) })
    }

    pub fn hostname(&self) -> &str {
        unsafe {
            let ptr = nw_endpoint_get_hostname(self.0);
            if ptr.is_null() { return ""; }
            let cstr = core::ffi::CStr::from_ptr(ptr as *const core::ffi::c_char);
            cstr.to_str().unwrap_or("")
        }
    }

    pub fn port(&self) -> String {
        unsafe {
            let ptr = nw_endpoint_copy_port_string(self.0);
            if ptr.is_null() { return String::new(); }
            let cstr = core::ffi::CStr::from_ptr(ptr as *const core::ffi::c_char);
            let s = cstr.to_string_lossy().into_owned();
            free(ptr as *mut c_void);
            s
        }
    }

    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}

impl Drop for Endpoint { fn drop(&mut self) { unsafe { nw_release(self.0); } } }

// ── Parameters ──────────────────────────────────────────────────────────────

/// Connection parameters.
pub struct Parameters(NwParametersT);

impl Parameters {
    /// TLS + TCP parameters (HTTPS-style).
    pub fn tls() -> Self {
        // NW_PARAMETERS_DEFAULT_CONFIGURATION = sentinel pointer (1)
        let default_config = 1usize as *const c_void;
        Self(unsafe { nw_parameters_create_secure_tcp(default_config, default_config) })
    }

    /// Plain TCP (no TLS).
    pub fn tcp() -> Self {
        let default_config = 1usize as *const c_void;
        Self(unsafe { nw_parameters_create_secure_tcp(core::ptr::null(), default_config) })
    }

    /// DTLS + UDP.
    pub fn dtls() -> Self {
        let default_config = 1usize as *const c_void;
        Self(unsafe { nw_parameters_create_secure_udp(default_config, default_config) })
    }

    /// Plain UDP.
    pub fn udp() -> Self {
        let default_config = 1usize as *const c_void;
        Self(unsafe { nw_parameters_create_secure_udp(core::ptr::null(), default_config) })
    }

    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}

impl Drop for Parameters { fn drop(&mut self) { unsafe { nw_release(self.0); } } }

// ── Connection ──────────────────────────────────────────────────────────────

/// A network connection.
pub struct Connection(NwConnectionT);

impl Connection {
    pub fn new(endpoint: &Endpoint, params: &Parameters) -> Self {
        Self(unsafe { nw_connection_create(endpoint.0, params.0) })
    }

    /// Set the dispatch queue and start the connection.
    pub fn start(&self) {
        unsafe {
            nw_connection_set_queue(self.0, dispatch_get_main_queue());
            nw_connection_start(self.0);
        }
    }

    /// Cancel the connection gracefully.
    pub fn cancel(&self) { unsafe { nw_connection_cancel(self.0); } }

    /// Force-cancel the connection.
    pub fn force_cancel(&self) { unsafe { nw_connection_force_cancel(self.0); } }

    pub fn as_ptr(&self) -> *mut c_void { self.0 }
}

impl Drop for Connection { fn drop(&mut self) { unsafe { nw_release(self.0); } } }

// ── PathMonitor ─────────────────────────────────────────────────────────────

/// Monitors network path changes (connectivity).
pub struct PathMonitor(NwPathMonitorT);

impl PathMonitor {
    pub fn new() -> Self {
        Self(unsafe { nw_path_monitor_create() })
    }

    pub fn start(&self) {
        unsafe {
            nw_path_monitor_set_queue(self.0, dispatch_get_main_queue());
            nw_path_monitor_start(self.0);
        }
    }

    pub fn cancel(&self) { unsafe { nw_path_monitor_cancel(self.0); } }
}

impl Default for PathMonitor { fn default() -> Self { Self::new() } }
impl Drop for PathMonitor { fn drop(&mut self) { unsafe { nw_release(self.0); } } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint() {
        let ep = Endpoint::host("example.com", "443");
        assert_eq!(ep.hostname(), "example.com");
        assert_eq!(ep.port(), "443");
    }

    #[test]
    fn test_parameters() {
        let _ = Parameters::tls();
        let _ = Parameters::tcp();
        let _ = Parameters::udp();
    }

    #[test]
    fn test_connection_create() {
        let ep = Endpoint::host("example.com", "443");
        let params = Parameters::tls();
        let conn = Connection::new(&ep, &params);
        // Just ensure no crash — don't actually start (requires run loop)
        let _ = conn;
    }
}
