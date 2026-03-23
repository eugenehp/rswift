//! Check Foundation availability and basic info.
//!
//! cargo run -p rswift-foundation --example check

fn main() {
    println!("=== Foundation Framework ===\n");
    println!("Available: {}", foundation::is_available());

    if !foundation::is_available() {
        println!("❌ Foundation not available.");
        return;
    }

    println!("✅ Foundation is ready\n");
    println!("UUID: {}", foundation::uuid());
    println!("Locale: {} ({})", foundation::Locale::identifier(), foundation::Locale::language());
    println!("Timezone: {} (UTC{:+}h)", foundation::timezone(), foundation::timezone_offset() / 3600);

    let ts = foundation::now();
    println!("Now: {}", foundation::format_date(ts, "yyyy-MM-dd HH:mm:ss"));
}
