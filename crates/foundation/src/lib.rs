//! Apple Foundation — core utilities from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! Provides Rust access to UserDefaults, FileManager, ProcessInfo, Bundle,
//! Locale, UUID, Date formatting, and JSON validation.
//! Linked automatically via `build.rs` — no manual setup needed.
//!
//! # Quick start
//!
//! ```ignore
//! // UserDefaults
//! foundation::UserDefaults::set_string("name", "Alice");
//! assert_eq!(foundation::UserDefaults::get_string("name"), Some("Alice".into()));
//!
//! // System info
//! let info = foundation::ProcessInfo;
//! println!("{} — {} cores, {} GB RAM",
//!     info.hostname(),
//!     info.processor_count(),
//!     info.physical_memory() / 1_073_741_824);
//!
//! // File operations
//! assert!(foundation::FileManager::file_exists("/etc/hosts"));
//!
//! // UUID
//! println!("{}", foundation::uuid());
//! ```

//!
//! ## Citation
//!
//! ```bibtex
//! @software{rswift,
//!   author       = {Eugene Hauptmann},
//!   title        = {rswift},
//!   year         = {2025},
//!   url          = {https://github.com/eugenehp/rswift},
//!   note         = {Build native Apple apps from Rust}
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

apple_sys_helpers::apple_framework!(c"foundation_available");

// ── FFI ─────────────────────────────────────────────────────────────────────

unsafe extern "C" {
    // UserDefaults
    fn foundation_userdefaults_set_string(k: *const u8, kl: usize, v: *const u8, vl: usize);
    fn foundation_userdefaults_get_string(k: *const u8, kl: usize, buf: *mut u8, bl: usize) -> isize;
    fn foundation_userdefaults_set_int(k: *const u8, kl: usize, v: i64);
    fn foundation_userdefaults_get_int(k: *const u8, kl: usize, out: *mut i64) -> bool;
    fn foundation_userdefaults_set_double(k: *const u8, kl: usize, v: f64);
    fn foundation_userdefaults_get_double(k: *const u8, kl: usize, out: *mut f64) -> bool;
    fn foundation_userdefaults_set_bool(k: *const u8, kl: usize, v: bool);
    fn foundation_userdefaults_get_bool(k: *const u8, kl: usize, out: *mut bool) -> bool;
    fn foundation_userdefaults_remove(k: *const u8, kl: usize);
    fn foundation_userdefaults_synchronize() -> bool;

    // FileManager
    fn foundation_filemanager_file_exists(p: *const u8, l: usize) -> bool;
    fn foundation_filemanager_is_directory(p: *const u8, l: usize) -> bool;
    fn foundation_filemanager_create_directory(p: *const u8, l: usize) -> bool;
    fn foundation_filemanager_remove_item(p: *const u8, l: usize) -> bool;
    fn foundation_filemanager_copy_item(s: *const u8, sl: usize, d: *const u8, dl: usize) -> bool;
    fn foundation_filemanager_move_item(s: *const u8, sl: usize, d: *const u8, dl: usize) -> bool;
    fn foundation_filemanager_contents_of_directory(p: *const u8, l: usize, buf: *mut u8, bl: usize) -> isize;
    fn foundation_filemanager_home_directory(buf: *mut u8, bl: usize) -> isize;
    fn foundation_filemanager_temp_directory(buf: *mut u8, bl: usize) -> isize;
    fn foundation_filemanager_file_size(p: *const u8, l: usize) -> i64;

    // ProcessInfo
    fn foundation_processinfo_hostname(buf: *mut u8, bl: usize) -> isize;
    fn foundation_processinfo_os_version(major: *mut isize, minor: *mut isize, patch: *mut isize);
    fn foundation_processinfo_processor_count() -> isize;
    fn foundation_processinfo_active_processor_count() -> isize;
    fn foundation_processinfo_physical_memory() -> u64;
    fn foundation_processinfo_system_uptime() -> f64;
    fn foundation_processinfo_process_name(buf: *mut u8, bl: usize) -> isize;
    fn foundation_processinfo_thermal_state() -> isize;
    fn foundation_processinfo_is_low_power_mode() -> bool;

    // Bundle
    fn foundation_bundle_main_path(buf: *mut u8, bl: usize) -> isize;
    fn foundation_bundle_resource_path(buf: *mut u8, bl: usize) -> isize;
    fn foundation_bundle_identifier(buf: *mut u8, bl: usize) -> isize;
    fn foundation_bundle_info_string(k: *const u8, kl: usize, buf: *mut u8, bl: usize) -> isize;

    // UUID
    fn foundation_uuid_generate(buf: *mut u8, bl: usize) -> isize;

    // Locale
    fn foundation_locale_current_identifier(buf: *mut u8, bl: usize) -> isize;
    fn foundation_locale_preferred_languages(buf: *mut u8, bl: usize) -> isize;
    fn foundation_locale_current_language(buf: *mut u8, bl: usize) -> isize;
    fn foundation_locale_current_region(buf: *mut u8, bl: usize) -> isize;

    // Date
    fn foundation_date_now() -> f64;
    fn foundation_date_format(ts: f64, fmt: *const u8, fl: usize, buf: *mut u8, bl: usize) -> isize;
    fn foundation_timezone_current(buf: *mut u8, bl: usize) -> isize;
    fn foundation_timezone_offset() -> isize;

    // JSON
    fn foundation_json_valid(d: *const u8, dl: usize) -> bool;

    // URL search paths
    fn foundation_url_search_path(dir: isize, domain: isize, buf: *mut u8, bl: usize) -> isize;
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn read_string(f: unsafe extern "C" fn(*mut u8, usize) -> isize) -> Option<String> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe { f(buf.as_mut_ptr(), buf.len()) };
    if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into()) }
}

fn read_string_key(f: unsafe extern "C" fn(*const u8, usize, *mut u8, usize) -> isize, key: &str) -> Option<String> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe { f(key.as_ptr(), key.len(), buf.as_mut_ptr(), buf.len()) };
    if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into()) }
}

// ── UserDefaults ────────────────────────────────────────────────────────────

/// Persistent key-value storage (wraps `NSUserDefaults.standard`).
pub struct UserDefaults;

impl UserDefaults {
    /// Store a string value.
    pub fn set_string(key: &str, value: &str) {
        unsafe { foundation_userdefaults_set_string(key.as_ptr(), key.len(), value.as_ptr(), value.len()) }
    }

    /// Read a string value. Returns `None` if the key doesn't exist.
    pub fn get_string(key: &str) -> Option<String> {
        read_string_key(foundation_userdefaults_get_string, key)
    }

    /// Store an integer value.
    pub fn set_int(key: &str, value: i64) {
        unsafe { foundation_userdefaults_set_int(key.as_ptr(), key.len(), value) }
    }

    /// Read an integer value.
    pub fn get_int(key: &str) -> Option<i64> {
        let mut v = 0i64;
        if unsafe { foundation_userdefaults_get_int(key.as_ptr(), key.len(), &mut v) } { Some(v) } else { None }
    }

    /// Store a floating-point value.
    pub fn set_double(key: &str, value: f64) {
        unsafe { foundation_userdefaults_set_double(key.as_ptr(), key.len(), value) }
    }

    /// Read a floating-point value.
    pub fn get_double(key: &str) -> Option<f64> {
        let mut v = 0.0f64;
        if unsafe { foundation_userdefaults_get_double(key.as_ptr(), key.len(), &mut v) } { Some(v) } else { None }
    }

    /// Store a boolean value.
    pub fn set_bool(key: &str, value: bool) {
        unsafe { foundation_userdefaults_set_bool(key.as_ptr(), key.len(), value) }
    }

    /// Read a boolean value.
    pub fn get_bool(key: &str) -> Option<bool> {
        let mut v = false;
        if unsafe { foundation_userdefaults_get_bool(key.as_ptr(), key.len(), &mut v) } { Some(v) } else { None }
    }

    /// Remove a key.
    pub fn remove(key: &str) {
        unsafe { foundation_userdefaults_remove(key.as_ptr(), key.len()) }
    }

    /// Force synchronize to disk.
    pub fn synchronize() -> bool {
        unsafe { foundation_userdefaults_synchronize() }
    }
}

// ── FileManager ─────────────────────────────────────────────────────────────

/// File system operations (wraps `FileManager.default`).
pub struct FileManager;

impl FileManager {
    /// Check if a file or directory exists.
    pub fn file_exists(path: &str) -> bool {
        unsafe { foundation_filemanager_file_exists(path.as_ptr(), path.len()) }
    }

    /// Check if a path is a directory.
    pub fn is_directory(path: &str) -> bool {
        unsafe { foundation_filemanager_is_directory(path.as_ptr(), path.len()) }
    }

    /// Create a directory (with intermediate directories).
    pub fn create_directory(path: &str) -> bool {
        unsafe { foundation_filemanager_create_directory(path.as_ptr(), path.len()) }
    }

    /// Remove a file or directory.
    pub fn remove(path: &str) -> bool {
        unsafe { foundation_filemanager_remove_item(path.as_ptr(), path.len()) }
    }

    /// Copy a file or directory.
    pub fn copy(src: &str, dst: &str) -> bool {
        unsafe { foundation_filemanager_copy_item(src.as_ptr(), src.len(), dst.as_ptr(), dst.len()) }
    }

    /// Move/rename a file or directory.
    pub fn rename(src: &str, dst: &str) -> bool {
        unsafe { foundation_filemanager_move_item(src.as_ptr(), src.len(), dst.as_ptr(), dst.len()) }
    }

    /// List directory contents. Returns entries separated by newlines.
    pub fn list(path: &str) -> Option<Vec<String>> {
        let mut buf = vec![0u8; 65536];
        let len = unsafe { foundation_filemanager_contents_of_directory(path.as_ptr(), path.len(), buf.as_mut_ptr(), buf.len()) };
        if len < 0 { return None; }
        let s = String::from_utf8_lossy(&buf[..len as usize]);
        Some(s.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect())
    }

    /// Current user's home directory.
    pub fn home_directory() -> String {
        read_string(foundation_filemanager_home_directory).unwrap_or_default()
    }

    /// System temporary directory.
    pub fn temp_directory() -> String {
        read_string(foundation_filemanager_temp_directory).unwrap_or_default()
    }

    /// File size in bytes (-1 on error).
    pub fn file_size(path: &str) -> i64 {
        unsafe { foundation_filemanager_file_size(path.as_ptr(), path.len()) }
    }

    /// Application Support directory for the current user.
    pub fn application_support_directory() -> Option<String> {
        search_path(14, 1) // NSApplicationSupportDirectory, NSUserDomainMask
    }

    /// Caches directory for the current user.
    pub fn caches_directory() -> Option<String> {
        search_path(13, 1) // NSCachesDirectory, NSUserDomainMask
    }

    /// Documents directory for the current user.
    pub fn documents_directory() -> Option<String> {
        search_path(9, 1) // NSDocumentDirectory, NSUserDomainMask
    }

    /// Downloads directory for the current user.
    pub fn downloads_directory() -> Option<String> {
        search_path(15, 1) // NSDownloadsDirectory, NSUserDomainMask
    }

    /// Desktop directory for the current user.
    pub fn desktop_directory() -> Option<String> {
        search_path(12, 1) // NSDesktopDirectory, NSUserDomainMask
    }
}

fn search_path(dir: isize, domain: isize) -> Option<String> {
    let mut buf = vec![0u8; 4096];
    let len = unsafe { foundation_url_search_path(dir, domain, buf.as_mut_ptr(), buf.len()) };
    if len < 0 { None } else { Some(String::from_utf8_lossy(&buf[..len as usize]).into()) }
}

// ── ProcessInfo ─────────────────────────────────────────────────────────────

/// System and process information (wraps `ProcessInfo.processInfo`).
pub struct ProcessInfo;

/// Thermal state of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Nominal = 0,
    Fair = 1,
    Serious = 2,
    Critical = 3,
}

impl ProcessInfo {
    /// Machine hostname.
    pub fn hostname() -> String {
        read_string(foundation_processinfo_hostname).unwrap_or_default()
    }

    /// Operating system version as (major, minor, patch).
    pub fn os_version() -> (usize, usize, usize) {
        let (mut ma, mut mi, mut pa) = (0isize, 0isize, 0isize);
        unsafe { foundation_processinfo_os_version(&mut ma, &mut mi, &mut pa) }
        (ma as usize, mi as usize, pa as usize)
    }

    /// Total number of processors.
    pub fn processor_count() -> usize {
        unsafe { foundation_processinfo_processor_count() as usize }
    }

    /// Number of active processors.
    pub fn active_processor_count() -> usize {
        unsafe { foundation_processinfo_active_processor_count() as usize }
    }

    /// Physical memory in bytes.
    pub fn physical_memory() -> u64 {
        unsafe { foundation_processinfo_physical_memory() }
    }

    /// System uptime in seconds.
    pub fn system_uptime() -> f64 {
        unsafe { foundation_processinfo_system_uptime() }
    }

    /// Current process name.
    pub fn process_name() -> String {
        read_string(foundation_processinfo_process_name).unwrap_or_default()
    }

    /// Current thermal state.
    pub fn thermal_state() -> ThermalState {
        match unsafe { foundation_processinfo_thermal_state() } {
            1 => ThermalState::Fair,
            2 => ThermalState::Serious,
            3 => ThermalState::Critical,
            _ => ThermalState::Nominal,
        }
    }

    /// Whether Low Power Mode is active.
    pub fn is_low_power_mode() -> bool {
        unsafe { foundation_processinfo_is_low_power_mode() }
    }
}

// ── Bundle ──────────────────────────────────────────────────────────────────

/// Main application bundle (wraps `Bundle.main`).
pub struct Bundle;

impl Bundle {
    /// Bundle path.
    pub fn main_path() -> String {
        read_string(foundation_bundle_main_path).unwrap_or_default()
    }

    /// Resource directory path.
    pub fn resource_path() -> Option<String> {
        read_string(foundation_bundle_resource_path)
    }

    /// Bundle identifier (e.g. `com.example.app`).
    pub fn identifier() -> Option<String> {
        read_string(foundation_bundle_identifier)
    }

    /// Read a string from Info.plist.
    pub fn info_string(key: &str) -> Option<String> {
        read_string_key(foundation_bundle_info_string, key)
    }
}

// ── Locale ──────────────────────────────────────────────────────────────────

/// Current locale information (wraps `Locale.current`).
pub struct Locale;

impl Locale {
    /// Current locale identifier (e.g. `en_US`).
    pub fn identifier() -> String {
        read_string(foundation_locale_current_identifier).unwrap_or_default()
    }

    /// Preferred language codes (e.g. `["en", "fr"]`).
    pub fn preferred_languages() -> Vec<String> {
        read_string(foundation_locale_preferred_languages)
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Current language code (e.g. `en`).
    pub fn language() -> String {
        read_string(foundation_locale_current_language).unwrap_or_default()
    }

    /// Current region code (e.g. `US`).
    pub fn region() -> String {
        read_string(foundation_locale_current_region).unwrap_or_default()
    }
}

// ── UUID ────────────────────────────────────────────────────────────────────

/// Generate a new UUID string.
pub fn uuid() -> String {
    read_string(foundation_uuid_generate).unwrap_or_default()
}

// ── Date / Time ─────────────────────────────────────────────────────────────

/// Current Unix timestamp (seconds since 1970).
pub fn now() -> f64 {
    unsafe { foundation_date_now() }
}

/// Format a timestamp using a date format string (e.g. `"yyyy-MM-dd HH:mm:ss"`).
pub fn format_date(timestamp: f64, format: &str) -> String {
    let mut buf = vec![0u8; 256];
    let len = unsafe {
        foundation_date_format(timestamp, format.as_ptr(), format.len(), buf.as_mut_ptr(), buf.len())
    };
    if len < 0 { String::new() } else { String::from_utf8_lossy(&buf[..len as usize]).into() }
}

/// Current timezone identifier (e.g. `America/New_York`).
pub fn timezone() -> String {
    read_string(foundation_timezone_current).unwrap_or_default()
}

/// Current timezone offset from GMT in seconds.
pub fn timezone_offset() -> isize {
    unsafe { foundation_timezone_offset() }
}

// ── JSON ────────────────────────────────────────────────────────────────────

/// Check if a byte slice is valid JSON.
pub fn is_valid_json(data: &[u8]) -> bool {
    unsafe { foundation_json_valid(data.as_ptr(), data.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_available() {
        assert!(is_available());
    }

    #[test]
    fn test_uuid_unique() {
        let a = uuid();
        let b = uuid();
        assert_ne!(a, b);
        assert_eq!(a.len(), 36); // UUID format: 8-4-4-4-12
    }

    #[test]
    fn test_userdefaults_string() {
        UserDefaults::set_string("_rswift_test_s", "hello");
        assert_eq!(UserDefaults::get_string("_rswift_test_s"), Some("hello".into()));
        UserDefaults::remove("_rswift_test_s");
        assert_eq!(UserDefaults::get_string("_rswift_test_s"), None);
    }

    #[test]
    fn test_userdefaults_int() {
        UserDefaults::set_int("_rswift_test_i", 42);
        assert_eq!(UserDefaults::get_int("_rswift_test_i"), Some(42));
        UserDefaults::remove("_rswift_test_i");
    }

    #[test]
    fn test_userdefaults_double() {
        UserDefaults::set_double("_rswift_test_d", 3.14);
        let v = UserDefaults::get_double("_rswift_test_d").unwrap();
        assert!((v - 3.14).abs() < 0.001);
        UserDefaults::remove("_rswift_test_d");
    }

    #[test]
    fn test_userdefaults_bool() {
        UserDefaults::set_bool("_rswift_test_b", true);
        assert_eq!(UserDefaults::get_bool("_rswift_test_b"), Some(true));
        UserDefaults::remove("_rswift_test_b");
    }

    #[test]
    fn test_filemanager_exists() {
        assert!(FileManager::file_exists("/etc/hosts"));
        assert!(!FileManager::file_exists("/nonexistent_path_xyz"));
    }

    #[test]
    fn test_filemanager_is_directory() {
        assert!(FileManager::is_directory("/tmp"));
        assert!(!FileManager::is_directory("/etc/hosts"));
    }

    #[test]
    fn test_filemanager_directories() {
        assert!(!FileManager::home_directory().is_empty());
        assert!(!FileManager::temp_directory().is_empty());
        assert!(FileManager::documents_directory().is_some());
    }

    #[test]
    fn test_processinfo() {
        assert!(!ProcessInfo::hostname().is_empty());
        let (major, _, _) = ProcessInfo::os_version();
        assert!(major > 0);
        assert!(ProcessInfo::processor_count() > 0);
        assert!(ProcessInfo::physical_memory() > 0);
        assert!(ProcessInfo::system_uptime() > 0.0);
    }

    #[test]
    fn test_locale() {
        assert!(!Locale::identifier().is_empty());
        assert!(!Locale::language().is_empty());
    }

    #[test]
    fn test_date_now() {
        let ts = now();
        assert!(ts > 1_700_000_000.0); // After 2023
    }

    #[test]
    fn test_format_date() {
        // Use a timestamp that's the same year in any timezone
        let s = format_date(1_700_000_000.0, "yyyy");
        assert_eq!(s, "2023");
    }

    #[test]
    fn test_json_valid() {
        assert!(is_valid_json(b"{}"));
        assert!(is_valid_json(b"[1,2,3]"));
        assert!(is_valid_json(b"{\"a\":1}"));
        assert!(!is_valid_json(b"not json"));
        assert!(!is_valid_json(b""));
    }

    #[test]
    fn test_filemanager_create_remove() {
        let dir = format!("{}/rswift_test_{}", FileManager::temp_directory(), std::process::id());
        assert!(FileManager::create_directory(&dir));
        assert!(FileManager::is_directory(&dir));
        assert!(FileManager::remove(&dir));
        assert!(!FileManager::file_exists(&dir));
    }
}
