//! ObjC selector constants for CoreData.
#![allow(dead_code)]

// ── NSPersistentContainer (6 methods, 5 properties) ──
pub mod n_s_persistent_container {
    pub const CLASS: &[u8] = b"NSPersistentContainer\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_VIEW_CONTEXT: &[u8] = b"viewContext\0";
    pub const SEL_SET_VIEW_CONTEXT: &[u8] = b"setViewContext:\0";
    pub const SEL_MANAGED_OBJECT_MODEL: &[u8] = b"managedObjectModel\0";
    pub const SEL_SET_MANAGED_OBJECT_MODEL: &[u8] = b"setManagedObjectModel:\0";
    pub const SEL_PERSISTENT_STORE_COORDINATOR: &[u8] = b"persistentStoreCoordinator\0";
    pub const SEL_SET_PERSISTENT_STORE_COORDINATOR: &[u8] = b"setPersistentStoreCoordinator:\0";
    pub const SEL_PERSISTENT_STORE_DESCRIPTIONS: &[u8] = b"persistentStoreDescriptions\0";
    pub const SEL_SET_PERSISTENT_STORE_DESCRIPTIONS: &[u8] = b"setPersistentStoreDescriptions:\0";
    pub const SEL_PERSISTENT_CONTAINER_WITH_NAME: &[u8] = b"persistentContainerWithName:\0";
    pub const SEL_DEFAULT_DIRECTORY_U_R_L: &[u8] = b"defaultDirectoryURL\0";
    pub const SEL_NEW_BACKGROUND_CONTEXT: &[u8] = b"newBackgroundContext\0";
}

// ── NSManagedObjectContext (28 methods, 11 properties) ──
pub mod n_s_managed_object_context {
    pub const SEL_UNDO_MANAGER: &[u8] = b"undoManager\0";
    pub const SEL_SET_UNDO_MANAGER: &[u8] = b"setUndoManager:\0";
    pub const SEL_HAS_CHANGES: &[u8] = b"hasChanges\0";
    pub const SEL_SET_HAS_CHANGES: &[u8] = b"setHasChanges:\0";
    pub const SEL_INSERTED_OBJECTS: &[u8] = b"insertedObjects\0";
    pub const SEL_SET_INSERTED_OBJECTS: &[u8] = b"setInsertedObjects:\0";
    pub const SEL_UPDATED_OBJECTS: &[u8] = b"updatedObjects\0";
    pub const SEL_SET_UPDATED_OBJECTS: &[u8] = b"setUpdatedObjects:\0";
    pub const SEL_DELETED_OBJECTS: &[u8] = b"deletedObjects\0";
    pub const SEL_SET_DELETED_OBJECTS: &[u8] = b"setDeletedObjects:\0";
    pub const SEL_REGISTERED_OBJECTS: &[u8] = b"registeredObjects\0";
    pub const SEL_SET_REGISTERED_OBJECTS: &[u8] = b"setRegisteredObjects:\0";
    pub const SEL_PROPAGATES_DELETES_AT_END_OF_EVENT: &[u8] = b"propagatesDeletesAtEndOfEvent\0";
    pub const SEL_SET_PROPAGATES_DELETES_AT_END_OF_EVENT: &[u8] = b"setPropagatesDeletesAtEndOfEvent:\0";
    pub const SEL_RETAINS_REGISTERED_OBJECTS: &[u8] = b"retainsRegisteredObjects\0";
    pub const SEL_SET_RETAINS_REGISTERED_OBJECTS: &[u8] = b"setRetainsRegisteredObjects:\0";
    pub const SEL_STALENESS_INTERVAL: &[u8] = b"stalenessInterval\0";
    pub const SEL_SET_STALENESS_INTERVAL: &[u8] = b"setStalenessInterval:\0";
    pub const SEL_MERGE_POLICY: &[u8] = b"mergePolicy\0";
    pub const SEL_SET_MERGE_POLICY: &[u8] = b"setMergePolicy:\0";
    pub const SEL_OBJECT_REGISTERED_FOR_I_D: &[u8] = b"objectRegisteredForID:\0";
    pub const SEL_OBJECT_WITH_I_D: &[u8] = b"objectWithID:\0";
    pub const SEL_EXISTING_OBJECT_WITH_I_D: &[u8] = b"existingObjectWithID:error:\0";
    pub const SEL_COUNT_FOR_FETCH_REQUEST: &[u8] = b"countForFetchRequest:error:\0";
    pub const SEL_EXECUTE_REQUEST: &[u8] = b"executeRequest:error:\0";
    pub const SEL_INSERT_OBJECT: &[u8] = b"insertObject:\0";
    pub const SEL_DELETE_OBJECT: &[u8] = b"deleteObject:\0";
    pub const SEL_REFRESH_OBJECT: &[u8] = b"refreshObject:mergeChanges:\0";
    pub const SEL_DETECT_CONFLICTS_FOR_OBJECT: &[u8] = b"detectConflictsForObject:\0";
    pub const SEL_OBSERVE_VALUE_FOR_KEY_PATH: &[u8] = b"observeValueForKeyPath:ofObject:change:context:\0";
    pub const SEL_PROCESS_PENDING_CHANGES: &[u8] = b"processPendingChanges\0";
    pub const SEL_ASSIGN_OBJECT: &[u8] = b"assignObject:toPersistentStore:\0";
    pub const SEL_UNDO: &[u8] = b"undo\0";
    pub const SEL_REDO: &[u8] = b"redo\0";
    pub const SEL_RESET: &[u8] = b"reset\0";
    pub const SEL_ROLLBACK: &[u8] = b"rollback\0";
    pub const SEL_SAVE: &[u8] = b"save:\0";
    pub const SEL_REFRESH_ALL_OBJECTS: &[u8] = b"refreshAllObjects\0";
    pub const SEL_LOCK: &[u8] = b"lock\0";
    pub const SEL_UNLOCK: &[u8] = b"unlock\0";
    pub const SEL_TRY_LOCK: &[u8] = b"tryLock\0";
    pub const SEL_SHOULD_HANDLE_INACCESSIBLE_FAULT: &[u8] = b"shouldHandleInaccessibleFault:forObjectID:triggeredByProperty:\0";
    pub const SEL_OBTAIN_PERMANENT_I_DS_FOR_OBJECTS: &[u8] = b"obtainPermanentIDsForObjects:error:\0";
    pub const SEL_MERGE_CHANGES_FROM_CONTEXT_DID_SAVE_NOTIFICATION: &[u8] = b"mergeChangesFromContextDidSaveNotification:\0";
    pub const SEL_MERGE_CHANGES_FROM_REMOTE_CONTEXT_SAVE: &[u8] = b"mergeChangesFromRemoteContextSave:intoContexts:\0";
    pub const SEL_SET_QUERY_GENERATION_FROM_TOKEN: &[u8] = b"setQueryGenerationFromToken:error:\0";
}

// ── NSManagedObject (0 methods, 0 properties) ──
pub mod n_s_managed_object {
}

// ── NSFetchRequest (2 methods, 5 properties) ──
pub mod n_s_fetch_request {
    pub const SEL_ENTITY: &[u8] = b"entity\0";
    pub const SEL_SET_ENTITY: &[u8] = b"setEntity:\0";
    pub const SEL_PREDICATE: &[u8] = b"predicate\0";
    pub const SEL_SET_PREDICATE: &[u8] = b"setPredicate:\0";
    pub const SEL_SORT_DESCRIPTORS: &[u8] = b"sortDescriptors\0";
    pub const SEL_SET_SORT_DESCRIPTORS: &[u8] = b"setSortDescriptors:\0";
    pub const SEL_FETCH_LIMIT: &[u8] = b"fetchLimit\0";
    pub const SEL_SET_FETCH_LIMIT: &[u8] = b"setFetchLimit:\0";
    pub const SEL_AFFECTED_STORES: &[u8] = b"affectedStores\0";
    pub const SEL_SET_AFFECTED_STORES: &[u8] = b"setAffectedStores:\0";
    pub const SEL_FETCH_REQUEST_WITH_ENTITY_NAME: &[u8] = b"fetchRequestWithEntityName:\0";
    pub const SEL_EXECUTE: &[u8] = b"execute:\0";
}

// ── NSEntityDescription (4 methods, 12 properties) ──
pub mod n_s_entity_description {
    pub const SEL_MANAGED_OBJECT_CLASS_NAME: &[u8] = b"managedObjectClassName\0";
    pub const SEL_SET_MANAGED_OBJECT_CLASS_NAME: &[u8] = b"setManagedObjectClassName:\0";
    pub const SEL_ABSTRACT: &[u8] = b"abstract\0";
    pub const SEL_SET_ABSTRACT: &[u8] = b"setAbstract:\0";
    pub const SEL_SUBENTITIES_BY_NAME: &[u8] = b"subentitiesByName\0";
    pub const SEL_SET_SUBENTITIES_BY_NAME: &[u8] = b"setSubentitiesByName:\0";
    pub const SEL_SUBENTITIES: &[u8] = b"subentities\0";
    pub const SEL_SET_SUBENTITIES: &[u8] = b"setSubentities:\0";
    pub const SEL_SUPERENTITY: &[u8] = b"superentity\0";
    pub const SEL_SET_SUPERENTITY: &[u8] = b"setSuperentity:\0";
    pub const SEL_PROPERTIES_BY_NAME: &[u8] = b"propertiesByName\0";
    pub const SEL_SET_PROPERTIES_BY_NAME: &[u8] = b"setPropertiesByName:\0";
    pub const SEL_PROPERTIES: &[u8] = b"properties\0";
    pub const SEL_SET_PROPERTIES: &[u8] = b"setProperties:\0";
    pub const SEL_USER_INFO: &[u8] = b"userInfo\0";
    pub const SEL_SET_USER_INFO: &[u8] = b"setUserInfo:\0";
    pub const SEL_ATTRIBUTES_BY_NAME: &[u8] = b"attributesByName\0";
    pub const SEL_SET_ATTRIBUTES_BY_NAME: &[u8] = b"setAttributesByName:\0";
    pub const SEL_RELATIONSHIPS_BY_NAME: &[u8] = b"relationshipsByName\0";
    pub const SEL_SET_RELATIONSHIPS_BY_NAME: &[u8] = b"setRelationshipsByName:\0";
    pub const SEL_ENTITY_FOR_NAME: &[u8] = b"entityForName:inManagedObjectContext:\0";
    pub const SEL_INSERT_NEW_OBJECT_FOR_ENTITY_FOR_NAME: &[u8] = b"insertNewObjectForEntityForName:inManagedObjectContext:\0";
    pub const SEL_RELATIONSHIPS_WITH_DESTINATION_ENTITY: &[u8] = b"relationshipsWithDestinationEntity:\0";
    pub const SEL_IS_KIND_OF_ENTITY: &[u8] = b"isKindOfEntity:\0";
}

// ── NSFetchedResultsController (6 methods, 8 properties) ──
pub mod n_s_fetched_results_controller {
    pub const SEL_FETCH_REQUEST: &[u8] = b"fetchRequest\0";
    pub const SEL_SET_FETCH_REQUEST: &[u8] = b"setFetchRequest:\0";
    pub const SEL_MANAGED_OBJECT_CONTEXT: &[u8] = b"managedObjectContext\0";
    pub const SEL_SET_MANAGED_OBJECT_CONTEXT: &[u8] = b"setManagedObjectContext:\0";
    pub const SEL_SECTION_NAME_KEY_PATH: &[u8] = b"sectionNameKeyPath\0";
    pub const SEL_SET_SECTION_NAME_KEY_PATH: &[u8] = b"setSectionNameKeyPath:\0";
    pub const SEL_CACHE_NAME: &[u8] = b"cacheName\0";
    pub const SEL_SET_CACHE_NAME: &[u8] = b"setCacheName:\0";
    pub const SEL_DELEGATE: &[u8] = b"delegate\0";
    pub const SEL_SET_DELEGATE: &[u8] = b"setDelegate:\0";
    pub const SEL_FETCHED_OBJECTS: &[u8] = b"fetchedObjects\0";
    pub const SEL_SET_FETCHED_OBJECTS: &[u8] = b"setFetchedObjects:\0";
    pub const SEL_SECTION_INDEX_TITLES: &[u8] = b"sectionIndexTitles\0";
    pub const SEL_SET_SECTION_INDEX_TITLES: &[u8] = b"setSectionIndexTitles:\0";
    pub const SEL_SECTIONS: &[u8] = b"sections\0";
    pub const SEL_SET_SECTIONS: &[u8] = b"setSections:\0";
    pub const SEL_PERFORM_FETCH: &[u8] = b"performFetch:\0";
    pub const SEL_DELETE_CACHE_WITH_NAME: &[u8] = b"deleteCacheWithName:\0";
    pub const SEL_OBJECT_AT_INDEX_PATH: &[u8] = b"objectAtIndexPath:\0";
    pub const SEL_INDEX_PATH_FOR_OBJECT: &[u8] = b"indexPathForObject:\0";
    pub const SEL_SECTION_INDEX_TITLE_FOR_SECTION_NAME: &[u8] = b"sectionIndexTitleForSectionName:\0";
    pub const SEL_SECTION_FOR_SECTION_INDEX_TITLE: &[u8] = b"sectionForSectionIndexTitle:atIndex:\0";
}

// ── NSPersistentStoreDescription (3 methods, 6 properties) ──
pub mod n_s_persistent_store_description {
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_CONFIGURATION: &[u8] = b"configuration\0";
    pub const SEL_SET_CONFIGURATION: &[u8] = b"setConfiguration:\0";
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:\0";
    pub const SEL_OPTIONS: &[u8] = b"options\0";
    pub const SEL_SET_OPTIONS: &[u8] = b"setOptions:\0";
    pub const SEL_READ_ONLY: &[u8] = b"readOnly\0";
    pub const SEL_SET_READ_ONLY: &[u8] = b"setReadOnly:\0";
    pub const SEL_SQLITE_PRAGMAS: &[u8] = b"sqlitePragmas\0";
    pub const SEL_SET_SQLITE_PRAGMAS: &[u8] = b"setSqlitePragmas:\0";
    pub const SEL_PERSISTENT_STORE_DESCRIPTION_WITH_U_R_L: &[u8] = b"persistentStoreDescriptionWithURL:\0";
    pub const SEL_SET_OPTION: &[u8] = b"setOption:forKey:\0";
    pub const SEL_SET_VALUE: &[u8] = b"setValue:forPragmaNamed:\0";
}

// ── NSManagedObjectModel (11 methods, 4 properties) ──
pub mod n_s_managed_object_model {
    pub const SEL_ENTITIES_BY_NAME: &[u8] = b"entitiesByName\0";
    pub const SEL_SET_ENTITIES_BY_NAME: &[u8] = b"setEntitiesByName:\0";
    pub const SEL_ENTITIES: &[u8] = b"entities\0";
    pub const SEL_SET_ENTITIES: &[u8] = b"setEntities:\0";
    pub const SEL_CONFIGURATIONS: &[u8] = b"configurations\0";
    pub const SEL_SET_CONFIGURATIONS: &[u8] = b"setConfigurations:\0";
    pub const SEL_LOCALIZATION_DICTIONARY: &[u8] = b"localizationDictionary\0";
    pub const SEL_SET_LOCALIZATION_DICTIONARY: &[u8] = b"setLocalizationDictionary:\0";
    pub const SEL_MERGED_MODEL_FROM_BUNDLES: &[u8] = b"mergedModelFromBundles:\0";
    pub const SEL_MODEL_BY_MERGING_MODELS: &[u8] = b"modelByMergingModels:\0";
    pub const SEL_ENTITIES_FOR_CONFIGURATION: &[u8] = b"entitiesForConfiguration:\0";
    pub const SEL_SET_FETCH_REQUEST_TEMPLATE: &[u8] = b"setFetchRequestTemplate:forName:\0";
    pub const SEL_FETCH_REQUEST_TEMPLATE_FOR_NAME: &[u8] = b"fetchRequestTemplateForName:\0";
    pub const SEL_FETCH_REQUEST_FROM_TEMPLATE_WITH_NAME: &[u8] = b"fetchRequestFromTemplateWithName:substitutionVariables:\0";
    pub const SEL_IS_CONFIGURATION: &[u8] = b"isConfiguration:compatibleWithStoreMetadata:\0";
    pub const SEL_CHECKSUMS_FOR_VERSIONED_MODEL_AT_U_R_L: &[u8] = b"checksumsForVersionedModelAtURL:error:\0";
}

// Total: 159 selector constants
