import Foundation
#if canImport(QuickLookUI)
import QuickLookUI
@_cdecl("quicklookui_swift_avail")
public func quicklookui_swift_availFn() -> Bool { true }
#else
@_cdecl("quicklookui_swift_avail")
public func quicklookui_swift_availFn() -> Bool { false }
#endif
