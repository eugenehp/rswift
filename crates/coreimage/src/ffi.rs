//! ObjC selector constants for CoreImage.
#![allow(dead_code)]

// ── CIFilter (5 methods, 4 properties) ──
pub mod c_i_filter {
    pub const CLASS: &[u8] = b"CIFilter\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_INPUT_KEYS: &[u8] = b"inputKeys\0";
    pub const SEL_SET_INPUT_KEYS: &[u8] = b"setInputKeys:\0";
    pub const SEL_OUTPUT_KEYS: &[u8] = b"outputKeys\0";
    pub const SEL_SET_OUTPUT_KEYS: &[u8] = b"setOutputKeys:\0";
    pub const SEL_ATTRIBUTES: &[u8] = b"attributes\0";
    pub const SEL_SET_ATTRIBUTES: &[u8] = b"setAttributes:\0";
    pub const SEL_SET_DEFAULTS: &[u8] = b"setDefaults\0";
    pub const SEL_APPLY: &[u8] = b"apply:arguments:options:\0";
}

// ── CIImage (50 methods, 3 properties) ──
pub mod c_i_image {
    pub const SEL_EXTENT: &[u8] = b"extent\0";
    pub const SEL_SET_EXTENT: &[u8] = b"setExtent:\0";
    pub const SEL_OPAQUE: &[u8] = b"opaque\0";
    pub const SEL_SET_OPAQUE: &[u8] = b"setOpaque:\0";
    pub const SEL_C_F_R_E_T_U_R_N_S_N_O_T_R_E_T_A_I_N_E_D: &[u8] = b"CF_RETURNS_NOT_RETAINED\0";
    pub const SEL_SET_C_F_R_E_T_U_R_N_S_N_O_T_R_E_T_A_I_N_E_D: &[u8] = b"setCF_RETURNS_NOT_RETAINED:\0";
    pub const SEL_IMAGE_WITH_C_G_IMAGE: &[u8] = b"imageWithCGImage:\0";
    pub const SEL_IMAGE_WITH_C_G_IMAGE_SOURCE: &[u8] = b"imageWithCGImageSource:index:options:\0";
    pub const SEL_IMAGE_WITH_C_G_LAYER: &[u8] = b"imageWithCGLayer:\0";
    pub const SEL_IMAGE_WITH_BITMAP_DATA: &[u8] = b"imageWithBitmapData:bytesPerRow:size:format:colorSpace:\0";
    pub const SEL_IMAGE_WITH_TEXTURE: &[u8] = b"imageWithTexture:size:flipped:colorSpace:CI_GL_DEPRECATED\0";
    pub const SEL_IMAGE_WITH_M_T_L_TEXTURE: &[u8] = b"imageWithMTLTexture:options:\0";
    pub const SEL_IMAGE_WITH_CONTENTS_OF_U_R_L: &[u8] = b"imageWithContentsOfURL:\0";
    pub const SEL_IMAGE_WITH_DATA: &[u8] = b"imageWithData:\0";
    pub const SEL_IMAGE_WITH_C_V_IMAGE_BUFFER: &[u8] = b"imageWithCVImageBuffer:\0";
    pub const SEL_IMAGE_WITH_C_V_PIXEL_BUFFER: &[u8] = b"imageWithCVPixelBuffer:\0";
    pub const SEL_IMAGE_WITH_I_O_SURFACE: &[u8] = b"imageWithIOSurface:\0";
    pub const SEL_IMAGE_WITH_COLOR: &[u8] = b"imageWithColor:\0";
    pub const SEL_EMPTY_IMAGE: &[u8] = b"emptyImage\0";
    pub const SEL_IMAGE_BY_APPLYING_TRANSFORM: &[u8] = b"imageByApplyingTransform:\0";
    pub const SEL_IMAGE_BY_APPLYING_ORIENTATION: &[u8] = b"imageByApplyingOrientation:\0";
    pub const SEL_IMAGE_TRANSFORM_FOR_ORIENTATION: &[u8] = b"imageTransformForOrientation:\0";
    pub const SEL_IMAGE_BY_APPLYING_C_G_ORIENTATION: &[u8] = b"imageByApplyingCGOrientation:\0";
    pub const SEL_IMAGE_TRANSFORM_FOR_C_G_ORIENTATION: &[u8] = b"imageTransformForCGOrientation:\0";
    pub const SEL_IMAGE_BY_COMPOSITING_OVER_IMAGE: &[u8] = b"imageByCompositingOverImage:\0";
    pub const SEL_IMAGE_BY_CROPPING_TO_RECT: &[u8] = b"imageByCroppingToRect:\0";
    pub const SEL_IMAGE_BY_CLAMPING_TO_EXTENT: &[u8] = b"imageByClampingToExtent\0";
    pub const SEL_IMAGE_BY_CLAMPING_TO_RECT: &[u8] = b"imageByClampingToRect:\0";
    pub const SEL_IMAGE_BY_APPLYING_FILTER: &[u8] = b"imageByApplyingFilter:withInputParameters:\0";
    pub const SEL_IMAGE_BY_COLOR_MATCHING_COLOR_SPACE_TO_WORKING_SPACE: &[u8] = b"imageByColorMatchingColorSpaceToWorkingSpace:\0";
    pub const SEL_IMAGE_BY_COLOR_MATCHING_WORKING_SPACE_TO_COLOR_SPACE: &[u8] = b"imageByColorMatchingWorkingSpaceToColorSpace:\0";
    pub const SEL_IMAGE_BY_PREMULTIPLYING_ALPHA: &[u8] = b"imageByPremultiplyingAlpha\0";
    pub const SEL_IMAGE_BY_UNPREMULTIPLYING_ALPHA: &[u8] = b"imageByUnpremultiplyingAlpha\0";
    pub const SEL_IMAGE_BY_SETTING_ALPHA_ONE_IN_EXTENT: &[u8] = b"imageBySettingAlphaOneInExtent:\0";
    pub const SEL_IMAGE_BY_APPLYING_GAUSSIAN_BLUR_WITH_SIGMA: &[u8] = b"imageByApplyingGaussianBlurWithSigma:\0";
    pub const SEL_IMAGE_BY_SETTING_PROPERTIES: &[u8] = b"imageBySettingProperties:\0";
    pub const SEL_IMAGE_BY_SAMPLING_LINEAR: &[u8] = b"imageBySamplingLinear\0";
    pub const SEL_IMAGE_BY_SAMPLING_NEAREST: &[u8] = b"imageBySamplingNearest\0";
    pub const SEL_IMAGE_BY_INSERTING_INTERMEDIATE: &[u8] = b"imageByInsertingIntermediate\0";
    pub const SEL_IMAGE_BY_INSERTING_TILED_INTERMEDIATE: &[u8] = b"imageByInsertingTiledIntermediate\0";
    pub const SEL_IMAGE_BY_APPLYING_GAIN_MAP: &[u8] = b"imageByApplyingGainMap:\0";
    pub const SEL_IMAGE_BY_SETTING_CONTENT_HEADROOM: &[u8] = b"imageBySettingContentHeadroom:\0";
    pub const SEL_IMAGE_BY_SETTING_CONTENT_AVERAGE_LIGHT_LEVEL: &[u8] = b"imageBySettingContentAverageLightLevel:\0";
    pub const SEL_REGION_OF_INTEREST_FOR_IMAGE: &[u8] = b"regionOfInterestForImage:inRect:\0";
}

// ── CIContext (23 methods, 0 properties) ──
pub mod c_i_context {
    pub const SEL_CONTEXT_WITH_C_G_L_CONTEXT: &[u8] = b"contextWithCGLContext:pixelFormat:colorSpace:options:CI_GL_DEPRECATED_MAC\0";
    pub const SEL_CONTEXT_WITH_C_G_CONTEXT: &[u8] = b"contextWithCGContext:options:\0";
    pub const SEL_CONTEXT_WITH_OPTIONS: &[u8] = b"contextWithOptions:\0";
    pub const SEL_CONTEXT: &[u8] = b"context\0";
    pub const SEL_CONTEXT_WITH_E_A_G_L_CONTEXT: &[u8] = b"contextWithEAGLContext:CI_GL_DEPRECATED_IOS\0";
    pub const SEL_CONTEXT_WITH_M_T_L_DEVICE: &[u8] = b"contextWithMTLDevice:\0";
    pub const SEL_CONTEXT_WITH_M_T_L_COMMAND_QUEUE: &[u8] = b"contextWithMTLCommandQueue:\0";
    pub const SEL_DRAW_IMAGE: &[u8] = b"drawImage:atPoint:fromRect:\0";
    pub const SEL_CREATE_C_G_LAYER_WITH_SIZE: &[u8] = b"createCGLayerWithSize:info:\0";
    pub const SEL_RENDER: &[u8] = b"render:toBitmap:rowBytes:bounds:format:colorSpace:\0";
    pub const SEL_RECLAIM_RESOURCES: &[u8] = b"reclaimResources\0";
    pub const SEL_CLEAR_CACHES: &[u8] = b"clearCaches\0";
    pub const SEL_INPUT_IMAGE_MAXIMUM_SIZE: &[u8] = b"inputImageMaximumSize\0";
    pub const SEL_OUTPUT_IMAGE_MAXIMUM_SIZE: &[u8] = b"outputImageMaximumSize\0";
}

// ── CIColor (6 methods, 8 properties) ──
pub mod c_i_color {
    pub const SEL_NUMBER_OF_COMPONENTS: &[u8] = b"numberOfComponents\0";
    pub const SEL_SET_NUMBER_OF_COMPONENTS: &[u8] = b"setNumberOfComponents:\0";
    pub const SEL_N_S_R_E_T_U_R_N_S_I_N_N_E_R_P_O_I_N_T_E_R: &[u8] = b"NS_RETURNS_INNER_POINTER\0";
    pub const SEL_SET_N_S_R_E_T_U_R_N_S_I_N_N_E_R_P_O_I_N_T_E_R: &[u8] = b"setNS_RETURNS_INNER_POINTER:\0";
    pub const SEL_ALPHA: &[u8] = b"alpha\0";
    pub const SEL_SET_ALPHA: &[u8] = b"setAlpha:\0";
    pub const SEL_RED: &[u8] = b"red\0";
    pub const SEL_SET_RED: &[u8] = b"setRed:\0";
    pub const SEL_GREEN: &[u8] = b"green\0";
    pub const SEL_SET_GREEN: &[u8] = b"setGreen:\0";
    pub const SEL_BLUE: &[u8] = b"blue\0";
    pub const SEL_SET_BLUE: &[u8] = b"setBlue:\0";
    pub const SEL_STRING_REPRESENTATION: &[u8] = b"stringRepresentation\0";
    pub const SEL_SET_STRING_REPRESENTATION: &[u8] = b"setStringRepresentation:\0";
    pub const SEL_COLOR_WITH_C_G_COLOR: &[u8] = b"colorWithCGColor:\0";
    pub const SEL_COLOR_WITH_RED: &[u8] = b"colorWithRed:green:blue:alpha:\0";
    pub const SEL_COLOR_WITH_STRING: &[u8] = b"colorWithString:\0";
}

// ── CIVector (10 methods, 6 properties) ──
pub mod c_i_vector {
    pub const SEL_COUNT: &[u8] = b"count\0";
    pub const SEL_SET_COUNT: &[u8] = b"setCount:\0";
    pub const SEL_X: &[u8] = b"X\0";
    pub const SEL_SET_X: &[u8] = b"setX:\0";
    pub const SEL_Y: &[u8] = b"Y\0";
    pub const SEL_SET_Y: &[u8] = b"setY:\0";
    pub const SEL_Z: &[u8] = b"Z\0";
    pub const SEL_SET_Z: &[u8] = b"setZ:\0";
    pub const SEL_W: &[u8] = b"W\0";
    pub const SEL_SET_W: &[u8] = b"setW:\0";
    pub const SEL_VECTOR_WITH_VALUES: &[u8] = b"vectorWithValues:count:\0";
    pub const SEL_VECTOR_WITH_X: &[u8] = b"vectorWithX:\0";
    pub const SEL_VECTOR_WITH_C_G_POINT: &[u8] = b"vectorWithCGPoint:\0";
    pub const SEL_VECTOR_WITH_C_G_RECT: &[u8] = b"vectorWithCGRect:\0";
    pub const SEL_VECTOR_WITH_C_G_AFFINE_TRANSFORM: &[u8] = b"vectorWithCGAffineTransform:\0";
    pub const SEL_VECTOR_WITH_STRING: &[u8] = b"vectorWithString:\0";
    pub const SEL_VALUE_AT_INDEX: &[u8] = b"valueAtIndex:\0";
}

// ── CIDetector (3 methods, 0 properties) ──
pub mod c_i_detector {
    pub const SEL_DETECTOR_OF_TYPE: &[u8] = b"detectorOfType:context:options:\0";
    pub const SEL_FEATURES_IN_IMAGE: &[u8] = b"featuresInImage:\0";
}

// Total: 109 selector constants
