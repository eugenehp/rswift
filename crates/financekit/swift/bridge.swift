import Foundation
#if canImport(FinanceKit) && (os(iOS) || os(visionOS))
import FinanceKit

@_cdecl("financekit_swift_available")
public func financekitSwiftAvailFn() -> Bool { true }

@available(macOS 15.0, iOS 17.0, *)
@_cdecl("financekit_authorization_status")
public func financekitAuthorizationStatus() -> Int {
    // 0=notDetermined, 1=authorized, 2=denied
    return 0 // Requires entitlement; return notDetermined by default
}

@available(macOS 15.0, iOS 17.0, *)
@_cdecl("financekit_request_authorization")
public func financekitRequestAuthorization(
    _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    Task {
        do {
            let status = try await FinanceStore.shared.requestAuthorization()
            cb(status == .authorized ? 1 : 2, ud)
        } catch {
            cb(2, ud) // denied
        }
    }
}

@available(macOS 15.0, iOS 17.0, *)
@_cdecl("financekit_fetch_accounts")
public func financekitFetchAccounts(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    Task {
        do {
            let query = AccountQuery()
            let accounts = try await FinanceStore.shared.accounts(query: query)
            var lines: [String] = []
            for account in accounts {
                switch account {
                case .asset(let a):
                    lines.append("{\"type\":\"asset\",\"name\":\"\(esc(a.displayName))\",\"id\":\"\(a.id)\"}")
                case .liability(let l):
                    lines.append("{\"type\":\"liability\",\"name\":\"\(esc(l.displayName))\",\"id\":\"\(l.id)\"}")
                @unknown default:
                    break
                }
            }
            let result = "[" + lines.joined(separator: ",") + "]"
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
}

#else
@_cdecl("financekit_swift_available")
public func financekitSwiftAvailFn() -> Bool { false }
#endif
