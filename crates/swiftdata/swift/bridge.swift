import Foundation
#if canImport(SwiftData)
import SwiftData

@_cdecl("swiftdata_swift_available")
public func swiftdataSwiftAvailFn() -> Bool { true }

// ModelContainer
@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_container_create")
public func swiftdataContainerCreate(
    _ schemasJson: UnsafePointer<UInt8>, _ schemasLen: Int,
    _ inMemory: Bool,
    _ url: UnsafePointer<UInt8>, _ urlLen: Int
) -> UnsafeMutableRawPointer? {
    do {
        var config = ModelConfiguration(isStoredInMemoryOnly: inMemory)
        if urlLen > 0 {
            let urlStr = String(bytes: UnsafeBufferPointer(start: url, count: urlLen), encoding: .utf8) ?? ""
            // ModelConfiguration path init: first positional arg is the String name/path
            config = ModelConfiguration(urlStr, schema: Schema([]), isStoredInMemoryOnly: inMemory)
        }
        // Note: without @Model types we can only create an empty container
        // In practice, the host app registers its model types
        let container = try ModelContainer(configurations: config)
        return Unmanaged.passRetained(container).toOpaque()
    } catch {
        return nil
    }
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_container_release")
public func swiftdataContainerRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<ModelContainer>.fromOpaque(ptr).release()
}

// ModelContext
@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_context_create")
public func swiftdataContextCreate(_ containerPtr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let container = Unmanaged<ModelContainer>.fromOpaque(containerPtr).takeUnretainedValue()
    let context = ModelContext(container)
    return Unmanaged.passRetained(context as AnyObject).toOpaque()
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_context_save")
public func swiftdataContextSave(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let context = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! ModelContext
    do {
        try context.save()
        return true
    } catch {
        return false
    }
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_context_has_changes")
public func swiftdataContextHasChanges(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let context = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! ModelContext
    return context.hasChanges
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_context_rollback")
public func swiftdataContextRollback(_ ptr: UnsafeMutableRawPointer) {
    let context = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! ModelContext
    context.rollback()
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_context_release")
public func swiftdataContextRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

// ModelConfiguration info
@available(macOS 14.0, iOS 17.0, *)
@_cdecl("swiftdata_config_in_memory")
public func swiftdataConfigInMemory(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    // Return a simple JSON config
    let result = "{\"in_memory\":true}"
    let data = Array(result.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

#else
@_cdecl("swiftdata_swift_available")
public func swiftdataSwiftAvailFn() -> Bool { false }
#endif
