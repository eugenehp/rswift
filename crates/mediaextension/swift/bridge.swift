import Foundation
#if canImport(MediaExtension)
import MediaExtension
@_cdecl("mediaextension_swift_avail")
public func mediaextension_swift_availFn() -> Bool { true }
#else
@_cdecl("mediaextension_swift_avail")
public func mediaextension_swift_availFn() -> Bool { false }
#endif
