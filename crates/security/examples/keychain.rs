//! Keychain CRUD example.
//!
//! cargo run -p rswift-security --example keychain

fn main() {
    println!("=== Security: Keychain ===\n");

    let service = "com.rswift.test";
    let account = "test@example.com";

    // Store
    match security::Keychain::set(service, account, b"s3cret_p@ss") {
        Ok(()) => println!("Stored password for {account}"),
        Err(e) => {
            println!("❌ Keychain write failed (error {e})");
            println!("   This is expected if running without Keychain access.");
            println!("   On macOS, you may need to allow access in the dialog.\n");

            // Still show random bytes
            let bytes = security::random_bytes(16).unwrap();
            println!("Random 16 bytes: {:02x?}", bytes);
            return;
        }
    }

    // Exists?
    println!("Exists: {}", security::Keychain::exists(service, account));

    // Read
    let pw = security::Keychain::get_string(service, account).unwrap();
    println!("Read:   {pw}");

    // Delete
    security::Keychain::delete(service, account).unwrap();
    println!("Deleted.");
    println!("Exists: {}", security::Keychain::exists(service, account));

    // Random bytes
    let bytes = security::random_bytes(16).unwrap();
    println!("\nRandom 16 bytes: {:02x?}", bytes);
}
