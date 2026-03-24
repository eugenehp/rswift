import Foundation
#if canImport(iTunesLibrary)
import iTunesLibrary
@_cdecl("ituneslibrary_swift_avail")
public func ituneslibrary_swift_availFn() -> Bool { true }
#else
@_cdecl("ituneslibrary_swift_avail")
public func ituneslibrary_swift_availFn() -> Bool { false }
#endif
