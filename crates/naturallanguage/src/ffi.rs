//! ObjC selector constants for NaturalLanguage.
#![allow(dead_code)]

// ── NLLanguageRecognizer (4 methods, 1 properties) ──
pub mod n_l_language_recognizer {
    pub const CLASS: &[u8] = b"NLLanguageRecognizer\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
    pub const SEL_DOMINANT_LANGUAGE_FOR_STRING: &[u8] = b"dominantLanguageForString:\0";
    pub const SEL_PROCESS_STRING: &[u8] = b"processString:\0";
    pub const SEL_RESET: &[u8] = b"reset\0";
    pub const SEL_LANGUAGE_HYPOTHESES_WITH_MAXIMUM: &[u8] = b"languageHypothesesWithMaximum:\0";
}

// ── NLTokenizer (5 methods, 0 properties) ──
pub mod n_l_tokenizer {
    pub const SEL_SET_LANGUAGE: &[u8] = b"setLanguage:\0";
    pub const SEL_TOKEN_RANGE_AT_INDEX: &[u8] = b"tokenRangeAtIndex:\0";
    pub const SEL_TOKEN_RANGE_FOR_RANGE: &[u8] = b"tokenRangeForRange:\0";
    pub const SEL_TOKENS_FOR_RANGE: &[u8] = b"tokensForRange:\0";
}

// ── NLTagger (14 methods, 0 properties) ──
pub mod n_l_tagger {
    pub const SEL_AVAILABLE_TAG_SCHEMES_FOR_UNIT: &[u8] = b"availableTagSchemesForUnit:language:\0";
    pub const SEL_TAG_AT_INDEX: &[u8] = b"tagAtIndex:unit:scheme:tokenRange:\0";
    pub const SEL_TAGS_IN_RANGE: &[u8] = b"tagsInRange:unit:scheme:options:tokenRanges:\0";
    pub const SEL_TAG_HYPOTHESES_AT_INDEX: &[u8] = b"tagHypothesesAtIndex:unit:scheme:maximumCount:tokenRange:\0";
    pub const SEL_SET_ORTHOGRAPHY: &[u8] = b"setOrthography:range:\0";
    pub const SEL_SET_MODELS: &[u8] = b"setModels:forTagScheme:\0";
    pub const SEL_MODELS_FOR_TAG_SCHEME: &[u8] = b"modelsForTagScheme:\0";
    pub const SEL_SET_GAZETTEERS: &[u8] = b"setGazetteers:forTagScheme:\0";
    pub const SEL_GAZETTEERS_FOR_TAG_SCHEME: &[u8] = b"gazetteersForTagScheme:\0";
}

// ── NLEmbedding (22 methods, 0 properties) ──
pub mod n_l_embedding {
    pub const SEL_WORD_EMBEDDING_FOR_LANGUAGE: &[u8] = b"wordEmbeddingForLanguage:\0";
    pub const SEL_SENTENCE_EMBEDDING_FOR_LANGUAGE: &[u8] = b"sentenceEmbeddingForLanguage:\0";
    pub const SEL_EMBEDDING_WITH_CONTENTS_OF_U_R_L: &[u8] = b"embeddingWithContentsOfURL:error:\0";
    pub const SEL_CONTAINS_STRING: &[u8] = b"containsString:\0";
    pub const SEL_DISTANCE_BETWEEN_STRING: &[u8] = b"distanceBetweenString:andString:distanceType:\0";
    pub const SEL_NEIGHBORS_FOR_STRING: &[u8] = b"neighborsForString:maximumCount:distanceType:\0";
    pub const SEL_VECTOR_FOR_STRING: &[u8] = b"vectorForString:\0";
    pub const SEL_GET_VECTOR: &[u8] = b"getVector:forString:\0";
    pub const SEL_NEIGHBORS_FOR_VECTOR: &[u8] = b"neighborsForVector:maximumCount:distanceType:\0";
    pub const SEL_SUPPORTED_REVISIONS_FOR_LANGUAGE: &[u8] = b"supportedRevisionsForLanguage:\0";
    pub const SEL_CURRENT_REVISION_FOR_LANGUAGE: &[u8] = b"currentRevisionForLanguage:\0";
    pub const SEL_SUPPORTED_SENTENCE_EMBEDDING_REVISIONS_FOR_LANGUAGE: &[u8] = b"supportedSentenceEmbeddingRevisionsForLanguage:\0";
    pub const SEL_CURRENT_SENTENCE_EMBEDDING_REVISION_FOR_LANGUAGE: &[u8] = b"currentSentenceEmbeddingRevisionForLanguage:\0";
    pub const SEL_WRITE_EMBEDDING_FOR_DICTIONARY: &[u8] = b"writeEmbeddingForDictionary:language:revision:toURL:error:\0";
}

// ── NLModel (6 methods, 0 properties) ──
pub mod n_l_model {
    pub const SEL_MODEL_WITH_CONTENTS_OF_U_R_L: &[u8] = b"modelWithContentsOfURL:error:\0";
    pub const SEL_MODEL_WITH_M_L_MODEL: &[u8] = b"modelWithMLModel:error:\0";
    pub const SEL_PREDICTED_LABEL_FOR_STRING: &[u8] = b"predictedLabelForString:\0";
    pub const SEL_PREDICTED_LABELS_FOR_TOKENS: &[u8] = b"predictedLabelsForTokens:\0";
    pub const SEL_PREDICTED_LABEL_HYPOTHESES_FOR_STRING: &[u8] = b"predictedLabelHypothesesForString:maximumCount:\0";
    pub const SEL_PREDICTED_LABEL_HYPOTHESES_FOR_TOKENS: &[u8] = b"predictedLabelHypothesesForTokens:maximumCount:\0";
}

// Total: 47 selector constants
