import Foundation
#if canImport(GeoToolbox)
import GeoToolbox
@_cdecl("geotoolbox_swift_avail")
public func geotoolbox_swift_availFn() -> Bool { true }
#else
@_cdecl("geotoolbox_swift_avail")
public func geotoolbox_swift_availFn() -> Bool { false }
#endif
