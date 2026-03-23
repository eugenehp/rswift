//! AppKit — screen, clipboard, dark mode.
//!
//! cargo run -p rswift-appkit --example check

fn main() {
    println!("=== AppKit ===\n");
    println!("Available: {}", appkit::is_available());

    if !appkit::is_available() {
        println!("(AppKit not available — try on macOS)");
        return;
    }

    let (w, h) = appkit::Screen::main_size();
    println!("Screen:     {w} × {h} pts");
    println!("Scale:      {}x", appkit::Screen::scale());
    println!("Screens:    {}", appkit::Screen::count());
    println!("Dark mode:  {}", appkit::is_dark_mode());
    println!("Windows:    {}", appkit::App::window_count());

    // Clipboard round-trip
    let orig = appkit::Clipboard::get();
    appkit::Clipboard::set("Hello from rswift!");
    println!("Clipboard:  {:?}", appkit::Clipboard::get());
    // Restore
    if let Some(o) = orig {
        appkit::Clipboard::set(&o);
    }
}
