//! Apple OSLog — unified structured logging from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 10+, tvOS 10+, watchOS 3+.
//!
//! Calls `os_log_create` and `_os_log_impl` directly — pure C, no bridge.
//!
//! # Quick start
//!
//! ```ignore
//! let log = oslog::Log::new("com.myapp", "network");
//! log.info("connection established");
//! log.error("request failed");
//! log.debug("packet dump ...");
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use core::ffi::c_void;

pub fn is_available() -> bool { true }

// ── Raw os_log C API ────────────────────────────────────────────────────────

/// os_log_type_t values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogType {
    Default = 0x00,
    Info    = 0x01,
    Debug   = 0x02,
    Error   = 0x10,
    Fault   = 0x11,
}

// os_log_t is an OS_OBJECT (pointer-sized opaque type)
type OsLogT = *mut c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn os_log_create(subsystem: *const u8, category: *const u8) -> OsLogT;
    fn os_log_type_enabled(log: OsLogT, log_type: u8) -> bool;
    // The real os_log API uses _os_log_impl with format descriptors.
    // For simple string logging we use the %{public}s format.
    fn _os_log_impl(
        dso: *const c_void,
        log: OsLogT,
        log_type: u8,
        format: *const u8,
        buf: *const u8,
        size: u32,
    );
}

// The __dso_handle symbol is needed by _os_log_impl
unsafe extern "C" {
    static __dso_handle: c_void;
}

/// An os_log handle for a subsystem + category pair.
///
/// Create one per module/subsystem, reuse across the app lifetime.
pub struct Log {
    inner: OsLogT,
}

// os_log_t is internally refcounted and thread-safe
unsafe impl Send for Log {}
unsafe impl Sync for Log {}

impl Log {
    /// Create a new log handle.
    ///
    /// - `subsystem`: reverse-DNS identifier (e.g. `"com.myapp.networking"`)
    /// - `category`: category within the subsystem (e.g. `"connection"`)
    pub fn new(subsystem: &str, category: &str) -> Self {
        let mut sub_buf = Vec::with_capacity(subsystem.len() + 1);
        sub_buf.extend_from_slice(subsystem.as_bytes());
        sub_buf.push(0);
        let mut cat_buf = Vec::with_capacity(category.len() + 1);
        cat_buf.extend_from_slice(category.as_bytes());
        cat_buf.push(0);
        let inner = unsafe { os_log_create(sub_buf.as_ptr(), cat_buf.as_ptr()) };
        Self { inner }
    }

    /// Check if a given log type is enabled (respects Console.app filter settings).
    pub fn is_enabled(&self, log_type: LogType) -> bool {
        unsafe { os_log_type_enabled(self.inner, log_type as u8) }
    }

    /// Log a message at the given level.
    ///
    /// Uses `%{public}s` format so messages appear in Console.app.
    pub fn log(&self, log_type: LogType, message: &str) {
        // Build the os_log buffer descriptor for a single %{public}s argument.
        // Buffer format: [summary_byte, arg_count_byte, [arg_descriptor, arg_data]...]
        // For one public string: summary=2 (has_non_scalar), count=1,
        //   arg: type=0x22 (string|public), size=8, pointer (8 bytes)
        let mut msg_buf = Vec::with_capacity(message.len() + 1);
        msg_buf.extend_from_slice(message.as_bytes());
        msg_buf.push(0);
        let ptr = msg_buf.as_ptr();

        let mut buf = [0u8; 12];
        buf[0] = 2;  // summary: has non-scalar
        buf[1] = 1;  // arg count
        buf[2] = 0x22; // type: string | public
        buf[3] = 8;    // size of pointer
        // Copy pointer bytes
        let ptr_bytes = (ptr as usize).to_ne_bytes();
        buf[4..12].copy_from_slice(&ptr_bytes);

        unsafe {
            _os_log_impl(
                &__dso_handle as *const _ as *const c_void,
                self.inner,
                log_type as u8,
                b"%{public}s\0".as_ptr(),
                buf.as_ptr(),
                buf.len() as u32,
            );
        }
    }

    /// Log at default level.
    pub fn default(&self, message: &str) { self.log(LogType::Default, message); }
    /// Log at info level.
    pub fn info(&self, message: &str) { self.log(LogType::Info, message); }
    /// Log at debug level (may be suppressed unless enabled in Console.app).
    pub fn debug(&self, message: &str) { self.log(LogType::Debug, message); }
    /// Log at error level.
    pub fn error(&self, message: &str) { self.log(LogType::Error, message); }
    /// Log at fault level (captures backtraces).
    pub fn fault(&self, message: &str) { self.log(LogType::Fault, message); }
}

/// The system default log (no subsystem/category).
pub fn default_log() -> Log {
    // os_log_create with empty strings gives the default log
    Log::new("", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_log() {
        let log = Log::new("com.rswift.test", "unit-tests");
        assert!(log.is_enabled(LogType::Default));
        assert!(log.is_enabled(LogType::Error));
    }

    #[test]
    fn test_log_message() {
        let log = Log::new("com.rswift.test", "unit-tests");
        // Just ensure it doesn't crash — output visible in Console.app
        log.default("oslog test: default");
        log.info("oslog test: info");
        log.debug("oslog test: debug");
        log.error("oslog test: error");
    }

    #[test]
    fn test_default_log() {
        let log = default_log();
        log.info("oslog default log test");
    }
}
