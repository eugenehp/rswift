// The Swift bridge is compiled and linked by realitykit-sys/build.rs.
// This build script only needs to pass the Swift runtime rpath.
fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
}
