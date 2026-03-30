//! CoreLocation — comprehensive example demonstrating location APIs from Rust.
//!
//! Shows distance calculations, authorization status, and service availability.
//!
//! cargo run -p corelocation --example corelocation

fn main() {
    println!("=== CoreLocation Example ===\n");

    // ── Service availability ─────────────────────────────────────────────
    println!("── Service Availability ──\n");
    println!("CoreLocation available:        {}", corelocation::is_available());
    println!("Location services enabled:     {}", corelocation::location_services_enabled());
    println!("Heading available:             {}", corelocation::heading_available());
    println!(
        "Significant change monitoring: {}",
        corelocation::significant_location_change_monitoring_available()
    );

    // ── Authorization ────────────────────────────────────────────────────
    println!("\n── Authorization ──\n");
    let status = corelocation::authorization_status();
    println!("Authorization status: {:?}", status);
    match status {
        corelocation::AuthorizationStatus::NotDetermined => {
            println!("  → User has not yet been asked for location permission.");
        }
        corelocation::AuthorizationStatus::Restricted => {
            println!("  → Location access is restricted (e.g. parental controls).");
        }
        corelocation::AuthorizationStatus::Denied => {
            println!("  → User denied location access.");
        }
        corelocation::AuthorizationStatus::AuthorizedAlways => {
            println!("  → App is authorized to use location at all times.");
        }
        corelocation::AuthorizationStatus::AuthorizedWhenInUse => {
            println!("  → App is authorized to use location while in use.");
        }
    }

    // ── Distance calculations ────────────────────────────────────────────
    println!("\n── Great-Circle Distances ──\n");

    let cities: &[(&str, f64, f64)] = &[
        ("San Francisco", 37.7749, -122.4194),
        ("Los Angeles", 34.0522, -118.2437),
        ("New York", 40.7128, -74.0060),
        ("London", 51.5074, -0.1278),
        ("Tokyo", 35.6762, 139.6503),
        ("Sydney", -33.8688, 151.2093),
        ("São Paulo", -23.5505, -46.6333),
    ];

    // Print distance matrix header
    print!("{:<16}", "");
    for (name, _, _) in cities.iter().skip(1) {
        print!("{:>14}", name);
    }
    println!();
    println!("{}", "-".repeat(16 + 14 * (cities.len() - 1)));

    for (i, (name_a, lat_a, lon_a)) in cities.iter().enumerate() {
        if i == cities.len() - 1 {
            break; // skip last row — no distances to compute
        }
        print!("{:<16}", name_a);
        for (j, (_name_b, lat_b, lon_b)) in cities.iter().enumerate() {
            if j == 0 {
                continue; // skip self-column
            }
            if j <= i {
                print!("{:>14}", "");
            } else {
                let d = corelocation::distance(*lat_a, *lon_a, *lat_b, *lon_b);
                print!("{:>11.0} km", d / 1000.0);
            }
        }
        println!();
    }

    // ── Coordinate struct ────────────────────────────────────────────────
    println!("\n── Coordinate Struct ──\n");

    let home = corelocation::Coordinate {
        latitude: 37.7749,
        longitude: -122.4194,
    };
    let dest = corelocation::Coordinate {
        latitude: 34.0522,
        longitude: -118.2437,
    };
    println!("Home: {:?}", home);
    println!("Dest: {:?}", dest);
    let d = corelocation::distance(home.latitude, home.longitude, dest.latitude, dest.longitude);
    println!("Distance: {:.1} km", d / 1000.0);

    // ── Geocoding (stub) ─────────────────────────────────────────────────
    println!("\n── Geocoding (sync) ──\n");
    match corelocation::geocode_sync("1 Infinite Loop, Cupertino, CA") {
        Some(coord) => println!("Geocoded: {:?}", coord),
        None => println!("Geocoding not yet implemented (requires block ABI support)."),
    }

    println!("\n=== Done ===");
}
