import Foundation
#if canImport(ProximityReader)
import ProximityReader
@_cdecl("proximityreader_swift_avail")
public func proximityreader_swift_availFn() -> Bool { true }
#else
@_cdecl("proximityreader_swift_avail")
public func proximityreader_swift_availFn() -> Bool { false }
#endif
