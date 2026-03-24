//! ObjC selector constants for QuartzCore.
#![allow(dead_code)]

// ── CALayer (2 methods, 1 properties) ──
pub mod c_a_layer {
    pub const CLASS: &[u8] = b"CALayer\0";
    pub const SEL_VISIBLE_RECT: &[u8] = b"visibleRect\0";
    pub const SEL_SET_VISIBLE_RECT: &[u8] = b"setVisibleRect:\0";
    pub const SEL_SCROLL_POINT: &[u8] = b"scrollPoint:\0";
    pub const SEL_SCROLL_RECT_TO_VISIBLE: &[u8] = b"scrollRectToVisible:\0";
}

// ── CAAnimation (3 methods, 3 properties) ──
pub mod c_a_animation {
    pub const SEL_TIMING_FUNCTION: &[u8] = b"timingFunction\0";
    pub const SEL_SET_TIMING_FUNCTION: &[u8] = b"setTimingFunction:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_REMOVED_ON_COMPLETION: &[u8] = b"removedOnCompletion\0";
    pub const SEL_SET_REMOVED_ON_COMPLETION: &[u8] = b"setRemovedOnCompletion:\0";
    pub const SEL_ANIMATION: &[u8] = b"animation\0";
    pub const SEL_DEFAULT_VALUE_FOR_KEY: &[u8] = b"defaultValueForKey:\0";
    pub const SEL_SHOULD_ARCHIVE_VALUE_FOR_KEY: &[u8] = b"shouldArchiveValueForKey:\0";
}

// ── CABasicAnimation (0 methods, 3 properties) ──
pub mod c_a_basic_animation {
    pub const SEL_FROM_VALUE: &[u8] = b"fromValue\0";
    pub const SEL_SET_FROM_VALUE: &[u8] = b"setFromValue:\0";
    pub const SEL_TO_VALUE: &[u8] = b"toValue\0";
    pub const SEL_SET_TO_VALUE: &[u8] = b"setToValue:\0";
    pub const SEL_BY_VALUE: &[u8] = b"byValue\0";
    pub const SEL_SET_BY_VALUE: &[u8] = b"setByValue:\0";
}

// ── CAKeyframeAnimation (0 methods, 9 properties) ──
pub mod c_a_keyframe_animation {
    pub const SEL_VALUES: &[u8] = b"values\0";
    pub const SEL_SET_VALUES: &[u8] = b"setValues:\0";
    pub const SEL_PATH: &[u8] = b"path\0";
    pub const SEL_SET_PATH: &[u8] = b"setPath:\0";
    pub const SEL_KEY_TIMES: &[u8] = b"keyTimes\0";
    pub const SEL_SET_KEY_TIMES: &[u8] = b"setKeyTimes:\0";
    pub const SEL_TIMING_FUNCTIONS: &[u8] = b"timingFunctions\0";
    pub const SEL_SET_TIMING_FUNCTIONS: &[u8] = b"setTimingFunctions:\0";
    pub const SEL_CALCULATION_MODE: &[u8] = b"calculationMode\0";
    pub const SEL_SET_CALCULATION_MODE: &[u8] = b"setCalculationMode:\0";
    pub const SEL_TENSION_VALUES: &[u8] = b"tensionValues\0";
    pub const SEL_SET_TENSION_VALUES: &[u8] = b"setTensionValues:\0";
    pub const SEL_CONTINUITY_VALUES: &[u8] = b"continuityValues\0";
    pub const SEL_SET_CONTINUITY_VALUES: &[u8] = b"setContinuityValues:\0";
    pub const SEL_BIAS_VALUES: &[u8] = b"biasValues\0";
    pub const SEL_SET_BIAS_VALUES: &[u8] = b"setBiasValues:\0";
    pub const SEL_ROTATION_MODE: &[u8] = b"rotationMode\0";
    pub const SEL_SET_ROTATION_MODE: &[u8] = b"setRotationMode:\0";
}

// ── CATransaction (15 methods, 0 properties) ──
pub mod c_a_transaction {
    pub const SEL_BEGIN: &[u8] = b"begin\0";
    pub const SEL_COMMIT: &[u8] = b"commit\0";
    pub const SEL_FLUSH: &[u8] = b"flush\0";
    pub const SEL_LOCK: &[u8] = b"lock\0";
    pub const SEL_UNLOCK: &[u8] = b"unlock\0";
    pub const SEL_ANIMATION_DURATION: &[u8] = b"animationDuration\0";
    pub const SEL_SET_ANIMATION_DURATION: &[u8] = b"setAnimationDuration:\0";
    pub const SEL_ANIMATION_TIMING_FUNCTION: &[u8] = b"animationTimingFunction\0";
    pub const SEL_SET_ANIMATION_TIMING_FUNCTION: &[u8] = b"setAnimationTimingFunction:\0";
    pub const SEL_DISABLE_ACTIONS: &[u8] = b"disableActions\0";
    pub const SEL_SET_DISABLE_ACTIONS: &[u8] = b"setDisableActions:\0";
    pub const SEL_VALUE_FOR_KEY: &[u8] = b"valueForKey:\0";
    pub const SEL_SET_VALUE: &[u8] = b"setValue:forKey:\0";
}

// ── CADisplayLink (4 methods, 3 properties) ──
pub mod c_a_display_link {
    pub const SEL_TIMESTAMP: &[u8] = b"timestamp\0";
    pub const SEL_SET_TIMESTAMP: &[u8] = b"setTimestamp:\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_PAUSED: &[u8] = b"paused\0";
    pub const SEL_SET_PAUSED: &[u8] = b"setPaused:\0";
    pub const SEL_DISPLAY_LINK_WITH_TARGET: &[u8] = b"displayLinkWithTarget:selector:\0";
    pub const SEL_ADD_TO_RUN_LOOP: &[u8] = b"addToRunLoop:forMode:\0";
    pub const SEL_REMOVE_FROM_RUN_LOOP: &[u8] = b"removeFromRunLoop:forMode:\0";
    pub const SEL_INVALIDATE: &[u8] = b"invalidate\0";
}

// ── CAMediaTimingFunction (3 methods, 0 properties) ──
pub mod c_a_media_timing_function {
    pub const SEL_FUNCTION_WITH_NAME: &[u8] = b"functionWithName:\0";
    pub const SEL_FUNCTION_WITH_CONTROL_POINTS: &[u8] = b"functionWithControlPoints::::\0";
    pub const SEL_GET_CONTROL_POINT_AT_INDEX: &[u8] = b"getControlPointAtIndex:values:\0";
}

// ── CAShapeLayer (0 methods, 7 properties) ──
pub mod c_a_shape_layer {
    pub const SEL_FILL_COLOR: &[u8] = b"fillColor\0";
    pub const SEL_SET_FILL_COLOR: &[u8] = b"setFillColor:\0";
    pub const SEL_FILL_RULE: &[u8] = b"fillRule\0";
    pub const SEL_SET_FILL_RULE: &[u8] = b"setFillRule:\0";
    pub const SEL_STROKE_COLOR: &[u8] = b"strokeColor\0";
    pub const SEL_SET_STROKE_COLOR: &[u8] = b"setStrokeColor:\0";
    pub const SEL_LINE_CAP: &[u8] = b"lineCap\0";
    pub const SEL_SET_LINE_CAP: &[u8] = b"setLineCap:\0";
    pub const SEL_LINE_JOIN: &[u8] = b"lineJoin\0";
    pub const SEL_SET_LINE_JOIN: &[u8] = b"setLineJoin:\0";
    pub const SEL_LINE_DASH_PATTERN: &[u8] = b"lineDashPattern\0";
    pub const SEL_SET_LINE_DASH_PATTERN: &[u8] = b"setLineDashPattern:\0";
}

// ── CAGradientLayer (0 methods, 3 properties) ──
pub mod c_a_gradient_layer {
    pub const SEL_COLORS: &[u8] = b"colors\0";
    pub const SEL_SET_COLORS: &[u8] = b"setColors:\0";
    pub const SEL_LOCATIONS: &[u8] = b"locations\0";
    pub const SEL_SET_LOCATIONS: &[u8] = b"setLocations:\0";
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
}

// ── CATextLayer (0 methods, 6 properties) ──
pub mod c_a_text_layer {
    pub const SEL_STRING: &[u8] = b"string\0";
    pub const SEL_SET_STRING: &[u8] = b"setString:\0";
    pub const SEL_FONT: &[u8] = b"font\0";
    pub const SEL_SET_FONT: &[u8] = b"setFont:\0";
    pub const SEL_FOREGROUND_COLOR: &[u8] = b"foregroundColor\0";
    pub const SEL_SET_FOREGROUND_COLOR: &[u8] = b"setForegroundColor:\0";
    pub const SEL_WRAPPED: &[u8] = b"wrapped\0";
    pub const SEL_SET_WRAPPED: &[u8] = b"setWrapped:\0";
    pub const SEL_TRUNCATION_MODE: &[u8] = b"truncationMode\0";
    pub const SEL_SET_TRUNCATION_MODE: &[u8] = b"setTruncationMode:\0";
    pub const SEL_ALIGNMENT_MODE: &[u8] = b"alignmentMode\0";
    pub const SEL_SET_ALIGNMENT_MODE: &[u8] = b"setAlignmentMode:\0";
}

// Total: 97 selector constants
