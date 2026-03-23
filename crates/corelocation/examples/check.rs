//! CoreLocation — distance and authorization check.
//!
//! cargo run -p corelocation --example check

fn main() {
    println!("=== CoreLocation ===\n");
    println!("Available: {}", corelocation::is_available());

    if !corelocation::is_available() {
        return;
    }

    println!("Auth status: {:?}", corelocation::authorization_status());

    // Distance: San Francisco → Los Angeles
    let d = corelocation::distance(37.7749, -122.4194, 34.0522, -118.2437);
    println!("SF → LA: {:.1} km", d / 1000.0);

    // Distance: New York → London
    let d2 = corelocation::distance(40.7128, -74.0060, 51.5074, -0.1278);
    println!("NYC → London: {:.0} km", d2 / 1000.0);
}
