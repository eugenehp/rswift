import Foundation
#if canImport(AppMigrationKit)
import AppMigrationKit
@_cdecl("appmigrationkit_swift_avail")
public func appmigrationkit_swift_availFn() -> Bool { true }
#else
@_cdecl("appmigrationkit_swift_avail")
public func appmigrationkit_swift_availFn() -> Bool { false }
#endif
