//! ObjC selector constants for PencilKit.
#![allow(dead_code)]

// ── PKCanvasView (0 methods, 0 properties) ──
pub mod p_k_canvas_view {
    pub const CLASS: &[u8] = b"PKCanvasView\0";
}

// ── PKDrawing (5 methods, 1 properties) ──
pub mod p_k_drawing {
    pub const SEL_BOUNDS: &[u8] = b"bounds\0";
    pub const SEL_SET_BOUNDS: &[u8] = b"setBounds:\0";
    pub const SEL_DATA_REPRESENTATION: &[u8] = b"dataRepresentation\0";
    pub const SEL_IMAGE_FROM_RECT: &[u8] = b"imageFromRect:scale:\0";
    pub const SEL_DRAWING_BY_APPLYING_TRANSFORM: &[u8] = b"drawingByApplyingTransform:\0";
    pub const SEL_DRAWING_BY_APPENDING_DRAWING: &[u8] = b"drawingByAppendingDrawing:\0";
    pub const SEL_DRAWING_BY_APPENDING_STROKES: &[u8] = b"drawingByAppendingStrokes:\0";
}

// ── PKStroke (0 methods, 6 properties) ──
pub mod p_k_stroke {
    pub const SEL_INK: &[u8] = b"ink\0";
    pub const SEL_SET_INK: &[u8] = b"setInk:\0";
    pub const SEL_TRANSFORM: &[u8] = b"transform\0";
    pub const SEL_SET_TRANSFORM: &[u8] = b"setTransform:\0";
    pub const SEL_PATH: &[u8] = b"path\0";
    pub const SEL_SET_PATH: &[u8] = b"setPath:\0";
    pub const SEL_MASK: &[u8] = b"mask\0";
    pub const SEL_SET_MASK: &[u8] = b"setMask:\0";
    pub const SEL_RENDER_BOUNDS: &[u8] = b"renderBounds\0";
    pub const SEL_SET_RENDER_BOUNDS: &[u8] = b"setRenderBounds:\0";
    pub const SEL_MASKED_PATH_RANGES: &[u8] = b"maskedPathRanges\0";
    pub const SEL_SET_MASKED_PATH_RANGES: &[u8] = b"setMaskedPathRanges:\0";
}

// ── PKInk (0 methods, 2 properties) ──
pub mod p_k_ink {
    pub const SEL_INK_TYPE: &[u8] = b"inkType\0";
    pub const SEL_SET_INK_TYPE: &[u8] = b"setInkType:\0";
    pub const SEL_COLOR: &[u8] = b"color\0";
    pub const SEL_SET_COLOR: &[u8] = b"setColor:\0";
}

// ── PKToolPicker (0 methods, 0 properties) ──
pub mod p_k_tool_picker {
}

// Total: 23 selector constants
