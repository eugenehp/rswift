import Foundation
#if canImport(PaperKit)
import PaperKit
@_cdecl("paperkit_swift_avail")
public func paperkit_swift_availFn() -> Bool { true }
#else
@_cdecl("paperkit_swift_avail")
public func paperkit_swift_availFn() -> Bool { false }
#endif
