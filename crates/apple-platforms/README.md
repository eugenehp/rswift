# apple-platforms

Apple platform metadata, target-triple conversion, and SDK name resolution for Rust build tooling.

```toml
[dependencies]
apple-platforms = "0.0.3"
```

## What it does

Maps the full Apple platform landscape — every Mach-O platform ID from LLVM's
`MachO.def` — to a typed `ApplePlatform` enum and its associated metadata:
Clang target strings, TAPI targets, SDK names, marketing names, and simulator
relationships.  Also converts Rust target triples to their Clang equivalents and
to `xcrun --sdk` names.

Primarily useful in `build.rs` scripts that need to invoke `swiftc`, `xcrun`,
or `clang` with the correct flags for the current Rust compilation target.

## Quick start

```rust
use apple_platforms::{ApplePlatform, triple};

// Parse a Rust target triple → typed platform
let platform = ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap();
assert_eq!(platform, ApplePlatform::IOSSimulator);

// SDK name for xcrun --sdk <name>
assert_eq!(platform.sdk(),    Some("iphonesimulator"));

// Navigate sim ↔ device
assert_eq!(platform.device(), ApplePlatform::IOS);
assert_eq!(platform.is_simulator(), true);
assert_eq!(ApplePlatform::IOS.simulator(), Some(ApplePlatform::IOSSimulator));

// Access full static metadata
let meta = platform.metadata();
println!("{}", meta.marketing);   // "iOS Simulator"
println!("{}", meta.tapi_target); // "ios-simulator"

// String-only path for build scripts
assert_eq!(triple::to_clang("aarch64-apple-darwin"), Some("arm64-apple-macosx"));
assert_eq!(triple::to_sdk("aarch64-apple-ios"),      Some("iphoneos"));
```

## Usage in `build.rs`

```rust
// build.rs
fn main() {
    let target = std::env::var("TARGET").unwrap();

    // Get the Clang triple and SDK for the current Rust target
    if let Some(clang) = apple_platforms::triple::to_clang(&target) {
        println!("cargo:rustc-env=CLANG_TARGET={clang}");
    }

    if let Ok(platform) = apple_platforms::ApplePlatform::from_rust_triple(&target) {
        if let Some(sdk) = platform.sdk() {
            println!("cargo:rustc-env=APPLE_SDK={sdk}");
        }
    }
}
```

## Platform coverage

All platforms from [LLVM `MachO.def`](https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def):

| `ApplePlatform` | Mach-O ID | SDK (`xcrun --sdk`) | Marketing name |
|-----------------|:---------:|---------------------|----------------|
| `Unknown`        | 0  | —                   | unknown         |
| `MacOS`          | 1  | `macosx`            | macOS           |
| `IOS`            | 2  | `iphoneos`          | iOS             |
| `TvOS`           | 3  | `appletvos`         | tvOS            |
| `WatchOS`        | 4  | `watchos`           | watchOS         |
| `BridgeOS`       | 5  | `bridgeos`          | bridgeOS        |
| `MacCatalyst`    | 6  | `macosx` ¹          | macCatalyst     |
| `IOSSimulator`   | 7  | `iphonesimulator`   | iOS Simulator   |
| `TvOSSimulator`  | 8  | `appletvsimulator`  | tvOS Simulator  |
| `WatchOSSimulator`| 9 | `watchsimulator`    | watchOS Simulator |
| `DriverKit`      | 10 | `driverkit`         | DriverKit       |
| `XrOS`           | 11 | `xros`              | visionOS        |
| `XrOSSimulator`  | 12 | `xrsimulator`       | visionOS Simulator |

¹ Mac Catalyst uses the macOS SDK with the `macabi` ABI environment.

## API surface

### `ApplePlatform` — the central type

```rust
// Construction
ApplePlatform::from_rust_triple("aarch64-apple-visionos")?   // from Rust triple
ApplePlatform::from_id(11)                                    // from Mach-O ID → Option<Self>
ApplePlatform::try_from(11u32)?                               // TryFrom<u32 / usize>
"visionos".parse::<ApplePlatform>()?                          // FromStr

// Metadata
platform.id()         // → u32 Mach-O ID
platform.sdk()        // → Option<&'static str>   e.g. "iphoneos"
platform.metadata()   // → &Platform  (full struct)
platform.to_string()  // → marketing name via Display

// Simulator ↔ device
platform.is_simulator()  // bool
platform.is_device()     // bool  (non-sim, non-Unknown)
platform.device()        // Self  (sim → device, or self)
platform.simulator()     // Option<Self>  (device → sim, or None)

// Iteration
ApplePlatform::iter()    // Iterator over all 13 variants
ApplePlatform::ALL       // [ApplePlatform; 13]
```

### `triple` — string-based build-script helpers

```rust
use apple_platforms::triple;

triple::to_clang("aarch64-apple-ios-sim")  // → Some("arm64-apple-ios-simulator")
triple::to_sdk("aarch64-apple-ios-sim")    // → Some("iphonesimulator")
// Both return None for non-Apple / unrecognised triples
```

### `Platform` — static metadata struct

```rust
pub struct Platform {
    pub platform:    &'static str,  // "IOSSIMULATOR"
    pub id:          u32,           // 7
    pub name:        &'static str,  // "iossimulator"
    pub build_name:  &'static str,  // "iossimulator"
    pub target:      &'static str,  // "ios-simulator"
    pub tapi_target: &'static str,  // "ios-simulator"
    pub marketing:   &'static str,  // "iOS Simulator"
    pub sdk:         Option<&'static str>, // Some("iphonesimulator")
}
```

## Rust triple → Clang triple mapping

| Rust triple | Clang triple |
|-------------|-------------|
| `aarch64-apple-darwin` | `arm64-apple-macosx` |
| `x86_64-apple-darwin` | `x86_64-apple-macosx` |
| `aarch64-apple-ios` | `arm64-apple-ios` |
| `aarch64-apple-ios-sim` | `arm64-apple-ios-simulator` |
| `x86_64-apple-ios` | `x86_64-apple-ios-simulator` |
| `aarch64-apple-ios-macabi` | `arm64-apple-ios-macabi` |
| `aarch64-apple-tvos` | `arm64-apple-tvos` |
| `aarch64-apple-tvos-sim` | `arm64-apple-tvos-simulator` |
| `aarch64-apple-watchos` | `arm64-apple-watchos` |
| `armv7k-apple-watchos` | `armv7k-apple-watchos` |
| `arm64_32-apple-watchos` | `arm64_32-apple-watchos` |
| `aarch64-apple-watchos-sim` | `arm64-apple-watchos-simulator` |
| `aarch64-apple-visionos` | `arm64-apple-xros` |
| `aarch64-apple-visionos-sim` | `arm64-apple-xros-simulator` |
| `aarch64-apple-driverkit` | `arm64-apple-driverkit` |
| `x86_64-apple-driverkit` | `x86_64-apple-driverkit` |
| anything else | `None` |

## Examples

```sh
cargo run -p apple-platforms --example platform  # full platform table
cargo run -p apple-platforms --example triple    # Rust → Clang triple table
cargo run -p apple-platforms --example sdk       # Rust triple → SDK + platform
```

## Tests

```sh
cargo test -p apple-platforms   # 33 unit tests + 9 doc tests
```

## Design notes

- **No lifetime parameters.** `Platform` holds only `&'static str` fields; no
  `Platform<'a>` in your signatures.
- **`None` not `Unknown`.** `from_id` and `TryFrom` return `None`/`Err` for
  unrecognised IDs rather than silently falling back to `Unknown`.
- **`Option` not passthrough.** `triple::to_clang` and `triple::to_sdk` return
  `None` for non-Apple triples; the old identity-passthrough behaviour masked
  typos silently.
- **Single source of truth.** `triple::to_sdk` delegates to
  `ApplePlatform::from_rust_triple` + `.sdk()`, so the triple→SDK mapping lives
  in exactly one place.
- **Ordered.** `ApplePlatform` derives `PartialOrd` + `Ord` by Mach-O ID.

## License

MIT — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
