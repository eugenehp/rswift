//! ObjC selector constants for GameKit.
#![allow(dead_code)]

// ── GKLocalPlayer (4 methods, 0 properties) ──
pub mod g_k_local_player {
    pub const CLASS: &[u8] = b"GKLocalPlayer\0";
}

// ── GKAchievement (2 methods, 0 properties) ──
pub mod g_k_achievement {
    pub const CLASS: &[u8] = b"GKAchievement\0";
}

// ── GKLeaderboard (6 methods, 1 properties) ──
pub mod g_k_leaderboard {
    pub const CLASS: &[u8] = b"GKLeaderboard\0";
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
}

// ── GKMatch (6 methods, 2 properties) ──
pub mod g_k_match {
    pub const CLASS: &[u8] = b"GKMatch\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_EXPECTED_PLAYER_COUNT: &[u8] = b"expectedPlayerCount\0";
    pub const SEL_SET_EXPECTED_PLAYER_COUNT: &[u8] = b"setExpectedPlayerCount:\0";
    pub const SEL_SEND_DATA: &[u8] = b"sendData:toPlayers:dataMode:error:\0";
    pub const SEL_SEND_DATA_TO_ALL_PLAYERS: &[u8] = b"sendDataToAllPlayers:withDataMode:error:\0";
    pub const SEL_DISCONNECT: &[u8] = b"disconnect\0";
    pub const SEL_VOICE_CHAT_WITH_NAME: &[u8] = b"voiceChatWithName:\0";
}

// ── GKMatchRequest (1 methods, 4 properties) ──
pub mod g_k_match_request {
    pub const CLASS: &[u8] = b"GKMatchRequest\0";
    pub const SEL_MIN_PLAYERS: &[u8] = b"minPlayers\0";
    pub const SEL_SET_MIN_PLAYERS: &[u8] = b"setMinPlayers:\0";
    pub const SEL_MAX_PLAYERS: &[u8] = b"maxPlayers\0";
    pub const SEL_SET_MAX_PLAYERS: &[u8] = b"setMaxPlayers:\0";
    pub const SEL_PLAYER_GROUP: &[u8] = b"playerGroup\0";
    pub const SEL_SET_PLAYER_GROUP: &[u8] = b"setPlayerGroup:\0";
    pub const SEL_PLAYER_ATTRIBUTES: &[u8] = b"playerAttributes\0";
    pub const SEL_SET_PLAYER_ATTRIBUTES: &[u8] = b"setPlayerAttributes:\0";
    pub const SEL_MAX_PLAYERS_ALLOWED_FOR_MATCH_OF_TYPE: &[u8] = b"maxPlayersAllowedForMatchOfType:\0";
}

// ── GKMatchmaker (16 methods, 0 properties) ──
pub mod g_k_matchmaker {
    pub const CLASS: &[u8] = b"GKMatchmaker\0";
    pub const SEL_SHARED_MATCHMAKER: &[u8] = b"sharedMatchmaker\0";
    pub const SEL_CANCEL: &[u8] = b"cancel\0";
    pub const SEL_CANCEL_PENDING_INVITE_TO_PLAYER: &[u8] = b"cancelPendingInviteToPlayer:\0";
    pub const SEL_FINISH_MATCHMAKING_FOR_MATCH: &[u8] = b"finishMatchmakingForMatch:\0";
    pub const SEL_STOP_BROWSING_FOR_NEARBY_PLAYERS: &[u8] = b"stopBrowsingForNearbyPlayers\0";
    pub const SEL_STOP_GROUP_ACTIVITY: &[u8] = b"stopGroupActivity\0";
}

// ── GKTurnBasedMatch (21 methods, 7 properties) ──
pub mod g_k_turn_based_match {
    pub const CLASS: &[u8] = b"GKTurnBasedMatch\0";
    pub const SEL_MATCH_I_D: &[u8] = b"matchID\0";
    pub const SEL_SET_MATCH_I_D: &[u8] = b"setMatchID:\0";
    pub const SEL_CREATION_DATE: &[u8] = b"creationDate\0";
    pub const SEL_SET_CREATION_DATE: &[u8] = b"setCreationDate:\0";
    pub const SEL_PARTICIPANTS: &[u8] = b"participants\0";
    pub const SEL_SET_PARTICIPANTS: &[u8] = b"setParticipants:\0";
    pub const SEL_STATUS: &[u8] = b"status\0";
    pub const SEL_SET_STATUS: &[u8] = b"setStatus:\0";
    pub const SEL_CURRENT_PARTICIPANT: &[u8] = b"currentParticipant\0";
    pub const SEL_SET_CURRENT_PARTICIPANT: &[u8] = b"setCurrentParticipant:\0";
    pub const SEL_MATCH_DATA: &[u8] = b"matchData\0";
    pub const SEL_SET_MATCH_DATA: &[u8] = b"setMatchData:\0";
    pub const SEL_MESSAGE: &[u8] = b"message\0";
    pub const SEL_SET_MESSAGE: &[u8] = b"setMessage:\0";
    pub const SEL_SET_LOCALIZABLE_MESSAGE_WITH_KEY: &[u8] = b"setLocalizableMessageWithKey:arguments:\0";
}

// Total: 80 selector constants
