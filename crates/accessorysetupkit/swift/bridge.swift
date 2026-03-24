import Foundation
#if canImport(AccessorySetupKit) && (os(iOS) || os(visionOS))
import AccessorySetupKit
@_cdecl("accessorysetupkit_swift_avail")
public func accessorysetupkit_swift_availFn() -> Bool { true }
#else
@_cdecl("accessorysetupkit_swift_avail")
public func accessorysetupkit_swift_availFn() -> Bool { false }
#endif
