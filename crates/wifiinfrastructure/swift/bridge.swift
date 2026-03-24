import Foundation
#if canImport(WiFiInfrastructure)
import WiFiInfrastructure
@_cdecl("wifiinfrastructure_swift_avail")
public func wifiinfrastructure_swift_availFn() -> Bool { true }
#else
@_cdecl("wifiinfrastructure_swift_avail")
public func wifiinfrastructure_swift_availFn() -> Bool { false }
#endif
