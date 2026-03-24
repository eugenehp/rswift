//! ObjC selector constants for Accessibility.
#![allow(dead_code)]

// ── AXCustomContent (2 methods, 5 properties) ──
pub mod a_x_custom_content {
    pub const CLASS: &[u8] = b"AXCustomContent\0";
    pub const SEL_LABEL: &[u8] = b"label\0";
    pub const SEL_SET_LABEL: &[u8] = b"setLabel:\0";
    pub const SEL_ATTRIBUTED_LABEL: &[u8] = b"attributedLabel\0";
    pub const SEL_SET_ATTRIBUTED_LABEL: &[u8] = b"setAttributedLabel:\0";
    pub const SEL_VALUE: &[u8] = b"value\0";
    pub const SEL_SET_VALUE: &[u8] = b"setValue:\0";
    pub const SEL_ATTRIBUTED_VALUE: &[u8] = b"attributedValue\0";
    pub const SEL_SET_ATTRIBUTED_VALUE: &[u8] = b"setAttributedValue:\0";
    pub const SEL_IMPORTANCE: &[u8] = b"importance\0";
    pub const SEL_SET_IMPORTANCE: &[u8] = b"setImportance:\0";
    pub const SEL_CUSTOM_CONTENT_WITH_LABEL: &[u8] = b"customContentWithLabel:value:\0";
    pub const SEL_CUSTOM_CONTENT_WITH_ATTRIBUTED_LABEL: &[u8] = b"customContentWithAttributedLabel:attributedValue:\0";
}

// ── AXChartDescriptor (0 methods, 8 properties) ──
pub mod a_x_chart_descriptor {
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
    pub const SEL_ATTRIBUTED_TITLE: &[u8] = b"attributedTitle\0";
    pub const SEL_SET_ATTRIBUTED_TITLE: &[u8] = b"setAttributedTitle:\0";
    pub const SEL_SUMMARY: &[u8] = b"summary\0";
    pub const SEL_SET_SUMMARY: &[u8] = b"setSummary:\0";
    pub const SEL_CONTENT_DIRECTION: &[u8] = b"contentDirection\0";
    pub const SEL_SET_CONTENT_DIRECTION: &[u8] = b"setContentDirection:\0";
    pub const SEL_CONTENT_FRAME: &[u8] = b"contentFrame\0";
    pub const SEL_SET_CONTENT_FRAME: &[u8] = b"setContentFrame:\0";
    pub const SEL_SERIES: &[u8] = b"series\0";
    pub const SEL_SET_SERIES: &[u8] = b"setSeries:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
    pub const SEL_Y_AXIS: &[u8] = b"yAxis\0";
    pub const SEL_SET_Y_AXIS: &[u8] = b"setYAxis:\0";
}

// ── AXDataPoint (0 methods, 5 properties) ──
pub mod a_x_data_point {
    pub const SEL_X_VALUE: &[u8] = b"xValue\0";
    pub const SEL_SET_X_VALUE: &[u8] = b"setXValue:\0";
    pub const SEL_Y_VALUE: &[u8] = b"yValue\0";
    pub const SEL_SET_Y_VALUE: &[u8] = b"setYValue:\0";
}

// ── AXNumericDataAxisDescriptor (0 methods, 2 properties) ──
pub mod a_x_numeric_data_axis_descriptor {
    pub const SEL_SCALE_TYPE: &[u8] = b"scaleType\0";
    pub const SEL_SET_SCALE_TYPE: &[u8] = b"setScaleType:\0";
}

// Total: 42 selector constants
