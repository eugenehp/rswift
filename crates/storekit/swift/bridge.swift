import Foundation
import StoreKit

// ═══════════════════════════════════════════════════════════════════════════
// StoreKit — in-app purchases
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("storekit_available")
public func storekitAvailable() -> Bool { true }

@_cdecl("storekit_can_make_payments")
public func storekitCanMakePayments() -> Bool {
    SKPaymentQueue.canMakePayments()
}

@_cdecl("storekit_app_store_receipt_url")
public func storekitAppStoreReceiptUrl(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let url = Bundle.main.appStoreReceiptURL else { return -1 }
    let s = url.path
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

// StoreKit 2 (async) — product listing
@available(macOS 12.0, iOS 15.0, tvOS 15.0, watchOS 8.0, *)
@_cdecl("storekit2_fetch_products")
public func storekit2FetchProducts(
    _ idsPtr: UnsafePointer<UInt8>, _ idsLen: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let idsStr = String(bytes: UnsafeBufferPointer(start: idsPtr, count: idsLen), encoding: .utf8) ?? ""
    let ids = Set(idsStr.split(separator: ",").map(String.init))

    Task {
        do {
            let products = try await Product.products(for: ids)
            var lines: [String] = []
            for p in products {
                lines.append("{\"id\":\"\(p.id)\",\"name\":\"\(esc(p.displayName))\",\"price\":\"\(p.displayPrice)\",\"type\":\"\(p.type)\"}")
            }
            let result = lines.joined(separator: "\n")
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, ud)
        }
    }
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
}
