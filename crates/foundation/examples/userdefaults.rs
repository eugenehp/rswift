//! UserDefaults — persistent key-value storage.
//!
//! cargo run -p rswift-foundation --example userdefaults

fn main() {
    println!("=== UserDefaults ===\n");

    // String
    foundation::UserDefaults::set_string("test_name", "Alice");
    println!("Set 'test_name' = 'Alice'");
    println!("Get 'test_name' = {:?}", foundation::UserDefaults::get_string("test_name"));

    // Integer
    foundation::UserDefaults::set_int("test_count", 42);
    println!("Set 'test_count' = 42");
    println!("Get 'test_count' = {:?}", foundation::UserDefaults::get_int("test_count"));

    // Bool
    foundation::UserDefaults::set_bool("test_flag", true);
    println!("Set 'test_flag' = true");
    println!("Get 'test_flag' = {:?}", foundation::UserDefaults::get_bool("test_flag"));

    // Double
    foundation::UserDefaults::set_double("test_pi", std::f64::consts::PI);
    println!("Set 'test_pi' = π");
    println!("Get 'test_pi' = {:?}", foundation::UserDefaults::get_double("test_pi"));

    // Cleanup
    foundation::UserDefaults::remove("test_name");
    foundation::UserDefaults::remove("test_count");
    foundation::UserDefaults::remove("test_flag");
    foundation::UserDefaults::remove("test_pi");
    println!("\nCleaned up test keys.");
    println!("Get 'test_name' after remove = {:?}", foundation::UserDefaults::get_string("test_name"));
}
