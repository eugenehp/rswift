import Foundation
#if canImport(TipKit)
import TipKit

@_cdecl("tipkit_swift_available")
public func tipkitSwiftAvailFn() -> Bool { true }

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("tipkit_configure")
public func tipkitConfigure(_ displayFrequency: Int) {
    // 0 = immediate, 1 = hourly, 2 = daily, 3 = weekly, 4 = monthly
    Task {
        do {
            switch displayFrequency {
            case 0: try Tips.configure([.displayFrequency(.immediate)])
            case 1: try Tips.configure([.displayFrequency(.hourly)])
            case 2: try Tips.configure([.displayFrequency(.daily)])
            case 3: try Tips.configure([.displayFrequency(.weekly)])
            case 4: try Tips.configure([.displayFrequency(.monthly)])
            default: try Tips.configure()
            }
        } catch {}
    }
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("tipkit_reset_datastore")
public func tipkitResetDatastore() {
    Task {
        do {
            try Tips.resetDatastore()
        } catch {}
    }
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("tipkit_show_all_tips")
public func tipkitShowAllTips() {
    Tips.showAllTipsForTesting()
}

@available(macOS 14.0, iOS 17.0, *)
@_cdecl("tipkit_hide_all_tips")
public func tipkitHideAllTips() {
    Tips.hideAllTipsForTesting()
}

#else
@_cdecl("tipkit_swift_available")
public func tipkitSwiftAvailFn() -> Bool { false }
#endif
