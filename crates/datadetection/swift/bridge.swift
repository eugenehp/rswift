import Foundation
#if canImport(DataDetection)
import DataDetection

@_cdecl("datadetection_swift_available")
public func datadetectionSwiftAvailable() -> Bool { true }

// Use NSDataDetector which is stable and available on macOS 10.7+
@_cdecl("datadetection_detect")
public func datadetectionDetect(
    _ textPtr: UnsafePointer<UInt8>, _ textLen: Int,
    _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int
) -> Int {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    guard let detector = try? NSDataDetector(types: NSTextCheckingAllTypes) else { return -1 }
    let range = NSRange(text.startIndex..., in: text)
    let matches = detector.matches(in: text, options: [], range: range)
    var lines: [String] = []
    for match in matches {
        if let r = Range(match.range, in: text) {
            let start = text.distance(from: text.startIndex, to: r.lowerBound)
            let end   = text.distance(from: text.startIndex, to: r.upperBound)
            let matched = esc(String(text[r]))
            let kind = detectionKind(match)
            lines.append("{\"start\":\(start),\"end\":\(end),\"text\":\"\(matched)\",\"type\":\"\(kind)\"}")
        }
    }
    let result = "[" + lines.joined(separator: ",") + "]"
    let data = Array(result.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}

private func detectionKind(_ m: NSTextCheckingResult) -> String {
    switch m.resultType {
    case .address:   return "address"
    case .link:      return "url"
    case .phoneNumber: return "phone"
    case .date:      return "date"
    default:         return "other"
    }
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
}
#else
@_cdecl("datadetection_swift_available")
public func datadetectionSwiftAvailable() -> Bool { false }
#endif
