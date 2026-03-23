import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// Foundation — core utilities
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("foundation_available")
public func foundationAvailable() -> Bool { true }

// ── UserDefaults ────────────────────────────────────────────────────────────

@_cdecl("foundation_userdefaults_set_string")
public func foundationUserDefaultsSetString(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int,
                                             _ valPtr: UnsafePointer<UInt8>, _ valLen: Int) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    let val = String(bytes: UnsafeBufferPointer(start: valPtr, count: valLen), encoding: .utf8)!
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("foundation_userdefaults_get_string")
public func foundationUserDefaultsGetString(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int,
                                             _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    guard let val = UserDefaults.standard.string(forKey: key) else { return -1 }
    let data = Array(val.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

@_cdecl("foundation_userdefaults_set_int")
public func foundationUserDefaultsSetInt(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ val: Int64) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("foundation_userdefaults_get_int")
public func foundationUserDefaultsGetInt(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ out: UnsafeMutablePointer<Int64>) -> Bool {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    guard UserDefaults.standard.object(forKey: key) != nil else { return false }
    out.pointee = Int64(UserDefaults.standard.integer(forKey: key))
    return true
}

@_cdecl("foundation_userdefaults_set_double")
public func foundationUserDefaultsSetDouble(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ val: Double) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("foundation_userdefaults_get_double")
public func foundationUserDefaultsGetDouble(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ out: UnsafeMutablePointer<Double>) -> Bool {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    guard UserDefaults.standard.object(forKey: key) != nil else { return false }
    out.pointee = UserDefaults.standard.double(forKey: key)
    return true
}

@_cdecl("foundation_userdefaults_set_bool")
public func foundationUserDefaultsSetBool(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ val: Bool) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("foundation_userdefaults_get_bool")
public func foundationUserDefaultsGetBool(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ out: UnsafeMutablePointer<Bool>) -> Bool {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    guard UserDefaults.standard.object(forKey: key) != nil else { return false }
    out.pointee = UserDefaults.standard.bool(forKey: key)
    return true
}

@_cdecl("foundation_userdefaults_remove")
public func foundationUserDefaultsRemove(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8)!
    UserDefaults.standard.removeObject(forKey: key)
}

@_cdecl("foundation_userdefaults_synchronize")
public func foundationUserDefaultsSynchronize() -> Bool {
    UserDefaults.standard.synchronize()
}

// ── FileManager ─────────────────────────────────────────────────────────────

private func makeString(_ ptr: UnsafePointer<UInt8>, _ len: Int) -> String {
    String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8)!
}

private func writeString(_ s: String, _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

@_cdecl("foundation_filemanager_file_exists")
public func foundationFileManagerFileExists(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> Bool {
    FileManager.default.fileExists(atPath: makeString(pathPtr, pathLen))
}

@_cdecl("foundation_filemanager_is_directory")
public func foundationFileManagerIsDirectory(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> Bool {
    var isDir: ObjCBool = false
    let exists = FileManager.default.fileExists(atPath: makeString(pathPtr, pathLen), isDirectory: &isDir)
    return exists && isDir.boolValue
}

@_cdecl("foundation_filemanager_create_directory")
public func foundationFileManagerCreateDirectory(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> Bool {
    do {
        try FileManager.default.createDirectory(atPath: makeString(pathPtr, pathLen),
                                                 withIntermediateDirectories: true)
        return true
    } catch { return false }
}

@_cdecl("foundation_filemanager_remove_item")
public func foundationFileManagerRemoveItem(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> Bool {
    do {
        try FileManager.default.removeItem(atPath: makeString(pathPtr, pathLen))
        return true
    } catch { return false }
}

@_cdecl("foundation_filemanager_copy_item")
public func foundationFileManagerCopyItem(_ srcPtr: UnsafePointer<UInt8>, _ srcLen: Int,
                                           _ dstPtr: UnsafePointer<UInt8>, _ dstLen: Int) -> Bool {
    do {
        try FileManager.default.copyItem(atPath: makeString(srcPtr, srcLen),
                                          toPath: makeString(dstPtr, dstLen))
        return true
    } catch { return false }
}

@_cdecl("foundation_filemanager_move_item")
public func foundationFileManagerMoveItem(_ srcPtr: UnsafePointer<UInt8>, _ srcLen: Int,
                                           _ dstPtr: UnsafePointer<UInt8>, _ dstLen: Int) -> Bool {
    do {
        try FileManager.default.moveItem(atPath: makeString(srcPtr, srcLen),
                                          toPath: makeString(dstPtr, dstLen))
        return true
    } catch { return false }
}

@_cdecl("foundation_filemanager_contents_of_directory")
public func foundationFileManagerContentsOfDirectory(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int,
                                                      _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    do {
        let items = try FileManager.default.contentsOfDirectory(atPath: makeString(pathPtr, pathLen))
        let joined = items.joined(separator: "\n")
        return writeString(joined, buf, bufLen)
    } catch { return -1 }
}

@_cdecl("foundation_filemanager_home_directory")
public func foundationFileManagerHomeDirectory(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(FileManager.default.homeDirectoryForCurrentUser.path, buf, bufLen)
}

@_cdecl("foundation_filemanager_temp_directory")
public func foundationFileManagerTempDirectory(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(FileManager.default.temporaryDirectory.path, buf, bufLen)
}

@_cdecl("foundation_filemanager_file_size")
public func foundationFileManagerFileSize(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) -> Int64 {
    do {
        let attrs = try FileManager.default.attributesOfItem(atPath: makeString(pathPtr, pathLen))
        return Int64(attrs[.size] as? UInt64 ?? 0)
    } catch { return -1 }
}

// ── ProcessInfo ─────────────────────────────────────────────────────────────

@_cdecl("foundation_processinfo_hostname")
public func foundationProcessInfoHostname(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(ProcessInfo.processInfo.hostName, buf, bufLen)
}

@_cdecl("foundation_processinfo_os_version")
public func foundationProcessInfoOsVersion(_ major: UnsafeMutablePointer<Int>, _ minor: UnsafeMutablePointer<Int>, _ patch: UnsafeMutablePointer<Int>) {
    let v = ProcessInfo.processInfo.operatingSystemVersion
    major.pointee = v.majorVersion
    minor.pointee = v.minorVersion
    patch.pointee = v.patchVersion
}

@_cdecl("foundation_processinfo_processor_count")
public func foundationProcessInfoProcessorCount() -> Int {
    ProcessInfo.processInfo.processorCount
}

@_cdecl("foundation_processinfo_active_processor_count")
public func foundationProcessInfoActiveProcessorCount() -> Int {
    ProcessInfo.processInfo.activeProcessorCount
}

@_cdecl("foundation_processinfo_physical_memory")
public func foundationProcessInfoPhysicalMemory() -> UInt64 {
    ProcessInfo.processInfo.physicalMemory
}

@_cdecl("foundation_processinfo_system_uptime")
public func foundationProcessInfoSystemUptime() -> Double {
    ProcessInfo.processInfo.systemUptime
}

@_cdecl("foundation_processinfo_process_name")
public func foundationProcessInfoProcessName(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(ProcessInfo.processInfo.processName, buf, bufLen)
}

@_cdecl("foundation_processinfo_thermal_state")
public func foundationProcessInfoThermalState() -> Int {
    ProcessInfo.processInfo.thermalState.rawValue
}

@_cdecl("foundation_processinfo_is_low_power_mode")
public func foundationProcessInfoIsLowPowerMode() -> Bool {
    ProcessInfo.processInfo.isLowPowerModeEnabled
}

// ── Bundle ──────────────────────────────────────────────────────────────────

@_cdecl("foundation_bundle_main_path")
public func foundationBundleMainPath(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(Bundle.main.bundlePath, buf, bufLen)
}

@_cdecl("foundation_bundle_resource_path")
public func foundationBundleResourcePath(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let p = Bundle.main.resourcePath else { return -1 }
    return writeString(p, buf, bufLen)
}

@_cdecl("foundation_bundle_identifier")
public func foundationBundleIdentifier(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let id = Bundle.main.bundleIdentifier else { return -1 }
    return writeString(id, buf, bufLen)
}

@_cdecl("foundation_bundle_info_string")
public func foundationBundleInfoString(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int,
                                        _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let key = makeString(keyPtr, keyLen)
    guard let val = Bundle.main.infoDictionary?[key] as? String else { return -1 }
    return writeString(val, buf, bufLen)
}

// ── UUID ────────────────────────────────────────────────────────────────────

@_cdecl("foundation_uuid_generate")
public func foundationUuidGenerate(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(UUID().uuidString, buf, bufLen)
}

// ── Locale ──────────────────────────────────────────────────────────────────

@_cdecl("foundation_locale_current_identifier")
public func foundationLocaleCurrentIdentifier(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(Locale.current.identifier, buf, bufLen)
}

@_cdecl("foundation_locale_preferred_languages")
public func foundationLocalePreferredLanguages(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(Locale.preferredLanguages.joined(separator: ","), buf, bufLen)
}

@_cdecl("foundation_locale_current_language")
public func foundationLocaleCurrentLanguage(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let lang = Locale.current.language.languageCode?.identifier ?? "en"
    return writeString(lang, buf, bufLen)
}

@_cdecl("foundation_locale_current_region")
public func foundationLocaleCurrentRegion(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let region = Locale.current.region?.identifier ?? ""
    return writeString(region, buf, bufLen)
}

// ── Date / TimeZone ─────────────────────────────────────────────────────────

@_cdecl("foundation_date_now")
public func foundationDateNow() -> Double {
    Date().timeIntervalSince1970
}

@_cdecl("foundation_date_format")
public func foundationDateFormat(_ timestamp: Double,
                                  _ fmtPtr: UnsafePointer<UInt8>, _ fmtLen: Int,
                                  _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let date = Date(timeIntervalSince1970: timestamp)
    let fmt = makeString(fmtPtr, fmtLen)
    let formatter = DateFormatter()
    formatter.dateFormat = fmt
    return writeString(formatter.string(from: date), buf, bufLen)
}

@_cdecl("foundation_timezone_current")
public func foundationTimezoneCurrent(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    writeString(TimeZone.current.identifier, buf, bufLen)
}

@_cdecl("foundation_timezone_offset")
public func foundationTimezoneOffset() -> Int {
    TimeZone.current.secondsFromGMT()
}

// ── JSON ────────────────────────────────────────────────────────────────────

@_cdecl("foundation_json_valid")
public func foundationJsonValid(_ dataPtr: UnsafePointer<UInt8>, _ dataLen: Int) -> Bool {
    let data = Data(bytes: dataPtr, count: dataLen)
    return (try? JSONSerialization.jsonObject(with: data)) != nil
}

// ── URL / paths ─────────────────────────────────────────────────────────────

@_cdecl("foundation_url_search_path")
public func foundationUrlSearchPath(_ directory: Int, _ domain: Int,
                                     _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let dir = FileManager.SearchPathDirectory(rawValue: UInt(directory)) else { return -1 }
    let domainMask = FileManager.SearchPathDomainMask(rawValue: UInt(domain))
    let paths = FileManager.default.urls(for: dir, in: domainMask)
    guard let first = paths.first else { return -1 }
    return writeString(first.path, buf, bufLen)
}
