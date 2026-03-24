//! Apple GameKit — Game Center from Rust.
//!
//! **Platform:** macOS 10.8+, iOS 4.1+, tvOS 9+, visionOS 1+.
//!
//! ```ignore
//! let player = gamekit::LocalPlayer::shared();
//! println!("Authenticated: {}", player.is_authenticated());
//! println!("Display name: {}", player.display_name());
//! println!("Game player ID: {}", player.game_player_id());
//! ```

//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

use apple_objc_sys::*;

/// Framework FFI constants.
pub mod ffi;

pub fn is_available() -> bool { true }

/// ObjC selector constants for framework classes.

/// The local player (wraps `GKLocalPlayer`).
pub struct LocalPlayer { inner: Id }

impl LocalPlayer {
    /// The singleton local player.
    pub fn shared() -> Self {
        Self { inner: unsafe { msg_send![class!(b"GKLocalPlayer\0"), localPlayer] } }
    }

    pub fn is_authenticated(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isAuthenticated] }
    }

    pub fn display_name(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, displayName]).unwrap_or_default() }
    }

    pub fn alias(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, alias]).unwrap_or_default() }
    }

    pub fn game_player_id(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, gamePlayerID]).unwrap_or_default() }
    }

    pub fn team_player_id(&self) -> String {
        unsafe { nsstring_to_string(msg_send![self.inner, teamPlayerID]).unwrap_or_default() }
    }

    pub fn is_underage(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isUnderage] }
    }

    pub fn is_multiplayer_gaming_restricted(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isMultiplayerGamingRestricted] }
    }

    pub fn is_personalized_communication_restricted(&self) -> bool {
        unsafe { msg_send_t![bool; self.inner, isPersonalizedCommunicationRestricted] }
    }

    pub fn as_ptr(&self) -> Id { self.inner }
}

/// A leaderboard score submission.
pub struct LeaderboardScore;

impl LeaderboardScore {
    /// Submit a score to a leaderboard (fire-and-forget).
    pub fn submit(leaderboard_id: &str, score: i64, context: u64) {
        unsafe {
            let entry: Id = msg_send![class!(b"GKLeaderboardScore\0"), new];
            let lid = nsstring(leaderboard_id);
            msg_send_void![entry, setLeaderboardID: lid];
            CFRelease(lid as CFTypeRef);

            let sel = sel_registerName(b"setValue:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, i64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(entry, sel, score);

            let sel_ctx = sel_registerName(b"setContext:\0".as_ptr());
            let f_ctx: unsafe extern "C" fn(Id, Sel, u64) =
                core::mem::transmute(objc_msgSend as *const ());
            f_ctx(entry, sel_ctx, context);

            let player = LocalPlayer::shared();
            msg_send_void![entry, setPlayer: player.inner];

            let _sel_arr = sel_registerName(b"arrayWithObject:\0".as_ptr());
            let arr: Id = msg_send![class!(b"NSArray\0"), arrayWithObject: entry];

            let _sel_submit = sel_registerName(b"submitScore:withEligibleChallenges:withCompletionHandler:\0".as_ptr());
            // Simplified: use class method
            // 5-arg message send — use manual transmute
            let sel5 = sel_registerName(b"submitScore:context:player:leaderboardIDs:completionHandler:\0".as_ptr());
            let f5: unsafe extern "C" fn(Id, Sel, i64, u64, Id, Id, Id) =
                core::mem::transmute(objc_msgSend as *const ());
            f5(class!(b"GKLeaderboard\0") as Id, sel5, score, context, player.inner, arr, NIL);
        }
    }
}

/// Achievement reporting.
pub struct Achievement;

impl Achievement {
    /// Report achievement progress (0.0 – 100.0).
    pub fn report(identifier: &str, percent: f64) {
        unsafe {
            let ns = nsstring(identifier);
            let a: Id = msg_send![class!(b"GKAchievement\0"), alloc];
            let a = msg_send![a, initWithIdentifier: ns];
            CFRelease(ns as CFTypeRef);

            let sel = sel_registerName(b"setPercentComplete:\0".as_ptr());
            let f: unsafe extern "C" fn(Id, Sel, f64) =
                core::mem::transmute(objc_msgSend as *const ());
            f(a, sel, percent);

            let arr = msg_send![class!(b"NSArray\0"), arrayWithObject: a];
            msg_send_void![class!(b"GKAchievement\0"), reportAchievements: arr, withCompletionHandler: NIL];
        }
    }

    /// Reset all achievements (fire-and-forget).
    pub fn reset_all() {
        unsafe {
            msg_send_void![class!(b"GKAchievement\0"), resetAchievementsWithCompletionHandler: NIL];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_player() {
        let p = LocalPlayer::shared();
        // Not authenticated in test env, but shouldn't crash
        let _ = p.is_authenticated();
        let _ = p.display_name();
    }
}
