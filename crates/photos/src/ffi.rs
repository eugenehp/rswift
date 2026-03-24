//! ObjC selector constants for Photos.
#![allow(dead_code)]

// ── PHAsset (2 methods, 0 properties) ──
pub mod p_h_asset {
    pub const CLASS: &[u8] = b"PHAsset\0";
    pub const SEL_CANCEL_CONTENT_EDITING_INPUT_REQUEST: &[u8] = b"cancelContentEditingInputRequest:\0";
}

// ── PHAssetCollection (8 methods, 7 properties) ──
pub mod p_h_asset_collection {
    pub const CLASS: &[u8] = b"PHAssetCollection\0";
    pub const SEL_ASSET_COLLECTION_TYPE: &[u8] = b"assetCollectionType\0";
    pub const SEL_SET_ASSET_COLLECTION_TYPE: &[u8] = b"setAssetCollectionType:\0";
    pub const SEL_ASSET_COLLECTION_SUBTYPE: &[u8] = b"assetCollectionSubtype\0";
    pub const SEL_SET_ASSET_COLLECTION_SUBTYPE: &[u8] = b"setAssetCollectionSubtype:\0";
    pub const SEL_ESTIMATED_ASSET_COUNT: &[u8] = b"estimatedAssetCount\0";
    pub const SEL_SET_ESTIMATED_ASSET_COUNT: &[u8] = b"setEstimatedAssetCount:\0";
    pub const SEL_START_DATE: &[u8] = b"startDate\0";
    pub const SEL_SET_START_DATE: &[u8] = b"setStartDate:\0";
    pub const SEL_END_DATE: &[u8] = b"endDate\0";
    pub const SEL_SET_END_DATE: &[u8] = b"setEndDate:\0";
    pub const SEL_APPROXIMATE_LOCATION: &[u8] = b"approximateLocation\0";
    pub const SEL_SET_APPROXIMATE_LOCATION: &[u8] = b"setApproximateLocation:\0";
    pub const SEL_LOCALIZED_LOCATION_NAMES: &[u8] = b"localizedLocationNames\0";
    pub const SEL_SET_LOCALIZED_LOCATION_NAMES: &[u8] = b"setLocalizedLocationNames:\0";
    pub const SEL_FETCH_ASSET_COLLECTIONS_WITH_LOCAL_IDENTIFIERS: &[u8] = b"fetchAssetCollectionsWithLocalIdentifiers:options:\0";
    pub const SEL_FETCH_ASSET_COLLECTIONS_WITH_TYPE: &[u8] = b"fetchAssetCollectionsWithType:subtype:options:\0";
    pub const SEL_FETCH_ASSET_COLLECTIONS_CONTAINING_ASSET: &[u8] = b"fetchAssetCollectionsContainingAsset:withType:options:\0";
    pub const SEL_FETCH_ASSET_COLLECTIONS_WITH_A_L_ASSET_GROUP_U_R_LS: &[u8] = b"fetchAssetCollectionsWithALAssetGroupURLs:options:\0";
    pub const SEL_FETCH_MOMENTS_IN_MOMENT_LIST: &[u8] = b"fetchMomentsInMomentList:options:\0";
    pub const SEL_FETCH_MOMENTS_WITH_OPTIONS: &[u8] = b"fetchMomentsWithOptions:\0";
    pub const SEL_TRANSIENT_ASSET_COLLECTION_WITH_ASSETS: &[u8] = b"transientAssetCollectionWithAssets:title:\0";
    pub const SEL_TRANSIENT_ASSET_COLLECTION_WITH_ASSET_FETCH_RESULT: &[u8] = b"transientAssetCollectionWithAssetFetchResult:title:\0";
}

// ── PHFetchOptions (0 methods, 4 properties) ──
pub mod p_h_fetch_options {
    pub const CLASS: &[u8] = b"PHFetchOptions\0";
    pub const SEL_PREDICATE: &[u8] = b"predicate\0";
    pub const SEL_SET_PREDICATE: &[u8] = b"setPredicate:\0";
    pub const SEL_SORT_DESCRIPTORS: &[u8] = b"sortDescriptors\0";
    pub const SEL_SET_SORT_DESCRIPTORS: &[u8] = b"setSortDescriptors:\0";
    pub const SEL_INCLUDE_HIDDEN_ASSETS: &[u8] = b"includeHiddenAssets\0";
    pub const SEL_SET_INCLUDE_HIDDEN_ASSETS: &[u8] = b"setIncludeHiddenAssets:\0";
    pub const SEL_WANTS_INCREMENTAL_CHANGE_DETAILS: &[u8] = b"wantsIncrementalChangeDetails\0";
    pub const SEL_SET_WANTS_INCREMENTAL_CHANGE_DETAILS: &[u8] = b"setWantsIncrementalChangeDetails:\0";
}

// ── PHImageManager (9 methods, 0 properties) ──
pub mod p_h_image_manager {
    pub const CLASS: &[u8] = b"PHImageManager\0";
    pub const SEL_DEFAULT_MANAGER: &[u8] = b"defaultManager\0";
    pub const SEL_CANCEL_IMAGE_REQUEST: &[u8] = b"cancelImageRequest:\0";
}

// ── PHPhotoLibrary (4 methods, 0 properties) ──
pub mod p_h_photo_library {
    pub const CLASS: &[u8] = b"PHPhotoLibrary\0";
    pub const SEL_LOCAL_IDENTIFIER_MAPPINGS_FOR_CLOUD_IDENTIFIERS: &[u8] = b"localIdentifierMappingsForCloudIdentifiers:\0";
    pub const SEL_CLOUD_IDENTIFIER_MAPPINGS_FOR_LOCAL_IDENTIFIERS: &[u8] = b"cloudIdentifierMappingsForLocalIdentifiers:\0";
    pub const SEL_LOCAL_IDENTIFIERS_FOR_CLOUD_IDENTIFIERS: &[u8] = b"localIdentifiersForCloudIdentifiers:\0";
    pub const SEL_CLOUD_IDENTIFIERS_FOR_LOCAL_IDENTIFIERS: &[u8] = b"cloudIdentifiersForLocalIdentifiers:\0";
}

// ── PHCollectionList (7 methods, 5 properties) ──
pub mod p_h_collection_list {
    pub const CLASS: &[u8] = b"PHCollectionList\0";
    pub const SEL_COLLECTION_LIST_TYPE: &[u8] = b"collectionListType\0";
    pub const SEL_SET_COLLECTION_LIST_TYPE: &[u8] = b"setCollectionListType:\0";
    pub const SEL_COLLECTION_LIST_SUBTYPE: &[u8] = b"collectionListSubtype\0";
    pub const SEL_SET_COLLECTION_LIST_SUBTYPE: &[u8] = b"setCollectionListSubtype:\0";
    pub const SEL_START_DATE: &[u8] = b"startDate\0";
    pub const SEL_SET_START_DATE: &[u8] = b"setStartDate:\0";
    pub const SEL_END_DATE: &[u8] = b"endDate\0";
    pub const SEL_SET_END_DATE: &[u8] = b"setEndDate:\0";
    pub const SEL_LOCALIZED_LOCATION_NAMES: &[u8] = b"localizedLocationNames\0";
    pub const SEL_SET_LOCALIZED_LOCATION_NAMES: &[u8] = b"setLocalizedLocationNames:\0";
    pub const SEL_FETCH_COLLECTION_LISTS_CONTAINING_COLLECTION: &[u8] = b"fetchCollectionListsContainingCollection:options:\0";
    pub const SEL_FETCH_COLLECTION_LISTS_WITH_LOCAL_IDENTIFIERS: &[u8] = b"fetchCollectionListsWithLocalIdentifiers:options:\0";
    pub const SEL_FETCH_COLLECTION_LISTS_WITH_TYPE: &[u8] = b"fetchCollectionListsWithType:subtype:options:\0";
    pub const SEL_FETCH_MOMENT_LISTS_WITH_SUBTYPE: &[u8] = b"fetchMomentListsWithSubtype:containingMoment:options:\0";
    pub const SEL_TRANSIENT_COLLECTION_LIST_WITH_COLLECTIONS: &[u8] = b"transientCollectionListWithCollections:title:\0";
    pub const SEL_TRANSIENT_COLLECTION_LIST_WITH_COLLECTIONS_FETCH_RESULT: &[u8] = b"transientCollectionListWithCollectionsFetchResult:title:\0";
}

// Total: 61 selector constants
