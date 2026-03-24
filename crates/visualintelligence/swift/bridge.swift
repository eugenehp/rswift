import Foundation
#if canImport(VisualIntelligence)
import VisualIntelligence
@_cdecl("visualintelligence_swift_avail")
public func visualintelligence_swift_availFn() -> Bool { true }
#else
@_cdecl("visualintelligence_swift_avail")
public func visualintelligence_swift_availFn() -> Bool { false }
#endif
