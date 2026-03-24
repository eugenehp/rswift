//! ObjC selector constants for MediaPlayer.
#![allow(dead_code)]

// ── MPMusicPlayerController (12 methods, 6 properties) ──
pub mod m_p_music_player_controller {
    pub const CLASS: &[u8] = b"MPMusicPlayerController\0";
    pub const SEL_APPLICATION_MUSIC_PLAYER: &[u8] = b"applicationMusicPlayer\0";
    pub const SEL_SET_APPLICATION_MUSIC_PLAYER: &[u8] = b"setApplicationMusicPlayer:\0";
    pub const SEL_SYSTEM_MUSIC_PLAYER: &[u8] = b"systemMusicPlayer\0";
    pub const SEL_SET_SYSTEM_MUSIC_PLAYER: &[u8] = b"setSystemMusicPlayer:\0";
    pub const SEL_PLAYBACK_STATE: &[u8] = b"playbackState\0";
    pub const SEL_SET_PLAYBACK_STATE: &[u8] = b"setPlaybackState:\0";
    pub const SEL_REPEAT_MODE: &[u8] = b"repeatMode\0";
    pub const SEL_SET_REPEAT_MODE: &[u8] = b"setRepeatMode:\0";
    pub const SEL_SHUFFLE_MODE: &[u8] = b"shuffleMode\0";
    pub const SEL_SET_SHUFFLE_MODE: &[u8] = b"setShuffleMode:\0";
    pub const SEL_NOW_PLAYING_ITEM: &[u8] = b"nowPlayingItem\0";
    pub const SEL_SET_NOW_PLAYING_ITEM: &[u8] = b"setNowPlayingItem:\0";
    pub const SEL_SET_QUEUE_WITH_QUERY: &[u8] = b"setQueueWithQuery:\0";
    pub const SEL_SET_QUEUE_WITH_ITEM_COLLECTION: &[u8] = b"setQueueWithItemCollection:\0";
    pub const SEL_SKIP_TO_NEXT_ITEM: &[u8] = b"skipToNextItem\0";
    pub const SEL_SKIP_TO_BEGINNING: &[u8] = b"skipToBeginning\0";
    pub const SEL_SKIP_TO_PREVIOUS_ITEM: &[u8] = b"skipToPreviousItem\0";
    pub const SEL_BEGIN_GENERATING_PLAYBACK_NOTIFICATIONS: &[u8] = b"beginGeneratingPlaybackNotifications\0";
    pub const SEL_END_GENERATING_PLAYBACK_NOTIFICATIONS: &[u8] = b"endGeneratingPlaybackNotifications\0";
}

// ── MPNowPlayingInfoCenter (1 methods, 1 properties) ──
pub mod m_p_now_playing_info_center {
    pub const SEL_NOW_PLAYING_INFO: &[u8] = b"nowPlayingInfo\0";
    pub const SEL_SET_NOW_PLAYING_INFO: &[u8] = b"setNowPlayingInfo:\0";
    pub const SEL_DEFAULT_CENTER: &[u8] = b"defaultCenter\0";
}

// ── MPRemoteCommandCenter (1 methods, 17 properties) ──
pub mod m_p_remote_command_center {
    pub const SEL_PAUSE_COMMAND: &[u8] = b"pauseCommand\0";
    pub const SEL_SET_PAUSE_COMMAND: &[u8] = b"setPauseCommand:\0";
    pub const SEL_PLAY_COMMAND: &[u8] = b"playCommand\0";
    pub const SEL_SET_PLAY_COMMAND: &[u8] = b"setPlayCommand:\0";
    pub const SEL_STOP_COMMAND: &[u8] = b"stopCommand\0";
    pub const SEL_SET_STOP_COMMAND: &[u8] = b"setStopCommand:\0";
    pub const SEL_TOGGLE_PLAY_PAUSE_COMMAND: &[u8] = b"togglePlayPauseCommand\0";
    pub const SEL_SET_TOGGLE_PLAY_PAUSE_COMMAND: &[u8] = b"setTogglePlayPauseCommand:\0";
    pub const SEL_CHANGE_PLAYBACK_RATE_COMMAND: &[u8] = b"changePlaybackRateCommand\0";
    pub const SEL_SET_CHANGE_PLAYBACK_RATE_COMMAND: &[u8] = b"setChangePlaybackRateCommand:\0";
    pub const SEL_CHANGE_REPEAT_MODE_COMMAND: &[u8] = b"changeRepeatModeCommand\0";
    pub const SEL_SET_CHANGE_REPEAT_MODE_COMMAND: &[u8] = b"setChangeRepeatModeCommand:\0";
    pub const SEL_CHANGE_SHUFFLE_MODE_COMMAND: &[u8] = b"changeShuffleModeCommand\0";
    pub const SEL_SET_CHANGE_SHUFFLE_MODE_COMMAND: &[u8] = b"setChangeShuffleModeCommand:\0";
    pub const SEL_NEXT_TRACK_COMMAND: &[u8] = b"nextTrackCommand\0";
    pub const SEL_SET_NEXT_TRACK_COMMAND: &[u8] = b"setNextTrackCommand:\0";
    pub const SEL_PREVIOUS_TRACK_COMMAND: &[u8] = b"previousTrackCommand\0";
    pub const SEL_SET_PREVIOUS_TRACK_COMMAND: &[u8] = b"setPreviousTrackCommand:\0";
    pub const SEL_SKIP_FORWARD_COMMAND: &[u8] = b"skipForwardCommand\0";
    pub const SEL_SET_SKIP_FORWARD_COMMAND: &[u8] = b"setSkipForwardCommand:\0";
    pub const SEL_SKIP_BACKWARD_COMMAND: &[u8] = b"skipBackwardCommand\0";
    pub const SEL_SET_SKIP_BACKWARD_COMMAND: &[u8] = b"setSkipBackwardCommand:\0";
    pub const SEL_SEEK_FORWARD_COMMAND: &[u8] = b"seekForwardCommand\0";
    pub const SEL_SET_SEEK_FORWARD_COMMAND: &[u8] = b"setSeekForwardCommand:\0";
    pub const SEL_SEEK_BACKWARD_COMMAND: &[u8] = b"seekBackwardCommand\0";
    pub const SEL_SET_SEEK_BACKWARD_COMMAND: &[u8] = b"setSeekBackwardCommand:\0";
    pub const SEL_RATING_COMMAND: &[u8] = b"ratingCommand\0";
    pub const SEL_SET_RATING_COMMAND: &[u8] = b"setRatingCommand:\0";
    pub const SEL_LIKE_COMMAND: &[u8] = b"likeCommand\0";
    pub const SEL_SET_LIKE_COMMAND: &[u8] = b"setLikeCommand:\0";
    pub const SEL_DISLIKE_COMMAND: &[u8] = b"dislikeCommand\0";
    pub const SEL_SET_DISLIKE_COMMAND: &[u8] = b"setDislikeCommand:\0";
    pub const SEL_BOOKMARK_COMMAND: &[u8] = b"bookmarkCommand\0";
    pub const SEL_SET_BOOKMARK_COMMAND: &[u8] = b"setBookmarkCommand:\0";
    pub const SEL_SHARED_COMMAND_CENTER: &[u8] = b"sharedCommandCenter\0";
}

// ── MPMediaItem (0 methods, 0 properties) ──
pub mod m_p_media_item {
}

// ── MPMediaQuery (11 methods, 4 properties) ──
pub mod m_p_media_query {
    pub const SEL_FILTER_PREDICATES: &[u8] = b"filterPredicates\0";
    pub const SEL_SET_FILTER_PREDICATES: &[u8] = b"setFilterPredicates:\0";
    pub const SEL_ITEMS: &[u8] = b"items\0";
    pub const SEL_SET_ITEMS: &[u8] = b"setItems:\0";
    pub const SEL_COLLECTIONS: &[u8] = b"collections\0";
    pub const SEL_SET_COLLECTIONS: &[u8] = b"setCollections:\0";
    pub const SEL_GROUPING_TYPE: &[u8] = b"groupingType\0";
    pub const SEL_SET_GROUPING_TYPE: &[u8] = b"setGroupingType:\0";
    pub const SEL_ADD_FILTER_PREDICATE: &[u8] = b"addFilterPredicate:\0";
    pub const SEL_REMOVE_FILTER_PREDICATE: &[u8] = b"removeFilterPredicate:\0";
    pub const SEL_ALBUMS_QUERY: &[u8] = b"albumsQuery\0";
    pub const SEL_ARTISTS_QUERY: &[u8] = b"artistsQuery\0";
    pub const SEL_SONGS_QUERY: &[u8] = b"songsQuery\0";
    pub const SEL_PLAYLISTS_QUERY: &[u8] = b"playlistsQuery\0";
    pub const SEL_PODCASTS_QUERY: &[u8] = b"podcastsQuery\0";
    pub const SEL_AUDIOBOOKS_QUERY: &[u8] = b"audiobooksQuery\0";
    pub const SEL_COMPILATIONS_QUERY: &[u8] = b"compilationsQuery\0";
    pub const SEL_COMPOSERS_QUERY: &[u8] = b"composersQuery\0";
    pub const SEL_GENRES_QUERY: &[u8] = b"genresQuery\0";
}

// ── MPMediaPlaylist (2 methods, 0 properties) ──
pub mod m_p_media_playlist {
}

// Total: 83 selector constants
