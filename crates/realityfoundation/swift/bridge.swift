import Foundation
#if canImport(RealityKit)
import RealityKit
@_cdecl("realityfoundation_swift_avail")
public func realityfoundationSwiftAvailFn() -> Bool { true }
#else
@_cdecl("realityfoundation_swift_avail")
public func realityfoundationSwiftAvailFn() -> Bool { false }
#endif
