import Foundation
import Security

// ═══════════════════════════════════════════════════════════════════════════
// Security — Keychain access
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("security_available")
public func securityAvailable() -> Bool { true }

private func makeString(_ ptr: UnsafePointer<UInt8>, _ len: Int) -> String {
    String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8)!
}

private func writeStr(_ s: String, _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

// ── Keychain CRUD ───────────────────────────────────────────────────────────

@_cdecl("security_keychain_set")
public func securityKeychainSet(_ servicePtr: UnsafePointer<UInt8>, _ serviceLen: Int,
                                 _ accountPtr: UnsafePointer<UInt8>, _ accountLen: Int,
                                 _ dataPtr: UnsafePointer<UInt8>, _ dataLen: Int) -> Int32 {
    let service = makeString(servicePtr, serviceLen)
    let account = makeString(accountPtr, accountLen)
    let data = Data(bytes: dataPtr, count: dataLen)

    // Delete existing first
    let deleteQuery: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: account,
    ]
    SecItemDelete(deleteQuery as CFDictionary)

    let addQuery: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: account,
        kSecValueData as String: data,
    ]
    return SecItemAdd(addQuery as CFDictionary, nil)
}

@_cdecl("security_keychain_get")
public func securityKeychainGet(_ servicePtr: UnsafePointer<UInt8>, _ serviceLen: Int,
                                 _ accountPtr: UnsafePointer<UInt8>, _ accountLen: Int,
                                 _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let service = makeString(servicePtr, serviceLen)
    let account = makeString(accountPtr, accountLen)

    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: account,
        kSecReturnData as String: true,
        kSecMatchLimit as String: kSecMatchLimitOne,
    ]
    var result: AnyObject?
    let status = SecItemCopyMatching(query as CFDictionary, &result)
    guard status == errSecSuccess, let data = result as? Data else { return -1 }
    let len = min(data.count, bufLen)
    data.withUnsafeBytes { ptr in
        let src = ptr.bindMemory(to: UInt8.self)
        for i in 0..<len { buf[i] = src[i] }
    }
    return len
}

@_cdecl("security_keychain_delete")
public func securityKeychainDelete(_ servicePtr: UnsafePointer<UInt8>, _ serviceLen: Int,
                                    _ accountPtr: UnsafePointer<UInt8>, _ accountLen: Int) -> Int32 {
    let service = makeString(servicePtr, serviceLen)
    let account = makeString(accountPtr, accountLen)

    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: account,
    ]
    return SecItemDelete(query as CFDictionary)
}

@_cdecl("security_keychain_exists")
public func securityKeychainExists(_ servicePtr: UnsafePointer<UInt8>, _ serviceLen: Int,
                                    _ accountPtr: UnsafePointer<UInt8>, _ accountLen: Int) -> Bool {
    let service = makeString(servicePtr, serviceLen)
    let account = makeString(accountPtr, accountLen)

    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrService as String: service,
        kSecAttrAccount as String: account,
        kSecReturnData as String: false,
    ]
    return SecItemCopyMatching(query as CFDictionary, nil) == errSecSuccess
}

// ── Random bytes ────────────────────────────────────────────────────────────

@_cdecl("security_random_bytes")
public func securityRandomBytes(_ buf: UnsafeMutablePointer<UInt8>, _ count: Int) -> Int32 {
    SecRandomCopyBytes(kSecRandomDefault, count, buf)
}
