//! ObjC selector constants for Cinematic.
#![allow(dead_code)]

// ── CNAssetInfo (2 methods, 9 properties) ──
pub mod c_n_asset_info {
    pub const CLASS: &[u8] = b"CNAssetInfo\0";
    pub const SEL_ASSET: &[u8] = b"asset\0";
    pub const SEL_SET_ASSET: &[u8] = b"setAsset:\0";
    pub const SEL_ALL_CINEMATIC_TRACKS: &[u8] = b"allCinematicTracks\0";
    pub const SEL_SET_ALL_CINEMATIC_TRACKS: &[u8] = b"setAllCinematicTracks:\0";
    pub const SEL_CINEMATIC_VIDEO_TRACK: &[u8] = b"cinematicVideoTrack\0";
    pub const SEL_SET_CINEMATIC_VIDEO_TRACK: &[u8] = b"setCinematicVideoTrack:\0";
    pub const SEL_CINEMATIC_DISPARITY_TRACK: &[u8] = b"cinematicDisparityTrack\0";
    pub const SEL_SET_CINEMATIC_DISPARITY_TRACK: &[u8] = b"setCinematicDisparityTrack:\0";
    pub const SEL_CINEMATIC_METADATA_TRACK: &[u8] = b"cinematicMetadataTrack\0";
    pub const SEL_SET_CINEMATIC_METADATA_TRACK: &[u8] = b"setCinematicMetadataTrack:\0";
    pub const SEL_TIME_RANGE: &[u8] = b"timeRange\0";
    pub const SEL_SET_TIME_RANGE: &[u8] = b"setTimeRange:\0";
    pub const SEL_NATURAL_SIZE: &[u8] = b"naturalSize\0";
    pub const SEL_SET_NATURAL_SIZE: &[u8] = b"setNaturalSize:\0";
    pub const SEL_PREFERRED_SIZE: &[u8] = b"preferredSize\0";
    pub const SEL_SET_PREFERRED_SIZE: &[u8] = b"setPreferredSize:\0";
    pub const SEL_PREFERRED_TRANSFORM: &[u8] = b"preferredTransform\0";
    pub const SEL_SET_PREFERRED_TRANSFORM: &[u8] = b"setPreferredTransform:\0";
}

// ── CNDecision (0 methods, 6 properties) ──
pub mod c_n_decision {
    pub const SEL_TIME: &[u8] = b"time\0";
    pub const SEL_SET_TIME: &[u8] = b"setTime:\0";
    pub const SEL_DETECTION_I_D: &[u8] = b"detectionID\0";
    pub const SEL_SET_DETECTION_I_D: &[u8] = b"setDetectionID:\0";
    pub const SEL_DETECTION_GROUP_I_D: &[u8] = b"detectionGroupID\0";
    pub const SEL_SET_DETECTION_GROUP_I_D: &[u8] = b"setDetectionGroupID:\0";
    pub const SEL_USER_DECISION: &[u8] = b"userDecision\0";
    pub const SEL_SET_USER_DECISION: &[u8] = b"setUserDecision:\0";
    pub const SEL_GROUP_DECISION: &[u8] = b"groupDecision\0";
    pub const SEL_SET_GROUP_DECISION: &[u8] = b"setGroupDecision:\0";
    pub const SEL_STRONG_DECISION: &[u8] = b"strongDecision\0";
    pub const SEL_SET_STRONG_DECISION: &[u8] = b"setStrongDecision:\0";
}

// ── CNDetection (4 methods, 6 properties) ──
pub mod c_n_detection {
    pub const SEL_DETECTION_TYPE: &[u8] = b"detectionType\0";
    pub const SEL_SET_DETECTION_TYPE: &[u8] = b"setDetectionType:\0";
    pub const SEL_NORMALIZED_RECT: &[u8] = b"normalizedRect\0";
    pub const SEL_SET_NORMALIZED_RECT: &[u8] = b"setNormalizedRect:\0";
    pub const SEL_FOCUS_DISPARITY: &[u8] = b"focusDisparity\0";
    pub const SEL_SET_FOCUS_DISPARITY: &[u8] = b"setFocusDisparity:\0";
    pub const SEL_IS_VALID_DETECTION_I_D: &[u8] = b"isValidDetectionID:\0";
    pub const SEL_IS_VALID_DETECTION_GROUP_I_D: &[u8] = b"isValidDetectionGroupID:\0";
    pub const SEL_ACCESSIBILITY_LABEL_FOR_DETECTION_TYPE: &[u8] = b"accessibilityLabelForDetectionType:\0";
    pub const SEL_DISPARITY_IN_NORMALIZED_RECT: &[u8] = b"disparityInNormalizedRect:sourceDisparity:detectionType:priorDisparity:\0";
}

// ── CNDetectionTrack (3 methods, 5 properties) ──
pub mod c_n_detection_track {
    pub const SEL_USER_CREATED: &[u8] = b"userCreated\0";
    pub const SEL_SET_USER_CREATED: &[u8] = b"setUserCreated:\0";
    pub const SEL_DISCRETE: &[u8] = b"discrete\0";
    pub const SEL_SET_DISCRETE: &[u8] = b"setDiscrete:\0";
    pub const SEL_DETECTION_AT_OR_BEFORE_TIME: &[u8] = b"detectionAtOrBeforeTime:\0";
    pub const SEL_DETECTION_NEAREST_TIME: &[u8] = b"detectionNearestTime:\0";
    pub const SEL_DETECTIONS_IN_TIME_RANGE: &[u8] = b"detectionsInTimeRange:\0";
}

// ── CNScript (23 methods, 2 properties) ──
pub mod c_n_script {
    pub const SEL_ADDED_DETECTION_TRACKS: &[u8] = b"addedDetectionTracks\0";
    pub const SEL_SET_ADDED_DETECTION_TRACKS: &[u8] = b"setAddedDetectionTracks:\0";
    pub const SEL_RELOAD_WITH_CHANGES: &[u8] = b"reloadWithChanges:\0";
    pub const SEL_CHANGES: &[u8] = b"changes\0";
    pub const SEL_CHANGES_TRIMMED_BY_TIME_RANGE: &[u8] = b"changesTrimmedByTimeRange:\0";
    pub const SEL_FRAME_AT_TIME: &[u8] = b"frameAtTime:tolerance:\0";
    pub const SEL_FRAMES_IN_TIME_RANGE: &[u8] = b"framesInTimeRange:\0";
    pub const SEL_DECISION_AT_TIME: &[u8] = b"decisionAtTime:tolerance:\0";
    pub const SEL_DECISIONS_IN_TIME_RANGE: &[u8] = b"decisionsInTimeRange:\0";
    pub const SEL_DECISION_AFTER_TIME: &[u8] = b"decisionAfterTime:\0";
    pub const SEL_DECISION_BEFORE_TIME: &[u8] = b"decisionBeforeTime:\0";
    pub const SEL_PRIMARY_DECISION_AT_TIME: &[u8] = b"primaryDecisionAtTime:\0";
    pub const SEL_SECONDARY_DECISION_AT_TIME: &[u8] = b"secondaryDecisionAtTime:\0";
    pub const SEL_TIME_RANGE_OF_TRANSITION_AFTER_DECISION: &[u8] = b"timeRangeOfTransitionAfterDecision:\0";
    pub const SEL_TIME_RANGE_OF_TRANSITION_BEFORE_DECISION: &[u8] = b"timeRangeOfTransitionBeforeDecision:\0";
    pub const SEL_USER_DECISIONS_IN_TIME_RANGE: &[u8] = b"userDecisionsInTimeRange:\0";
    pub const SEL_BASE_DECISIONS_IN_TIME_RANGE: &[u8] = b"baseDecisionsInTimeRange:\0";
    pub const SEL_DETECTION_TRACK_FOR_I_D: &[u8] = b"detectionTrackForID:\0";
    pub const SEL_DETECTION_TRACK_FOR_DECISION: &[u8] = b"detectionTrackForDecision:\0";
    pub const SEL_ADD_USER_DECISION: &[u8] = b"addUserDecision:\0";
    pub const SEL_REMOVE_USER_DECISION: &[u8] = b"removeUserDecision:\0";
    pub const SEL_REMOVE_ALL_USER_DECISIONS: &[u8] = b"removeAllUserDecisions\0";
    pub const SEL_ADD_DETECTION_TRACK: &[u8] = b"addDetectionTrack:\0";
    pub const SEL_REMOVE_DETECTION_TRACK: &[u8] = b"removeDetectionTrack:\0";
}

// ── CNRenderingSession (3 methods, 6 properties) ──
pub mod c_n_rendering_session {
    pub const SEL_COMMAND_QUEUE: &[u8] = b"commandQueue\0";
    pub const SEL_SET_COMMAND_QUEUE: &[u8] = b"setCommandQueue:\0";
    pub const SEL_SESSION_ATTRIBUTES: &[u8] = b"sessionAttributes\0";
    pub const SEL_SET_SESSION_ATTRIBUTES: &[u8] = b"setSessionAttributes:\0";
    pub const SEL_QUALITY: &[u8] = b"quality\0";
    pub const SEL_SET_QUALITY: &[u8] = b"setQuality:\0";
    pub const SEL_SOURCE_PIXEL_FORMAT_TYPES: &[u8] = b"sourcePixelFormatTypes\0";
    pub const SEL_SET_SOURCE_PIXEL_FORMAT_TYPES: &[u8] = b"setSourcePixelFormatTypes:\0";
    pub const SEL_DESTINATION_PIXEL_FORMAT_TYPES: &[u8] = b"destinationPixelFormatTypes\0";
    pub const SEL_SET_DESTINATION_PIXEL_FORMAT_TYPES: &[u8] = b"setDestinationPixelFormatTypes:\0";
    pub const SEL_ENCODE_RENDER_TO_COMMAND_BUFFER: &[u8] = b"encodeRenderToCommandBuffer:frameAttributes:sourceImage:sourceDisparity:destinationImage:\0";
}

// Total: 101 selector constants
