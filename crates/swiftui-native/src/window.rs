//! Window hosting via NSHostingController + ObjC runtime. Pure Rust.

use crate::resolve;
use crate::views::ViewHandle;
use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn objc_msgSend(receiver: *mut c_void, sel: *const c_void, ...) -> *mut c_void;
    fn sel_registerName(name: *const c_char) -> *const c_void;
}

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;
fn sel(n: &core::ffi::CStr) -> *const c_void { unsafe { sel_registerName(n.as_ptr()) } }
fn cls(n: &core::ffi::CStr) -> *mut c_void { unsafe { dlsym(RTLD_DEFAULT, n.as_ptr()) as *mut c_void } }

/// Show an AnyView in an NSWindow. Blocks on `NSApplication.run()`.
pub fn show_window(view: &ViewHandle, title: &str, width: f64, height: f64) {
    unsafe {
        // Create NSHostingController<AnyView>(rootView: view)
        let hc_init = resolve::hosting_ctrl_init();
        let hc_ma = resolve::hosting_ctrl_ma();
        let av_meta = resolve::anyview_meta();

        // Get NSHostingController<AnyView> metadata
        type MetaAccessor = unsafe extern "C" fn(usize, *const c_void) -> *const c_void;
        let get_hc_meta: MetaAccessor = core::mem::transmute(hc_ma);
        let hc_meta = get_hc_meta(0, av_meta);

        // AnyView is 8 bytes (class ref). Pass in x0, hc_meta in x20.
        let av_wt = resolve::anyview_wt();
        let mut av_buf = view.0.to_le_bytes();
        let controller: *mut c_void;
        core::arch::asm!(
            "blr {f}", f = in(reg) hc_init,
            in("x0") av_buf.as_ptr(), // @in AnyView (pointer to 8-byte value)
            in("x20") hc_meta,
            in("x21") av_wt,
            lateout("x0") controller,
            lateout("x1") _, lateout("x2") _, lateout("x3") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
        assert!(!controller.is_null(), "NSHostingController creation failed");

        // Get the controller's NSView
        let ns_view: *mut c_void = objc_msgSend(controller, sel(c"view"));

        // Setup NSApplication
        let app_cls = cls(c"OBJC_CLASS_$_NSApplication");
        let app: *mut c_void = objc_msgSend(app_cls, sel(c"sharedApplication"));

        // Create NSWindow
        let win_cls = cls(c"OBJC_CLASS_$_NSWindow");
        let win: *mut c_void = objc_msgSend(win_cls, sel(c"alloc"));

        #[repr(C)]
        struct NSRect { x: f64, y: f64, w: f64, h: f64 }

        type InitWin = unsafe extern "C" fn(*mut c_void, *const c_void, NSRect, u64, u64, bool) -> *mut c_void;
        let init_win: InitWin = core::mem::transmute(objc_msgSend as *const c_void);
        let win = init_win(
            win, sel(c"initWithContentRect:styleMask:backing:defer:"),
            NSRect { x: 0.0, y: 0.0, w: width, h: height },
            1 | 2 | 4 | 8, // titled + closable + resizable + miniaturizable
            2, // buffered
            false,
        );

        // Set title
        let ns_str_cls = cls(c"OBJC_CLASS_$_NSString");
        let ns_str: *mut c_void = objc_msgSend(ns_str_cls, sel(c"alloc"));
        type InitStr = unsafe extern "C" fn(*mut c_void, *const c_void, *const u8, u64, u64) -> *mut c_void;
        let init_str: InitStr = core::mem::transmute(objc_msgSend as *const c_void);
        let ns_title = init_str(ns_str, sel(c"initWithBytes:length:encoding:"), title.as_ptr(), title.len() as u64, 4);
        objc_msgSend(win, sel(c"setTitle:"), ns_title);
        objc_msgSend(win, sel(c"setContentView:"), ns_view);
        objc_msgSend(win, sel(c"center"));
        objc_msgSend(win, sel(c"makeKeyAndOrderFront:"), core::ptr::null::<c_void>());

        type SetPolicy = unsafe extern "C" fn(*mut c_void, *const c_void, i64) -> bool;
        let set_policy: SetPolicy = core::mem::transmute(objc_msgSend as *const c_void);
        set_policy(app, sel(c"setActivationPolicy:"), 0);

        type Activate = unsafe extern "C" fn(*mut c_void, *const c_void, bool);
        let activate: Activate = core::mem::transmute(objc_msgSend as *const c_void);
        activate(app, sel(c"activateIgnoringOtherApps:"), true);

        objc_msgSend(app, sel(c"run"));
    }
}
