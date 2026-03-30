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
/// AnyView.init<V:View>(@in V, @thin AnyView.Type) -> @owned AnyView
/// x0 = ptr to V (consumed), x1 = AnyView.Type, x2 = V:View WT
/// x20 = V.Type metadata
/// Returns x0 = AnyView (8-byte class ref, owned)
pub unsafe fn anyview_wrap(
    value_ptr: *const c_void,
    value_meta: *const c_void,
    value_view_wt: *const c_void,
) -> u64 {
    let func = resolve::anyview_init();
    let anyview_meta = resolve::anyview_meta();
    // For @in parameters, even zero-size types need a valid (non-null) pointer.
    // Allocate a small stack buffer for the value.
    let vwt = &*swift_runtime_sys::SwiftABI::get_value_witness_table(value_meta);
    let mut stack_buf = [0u8; 128];
    let actual_ptr = if vwt.size == 0 {
        stack_buf.as_ptr() as *const c_void
    } else {
        // Copy value into stack buffer if it fits, otherwise use original ptr
        if vwt.size <= 128 {
            core::ptr::copy_nonoverlapping(
                value_ptr as *const u8,
                stack_buf.as_mut_ptr(),
                vwt.size,
            );
            stack_buf.as_ptr() as *const c_void
        } else {
            value_ptr
        }
    };
    let result: u64;
    core::arch::asm!(
        "blr {func}",
        func = in(reg) func,
        in("x0") actual_ptr,     // @in V (pointer to value, consumed)
        in("x1") anyview_meta,   // @thin AnyView.Type
        in("x2") value_view_wt,  // V:View witness table
        in("x20") value_meta,    // V.Type (generic metadata)
        lateout("x0") result,
        lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _,
        clobber_abi("C"),
    );
    result
}

/// Call a generic View modifier on an AnyView.
/// Pattern: (Self.Type, Self:View WT, @out result, @in_guaranteed self, args...) → @out result
/// Returns the raw bytes of the result (ModifiedContent<AnyView, Modifier>).
pub unsafe fn call_modifier_d0(
    func: *const c_void,
    anyview: u64,
    d0_arg: f64,
    result_size: usize,
) -> Vec<u8> {
    let mut result = vec![0u8; result_size];
    let av_meta = resolve::anyview_meta();
    let av_wt = resolve::anyview_wt();
    let mut self_buf = anyview.to_le_bytes();
    core::arch::asm!(
        "blr {func}",
        func = in(reg) func,
        in("x0") av_meta,           // Self.Type
        in("x1") av_wt,             // Self:View WT
        in("x8") result.as_mut_ptr(), // @out result
        in("x20") self_buf.as_ptr(), // @in_guaranteed self
        in("d0") d0_arg,            // Double/CGFloat arg
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _,
        clobber_abi("C"),
    );
    result
}

/// Get the View:View witness table for ModifiedContent<AnyView, M> from the runtime.
pub unsafe fn get_view_wt(meta: *const c_void) -> *const c_void {
    let proto = resolve::view_proto();
    type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    let f: ConformsFn = core::mem::transmute(
        libc::dlsym((-2isize) as *mut c_void, c"swift_conformsToProtocol".as_ptr())
    );
    f(meta, proto)
}

/// Instantiate a concrete type from a mangled name (needed for ModifiedContent metadata).
pub unsafe fn instantiate_type(mangled_ptr: *const u8) -> *const c_void {
    type F = unsafe extern "C" fn(*const u8) -> *const c_void;
    let f: F = core::mem::transmute(
        libc::dlsym(
            (-2isize) as *mut c_void,
            c"__swift_instantiateConcreteTypeFromMangledName".as_ptr(),
        )
    );
    f(mangled_ptr)
}
