import Foundation
#if canImport(GameSave)
import GameSave
@_cdecl("gamesave_swift_avail")
public func gamesave_swift_availFn() -> Bool { true }
#else
@_cdecl("gamesave_swift_avail")
public func gamesave_swift_availFn() -> Bool { false }
#endif
