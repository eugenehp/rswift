import Foundation
#if canImport(JournalingSuggestions)
import JournalingSuggestions
@_cdecl("journalingsuggestions_swift_avail")
public func journalingsuggestions_swift_availFn() -> Bool { true }
#else
@_cdecl("journalingsuggestions_swift_avail")
public func journalingsuggestions_swift_availFn() -> Bool { false }
#endif
