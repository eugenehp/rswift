import Foundation
#if canImport(LiveCommunicationKit)
import LiveCommunicationKit
@_cdecl("livecommunicationkit_swift_avail")
public func livecommunicationkit_swift_availFn() -> Bool { true }
#else
@_cdecl("livecommunicationkit_swift_avail")
public func livecommunicationkit_swift_availFn() -> Bool { false }
#endif
