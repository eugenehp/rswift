//! StoreKit — payment capability check.
//!
//! cargo run -p storekit --example check

fn main() {
    println!("=== StoreKit ===\n");
    println!("Available: {}", storekit::is_available());
    println!("Can make payments: {}", storekit::can_make_payments());
    println!("Receipt URL: {:?}", storekit::app_store_receipt_url());
}
