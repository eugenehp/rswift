import Foundation
#if canImport(RelevanceKit)
import RelevanceKit
@_cdecl("relevancekit_swift_avail")
public func relevancekit_swift_availFn() -> Bool { true }
#else
@_cdecl("relevancekit_swift_avail")
public func relevancekit_swift_availFn() -> Bool { false }
#endif
