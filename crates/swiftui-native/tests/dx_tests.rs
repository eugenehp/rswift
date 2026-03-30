//! DX tests — ergonomic API surface.

use swiftui_native::prelude::*;

#[test]
fn test_text_chainable() {
    let v = text("hello").padding(10.0).opacity(0.8);
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_text_card() {
    let v = text("card").card(12.0, Color::DARK, 8.0);
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_color_constant() {
    let v = color(Color::RED).frame(50.0, 50.0);
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_color_hex() {
    let v = color(Color::hex(0xFF6600));
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_vstack_dsl() {
    let v = vstack(&[
        text("A").padding(4.0),
        divider(),
        text("B").padding(4.0),
    ]);
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_hstack_dsl() {
    let v = hstack(&[
        image("star.fill"),
        text("Label"),
        spacer(),
    ]);
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_into_view_str() {
    let v: View = "hello".into_view();
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_into_view_string() {
    let v: View = String::from("world").into_view();
    assert!(!v.handle().as_ptr().is_null());
}

#[test]
fn test_complex_layout() {
    let header = hstack(&[
        text("Dashboard").padding(8.0),
        spacer(),
        image("gear"),
    ]);

    let body = vstack(&[
        text("Item 1").padding(4.0),
        text("Item 2").padding(4.0),
        text("Item 3").padding(4.0),
    ]);

    let page = vstack(&[
        header,
        divider(),
        body,
        spacer(),
    ]).padding(20.0).bg(Color::DARK);

    assert!(!page.handle().as_ptr().is_null());
}

#[test]
fn test_debug_format() {
    let v = text("debug");
    let dbg = format!("{v:?}");
    assert!(dbg.contains("View"), "Debug should show View: {dbg}");
}
