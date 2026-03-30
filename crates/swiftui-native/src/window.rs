//! Window hosting — display a view via NSHostingController. Pure Rust.

use crate::existential::ViewExistential;
use crate::resolve;
use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn objc_msgSend(receiver: *mut c_void, sel: *const c_void, ...) -> *mut c_void;
    fn sel_registerName(name: *const c_char) -> *const c_void;
}

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

fn sel(name: &core::ffi::CStr) -> *const c_void {
    unsafe { sel_registerName(name.as_ptr()) }
}

fn cls(name: &core::ffi::CStr) -> *mut c_void {
    unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) as *mut c_void }
}

/// Show a native view in an NSWindow via NSHostingController.
///
/// This calls NSHostingController.init(rootView:) with the concrete view
/// type from the existential container, then creates an NSWindow.
///
/// # Note
/// This blocks on `NSApplication.run()`.
pub fn show_window(view: &ViewExistential, title: &str, width: f64, height: f64) {
    unsafe {
        let hosting_init = resolve::hosting_controller_init();
        let meta_accessor = resolve::hosting_controller_meta_accessor();

        // Get NSHostingController<V> metadata by calling the metadata accessor
        // with the concrete view type metadata
        let view_meta = view.metadata();
        let view_wt = view.witness_table();

        // Call metadata accessor: (MetadataRequest, ViewMetadata) -> Metadata
        type MetaAccessor = unsafe extern "C" fn(usize, *const c_void) -> *const c_void;
        let get_meta: MetaAccessor = core::mem::transmute(meta_accessor);
        let hc_meta = get_meta(0, view_meta);

        // Create NSHostingController<V>(rootView: view)
        // Swift CC: allocating init takes (rootView, self_metatype) -> instance
        let value_ptr = view.value_ptr();

        #[cfg(target_arch = "aarch64")]
        let controller: *mut c_void;

        #[cfg(target_arch = "aarch64")]
        {
            // Load view value into registers based on size
            let vwt = &*swift_runtime_sys::SwiftABI::get_value_witness_table(view_meta);

            if vwt.size <= 8 {
                let v0 = *(value_ptr as *const u64);
                core::arch::asm!(
                    "blr {func}",
                    func = in(reg) hosting_init,
                    in("x0") v0,
                    in("x20") hc_meta,
                    in("x21") view_wt,
                    lateout("x0") controller,
                    lateout("x1") _, lateout("x2") _, lateout("x3") _,
                    lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                    lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                    lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                    lateout("x16") _, lateout("x17") _, lateout("lr") _,
                    clobber_abi("C"),
                );
            } else {
                // Pass view via indirect pointer (x0 = pointer to value)
                core::arch::asm!(
                    "blr {func}",
                    func = in(reg) hosting_init,
                    in("x0") value_ptr,
                    in("x20") hc_meta,
                    in("x21") view_wt,
                    lateout("x0") controller,
                    lateout("x1") _, lateout("x2") _, lateout("x3") _,
                    lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
                    lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
                    lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
                    lateout("x16") _, lateout("x17") _, lateout("lr") _,
                    clobber_abi("C"),
                );
            }
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            panic!("swiftui-native only supports aarch64");
        }

        if controller.is_null() {
            eprintln!("swiftui-native: NSHostingController creation failed");
            return;
        }

        // Get the controller's view (NSView)
        let ns_view: *mut c_void = objc_msgSend(controller, sel(c"view"));

        // Create NSWindow
        let ns_app_class = cls(c"OBJC_CLASS_$_NSApplication");
        let app: *mut c_void = objc_msgSend(ns_app_class, sel(c"sharedApplication"));

        // NSWindow alloc/init
        let ns_window_class = cls(c"OBJC_CLASS_$_NSWindow");
        let window: *mut c_void = objc_msgSend(ns_window_class, sel(c"alloc"));

        // initWithContentRect:styleMask:backing:defer:
        #[repr(C)]
        struct NSRect { x: f64, y: f64, w: f64, h: f64 }
        let rect = NSRect { x: 0.0, y: 0.0, w: width, h: height };
        let style_mask: u64 = 1 | 2 | 4 | 8; // titled + closable + resizable + miniaturizable
        let backing: u64 = 2; // buffered

        let sel_init = sel(c"initWithContentRect:styleMask:backing:defer:");
        type InitWindowFn = unsafe extern "C" fn(
            *mut c_void, *const c_void,
            NSRect, u64, u64, bool,
        ) -> *mut c_void;
        let init_window: InitWindowFn = core::mem::transmute(objc_msgSend as *const c_void);
        let window = init_window(window, sel_init, rect, style_mask, backing, false);

        // Set title
        let ns_string_class = cls(c"OBJC_CLASS_$_NSString");
        let title_nsstr: *mut c_void = {
            let alloc: *mut c_void = objc_msgSend(ns_string_class, sel(c"alloc"));
            type InitWithBytes = unsafe extern "C" fn(
                *mut c_void, *const c_void,
                *const u8, u64, u64,
            ) -> *mut c_void;
            let f: InitWithBytes = core::mem::transmute(objc_msgSend as *const c_void);
            f(alloc, sel(c"initWithBytes:length:encoding:"),
              title.as_ptr(), title.len() as u64, 4) // NSUTF8StringEncoding = 4
        };
        objc_msgSend(window, sel(c"setTitle:"), title_nsstr);

        // Set content view
        objc_msgSend(window, sel(c"setContentView:"), ns_view);

        // Center and show
        objc_msgSend(window, sel(c"center"));
        objc_msgSend(window, sel(c"makeKeyAndOrderFront:"), core::ptr::null::<c_void>());

        // Activate app
        type ActivateFn = unsafe extern "C" fn(*mut c_void, *const c_void, bool);
        let activate: ActivateFn = core::mem::transmute(objc_msgSend as *const c_void);
        activate(app, sel(c"activateIgnoringOtherApps:"), true);

        // Set activation policy to regular
        type SetPolicyFn = unsafe extern "C" fn(*mut c_void, *const c_void, i64) -> bool;
        let set_policy: SetPolicyFn = core::mem::transmute(objc_msgSend as *const c_void);
        set_policy(app, sel(c"setActivationPolicy:"), 0); // NSApplicationActivationPolicyRegular

        // Run
        objc_msgSend(app, sel(c"run"));
    }
}
