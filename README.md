# rswift

Build native Apple apps from Rust. Access 150+ Apple frameworks through safe Rust APIs.

## Quick start

```toml
[dependencies]
rswift = { version = "0.0.1", features = ["foundation", "avfaudio", "metal"] }
```

```rust
use rswift::prelude::*;

fn main() {
    // System info
    let (major, minor, _) = foundation::ProcessInfo::os_version();
    println!("macOS {major}.{minor} — {} cores, {:.0} GB RAM",
        foundation::ProcessInfo::processor_count(),
        foundation::ProcessInfo::physical_memory() as f64 / 1e9);

    // Audio engine
    let engine = avfaudio::AudioEngine::new();
    let (sr, ch) = engine.output_format();
    println!("Audio: {sr} Hz, {ch} channels");

    // GPU compute with Metal
    metal::auto_load();
    let gpu = metal::Device::system_default().unwrap();
    println!("GPU: {}", gpu.name());
}
```

## Feature categories

Pick what you need — only the frameworks you enable get compiled:

```toml
rswift = { features = ["media"] }       # AVFAudio, AVFoundation, CoreMedia, ...
rswift = { features = ["ml"] }          # CoreML, Vision, Accelerate, ...
rswift = { features = ["graphics"] }    # Metal, SceneKit, CoreAnimation, ...
rswift = { features = ["full"] }        # Everything
```

| Category    | Frameworks |
|-------------|-----------|
| `runtime`   | Foundation, Security, AppKit/UIKit, swift-runtime |
| `ui`        | SwiftUI, Charts, TipKit, WidgetKit |
| `media`     | AVFAudio, AVFoundation, AVKit, CoreMedia, AudioToolbox, MediaPlayer, MusicKit, … |
| `ml`        | CoreML, CreateML, Vision, NaturalLanguage, Foundation Models, Translation, Accelerate |
| `graphics`  | Metal, SceneKit, SpriteKit, CoreGraphics, CoreImage, CoreText, CoreAnimation |
| `data`      | SwiftData, CoreData, CloudKit, Combine |
| `auth`      | AuthenticationServices, CryptoKit, LocalAuthentication, DeviceCheck |
| `network`   | Network, NetworkExtension, MultipeerConnectivity, WebKit |
| `location`  | CoreLocation, MapKit |
| `health`    | HealthKit, WorkoutKit |
| `commerce`  | StoreKit, PassKit, FinanceKit |
| `games`     | GameKit, GameController, GameplayKit |
| `hardware`  | CoreBluetooth, CoreHaptics, CoreMotion, CoreNFC, DockKit, SensorKit |
| `system`    | OSLog, BackgroundTasks, UserNotifications, PushKit, EventKit |
| `comms`     | CallKit, MessageUI, Contacts, GroupActivities |
| `ar-vr`     | ARKit, RealityKit, Spatial, RoomPlan, CompositorServices |

## Crates with full Rust APIs

These crates go beyond availability checks and provide complete, safe Rust wrappers:

| Crate | APIs |
|-------|------|
| **`foundation`** | UserDefaults, FileManager, ProcessInfo, Bundle, Locale, UUID, Date formatting, JSON |
| **`security`** | Keychain CRUD, secure random bytes |
| **`appkit`** | Screen info, Clipboard, App lifecycle, Alerts, Dark mode, Open URL, Finder |
| **`avfaudio`** | AudioEngine (start/stop/format), AudioPlayer (play/pause/volume/seek/loops) |
| **`avfoundation`** | Player (play/pause/rate/volume/seek/mute), Asset (duration), PlayerStatus |
| **`accelerate`** | vDSP (add/sub/mul/div/scale/dot/sum/mean/max/min/rms/normalize/FFT), BLAS (sgemm/dgemm) |
| **`coremedia`** | CMTime (arithmetic, comparison, seconds conversion, operator overloads) |
| **`coreanimation`** | CACurrentMediaTime, CATransaction (begin/commit/duration/timing) |
| **`audiotoolbox`** | System sounds, alert sounds, custom sound loading, well-known sound IDs |
| **`metal`** | Device, CommandQueue, Buffers, Textures, Shaders, Compute/Blit pipelines |
| **`corelocation`** | Authorization status, great-circle distance |
| **`storekit`** | Payment capability check, receipt URL |
| **`contacts`** | Authorization status, contact count |
| **`swiftui`** | View DSL, modifiers, app builder |
| **`realitykit`** | Entity, Scene, materials |
| **`charts`** | Chart views, data series |
| **`combine`** | Subject, CurrentValue, Subscription |
| **`spatial`** | Point3D, Rotation3D, distance |
| **`swift-data`** | Model container, context, queries |
| **`foundation-models`** | Apple Intelligence chat sessions |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  Your Rust App                   │
├─────────────────────────────────────────────────┤
│        rswift (unified crate + prelude)          │
├──────┬──────┬──────┬──────┬──────┬──────────────┤
│ avf  │metal │found │accel │ ...  │  150+ crates │
│audio │      │ation │erate │      │              │
├──────┴──────┴──────┴──────┴──────┴──────────────┤
│         swift-helper-build (build.rs)            │
│     Auto-compiles & links Swift helper dylib     │
├─────────────────────────────────────────────────┤
│        libSwiftUIHelper.dylib (Swift)            │
│    @_cdecl exports for all framework bridges     │
├─────────────────────────────────────────────────┤
│            Apple Frameworks (system)             │
│  Foundation │ AVFAudio │ Metal │ Security │ ...  │
└─────────────────────────────────────────────────┘
```

**No manual `dlopen`** — the `build.rs` in each crate automatically:
1. Finds `swift_helper/` and compiles all Swift sources
2. Sets `@rpath` install name for portable dylib loading
3. Emits `cargo:rustc-link-lib` and rpath directives

## Examples

```sh
# Foundation — system info
cargo run -p rswift-foundation --example sysinfo

# Foundation — UserDefaults CRUD
cargo run -p rswift-foundation --example userdefaults

# Accelerate — vector math + matrix multiply
cargo run -p rswift-accelerate --example check

# Security — Keychain + random bytes
cargo run -p rswift-security --example keychain

# AVFAudio — audio engine + playback
cargo run -p avfaudio --example check
cargo run -p avfaudio --example play_audio -- /path/to/sound.wav

# AVFoundation — media player
cargo run -p rswift-avfoundation --example player -- /path/to/video.mp4

# Metal — GPU compute
cargo run -p rswift-metal --example gpu_compute

# CoreMedia — CMTime arithmetic
cargo run -p rswift-coremedia --example check

# AppKit — screen, clipboard, dark mode
cargo run -p rswift-appkit --example check

# CoreLocation — distance calculation
cargo run -p corelocation --example check

# Unified crate
cargo run -p rswift --example check --features avfaudio,avfoundation,metal
```

## Testing

```sh
cargo test -p rswift-foundation    # 15 tests — UserDefaults, FileManager, ProcessInfo, ...
cargo test -p rswift-accelerate    # 11 tests — vDSP, BLAS matrix multiply
cargo test -p rswift-coremedia     #  6 tests — CMTime arithmetic, comparison
cargo test -p rswift-security      #  3 tests — secure random bytes
```

## Platform support

See [PLATFORMS.md](PLATFORMS.md) for the full compatibility matrix.

**Primary:** macOS (arm64, x86_64)
**Planned:** iOS, tvOS, visionOS, watchOS

## Building from source

```sh
# Prerequisites: Xcode Command Line Tools
xcode-select --install

# Clone and build
git clone https://github.com/eugenehp/rswift.git
cd rswift
cargo build -p rswift --features full
cargo test -p rswift-foundation -p rswift-accelerate -p rswift-coremedia -p rswift-security
```

## License

GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

## Citation

```bibtex
@software{rswift,
  author = {Eugene Hauptmann},
  title  = {rswift},
  year   = {2025},
  url    = {https://github.com/eugenehp/rswift},
  note   = {Build native Apple apps from Rust}
}
```
