import Foundation
#if canImport(BrowserEngineCore)
import BrowserEngineCore
@_cdecl("browserenginecore_swift_avail")
public func browserenginecore_swift_availFn() -> Bool { true }
#else
@_cdecl("browserenginecore_swift_avail")
public func browserenginecore_swift_availFn() -> Bool { false }
#endif
