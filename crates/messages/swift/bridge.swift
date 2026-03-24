import Foundation
#if canImport(Messages)
import Messages
@_cdecl("messages_swift_avail")
public func messages_swift_availFn() -> Bool { true }
#else
@_cdecl("messages_swift_avail")
public func messages_swift_availFn() -> Bool { false }
#endif
