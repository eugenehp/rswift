//! ObjC selector constants for Contacts.
#![allow(dead_code)]

// ── CNContactStore (12 methods, 0 properties) ──
pub mod c_n_contact_store {
    pub const CLASS: &[u8] = b"CNContactStore\0";
    pub const SEL_AUTHORIZATION_STATUS_FOR_ENTITY_TYPE: &[u8] = b"authorizationStatusForEntityType:\0";
    pub const SEL_UNIFIED_CONTACTS_MATCHING_PREDICATE: &[u8] = b"unifiedContactsMatchingPredicate:keysToFetch:error:\0";
    pub const SEL_UNIFIED_CONTACT_WITH_IDENTIFIER: &[u8] = b"unifiedContactWithIdentifier:keysToFetch:error:\0";
    pub const SEL_UNIFIED_ME_CONTACT_WITH_KEYS_TO_FETCH: &[u8] = b"unifiedMeContactWithKeysToFetch:error:\0";
    pub const SEL_ENUMERATOR_FOR_CONTACT_FETCH_REQUEST: &[u8] = b"enumeratorForContactFetchRequest:error:\0";
    pub const SEL_ENUMERATOR_FOR_CHANGE_HISTORY_FETCH_REQUEST: &[u8] = b"enumeratorForChangeHistoryFetchRequest:error:\0";
    pub const SEL_GROUPS_MATCHING_PREDICATE: &[u8] = b"groupsMatchingPredicate:error:\0";
    pub const SEL_CONTAINERS_MATCHING_PREDICATE: &[u8] = b"containersMatchingPredicate:error:\0";
    pub const SEL_EXECUTE_SAVE_REQUEST: &[u8] = b"executeSaveRequest:error:__WATCHOS_PROHIBITED\0";
    pub const SEL_DEFAULT_CONTAINER_IDENTIFIER: &[u8] = b"defaultContainerIdentifier\0";
}

// ── CNContact (6 methods, 0 properties) ──
pub mod c_n_contact {
    pub const SEL_PREDICATE_FOR_CONTACTS_MATCHING_NAME: &[u8] = b"predicateForContactsMatchingName:\0";
    pub const SEL_PREDICATE_FOR_CONTACTS_MATCHING_EMAIL_ADDRESS: &[u8] = b"predicateForContactsMatchingEmailAddress:\0";
    pub const SEL_PREDICATE_FOR_CONTACTS_MATCHING_PHONE_NUMBER: &[u8] = b"predicateForContactsMatchingPhoneNumber:\0";
    pub const SEL_PREDICATE_FOR_CONTACTS_WITH_IDENTIFIERS: &[u8] = b"predicateForContactsWithIdentifiers:\0";
    pub const SEL_PREDICATE_FOR_CONTACTS_IN_GROUP_WITH_IDENTIFIER: &[u8] = b"predicateForContactsInGroupWithIdentifier:\0";
    pub const SEL_PREDICATE_FOR_CONTACTS_IN_CONTAINER_WITH_IDENTIFIER: &[u8] = b"predicateForContactsInContainerWithIdentifier:\0";
}

// ── CNContactFetchRequest (0 methods, 4 properties) ──
pub mod c_n_contact_fetch_request {
    pub const SEL_PREDICATE: &[u8] = b"predicate\0";
    pub const SEL_SET_PREDICATE: &[u8] = b"setPredicate:\0";
    pub const SEL_KEYS_TO_FETCH: &[u8] = b"keysToFetch\0";
    pub const SEL_SET_KEYS_TO_FETCH: &[u8] = b"setKeysToFetch:\0";
    pub const SEL_UNIFY_RESULTS: &[u8] = b"unifyResults\0";
    pub const SEL_SET_UNIFY_RESULTS: &[u8] = b"setUnifyResults:\0";
    pub const SEL_SORT_ORDER: &[u8] = b"sortOrder\0";
    pub const SEL_SET_SORT_ORDER: &[u8] = b"setSortOrder:\0";
}

// ── CNMutableContact (0 methods, 27 properties) ──
pub mod c_n_mutable_contact {
    pub const SEL_CONTACT_TYPE: &[u8] = b"contactType\0";
    pub const SEL_SET_CONTACT_TYPE: &[u8] = b"setContactType:\0";
    pub const SEL_NAME_PREFIX: &[u8] = b"namePrefix\0";
    pub const SEL_SET_NAME_PREFIX: &[u8] = b"setNamePrefix:\0";
    pub const SEL_GIVEN_NAME: &[u8] = b"givenName\0";
    pub const SEL_SET_GIVEN_NAME: &[u8] = b"setGivenName:\0";
    pub const SEL_MIDDLE_NAME: &[u8] = b"middleName\0";
    pub const SEL_SET_MIDDLE_NAME: &[u8] = b"setMiddleName:\0";
    pub const SEL_FAMILY_NAME: &[u8] = b"familyName\0";
    pub const SEL_SET_FAMILY_NAME: &[u8] = b"setFamilyName:\0";
    pub const SEL_PREVIOUS_FAMILY_NAME: &[u8] = b"previousFamilyName\0";
    pub const SEL_SET_PREVIOUS_FAMILY_NAME: &[u8] = b"setPreviousFamilyName:\0";
    pub const SEL_NAME_SUFFIX: &[u8] = b"nameSuffix\0";
    pub const SEL_SET_NAME_SUFFIX: &[u8] = b"setNameSuffix:\0";
    pub const SEL_NICKNAME: &[u8] = b"nickname\0";
    pub const SEL_SET_NICKNAME: &[u8] = b"setNickname:\0";
    pub const SEL_ORGANIZATION_NAME: &[u8] = b"organizationName\0";
    pub const SEL_SET_ORGANIZATION_NAME: &[u8] = b"setOrganizationName:\0";
    pub const SEL_DEPARTMENT_NAME: &[u8] = b"departmentName\0";
    pub const SEL_SET_DEPARTMENT_NAME: &[u8] = b"setDepartmentName:\0";
    pub const SEL_JOB_TITLE: &[u8] = b"jobTitle\0";
    pub const SEL_SET_JOB_TITLE: &[u8] = b"setJobTitle:\0";
    pub const SEL_PHONETIC_GIVEN_NAME: &[u8] = b"phoneticGivenName\0";
    pub const SEL_SET_PHONETIC_GIVEN_NAME: &[u8] = b"setPhoneticGivenName:\0";
    pub const SEL_PHONETIC_MIDDLE_NAME: &[u8] = b"phoneticMiddleName\0";
    pub const SEL_SET_PHONETIC_MIDDLE_NAME: &[u8] = b"setPhoneticMiddleName:\0";
    pub const SEL_PHONETIC_FAMILY_NAME: &[u8] = b"phoneticFamilyName\0";
    pub const SEL_SET_PHONETIC_FAMILY_NAME: &[u8] = b"setPhoneticFamilyName:\0";
    pub const SEL_PHONETIC_ORGANIZATION_NAME: &[u8] = b"phoneticOrganizationName\0";
    pub const SEL_SET_PHONETIC_ORGANIZATION_NAME: &[u8] = b"setPhoneticOrganizationName:\0";
    pub const SEL_NOTE: &[u8] = b"note\0";
    pub const SEL_SET_NOTE: &[u8] = b"setNote:\0";
    pub const SEL_IMAGE_DATA: &[u8] = b"imageData\0";
    pub const SEL_SET_IMAGE_DATA: &[u8] = b"setImageData:\0";
    pub const SEL_PHONE_NUMBERS: &[u8] = b"phoneNumbers\0";
    pub const SEL_SET_PHONE_NUMBERS: &[u8] = b"setPhoneNumbers:\0";
    pub const SEL_EMAIL_ADDRESSES: &[u8] = b"emailAddresses\0";
    pub const SEL_SET_EMAIL_ADDRESSES: &[u8] = b"setEmailAddresses:\0";
    pub const SEL_POSTAL_ADDRESSES: &[u8] = b"postalAddresses\0";
    pub const SEL_SET_POSTAL_ADDRESSES: &[u8] = b"setPostalAddresses:\0";
    pub const SEL_URL_ADDRESSES: &[u8] = b"urlAddresses\0";
    pub const SEL_SET_URL_ADDRESSES: &[u8] = b"setUrlAddresses:\0";
    pub const SEL_CONTACT_RELATIONS: &[u8] = b"contactRelations\0";
    pub const SEL_SET_CONTACT_RELATIONS: &[u8] = b"setContactRelations:\0";
    pub const SEL_SOCIAL_PROFILES: &[u8] = b"socialProfiles\0";
    pub const SEL_SET_SOCIAL_PROFILES: &[u8] = b"setSocialProfiles:\0";
    pub const SEL_INSTANT_MESSAGE_ADDRESSES: &[u8] = b"instantMessageAddresses\0";
    pub const SEL_SET_INSTANT_MESSAGE_ADDRESSES: &[u8] = b"setInstantMessageAddresses:\0";
    pub const SEL_BIRTHDAY: &[u8] = b"birthday\0";
    pub const SEL_SET_BIRTHDAY: &[u8] = b"setBirthday:\0";
    pub const SEL_NON_GREGORIAN_BIRTHDAY: &[u8] = b"nonGregorianBirthday\0";
    pub const SEL_SET_NON_GREGORIAN_BIRTHDAY: &[u8] = b"setNonGregorianBirthday:\0";
    pub const SEL_DATES: &[u8] = b"dates\0";
    pub const SEL_SET_DATES: &[u8] = b"setDates:\0";
}

// ── CNSaveRequest (10 methods, 0 properties) ──
pub mod c_n_save_request {
    pub const SEL_ADD_CONTACT: &[u8] = b"addContact:toContainerWithIdentifier:\0";
    pub const SEL_UPDATE_CONTACT: &[u8] = b"updateContact:\0";
    pub const SEL_DELETE_CONTACT: &[u8] = b"deleteContact:\0";
    pub const SEL_ADD_GROUP: &[u8] = b"addGroup:toContainerWithIdentifier:\0";
    pub const SEL_UPDATE_GROUP: &[u8] = b"updateGroup:\0";
    pub const SEL_DELETE_GROUP: &[u8] = b"deleteGroup:\0";
    pub const SEL_ADD_SUBGROUP: &[u8] = b"addSubgroup:toGroup:\0";
    pub const SEL_REMOVE_SUBGROUP: &[u8] = b"removeSubgroup:fromGroup:\0";
    pub const SEL_ADD_MEMBER: &[u8] = b"addMember:toGroup:\0";
    pub const SEL_REMOVE_MEMBER: &[u8] = b"removeMember:fromGroup:\0";
}

// ── CNLabeledValue (5 methods, 3 properties) ──
pub mod c_n_labeled_value {
    pub const SEL_IDENTIFIER: &[u8] = b"identifier\0";
    pub const SEL_SET_IDENTIFIER: &[u8] = b"setIdentifier:\0";
    pub const SEL_LABEL: &[u8] = b"label\0";
    pub const SEL_SET_LABEL: &[u8] = b"setLabel:\0";
    pub const SEL_VALUE: &[u8] = b"value\0";
    pub const SEL_SET_VALUE: &[u8] = b"setValue:\0";
    pub const SEL_LABELED_VALUE_WITH_LABEL: &[u8] = b"labeledValueWithLabel:value:\0";
    pub const SEL_LABELED_VALUE_BY_SETTING_LABEL: &[u8] = b"labeledValueBySettingLabel:\0";
    pub const SEL_LABELED_VALUE_BY_SETTING_VALUE: &[u8] = b"labeledValueBySettingValue:\0";
    pub const SEL_LOCALIZED_STRING_FOR_LABEL: &[u8] = b"localizedStringForLabel:\0";
}

// Total: 100 selector constants
