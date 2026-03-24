import Foundation
#if canImport(DeviceDiscoveryExtension)
import DeviceDiscoveryExtension
@_cdecl("devicediscoveryextension_swift_avail")
public func devicediscoveryextension_swift_availFn() -> Bool { true }
#else
@_cdecl("devicediscoveryextension_swift_avail")
public func devicediscoveryextension_swift_availFn() -> Bool { false }
#endif
