//! Apple platform definitions from LLVM's MachO.def.
//!
//! Source: <https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def>

use std::fmt;

/// Metadata for a single Apple platform.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Platform<'a> {
    /// Enum-style name (e.g. `"MACOS"`).
    pub platform: &'a str,
    /// Mach-O platform ID.
    pub id: usize,
    /// Lowercase name (e.g. `"macos"`).
    pub name: &'a str,
    /// Build name (e.g. `"macos"`).
    pub build_name: &'a str,
    /// Clang target OS component (e.g. `"macos"`, `"ios-simulator"`).
    pub target: &'a str,
    /// TAPI target (e.g. `"macos"`, `"maccatalyst"`).
    pub tapi_target: &'a str,
    /// Marketing name (e.g. `"macOS"`, `"iOS Simulator"`).
    pub marketing: &'a str,
}

impl fmt::Debug for Platform<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Platform")
            .field("platform", &self.platform)
            .field("id", &self.id)
            .field("name", &self.name)
            .field("build_name", &self.build_name)
            .field("target", &self.target)
            .field("tapi_target", &self.tapi_target)
            .field("marketing", &self.marketing)
            .finish()
    }
}

impl fmt::Display for Platform<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (id={}, target={})", self.marketing, self.id, self.target)
    }
}

/// Apple platform enumeration.
///
/// Matches LLVM's `MachO::PlatformType` identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ApplePlatform {
    Unknown = 0,
    MacOS = 1,
    IOS = 2,
    TvOS = 3,
    WatchOS = 4,
    BridgeOS = 5,
    MacCatalyst = 6,
    IOSSimulator = 7,
    TvOSSimulator = 8,
    WatchOSSimulator = 9,
    DriverKit = 10,
    XrOS = 11,
    XrOSSimulator = 12,
}

impl Default for ApplePlatform {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for ApplePlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = Platform::from(*self);
        write!(f, "{}", p.marketing)
    }
}

impl ApplePlatform {
    /// All platform variants.
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

    /// Iterate over all platform variants.
    pub fn iter() -> impl Iterator<Item = ApplePlatform> {
        Self::ALL.into_iter()
    }

    /// Legacy: same as [`iter`](Self::iter).
    pub fn into_iter() -> std::array::IntoIter<ApplePlatform, 13> {
        Self::ALL.into_iter()
    }

    /// Create from a Mach-O platform ID.
    pub fn from_id(id: usize) -> Self {
        match id {
            1 => Self::MacOS,
            2 => Self::IOS,
            3 => Self::TvOS,
            4 => Self::WatchOS,
            5 => Self::BridgeOS,
            6 => Self::MacCatalyst,
            7 => Self::IOSSimulator,
            8 => Self::TvOSSimulator,
            9 => Self::WatchOSSimulator,
            10 => Self::DriverKit,
            11 => Self::XrOS,
            12 => Self::XrOSSimulator,
            _ => Self::Unknown,
        }
    }

    /// Whether this is a simulator platform.
    pub fn is_simulator(&self) -> bool {
        matches!(self, Self::IOSSimulator | Self::TvOSSimulator | Self::WatchOSSimulator | Self::XrOSSimulator)
    }

    /// Get the corresponding device platform for a simulator.
    pub fn device_platform(&self) -> Self {
        match self {
            Self::IOSSimulator => Self::IOS,
            Self::TvOSSimulator => Self::TvOS,
            Self::WatchOSSimulator => Self::WatchOS,
            Self::XrOSSimulator => Self::XrOS,
            other => *other,
        }
    }
}

impl std::str::FromStr for ApplePlatform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "macos" | "macosx" | "osx" => Ok(Self::MacOS),
            "ios" | "iphoneos" => Ok(Self::IOS),
            "tvos" | "appletvos" => Ok(Self::TvOS),
            "watchos" => Ok(Self::WatchOS),
            "bridgeos" => Ok(Self::BridgeOS),
            "maccatalyst" | "macabi" => Ok(Self::MacCatalyst),
            "iossimulator" | "iphonesimulator" => Ok(Self::IOSSimulator),
            "tvossimulator" | "appletvsimulator" => Ok(Self::TvOSSimulator),
            "watchossimulator" | "watchsimulator" => Ok(Self::WatchOSSimulator),
            "driverkit" => Ok(Self::DriverKit),
            "xros" | "visionos" => Ok(Self::XrOS),
            "xrossimulator" | "xrsimulator" | "visionossimulator" => Ok(Self::XrOSSimulator),
            "unknown" => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

// ── Platform data ───────────────────────────────────────────────────────────

// PLATFORM(platform, id, name, build_name, target, tapi_target, marketing)
const PLATFORMS: [Platform<'static>; 13] = [
    Platform { platform: "UNKNOWN",            id: 0,  name: "unknown",            build_name: "unknown",            target: "unknown",            tapi_target: "unknown",            marketing: "unknown" },
    Platform { platform: "MACOS",              id: 1,  name: "macos",              build_name: "macos",              target: "macos",              tapi_target: "macos",              marketing: "macOS" },
    Platform { platform: "IOS",                id: 2,  name: "ios",                build_name: "ios",                target: "ios",                tapi_target: "ios",                marketing: "iOS" },
    Platform { platform: "TVOS",               id: 3,  name: "tvos",               build_name: "tvos",               target: "tvos",               tapi_target: "tvos",               marketing: "tvOS" },
    Platform { platform: "WATCHOS",            id: 4,  name: "watchos",            build_name: "watchos",            target: "watchos",            tapi_target: "watchos",            marketing: "watchOS" },
    Platform { platform: "BRIDGEOS",           id: 5,  name: "bridgeos",           build_name: "bridgeos",           target: "bridgeos",           tapi_target: "bridgeos",           marketing: "bridgeOS" },
    Platform { platform: "MACCATALYST",        id: 6,  name: "macCatalyst",        build_name: "macCatalyst",        target: "ios-macabi",         tapi_target: "maccatalyst",        marketing: "macCatalyst" },
    Platform { platform: "IOSSIMULATOR",       id: 7,  name: "iossimulator",       build_name: "iossimulator",       target: "ios-simulator",      tapi_target: "ios-simulator",      marketing: "iOS Simulator" },
    Platform { platform: "TVOSSIMULATOR",      id: 8,  name: "tvossimulator",      build_name: "tvossimulator",      target: "tvos-simulator",     tapi_target: "tvos-simulator",     marketing: "tvOS Simulator" },
    Platform { platform: "WATCHOSSIMULATOR",   id: 9,  name: "watchossimulator",   build_name: "watchossimulator",   target: "watchos-simulator",  tapi_target: "watchos-simulator",  marketing: "watchOS Simulator" },
    Platform { platform: "DRIVERKIT",          id: 10, name: "driverkit",          build_name: "driverkit",          target: "driverkit",          tapi_target: "driverkit",          marketing: "DriverKit" },
    Platform { platform: "XROS",               id: 11, name: "xros",               build_name: "xros",               target: "xros",               tapi_target: "xros",               marketing: "visionOS" },
    Platform { platform: "XROS_SIMULATOR",     id: 12, name: "xrsimulator",        build_name: "xrsimulator",        target: "xros-simulator",     tapi_target: "xros-simulator",     marketing: "visionOS Simulator" },
];

impl From<ApplePlatform> for Platform<'static> {
    fn from(value: ApplePlatform) -> Self {
        PLATFORMS[value as usize]
    }
}

impl From<Platform<'static>> for ApplePlatform {
    fn from(p: Platform<'static>) -> Self {
        Self::from_id(p.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_platforms() {
        let names: Vec<&str> = ApplePlatform::iter()
            .map(|p| Platform::from(p).platform)
            .collect();
        assert_eq!(names, vec![
            "UNKNOWN", "MACOS", "IOS", "TVOS", "WATCHOS", "BRIDGEOS",
            "MACCATALYST", "IOSSIMULATOR", "TVOSSIMULATOR", "WATCHOSSIMULATOR",
            "DRIVERKIT", "XROS", "XROS_SIMULATOR"
        ]);
    }

    #[test]
    fn platform_ids() {
        assert_eq!(Platform::from(ApplePlatform::MacOS).id, 1);
        assert_eq!(Platform::from(ApplePlatform::IOS).id, 2);
        assert_eq!(Platform::from(ApplePlatform::XrOS).id, 11);
    }

    #[test]
    fn from_id() {
        assert_eq!(ApplePlatform::from_id(1), ApplePlatform::MacOS);
        assert_eq!(ApplePlatform::from_id(11), ApplePlatform::XrOS);
        assert_eq!(ApplePlatform::from_id(999), ApplePlatform::Unknown);
    }

    #[test]
    fn from_str() {
        assert_eq!("macos".parse::<ApplePlatform>(), Ok(ApplePlatform::MacOS));
        assert_eq!("visionos".parse::<ApplePlatform>(), Ok(ApplePlatform::XrOS));
        assert_eq!("iphonesimulator".parse::<ApplePlatform>(), Ok(ApplePlatform::IOSSimulator));
        assert!("nope".parse::<ApplePlatform>().is_err());
    }

    #[test]
    fn tapi_targets_match_llvm() {
        // Verify against LLVM's MachO.def
        assert_eq!(Platform::from(ApplePlatform::MacCatalyst).tapi_target, "maccatalyst");
        assert_eq!(Platform::from(ApplePlatform::IOSSimulator).tapi_target, "ios-simulator");
        assert_eq!(Platform::from(ApplePlatform::XrOSSimulator).tapi_target, "xros-simulator");
    }

    #[test]
    fn simulator_helpers() {
        assert!(ApplePlatform::IOSSimulator.is_simulator());
        assert!(!ApplePlatform::IOS.is_simulator());
        assert_eq!(ApplePlatform::IOSSimulator.device_platform(), ApplePlatform::IOS);
        assert_eq!(ApplePlatform::XrOSSimulator.device_platform(), ApplePlatform::XrOS);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", ApplePlatform::MacOS), "macOS");
        assert_eq!(format!("{}", ApplePlatform::XrOS), "visionOS");
    }

    #[test]
    fn marketing_names() {
        // xrOS is now marketed as "visionOS"
        assert_eq!(Platform::from(ApplePlatform::XrOS).marketing, "visionOS");
        assert_eq!(Platform::from(ApplePlatform::XrOSSimulator).marketing, "visionOS Simulator");
    }
}
