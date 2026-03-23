// https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/MachO.def#L123-L138

// PLATFORM(platform, id, name, build_name, target, tapi_target, marketing)
// PLATFORM(UNKNOWN, 0, unknown, unknown, unknown, unknown, unknown)
// PLATFORM(MACOS, 1, macos, macos, macos, macos, macOS)
// PLATFORM(IOS, 2, ios, ios, ios, ios, iOS)
// PLATFORM(TVOS, 3, tvos, tvos, tvos, tvos, tvOS)
// PLATFORM(WATCHOS, 4, watchos, watchos, watchos, watchos, watchOS)
// PLATFORM(BRIDGEOS, 5, bridgeos, bridgeos, bridgeos, bridgeos, bridgeOS)
// PLATFORM(MACCATALYST, 6, macCatalyst, macCatalyst, ios-macabi, maccatalyst, macCatalyst)
// PLATFORM(IOSSIMULATOR, 7, iossimulator, iossimulator, ios-simulator, ios-simulator, iOS Simulator)
// PLATFORM(TVOSSIMULATOR, 8, tvossimulator, tvossimulator, tvos-simulator, tvos-simulator, tvOS Simulator)
// PLATFORM(WATCHOSSIMULATOR, 9, watchossimulator, watchossimulator, watchos-simulator, watchos-simulator, watchOS Simulator)
// PLATFORM(DRIVERKIT, 10, driverkit, driverkit, driverkit, driverkit, DriverKit)
// PLATFORM(XROS, 11, xros, xros, xros, xros, xrOS)
// PLATFORM(XROS_SIMULATOR, 12, xrsimulator, xrsimulator, xrsimulator, xros-simulator, xrOS Simulator)

use std::fmt;

#[derive(Clone, Copy)]
pub struct Platform<'a> {
    platform: &'a str,
    id: usize,
    name: &'a str,
    build_name: &'a str,
    target: &'a str,
    tapi_target: &'a str,
    marketing: &'a str,
}

impl Default for Platform<'static> {
    fn default() -> Self {
        Self {
            platform: "unknown",
            id: 0,
            name: "unknown",
            build_name: "unknown",
            target: "unknown",
            tapi_target: "unknown",
            marketing: "unknown",
        }
    }
}

impl fmt::Debug for Platform<'static> {
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

#[derive(Clone, Copy)]
pub enum ApplePlatform {
    UNKNOWN,
    MACOS,
    IOS,
    TVOS,
    WATCHOS,
    BRIDGEOS,
    MACCATALYST,
    IOSSIMULATOR,
    TVOSSIMULATOR,
    WATCHOSSIMULATOR,
    DRIVERKIT,
    XROS,
    #[allow(non_camel_case_types)]
    XROS_SIMULATOR,
}

impl Default for ApplePlatform {
    fn default() -> Self {
        ApplePlatform::UNKNOWN
    }
}

impl fmt::Debug for ApplePlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let platform = Platform::try_from(*self).unwrap();
        write!(f, "{}", platform.platform)
    }
}

impl ApplePlatform {
    pub fn into_iter() -> std::array::IntoIter<ApplePlatform, 13> {
        [
            ApplePlatform::UNKNOWN,
            ApplePlatform::MACOS,
            ApplePlatform::IOS,
            ApplePlatform::TVOS,
            ApplePlatform::WATCHOS,
            ApplePlatform::BRIDGEOS,
            ApplePlatform::MACCATALYST,
            ApplePlatform::IOSSIMULATOR,
            ApplePlatform::TVOSSIMULATOR,
            ApplePlatform::WATCHOSSIMULATOR,
            ApplePlatform::DRIVERKIT,
            ApplePlatform::XROS,
            ApplePlatform::XROS_SIMULATOR,
        ]
        .into_iter()
    }
}

impl TryFrom<ApplePlatform> for Platform<'_> {
    type Error = ();

    fn try_from(value: ApplePlatform) -> Result<Self, Self::Error> {
        Ok(match value {
            ApplePlatform::UNKNOWN => Platform {
                platform: "UNKNOWN",
                id: 0,
                name: "unknown",
                build_name: "unknown",
                target: "unknown",
                tapi_target: "unknown",
                marketing: "unknown",
            },
            ApplePlatform::MACOS => Platform {
                platform: "MACOS",
                id: 1,
                name: "macos",
                build_name: "macos",
                target: "macos",
                tapi_target: "macos",
                marketing: "macOS",
            },
            ApplePlatform::IOS => Platform {
                platform: "IOS",
                id: 2,
                name: "ios",
                build_name: "ios",
                target: "ios",
                tapi_target: "ios",
                marketing: "iOS",
            },
            ApplePlatform::TVOS => Platform {
                platform: "TVOS",
                id: 3,
                name: "tvos",
                build_name: "tvos",
                target: "tvos",
                tapi_target: "tvos",
                marketing: "tvOS",
            },
            ApplePlatform::WATCHOS => Platform {
                platform: "WATCHOS",
                id: 4,
                name: "watchos",
                build_name: "watchos",
                target: "watchos",
                tapi_target: "watchos",
                marketing: "watchOS",
            },
            ApplePlatform::BRIDGEOS => Platform {
                platform: "BRIDGEOS",
                id: 5,
                name: "bridgeos",
                build_name: "bridgeos",
                target: "bridgeos",
                tapi_target: "bridgeos",
                marketing: "bridgeOS",
            },
            ApplePlatform::MACCATALYST => Platform {
                platform: "MACCATALYST",
                id: 6,
                name: "macCatalyst",
                build_name: "macCatalyst",
                target: "ios-macabi",
                tapi_target: "ios-macabi",
                marketing: "macCatalyst",
            },
            ApplePlatform::IOSSIMULATOR => Platform {
                platform: "IOSSIMULATOR",
                id: 7,
                name: "iossimulator",
                build_name: "iossimulator",
                target: "ios-simulator",
                tapi_target: "ios-simulator",
                marketing: "iOS Simulator",
            },
            ApplePlatform::TVOSSIMULATOR => Platform {
                platform: "TVOSSIMULATOR",
                id: 8,
                name: "tvossimulator",
                build_name: "tvossimulator",
                target: "tvos-simulator",
                tapi_target: "tvos-simulator",
                marketing: "tvOS Simulator",
            },
            ApplePlatform::WATCHOSSIMULATOR => Platform {
                platform: "WATCHOSSIMULATOR",
                id: 9,
                name: "watchossimulator",
                build_name: "watchossimulator",
                target: "watchos-simulator",
                tapi_target: "watchos-simulator",
                marketing: "watchOS Simulator",
            },
            ApplePlatform::DRIVERKIT => Platform {
                platform: "DRIVERKIT",
                id: 10,
                name: "driverkit",
                build_name: "driverkit",
                target: "driverkit",
                tapi_target: "driverkit",
                marketing: "DriverKit",
            },
            ApplePlatform::XROS => Platform {
                platform: "XROS",
                id: 11,
                name: "xros",
                build_name: "xros",
                target: "xros",
                tapi_target: "xros",
                marketing: "xrOS",
            },
            ApplePlatform::XROS_SIMULATOR => Platform {
                platform: "XROS_SIMULATOR",
                id: 12,
                name: "xrsimulator",
                build_name: "xrsimulator",
                target: "xrsimulator",
                tapi_target: "xros-simulator",
                marketing: "Simulator",
            },
        })
    }
}

impl Into<ApplePlatform> for Platform<'static> {
    fn into(self) -> ApplePlatform {
        match self.platform {
            "UNKNOWN" => ApplePlatform::UNKNOWN,
            "MACOS" => ApplePlatform::MACOS,
            "IOS" => ApplePlatform::IOS,
            "TVOS" => ApplePlatform::TVOS,
            "WATCHOS" => ApplePlatform::WATCHOS,
            "BRIDGEOS" => ApplePlatform::BRIDGEOS,
            "MACCATALYST" => ApplePlatform::MACCATALYST,
            "IOSSIMULATOR" => ApplePlatform::IOSSIMULATOR,
            "TVOSSIMULATOR" => ApplePlatform::TVOSSIMULATOR,
            "WATCHOSSIMULATOR" => ApplePlatform::WATCHOSSIMULATOR,
            "DRIVERKIT" => ApplePlatform::DRIVERKIT,
            "XROS" => ApplePlatform::XROS,
            "XROS_SIMULATOR" => ApplePlatform::XROS_SIMULATOR,
            _ => ApplePlatform::UNKNOWN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_platforms() {
        let platforms = ApplePlatform::into_iter()
            .map(|platform| platform)
            .collect::<Vec<_>>();
        assert_eq!(
            format!("{:?}", platforms),
            "[UNKNOWN, MACOS, IOS, TVOS, WATCHOS, BRIDGEOS, MACCATALYST, IOSSIMULATOR, TVOSSIMULATOR, WATCHOSSIMULATOR, DRIVERKIT, XROS, XROS_SIMULATOR]"
        )
    }
}
