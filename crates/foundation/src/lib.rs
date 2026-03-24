#![allow(unsafe_op_in_unsafe_fn)]
//! Apple Foundation — core utilities from Rust.
//!
//! **Platform support:** all Apple platforms.
//!
//! Pure Rust ObjC message dispatch via `apple-objc-sys` — no `.m` thunks,
//! no `cc` build step, no Swift bridge.
//!
//! # Quick start
//!
//! ```ignore
//! foundation::UserDefaults::set_string("name", "Alice");
//! assert_eq!(foundation::UserDefaults::get_string("name"), Some("Alice".into()));
//!
//! let info = foundation::ProcessInfo;
//! println!("{} — {} cores", info.hostname(), info.processor_count());
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Foundation is always available on Apple platforms.
/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


// ── Internal helpers ────────────────────────────────────────────────────────

#[allow(dead_code)]
unsafe fn read_nsstring_buf(obj: Id, sel_name: &[u8]) -> Option<String> {
    let sel = sel_registerName(sel_name.as_ptr());
    let f: unsafe extern "C" fn(Id, Sel) -> Id = core::mem::transmute(objc_msgSend as *const ());
    let s = f(obj, sel);
    nsstring_to_string(s)
}

// ── UserDefaults ────────────────────────────────────────────────────────────

/// Persistent key-value storage (wraps `NSUserDefaults.standard`).
pub struct UserDefaults;

impl UserDefaults {
    unsafe fn defaults() -> Id {
        msg_send![class!(b"NSUserDefaults\0"), standardUserDefaults]
    }

    pub fn set_string(key: &str, value: &str) {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            let v = nsstring(value);
            msg_send_void![d, setObject: v, forKey: k];
            CFRelease(k as CFTypeRef);
            CFRelease(v as CFTypeRef);
        }
    }

    pub fn get_string(key: &str) -> Option<String> {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            let v = msg_send![d, stringForKey: k];
            CFRelease(k as CFTypeRef);
            nsstring_to_string(v)
        }
    }

    pub fn set_int(key: &str, value: i64) {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            msg_send_void![d, setInteger: value as isize, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    pub fn get_int(key: &str) -> Option<i64> {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            let has = msg_send![d, objectForKey: k];
            if has.is_null() { CFRelease(k as CFTypeRef); return None; }
            let v: isize = msg_send_t![isize; d, integerForKey: k];
            CFRelease(k as CFTypeRef);
            Some(v as i64)
        }
    }

    pub fn set_double(key: &str, value: f64) {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            // setDouble:forKey: — f64 arg
            let sel = sel_registerName(b"setDouble:forKey:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f(d, sel, value, k);
            CFRelease(k as CFTypeRef);
        }
    }

    pub fn get_double(key: &str) -> Option<f64> {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            let has = msg_send![d, objectForKey: k];
            if has.is_null() { CFRelease(k as CFTypeRef); return None; }
            let sel = sel_registerName(b"doubleForKey:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            let v = f(d, sel, k);
            CFRelease(k as CFTypeRef);
            Some(v)
        }
    }

    pub fn set_bool(key: &str, value: bool) {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            msg_send_void![d, setBool: value as u8, forKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    pub fn get_bool(key: &str) -> Option<bool> {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            let has = msg_send![d, objectForKey: k];
            if has.is_null() { CFRelease(k as CFTypeRef); return None; }
            let v: bool = msg_send_t![bool; d, boolForKey: k];
            CFRelease(k as CFTypeRef);
            Some(v)
        }
    }

    pub fn remove(key: &str) {
        unsafe {
            let d = Self::defaults();
            let k = nsstring(key);
            msg_send_void![d, removeObjectForKey: k];
            CFRelease(k as CFTypeRef);
        }
    }

    pub fn synchronize() -> bool {
        unsafe { msg_send_t![bool; Self::defaults(), synchronize] }
    }
}

// ── FileManager ─────────────────────────────────────────────────────────────

/// File system operations (wraps `FileManager.default`).
pub struct FileManager;

impl FileManager {
    unsafe fn default_mgr() -> Id {
        msg_send![class!(b"NSFileManager\0"), defaultManager]
    }

    pub fn file_exists(path: &str) -> bool {
        unsafe {
            let p = nsstring(path);
            let r = msg_send_t![bool; Self::default_mgr(), fileExistsAtPath: p];
            CFRelease(p as CFTypeRef);
            r
        }
    }

    pub fn is_directory(path: &str) -> bool {
        unsafe {
            let p = nsstring(path);
            let mut is_dir: u8 = 0; // BOOL out-param
            let sel = sel_registerName(b"fileExistsAtPath:isDirectory:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, *mut u8) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let exists = f(Self::default_mgr(), sel, p, &mut is_dir);
            CFRelease(p as CFTypeRef);
            exists && is_dir != 0
        }
    }

    pub fn create_directory(path: &str) -> bool {
        unsafe {
            let p = nsstring(path);
            let sel = sel_registerName(
                b"createDirectoryAtPath:withIntermediateDirectories:attributes:error:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, Id, bool, Id, Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            let r = f(Self::default_mgr(), sel, p, true, NIL, NIL);
            CFRelease(p as CFTypeRef);
            r
        }
    }

    pub fn remove(path: &str) -> bool {
        unsafe {
            let p = nsstring(path);
            let r = msg_send_t![bool; Self::default_mgr(), removeItemAtPath: p, error: NIL];
            CFRelease(p as CFTypeRef);
            r
        }
    }

    pub fn copy(src: &str, dst: &str) -> bool {
        unsafe {
            let s = nsstring(src);
            let d = nsstring(dst);
            let r = msg_send_t![bool; Self::default_mgr(),
                copyItemAtPath: s, toPath: d, error: NIL];
            CFRelease(s as CFTypeRef);
            CFRelease(d as CFTypeRef);
            r
        }
    }

    pub fn rename(src: &str, dst: &str) -> bool {
        unsafe {
            let s = nsstring(src);
            let d = nsstring(dst);
            let r = msg_send_t![bool; Self::default_mgr(),
                moveItemAtPath: s, toPath: d, error: NIL];
            CFRelease(s as CFTypeRef);
            CFRelease(d as CFTypeRef);
            r
        }
    }

    pub fn list(path: &str) -> Option<Vec<String>> {
        unsafe {
            let p = nsstring(path);
            let arr = msg_send![Self::default_mgr(),
                contentsOfDirectoryAtPath: p, error: NIL];
            CFRelease(p as CFTypeRef);
            if arr.is_null() { return None; }
            let count: usize = msg_send_t![usize; arr, count];
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let item: Id = msg_send![arr, objectAtIndex: i];
                if let Some(s) = nsstring_to_string(item) {
                    result.push(s);
                }
            }
            Some(result)
        }
    }

    pub fn home_directory() -> String {
        unsafe {
            // NSHomeDirectory() — C function
            extern "C" { fn NSHomeDirectory() -> Id; }
            nsstring_to_string(NSHomeDirectory()).unwrap_or_default()
        }
    }

    pub fn temp_directory() -> String {
        unsafe {
            extern "C" { fn NSTemporaryDirectory() -> Id; }
            nsstring_to_string(NSTemporaryDirectory()).unwrap_or_default()
        }
    }

    pub fn file_size(path: &str) -> i64 {
        unsafe {
            let p = nsstring(path);
            let attrs = msg_send![Self::default_mgr(),
                attributesOfItemAtPath: p, error: NIL];
            CFRelease(p as CFTypeRef);
            if attrs.is_null() { return -1; }
            let size: u64 = msg_send_t![u64; attrs, fileSize];
            size as i64
        }
    }

    pub fn application_support_directory() -> Option<String> { search_path(14, 1) }
    pub fn caches_directory() -> Option<String> { search_path(13, 1) }
    pub fn documents_directory() -> Option<String> { search_path(9, 1) }
    pub fn downloads_directory() -> Option<String> { search_path(15, 1) }
    pub fn desktop_directory() -> Option<String> { search_path(12, 1) }
}

fn search_path(directory: usize, domain: usize) -> Option<String> {
    unsafe {
        let mgr = FileManager::default_mgr();
        let sel = sel_registerName(b"URLsForDirectory:inDomains:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, usize, usize) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let urls = f(mgr, sel, directory, domain);
        if urls.is_null() { return None; }
        let count: usize = msg_send_t![usize; urls, count];
        if count == 0 { return None; }
        let url: Id = msg_send![urls, firstObject];
        let path: Id = msg_send![url, path];
        nsstring_to_string(path)
    }
}

// ── ProcessInfo ─────────────────────────────────────────────────────────────

/// System and process information.
pub struct ProcessInfo;

/// Thermal state of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState { Nominal = 0, Fair = 1, Serious = 2, Critical = 3 }

impl ProcessInfo {
    unsafe fn info() -> Id {
        msg_send![class!(b"NSProcessInfo\0"), processInfo]
    }

    pub fn hostname() -> String {
        unsafe { nsstring_to_string(msg_send![Self::info(), hostName]).unwrap_or_default() }
    }

    pub fn os_version() -> (usize, usize, usize) {
        // NSOperatingSystemVersion is { NSInteger major, minor, patch }
        #[repr(C)]
        struct OSVer { major: isize, minor: isize, patch: isize }
        unsafe {
            let sel = sel_registerName(b"operatingSystemVersion\0".as_ptr());
            // On ARM64, small structs are returned in registers
            let f: unsafe extern "C" fn(Id, Sel) -> OSVer =
                core::mem::transmute(objc_msgSend as *const ());
            let v = f(Self::info(), sel);
            (v.major as usize, v.minor as usize, v.patch as usize)
        }
    }

    pub fn processor_count() -> usize {
        unsafe { msg_send_t![usize; Self::info(), processorCount] }
    }

    pub fn active_processor_count() -> usize {
        unsafe { msg_send_t![usize; Self::info(), activeProcessorCount] }
    }

    pub fn physical_memory() -> u64 {
        unsafe { msg_send_t![u64; Self::info(), physicalMemory] }
    }

    pub fn system_uptime() -> f64 {
        unsafe {
            let sel = sel_registerName(b"systemUptime\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel) -> f64 =
                core::mem::transmute(objc_msgSend as *const ());
            f(Self::info(), sel)
        }
    }

    pub fn process_name() -> String {
        unsafe { nsstring_to_string(msg_send![Self::info(), processName]).unwrap_or_default() }
    }

    pub fn thermal_state() -> ThermalState {
        unsafe {
            match msg_send_t![isize; Self::info(), thermalState] {
                1 => ThermalState::Fair,
                2 => ThermalState::Serious,
                3 => ThermalState::Critical,
                _ => ThermalState::Nominal,
            }
        }
    }

    pub fn is_low_power_mode() -> bool {
        unsafe { msg_send_t![bool; Self::info(), isLowPowerModeEnabled] }
    }
}

// ── Bundle ──────────────────────────────────────────────────────────────────

pub struct Bundle;

impl Bundle {
    unsafe fn main() -> Id { msg_send![class!(b"NSBundle\0"), mainBundle] }

    pub fn main_path() -> String {
        unsafe { nsstring_to_string(msg_send![Self::main(), bundlePath]).unwrap_or_default() }
    }
    pub fn resource_path() -> Option<String> {
        unsafe { nsstring_to_string(msg_send![Self::main(), resourcePath]) }
    }
    pub fn identifier() -> Option<String> {
        unsafe { nsstring_to_string(msg_send![Self::main(), bundleIdentifier]) }
    }
    pub fn info_string(key: &str) -> Option<String> {
        unsafe {
            let dict: Id = msg_send![Self::main(), infoDictionary];
            if dict.is_null() { return None; }
            let k = nsstring(key);
            let val = msg_send![dict, objectForKey: k];
            CFRelease(k as CFTypeRef);
            nsstring_to_string(val)
        }
    }
}

// ── Locale ──────────────────────────────────────────────────────────────────

pub struct Locale;

impl Locale {
    pub fn identifier() -> String {
        unsafe {
            let loc = msg_send![class!(b"NSLocale\0"), currentLocale];
            nsstring_to_string(msg_send![loc, localeIdentifier]).unwrap_or_default()
        }
    }
    pub fn preferred_languages() -> Vec<String> {
        unsafe {
            let arr = msg_send![class!(b"NSLocale\0"), preferredLanguages];
            let count: usize = msg_send_t![usize; arr, count];
            (0..count)
                .filter_map(|i| nsstring_to_string(msg_send![arr, objectAtIndex: i]))
                .collect()
        }
    }
    pub fn language() -> String {
        unsafe {
            let loc = msg_send![class!(b"NSLocale\0"), currentLocale];
            nsstring_to_string(msg_send![loc, languageCode]).unwrap_or_else(|| "en".into())
        }
    }
    pub fn region() -> String {
        unsafe {
            let loc = msg_send![class!(b"NSLocale\0"), currentLocale];
            nsstring_to_string(msg_send![loc, countryCode]).unwrap_or_default()
        }
    }
}

// ── UUID ────────────────────────────────────────────────────────────────────

pub fn uuid() -> String {
    unsafe {
        let u = msg_send![class!(b"NSUUID\0"), UUID];
        nsstring_to_string(msg_send![u, UUIDString]).unwrap_or_default()
    }
}

// ── Date / Time ─────────────────────────────────────────────────────────────

pub fn now() -> f64 {
    unsafe {
        let d = msg_send![class!(b"NSDate\0"), date];
        let sel = sel_registerName(b"timeIntervalSince1970\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel) -> f64 =
            core::mem::transmute(objc_msgSend as *const ());
        f(d, sel)
    }
}

pub fn format_date(timestamp: f64, format: &str) -> String {
    unsafe {
        let date_sel = sel_registerName(b"dateWithTimeIntervalSince1970:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, f64) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let date = f(class!(b"NSDate\0") as Id, date_sel, timestamp);

        let formatter = msg_send![class!(b"NSDateFormatter\0"), new];
        let fmt_str = nsstring(format);
        msg_send_void![formatter, setDateFormat: fmt_str];
        CFRelease(fmt_str as CFTypeRef);

        let result = msg_send![formatter, stringFromDate: date];
        nsstring_to_string(result).unwrap_or_default()
    }
}

pub fn timezone() -> String {
    unsafe {
        let tz = msg_send![class!(b"NSTimeZone\0"), localTimeZone];
        nsstring_to_string(msg_send![tz, name]).unwrap_or_default()
    }
}

pub fn timezone_offset() -> isize {
    unsafe {
        let tz = msg_send![class!(b"NSTimeZone\0"), localTimeZone];
        msg_send_t![isize; tz, secondsFromGMT]
    }
}

// ── JSON ────────────────────────────────────────────────────────────────────

pub fn is_valid_json(data: &[u8]) -> bool {
    unsafe {
        let nsdata = msg_send![class!(b"NSData\0"), dataWithBytes: data.as_ptr(), length: data.len()];
        let sel = sel_registerName(b"JSONObjectWithData:options:error:\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel, Id, usize, Id) -> Id =
            core::mem::transmute(objc_msgSend as *const ());
        let obj = f(class!(b"NSJSONSerialization\0") as Id, sel, nsdata, 0, NIL);
        !obj.is_null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_available() { assert!(is_available()); }

    #[test]
    fn test_uuid_unique() {
        let a = uuid();
        let b = uuid();
        assert_ne!(a, b);
        assert_eq!(a.len(), 36);
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
        assert!(ts > 1_700_000_000.0);
    }

    #[test]
    fn test_format_date() {
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
