//! Window hosting — pure Rust.
//!
//! - `NSHostingView.init(rootView:)` via **Swift CC** (same as Text, Color, etc.)
//! - `NSWindow` / `NSApplication` via **ObjC runtime** (their native calling convention)
//!
//! Both paths use the same dlsym + asm approach — no Swift source code.

use crate::resolve;
use crate::views::ViewHandle;
use core::ffi::{c_char, c_void};

// ═══════════════════════════════════════════════════════════════════════════
// ObjC runtime — the native calling convention for AppKit classes
// ═══════════════════════════════════════════════════════════════════════════

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn objc_msgSend(receiver: *mut c_void, sel: *const c_void, ...) -> *mut c_void;
    fn sel_registerName(name: *const c_char) -> *const c_void;
}

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

fn sel(n: &core::ffi::CStr) -> *const c_void {
    unsafe { sel_registerName(n.as_ptr()) }
}

fn cls(n: &core::ffi::CStr) -> *mut c_void {
    unsafe { dlsym(RTLD_DEFAULT, n.as_ptr()) as *mut c_void }
}

/// Create an NSString from a Rust &str via ObjC runtime.
unsafe fn nsstring(s: &str) -> *mut c_void {
    let cls = cls(c"OBJC_CLASS_$_NSString");
    let alloc: *mut c_void = objc_msgSend(cls, sel(c"alloc"));
    type InitBytes = unsafe extern "C" fn(*mut c_void, *const c_void, *const u8, u64, u64) -> *mut c_void;
    let f: InitBytes = core::mem::transmute(objc_msgSend as *const c_void);
    f(alloc, sel(c"initWithBytes:length:encoding:"), s.as_ptr(), s.len() as u64, 4)
}

// ═══════════════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════════════

/// Show an AnyView in an NSWindow. Blocks on `NSApp.run()`.
///
/// Uses **Swift CC** for `NSHostingView.init(rootView:)` (same mechanism as
/// Text, Color, etc.) and **ObjC runtime** for NSWindow/NSApplication
/// (their native calling convention — equivalent to Swift CC for Swift types).
pub fn show_window(view: &ViewHandle, title: &str, width: f64, height: f64) {
    unsafe {
        // ── 1. Create NSHostingView<AnyView> via Swift CC ──────────────
        let hv_init = resolve::hosting_view_init();
        let hv_ma = resolve::hosting_view_ma();

        // Get NSHostingView<AnyView> metatype
        type MetaAccessor = unsafe extern "C" fn(usize, *const c_void) -> *const c_void;
        let get_meta: MetaAccessor = core::mem::transmute(hv_ma);
        let hv_meta = get_meta(0, resolve::anyview_meta());

        // NSHostingView.init(rootView:) — Swift CC
        // x0 = @in AnyView ptr, x20 = @thick NSHostingView<AnyView>.Type
        let mut av_storage: u64 = view.0;
        let hosting_view: *mut c_void;
        core::arch::asm!("blr {f}", f = in(reg) hv_init,
            in("x0") &mut av_storage as *mut u64,
            in("x20") hv_meta,
            lateout("x0") hosting_view,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            lateout("x20") _, lateout("x21") _,
            clobber_abi("C"),
        );
        assert!(!hosting_view.is_null(), "NSHostingView creation failed");

        // ── 2. Create NSWindow via ObjC runtime ───────────────────────
        let app: *mut c_void = objc_msgSend(
            cls(c"OBJC_CLASS_$_NSApplication"),
            sel(c"sharedApplication"),
        );

        let win: *mut c_void = objc_msgSend(
            cls(c"OBJC_CLASS_$_NSWindow"),
            sel(c"alloc"),
        );

        #[repr(C)]
        struct NSRect { x: f64, y: f64, w: f64, h: f64 }

        type InitWin = unsafe extern "C" fn(
            *mut c_void, *const c_void, NSRect, u64, u64, bool,
        ) -> *mut c_void;
        let init_win: InitWin = core::mem::transmute(objc_msgSend as *const c_void);
        let win = init_win(
            win,
            sel(c"initWithContentRect:styleMask:backing:defer:"),
            NSRect { x: 0.0, y: 0.0, w: width, h: height },
            1 | 2 | 4 | 8, // titled | closable | resizable | miniaturizable
            2,              // buffered
            false,
        );

        // ── 3. Configure window via ObjC runtime ──────────────────────
        objc_msgSend(win, sel(c"setTitle:"), nsstring(title));
        objc_msgSend(win, sel(c"setContentView:"), hosting_view);
        objc_msgSend(win, sel(c"center"));
        objc_msgSend(win, sel(c"makeKeyAndOrderFront:"), core::ptr::null::<c_void>());

        // ── 4. Launch app via ObjC runtime ────────────────────────────
        type SetPolicy = unsafe extern "C" fn(*mut c_void, *const c_void, i64) -> bool;
        let set_policy: SetPolicy = core::mem::transmute(objc_msgSend as *const c_void);
        set_policy(app, sel(c"setActivationPolicy:"), 0);

        type Activate = unsafe extern "C" fn(*mut c_void, *const c_void, bool);
        let activate: Activate = core::mem::transmute(objc_msgSend as *const c_void);
        activate(app, sel(c"activateIgnoringOtherApps:"), true);

        objc_msgSend(app, sel(c"run"));
    }
}
