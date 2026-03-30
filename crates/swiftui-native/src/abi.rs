//! Swift ABI helpers — arm64 asm thunks for calling SwiftUI from Rust.
//!
//! All functions use Swift calling convention on aarch64:
//! - Generic Self metadata in x20, witness tables in x21+
//! - Indirect results via x8
//! - Float/Double args in d0-d7

use core::ffi::c_void;
use crate::resolve;

/// Create a Swift.String from a Rust &str.
pub unsafe fn swift_string(s: &str) -> [u8; 16] {
    swift_runtime_sys::SwiftUIBridge::create_swift_string(s)
        .expect("Failed to create Swift.String")
}

/// Wrap any view value in AnyView.
///
/// ARM64 calling convention (from SIL + ASM analysis):
///   x0 = pointer to V value (@in, consumed) — may be arbitrary for zero-size types
///   x1 = V.Type metadata (generic type parameter)
///   x2 = V:View witness table
///   Returns: x0 = AnyView (8-byte class ref, owned)
pub unsafe fn anyview_wrap(
    value_ptr: *const c_void,
    value_meta: *const c_void,
    value_view_wt: *const c_void,
) -> u64 {
    let func = resolve::anyview_init();
    let vwt = &*swift_runtime_sys::SwiftABI::get_value_witness_table(value_meta);
    // For @in: need a valid pointer even for zero-size types.
    let mut stack_buf = [0u8; 128];
    let actual_ptr = if vwt.size == 0 {
        stack_buf.as_mut_ptr() as *const c_void
    } else if vwt.size <= 128 {
        core::ptr::copy_nonoverlapping(
            value_ptr as *const u8, stack_buf.as_mut_ptr(), vwt.size,
        );
        stack_buf.as_mut_ptr() as *const c_void
    } else {
        value_ptr
    };
    let result: u64;
    core::arch::asm!(
        "blr {func}",
        func = in(reg) func,
        in("x0") actual_ptr,      // @in V (pointer to value)
        in("x1") value_meta,      // V.Type (generic metadata)
        in("x2") value_view_wt,   // V:View witness table
        lateout("x0") result,
        lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _,
        lateout("x20") _, lateout("x21") _,
        clobber_abi("C"),
    );
    result
}

/// Call a generic View modifier on an AnyView.
///
/// ARM64 calling convention (from ASM analysis of View.padding):
///   x0  = Self.Type (AnyView metadata)
///   x1  = Self:View WT (AnyView:View witness table)
///   x8  = @out result pointer
///   x20 = @in_guaranteed self pointer (points to AnyView 8-byte value on stack)
///   d0  = Double/CGFloat argument
///
/// Returns raw bytes of result (ModifiedContent<AnyView, Modifier>).
pub unsafe fn call_modifier_d0(
    func: *const c_void,
    anyview: u64,
    d0_arg: f64,
    result_size: usize,
) -> Vec<u8> {
    let mut result = vec![0u8; result_size];
    let av_meta = resolve::anyview_meta();
    let av_wt = resolve::anyview_wt();
    // Store AnyView value on stack and point x20 to it
    let mut self_storage: u64 = anyview;
    core::arch::asm!(
        "blr {func}",
        func = in(reg) func,
        in("x0") av_meta,                          // Self.Type
        in("x1") av_wt,                             // Self:View WT
        in("x8") result.as_mut_ptr(),                // @out result
        in("x20") &mut self_storage as *mut u64,     // @in_guaranteed self
        in("d0") d0_arg,                             // Double/CGFloat arg
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _,
        clobber_abi("C"),
    );
    result
}

/// Get the View witness table for a type from the runtime.
pub unsafe fn get_view_wt(meta: *const c_void) -> *const c_void {
    get_view_wt_for_proto(meta, resolve::view_proto())
}

/// Get a protocol witness table for a type via `swift_conformsToProtocol`.
pub unsafe fn get_view_wt_for_proto(meta: *const c_void, proto: *const c_void) -> *const c_void {
    type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    let f: ConformsFn = core::mem::transmute(
        libc::dlsym((-2isize) as *mut c_void, c"swift_conformsToProtocol".as_ptr())
    );
    f(meta, proto)
}

/// Resolve a Swift type from its mangled name string.
/// Uses swift_getTypeByMangledNameInEnvironment from the Swift runtime.
pub unsafe fn resolve_type_by_mangled_name(mangled: &[u8]) -> *const c_void {
    swift_runtime_sys::RuntimeRaw::swift_getTypeByMangledNameInEnvironment(
        mangled.as_ptr(),
        mangled.len(),
        core::ptr::null(),
        0,
    )
}
