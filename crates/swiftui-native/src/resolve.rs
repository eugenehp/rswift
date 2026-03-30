//! Symbol resolution from SwiftUI.framework — pure dlsym, no Swift code.

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
        let h = dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);
        assert!(!h.is_null(), "Failed to load SwiftUI.framework");
        dlopen(c"/System/Library/Frameworks/AppKit.framework/AppKit".as_ptr(), 1);
        FRAMEWORK = h;
    });
    unsafe { FRAMEWORK }
}

pub fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym(framework(), name.as_ptr()) as *const c_void }
}

pub fn require(name: &core::ffi::CStr) -> *const c_void {
    let p = sym(name);
    assert!(!p.is_null(), "Symbol not found: {}", name.to_str().unwrap());
    p
}

macro_rules! cached {
    ($name:ident, $sym:expr) => {
        pub fn $name() -> *const c_void {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static C: AtomicUsize = AtomicUsize::new(0);
            let v = C.load(Ordering::Relaxed);
            if v != 0 { return v as *const c_void; }
            let p = require($sym);
            C.store(p as usize, Ordering::Relaxed);
            p
        }
    };
}

// ── Type metadata ──
cached!(text_meta,       c"$s7SwiftUI4TextVN");
cached!(anyview_meta,    c"$s7SwiftUI7AnyViewVN");
cached!(color_meta,      c"$s7SwiftUI5ColorVN");
cached!(spacer_meta,     c"$s7SwiftUI6SpacerVN");
cached!(empty_meta,      c"$s7SwiftUI9EmptyViewVN");
cached!(divider_meta,    c"$s7SwiftUI7DividerVN");
cached!(image_meta,      c"$s7SwiftUI5ImageVN");

// ── View protocol ──
cached!(view_proto,      c"$s7SwiftUI4ViewMp");

// ── View witness tables ──
cached!(text_wt,         c"$s7SwiftUI4TextVAA4ViewAAWP");
cached!(anyview_wt,      c"$s7SwiftUI7AnyViewVAA0D0AAWP");
cached!(color_wt,        c"$s7SwiftUI5ColorVAA4ViewAAWP");
cached!(spacer_wt,       c"$s7SwiftUI6SpacerVAA4ViewAAWP");
cached!(empty_wt,        c"$s7SwiftUI9EmptyViewVAA0D0AAWP");
cached!(divider_wt,      c"$s7SwiftUI7DividerVAA4ViewAAWP");
cached!(image_wt,        c"$s7SwiftUI5ImageVAA4ViewAAWP");
cached!(color_shapestyle_wt, c"$s7SwiftUI5ColorVAA10ShapeStyleAAWP");

// ── Initializers ──
cached!(color_init,      c"$s7SwiftUI5ColorV_3red5green4blue7opacityA2C13RGBColorSpaceO_S4dtcfC");
cached!(spacer_init,     c"$s7SwiftUI6SpacerV9minLengthAC12CoreGraphics7CGFloatVSg_tcfC");
cached!(empty_init,      c"$s7SwiftUI9EmptyViewVACycfC");
cached!(divider_init,    c"$s7SwiftUI7DividerVACycfC");
cached!(image_sysname_init, c"$s7SwiftUI5ImageV10systemNameACSS_tcfC");
cached!(lsk_init,        c"$s7SwiftUI18LocalizedStringKeyV13stringLiteralACSS_tcfC");
cached!(text_lsk_init,   c"$s7SwiftUI4TextV_9tableName6bundle7commentAcA18LocalizedStringKeyV_SSSgSo8NSBundleCSgs06StaticI0VSgtcfC");
cached!(anyview_init,    c"$s7SwiftUI7AnyViewVyACxcAA0D0RzlufC");

// ── Modifiers ──
cached!(padding_fn,      c"$s7SwiftUI4ViewPAAE7paddingyQr12CoreGraphics7CGFloatVF");
cached!(opacity_fn,      c"$s7SwiftUI4ViewPAAE7opacityyQrSdF");
cached!(frame_fn,        c"$s7SwiftUI4ViewPAAE5frame5width6height9alignmentQr12CoreGraphics7CGFloatVSg_AkA9AlignmentVtF");
cached!(bg_fn,           c"$s7SwiftUI4ViewPAAE10background_20ignoresSafeAreaEdgesQrqd___AA4EdgeO3SetVtAA10ShapeStyleRd__lF");

// ── Stacks ──
cached!(vstack_init,     c"$s7SwiftUI6VStackV9alignment7spacing7contentACyxGAA19HorizontalAlignmentV_12CoreGraphics7CGFloatVSgxyXEtcfC");
cached!(hstack_init,     c"$s7SwiftUI6HStackV9alignment7spacing7contentACyxGAA17VerticalAlignmentV_12CoreGraphics7CGFloatVSgxyXEtcfC");

// ── Stack defaults ──
cached!(halign_center,   c"$s7SwiftUI19HorizontalAlignmentV6centerACvgZ");
cached!(valign_center,   c"$s7SwiftUI17VerticalAlignmentV6centerACvgZ");
cached!(alignment_center, c"$s7SwiftUI9AlignmentV6centerACvgZ");
cached!(edge_set_all,    c"$s7SwiftUI4EdgeO3SetV3allAEvgZ");

// ── Button ──
cached!(button_init,     c"$s7SwiftUI6ButtonVA2A4TextVRszrlE_6actionACyAEGqd___yyctcSyRd__lufC");

// ── Hosting ──
cached!(hosting_ctrl_init, c"$s7SwiftUI19NSHostingControllerC8rootViewACyxGx_tcfC");
cached!(hosting_ctrl_ma,  c"$s7SwiftUI19NSHostingControllerCMa");
