import Foundation
#if canImport(WiFiAware)
import WiFiAware
@_cdecl("wifiaware_swift_avail")
public func wifiaware_swift_availFn() -> Bool { true }
#else
@_cdecl("wifiaware_swift_avail")
public func wifiaware_swift_availFn() -> Bool { false }
#endif
