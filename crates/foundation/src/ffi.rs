//! ObjC selector constants for Foundation.
#![allow(dead_code)]

// ── NSUserDefaults (33 methods, 2 properties) ──
pub mod n_s_user_defaults {
    pub const CLASS: &[u8] = b"NSUserDefaults\0";
    pub const SEL_STANDARD_USER_DEFAULTS: &[u8] = b"standardUserDefaults\0";
    pub const SEL_SET_STANDARD_USER_DEFAULTS: &[u8] = b"setStandardUserDefaults:\0";
    pub const SEL_VOLATILE_DOMAIN_NAMES: &[u8] = b"volatileDomainNames\0";
    pub const SEL_SET_VOLATILE_DOMAIN_NAMES: &[u8] = b"setVolatileDomainNames:\0";
    pub const SEL_RESET_STANDARD_USER_DEFAULTS: &[u8] = b"resetStandardUserDefaults\0";
    pub const SEL_OBJECT_FOR_KEY: &[u8] = b"objectForKey:\0";
    pub const SEL_SET_OBJECT: &[u8] = b"setObject:forKey:\0";
    pub const SEL_REMOVE_OBJECT_FOR_KEY: &[u8] = b"removeObjectForKey:\0";
    pub const SEL_STRING_FOR_KEY: &[u8] = b"stringForKey:\0";
    pub const SEL_ARRAY_FOR_KEY: &[u8] = b"arrayForKey:\0";
    pub const SEL_DICTIONARY_FOR_KEY: &[u8] = b"dictionaryForKey:\0";
    pub const SEL_DATA_FOR_KEY: &[u8] = b"dataForKey:\0";
    pub const SEL_STRING_ARRAY_FOR_KEY: &[u8] = b"stringArrayForKey:\0";
    pub const SEL_INTEGER_FOR_KEY: &[u8] = b"integerForKey:\0";
    pub const SEL_FLOAT_FOR_KEY: &[u8] = b"floatForKey:\0";
    pub const SEL_DOUBLE_FOR_KEY: &[u8] = b"doubleForKey:\0";
    pub const SEL_BOOL_FOR_KEY: &[u8] = b"boolForKey:\0";
    pub const SEL_U_R_L_FOR_KEY: &[u8] = b"URLForKey:\0";
    pub const SEL_SET_INTEGER: &[u8] = b"setInteger:forKey:\0";
    pub const SEL_SET_FLOAT: &[u8] = b"setFloat:forKey:\0";
    pub const SEL_SET_DOUBLE: &[u8] = b"setDouble:forKey:\0";
    pub const SEL_SET_BOOL: &[u8] = b"setBool:forKey:\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:forKey:\0";
    pub const SEL_REGISTER_DEFAULTS: &[u8] = b"registerDefaults:\0";
    pub const SEL_ADD_SUITE_NAMED: &[u8] = b"addSuiteNamed:\0";
    pub const SEL_REMOVE_SUITE_NAMED: &[u8] = b"removeSuiteNamed:\0";
    pub const SEL_DICTIONARY_REPRESENTATION: &[u8] = b"dictionaryRepresentation\0";
    pub const SEL_VOLATILE_DOMAIN_FOR_NAME: &[u8] = b"volatileDomainForName:\0";
    pub const SEL_SET_VOLATILE_DOMAIN: &[u8] = b"setVolatileDomain:forName:\0";
    pub const SEL_REMOVE_VOLATILE_DOMAIN_FOR_NAME: &[u8] = b"removeVolatileDomainForName:\0";
    pub const SEL_PERSISTENT_DOMAIN_NAMES: &[u8] = b"persistentDomainNames\0";
    pub const SEL_PERSISTENT_DOMAIN_FOR_NAME: &[u8] = b"persistentDomainForName:\0";
    pub const SEL_SET_PERSISTENT_DOMAIN: &[u8] = b"setPersistentDomain:forName:\0";
    pub const SEL_REMOVE_PERSISTENT_DOMAIN_FOR_NAME: &[u8] = b"removePersistentDomainForName:\0";
    pub const SEL_SYNCHRONIZE: &[u8] = b"synchronize\0";
    pub const SEL_OBJECT_IS_FORCED_FOR_KEY: &[u8] = b"objectIsForcedForKey:\0";
}

// ── NSFileManager (67 methods, 2 properties) ──
pub mod n_s_file_manager {
    pub const SEL_DEFAULT_MANAGER: &[u8] = b"defaultManager\0";
    pub const SEL_SET_DEFAULT_MANAGER: &[u8] = b"setDefaultManager:\0";
    pub const SEL_CURRENT_DIRECTORY_PATH: &[u8] = b"currentDirectoryPath\0";
    pub const SEL_SET_CURRENT_DIRECTORY_PATH: &[u8] = b"setCurrentDirectoryPath:\0";
    pub const SEL_MOUNTED_VOLUME_U_R_LS_INCLUDING_RESOURCE_VALUES_FOR_KEYS: &[u8] = b"mountedVolumeURLsIncludingResourceValuesForKeys:options:\0";
    pub const SEL_CONTENTS_OF_DIRECTORY_AT_U_R_L: &[u8] = b"contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:\0";
    pub const SEL_U_R_LS_FOR_DIRECTORY: &[u8] = b"URLsForDirectory:inDomains:\0";
    pub const SEL_U_R_L_FOR_DIRECTORY: &[u8] = b"URLForDirectory:inDomain:appropriateForURL:create:error:\0";
    pub const SEL_GET_RELATIONSHIP: &[u8] = b"getRelationship:ofDirectoryAtURL:toItemAtURL:error:\0";
    pub const SEL_CREATE_DIRECTORY_AT_U_R_L: &[u8] = b"createDirectoryAtURL:withIntermediateDirectories:attributes:error:\0";
    pub const SEL_CREATE_SYMBOLIC_LINK_AT_U_R_L: &[u8] = b"createSymbolicLinkAtURL:withDestinationURL:error:\0";
    pub const SEL_SET_ATTRIBUTES: &[u8] = b"setAttributes:ofItemAtPath:error:\0";
    pub const SEL_CREATE_DIRECTORY_AT_PATH: &[u8] = b"createDirectoryAtPath:withIntermediateDirectories:attributes:error:\0";
    pub const SEL_CONTENTS_OF_DIRECTORY_AT_PATH: &[u8] = b"contentsOfDirectoryAtPath:error:\0";
    pub const SEL_SUBPATHS_OF_DIRECTORY_AT_PATH: &[u8] = b"subpathsOfDirectoryAtPath:error:\0";
    pub const SEL_ATTRIBUTES_OF_ITEM_AT_PATH: &[u8] = b"attributesOfItemAtPath:error:\0";
    pub const SEL_ATTRIBUTES_OF_FILE_SYSTEM_FOR_PATH: &[u8] = b"attributesOfFileSystemForPath:error:\0";
    pub const SEL_CREATE_SYMBOLIC_LINK_AT_PATH: &[u8] = b"createSymbolicLinkAtPath:withDestinationPath:error:\0";
    pub const SEL_DESTINATION_OF_SYMBOLIC_LINK_AT_PATH: &[u8] = b"destinationOfSymbolicLinkAtPath:error:\0";
    pub const SEL_COPY_ITEM_AT_PATH: &[u8] = b"copyItemAtPath:toPath:error:\0";
    pub const SEL_MOVE_ITEM_AT_PATH: &[u8] = b"moveItemAtPath:toPath:error:\0";
    pub const SEL_LINK_ITEM_AT_PATH: &[u8] = b"linkItemAtPath:toPath:error:\0";
    pub const SEL_REMOVE_ITEM_AT_PATH: &[u8] = b"removeItemAtPath:error:\0";
    pub const SEL_COPY_ITEM_AT_U_R_L: &[u8] = b"copyItemAtURL:toURL:error:\0";
    pub const SEL_MOVE_ITEM_AT_U_R_L: &[u8] = b"moveItemAtURL:toURL:error:\0";
    pub const SEL_LINK_ITEM_AT_U_R_L: &[u8] = b"linkItemAtURL:toURL:error:\0";
    pub const SEL_REMOVE_ITEM_AT_U_R_L: &[u8] = b"removeItemAtURL:error:\0";
    pub const SEL_TRASH_ITEM_AT_U_R_L: &[u8] = b"trashItemAtURL:resultingItemURL:error:\0";
    pub const SEL_FILE_ATTRIBUTES_AT_PATH: &[u8] = b"fileAttributesAtPath:traverseLink:\0";
    pub const SEL_CHANGE_FILE_ATTRIBUTES: &[u8] = b"changeFileAttributes:atPath:\0";
    pub const SEL_DIRECTORY_CONTENTS_AT_PATH: &[u8] = b"directoryContentsAtPath:\0";
    pub const SEL_FILE_SYSTEM_ATTRIBUTES_AT_PATH: &[u8] = b"fileSystemAttributesAtPath:\0";
    pub const SEL_PATH_CONTENT_OF_SYMBOLIC_LINK_AT_PATH: &[u8] = b"pathContentOfSymbolicLinkAtPath:\0";
    pub const SEL_LINK_PATH: &[u8] = b"linkPath:toPath:handler:\0";
    pub const SEL_COPY_PATH: &[u8] = b"copyPath:toPath:handler:\0";
    pub const SEL_MOVE_PATH: &[u8] = b"movePath:toPath:handler:\0";
    pub const SEL_REMOVE_FILE_AT_PATH: &[u8] = b"removeFileAtPath:handler:\0";
    pub const SEL_CHANGE_CURRENT_DIRECTORY_PATH: &[u8] = b"changeCurrentDirectoryPath:\0";
    pub const SEL_FILE_EXISTS_AT_PATH: &[u8] = b"fileExistsAtPath:\0";
    pub const SEL_IS_READABLE_FILE_AT_PATH: &[u8] = b"isReadableFileAtPath:\0";
    pub const SEL_IS_WRITABLE_FILE_AT_PATH: &[u8] = b"isWritableFileAtPath:\0";
    pub const SEL_IS_EXECUTABLE_FILE_AT_PATH: &[u8] = b"isExecutableFileAtPath:\0";
    pub const SEL_IS_DELETABLE_FILE_AT_PATH: &[u8] = b"isDeletableFileAtPath:\0";
    pub const SEL_CONTENTS_EQUAL_AT_PATH: &[u8] = b"contentsEqualAtPath:andPath:\0";
    pub const SEL_DISPLAY_NAME_AT_PATH: &[u8] = b"displayNameAtPath:\0";
    pub const SEL_COMPONENTS_TO_DISPLAY_FOR_PATH: &[u8] = b"componentsToDisplayForPath:\0";
    pub const SEL_ENUMERATOR_AT_PATH: &[u8] = b"enumeratorAtPath:\0";
    pub const SEL_SUBPATHS_AT_PATH: &[u8] = b"subpathsAtPath:\0";
    pub const SEL_CONTENTS_AT_PATH: &[u8] = b"contentsAtPath:\0";
    pub const SEL_CREATE_FILE_AT_PATH: &[u8] = b"createFileAtPath:contents:attributes:\0";
    pub const SEL_FILE_SYSTEM_REPRESENTATION_WITH_PATH: &[u8] = b"fileSystemRepresentationWithPath:\0";
    pub const SEL_STRING_WITH_FILE_SYSTEM_REPRESENTATION: &[u8] = b"stringWithFileSystemRepresentation:length:\0";
    pub const SEL_REPLACE_ITEM_AT_U_R_L: &[u8] = b"replaceItemAtURL:withItemAtURL:backupItemName:options:resultingItemURL:error:\0";
    pub const SEL_SET_UBIQUITOUS: &[u8] = b"setUbiquitous:itemAtURL:destinationURL:error:\0";
    pub const SEL_IS_UBIQUITOUS_ITEM_AT_U_R_L: &[u8] = b"isUbiquitousItemAtURL:\0";
    pub const SEL_START_DOWNLOADING_UBIQUITOUS_ITEM_AT_U_R_L: &[u8] = b"startDownloadingUbiquitousItemAtURL:error:\0";
    pub const SEL_EVICT_UBIQUITOUS_ITEM_AT_U_R_L: &[u8] = b"evictUbiquitousItemAtURL:error:\0";
    pub const SEL_U_R_L_FOR_UBIQUITY_CONTAINER_IDENTIFIER: &[u8] = b"URLForUbiquityContainerIdentifier:\0";
    pub const SEL_U_R_L_FOR_PUBLISHING_UBIQUITOUS_ITEM_AT_U_R_L: &[u8] = b"URLForPublishingUbiquitousItemAtURL:expirationDate:error:\0";
    pub const SEL_CONTAINER_U_R_L_FOR_SECURITY_APPLICATION_GROUP_IDENTIFIER: &[u8] = b"containerURLForSecurityApplicationGroupIdentifier:\0";
}

// ── NSProcessInfo (7 methods, 8 properties) ──
pub mod n_s_process_info {
    pub const SEL_PROCESS_INFO: &[u8] = b"processInfo\0";
    pub const SEL_SET_PROCESS_INFO: &[u8] = b"setProcessInfo:\0";
    pub const SEL_ENVIRONMENT: &[u8] = b"environment\0";
    pub const SEL_SET_ENVIRONMENT: &[u8] = b"setEnvironment:\0";
    pub const SEL_ARGUMENTS: &[u8] = b"arguments\0";
    pub const SEL_SET_ARGUMENTS: &[u8] = b"setArguments:\0";
    pub const SEL_HOST_NAME: &[u8] = b"hostName\0";
    pub const SEL_SET_HOST_NAME: &[u8] = b"setHostName:\0";
    pub const SEL_PROCESS_NAME: &[u8] = b"processName\0";
    pub const SEL_SET_PROCESS_NAME: &[u8] = b"setProcessName:\0";
    pub const SEL_PROCESS_IDENTIFIER: &[u8] = b"processIdentifier\0";
    pub const SEL_SET_PROCESS_IDENTIFIER: &[u8] = b"setProcessIdentifier:\0";
    pub const SEL_GLOBALLY_UNIQUE_STRING: &[u8] = b"globallyUniqueString\0";
    pub const SEL_SET_GLOBALLY_UNIQUE_STRING: &[u8] = b"setGloballyUniqueString:\0";
    pub const SEL_OPERATING_SYSTEM_VERSION_STRING: &[u8] = b"operatingSystemVersionString\0";
    pub const SEL_SET_OPERATING_SYSTEM_VERSION_STRING: &[u8] = b"setOperatingSystemVersionString:\0";
    pub const SEL_OPERATING_SYSTEM: &[u8] = b"operatingSystem\0";
    pub const SEL_OPERATING_SYSTEM_NAME: &[u8] = b"operatingSystemName\0";
    pub const SEL_IS_OPERATING_SYSTEM_AT_LEAST_VERSION: &[u8] = b"isOperatingSystemAtLeastVersion:\0";
    pub const SEL_DISABLE_SUDDEN_TERMINATION: &[u8] = b"disableSuddenTermination\0";
    pub const SEL_ENABLE_SUDDEN_TERMINATION: &[u8] = b"enableSuddenTermination\0";
    pub const SEL_DISABLE_AUTOMATIC_TERMINATION: &[u8] = b"disableAutomaticTermination:\0";
    pub const SEL_ENABLE_AUTOMATIC_TERMINATION: &[u8] = b"enableAutomaticTermination:\0";
}

// ── NSBundle (29 methods, 18 properties) ──
pub mod n_s_bundle {
    pub const SEL_MAIN_BUNDLE: &[u8] = b"mainBundle\0";
    pub const SEL_SET_MAIN_BUNDLE: &[u8] = b"setMainBundle:\0";
    pub const SEL_ALL_BUNDLES: &[u8] = b"allBundles\0";
    pub const SEL_SET_ALL_BUNDLES: &[u8] = b"setAllBundles:\0";
    pub const SEL_ALL_FRAMEWORKS: &[u8] = b"allFrameworks\0";
    pub const SEL_SET_ALL_FRAMEWORKS: &[u8] = b"setAllFrameworks:\0";
    pub const SEL_LOADED: &[u8] = b"loaded\0";
    pub const SEL_SET_LOADED: &[u8] = b"setLoaded:\0";
    pub const SEL_BUNDLE_PATH: &[u8] = b"bundlePath\0";
    pub const SEL_SET_BUNDLE_PATH: &[u8] = b"setBundlePath:\0";
    pub const SEL_RESOURCE_PATH: &[u8] = b"resourcePath\0";
    pub const SEL_SET_RESOURCE_PATH: &[u8] = b"setResourcePath:\0";
    pub const SEL_EXECUTABLE_PATH: &[u8] = b"executablePath\0";
    pub const SEL_SET_EXECUTABLE_PATH: &[u8] = b"setExecutablePath:\0";
    pub const SEL_PRIVATE_FRAMEWORKS_PATH: &[u8] = b"privateFrameworksPath\0";
    pub const SEL_SET_PRIVATE_FRAMEWORKS_PATH: &[u8] = b"setPrivateFrameworksPath:\0";
    pub const SEL_SHARED_FRAMEWORKS_PATH: &[u8] = b"sharedFrameworksPath\0";
    pub const SEL_SET_SHARED_FRAMEWORKS_PATH: &[u8] = b"setSharedFrameworksPath:\0";
    pub const SEL_SHARED_SUPPORT_PATH: &[u8] = b"sharedSupportPath\0";
    pub const SEL_SET_SHARED_SUPPORT_PATH: &[u8] = b"setSharedSupportPath:\0";
    pub const SEL_BUILT_IN_PLUG_INS_PATH: &[u8] = b"builtInPlugInsPath\0";
    pub const SEL_SET_BUILT_IN_PLUG_INS_PATH: &[u8] = b"setBuiltInPlugInsPath:\0";
    pub const SEL_BUNDLE_IDENTIFIER: &[u8] = b"bundleIdentifier\0";
    pub const SEL_SET_BUNDLE_IDENTIFIER: &[u8] = b"setBundleIdentifier:\0";
    pub const SEL_INFO_DICTIONARY: &[u8] = b"infoDictionary\0";
    pub const SEL_SET_INFO_DICTIONARY: &[u8] = b"setInfoDictionary:\0";
    pub const SEL_LOCALIZED_INFO_DICTIONARY: &[u8] = b"localizedInfoDictionary\0";
    pub const SEL_SET_LOCALIZED_INFO_DICTIONARY: &[u8] = b"setLocalizedInfoDictionary:\0";
    pub const SEL_PRINCIPAL_CLASS: &[u8] = b"principalClass\0";
    pub const SEL_SET_PRINCIPAL_CLASS: &[u8] = b"setPrincipalClass:\0";
    pub const SEL_PREFERRED_LOCALIZATIONS: &[u8] = b"preferredLocalizations\0";
    pub const SEL_SET_PREFERRED_LOCALIZATIONS: &[u8] = b"setPreferredLocalizations:\0";
    pub const SEL_LOCALIZATIONS: &[u8] = b"localizations\0";
    pub const SEL_SET_LOCALIZATIONS: &[u8] = b"setLocalizations:\0";
    pub const SEL_DEVELOPMENT_LOCALIZATION: &[u8] = b"developmentLocalization\0";
    pub const SEL_SET_DEVELOPMENT_LOCALIZATION: &[u8] = b"setDevelopmentLocalization:\0";
    pub const SEL_BUNDLE_WITH_PATH: &[u8] = b"bundleWithPath:\0";
    pub const SEL_BUNDLE_WITH_U_R_L: &[u8] = b"bundleWithURL:\0";
    pub const SEL_BUNDLE_FOR_CLASS: &[u8] = b"bundleForClass:\0";
    pub const SEL_BUNDLE_WITH_IDENTIFIER: &[u8] = b"bundleWithIdentifier:\0";
    pub const SEL_LOAD: &[u8] = b"load\0";
    pub const SEL_UNLOAD: &[u8] = b"unload\0";
    pub const SEL_PREFLIGHT_AND_RETURN_ERROR: &[u8] = b"preflightAndReturnError:\0";
    pub const SEL_LOAD_AND_RETURN_ERROR: &[u8] = b"loadAndReturnError:\0";
    pub const SEL_U_R_L_FOR_AUXILIARY_EXECUTABLE: &[u8] = b"URLForAuxiliaryExecutable:\0";
    pub const SEL_PATH_FOR_AUXILIARY_EXECUTABLE: &[u8] = b"pathForAuxiliaryExecutable:\0";
    pub const SEL_U_R_L_FOR_RESOURCE: &[u8] = b"URLForResource:withExtension:subdirectory:inBundleWithURL:\0";
    pub const SEL_U_R_LS_FOR_RESOURCES_WITH_EXTENSION: &[u8] = b"URLsForResourcesWithExtension:subdirectory:inBundleWithURL:\0";
    pub const SEL_PATH_FOR_RESOURCE: &[u8] = b"pathForResource:ofType:inDirectory:\0";
    pub const SEL_PATHS_FOR_RESOURCES_OF_TYPE: &[u8] = b"pathsForResourcesOfType:inDirectory:\0";
    pub const SEL_LOCALIZED_STRING_FOR_KEY: &[u8] = b"localizedStringForKey:value:table:\0";
    pub const SEL_LOCALIZED_ATTRIBUTED_STRING_FOR_KEY: &[u8] = b"localizedAttributedStringForKey:value:table:\0";
    pub const SEL_OBJECT_FOR_INFO_DICTIONARY_KEY: &[u8] = b"objectForInfoDictionaryKey:\0";
    pub const SEL_CLASS_NAMED: &[u8] = b"classNamed:\0";
    pub const SEL_PREFERRED_LOCALIZATIONS_FROM_ARRAY: &[u8] = b"preferredLocalizationsFromArray:\0";
}

// ── NSLocale (2 methods, 0 properties) ──
pub mod n_s_locale {
    pub const SEL_DISPLAY_NAME_FOR_KEY: &[u8] = b"displayNameForKey:value:\0";
}

// ── NSDateFormatter (6 methods, 15 properties) ──
pub mod n_s_date_formatter {
    pub const SEL_DEFAULT_FORMATTER_BEHAVIOR: &[u8] = b"defaultFormatterBehavior\0";
    pub const SEL_SET_DEFAULT_FORMATTER_BEHAVIOR: &[u8] = b"setDefaultFormatterBehavior:\0";
    pub const SEL_DATE_FORMAT: &[u8] = b"dateFormat\0";
    pub const SEL_SET_DATE_FORMAT: &[u8] = b"setDateFormat:\0";
    pub const SEL_LOCALE: &[u8] = b"locale\0";
    pub const SEL_SET_LOCALE: &[u8] = b"setLocale:\0";
    pub const SEL_TIME_ZONE: &[u8] = b"timeZone\0";
    pub const SEL_SET_TIME_ZONE: &[u8] = b"setTimeZone:\0";
    pub const SEL_CALENDAR: &[u8] = b"calendar\0";
    pub const SEL_SET_CALENDAR: &[u8] = b"setCalendar:\0";
    pub const SEL_LENIENT: &[u8] = b"lenient\0";
    pub const SEL_SET_LENIENT: &[u8] = b"setLenient:\0";
    pub const SEL_TWO_DIGIT_START_DATE: &[u8] = b"twoDigitStartDate\0";
    pub const SEL_SET_TWO_DIGIT_START_DATE: &[u8] = b"setTwoDigitStartDate:\0";
    pub const SEL_DEFAULT_DATE: &[u8] = b"defaultDate\0";
    pub const SEL_SET_DEFAULT_DATE: &[u8] = b"setDefaultDate:\0";
    pub const SEL_ERA_SYMBOLS: &[u8] = b"eraSymbols\0";
    pub const SEL_SET_ERA_SYMBOLS: &[u8] = b"setEraSymbols:\0";
    pub const SEL_MONTH_SYMBOLS: &[u8] = b"monthSymbols\0";
    pub const SEL_SET_MONTH_SYMBOLS: &[u8] = b"setMonthSymbols:\0";
    pub const SEL_SHORT_MONTH_SYMBOLS: &[u8] = b"shortMonthSymbols\0";
    pub const SEL_SET_SHORT_MONTH_SYMBOLS: &[u8] = b"setShortMonthSymbols:\0";
    pub const SEL_WEEKDAY_SYMBOLS: &[u8] = b"weekdaySymbols\0";
    pub const SEL_SET_WEEKDAY_SYMBOLS: &[u8] = b"setWeekdaySymbols:\0";
    pub const SEL_SHORT_WEEKDAY_SYMBOLS: &[u8] = b"shortWeekdaySymbols\0";
    pub const SEL_SET_SHORT_WEEKDAY_SYMBOLS: &[u8] = b"setShortWeekdaySymbols:\0";
    pub const SEL_A_M_SYMBOL: &[u8] = b"AMSymbol\0";
    pub const SEL_SET_A_M_SYMBOL: &[u8] = b"setAMSymbol:\0";
    pub const SEL_P_M_SYMBOL: &[u8] = b"PMSymbol\0";
    pub const SEL_SET_P_M_SYMBOL: &[u8] = b"setPMSymbol:\0";
    pub const SEL_GET_OBJECT_VALUE: &[u8] = b"getObjectValue:forString:range:error:\0";
    pub const SEL_STRING_FROM_DATE: &[u8] = b"stringFromDate:\0";
    pub const SEL_DATE_FROM_STRING: &[u8] = b"dateFromString:\0";
    pub const SEL_LOCALIZED_STRING_FROM_DATE: &[u8] = b"localizedStringFromDate:dateStyle:timeStyle:\0";
    pub const SEL_DATE_FORMAT_FROM_TEMPLATE: &[u8] = b"dateFormatFromTemplate:options:locale:\0";
    pub const SEL_SET_LOCALIZED_DATE_FORMAT_FROM_TEMPLATE: &[u8] = b"setLocalizedDateFormatFromTemplate:\0";
}

// ── NSJSONSerialization (5 methods, 0 properties) ──
pub mod n_s_j_s_o_n_serialization {
    pub const SEL_IS_VALID_J_S_O_N_OBJECT: &[u8] = b"isValidJSONObject:\0";
    pub const SEL_DATA_WITH_J_S_O_N_OBJECT: &[u8] = b"dataWithJSONObject:options:error:\0";
    pub const SEL_J_S_O_N_OBJECT_WITH_DATA: &[u8] = b"JSONObjectWithData:options:error:\0";
    pub const SEL_WRITE_J_S_O_N_OBJECT: &[u8] = b"writeJSONObject:toStream:options:error:\0";
    pub const SEL_J_S_O_N_OBJECT_WITH_STREAM: &[u8] = b"JSONObjectWithStream:options:error:\0";
}

// ── NSUUID (3 methods, 1 properties) ──
pub mod n_s_u_u_i_d {
    pub const SEL_U_U_I_D_STRING: &[u8] = b"UUIDString\0";
    pub const SEL_SET_U_U_I_D_STRING: &[u8] = b"setUUIDString:\0";
    pub const SEL_U_U_I_D: &[u8] = b"UUID\0";
    pub const SEL_GET_U_U_I_D_BYTES: &[u8] = b"getUUIDBytes:\0";
    pub const SEL_COMPARE: &[u8] = b"compare:\0";
}

// ── NSURLSession (22 methods, 5 properties) ──
pub mod n_s_u_r_l_session {
    pub const SEL_SHARED_SESSION: &[u8] = b"sharedSession\0";
    pub const SEL_SET_SHARED_SESSION: &[u8] = b"setSharedSession:\0";
    pub const SEL_DELEGATE_QUEUE: &[u8] = b"delegateQueue\0";
    pub const SEL_SET_DELEGATE_QUEUE: &[u8] = b"setDelegateQueue:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_CONFIGURATION: &[u8] = b"configuration\0";
    pub const SEL_SET_CONFIGURATION: &[u8] = b"setConfiguration:\0";
    pub const SEL_SESSION_DESCRIPTION: &[u8] = b"sessionDescription\0";
    pub const SEL_SET_SESSION_DESCRIPTION: &[u8] = b"setSessionDescription:\0";
    pub const SEL_SESSION_WITH_CONFIGURATION: &[u8] = b"sessionWithConfiguration:\0";
    pub const SEL_FINISH_TASKS_AND_INVALIDATE: &[u8] = b"finishTasksAndInvalidate\0";
    pub const SEL_INVALIDATE_AND_CANCEL: &[u8] = b"invalidateAndCancel\0";
    pub const SEL_DATA_TASK_WITH_REQUEST: &[u8] = b"dataTaskWithRequest:\0";
    pub const SEL_DATA_TASK_WITH_U_R_L: &[u8] = b"dataTaskWithURL:\0";
    pub const SEL_UPLOAD_TASK_WITH_REQUEST: &[u8] = b"uploadTaskWithRequest:fromFile:\0";
    pub const SEL_UPLOAD_TASK_WITH_RESUME_DATA: &[u8] = b"uploadTaskWithResumeData:\0";
    pub const SEL_UPLOAD_TASK_WITH_STREAMED_REQUEST: &[u8] = b"uploadTaskWithStreamedRequest:\0";
    pub const SEL_DOWNLOAD_TASK_WITH_REQUEST: &[u8] = b"downloadTaskWithRequest:\0";
    pub const SEL_DOWNLOAD_TASK_WITH_U_R_L: &[u8] = b"downloadTaskWithURL:\0";
    pub const SEL_DOWNLOAD_TASK_WITH_RESUME_DATA: &[u8] = b"downloadTaskWithResumeData:\0";
    pub const SEL_STREAM_TASK_WITH_HOST_NAME: &[u8] = b"streamTaskWithHostName:port:\0";
    pub const SEL_STREAM_TASK_WITH_NET_SERVICE: &[u8] = b"streamTaskWithNetService:\0";
    pub const SEL_WEB_SOCKET_TASK_WITH_U_R_L: &[u8] = b"webSocketTaskWithURL:\0";
    pub const SEL_WEB_SOCKET_TASK_WITH_REQUEST: &[u8] = b"webSocketTaskWithRequest:\0";
}

// ── NSURLRequest (2 methods, 5 properties) ──
pub mod n_s_u_r_l_request {
    pub const SEL_SUPPORTS_SECURE_CODING: &[u8] = b"supportsSecureCoding\0";
    pub const SEL_SET_SUPPORTS_SECURE_CODING: &[u8] = b"setSupportsSecureCoding:\0";
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_CACHE_POLICY: &[u8] = b"cachePolicy\0";
    pub const SEL_SET_CACHE_POLICY: &[u8] = b"setCachePolicy:\0";
    pub const SEL_TIMEOUT_INTERVAL: &[u8] = b"timeoutInterval\0";
    pub const SEL_SET_TIMEOUT_INTERVAL: &[u8] = b"setTimeoutInterval:\0";
    pub const SEL_MAIN_DOCUMENT_U_R_L: &[u8] = b"mainDocumentURL\0";
    pub const SEL_SET_MAIN_DOCUMENT_U_R_L: &[u8] = b"setMainDocumentURL:\0";
    pub const SEL_REQUEST_WITH_U_R_L: &[u8] = b"requestWithURL:\0";
}

// Total: 269 selector constants
