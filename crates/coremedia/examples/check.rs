//! CoreMedia CMTime demo.
//!
//! cargo run -p rswift-coremedia --example check

fn main() {
    println!("=== CoreMedia: CMTime ===\n");
    println!("Available: {}", coremedia::is_available());

    let t1 = coremedia::CMTime::from_seconds(2.5);
    let t2 = coremedia::CMTime::from_seconds(1.5);
    println!("t1 = {:.1}s (value={}, timescale={})", t1.seconds(), t1.value, t1.timescale);
    println!("t2 = {:.1}s", t2.seconds());
    println!("t1 + t2 = {:.1}s", (t1 + t2).seconds());
    println!("t1 - t2 = {:.1}s", (t1 - t2).seconds());
    println!("t1 == t2: {}", t1 == t2);
    println!("t1 > t2:  {}", t1 > t2);
    println!("valid:    {}", t1.is_valid());
}
