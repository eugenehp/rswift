import Foundation
#if canImport(Symbols)
import Symbols

@_cdecl("symbols_swift_available")
public func symbols_swift_availableImpl() -> Bool { true }


@available(macOS 14.0, iOS 17.0, *)
@_cdecl("symbols_effect_available")
public func symbolsEffectAvailable() -> Bool { true }


#else
@_cdecl("symbols_swift_available")
public func symbols_swift_availableImpl() -> Bool { false }
#endif
