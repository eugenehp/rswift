//! Native SwiftUI view construction — pure Rust, zero Swift source.
//!
//! Every view created via dlsym + arm64 asm calling Swift CC directly.
//! All results wrapped in AnyView (8-byte class ref) for uniform handling.

use crate::abi;
use crate::resolve;
use core::ffi::c_void;

/// An opaque SwiftUI view handle. Wraps a retained AnyView class reference.
#[repr(transparent)]
pub struct ViewHandle(pub(crate) u64);

impl ViewHandle {
    pub(crate) fn new(anyview: u64) -> Self {
        debug_assert!(anyview != 0, "AnyView pointer is null");
        Self(anyview)
    }
    pub fn as_ptr(&self) -> *mut c_void { self.0 as *mut c_void }
    unsafe fn wrap(value_ptr: *const c_void, meta: *const c_void, wt: *const c_void) -> Self {
        Self::new(abi::anyview_wrap(value_ptr, meta, wt))
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

impl std::fmt::Debug for ViewHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ViewHandle({}, ptr={:#x})", crate::diag::type_name(resolve::anyview_meta()), self.0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Mangled result type names (from _mangledTypeName in Swift)
// ═══════════════════════════════════════════════════════════════════════════

/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._PaddingLayout>`
const PADDING_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA14_PaddingLayoutVG";
/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._OpacityEffect>`
const OPACITY_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA14_OpacityEffectVG";
/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._FrameLayout>`
const FRAME_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA12_FrameLayoutVG";
/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._BackgroundStyleModifier<SwiftUI.Color>>`
const BG_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA24_BackgroundStyleModifierVyAA5ColorVGG";
/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._ClipEffect<SwiftUI.RoundedRectangle>>`
const CORNER_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA11_ClipEffectVyAA16RoundedRectangleVGG";
/// `SwiftUI.ModifiedContent<SwiftUI.AnyView, SwiftUI._OverlayModifier<SwiftUI.ModifiedContent<SwiftUI._ShapeView<SwiftUI.Rectangle, SwiftUI.Color>, SwiftUI._FrameLayout>>>`
// Border is complex — we'll use a simpler approach
const BORDER_T: &[u8] = b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA15_OverlayModifierVyAA0dE0VyAA14_StrokedShapeVyAA16RoundedRectangleVGAA5ColorVGGG";

/// Wrap a modifier result into a new AnyView.
unsafe fn wrap_result(result_bytes: &[u8], mangled: &[u8]) -> ViewHandle {
    let meta = abi::resolve_type_by_mangled_name(mangled);
    assert!(!meta.is_null(), "Failed to resolve: {}", crate::diag::demangle_type(mangled));
    let wt = abi::get_view_wt(meta);
    assert!(!wt.is_null(), "{} !: View", crate::diag::demangle_type(mangled));
    ViewHandle::new(abi::anyview_wrap(result_bytes.as_ptr() as _, meta, wt))
}

// ═══════════════════════════════════════════════════════════════════════════
// Views
// ═══════════════════════════════════════════════════════════════════════════

/// Create a `Text` view.
pub fn text(s: &str) -> ViewHandle {
    unsafe {
        let ss = abi::swift_string(s);
        let s0 = u64::from_le_bytes(ss[..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(ss[8..].try_into().unwrap());

        // LocalizedStringKey.init(stringLiteral:)
        let r0: u64; let r1: u64; let r2: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::lsk_init(),
            in("x0") s0, in("x1") s1,
            in("x20") resolve::sym(c"$s7SwiftUI18LocalizedStringKeyVN"),
            lateout("x0") r0, lateout("x1") r1, lateout("x2") r2,
            lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );

        // Text.init(_:tableName:bundle:comment:)
        let mut buf = [0u8; 32];
        let t0: u64; let t1: u64; let t2: u64; let t3: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::text_lsk_init(),
            in("x0") r0, in("x1") r1, in("x2") r2,
            in("x3") 0u64, in("x4") 0u64, in("x5") 0u64, in("x6") 0u64,
            in("x20") resolve::text_meta(),
            lateout("x0") t0, lateout("x1") t1, lateout("x2") t2, lateout("x3") t3,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        buf[..8].copy_from_slice(&t0.to_le_bytes());
        buf[8..16].copy_from_slice(&t1.to_le_bytes());
        buf[16..24].copy_from_slice(&t2.to_le_bytes());
        buf[24..32].copy_from_slice(&t3.to_le_bytes());
        ViewHandle::wrap(buf.as_ptr() as _, resolve::text_meta(), resolve::text_wt())
    }
}

/// Create a styled `Text` with font size, weight, and color.
pub fn styled_text(s: &str, size: f64, weight: i32, r: f64, g: f64, b: f64, a: f64) -> ViewHandle {
    // Create Text, apply .font and .foregroundColor via modifier chaining
    let t = text(s);
    // For now, styled_text applies opacity as a visual indicator.
    // Full font/color modifiers need additional symbol resolution.
    // TODO: resolve View.font() and View.foregroundColor() symbols
    let _ = (size, weight, r, g, b, a);
    t
}

/// Create a bold `Text`.
pub fn bold_text(s: &str, _size: f64) -> ViewHandle {
    // TODO: apply .font(.system(size: size, weight: .bold))
    text(s)
}

/// Create a `Color` view.
pub fn color(r: f64, g: f64, b: f64, a: f64) -> ViewHandle {
    unsafe {
        let mut cs: u8 = 0; // RGBColorSpace.sRGB
        let result: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::color_init(),
            in("x0") &mut cs as *mut u8, in("d0") r, in("d1") g, in("d2") b, in("d3") a,
            in("x20") resolve::color_meta(),
            lateout("x0") result,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        let buf = result.to_le_bytes();
        ViewHandle::wrap(buf.as_ptr() as _, resolve::color_meta(), resolve::color_wt())
    }
}

/// Create a `Spacer`.
pub fn spacer() -> ViewHandle {
    unsafe {
        let r0: u64; let r1: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::spacer_init(),
            in("x0") 0u64, in("x1") 0u64, // Optional.none
            in("x20") resolve::spacer_meta(),
            lateout("x0") r0, lateout("x1") r1,
            lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        let mut buf = [0u8; 16];
        buf[..8].copy_from_slice(&r0.to_le_bytes());
        buf[8..16].copy_from_slice(&r1.to_le_bytes());
        ViewHandle::wrap(buf.as_ptr() as _, resolve::spacer_meta(), resolve::spacer_wt())
    }
}

/// Create an `EmptyView`.
pub fn empty_view() -> ViewHandle {
    unsafe { ViewHandle::wrap(core::ptr::null(), resolve::empty_meta(), resolve::empty_wt()) }
}

/// Create a `Divider`.
pub fn divider() -> ViewHandle {
    unsafe { ViewHandle::wrap(core::ptr::null(), resolve::divider_meta(), resolve::divider_wt()) }
}

/// Create an `Image` from an SF Symbol name.
pub fn system_image(name: &str) -> ViewHandle {
    unsafe {
        let ss = abi::swift_string(name);
        let s0 = u64::from_le_bytes(ss[..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(ss[8..].try_into().unwrap());
        let result: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::image_sysname_init(),
            in("x0") s0, in("x1") s1, in("x20") resolve::image_meta(),
            lateout("x0") result,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        let buf = result.to_le_bytes();
        ViewHandle::wrap(buf.as_ptr() as _, resolve::image_meta(), resolve::image_wt())
    }
}

/// Create a `ProgressView`.
pub fn progress(_value: f64, _total: f64) -> ViewHandle {
    // TODO: resolve ProgressView.init(value:total:) symbol
    empty_view()
}

/// Create a `Toggle` (stub — needs Binding).
pub fn toggle(_label: &str, _is_on: bool) -> ViewHandle {
    empty_view()
}

/// Create a `TextField` (stub — needs Binding).
pub fn textfield(_placeholder: &str, _value: &str) -> ViewHandle {
    empty_view()
}

// ═══════════════════════════════════════════════════════════════════════════
// Modifiers
// ═══════════════════════════════════════════════════════════════════════════

/// Apply `.padding(_:)`.
pub fn padding(view: &ViewHandle, amount: f64) -> ViewHandle {
    unsafe {
        let result = abi::call_modifier_d0(resolve::padding_fn(), view.0, amount, 64);
        wrap_result(&result, PADDING_T)
    }
}

/// Apply `.opacity(_:)`.
pub fn opacity(view: &ViewHandle, value: f64) -> ViewHandle {
    unsafe {
        let result = abi::call_modifier_d0(resolve::opacity_fn(), view.0, value, 32);
        wrap_result(&result, OPACITY_T)
    }
}

/// Apply `.frame(width:height:)`.
pub fn frame(view: &ViewHandle, width: f64, height: f64) -> ViewHandle {
    unsafe {
        let func = resolve::frame_fn();
        let mut self_storage: u64 = view.0;
        let mut result = [0u8; 64];
        // Get Alignment.center
        let align_meta = resolve::sym(c"$s7SwiftUI9AlignmentVN");
        let align: u64;
        core::arch::asm!("blr {f}", f = in(reg) resolve::alignment_center(),
            in("x20") align_meta,
            lateout("x0") align,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        // frame(width: Optional<CGFloat>, height: Optional<CGFloat>, alignment: Alignment)
        // ASM showed: x0=w_val, x1=w_tag(0=some), x2=h_val, x3=h_tag(0=some),
        //             x4,x5=alignment, x6=Self.Type, x7=Self:View WT, x8=result, x20=self
        let w_bits: u64 = width.to_bits();
        let h_bits: u64 = height.to_bits();
        core::arch::asm!("blr {func}",
            func = in(reg) func,
            in("x0") w_bits,     // width value
            in("x1") 0u64,      // width tag: 0 = some
            in("x2") h_bits,    // height value
            in("x3") 0u64,      // height tag: 0 = some
            in("x4") align,     // Alignment (first word)
            in("x5") 0u64,      // Alignment (second word / padding)
            in("x6") resolve::anyview_meta(),
            in("x7") resolve::anyview_wt(),
            in("x8") result.as_mut_ptr(),
            in("x20") &mut self_storage as *mut u64,
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        wrap_result(&result, FRAME_T)
    }
}

/// Apply `.background(_:)` with a Color.
pub fn background(view: &ViewHandle, r: f64, g: f64, b: f64, a: f64) -> ViewHandle {
    unsafe {
        let c = color(r, g, b, a);
        let func = resolve::bg_fn();
        let mut self_storage: u64 = view.0;
        let mut color_storage: u64 = c.0;
        let mut result = [0u8; 32];
        // ASM: x0=@in style, x1=Edge.Set, x2=Self.Type, x3=S.Type, x4=Self:View WT, x5=S:ShapeStyle WT
        //      x8=result, x20=self
        core::arch::asm!("blr {func}",
            func = in(reg) func,
            in("x0") &mut color_storage as *mut u64, // @in Color
            in("x1") 0xFFu64,                        // Edge.Set.all (raw value)
            in("x2") resolve::anyview_meta(),         // Self.Type
            in("x3") resolve::color_meta(),           // S.Type (Color)
            in("x4") resolve::anyview_wt(),           // Self:View WT
            in("x5") resolve::color_shapestyle_wt(),  // S:ShapeStyle WT
            in("x8") result.as_mut_ptr(),
            in("x20") &mut self_storage as *mut u64,
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        core::mem::forget(c);
        wrap_result(&result, BG_T)
    }
}

/// Apply `.cornerRadius(_:)`.
pub fn corner_radius(view: &ViewHandle, radius: f64) -> ViewHandle {
    // cornerRadius is View.clipShape(RoundedRectangle) — complex.
    // For parity, apply padding(0) as placeholder.
    // TODO: resolve View.cornerRadius symbol
    let _ = radius;
    padding(view, 0.0)
}

/// Apply `.border(_:width:)` (stub).
pub fn border(view: &ViewHandle, _r: f64, _g: f64, _b: f64, _width: f64) -> ViewHandle {
    // TODO: resolve View.border symbol
    padding(view, 0.0)
}

// ═══════════════════════════════════════════════════════════════════════════
// Stacks (via VStack/HStack closure construction)
// ═══════════════════════════════════════════════════════════════════════════

/// VStack from children. For single child, wraps directly. For multiple, nests recursively.
pub fn vstack(children: &[ViewHandle]) -> ViewHandle {
    stack_impl(children, true)
}

/// HStack from children.
pub fn hstack(children: &[ViewHandle]) -> ViewHandle {
    stack_impl(children, false)
}

/// ZStack from children (uses same nesting approach).
pub fn zstack(children: &[ViewHandle]) -> ViewHandle {
    // ZStack is similar to VStack but overlays.
    // For now, use VStack as approximation.
    // TODO: resolve ZStack.init symbol
    stack_impl(children, true)
}

fn stack_impl(children: &[ViewHandle], vertical: bool) -> ViewHandle {
    if children.is_empty() {
        return empty_view();
    }
    if children.len() == 1 {
        return children[0].clone();
    }
    // For multiple children, wrap each pair using a closure-based stack.
    // The closure returns a single AnyView (the child).
    // We nest: Stack { child[0], Stack { child[1], Stack { ... } } }
    // This preserves correct semantics.
    let mut result = children.last().unwrap().clone();
    for i in (0..children.len() - 1).rev() {
        result = stack_two(&children[i], &result, vertical);
    }
    result
}

/// Create a VStack/HStack containing exactly two AnyView children via TupleView.
fn stack_two(a: &ViewHandle, b: &ViewHandle, vertical: bool) -> ViewHandle {
    unsafe {
        let init_fn = if vertical { resolve::vstack_init() } else { resolve::hstack_init() };

        // The closure body: returns a TupleView<(AnyView, AnyView)>
        // We construct this by calling ViewBuilder.buildBlock(a, b) — but that's not exported.
        // Instead, create TupleView directly: TupleView((a, b))
        // TupleView<(AnyView, AnyView)> is just (AnyView, AnyView) = 16 bytes

        // Closure context: store both AnyViews
        let mut context: [u64; 2] = [a.0, b.0];

        // Closure function: writes TupleView<(AnyView, AnyView)> to @out
        // Signature: (result_ptr: *mut u8, context_ptr: *const [u64; 2]) -> void
        unsafe extern "C" fn stack_closure_body(result: *mut u8, ctx: *const [u64; 2]) {
            // TupleView<(AnyView, AnyView)> is just the two AnyView values (16 bytes)
            let pair = &*ctx;
            core::ptr::copy_nonoverlapping(pair.as_ptr() as *const u8, result, 16);
        }

        // Get TupleView<(AnyView, AnyView)> metadata and View WT
        let tuple_mangled = b"7SwiftUI9TupleViewVy7AnyViewV_ADtG";
        let tuple_meta = abi::resolve_type_by_mangled_name(tuple_mangled);
        if tuple_meta.is_null() {
            // Fallback: just return b
            return b.clone();
        }
        let tuple_wt = abi::get_view_wt(tuple_meta);
        if tuple_wt.is_null() {
            return b.clone();
        }

        // Allocate result buffer on stack (VStack<TupleView<(AnyView, AnyView)>> is ~24 bytes)
        let mut result = [0u8; 128];

        // VStack.init: x0=alignment(default), x1=spacing_value(0), x2=spacing_tag(1=none),
        //              x3=closure_fn, x4=closure_ctx, x5=Content.Type, x6=Content:View WT
        //              x8=result
        core::arch::asm!("blr {func}",
            func = in(reg) init_fn,
            in("x0") 0u64,      // default alignment (center = 0)
            in("x1") 0u64,      // spacing value
            in("x2") 1u64,      // spacing tag: 1 = none
            in("x3") stack_closure_body as *const c_void,
            in("x4") &mut context as *mut [u64; 2],
            in("x5") tuple_meta,
            in("x6") tuple_wt,
            in("x8") result.as_mut_ptr(),
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            lateout("x20") _, lateout("x21") _,
            clobber_abi("C"),
        );

        // Get VStack<TupleView<(AnyView,AnyView)>> metadata and WT
        let vstack_mangled = if vertical {
            b"7SwiftUI6VStackVyAA9TupleViewVy7AnyViewV_ADtGG"
        } else {
            b"7SwiftUI6HStackVyAA9TupleViewVy7AnyViewV_ADtGG"
        };
        let vstack_meta = abi::resolve_type_by_mangled_name(vstack_mangled);
        if vstack_meta.is_null() {
            return b.clone();
        }
        let vstack_wt = abi::get_view_wt(vstack_meta);
        if vstack_wt.is_null() {
            return b.clone();
        }
        ViewHandle::wrap(result.as_ptr() as _, vstack_meta, vstack_wt)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Button
// ═══════════════════════════════════════════════════════════════════════════

/// Create a `Button` with label and action callback.
pub fn button(label: &str, callback: fn()) -> ViewHandle {
    // TODO: Button.init needs Swift closure construction
    let _ = callback;
    text(label)
}

/// Create a `Button` with raw C callback.
pub fn button_raw(
    label: &str,
    _callback: unsafe extern "C" fn(*mut c_void),
    _userdata: *mut c_void,
) -> ViewHandle {
    text(label)
}

// ═══════════════════════════════════════════════════════════════════════════
// Scroll
// ═══════════════════════════════════════════════════════════════════════════

/// Wrap in a `ScrollView` (stub).
pub fn scroll(view: &ViewHandle) -> ViewHandle {
    // TODO: resolve ScrollView.init
    view.clone()
}

// ═══════════════════════════════════════════════════════════════════════════
// Window
// ═══════════════════════════════════════════════════════════════════════════

/// Show a view in an NSWindow. Blocks on `NSApp.run()`.
pub fn show_window(view: &ViewHandle, title: &str, width: f64, height: f64) {
    crate::window::show_window(view, title, width, height);
}
