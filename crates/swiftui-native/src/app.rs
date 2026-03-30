//! SwiftUI App lifecycle — pure Rust, no Swift compiler.
//!
//! Constructs a SwiftUI `App` conformance at runtime by building:
//! 1. A zero-size struct metadata (the "app type")
//! 2. An App protocol witness table with body.getter
//! 3. WindowGroup scene with a content closure
//!
//! Then calls `App.main()` which enters the SwiftUI event loop.
//!
//! # Example
//! ```ignore
//! use swiftui_native::{views, app};
//!
//! fn main() {
//!     app::run(|| views::text("Hello from pure Rust!"));
//! }
//! ```

use crate::abi;
use crate::resolve;
use crate::views::ViewHandle;
use core::ffi::c_void;

/// The content builder function type.
/// Called by SwiftUI each time the scene needs to render.
type ContentBuilder = Box<dyn Fn() -> ViewHandle>;

/// Global storage for the content builder (needed for the trampoline).
static mut CONTENT_BUILDER: Option<ContentBuilder> = None;

/// Launch a SwiftUI app with a Rust-built root view. **Never returns.**
///
/// The builder closure is called by SwiftUI to construct the root view.
/// It should return a `ViewHandle` (any view created via `views::*`).
///
/// ```ignore
/// app::run(|| {
///     let title = views::text("Hello!");
///     let padded = views::padding(&title, 20.0);
///     padded
/// });
/// ```
pub fn run(builder: impl Fn() -> ViewHandle + 'static) -> ! {
    unsafe {
        CONTENT_BUILDER = Some(Box::new(builder));
        launch_app();
    }
    std::process::exit(0);
}

/// The WindowGroup content closure trampoline.
/// Called by SwiftUI: writes an AnyView to the @out pointer.
///
/// Signature: `(@out AnyView, context: *const c_void) -> void`
unsafe extern "C" fn content_trampoline(result: *mut u64, _ctx: *const c_void) {
    let view = CONTENT_BUILDER.as_ref().expect("content builder not set")();
    // Write AnyView (8 bytes) to result
    *result = view.0;
    // Don't drop — ownership transferred to SwiftUI
    core::mem::forget(view);
}

/// Build the App witness table and call App.main().
unsafe fn launch_app() {
    let app_main = resolve::app_main();
    let anyview_meta = resolve::anyview_meta();
    let anyview_wt = resolve::anyview_wt();

    // ── 1. Build app type metadata (zero-size struct) ──────────────
    // Full metadata layout: [VWT ptr] [Kind=0x200] [Descriptor ptr]
    // We use the empty tuple VWT for zero-size types.
    let empty_vwt = resolve::sym(c"$sytWV");
    assert!(!empty_vwt.is_null(), "Empty tuple VWT not found");

    let meta_buf = libc::malloc(24) as *mut *const c_void;
    assert!(!meta_buf.is_null());
    core::ptr::write_bytes(meta_buf as *mut u8, 0, 24);
    *meta_buf.add(0) = empty_vwt;                       // VWT pointer
    *(meta_buf.add(1) as *mut usize) = 0x200;           // Kind = Struct
    *meta_buf.add(2) = core::ptr::null();                // Descriptor (null is ok for runtime-only)
    let app_meta = (meta_buf as *const u8).add(8) as *const c_void; // points to Kind field

    // ── 2. Get WindowGroup<AnyView> metadata + Scene WT ────────────
    let wg_ma = resolve::windowgroup_ma();
    type MetaAccessor = unsafe extern "C" fn(usize, *const c_void) -> *const c_void;
    let get_wg_meta: MetaAccessor = core::mem::transmute(wg_ma);
    let wg_meta = get_wg_meta(0, anyview_meta);

    let scene_proto = resolve::scene_proto();
    let wg_scene_wt = abi::get_view_wt_for_proto(wg_meta, scene_proto);
    assert!(!wg_scene_wt.is_null(), "WindowGroup<AnyView>:Scene WT not found");

    // ── 3. Build App:App witness table ─────────────────────────────
    // Layout (absolute pointers, instantiated):
    //   [0] = protocol conformance descriptor (null for dynamic)
    //   [1] = associated type Body metadata accessor
    //   [2] = associated conformance Body: Scene accessor
    //   [3] = body.getter witness function
    //   [4] = init!allocator witness function
    let wt_buf = libc::malloc(5 * 8) as *mut *const c_void;
    assert!(!wt_buf.is_null());
    core::ptr::write_bytes(wt_buf as *mut u8, 0, 5 * 8);

    // [0] conformance descriptor — null for runtime-constructed
    *wt_buf.add(0) = core::ptr::null();

    // [1] associated type Body metadata accessor
    // Returns WindowGroup<AnyView> metadata
    static WG_META_CACHE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    WG_META_CACHE.store(wg_meta as usize, std::sync::atomic::Ordering::Release);

    unsafe extern "C" fn body_type_accessor(
        _request: usize,
        _wtable: *const c_void,
    ) -> *const c_void {
        WG_META_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void
    }
    *wt_buf.add(1) = body_type_accessor as *const c_void;

    // [2] associated conformance Body: Scene accessor
    // Returns WindowGroup<AnyView>:Scene witness table
    static WG_SCENE_WT_CACHE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    WG_SCENE_WT_CACHE.store(wg_scene_wt as usize, std::sync::atomic::Ordering::Release);

    unsafe extern "C" fn body_scene_accessor(
        _assoc: *const c_void,
        _conforming: *const c_void,
        _wtable: *const c_void,
    ) -> *const c_void {
        WG_SCENE_WT_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void
    }
    *wt_buf.add(2) = body_scene_accessor as *const c_void;

    // [3] body.getter witness
    // Signature: (@out WindowGroup<AnyView>, @in_guaranteed Self) -> void
    // Constructs WindowGroup with our content closure
    static ANYVIEW_META_CACHE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static ANYVIEW_WT_CACHE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static WG_INIT_CACHE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static SCENE_BB_CACHE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static WG_META_FOR_BODY: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);
    static WG_SCENE_WT_FOR_BODY: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    ANYVIEW_META_CACHE.store(anyview_meta as usize, std::sync::atomic::Ordering::Release);
    ANYVIEW_WT_CACHE.store(anyview_wt as usize, std::sync::atomic::Ordering::Release);
    WG_INIT_CACHE.store(
        resolve::windowgroup_init() as usize,
        std::sync::atomic::Ordering::Release,
    );
    SCENE_BB_CACHE.store(
        resolve::scene_buildblock() as usize,
        std::sync::atomic::Ordering::Release,
    );
    WG_META_FOR_BODY.store(wg_meta as usize, std::sync::atomic::Ordering::Release);
    WG_SCENE_WT_FOR_BODY.store(wg_scene_wt as usize, std::sync::atomic::Ordering::Release);

    unsafe extern "C" fn body_getter(
        // x8 = @out result, x0 = @in_guaranteed self
        // But as witness_method, the real convention may differ.
        // From ASM: x8 = @out result ptr, other args are self (ignored for zero-size)
    ) {
        // We use a naked-like approach: read x8 (result ptr) from the register
        // and construct the WindowGroup there.
        let result_ptr: *mut u8;
        core::arch::asm!("", out("x8") result_ptr);

        let av_meta =
            ANYVIEW_META_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void;
        let av_wt = ANYVIEW_WT_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void;
        let wg_init = WG_INIT_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void;
        let scene_bb = SCENE_BB_CACHE.load(std::sync::atomic::Ordering::Acquire) as *const c_void;
        let wg_meta =
            WG_META_FOR_BODY.load(std::sync::atomic::Ordering::Acquire) as *const c_void;
        let wg_scene_wt =
            WG_SCENE_WT_FOR_BODY.load(std::sync::atomic::Ordering::Acquire) as *const c_void;

        // Allocate stack space for WindowGroup (96 bytes)
        let mut wg_buf = [0u8; 128];

        // Push generic context (AnyView.Type + AnyView:View WT) on stack
        // as the WindowGroup.init expects them there
        // Then call WindowGroup.init(id:nil, title:nil, lazyContent: trampoline)

        // WindowGroup.init calling convention (from ASM):
        // x0=nil(id), x1=nil, x2=nil(title), x3=nil, x4=nil, x5=nil,
        // x6=closure_fn, x7=nil(closure_ctx),
        // x8=@out result, [sp]=AnyView.Type, [sp+8]=AnyView:View WT
        //
        // Actually from the ASM, the generic context is pushed BEFORE the call:
        //   stp x9, x8, [sp, #-16]!  where x9=AnyView.Type, x8=AnyView:View WT
        // So we need to push these on the stack manually.

        core::arch::asm!(
            "stp {av_meta}, {av_wt}, [sp, #-16]!",  // push generic context
            "blr {wg_init}",
            "add sp, sp, #16",                        // pop generic context
            wg_init = in(reg) wg_init,
            av_meta = in(reg) av_meta,
            av_wt = in(reg) av_wt,
            in("x0") 0u64,  // id: nil
            in("x1") 0u64,
            in("x2") 0u64,  // title: nil
            in("x3") 0u64,
            in("x4") 0u64,
            in("x5") 0u64,
            in("x6") content_trampoline as *const c_void, // lazyContent fn
            in("x7") 0u64,  // lazyContent context (nil = no captures)
            in("x8") wg_buf.as_mut_ptr(),  // @out result
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            lateout("x20") _, lateout("x21") _,
            clobber_abi("C"),
        );

        // Call SceneBuilder.buildBlock<WindowGroup<AnyView>>(wg_buf) -> result_ptr
        // x0 = @in_guaranteed value, x1 = type metadata, x8 = @out
        // x2 = WT for Scene conformance
        core::arch::asm!(
            "blr {scene_bb}",
            scene_bb = in(reg) scene_bb,
            in("x0") wg_buf.as_ptr(),
            in("x1") wg_meta,
            in("x2") wg_scene_wt,
            in("x8") result_ptr,
            lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            lateout("x20") _, lateout("x21") _,
            clobber_abi("C"),
        );
    }
    *wt_buf.add(3) = body_getter as *const c_void;

    // [4] init!allocator witness — no-op for zero-size struct
    unsafe extern "C" fn init_allocator(result: *mut u8, _metatype: *const c_void) {
        // Zero-size struct: just zero the output (nothing to do)
        let _ = result;
    }
    *wt_buf.add(4) = init_allocator as *const c_void;

    let app_wt = wt_buf as *const c_void;

    // ── 4. Call App.main() ─────────────────────────────────────────
    // x0 = x20 = A.Type metadata, x1 = A:App witness table
    core::arch::asm!(
        "blr {main}",
        main = in(reg) app_main,
        in("x0") app_meta,
        in("x1") app_wt,
        in("x20") app_meta,
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _,
        lateout("x20") _, lateout("x21") _,
        clobber_abi("C"),
    );
}


