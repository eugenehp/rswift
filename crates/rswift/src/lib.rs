//! # rswift — Build native Apple apps from Rust
//!
//! Unified access to Apple frameworks through a single crate with feature flags.
//!
//! ## Quick start
//!
//! ```toml
//! [dependencies]
//! rswift = { version = "0.0.1", features = ["avfaudio", "metal"] }
//! ```
//!
//! ```ignore
//! use rswift::prelude::*;
//!
//! assert!(avfaudio::is_available());
//! let engine = avfaudio::AudioEngine::new();
//! let (sr, ch) = engine.output_format();
//! println!("Audio: {sr} Hz, {ch} ch");
//! ```
//!
//! ## Feature categories
//!
//! | Feature      | Frameworks included |
//! |--------------|-------------------|
//! | `runtime`    | swift-runtime, swift-runtime-sys |
//! | `ui`         | swiftui, swiftui-app, charts, tipkit, widgetkit |
//! | `media`      | avfaudio, avfoundation, avkit, mediaplayer, musickit, … |
//! | `ml`         | coreml, createml, vision, naturallanguage, foundation-models, translation |
//! | `graphics`   | metal, scenekit, spritekit, coregraphics, coreimage, coretext |
//! | `data`       | swift-data, swiftdata, coredata, cloudkit, combine |
//! | `auth`       | authenticationservices, cryptokit, localauthentication, … |
//! | `network`    | network, networkextension, multipeerconnectivity, webkit, … |
//! | `location`   | corelocation, mapkit |
//! | `health`     | healthkit, workoutkit |
//! | `commerce`   | storekit, passkit, financekit |
//! | `games`      | gamekit, gamecontroller, gameplaykit, gamesave |
//! | `hardware`   | corebluetooth, corehaptics, corenfc, coremotion, … |
//! | `system`     | oslog, backgroundtasks, usernotifications, pushkit, … |
//! | `comms`      | callkit, messageui, contacts, groupactivities, … |
//! | `ar-vr`      | arkit, realitykit, spatial, roomplan, compositorservices |
//! | `full`       | All of the above |
//!
//! ## Individual features
//!
//! Every framework is also available as its own feature:
//!
//! ```toml
//! rswift = { version = "0.0.1", features = ["avfaudio", "corelocation", "storekit"] }
//! ```

//!
//! ## Citation
//!
//! ```bibtex
//! @software{rswift,
//!   author       = {Eugene Hauptmann},
//!   title        = {rswift},
//!   year         = {2025},
//!   url          = {https://github.com/eugenehp/rswift},
//!   note         = {Build native Apple apps from Rust}
//! }
//! ```
//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

// ── Re-exports ──────────────────────────────────────────────────────────────
//
// Each enabled feature re-exports the corresponding crate at the top level.
// Use `rswift::avfaudio`, `rswift::metal`, etc.

// Core
#[cfg(feature = "swift-runtime")]
pub use swift_runtime;
#[cfg(feature = "swift-runtime-sys")]
pub use swift_runtime_sys;
#[cfg(feature = "apple-sys-helpers")]
pub use apple_sys_helpers;
#[cfg(feature = "foundation")]
pub use foundation;
#[cfg(feature = "security")]
pub use security;

// UI
#[cfg(feature = "swiftui")]
pub use swiftui;
#[cfg(feature = "swiftui-app")]
pub use swiftui_app;
#[cfg(feature = "charts")]
pub use swift_charts as charts;
#[cfg(feature = "tipkit")]
pub use tipkit;
#[cfg(feature = "widgetkit")]
pub use widgetkit;

// Media & AV
#[cfg(feature = "avfaudio")]
pub use avfaudio;
#[cfg(feature = "avfoundation")]
pub use avfoundation;
#[cfg(feature = "avkit")]
pub use avkit;
#[cfg(feature = "mediaplayer")]
pub use mediaplayer;
#[cfg(feature = "musickit")]
pub use musickit;
#[cfg(feature = "cinematic")]
pub use cinematic;
#[cfg(feature = "shazamkit")]
pub use shazamkit;
#[cfg(feature = "soundanalysis")]
pub use soundanalysis;
#[cfg(feature = "speech")]
pub use speech;
#[cfg(feature = "screencapturekit")]
pub use screencapturekit;
#[cfg(feature = "replaykit")]
pub use replaykit;
#[cfg(feature = "photosui")]
pub use photosui;
#[cfg(feature = "coremedia")]
pub use coremedia;
#[cfg(feature = "audiotoolbox")]
pub use audiotoolbox;

// ML & AI
#[cfg(feature = "coreml")]
pub use coreml;
#[cfg(feature = "createml")]
pub use createml;
#[cfg(feature = "vision")]
pub use vision;
#[cfg(feature = "naturallanguage")]
pub use naturallanguage;
#[cfg(feature = "foundation-models")]
pub use foundation_models;
#[cfg(feature = "translation")]
pub use translation;
#[cfg(feature = "accelerate")]
pub use accelerate;

// Graphics & Animation
#[cfg(feature = "coreanimation")]
pub use coreanimation;
#[cfg(feature = "metal")]
pub use metal;
#[cfg(feature = "scenekit")]
pub use scenekit;
#[cfg(feature = "spritekit")]
pub use spritekit;
#[cfg(feature = "coregraphics")]
pub use coregraphics;
#[cfg(feature = "coreimage")]
pub use coreimage;
#[cfg(feature = "coretext")]
pub use coretext;

// Data & Persistence
#[cfg(feature = "swift-data")]
pub use swift_data;
#[cfg(feature = "swiftdata")]
pub use swiftdata;
#[cfg(feature = "coredata")]
pub use coredata;
#[cfg(feature = "cloudkit")]
pub use cloudkit;
#[cfg(feature = "combine")]
pub use combine;

// Auth & Security
#[cfg(feature = "authenticationservices")]
pub use authenticationservices;
#[cfg(feature = "cryptokit")]
pub use cryptokit;
#[cfg(feature = "cryptotokenkit")]
pub use cryptotokenkit;
#[cfg(feature = "localauthentication")]
pub use localauthentication;
#[cfg(feature = "devicecheck")]
pub use devicecheck;

// Networking
#[cfg(feature = "network-framework")]
pub use network;
#[cfg(feature = "networkextension")]
pub use networkextension;
#[cfg(feature = "multipeerconnectivity")]
pub use multipeerconnectivity;
#[cfg(feature = "linkpresentation")]
pub use linkpresentation;
#[cfg(feature = "webkit")]
pub use webkit;

// Location
#[cfg(feature = "corelocation")]
pub use corelocation;
#[cfg(feature = "mapkit")]
pub use mapkit;

// Health
#[cfg(feature = "healthkit")]
pub use healthkit;
#[cfg(feature = "workoutkit")]
pub use workoutkit;

// Commerce
#[cfg(feature = "storekit")]
pub use storekit;
#[cfg(feature = "passkit")]
pub use passkit;
#[cfg(feature = "financekit")]
pub use financekit;

// Games
#[cfg(feature = "gamekit")]
pub use gamekit;
#[cfg(feature = "gamecontroller")]
pub use gamecontroller;
#[cfg(feature = "gameplaykit")]
pub use gameplaykit;
#[cfg(feature = "gamesave")]
pub use gamesave;

// Hardware & Sensors
#[cfg(feature = "corebluetooth")]
pub use corebluetooth;
#[cfg(feature = "corehaptics")]
pub use corehaptics;
#[cfg(feature = "corehid")]
pub use corehid;
#[cfg(feature = "coremotion")]
pub use coremotion;
#[cfg(feature = "corenfc")]
pub use corenfc;
#[cfg(feature = "nearbyinteraction")]
pub use nearbyinteraction;
#[cfg(feature = "dockkit")]
pub use dockkit;
#[cfg(feature = "sensorkit")]
pub use sensorkit;

// System
#[cfg(feature = "oslog")]
pub use oslog;
#[cfg(feature = "backgroundtasks")]
pub use backgroundtasks;
#[cfg(feature = "extensionkit")]
pub use extensionkit;
#[cfg(feature = "uniformtypeidentifiers")]
pub use uniformtypeidentifiers;
#[cfg(feature = "usernotifications")]
pub use usernotifications;
#[cfg(feature = "pushkit")]
pub use pushkit;
#[cfg(feature = "eventkit")]
pub use eventkit;

// Communication
#[cfg(feature = "callkit")]
pub use callkit;
#[cfg(feature = "messageui")]
pub use messageui;
#[cfg(feature = "contacts")]
pub use contacts;
#[cfg(feature = "livecommunicationkit")]
pub use livecommunicationkit;
#[cfg(feature = "groupactivities")]
pub use groupactivities;

// AR / VR / Spatial
#[cfg(feature = "arkit")]
pub use arkit;
#[cfg(feature = "realitykit")]
pub use realitykit;
#[cfg(feature = "realityfoundation")]
pub use realityfoundation;
#[cfg(feature = "spatial")]
pub use spatial;
#[cfg(feature = "roomplan")]
pub use roomplan;
#[cfg(feature = "compositorservices")]
pub use compositorservices;

// Additional
#[cfg(feature = "accessibility")]
pub use accessibility;
#[cfg(feature = "accessorysetupkit")]
pub use accessorysetupkit;
#[cfg(feature = "activitykit")]
pub use activitykit;
#[cfg(feature = "adservices")]
pub use adservices;
#[cfg(feature = "adsupport")]
pub use adsupport;
#[cfg(feature = "appintents")]
pub use appintents;
#[cfg(feature = "apptrackingtransparency")]
pub use apptrackingtransparency;
#[cfg(feature = "corespotlight")]
pub use corespotlight;
#[cfg(feature = "coretransferable")]
pub use coretransferable;
#[cfg(feature = "datadetection")]
pub use datadetection;
#[cfg(feature = "fileprovider")]
pub use fileprovider;
#[cfg(feature = "homekit")]
pub use homekit;
#[cfg(feature = "identitylookup")]
pub use identitylookup;
#[cfg(feature = "imageplayground")]
pub use imageplayground;
#[cfg(feature = "intents")]
pub use intents;
#[cfg(feature = "metrickit")]
pub use metrickit;
#[cfg(feature = "pdfkit")]
pub use pdfkit;
#[cfg(feature = "pencilkit")]
pub use pencilkit;
#[cfg(feature = "quicklook")]
pub use quicklook;
#[cfg(feature = "safariservices")]
pub use safariservices;
#[cfg(feature = "videosubscriberaccount")]
pub use videosubscriberaccount;
#[cfg(feature = "visionkit")]
pub use visionkit;
#[cfg(feature = "weatherkit")]
pub use weatherkit;

// ── Prelude ─────────────────────────────────────────────────────────────────

/// Convenience prelude — `use rswift::prelude::*` to import all enabled frameworks.
///
/// Each framework is available as a module name matching its feature flag.
/// Only frameworks whose features are enabled will be in scope.
///
/// ```ignore
/// use rswift::prelude::*;
///
/// // With features = ["avfaudio", "avfoundation"]
/// let engine = avfaudio::AudioEngine::new();
/// let player = avfoundation::Player::new("/path/to/video.mp4");
/// ```
pub mod prelude {
    // Core
    #[cfg(feature = "swift-runtime")]
    pub use swift_runtime;
    #[cfg(feature = "apple-sys-helpers")]
    pub use apple_sys_helpers;
    #[cfg(feature = "foundation")]
    pub use foundation;
    #[cfg(feature = "security")]
    pub use security;

    // UI
    #[cfg(feature = "swiftui")]
    pub use swiftui;
    #[cfg(feature = "charts")]
    pub use swift_charts as charts;

    // Media & AV
    #[cfg(feature = "avfaudio")]
    pub use avfaudio;
    #[cfg(feature = "avfoundation")]
    pub use avfoundation;
    #[cfg(feature = "avkit")]
    pub use avkit;
    #[cfg(feature = "mediaplayer")]
    pub use mediaplayer;
    #[cfg(feature = "musickit")]
    pub use musickit;
    #[cfg(feature = "screencapturekit")]
    pub use screencapturekit;

    // ML & AI
    #[cfg(feature = "coreml")]
    pub use coreml;
    #[cfg(feature = "vision")]
    pub use vision;
    #[cfg(feature = "foundation-models")]
    pub use foundation_models;
    #[cfg(feature = "translation")]
    pub use translation;

    // Graphics
    #[cfg(feature = "coreanimation")]
    pub use coreanimation;
    #[cfg(feature = "metal")]
    pub use metal;
    #[cfg(feature = "accelerate")]
    pub use accelerate;
    #[cfg(feature = "coregraphics")]
    pub use coregraphics;
    #[cfg(feature = "coreimage")]
    pub use coreimage;

    // Data
    #[cfg(feature = "swift-data")]
    pub use swift_data;
    #[cfg(feature = "cloudkit")]
    pub use cloudkit;
    #[cfg(feature = "combine")]
    pub use combine;

    // Auth
    #[cfg(feature = "authenticationservices")]
    pub use authenticationservices;
    #[cfg(feature = "cryptokit")]
    pub use cryptokit;

    // Network
    #[cfg(feature = "network-framework")]
    pub use network;
    #[cfg(feature = "webkit")]
    pub use webkit;

    // Location
    #[cfg(feature = "corelocation")]
    pub use corelocation;
    #[cfg(feature = "mapkit")]
    pub use mapkit;

    // Health
    #[cfg(feature = "healthkit")]
    pub use healthkit;

    // Commerce
    #[cfg(feature = "storekit")]
    pub use storekit;

    // Games
    #[cfg(feature = "gamekit")]
    pub use gamekit;

    // Hardware
    #[cfg(feature = "corebluetooth")]
    pub use corebluetooth;
    #[cfg(feature = "corehaptics")]
    pub use corehaptics;
    #[cfg(feature = "coremotion")]
    pub use coremotion;

    // System
    #[cfg(feature = "usernotifications")]
    pub use usernotifications;

    // AR / VR
    #[cfg(feature = "arkit")]
    pub use arkit;
    #[cfg(feature = "realitykit")]
    pub use realitykit;
    #[cfg(feature = "spatial")]
    pub use spatial;
}
