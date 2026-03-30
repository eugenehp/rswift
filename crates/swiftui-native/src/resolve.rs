//! Symbol resolution from SwiftUI.framework — no Swift code, pure dlsym.

use core::ffi::{c_char, c_void};
use std::sync::Once;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

static INIT: Once = Once::new();
static mut FRAMEWORK: *mut c_void = std::ptr::null_mut();

fn framework() -> *mut c_void {
    INIT.call_once(|| unsafe {
        // Load SwiftUI + AppKit
        let h = dlopen(
            c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(),
            1, // RTLD_LAZY
        );
        assert!(!h.is_null(), "Failed to load SwiftUI.framework");
        dlopen(
            c"/System/Library/Frameworks/AppKit.framework/AppKit".as_ptr(),
            1,
        );
        FRAMEWORK = h;
    });
    unsafe { FRAMEWORK }
}

/// Resolve a symbol from SwiftUI.framework. Returns null if not found.
pub fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym(framework(), name.as_ptr()) as *const c_void }
}

/// Resolve a symbol, panicking if not found.
pub fn require(name: &core::ffi::CStr) -> *const c_void {
    let p = sym(name);
    assert!(
        !p.is_null(),
        "Required SwiftUI symbol not found: {}",
        name.to_str().unwrap()
    );
    p
}

// ═══════════════════════════════════════════════════════════════════════════
// Cached symbol accessors
// ═══════════════════════════════════════════════════════════════════════════

macro_rules! cached_sym {
    ($fn_name:ident, $sym:expr) => {
        pub fn $fn_name() -> *const c_void {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static CACHE: AtomicUsize = AtomicUsize::new(0);
            let v = CACHE.load(Ordering::Relaxed);
            if v != 0 {
                return v as *const c_void;
            }
            let p = require($sym);
            CACHE.store(p as usize, Ordering::Relaxed);
            p
        }
    };
}

// Type metadata
cached_sym!(text_metadata, c"$s7SwiftUI4TextVN");
cached_sym!(empty_view_metadata, c"$s7SwiftUI9EmptyViewVN");
cached_sym!(divider_metadata, c"$s7SwiftUI7DividerVN");
cached_sym!(image_metadata, c"$s7SwiftUI5ImageVN");
cached_sym!(color_metadata, c"$s7SwiftUI5ColorVN");
cached_sym!(spacer_metadata, c"$s7SwiftUI6SpacerVN");

// View protocol
cached_sym!(view_protocol, c"$s7SwiftUI4ViewMp");

// View witness tables
cached_sym!(text_view_wt, c"$s7SwiftUI4TextVAA4ViewAAWP");
cached_sym!(empty_view_view_wt, c"$s7SwiftUI9EmptyViewVAA0D0AAWP");
cached_sym!(divider_view_wt, c"$s7SwiftUI7DividerVAA4ViewAAWP");
cached_sym!(image_view_wt, c"$s7SwiftUI5ImageVAA4ViewAAWP");
cached_sym!(color_view_wt, c"$s7SwiftUI5ColorVAA4ViewAAWP");
cached_sym!(spacer_view_wt, c"$s7SwiftUI6SpacerVAA4ViewAAWP");

// Initializers (only the ones actually exported)
cached_sym!(empty_view_init, c"$s7SwiftUI9EmptyViewVACycfC");
cached_sym!(divider_init, c"$s7SwiftUI7DividerVACycfC");
cached_sym!(image_systemname_init, c"$s7SwiftUI5ImageV10systemNameACSS_tcfC");
cached_sym!(lsk_string_literal_init, c"$s7SwiftUI18LocalizedStringKeyV13stringLiteralACSS_tcfC");
cached_sym!(text_lsk_init, c"$s7SwiftUI4TextV_9tableName6bundle7commentAcA18LocalizedStringKeyV_SSSgSo8NSBundleCSgs06StaticI0VSgtcfC");

// NSHostingController
cached_sym!(hosting_controller_init, c"$s7SwiftUI19NSHostingControllerC8rootViewACyxGx_tcfC");
cached_sym!(hosting_controller_meta_accessor, c"$s7SwiftUI19NSHostingControllerCMa");
