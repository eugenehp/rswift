//! Apple Virtualization — virtual machines from Rust.
//!
//! **Platform:** macOS 11+.
//!
//! ```ignore
//! if virtualization::is_supported() {
//!     let config = virtualization::VirtualMachineConfiguration::new();
//!     config.set_cpu_count(4);
//!     config.set_memory_size(4 * 1024 * 1024 * 1024); // 4 GB
//! }
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }


/// Whether virtualization is supported on this hardware.
pub fn is_supported() -> bool {
    unsafe { msg_send_t![bool; class!(b"VZVirtualMachine\0"), isSupported] }
}

/// Maximum supported CPU count.
pub fn max_cpu_count() -> usize {
    unsafe {
        let sel = sel_registerName(b"maximumAllowedCPUCount\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel) -> usize =
            core::mem::transmute(objc_msgSend as *const ());
        f(class!(b"VZVirtualMachineConfiguration\0") as Id, sel)
    }
}

/// Minimum supported CPU count.
pub fn min_cpu_count() -> usize {
    unsafe {
        let sel = sel_registerName(b"minimumAllowedCPUCount\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel) -> usize =
            core::mem::transmute(objc_msgSend as *const ());
        f(class!(b"VZVirtualMachineConfiguration\0") as Id, sel)
    }
}

/// Maximum supported memory size in bytes.
pub fn max_memory_size() -> u64 {
    unsafe {
        let sel = sel_registerName(b"maximumAllowedMemorySize\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel) -> u64 =
            core::mem::transmute(objc_msgSend as *const ());
        f(class!(b"VZVirtualMachineConfiguration\0") as Id, sel)
    }
}

/// Minimum supported memory size in bytes.
pub fn min_memory_size() -> u64 {
    unsafe {
        let sel = sel_registerName(b"minimumAllowedMemorySize\0".as_ptr());
        let f: unsafe extern "C" fn(Id, Sel) -> u64 =
            core::mem::transmute(objc_msgSend as *const ());
        f(class!(b"VZVirtualMachineConfiguration\0") as Id, sel)
    }
}

/// Wraps `VZVirtualMachineConfiguration`.
pub struct VirtualMachineConfiguration { inner: Id }

impl VirtualMachineConfiguration {
    pub fn new() -> Self {
        Self { inner: unsafe { msg_send![class!(b"VZVirtualMachineConfiguration\0"), new] } }
    }

    pub fn set_cpu_count(&self, count: usize) {
        unsafe {
            let sel = sel_registerName(b"setCPUCount:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, usize) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, count);
        }
    }

    pub fn set_memory_size(&self, bytes: u64) {
        unsafe {
            let sel = sel_registerName(b"setMemorySize:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, u64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(self.inner, sel, bytes);
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), String> {
        unsafe {
            let mut error: Id = NIL;
            let sel = sel_registerName(b"validateWithError:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, *mut Id) -> bool =
                core::mem::transmute(objc_msgSend as *const ());
            if f(self.inner, sel, &mut error) {
                Ok(())
            } else {
                let desc = if !error.is_null() {
                    nsstring_to_string(msg_send![error, localizedDescription])
                        .unwrap_or_else(|| "Validation failed".into())
                } else { "Validation failed".into() };
                Err(desc)
            }
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Default for VirtualMachineConfiguration { fn default() -> Self { Self::new() } }
impl Drop for VirtualMachineConfiguration {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

/// Wraps `VZLinuxBootLoader`.
pub struct LinuxBootLoader { inner: Id }

impl LinuxBootLoader {
    pub fn new(kernel_path: &str) -> Self {
        unsafe {
            let ns = nsstring(kernel_path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            let bl: Id = msg_send![class!(b"VZLinuxBootLoader\0"), alloc];
            let bl = msg_send![bl, initWithKernelURL: url];
            Self { inner: bl }
        }
    }

    pub fn set_command_line(&self, cmdline: &str) {
        unsafe {
            let ns = nsstring(cmdline);
            msg_send_void![self.inner, setCommandLine: ns];
            CFRelease(ns as CFTypeRef);
        }
    }

    pub fn set_initial_ramdisk(&self, path: &str) {
        unsafe {
            let ns = nsstring(path);
            let url: Id = msg_send![class!(b"NSURL\0"), fileURLWithPath: ns];
            CFRelease(ns as CFTypeRef);
            msg_send_void![self.inner, setInitialRamdiskURL: url];
        }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

impl Drop for LinuxBootLoader {
    fn drop(&mut self) { unsafe { CFRelease(self.inner as CFTypeRef); } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported() {
        let s = is_supported();
        println!("Virtualization supported: {s}");
    }

    #[test]
    fn test_limits() {
        if !is_supported() { return; }
        let max_cpu = max_cpu_count();
        let min_cpu = min_cpu_count();
        let max_mem = max_memory_size();
        let min_mem = min_memory_size();
        println!("CPU: {min_cpu}–{max_cpu}, Memory: {}–{} GB",
            min_mem / (1024*1024*1024), max_mem / (1024*1024*1024));
        assert!(max_cpu >= min_cpu);
        assert!(max_mem >= min_mem);
    }

    #[test]
    fn test_config() {
        if !is_supported() { return; }
        let config = VirtualMachineConfiguration::new();
        config.set_cpu_count(2);
        config.set_memory_size(2 * 1024 * 1024 * 1024);
        // Without boot loader/storage this won't validate, which is expected
        let _ = config.validate();
    }
}
