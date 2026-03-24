import Foundation
#if canImport(EnergyKit)
import EnergyKit
@_cdecl("energykit_swift_avail")
public func energykit_swift_availFn() -> Bool { true }
#else
@_cdecl("energykit_swift_avail")
public func energykit_swift_availFn() -> Bool { false }
#endif
