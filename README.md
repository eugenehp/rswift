# rswift

Build native Apple apps from Rust. SwiftUI, RealityKit, Combine, Metal, visionOS — all with a declarative DSL, reactive state, and 100% pixel parity.

```rust
use swiftui::prelude::*;

fn main() {
    app("My App", 400.0, 300.0, |cx| {
        let count = cx.state(0i32);
        vstack![
            text_fmt!("Count: {count}").bold().size(48.0),
            button("+1", count.increment()),
            button("Reset", count.set_to(0)),
        ].style(Page)
    });
}
```

```bash
git clone https://github.com/eugenehp/rswift
cd rswift
cargo run -p swiftui --example showcase
```

Zero setup — `build.rs` auto-compiles the Swift helper and auto-detects your SDK.

## Crates

### Core

| Crate | Description |
|-------|-------------|
| [`swift-runtime-sys`](crates/swift-runtime-sys) | Raw FFI to the Swift runtime (490+ symbols, arm64 asm thunks) |
| [`swift-runtime`](crates/swift-runtime) | Safe Rust wrappers (Metadata, types, Retained) |
| [`swiftui-sys`](crates/swiftui-sys) | Raw SwiftUI FFI (dlsym function pointers) |
| [`swiftui-macros`](crates/swiftui-macros) | Proc macros (`text_fmt!`, `#[derive(View)]`) |
| [`swiftui`](crates/swiftui) | SwiftUI DSL — 142/142 API coverage |
| [`realitykit-sys`](crates/realitykit-sys) | Raw RealityKit FFI |
| [`realitykit`](crates/realitykit) | RealityKit 3D scene builder |
| [`apple-sys-helpers`](crates/apple-sys-helpers) | Shared dlsym helpers and `apple_framework!` macro |
| [`swift-bridge-gen`](crates/swift-bridge-gen) | Auto-generate bindings for any Apple framework |
| [`winit-swift`](crates/winit-swift) | Drop-in winit replacement with Metal, haptics, HDR, visionOS |
| [`swiftui-app`](crates/swiftui-app) | Declarative SwiftUI apps from build.rs — Metal at 60fps |

### Apple Frameworks (158 crates)

Every Apple framework with a Swift module is covered. All available as `rswift-{name}`.

<details>
<summary>Full framework list</summary>

| Crate | Description |
|-------|-------------|
| [`accessibility`](crates/accessibility) | Accessibility — assistive technology support from Rust |
| [`accessorysetupkit`](crates/accessorysetupkit) | AccessorySetupKit — accessory pairing from Rust |
| [`accessorytransportextension`](crates/accessorytransportextension) | AccessoryTransportExtension — accessory transport from Rust |
| [`activitykit`](crates/activitykit) | ActivityKit — Live Activities and Dynamic Island from Rust |
| [`adattributionkit`](crates/adattributionkit) | AdAttributionKit — ad attribution from Rust |
| [`adservices`](crates/adservices) | AdServices — ad attribution from Rust |
| [`adsupport`](crates/adsupport) | AdSupport — advertising identifier from Rust |
| [`alarmkit`](crates/alarmkit) | AlarmKit — alarm management from Rust |
| [`appintents`](crates/appintents) | AppIntents — Siri shortcuts and Spotlight integration from Rust |
| [`appmigrationkit`](crates/appmigrationkit) | AppMigrationKit — app migration utilities from Rust |
| [`apptrackingtransparency`](crates/apptrackingtransparency) | AppTrackingTransparency — tracking permission from Rust |
| [`arkit`](crates/arkit) | ARKit — augmented reality from Rust |
| [`assignables`](crates/assignables) | Assignables — education assignment management from Rust |
| [`authenticationservices`](crates/authenticationservices) | AuthenticationServices — Sign in with Apple from Rust |
| [`automateddeviceenrollment`](crates/automateddeviceenrollment) | AutomatedDeviceEnrollment — MDM enrollment from Rust |
| [`automaticassessmentconfiguration`](crates/automaticassessmentconfiguration) | AutomaticAssessmentConfiguration — exam lockdown from Rust |
| [`avfaudio`](crates/avfaudio) | AVFAudio — audio playback, recording, and processing from Rust |
| [`avfoundation`](crates/avfoundation) | AVFoundation — media capture, playback, and editing from Rust |
| [`avkit`](crates/avkit) | AVKit — media playback UI from Rust |
| [`backgroundassets`](crates/backgroundassets) | BackgroundAssets — background asset downloads from Rust |
| [`backgroundtasks`](crates/backgroundtasks) | BackgroundTasks — background work scheduling from Rust |
| [`browserenginecore`](crates/browserenginecore) | BrowserEngineCore — browser engine hosting from Rust |
| [`browserenginekit`](crates/browserenginekit) | BrowserEngineKit — browser engine integration from Rust |
| [`browserkit`](crates/browserkit) | BrowserKit — browser kit from Rust |
| [`callkit`](crates/callkit) | CallKit — VoIP call integration from Rust |
| [`carkey`](crates/carkey) | CarKey — digital car keys from Rust |
| [`charts`](crates/charts) | Swift Charts — data visualization from Rust ⚡ |
| [`cinematic`](crates/cinematic) | Cinematic — cinematic video processing from Rust |
| [`clockkit`](crates/clockkit) | ClockKit — watchOS complications from Rust |
| [`cloudkit`](crates/cloudkit) | CloudKit — iCloud database from Rust |
| [`combine`](crates/combine) | Combine publisher/subscriber bridge from Rust ⚡ |
| [`compositorservices`](crates/compositorservices) | CompositorServices — visionOS rendering from Rust |
| [`contactprovider`](crates/contactprovider) | ContactProvider — contact provider extensions from Rust |
| [`contacts`](crates/contacts) | Contacts — address book access from Rust |
| [`corebluetooth`](crates/corebluetooth) | Core Bluetooth — BLE from Rust |
| [`coredata`](crates/coredata) | Core Data — persistent object graph from Rust |
| [`coregraphics`](crates/coregraphics) | Core Graphics — 2D drawing from Rust |
| [`corehaptics`](crates/corehaptics) | Core Haptics — haptic feedback from Rust |
| [`corehid`](crates/corehid) | CoreHID — USB and Bluetooth HID devices from Rust |
| [`coreimage`](crates/coreimage) | Core Image — image processing and filters from Rust |
| [`corelocation`](crates/corelocation) | Core Location — GPS and location services from Rust |
| [`coreml`](crates/coreml) | Core ML — on-device machine learning from Rust |
| [`coremotion`](crates/coremotion) | Core Motion — accelerometer and gyroscope from Rust |
| [`corenfc`](crates/corenfc) | Core NFC — NFC tag reading and writing from Rust |
| [`corespotlight`](crates/corespotlight) | Core Spotlight — search indexing from Rust |
| [`coretext`](crates/coretext) | Core Text — text layout and font handling from Rust |
| [`coretransferable`](crates/coretransferable) | CoreTransferable — drag-and-drop and sharing from Rust |
| [`createml`](crates/createml) | Create ML — train machine learning models from Rust |
| [`cryptokit`](crates/cryptokit) | CryptoKit — cryptography from Rust |
| [`cryptotokenkit`](crates/cryptotokenkit) | CryptoTokenKit — smart cards and crypto tokens from Rust |
| [`datadetection`](crates/datadetection) | DataDetection — structured data extraction from Rust |
| [`declaredagerange`](crates/declaredagerange) | DeclaredAgeRange — age range declaration from Rust |
| [`deviceactivity`](crates/deviceactivity) | DeviceActivity — Screen Time monitoring from Rust |
| [`devicecheck`](crates/devicecheck) | DeviceCheck — device attestation from Rust |
| [`devicediscoveryextension`](crates/devicediscoveryextension) | DeviceDiscoveryExtension — streaming device discovery from Rust |
| [`dockkit`](crates/dockkit) | DockKit — motorized stand control from Rust |
| [`energykit`](crates/energykit) | EnergyKit — energy usage information from Rust |
| [`eventkit`](crates/eventkit) | EventKit — calendar and reminders from Rust |
| [`extensionkit`](crates/extensionkit) | ExtensionKit — app extensions from Rust |
| [`familycontrols`](crates/familycontrols) | FamilyControls — parental controls from Rust |
| [`fileprovider`](crates/fileprovider) | FileProvider — file sync from Rust |
| [`financekit`](crates/financekit) | FinanceKit — financial data from Rust |
| [`foundation-models`](crates/foundation-models) | Intelligence on-device LLM from Rust ⚡ |
| [`fskit`](crates/fskit) | FSKit — file system extensions from Rust |
| [`gamecontroller`](crates/gamecontroller) | GameController — controller input from Rust |
| [`gamekit`](crates/gamekit) | GameKit — Game Center from Rust |
| [`gameplaykit`](crates/gameplaykit) | GameplayKit — game logic from Rust |
| [`gamesave`](crates/gamesave) | GameSave — game save management from Rust |
| [`geotoolbox`](crates/geotoolbox) | GeoToolbox — geographic utilities from Rust |
| [`groupactivities`](crates/groupactivities) | GroupActivities — SharePlay from Rust |
| [`healthkit`](crates/healthkit) | HealthKit — health and fitness data from Rust |
| [`homekit`](crates/homekit) | HomeKit — smart home control from Rust |
| [`identitylookup`](crates/identitylookup) | IdentityLookup — caller ID and message filtering from Rust |
| [`imageplayground`](crates/imageplayground) | Image Playground — AI image generation from Rust |
| [`immersivemediasupport`](crates/immersivemediasupport) | ImmersiveMediaSupport — immersive media playback from Rust |
| [`intents`](crates/intents) | Intents — Siri intents and shortcuts from Rust |
| [`ituneslibrary`](crates/ituneslibrary) | iTunesLibrary — Music library access from Rust |
| [`journalingsuggestions`](crates/journalingsuggestions) | JournalingSuggestions — journaling suggestion picker from Rust |
| [`linkpresentation`](crates/linkpresentation) | LinkPresentation — URL previews from Rust |
| [`livecommunicationkit`](crates/livecommunicationkit) | LiveCommunicationKit — live calling from Rust |
| [`localauthentication`](crates/localauthentication) | LocalAuthentication — biometric auth from Rust |
| [`lockedcameracapture`](crates/lockedcameracapture) | LockedCameraCapture — locked screen camera capture from Rust |
| [`managedapp`](crates/managedapp) | ManagedApp — managed app configuration from Rust |
| [`managedappdistribution`](crates/managedappdistribution) | ManagedAppDistribution — enterprise app distribution from Rust |
| [`managedsettings`](crates/managedsettings) | ManagedSettings — device restrictions from Rust |
| [`mapkit`](crates/mapkit) | MapKit — maps and directions from Rust |
| [`marketplacekit`](crates/marketplacekit) | MarketplaceKit — alternative app marketplace from Rust |
| [`matter`](crates/matter) | Matter — smart home connectivity from Rust |
| [`mediaaccessibility`](crates/mediaaccessibility) | MediaAccessibility — closed captions and audio descriptions from Rust |
| [`mediaextension`](crates/mediaextension) | MediaExtension — media codec extensions from Rust |
| [`mediaplayer`](crates/mediaplayer) | MediaPlayer — music and media playback from Rust |
| [`messages`](crates/messages) | Messages — iMessage app extensions from Rust |
| [`messageui`](crates/messageui) | MessageUI — in-app email and SMS compose from Rust |
| [`metal`](crates/metal) | Metal — GPU programming from Rust ⚡ |
| [`metalkit`](crates/metalkit) | MetalKit — Metal utilities from Rust |
| [`metalperformanceshadersgraph`](crates/metalperformanceshadersgraph) | MPS Graph — GPU ML graph operations from Rust |
| [`metrickit`](crates/metrickit) | MetricKit — app diagnostics from Rust |
| [`mlcompute`](crates/mlcompute) | MLCompute — ML compute operations from Rust |
| [`modelio`](crates/modelio) | Model I/O — 3D model import/export from Rust |
| [`multipeerconnectivity`](crates/multipeerconnectivity) | MultipeerConnectivity — peer-to-peer networking from Rust |
| [`musickit`](crates/musickit) | MusicKit — Apple Music integration from Rust |
| [`naturallanguage`](crates/naturallanguage) | NaturalLanguage — text processing and NLP from Rust |
| [`nearbyinteraction`](crates/nearbyinteraction) | NearbyInteraction — UWB ranging from Rust |
| [`network`](crates/network) | Network framework — modern networking from Rust |
| [`networkextension`](crates/networkextension) | NetworkExtension — VPN and content filtering from Rust |
| [`oslog`](crates/oslog) | OSLog — unified logging from Rust |
| [`paperkit`](crates/paperkit) | PaperKit — paper detection and interaction from Rust |
| [`passkit`](crates/passkit) | PassKit — Wallet and Apple Pay from Rust |
| [`pdfkit`](crates/pdfkit) | PDFKit — PDF viewing and annotation from Rust |
| [`pencilkit`](crates/pencilkit) | PencilKit — drawing and handwriting from Rust |
| [`permissionkit`](crates/permissionkit) | PermissionKit — permission management from Rust |
| [`photos`](crates/photos) | Photos — photo library access from Rust |
| [`photosui`](crates/photosui) | PhotosUI — photo picker from Rust |
| [`proximityreader`](crates/proximityreader) | ProximityReader — Tap to Pay on iPhone from Rust |
| [`pushkit`](crates/pushkit) | PushKit — VoIP and complication push notifications from Rust |
| [`quicklook`](crates/quicklook) | QuickLook — file previews from Rust |
| [`quicklookthumbnailing`](crates/quicklookthumbnailing) | QuickLook Thumbnailing — file thumbnail generation from Rust |
| [`quicklookui`](crates/quicklookui) | QuickLookUI — QuickLook preview panel from Rust |
| [`realityfoundation`](crates/realityfoundation) | RealityFoundation — RealityKit foundation types from Rust |
| [`relevancekit`](crates/relevancekit) | RelevanceKit — relevance engine from Rust |
| [`replaykit`](crates/replaykit) | ReplayKit — screen recording and broadcasting from Rust |
| [`roomplan`](crates/roomplan) | RoomPlan — 3D room scanning with LiDAR from Rust |
| [`safariservices`](crates/safariservices) | SafariServices — in-app browser from Rust |
| [`scenekit`](crates/scenekit) | SceneKit — 3D rendering from Rust |
| [`screencapturekit`](crates/screencapturekit) | ScreenCaptureKit — screen recording from Rust |
| [`secureelementcredential`](crates/secureelementcredential) | SecureElementCredential — secure element credentials from Rust |
| [`sensitivecontentanalysis`](crates/sensitivecontentanalysis) | SensitiveContentAnalysis — CSAM/nudity detection from Rust |
| [`sensorkit`](crates/sensorkit) | SensorKit — research sensor data from Rust |
| [`servicesaccountlinking`](crates/servicesaccountlinking) | ServicesAccountLinking — service account linking from Rust |
| [`sharedwithyou`](crates/sharedwithyou) | SharedWithYou — Messages collaboration from Rust |
| [`shazamkit`](crates/shazamkit) | ShazamKit — music recognition from Rust |
| [`soundanalysis`](crates/soundanalysis) | SoundAnalysis — audio classification from Rust |
| [`spatial`](crates/spatial) | Spatial framework — 3D math types from Rust ⚡ |
| [`speech`](crates/speech) | Speech — speech recognition from Rust |
| [`spritekit`](crates/spritekit) | SpriteKit — 2D game engine from Rust |
| [`stickerkit`](crates/stickerkit) | StickerKit — iMessage sticker packs from Rust |
| [`storekit`](crates/storekit) | StoreKit — in-app purchases and subscriptions from Rust |
| [`swift-data`](crates/swift-data) | Persistent key-value store backed by UserDefaults ⚡ |
| [`swiftdata`](crates/swiftdata) | SwiftData — modern persistence framework from Rust |
| [`symbols`](crates/symbols) | Symbols — SF Symbols metadata from Rust |
| [`tabulardata`](crates/tabulardata) | TabularData — data tables and CSV from Rust |
| [`telephonymessagingkit`](crates/telephonymessagingkit) | TelephonyMessagingKit — telephony messaging from Rust |
| [`tipkit`](crates/tipkit) | TipKit — in-app tips and hints from Rust |
| [`translation`](crates/translation) | Translation framework — on-device text translation from Rust |
| [`uniformtypeidentifiers`](crates/uniformtypeidentifiers) | UniformTypeIdentifiers — UTI system from Rust |
| [`usernotifications`](crates/usernotifications) | UserNotifications — push and local notifications from Rust |
| [`videosubscriberaccount`](crates/videosubscriberaccount) | VideoSubscriberAccount — TV provider authentication from Rust |
| [`virtualization`](crates/virtualization) | Virtualization — virtual machines from Rust |
| [`vision`](crates/vision) | Vision framework — image analysis and computer vision from Rust |
| [`visionkit`](crates/visionkit) | VisionKit — document scanning and visual lookup from Rust |
| [`visualintelligence`](crates/visualintelligence) | VisualIntelligence — visual lookup and intelligence from Rust |
| [`weatherkit`](crates/weatherkit) | WeatherKit — weather data from Rust |
| [`webkit`](crates/webkit) | WebKit — web views from Rust |
| [`widgetkit`](crates/widgetkit) | WidgetKit — reload widget timelines from Rust ⚡ |
| [`wifiaware`](crates/wifiaware) | WiFiAware — WiFi Aware networking from Rust |
| [`wifiinfrastructure`](crates/wifiinfrastructure) | WiFiInfrastructure — WiFi infrastructure management from Rust |
| [`wirelessinsights`](crates/wirelessinsights) | WirelessInsights — wireless diagnostics from Rust |
| [`workoutkit`](crates/workoutkit) | WorkoutKit — workout composition from Rust |

</details>

## Platform Support

| | macOS | iOS | visionOS |
|--|-------|-----|----------|
| SwiftUI | ✅ | ✅ | ✅ |
| Metal | ✅ | ✅ | ✅ |
| RealityKit | ✅ | ✅ | ✅ |
| Combine | ✅ | ✅ | ✅ |
| Persistence | ✅ | ✅ | ✅ |

## Tests

```bash
cargo test -p winit-swift -p swiftui-app   # 55 tests
cargo test --workspace -- --test-threads=1  # full suite
```

## Citation

If you use rswift in your research or project, please cite:

```bibtex
@software{rswift,
  author       = {Eugene Hauptmann},
  title        = {rswift},
  year         = {2025},
  url          = {https://github.com/eugenehp/rswift},
  note         = {Build native Apple apps from Rust}
}
```

## License

GPL-3.0 — see [LICENSE](LICENSE)

Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)
