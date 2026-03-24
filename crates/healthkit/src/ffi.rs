//! ObjC selector constants for HealthKit.
#![allow(dead_code)]

// ── HKHealthStore (23 methods, 0 properties) ──
pub mod h_k_health_store {
    pub const CLASS: &[u8] = b"HKHealthStore\0";
    pub const SEL_IS_HEALTH_DATA_AVAILABLE: &[u8] = b"isHealthDataAvailable\0";
    pub const SEL_SUPPORTS_HEALTH_RECORDS: &[u8] = b"supportsHealthRecords\0";
    pub const SEL_AUTHORIZATION_STATUS_FOR_TYPE: &[u8] = b"authorizationStatusForType:\0";
    pub const SEL_EARLIEST_PERMITTED_SAMPLE_DATE: &[u8] = b"earliestPermittedSampleDate\0";
    pub const SEL_EXECUTE_QUERY: &[u8] = b"executeQuery:\0";
    pub const SEL_STOP_QUERY: &[u8] = b"stopQuery:\0";
    pub const SEL_DATE_OF_BIRTH_WITH_ERROR: &[u8] = b"dateOfBirthWithError:\0";
    pub const SEL_DATE_OF_BIRTH_COMPONENTS_WITH_ERROR: &[u8] = b"dateOfBirthComponentsWithError:\0";
    pub const SEL_BIOLOGICAL_SEX_WITH_ERROR: &[u8] = b"biologicalSexWithError:\0";
    pub const SEL_BLOOD_TYPE_WITH_ERROR: &[u8] = b"bloodTypeWithError:\0";
    pub const SEL_FITZPATRICK_SKIN_TYPE_WITH_ERROR: &[u8] = b"fitzpatrickSkinTypeWithError:\0";
    pub const SEL_WHEELCHAIR_USE_WITH_ERROR: &[u8] = b"wheelchairUseWithError:\0";
    pub const SEL_ACTIVITY_MOVE_MODE_WITH_ERROR: &[u8] = b"activityMoveModeWithError:\0";
}

// ── HKQuantityType (1 methods, 1 properties) ──
pub mod h_k_quantity_type {
    pub const CLASS: &[u8] = b"HKQuantityType\0";
    pub const SEL_AGGREGATION_STYLE: &[u8] = b"aggregationStyle\0";
    pub const SEL_SET_AGGREGATION_STYLE: &[u8] = b"setAggregationStyle:\0";
    pub const SEL_IS_COMPATIBLE_WITH_UNIT: &[u8] = b"isCompatibleWithUnit:\0";
}

// ── HKCategoryType (0 methods, 0 properties) ──
pub mod h_k_category_type {
    pub const CLASS: &[u8] = b"HKCategoryType\0";
}

// ── HKUnit (8 methods, 1 properties) ──
pub mod h_k_unit {
    pub const CLASS: &[u8] = b"HKUnit\0";
    pub const SEL_UNIT_STRING: &[u8] = b"unitString\0";
    pub const SEL_SET_UNIT_STRING: &[u8] = b"setUnitString:\0";
    pub const SEL_UNIT_FROM_STRING: &[u8] = b"unitFromString:\0";
    pub const SEL_UNIT_FROM_MASS_FORMATTER_UNIT: &[u8] = b"unitFromMassFormatterUnit:\0";
    pub const SEL_MASS_FORMATTER_UNIT_FROM_UNIT: &[u8] = b"massFormatterUnitFromUnit:\0";
    pub const SEL_UNIT_FROM_LENGTH_FORMATTER_UNIT: &[u8] = b"unitFromLengthFormatterUnit:\0";
    pub const SEL_LENGTH_FORMATTER_UNIT_FROM_UNIT: &[u8] = b"lengthFormatterUnitFromUnit:\0";
    pub const SEL_UNIT_FROM_ENERGY_FORMATTER_UNIT: &[u8] = b"unitFromEnergyFormatterUnit:\0";
    pub const SEL_ENERGY_FORMATTER_UNIT_FROM_UNIT: &[u8] = b"energyFormatterUnitFromUnit:\0";
    pub const SEL_IS_NULL: &[u8] = b"isNull\0";
}

// ── HKQuantity (4 methods, 0 properties) ──
pub mod h_k_quantity {
    pub const CLASS: &[u8] = b"HKQuantity\0";
    pub const SEL_QUANTITY_WITH_UNIT: &[u8] = b"quantityWithUnit:doubleValue:\0";
    pub const SEL_IS_COMPATIBLE_WITH_UNIT: &[u8] = b"isCompatibleWithUnit:\0";
    pub const SEL_DOUBLE_VALUE_FOR_UNIT: &[u8] = b"doubleValueForUnit:\0";
    pub const SEL_COMPARE: &[u8] = b"compare:\0";
}

// ── HKQuantitySample (3 methods, 2 properties) ──
pub mod h_k_quantity_sample {
    pub const CLASS: &[u8] = b"HKQuantitySample\0";
    pub const SEL_QUANTITY_TYPE: &[u8] = b"quantityType\0";
    pub const SEL_SET_QUANTITY_TYPE: &[u8] = b"setQuantityType:\0";
    pub const SEL_QUANTITY: &[u8] = b"quantity\0";
    pub const SEL_SET_QUANTITY: &[u8] = b"setQuantity:\0";
    pub const SEL_QUANTITY_SAMPLE_WITH_TYPE: &[u8] = b"quantitySampleWithType:quantity:startDate:endDate:\0";
}

// ── HKStatisticsQuery (0 methods, 0 properties) ──
pub mod h_k_statistics_query {
    pub const CLASS: &[u8] = b"HKStatisticsQuery\0";
}

// ── HKSampleQuery (0 methods, 2 properties) ──
pub mod h_k_sample_query {
    pub const CLASS: &[u8] = b"HKSampleQuery\0";
    pub const SEL_LIMIT: &[u8] = b"limit\0";
    pub const SEL_SET_LIMIT: &[u8] = b"setLimit:\0";
    pub const SEL_SORT_DESCRIPTORS: &[u8] = b"sortDescriptors\0";
    pub const SEL_SET_SORT_DESCRIPTORS: &[u8] = b"setSortDescriptors:\0";
}

// ── HKWorkout (8 methods, 3 properties) ──
pub mod h_k_workout {
    pub const CLASS: &[u8] = b"HKWorkout\0";
    pub const SEL_WORKOUT_ACTIVITY_TYPE: &[u8] = b"workoutActivityType\0";
    pub const SEL_SET_WORKOUT_ACTIVITY_TYPE: &[u8] = b"setWorkoutActivityType:\0";
    pub const SEL_WORKOUT_EVENTS: &[u8] = b"workoutEvents\0";
    pub const SEL_SET_WORKOUT_EVENTS: &[u8] = b"setWorkoutEvents:\0";
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_STATISTICS_FOR_TYPE: &[u8] = b"statisticsForType:\0";
    pub const SEL_WORKOUT_WITH_ACTIVITY_TYPE: &[u8] = b"workoutWithActivityType:startDate:endDate:\0";
}

// Total: 57 selector constants
