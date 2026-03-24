import Foundation
#if canImport(BrowserEngineKit)
import BrowserEngineKit
@_cdecl("browserenginekit_swift_avail")
public func browserenginekit_swift_availFn() -> Bool { true }
#else
@_cdecl("browserenginekit_swift_avail")
public func browserenginekit_swift_availFn() -> Bool { false }
#endif
