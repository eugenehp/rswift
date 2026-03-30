//! Native view construction — pure Rust, zero Swift source.
//!
//! Every view is created via dlsym + arm64 asm calling Swift CC directly.
//! Results are wrapped in AnyView (8-byte class ref) for uniform handling.

use crate::abi;
use crate::resolve;
use core::ffi::c_void;

/// An opaque SwiftUI view handle. Wraps a retained AnyView class reference.
///
/// Created by the native view constructors. Automatically released on drop.
#[repr(transparent)]
pub struct ViewHandle(pub(crate) u64);

impl ViewHandle {
    fn new(anyview: u64) -> Self {
        debug_assert!(anyview != 0, "AnyView pointer is null");
        Self(anyview)
    }

    /// Raw pointer for interop.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0 as *mut c_void
    }

    /// Wrap a concrete view value into AnyView and return a handle.
    unsafe fn wrap(
        value_ptr: *const c_void,
        meta: *const c_void,
        view_wt: *const c_void,
    ) -> Self {
        Self::new(abi::anyview_wrap(value_ptr, meta, view_wt))
    }
}

impl Clone for ViewHandle {
    fn clone(&self) -> Self {
        unsafe { swift_runtime_sys::RuntimeRaw::swift_retain(self.0 as *mut c_void) };
        Self(self.0)
    }
}

impl Drop for ViewHandle {
    fn drop(&mut self) {
        unsafe { swift_runtime_sys::RuntimeRaw::swift_release(self.0 as *mut c_void) };
    }
}

unsafe impl Send for ViewHandle {}

// ═══════════════════════════════════════════════════════════════════════════
// Views
// ═══════════════════════════════════════════════════════════════════════════

/// Create a `Text` view.
pub fn text(s: &str) -> ViewHandle {
    unsafe {
        let ss = abi::swift_string(s);
        // LocalizedStringKey.init(stringLiteral: String)
        let lsk_fn = resolve::lsk_init();
        let lsk_meta = resolve::sym(c"$s7SwiftUI18LocalizedStringKeyVN");
        let s0 = u64::from_le_bytes(ss[..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(ss[8..].try_into().unwrap());
        let r0: u64; let r1: u64; let r2: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) lsk_fn,
            in("x0") s0, in("x1") s1, in("x20") lsk_meta,
            lateout("x0") r0, lateout("x1") r1, lateout("x2") r2,
            lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        // Text.init(_:tableName:bundle:comment:)
        let text_fn = resolve::text_lsk_init();
        let text_meta = resolve::text_meta();
        let mut text_buf = [0u8; 64]; // Text is 32 bytes
        let t0: u64; let t1: u64; let t2: u64; let t3: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) text_fn,
            in("x0") r0, in("x1") r1, in("x2") r2, // LSK
            in("x3") 0u64, in("x4") 0u64, in("x5") 0u64, in("x6") 0u64, // nil args
            in("x20") text_meta,
            lateout("x0") t0, lateout("x1") t1, lateout("x2") t2, lateout("x3") t3,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        text_buf[..8].copy_from_slice(&t0.to_le_bytes());
        text_buf[8..16].copy_from_slice(&t1.to_le_bytes());
        text_buf[16..24].copy_from_slice(&t2.to_le_bytes());
        text_buf[24..32].copy_from_slice(&t3.to_le_bytes());

        ViewHandle::wrap(text_buf.as_ptr() as _, text_meta, resolve::text_wt())
    }
}

/// Create a `Color` view.
pub fn color(r: f64, g: f64, b: f64, a: f64) -> ViewHandle {
    unsafe {
        let func = resolve::color_init();
        let meta = resolve::color_meta();
        // RGBColorSpace.sRGB = enum case 0, 1 byte
        let mut colorspace: u8 = 0; // sRGB
        let result: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) func,
            in("x0") &mut colorspace as *mut u8, // @in RGBColorSpace
            in("d0") r, in("d1") g, in("d2") b, in("d3") a,
            in("x20") meta, // @thin Color.Type (not used but convention)
            lateout("x0") result,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        // Color is 8 bytes (class ref), result already in x0
        let buf = result.to_le_bytes();
        ViewHandle::wrap(buf.as_ptr() as _, meta, resolve::color_wt())
    }
}

/// Create a `Spacer`.
pub fn spacer() -> ViewHandle {
    unsafe {
        let func = resolve::spacer_init();
        let meta = resolve::spacer_meta();
        // Spacer.init(minLength: Optional<CGFloat>)
        // Optional.none = 0 (8 bytes value) + 0 (1 byte tag for none)
        let r0: u64; let r1: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) func,
            in("x0") 0u64, // value (ignored for none)
            in("x1") 0u64, // tag = 0 = none
            in("x20") meta,
            lateout("x0") r0, lateout("x1") r1,
            lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        // Spacer is 9 bytes = Optional<CGFloat>
        let mut buf = [0u8; 16];
        buf[..8].copy_from_slice(&r0.to_le_bytes());
        buf[8..16].copy_from_slice(&r1.to_le_bytes());
        ViewHandle::wrap(buf.as_ptr() as _, meta, resolve::spacer_wt())
    }
}

/// Create an `EmptyView`.
pub fn empty_view() -> ViewHandle {
    unsafe {
        // EmptyView is zero-size, init is a no-op
        ViewHandle::wrap(core::ptr::null(), resolve::empty_meta(), resolve::empty_wt())
    }
}

/// Create a `Divider`.
pub fn divider() -> ViewHandle {
    unsafe {
        let func = resolve::divider_init();
        let meta = resolve::divider_meta();
        let r0: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) func,
            in("x20") meta,
            lateout("x0") r0,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        let buf = r0.to_le_bytes();
        ViewHandle::wrap(buf.as_ptr() as _, meta, resolve::divider_wt())
    }
}

/// Create an `Image` from an SF Symbol name.
pub fn system_image(name: &str) -> ViewHandle {
    unsafe {
        let ss = abi::swift_string(name);
        let func = resolve::image_sysname_init();
        let meta = resolve::image_meta();
        let s0 = u64::from_le_bytes(ss[..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(ss[8..].try_into().unwrap());
        let result: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) func,
            in("x0") s0, in("x1") s1, in("x20") meta,
            lateout("x0") result,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        let buf = result.to_le_bytes();
        ViewHandle::wrap(buf.as_ptr() as _, meta, resolve::image_wt())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Modifiers (operate on AnyView, return new AnyView)
// ═══════════════════════════════════════════════════════════════════════════

// Mangled type name strings for ModifiedContent<AnyView, Modifier> result types.
// Obtained from _mangledTypeName() in Swift. Used with swift_getTypeByMangledNameInEnvironment.
const PADDING_MANGLED: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA14_PaddingLayoutVG";
const OPACITY_MANGLED: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA14_OpacityEffectVG";
const FRAME_MANGLED: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA12_FrameLayoutVG";
const BG_MANGLED: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA24_BackgroundStyleModifierVyAA5ColorVGG";

/// Apply `.padding(_:)` modifier.
pub fn padding(view: &ViewHandle, amount: f64) -> ViewHandle {
    unsafe {
        let result = abi::call_modifier_d0(resolve::padding_fn(), view.0, amount, 64);
        wrap_modifier_result_typed(&result, PADDING_MANGLED)
    }
}

/// Apply `.opacity(_:)` modifier.
pub fn opacity(view: &ViewHandle, value: f64) -> ViewHandle {
    unsafe {
        let result = abi::call_modifier_d0(resolve::opacity_fn(), view.0, value, 32);
        wrap_modifier_result_typed(&result, OPACITY_MANGLED)
    }
}

/// Apply `.frame(width:height:alignment:)` modifier.
pub fn frame(view: &ViewHandle, width: f64, height: f64) -> ViewHandle {
    unsafe {
        let func = resolve::frame_fn();
        let av_meta = resolve::anyview_meta();
        let av_wt = resolve::anyview_wt();
        let mut self_buf = view.0.to_le_bytes();
        let mut result = [0u8; 64];
        // Optional<CGFloat>.some(value) = value(8 bytes) + tag=1(1 byte)
        // Alignment.center - get it
        let align_fn = resolve::alignment_center();
        let align: u64;
        core::arch::asm!(
            "blr {f}", f = in(reg) align_fn,
            in("x20") resolve::sym(c"$s7SwiftUI9AlignmentVN"),
            lateout("x0") align,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        // frame(width: Optional<CGFloat>, height: Optional<CGFloat>, alignment: Alignment)
        // For Optional<CGFloat>.some(v): pass as (value, 1) on stack or registers
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") av_meta,
            in("x1") av_wt,
            in("x8") result.as_mut_ptr(),
            in("x20") self_buf.as_ptr(),
            in("d0") width,      // Optional<CGFloat> width value
            in("x2") 1u64,      // width tag = some
            in("d1") height,     // Optional<CGFloat> height value
            in("x3") 1u64,      // height tag = some
            in("x4") align,     // Alignment.center
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        wrap_modifier_result_typed(&result, FRAME_MANGLED)
    }
}

/// Apply `.background(_:)` with a Color.
pub fn background(view: &ViewHandle, r: f64, g: f64, b: f64, a: f64) -> ViewHandle {
    unsafe {
        let c = color(r, g, b, a);
        let func = resolve::bg_fn();
        let av_meta = resolve::anyview_meta();
        let av_wt = resolve::anyview_wt();
        let color_meta = resolve::color_meta();
        let color_ss_wt = resolve::color_shapestyle_wt();
        let mut self_buf = view.0.to_le_bytes();
        let color_buf = c.0.to_le_bytes();
        let mut result = [0u8; 64];
        // Edge.Set.all = raw value, 1 byte
        let edge_all: u8 = 0xFF; // All edges
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") av_meta,             // Self.Type
            in("x1") av_wt,              // Self:View WT
            in("x2") color_meta,         // S.Type (ShapeStyle type)
            in("x3") color_ss_wt,        // S:ShapeStyle WT
            in("x8") result.as_mut_ptr(), // @out result
            in("x20") self_buf.as_ptr(), // @in_guaranteed self (AnyView)
            in("x21") color_buf.as_ptr(), // @in style (Color)
            in("x4") edge_all as u64,    // Edge.Set.all
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        // Don't drop c — consumed by the modifier
        core::mem::forget(c);
        wrap_modifier_result_typed(&result, BG_MANGLED)
    }
}

/// Wrap a modifier result (ModifiedContent<AnyView, M>) into a new AnyView.
///
/// Takes the result bytes + the mangled type name string.
/// Uses swift_getTypeByMangledNameInEnvironment to get metadata,
/// then swift_conformsToProtocol to get the View WT.
unsafe fn wrap_modifier_result_typed(
    result_bytes: &[u8],
    mangled_name: &[u8],
) -> ViewHandle {
    let meta = abi::resolve_type_by_mangled_name(mangled_name);
    assert!(!meta.is_null(), "Failed to resolve modifier result type");
    let view_wt = abi::get_view_wt(meta);
    assert!(!view_wt.is_null(), "Failed to get View WT for modifier result");
    ViewHandle::new(abi::anyview_wrap(
        result_bytes.as_ptr() as *const c_void,
        meta,
        view_wt,
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// Window
// ═══════════════════════════════════════════════════════════════════════════

/// Show a view in an NSWindow via NSHostingController. Blocks on NSApp.run().
pub fn show_window(view: &ViewHandle, title: &str, width: f64, height: f64) {
    crate::window::show_window(view, title, width, height);
}
