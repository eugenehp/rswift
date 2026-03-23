import Foundation
import QuartzCore

// ═══════════════════════════════════════════════════════════════════════════
// CoreAnimation — CADisplayLink, timing
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("coreanimation_available")
public func coreanimationAvailable() -> Bool { true }

@_cdecl("coreanimation_current_media_time")
public func coreanimationCurrentMediaTime() -> Double {
    CACurrentMediaTime()
}

// ── CATransaction ───────────────────────────────────────────────────────────

@_cdecl("coreanimation_transaction_begin")
public func coreanimationTransactionBegin() {
    CATransaction.begin()
}

@_cdecl("coreanimation_transaction_commit")
public func coreanimationTransactionCommit() {
    CATransaction.commit()
}

@_cdecl("coreanimation_transaction_flush")
public func coreanimationTransactionFlush() {
    CATransaction.flush()
}

@_cdecl("coreanimation_transaction_set_duration")
public func coreanimationTransactionSetDuration(_ duration: Double) {
    CATransaction.setAnimationDuration(duration)
}

@_cdecl("coreanimation_transaction_set_disable_actions")
public func coreanimationTransactionSetDisableActions(_ disable: Bool) {
    CATransaction.setDisableActions(disable)
}

// ── CAMediaTimingFunction ───────────────────────────────────────────────────

// Constants for timing function names (returned as Int for simplicity)
// 0=linear, 1=easeIn, 2=easeOut, 3=easeInEaseOut, 4=default
@_cdecl("coreanimation_transaction_set_timing")
public func coreanimationTransactionSetTiming(_ timing: Int) {
    let name: CAMediaTimingFunctionName
    switch timing {
    case 0: name = .linear
    case 1: name = .easeIn
    case 2: name = .easeOut
    case 3: name = .easeInEaseOut
    default: name = .default
    }
    CATransaction.setAnimationTimingFunction(CAMediaTimingFunction(name: name))
}
