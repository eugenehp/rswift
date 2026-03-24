//! ObjC selector constants for CloudKit.
#![allow(dead_code)]

// ── CKContainer (3 methods, 1 properties) ──
pub mod c_k_container {
    pub const CLASS: &[u8] = b"CKContainer\0";
    pub const SEL_CONTAINER_IDENTIFIER: &[u8] = b"containerIdentifier\0";
    pub const SEL_SET_CONTAINER_IDENTIFIER: &[u8] = b"setContainerIdentifier:\0";
    pub const SEL_DEFAULT_CONTAINER: &[u8] = b"defaultContainer\0";
    pub const SEL_CONTAINER_WITH_IDENTIFIER: &[u8] = b"containerWithIdentifier:\0";
    pub const SEL_ADD_OPERATION: &[u8] = b"addOperation:\0";
}

// ── CKDatabase (1 methods, 0 properties) ──
pub mod c_k_database {
    pub const CLASS: &[u8] = b"CKDatabase\0";
    pub const SEL_ADD_OPERATION: &[u8] = b"addOperation:\0";
}

// ── CKRecord (10 methods, 7 properties) ──
pub mod c_k_record {
    pub const CLASS: &[u8] = b"CKRecord\0";
    pub const SEL_RECORD_TYPE: &[u8] = b"recordType\0";
    pub const SEL_SET_RECORD_TYPE: &[u8] = b"setRecordType:\0";
    pub const SEL_RECORD_I_D: &[u8] = b"recordID\0";
    pub const SEL_SET_RECORD_I_D: &[u8] = b"setRecordID:\0";
    pub const SEL_RECORD_CHANGE_TAG: &[u8] = b"recordChangeTag\0";
    pub const SEL_SET_RECORD_CHANGE_TAG: &[u8] = b"setRecordChangeTag:\0";
    pub const SEL_CREATOR_USER_RECORD_I_D: &[u8] = b"creatorUserRecordID\0";
    pub const SEL_SET_CREATOR_USER_RECORD_I_D: &[u8] = b"setCreatorUserRecordID:\0";
    pub const SEL_CREATION_DATE: &[u8] = b"creationDate\0";
    pub const SEL_SET_CREATION_DATE: &[u8] = b"setCreationDate:\0";
    pub const SEL_LAST_MODIFIED_USER_RECORD_I_D: &[u8] = b"lastModifiedUserRecordID\0";
    pub const SEL_SET_LAST_MODIFIED_USER_RECORD_I_D: &[u8] = b"setLastModifiedUserRecordID:\0";
    pub const SEL_MODIFICATION_DATE: &[u8] = b"modificationDate\0";
    pub const SEL_SET_MODIFICATION_DATE: &[u8] = b"setModificationDate:\0";
    pub const SEL_OBJECT_FOR_KEY: &[u8] = b"objectForKey:\0";
    pub const SEL_SET_OBJECT: &[u8] = b"setObject:forKey:\0";
    pub const SEL_ALL_KEYS: &[u8] = b"allKeys\0";
    pub const SEL_ALL_TOKENS: &[u8] = b"allTokens\0";
    pub const SEL_OBJECT_FOR_KEYED_SUBSCRIPT: &[u8] = b"objectForKeyedSubscript:\0";
    pub const SEL_CHANGED_KEYS: &[u8] = b"changedKeys\0";
    pub const SEL_ENCODE_SYSTEM_FIELDS_WITH_CODER: &[u8] = b"encodeSystemFieldsWithCoder:\0";
    pub const SEL_SET_PARENT_REFERENCE_FROM_RECORD: &[u8] = b"setParentReferenceFromRecord:\0";
    pub const SEL_SET_PARENT_REFERENCE_FROM_RECORD_I_D: &[u8] = b"setParentReferenceFromRecordID:\0";
}

// ── CKRecordID (0 methods, 2 properties) ──
pub mod c_k_record_i_d {
    pub const CLASS: &[u8] = b"CKRecordID\0";
    pub const SEL_RECORD_NAME: &[u8] = b"recordName\0";
    pub const SEL_SET_RECORD_NAME: &[u8] = b"setRecordName:\0";
    pub const SEL_ZONE_I_D: &[u8] = b"zoneID\0";
    pub const SEL_SET_ZONE_I_D: &[u8] = b"setZoneID:\0";
}

// ── CKRecordZone (1 methods, 2 properties) ──
pub mod c_k_record_zone {
    pub const CLASS: &[u8] = b"CKRecordZone\0";
    pub const SEL_ZONE_I_D: &[u8] = b"zoneID\0";
    pub const SEL_SET_ZONE_I_D: &[u8] = b"setZoneID:\0";
    pub const SEL_CAPABILITIES: &[u8] = b"capabilities\0";
    pub const SEL_SET_CAPABILITIES: &[u8] = b"setCapabilities:\0";
    pub const SEL_DEFAULT_RECORD_ZONE: &[u8] = b"defaultRecordZone\0";
}

// ── CKQuery (0 methods, 3 properties) ──
pub mod c_k_query {
    pub const CLASS: &[u8] = b"CKQuery\0";
    pub const SEL_RECORD_TYPE: &[u8] = b"recordType\0";
    pub const SEL_SET_RECORD_TYPE: &[u8] = b"setRecordType:\0";
    pub const SEL_PREDICATE: &[u8] = b"predicate\0";
    pub const SEL_SET_PREDICATE: &[u8] = b"setPredicate:\0";
    pub const SEL_SORT_DESCRIPTORS: &[u8] = b"sortDescriptors\0";
    pub const SEL_SET_SORT_DESCRIPTORS: &[u8] = b"setSortDescriptors:\0";
}

// ── CKQueryOperation (0 methods, 6 properties) ──
pub mod c_k_query_operation {
    pub const CLASS: &[u8] = b"CKQueryOperation\0";
    pub const SEL_QUERY: &[u8] = b"query\0";
    pub const SEL_SET_QUERY: &[u8] = b"setQuery:\0";
    pub const SEL_CURSOR: &[u8] = b"cursor\0";
    pub const SEL_SET_CURSOR: &[u8] = b"setCursor:\0";
    pub const SEL_ZONE_I_D: &[u8] = b"zoneID\0";
    pub const SEL_SET_ZONE_I_D: &[u8] = b"setZoneID:\0";
    pub const SEL_RESULTS_LIMIT: &[u8] = b"resultsLimit\0";
    pub const SEL_SET_RESULTS_LIMIT: &[u8] = b"setResultsLimit:\0";
    pub const SEL_DESIRED_KEYS: &[u8] = b"desiredKeys\0";
    pub const SEL_SET_DESIRED_KEYS: &[u8] = b"setDesiredKeys:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
}

// ── CKModifyRecordsOperation (0 methods, 5 properties) ──
pub mod c_k_modify_records_operation {
    pub const CLASS: &[u8] = b"CKModifyRecordsOperation\0";
    pub const SEL_RECORDS_TO_SAVE: &[u8] = b"recordsToSave\0";
    pub const SEL_SET_RECORDS_TO_SAVE: &[u8] = b"setRecordsToSave:\0";
    pub const SEL_RECORD_I_DS_TO_DELETE: &[u8] = b"recordIDsToDelete\0";
    pub const SEL_SET_RECORD_I_DS_TO_DELETE: &[u8] = b"setRecordIDsToDelete:\0";
    pub const SEL_SAVE_POLICY: &[u8] = b"savePolicy\0";
    pub const SEL_SET_SAVE_POLICY: &[u8] = b"setSavePolicy:\0";
    pub const SEL_CLIENT_CHANGE_TOKEN_DATA: &[u8] = b"clientChangeTokenData\0";
    pub const SEL_SET_CLIENT_CHANGE_TOKEN_DATA: &[u8] = b"setClientChangeTokenData:\0";
    pub const SEL_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"NS_REFINED_FOR_SWIFT\0";
    pub const SEL_SET_N_S_R_E_F_I_N_E_D_F_O_R_S_W_I_F_T: &[u8] = b"setNS_REFINED_FOR_SWIFT:\0";
}

// ── CKSubscription (0 methods, 0 properties) ──
pub mod c_k_subscription {
    pub const CLASS: &[u8] = b"CKSubscription\0";
}

// Total: 66 selector constants
