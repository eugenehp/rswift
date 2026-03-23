//! CoreAnimation timing demo.
//!
//! cargo run -p rswift-coreanimation --example check

fn main() {
    println!("=== CoreAnimation ===\n");
    println!("Available: {}", coreanimation::is_available());

    let t1 = coreanimation::current_media_time();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let t2 = coreanimation::current_media_time();
    println!("Media time: {t1:.6}s");
    println!("After 10ms: {t2:.6}s (delta: {:.3}ms)", (t2 - t1) * 1000.0);
}
