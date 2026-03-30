//! Diagnostics — human-readable Swift type names for errors and debug output.

use core::ffi::c_void;

/// Demangle a Swift symbol name to human-readable form.
/// Falls back to the raw mangled name if demangling fails.
pub fn demangle(mangled: &str) -> String {
    // swift_demangle expects the $s prefix for type manglings
    let prefixed = if mangled.starts_with("$s") || mangled.starts_with("_$s") {
        mangled.to_string()
    } else {
        format!("$s{mangled}")
    };
    let Ok(cstr) = std::ffi::CString::new(prefixed.as_bytes()) else {
        return mangled.to_string();
    };
    swift_runtime::demangle::demangle(&cstr).unwrap_or_else(|| mangled.to_string())
}

/// Demangle a mangled type name byte slice (as used in our MANGLED constants).
pub fn demangle_type(mangled: &[u8]) -> String {
    let s = std::str::from_utf8(mangled).unwrap_or("<invalid utf8>");
    demangle(s)
}

/// Get the human-readable name of a Swift type from its metadata pointer.
pub fn type_name(metadata: *const c_void) -> String {
    if metadata.is_null() {
        return "<null>".to_string();
    }
    unsafe {
        match swift_runtime_sys::SwiftCCThunks::swift_getTypeName(metadata, true) {
            Ok((name, len)) if len > 0 => name.to_string(),
            _ => format!("<metadata {metadata:?}>"),
        }
    }
}

/// Demangle a dlsym symbol name for display in error messages.
pub fn symbol_name(sym: &core::ffi::CStr) -> String {
    let raw = sym.to_str().unwrap_or("<invalid>");
    let demangled = demangle(raw);
    if demangled == raw {
        raw.to_string()
    } else {
        format!("{demangled} ({raw})")
    }
}
