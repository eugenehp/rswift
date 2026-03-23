import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// AppKit / UIKit — cross-platform app lifecycle & UI basics
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AppKit) && !targetEnvironment(macCatalyst)
import AppKit

@_cdecl("appkit_available")
public func appkitAvailable() -> Bool { true }

@_cdecl("uikit_available")
public func uikitAvailable() -> Bool { false }

// ── NSApplication ───────────────────────────────────────────────────────────

@_cdecl("appkit_shared_application")
public func appkitSharedApplication() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(NSApplication.shared).toOpaque()
}

@_cdecl("appkit_app_is_running")
public func appkitAppIsRunning() -> Bool {
    guard let app = NSApp else { return false }
    return app.isRunning
}

@_cdecl("appkit_app_activate")
public func appkitAppActivate() {
    NSApplication.shared.activate(ignoringOtherApps: true)
}

@_cdecl("appkit_app_terminate")
public func appkitAppTerminate() {
    NSApplication.shared.terminate(nil)
}

@_cdecl("appkit_app_hide")
public func appkitAppHide() {
    NSApplication.shared.hide(nil)
}

@_cdecl("appkit_app_unhide")
public func appkitAppUnhide() {
    NSApplication.shared.unhide(nil)
}

@_cdecl("appkit_app_order_front_regardless")
public func appkitAppOrderFrontRegardless() {
    NSApplication.shared.windows.first?.orderFrontRegardless()
}

// ── NSScreen ────────────────────────────────────────────────────────────────

@_cdecl("appkit_main_screen_size")
public func appkitMainScreenSize(_ w: UnsafeMutablePointer<Double>, _ h: UnsafeMutablePointer<Double>) {
    if let screen = NSScreen.main {
        w.pointee = screen.frame.width
        h.pointee = screen.frame.height
    }
}

@_cdecl("appkit_screen_count")
public func appkitScreenCount() -> Int {
    NSScreen.screens.count
}

@_cdecl("appkit_main_screen_scale")
public func appkitMainScreenScale() -> Double {
    Double(NSScreen.main?.backingScaleFactor ?? 1.0)
}

// ── Clipboard (NSPasteboard) ────────────────────────────────────────────────

@_cdecl("appkit_clipboard_get_string")
public func appkitClipboardGetString(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let s = NSPasteboard.general.string(forType: .string) else { return -1 }
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return data.count
}

@_cdecl("appkit_clipboard_set_string")
public func appkitClipboardSetString(_ ptr: UnsafePointer<UInt8>, _ len: Int) {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    let pb = NSPasteboard.general
    pb.clearContents()
    pb.setString(s, forType: .string)
}

// ── NSAlert ─────────────────────────────────────────────────────────────────

@_cdecl("appkit_show_alert")
public func appkitShowAlert(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ msgPtr: UnsafePointer<UInt8>, _ msgLen: Int,
    _ style: Int  // 0=warning, 1=informational, 2=critical
) -> Int {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let msg = String(bytes: UnsafeBufferPointer(start: msgPtr, count: msgLen), encoding: .utf8) ?? ""

    let alert = NSAlert()
    alert.messageText = title
    alert.informativeText = msg
    switch style {
    case 1: alert.alertStyle = .informational
    case 2: alert.alertStyle = .critical
    default: alert.alertStyle = .warning
    }
    alert.addButton(withTitle: "OK")
    alert.addButton(withTitle: "Cancel")
    return alert.runModal() == .alertFirstButtonReturn ? 1 : 0
}

// ── Open file/URL ───────────────────────────────────────────────────────────

@_cdecl("appkit_open_url")
public func appkitOpenUrl(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> Bool {
    let s = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    guard let url = URL(string: s) else { return false }
    return NSWorkspace.shared.open(url)
}

@_cdecl("appkit_reveal_in_finder")
public func appkitRevealInFinder(_ pathPtr: UnsafePointer<UInt8>, _ pathLen: Int) {
    let path = String(bytes: UnsafeBufferPointer(start: pathPtr, count: pathLen), encoding: .utf8) ?? ""
    NSWorkspace.shared.selectFile(path, inFileViewerRootedAtPath: "")
}

// ── Dark mode ───────────────────────────────────────────────────────────────

@_cdecl("appkit_is_dark_mode")
public func appkitIsDarkMode() -> Bool {
    guard let app = NSApp else {
        // Fallback: check system appearance via UserDefaults
        let style = UserDefaults.standard.string(forKey: "AppleInterfaceStyle")
        return style?.lowercased() == "dark"
    }
    return app.effectiveAppearance.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
}

// ── NSWindow basic ──────────────────────────────────────────────────────────

@_cdecl("appkit_key_window_title")
public func appkitKeyWindowTitle(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let app = NSApp, let title = app.keyWindow?.title else { return -1 }
    let data = Array(title.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return data.count
}

@_cdecl("appkit_window_count")
public func appkitWindowCount() -> Int {
    guard let app = NSApp else { return 0 }
    return app.windows.count
}

#elseif canImport(UIKit)
import UIKit

@_cdecl("appkit_available")
public func appkitAvailable() -> Bool { false }

@_cdecl("uikit_available")
public func uikitAvailable() -> Bool { true }

@_cdecl("uikit_main_screen_size")
public func uikitMainScreenSize(_ w: UnsafeMutablePointer<Double>, _ h: UnsafeMutablePointer<Double>) {
    let bounds = UIScreen.main.bounds
    w.pointee = bounds.width
    h.pointee = bounds.height
}

@_cdecl("uikit_main_screen_scale")
public func uikitMainScreenScale() -> Double {
    Double(UIScreen.main.scale)
}

@_cdecl("uikit_is_dark_mode")
public func uikitIsDarkMode() -> Bool {
    UITraitCollection.current.userInterfaceStyle == .dark
}

@_cdecl("uikit_clipboard_get_string")
public func uikitClipboardGetString(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let s = UIPasteboard.general.string else { return -1 }
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return data.count
}

@_cdecl("uikit_clipboard_set_string")
public func uikitClipboardSetString(_ ptr: UnsafePointer<UInt8>, _ len: Int) {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    UIPasteboard.general.string = s
}

@_cdecl("uikit_open_url")
public func uikitOpenUrl(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> Bool {
    let s = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    guard let url = URL(string: s) else { return false }
    UIApplication.shared.open(url)
    return true
}

#else
@_cdecl("appkit_available")
public func appkitAvailable() -> Bool { false }
@_cdecl("uikit_available")
public func uikitAvailable() -> Bool { false }
#endif
