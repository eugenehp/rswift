//! ObjC selector constants for CoreML.
#![allow(dead_code)]

// ── MLModel (0 methods, 1 properties) ──
pub mod m_l_model {
    pub const CLASS: &[u8] = b"MLModel\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
}

// ── MLModelDescription (0 methods, 5 properties) ──
pub mod m_l_model_description {
    pub const CLASS: &[u8] = b"MLModelDescription\0";
    pub const SEL_INPUT_DESCRIPTIONS_BY_NAME: &[u8] = b"inputDescriptionsByName\0";
    pub const SEL_SET_INPUT_DESCRIPTIONS_BY_NAME: &[u8] = b"setInputDescriptionsByName:\0";
    pub const SEL_OUTPUT_DESCRIPTIONS_BY_NAME: &[u8] = b"outputDescriptionsByName\0";
    pub const SEL_SET_OUTPUT_DESCRIPTIONS_BY_NAME: &[u8] = b"setOutputDescriptionsByName:\0";
    pub const SEL_PREDICTED_FEATURE_NAME: &[u8] = b"predictedFeatureName\0";
    pub const SEL_SET_PREDICTED_FEATURE_NAME: &[u8] = b"setPredictedFeatureName:\0";
    pub const SEL_PREDICTED_PROBABILITIES_NAME: &[u8] = b"predictedProbabilitiesName\0";
    pub const SEL_SET_PREDICTED_PROBABILITIES_NAME: &[u8] = b"setPredictedProbabilitiesName:\0";
    pub const SEL_METADATA: &[u8] = b"metadata\0";
    pub const SEL_SET_METADATA: &[u8] = b"setMetadata:\0";
}

// ── MLFeatureValue (9 methods, 8 properties) ──
pub mod m_l_feature_value {
    pub const CLASS: &[u8] = b"MLFeatureValue\0";
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_UNDEFINED: &[u8] = b"undefined\0";
    pub const SEL_SET_UNDEFINED: &[u8] = b"setUndefined:\0";
    pub const SEL_INT64_VALUE: &[u8] = b"int64Value\0";
    pub const SEL_SET_INT64_VALUE: &[u8] = b"setInt64Value:\0";
    pub const SEL_DOUBLE_VALUE: &[u8] = b"doubleValue\0";
    pub const SEL_SET_DOUBLE_VALUE: &[u8] = b"setDoubleValue:\0";
    pub const SEL_STRING_VALUE: &[u8] = b"stringValue\0";
    pub const SEL_SET_STRING_VALUE: &[u8] = b"setStringValue:\0";
    pub const SEL_MULTI_ARRAY_VALUE: &[u8] = b"multiArrayValue\0";
    pub const SEL_SET_MULTI_ARRAY_VALUE: &[u8] = b"setMultiArrayValue:\0";
    pub const SEL_DICTIONARY_VALUE: &[u8] = b"dictionaryValue\0";
    pub const SEL_SET_DICTIONARY_VALUE: &[u8] = b"setDictionaryValue:\0";
    pub const SEL_IMAGE_BUFFER_VALUE: &[u8] = b"imageBufferValue\0";
    pub const SEL_SET_IMAGE_BUFFER_VALUE: &[u8] = b"setImageBufferValue:\0";
    pub const SEL_FEATURE_VALUE_WITH_INT64: &[u8] = b"featureValueWithInt64:\0";
    pub const SEL_FEATURE_VALUE_WITH_DOUBLE: &[u8] = b"featureValueWithDouble:\0";
    pub const SEL_FEATURE_VALUE_WITH_STRING: &[u8] = b"featureValueWithString:\0";
    pub const SEL_FEATURE_VALUE_WITH_MULTI_ARRAY: &[u8] = b"featureValueWithMultiArray:\0";
    pub const SEL_FEATURE_VALUE_WITH_PIXEL_BUFFER: &[u8] = b"featureValueWithPixelBuffer:\0";
    pub const SEL_FEATURE_VALUE_WITH_SEQUENCE: &[u8] = b"featureValueWithSequence:\0";
    pub const SEL_UNDEFINED_FEATURE_VALUE_WITH_TYPE: &[u8] = b"undefinedFeatureValueWithType:\0";
    pub const SEL_FEATURE_VALUE_WITH_DICTIONARY: &[u8] = b"featureValueWithDictionary:error:\0";
    pub const SEL_IS_EQUAL_TO_FEATURE_VALUE: &[u8] = b"isEqualToFeatureValue:\0";
}

// ── MLFeatureDescription (1 methods, 3 properties) ──
pub mod m_l_feature_description {
    pub const CLASS: &[u8] = b"MLFeatureDescription\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_OPTIONAL: &[u8] = b"optional\0";
    pub const SEL_SET_OPTIONAL: &[u8] = b"setOptional:\0";
    pub const SEL_IS_ALLOWED_VALUE: &[u8] = b"isAllowedValue:\0";
}

// ── MLDictionaryFeatureProvider (1 methods, 1 properties) ──
pub mod m_l_dictionary_feature_provider {
    pub const CLASS: &[u8] = b"MLDictionaryFeatureProvider\0";
    pub const SEL_DICTIONARY: &[u8] = b"dictionary\0";
    pub const SEL_SET_DICTIONARY: &[u8] = b"setDictionary:\0";
    pub const SEL_OBJECT_FOR_KEYED_SUBSCRIPT: &[u8] = b"objectForKeyedSubscript:\0";
}

// ── MLMultiArray (0 methods, 4 properties) ──
pub mod m_l_multi_array {
    pub const CLASS: &[u8] = b"MLMultiArray\0";
    pub const SEL_DATA_TYPE: &[u8] = b"dataType\0";
    pub const SEL_SET_DATA_TYPE: &[u8] = b"setDataType:\0";
    pub const SEL_SHAPE: &[u8] = b"shape\0";
    pub const SEL_SET_SHAPE: &[u8] = b"setShape:\0";
    pub const SEL_STRIDES: &[u8] = b"strides\0";
    pub const SEL_SET_STRIDES: &[u8] = b"setStrides:\0";
    pub const SEL_COUNT: &[u8] = b"count\0";
    pub const SEL_SET_COUNT: &[u8] = b"setCount:\0";
}

// ── MLModelConfiguration (0 methods, 2 properties) ──
pub mod m_l_model_configuration {
    pub const CLASS: &[u8] = b"MLModelConfiguration\0";
    pub const SEL_COMPUTE_UNITS: &[u8] = b"computeUnits\0";
    pub const SEL_SET_COMPUTE_UNITS: &[u8] = b"setComputeUnits:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
}

// Total: 59 selector constants
