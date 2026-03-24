import Foundation
#if canImport(WirelessInsights)
import WirelessInsights
@_cdecl("wirelessinsights_swift_avail")
public func wirelessinsights_swift_availFn() -> Bool { true }
#else
@_cdecl("wirelessinsights_swift_avail")
public func wirelessinsights_swift_availFn() -> Bool { false }
#endif
