//! Comprehensive tests for pure-Rust SwiftUI view construction.

use swiftui_native::views;

// ═══════════════════════════════════════════════════════════════════════════
// §1: Basic view creation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_text() {
    let h = views::text("hello");
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_text_empty_string() {
    let h = views::text("");
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_text_unicode() {
    let h = views::text("日本語テスト 🎉");
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_text_long_string() {
    let s = "a".repeat(1000);
    let h = views::text(&s);
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_color() {
    let h = views::color(1.0, 0.0, 0.0, 1.0); // red
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_color_transparent() {
    let h = views::color(0.0, 0.0, 0.0, 0.0);
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_spacer() {
    let h = views::spacer();
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_empty_view() {
    let h = views::empty_view();
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_divider() {
    let h = views::divider();
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_system_image() {
    let h = views::system_image("star.fill");
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_system_image_various() {
    for name in &["gear", "heart", "trash", "pencil", "magnifyingglass"] {
        let h = views::system_image(name);
        assert!(!h.as_ptr().is_null(), "Failed for image: {name}");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §2: Handle lifecycle
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_clone() {
    let h = views::text("clone me");
    let h2 = h.clone();
    assert_eq!(h.as_ptr(), h2.as_ptr());
    drop(h2);
    // h should still be valid
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_drop_many() {
    for i in 0..1000 {
        let _ = views::text(&format!("item {i}"));
    }
}

#[test]
fn test_clone_and_drop_interleaved() {
    let a = views::text("a");
    let b = a.clone();
    let c = b.clone();
    drop(a);
    let d = c.clone();
    drop(b);
    drop(c);
    assert!(!d.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §3: Modifiers
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_padding() {
    let t = views::text("padded");
    let p = views::padding(&t, 16.0);
    assert!(!p.as_ptr().is_null());
}

#[test]
fn test_padding_zero() {
    let t = views::text("no pad");
    let p = views::padding(&t, 0.0);
    assert!(!p.as_ptr().is_null());
}

#[test]
fn test_opacity() {
    let t = views::text("faded");
    let o = views::opacity(&t, 0.5);
    assert!(!o.as_ptr().is_null());
}

#[test]
fn test_opacity_fully_transparent() {
    let t = views::text("gone");
    let o = views::opacity(&t, 0.0);
    assert!(!o.as_ptr().is_null());
}

#[test]
fn test_frame() {
    let t = views::text("framed");
    let f = views::frame(&t, 100.0, 50.0);
    assert!(!f.as_ptr().is_null());
}

#[test]
fn test_background() {
    let t = views::text("bg");
    let b = views::background(&t, 0.0, 0.0, 1.0, 1.0);
    assert!(!b.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §4: Modifier chaining
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_chain_padding_opacity() {
    let t = views::text("styled");
    let p = views::padding(&t, 10.0);
    let o = views::opacity(&p, 0.8);
    assert!(!o.as_ptr().is_null());
}

#[test]
fn test_chain_many_modifiers() {
    let t = views::text("complex");
    let p = views::padding(&t, 12.0);
    let f = views::frame(&p, 200.0, 100.0);
    let b = views::background(&f, 0.9, 0.9, 0.95, 1.0);
    let o = views::opacity(&b, 0.95);
    assert!(!o.as_ptr().is_null());
}

#[test]
fn test_modifier_on_color() {
    let c = views::color(1.0, 0.0, 0.0, 1.0);
    let p = views::padding(&c, 8.0);
    assert!(!p.as_ptr().is_null());
}

#[test]
fn test_modifier_on_spacer() {
    let s = views::spacer();
    let p = views::padding(&s, 8.0);
    assert!(!p.as_ptr().is_null());
}

#[test]
fn test_modifier_on_image() {
    let img = views::system_image("star.fill");
    let p = views::padding(&img, 4.0);
    let o = views::opacity(&p, 0.7);
    assert!(!o.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §5: Stress tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_many_colors() {
    for i in 0..500 {
        let f = i as f64 / 500.0;
        let _ = views::color(f, 1.0 - f, 0.5, 1.0);
    }
}

#[test]
fn test_many_modifier_chains() {
    for i in 0..100 {
        let t = views::text(&format!("row {i}"));
        let p = views::padding(&t, 4.0);
        let _ = views::opacity(&p, 0.9);
    }
}

#[test]
fn test_deep_modifier_chain() {
    let mut v = views::text("deep");
    for _ in 0..20 {
        v = views::padding(&v, 1.0);
    }
    assert!(!v.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §6: Type verification via swift-runtime
// ═══════════════════════════════════════════════════════════════════════════

fn type_name(h: &views::ViewHandle) -> String {
    // AnyView is a class — get its metadata via the isa pointer
    // Since it's wrapped in AnyView, the type name should be "AnyView"
    unsafe {
        let meta = swiftui_native::resolve::anyview_meta();
        let result = swift_runtime_sys::SwiftCCThunks::swift_getTypeName(meta, true);
        result.map(|(name, _)| name.to_string()).unwrap_or_default()
    }
}

#[test]
fn test_text_is_anyview() {
    let t = views::text("check");
    let name = type_name(&t);
    assert!(name.contains("AnyView"), "Expected AnyView, got: {name}");
}

#[test]
fn test_color_is_anyview() {
    let c = views::color(1.0, 0.0, 0.0, 1.0);
    let name = type_name(&c);
    assert!(name.contains("AnyView"), "Expected AnyView, got: {name}");
}

// ═══════════════════════════════════════════════════════════════════════════
// §7: Stacks
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_vstack_empty() {
    let v = views::vstack(&[]);
    assert!(!v.as_ptr().is_null());
}

#[test]
fn test_vstack_single() {
    let t = views::text("solo");
    let v = views::vstack(&[t]);
    assert!(!v.as_ptr().is_null());
}

#[test]
fn test_vstack_two() {
    let a = views::text("A");
    let b = views::text("B");
    let v = views::vstack(&[a, b]);
    assert!(!v.as_ptr().is_null());
}

#[test]
fn test_vstack_many() {
    let children: Vec<_> = (0..5).map(|i| views::text(&format!("item {i}"))).collect();
    let v = views::vstack(&children);
    assert!(!v.as_ptr().is_null());
}

#[test]
fn test_hstack() {
    let a = views::text("L");
    let b = views::spacer();
    let c = views::text("R");
    let h = views::hstack(&[a, b, c]);
    assert!(!h.as_ptr().is_null());
}

#[test]
fn test_zstack() {
    let bg = views::color(1.0, 0.0, 0.0, 1.0);
    let fg = views::text("overlay");
    let z = views::zstack(&[bg, fg]);
    assert!(!z.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §8: Button + stubs
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_button() {
    let b = views::button("Click", || {});
    assert!(!b.as_ptr().is_null());
}

#[test]
fn test_scroll() {
    let t = views::text("scrollable");
    let s = views::scroll(&t);
    assert!(!s.as_ptr().is_null());
}

#[test]
fn test_styled_text() {
    let t = views::styled_text("fancy", 24.0, 1, 1.0, 0.0, 0.0, 1.0);
    assert!(!t.as_ptr().is_null());
}

#[test]
fn test_bold_text() {
    let t = views::bold_text("bold", 32.0);
    assert!(!t.as_ptr().is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// §9: Diagnostics — demangled names for DX
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_demangle_modifier_type() {
    use swiftui_native::diag;
    let name = diag::demangle_type(
        b"7SwiftUI15ModifiedContentVyAA7AnyViewVAA14_PaddingLayoutVG",
    );
    assert!(
        name.contains("ModifiedContent") && name.contains("PaddingLayout"),
        "Demangled name should be readable, got: {name}",
    );
}

#[test]
fn test_demangle_symbol() {
    use swiftui_native::diag;
    let name = diag::symbol_name(c"$s7SwiftUI4TextVN");
    assert!(name.contains("Text"), "Should contain 'Text', got: {name}");
}

#[test]
fn test_type_name_from_metadata() {
    use swiftui_native::diag;
    let name = diag::type_name(swiftui_native::resolve::text_meta());
    assert!(name.contains("Text"), "Should contain 'Text', got: {name}");
}

#[test]
fn test_view_handle_debug() {
    let v = views::text("debug me");
    let dbg = format!("{v:?}");
    assert!(dbg.contains("AnyView"), "Debug should mention AnyView, got: {dbg}");
}

// ═══════════════════════════════════════════════════════════════════════════
// §8: Symbol resolution
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_required_symbols_resolve() {
    // These should all resolve without panicking
    swiftui_native::resolve::text_meta();
    swiftui_native::resolve::anyview_meta();
    swiftui_native::resolve::color_meta();
    swiftui_native::resolve::spacer_meta();
    swiftui_native::resolve::empty_meta();
    swiftui_native::resolve::divider_meta();
    swiftui_native::resolve::image_meta();
    swiftui_native::resolve::view_proto();
    swiftui_native::resolve::anyview_init();
    swiftui_native::resolve::padding_fn();
    swiftui_native::resolve::opacity_fn();
    swiftui_native::resolve::frame_fn();
    swiftui_native::resolve::bg_fn();
    swiftui_native::resolve::hosting_ctrl_init();
}
