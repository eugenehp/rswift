import Foundation
#if canImport(CoreTransferable)
import CoreTransferable
@_cdecl("coretransferable_swift_avail")
public func coretransferable_swift_availFn() -> Bool { true }
#else
@_cdecl("coretransferable_swift_avail")
public func coretransferable_swift_availFn() -> Bool { false }
#endif
