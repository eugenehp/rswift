import Foundation
#if canImport(AutomatedDeviceEnrollment)
import AutomatedDeviceEnrollment
@_cdecl("automateddeviceenrollment_swift_avail")
public func automateddeviceenrollment_swift_availFn() -> Bool { true }
#else
@_cdecl("automateddeviceenrollment_swift_avail")
public func automateddeviceenrollment_swift_availFn() -> Bool { false }
#endif
