import Foundation
#if canImport(CoreNFC)
import CoreNFC

@_cdecl("corenfc_swift_avail")
public func corenfcSwiftAvailFn() -> Bool { true }

@_cdecl("corenfc_reading_available")
public func corenfcReadingAvailable() -> Bool {
    if #available(iOS 11.0, *) {
        return NFCNDEFReaderSession.readingAvailable
    }
    return false
}

@available(iOS 13.0, *)
@_cdecl("corenfc_tag_reader_available")
public func corenfcTagReaderAvailable() -> Bool {
    return NFCTagReaderSession.readingAvailable
}

// NDEF message writing helper
@available(iOS 13.0, *)
@_cdecl("corenfc_create_text_payload")
public func corenfcCreateTextPayload(
    _ textPtr: UnsafePointer<UInt8>, _ textLen: Int,
    _ locale: UnsafePointer<UInt8>, _ localeLen: Int,
    _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int
) -> Int {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    let lang = String(bytes: UnsafeBufferPointer(start: locale, count: localeLen), encoding: .utf8) ?? "en"
    guard let payload = NFCNDEFPayload.wellKnownTypeTextPayload(string: text, locale: Locale(identifier: lang)) else {
        return -1
    }
    let data = payload.payload
    let len = min(data.count, bufLen)
    data.withUnsafeBytes { ptr in
        let src = ptr.bindMemory(to: UInt8.self)
        for i in 0..<len { buf[i] = src[i] }
    }
    return len
}

// NDEF URI payload
@available(iOS 13.0, *)
@_cdecl("corenfc_create_uri_payload")
public func corenfcCreateUriPayload(
    _ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int,
    _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int
) -> Int {
    let urlStr = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    guard let url = URL(string: urlStr) else { return -1 }
    guard let payload = NFCNDEFPayload.wellKnownTypeURIPayload(url: url) else { return -1 }
    let data = payload.payload
    let len = min(data.count, bufLen)
    data.withUnsafeBytes { ptr in
        let src = ptr.bindMemory(to: UInt8.self)
        for i in 0..<len { buf[i] = src[i] }
    }
    return len
}

#else
@_cdecl("corenfc_swift_avail")
public func corenfcSwiftAvailFn() -> Bool { false }
@_cdecl("corenfc_reading_available")
public func corenfcReadingAvailable() -> Bool { false }
#endif
