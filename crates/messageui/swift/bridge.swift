import Foundation
#if canImport(MessageUI)
import MessageUI
@_cdecl("messageui_swift_avail")
public func messageui_swift_availFn() -> Bool { true }
#else
@_cdecl("messageui_swift_avail")
public func messageui_swift_availFn() -> Bool { false }
#endif
