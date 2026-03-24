import Foundation

#if canImport(Translation)
import Translation

@_cdecl("translation_available")
public func translationAvailable() -> Bool { true }

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("translation_supported_languages")
public func translationSupportedLanguages(
    _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int
) -> Int {
    // LanguageAvailability is async — return known languages synchronously
    let langs = Locale.LanguageCode.isoLanguageCodes.prefix(100).map { $0.identifier }
    let result = langs.joined(separator: ",")
    let data = Array(result.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

#else
@_cdecl("translation_available")
public func translationAvailable() -> Bool { false }
#endif
