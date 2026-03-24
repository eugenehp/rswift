//! Apple platform definitions from LLVM's MachO.def.
//!
//! This module exposes two closely related types:
//!
//! * [`ApplePlatform`] — a `Copy` enum whose variants map 1-to-1 to LLVM's
//!   `MachO::PlatformType` identifiers.  It is the primary API surface: parse
//!   from a Rust target triple, look up an SDK name, or navigate the
//!   simulator/device relationship.
//!
//! * [`Platform`] — a plain `Copy` struct of `&'static str` and `u32` fields
//!   that carries the raw LLVM metadata for a single platform (the data that
//!   lives in the `PLATFORM(…)` macro rows in `MachO.def`).  Obtain one via
//!   [`ApplePlatform::metadata()`] or `Platform::from(platform)`.
//!
//! # Error handling
//!
//! All fallible constructors return [`ParsePlatformError`] on failure.  It
//! implements both `Display` and `std::error::Error`, so it composes naturally
//! with `anyhow`, `thiserror`, or plain `?` propagation.
//!
//! # Examples
//!
//! ```
//! use apple_platforms::platform::ApplePlatform;
//!
//! // Parse from a Rust target triple
//! let sim = ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap();
//! assert_eq!(sim, ApplePlatform::IOSSimulator);
//! assert!(sim.is_simulator());
//! assert!(!sim.is_device());
//!
//! // Navigate to the device platform
//! let device = sim.device();
//! assert_eq!(device, ApplePlatform::IOS);
//!
//! // Navigate back to the simulator
//! assert_eq!(device.simulator(), Some(ApplePlatform::IOSSimulator));
//!
//! // SDK name for xcrun
//! assert_eq!(sim.sdk(), Some("iphonesimulator"));
//!
//! // Iterate all platforms
//! let count = ApplePlatform::iter().count();
//! assert_eq!(count, 13); // Unknown + 12 real platforms
//!
//! // Sort by Mach-O ID (PartialOrd / Ord)
//! let mut platforms = vec![ApplePlatform::XrOS, ApplePlatform::MacOS];
//! platforms.sort();
//! assert_eq!(platforms[0], ApplePlatform::MacOS);
//! ```
//!
//! Source: <https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def>

use std::fmt;

// ── Error type ───────────────────────────────────────────────────────────────

/// Error returned when a platform name, ID, or target triple cannot be
/// recognised as a known Apple platform.
///
/// Implements [`std::error::Error`] and [`Display`](std::fmt::Display), so it
/// composes naturally with `?` and `anyhow`/`thiserror`.
///
/// ```
/// use apple_platforms::platform::{ApplePlatform, ParsePlatformError};
///
/// let err: ParsePlatformError = "bogus".parse::<ApplePlatform>().unwrap_err();
/// assert!(err.to_string().contains("bogus"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePlatformError(String);

impl ParsePlatformError {
    pub(crate) fn new(s: impl fmt::Display) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for ParsePlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown Apple platform: {:?}", self.0)
    }
}

impl std::error::Error for ParsePlatformError {}

// ── Platform metadata ────────────────────────────────────────────────────────

/// Static metadata for a single Apple platform.
///
/// All string fields borrow from `'static` storage; there is no lifetime
/// parameter.  Obtain instances via [`ApplePlatform::metadata()`] or the
/// [`From`] / [`TryFrom`] conversions below.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Platform {
    /// Enum-style name (e.g. `"MACOS"`).
    pub platform: &'static str,
    /// Mach-O platform ID.
    pub id: u32,
    /// Lowercase name (e.g. `"macos"`).
    pub name: &'static str,
    /// Build name (e.g. `"macos"`).
    pub build_name: &'static str,
    /// Clang target OS component (e.g. `"macos"`, `"ios-simulator"`).
    pub target: &'static str,
    /// TAPI target (e.g. `"macos"`, `"maccatalyst"`).
    pub tapi_target: &'static str,
    /// Marketing name (e.g. `"macOS"`, `"iOS Simulator"`).
    pub marketing: &'static str,
    /// SDK name for `xcrun --sdk <name>` (e.g. `"macosx"`, `"iphoneos"`).
    /// `None` for the `Unknown` platform.
    pub sdk: Option<&'static str>,
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Platform")
            .field("platform", &self.platform)
            .field("id", &self.id)
            .field("name", &self.name)
            .field("build_name", &self.build_name)
            .field("target", &self.target)
            .field("tapi_target", &self.tapi_target)
            .field("marketing", &self.marketing)
            .field("sdk", &self.sdk)
            .finish()
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (id={}, target={})", self.marketing, self.id, self.target)
    }
}

// ── ApplePlatform enum ───────────────────────────────────────────────────────

/// Apple platform enumeration.
///
/// Matches LLVM's `MachO::PlatformType` identifiers.  Numeric discriminants
/// are stable and correspond to Mach-O platform IDs, so `PartialOrd`/`Ord`
/// order platforms by ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApplePlatform {
    Unknown          = 0,
    MacOS            = 1,
    IOS              = 2,
    TvOS             = 3,
    WatchOS          = 4,
    BridgeOS         = 5,
    MacCatalyst      = 6,
    IOSSimulator     = 7,
    TvOSSimulator    = 8,
    WatchOSSimulator = 9,
    DriverKit        = 10,
    XrOS             = 11,
    XrOSSimulator    = 12,
}

impl Default for ApplePlatform {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ApplePlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.metadata().marketing)
    }
}

impl ApplePlatform {
    /// All platform variants in Mach-O ID order.
    pub const ALL: [ApplePlatform; 13] = [
        Self::Unknown,
        Self::MacOS,
        Self::IOS,
        Self::TvOS,
        Self::WatchOS,
        Self::BridgeOS,
        Self::MacCatalyst,
        Self::IOSSimulator,
        Self::TvOSSimulator,
        Self::WatchOSSimulator,
        Self::DriverKit,
        Self::XrOS,
        Self::XrOSSimulator,
    ];

    /// Iterate over all platform variants in Mach-O ID order.
    pub fn iter() -> impl Iterator<Item = ApplePlatform> {
        Self::ALL.into_iter()
    }

    /// Returns this platform's static [`Platform`] metadata.
    #[inline]
    pub fn metadata(self) -> Platform {
        PLATFORMS[self as usize]
    }

    /// The Mach-O platform ID.
    #[inline]
    pub fn id(self) -> u32 {
        self.metadata().id
    }

    // ── Construction ─────────────────────────────────────────────────────────

    /// Create from a Mach-O platform ID.
    ///
    /// Returns `None` for unrecognised IDs instead of silently falling back to
    /// `Unknown`.  Use [`TryFrom<u32>`] for the same behaviour as a trait impl.
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            0  => Some(Self::Unknown),
            1  => Some(Self::MacOS),
            2  => Some(Self::IOS),
            3  => Some(Self::TvOS),
            4  => Some(Self::WatchOS),
            5  => Some(Self::BridgeOS),
            6  => Some(Self::MacCatalyst),
            7  => Some(Self::IOSSimulator),
            8  => Some(Self::TvOSSimulator),
            9  => Some(Self::WatchOSSimulator),
            10 => Some(Self::DriverKit),
            11 => Some(Self::XrOS),
            12 => Some(Self::XrOSSimulator),
            _  => None,
        }
    }

    /// Create from a Rust target triple (e.g. `"aarch64-apple-ios-sim"`).
    ///
    /// Returns an error if the triple is not a recognised Apple target.
    ///
    /// ```
    /// use apple_platforms::platform::ApplePlatform;
    /// assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-darwin").unwrap(), ApplePlatform::MacOS);
    /// assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap(), ApplePlatform::IOSSimulator);
    /// assert!(ApplePlatform::from_rust_triple("x86_64-unknown-linux-gnu").is_err());
    /// ```
    pub fn from_rust_triple(target: &str) -> Result<Self, ParsePlatformError> {
        match target {
            // macOS
            "aarch64-apple-darwin" | "x86_64-apple-darwin"
                => Ok(Self::MacOS),
            // iOS device
            "aarch64-apple-ios" | "armv7-apple-ios" | "armv7s-apple-ios"
                => Ok(Self::IOS),
            // iOS simulator  (x86_64-apple-ios is always sim)
            "aarch64-apple-ios-sim" | "x86_64-apple-ios" | "i386-apple-ios"
                => Ok(Self::IOSSimulator),
            // Mac Catalyst (macOS SDK, macabi ABI)
            "aarch64-apple-ios-macabi" | "x86_64-apple-ios-macabi"
                => Ok(Self::MacCatalyst),
            // tvOS
            "aarch64-apple-tvos"
                => Ok(Self::TvOS),
            "aarch64-apple-tvos-sim" | "x86_64-apple-tvos"
                => Ok(Self::TvOSSimulator),
            // watchOS
            "aarch64-apple-watchos" | "armv7k-apple-watchos" | "arm64_32-apple-watchos"
                => Ok(Self::WatchOS),
            "aarch64-apple-watchos-sim" | "x86_64-apple-watchos-sim"
                => Ok(Self::WatchOSSimulator),
            // visionOS (xrOS internal name)
            "aarch64-apple-visionos"
                => Ok(Self::XrOS),
            "aarch64-apple-visionos-sim"
                => Ok(Self::XrOSSimulator),
            // DriverKit
            "aarch64-apple-driverkit" | "x86_64-apple-driverkit"
                => Ok(Self::DriverKit),
            _   => Err(ParsePlatformError::new(target)),
        }
    }

    // ── SDK / Clang helpers ───────────────────────────────────────────────────

    /// The SDK name for `xcrun --sdk <name>`.
    ///
    /// Returns `None` only for the `Unknown` variant.
    ///
    /// Note: [`MacCatalyst`](Self::MacCatalyst) builds use the macOS SDK
    /// (`"macosx"`) with the `macabi` ABI environment.
    ///
    /// ```
    /// use apple_platforms::platform::ApplePlatform;
    /// assert_eq!(ApplePlatform::MacOS.sdk(),          Some("macosx"));
    /// assert_eq!(ApplePlatform::IOS.sdk(),            Some("iphoneos"));
    /// assert_eq!(ApplePlatform::IOSSimulator.sdk(),   Some("iphonesimulator"));
    /// assert_eq!(ApplePlatform::MacCatalyst.sdk(),    Some("macosx"));
    /// assert_eq!(ApplePlatform::Unknown.sdk(),        None);
    /// ```
    #[inline]
    pub fn sdk(self) -> Option<&'static str> {
        self.metadata().sdk
    }

    // ── Simulator helpers ─────────────────────────────────────────────────────

    /// Whether this is a simulator platform.
    pub fn is_simulator(self) -> bool {
        matches!(
            self,
            Self::IOSSimulator | Self::TvOSSimulator | Self::WatchOSSimulator | Self::XrOSSimulator
        )
    }

    /// Whether this is a real-device platform (non-simulator, non-Unknown).
    pub fn is_device(self) -> bool {
        !self.is_simulator() && self != Self::Unknown
    }

    /// The corresponding device (non-simulator) platform.
    ///
    /// Returns `self` if already a device platform.
    ///
    /// ```
    /// use apple_platforms::platform::ApplePlatform;
    /// assert_eq!(ApplePlatform::IOSSimulator.device(), ApplePlatform::IOS);
    /// assert_eq!(ApplePlatform::IOS.device(),          ApplePlatform::IOS);
    /// ```
    pub fn device(self) -> Self {
        match self {
            Self::IOSSimulator       => Self::IOS,
            Self::TvOSSimulator      => Self::TvOS,
            Self::WatchOSSimulator   => Self::WatchOS,
            Self::XrOSSimulator      => Self::XrOS,
            other                    => other,
        }
    }

    /// The corresponding simulator platform, or `None` if this platform has
    /// no simulator counterpart (e.g. macOS, DriverKit, BridgeOS).
    ///
    /// Returns `Some(self)` if already a simulator.
    ///
    /// ```
    /// use apple_platforms::platform::ApplePlatform;
    /// assert_eq!(ApplePlatform::IOS.simulator(),            Some(ApplePlatform::IOSSimulator));
    /// assert_eq!(ApplePlatform::IOSSimulator.simulator(),   Some(ApplePlatform::IOSSimulator));
    /// assert_eq!(ApplePlatform::MacOS.simulator(),          None);
    /// assert_eq!(ApplePlatform::DriverKit.simulator(),      None);
    /// ```
    pub fn simulator(self) -> Option<Self> {
        match self {
            Self::IOS       => Some(Self::IOSSimulator),
            Self::TvOS      => Some(Self::TvOSSimulator),
            Self::WatchOS   => Some(Self::WatchOSSimulator),
            Self::XrOS      => Some(Self::XrOSSimulator),
            sim if sim.is_simulator() => Some(sim),
            _ => None,
        }
    }
}

// ── FromStr ──────────────────────────────────────────────────────────────────

impl std::str::FromStr for ApplePlatform {
    type Err = ParsePlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "macos" | "macosx" | "osx"                         => Ok(Self::MacOS),
            "ios" | "iphoneos"                                  => Ok(Self::IOS),
            "tvos" | "appletvos"                                => Ok(Self::TvOS),
            "watchos"                                           => Ok(Self::WatchOS),
            "bridgeos"                                          => Ok(Self::BridgeOS),
            "maccatalyst" | "macabi"                            => Ok(Self::MacCatalyst),
            "iossimulator" | "iphonesimulator"                  => Ok(Self::IOSSimulator),
            "tvossimulator" | "appletvsimulator"                => Ok(Self::TvOSSimulator),
            "watchossimulator" | "watchsimulator"               => Ok(Self::WatchOSSimulator),
            "driverkit"                                         => Ok(Self::DriverKit),
            "xros" | "visionos"                                 => Ok(Self::XrOS),
            "xrossimulator" | "xrsimulator" | "visionossimulator" => Ok(Self::XrOSSimulator),
            "unknown"                                           => Ok(Self::Unknown),
            _ => Err(ParsePlatformError::new(s)),
        }
    }
}

// ── TryFrom ──────────────────────────────────────────────────────────────────

impl TryFrom<u32> for ApplePlatform {
    type Error = ParsePlatformError;
    fn try_from(id: u32) -> Result<Self, Self::Error> {
        Self::from_id(id).ok_or_else(|| ParsePlatformError::new(id))
    }
}

impl TryFrom<usize> for ApplePlatform {
    type Error = ParsePlatformError;
    fn try_from(id: usize) -> Result<Self, Self::Error> {
        u32::try_from(id)
            .ok()
            .and_then(Self::from_id)
            .ok_or_else(|| ParsePlatformError::new(id))
    }
}

// ── From / TryFrom for Platform ──────────────────────────────────────────────

impl From<ApplePlatform> for Platform {
    fn from(value: ApplePlatform) -> Self {
        value.metadata()
    }
}

/// Convert a [`Platform`] back to an [`ApplePlatform`] by its numeric ID.
///
/// Fails if `p.id` does not correspond to a known platform.
impl TryFrom<Platform> for ApplePlatform {
    type Error = ParsePlatformError;
    fn try_from(p: Platform) -> Result<Self, Self::Error> {
        Self::from_id(p.id).ok_or_else(|| ParsePlatformError::new(p.id))
    }
}

// ── Platform data table ──────────────────────────────────────────────────────

// PLATFORM(platform, id, name, build_name, target, tapi_target, marketing, sdk)
const PLATFORMS: [Platform; 13] = [
    Platform { platform: "UNKNOWN",          id: 0,  name: "unknown",          build_name: "unknown",          target: "unknown",           tapi_target: "unknown",          marketing: "unknown",           sdk: None                  },
    Platform { platform: "MACOS",            id: 1,  name: "macos",            build_name: "macos",            target: "macos",             tapi_target: "macos",            marketing: "macOS",             sdk: Some("macosx")         },
    Platform { platform: "IOS",              id: 2,  name: "ios",              build_name: "ios",              target: "ios",               tapi_target: "ios",              marketing: "iOS",               sdk: Some("iphoneos")       },
    Platform { platform: "TVOS",             id: 3,  name: "tvos",             build_name: "tvos",             target: "tvos",              tapi_target: "tvos",             marketing: "tvOS",              sdk: Some("appletvos")      },
    Platform { platform: "WATCHOS",          id: 4,  name: "watchos",          build_name: "watchos",          target: "watchos",           tapi_target: "watchos",          marketing: "watchOS",           sdk: Some("watchos")        },
    Platform { platform: "BRIDGEOS",         id: 5,  name: "bridgeos",         build_name: "bridgeos",         target: "bridgeos",          tapi_target: "bridgeos",         marketing: "bridgeOS",          sdk: Some("bridgeos")       },
    Platform { platform: "MACCATALYST",      id: 6,  name: "macCatalyst",      build_name: "macCatalyst",      target: "ios-macabi",        tapi_target: "maccatalyst",      marketing: "macCatalyst",       sdk: Some("macosx")         },
    Platform { platform: "IOSSIMULATOR",     id: 7,  name: "iossimulator",     build_name: "iossimulator",     target: "ios-simulator",     tapi_target: "ios-simulator",    marketing: "iOS Simulator",     sdk: Some("iphonesimulator") },
    Platform { platform: "TVOSSIMULATOR",    id: 8,  name: "tvossimulator",    build_name: "tvossimulator",    target: "tvos-simulator",    tapi_target: "tvos-simulator",   marketing: "tvOS Simulator",    sdk: Some("appletvsimulator") },
    Platform { platform: "WATCHOSSIMULATOR", id: 9,  name: "watchossimulator", build_name: "watchossimulator", target: "watchos-simulator", tapi_target: "watchos-simulator", marketing: "watchOS Simulator", sdk: Some("watchsimulator") },
    Platform { platform: "DRIVERKIT",        id: 10, name: "driverkit",        build_name: "driverkit",        target: "driverkit",         tapi_target: "driverkit",        marketing: "DriverKit",         sdk: Some("driverkit")      },
    Platform { platform: "XROS",             id: 11, name: "xros",             build_name: "xros",             target: "xros",              tapi_target: "xros",             marketing: "visionOS",          sdk: Some("xros")           },
    Platform { platform: "XROS_SIMULATOR",   id: 12, name: "xrsimulator",      build_name: "xrsimulator",      target: "xros-simulator",    tapi_target: "xros-simulator",   marketing: "visionOS Simulator", sdk: Some("xrsimulator")   },
];

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_platforms() {
        let names: Vec<&str> = ApplePlatform::iter()
            .map(|p| p.metadata().platform)
            .collect();
        assert_eq!(names, vec![
            "UNKNOWN", "MACOS", "IOS", "TVOS", "WATCHOS", "BRIDGEOS",
            "MACCATALYST", "IOSSIMULATOR", "TVOSSIMULATOR", "WATCHOSSIMULATOR",
            "DRIVERKIT", "XROS", "XROS_SIMULATOR",
        ]);
    }

    #[test]
    fn platform_ids() {
        assert_eq!(ApplePlatform::MacOS.id(), 1);
        assert_eq!(ApplePlatform::IOS.id(), 2);
        assert_eq!(ApplePlatform::XrOS.id(), 11);
    }

    #[test]
    fn from_id_known() {
        assert_eq!(ApplePlatform::from_id(1), Some(ApplePlatform::MacOS));
        assert_eq!(ApplePlatform::from_id(11), Some(ApplePlatform::XrOS));
        assert_eq!(ApplePlatform::from_id(0), Some(ApplePlatform::Unknown));
    }

    #[test]
    fn from_id_unknown_is_none() {
        assert_eq!(ApplePlatform::from_id(999), None);
        assert_eq!(ApplePlatform::from_id(13), None);
    }

    #[test]
    fn try_from_u32() {
        assert_eq!(ApplePlatform::try_from(1u32).unwrap(), ApplePlatform::MacOS);
        assert!(ApplePlatform::try_from(99u32).is_err());
    }

    #[test]
    fn try_from_usize() {
        assert_eq!(ApplePlatform::try_from(2usize).unwrap(), ApplePlatform::IOS);
        assert!(ApplePlatform::try_from(999usize).is_err());
    }

    #[test]
    fn from_rust_triple() {
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-darwin").unwrap(), ApplePlatform::MacOS);
        assert_eq!(ApplePlatform::from_rust_triple("x86_64-apple-darwin").unwrap(), ApplePlatform::MacOS);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-ios").unwrap(), ApplePlatform::IOS);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-ios-sim").unwrap(), ApplePlatform::IOSSimulator);
        assert_eq!(ApplePlatform::from_rust_triple("x86_64-apple-ios").unwrap(), ApplePlatform::IOSSimulator);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-ios-macabi").unwrap(), ApplePlatform::MacCatalyst);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-tvos").unwrap(), ApplePlatform::TvOS);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-tvos-sim").unwrap(), ApplePlatform::TvOSSimulator);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-watchos").unwrap(), ApplePlatform::WatchOS);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-watchos-sim").unwrap(), ApplePlatform::WatchOSSimulator);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-visionos").unwrap(), ApplePlatform::XrOS);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-visionos-sim").unwrap(), ApplePlatform::XrOSSimulator);
        assert_eq!(ApplePlatform::from_rust_triple("aarch64-apple-driverkit").unwrap(), ApplePlatform::DriverKit);
        assert!(ApplePlatform::from_rust_triple("x86_64-unknown-linux-gnu").is_err());
    }

    #[test]
    fn from_str() {
        assert_eq!("macos".parse::<ApplePlatform>().unwrap(), ApplePlatform::MacOS);
        assert_eq!("visionos".parse::<ApplePlatform>().unwrap(), ApplePlatform::XrOS);
        assert_eq!("iphonesimulator".parse::<ApplePlatform>().unwrap(), ApplePlatform::IOSSimulator);
        let err = "nope".parse::<ApplePlatform>().unwrap_err();
        assert!(err.to_string().contains("nope"));
    }

    #[test]
    fn sdk_names() {
        assert_eq!(ApplePlatform::MacOS.sdk(),          Some("macosx"));
        assert_eq!(ApplePlatform::IOS.sdk(),            Some("iphoneos"));
        assert_eq!(ApplePlatform::IOSSimulator.sdk(),   Some("iphonesimulator"));
        assert_eq!(ApplePlatform::TvOS.sdk(),           Some("appletvos"));
        assert_eq!(ApplePlatform::TvOSSimulator.sdk(),  Some("appletvsimulator"));
        assert_eq!(ApplePlatform::WatchOS.sdk(),        Some("watchos"));
        assert_eq!(ApplePlatform::WatchOSSimulator.sdk(), Some("watchsimulator"));
        assert_eq!(ApplePlatform::MacCatalyst.sdk(),    Some("macosx"));
        assert_eq!(ApplePlatform::XrOS.sdk(),           Some("xros"));
        assert_eq!(ApplePlatform::XrOSSimulator.sdk(),  Some("xrsimulator"));
        assert_eq!(ApplePlatform::DriverKit.sdk(),      Some("driverkit"));
        assert_eq!(ApplePlatform::Unknown.sdk(),        None);
    }

    #[test]
    fn tapi_targets_match_llvm() {
        assert_eq!(ApplePlatform::MacCatalyst.metadata().tapi_target, "maccatalyst");
        assert_eq!(ApplePlatform::IOSSimulator.metadata().tapi_target, "ios-simulator");
        assert_eq!(ApplePlatform::XrOSSimulator.metadata().tapi_target, "xros-simulator");
    }

    #[test]
    fn simulator_helpers() {
        assert!(ApplePlatform::IOSSimulator.is_simulator());
        assert!(!ApplePlatform::IOS.is_simulator());
        assert!(ApplePlatform::IOS.is_device());
        assert!(!ApplePlatform::IOSSimulator.is_device());
        assert!(!ApplePlatform::Unknown.is_device());
    }

    #[test]
    fn device_method() {
        assert_eq!(ApplePlatform::IOSSimulator.device(), ApplePlatform::IOS);
        assert_eq!(ApplePlatform::XrOSSimulator.device(), ApplePlatform::XrOS);
        assert_eq!(ApplePlatform::IOS.device(), ApplePlatform::IOS);
        assert_eq!(ApplePlatform::MacOS.device(), ApplePlatform::MacOS);
    }

    #[test]
    fn simulator_method() {
        assert_eq!(ApplePlatform::IOS.simulator(),     Some(ApplePlatform::IOSSimulator));
        assert_eq!(ApplePlatform::TvOS.simulator(),    Some(ApplePlatform::TvOSSimulator));
        assert_eq!(ApplePlatform::WatchOS.simulator(), Some(ApplePlatform::WatchOSSimulator));
        assert_eq!(ApplePlatform::XrOS.simulator(),    Some(ApplePlatform::XrOSSimulator));
        // Already a simulator — returns self
        assert_eq!(ApplePlatform::IOSSimulator.simulator(), Some(ApplePlatform::IOSSimulator));
        // No simulator counterpart
        assert_eq!(ApplePlatform::MacOS.simulator(),     None);
        assert_eq!(ApplePlatform::DriverKit.simulator(), None);
        assert_eq!(ApplePlatform::BridgeOS.simulator(),  None);
    }

    #[test]
    fn ordering() {
        assert!(ApplePlatform::MacOS < ApplePlatform::IOS);
        assert!(ApplePlatform::IOS   < ApplePlatform::IOSSimulator);
        let mut platforms = vec![ApplePlatform::XrOS, ApplePlatform::MacOS, ApplePlatform::IOS];
        platforms.sort();
        assert_eq!(platforms, vec![ApplePlatform::MacOS, ApplePlatform::IOS, ApplePlatform::XrOS]);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", ApplePlatform::MacOS), "macOS");
        assert_eq!(format!("{}", ApplePlatform::XrOS),  "visionOS");
    }

    #[test]
    fn marketing_names() {
        assert_eq!(ApplePlatform::XrOS.metadata().marketing,          "visionOS");
        assert_eq!(ApplePlatform::XrOSSimulator.metadata().marketing,  "visionOS Simulator");
    }

    #[test]
    fn from_into_platform() {
        let p = Platform::from(ApplePlatform::MacOS);
        assert_eq!(p.id, 1);
        let back = ApplePlatform::try_from(p).unwrap();
        assert_eq!(back, ApplePlatform::MacOS);
    }
}
