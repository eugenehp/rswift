//! ObjC selector constants for ModelIO.
#![allow(dead_code)]

// ── MDLAsset (12 methods, 8 properties) ──
pub mod m_d_l_asset {
    pub const CLASS: &[u8] = b"MDLAsset\0";
    pub const SEL_BOUNDING_BOX: &[u8] = b"boundingBox\0";
    pub const SEL_SET_BOUNDING_BOX: &[u8] = b"setBoundingBox:\0";
    pub const SEL_FRAME_INTERVAL: &[u8] = b"frameInterval\0";
    pub const SEL_SET_FRAME_INTERVAL: &[u8] = b"setFrameInterval:\0";
    pub const SEL_START_TIME: &[u8] = b"startTime\0";
    pub const SEL_SET_START_TIME: &[u8] = b"setStartTime:\0";
    pub const SEL_END_TIME: &[u8] = b"endTime\0";
    pub const SEL_SET_END_TIME: &[u8] = b"setEndTime:\0";
    pub const SEL_U_R_L: &[u8] = b"URL\0";
    pub const SEL_SET_U_R_L: &[u8] = b"setURL:\0";
    pub const SEL_BUFFER_ALLOCATOR: &[u8] = b"bufferAllocator\0";
    pub const SEL_SET_BUFFER_ALLOCATOR: &[u8] = b"setBufferAllocator:\0";
    pub const SEL_VERTEX_DESCRIPTOR: &[u8] = b"vertexDescriptor\0";
    pub const SEL_SET_VERTEX_DESCRIPTOR: &[u8] = b"setVertexDescriptor:\0";
    pub const SEL_COUNT: &[u8] = b"count\0";
    pub const SEL_SET_COUNT: &[u8] = b"setCount:\0";
    pub const SEL_EXPORT_ASSET_TO_U_R_L: &[u8] = b"exportAssetToURL:\0";
    pub const SEL_OBJECT_AT_PATH: &[u8] = b"objectAtPath:\0";
    pub const SEL_CAN_IMPORT_FILE_EXTENSION: &[u8] = b"canImportFileExtension:\0";
    pub const SEL_CAN_EXPORT_FILE_EXTENSION: &[u8] = b"canExportFileExtension:\0";
    pub const SEL_CHILD_OBJECTS_OF_CLASS: &[u8] = b"childObjectsOfClass:\0";
    pub const SEL_LOAD_TEXTURES: &[u8] = b"loadTextures\0";
    pub const SEL_BOUNDING_BOX_AT_TIME: &[u8] = b"boundingBoxAtTime:\0";
    pub const SEL_ADD_OBJECT: &[u8] = b"addObject:\0";
    pub const SEL_REMOVE_OBJECT: &[u8] = b"removeObject:\0";
    pub const SEL_OBJECT_AT_INDEXED_SUBSCRIPT: &[u8] = b"objectAtIndexedSubscript:\0";
    pub const SEL_OBJECT_AT_INDEX: &[u8] = b"objectAtIndex:\0";
}

// ── MDLMesh (2 methods, 6 properties) ──
pub mod m_d_l_mesh {
    pub const SEL_VERTEX_COUNT: &[u8] = b"vertexCount\0";
    pub const SEL_SET_VERTEX_COUNT: &[u8] = b"setVertexCount:\0";
    pub const SEL_VERTEX_BUFFERS: &[u8] = b"vertexBuffers\0";
    pub const SEL_SET_VERTEX_BUFFERS: &[u8] = b"setVertexBuffers:\0";
    pub const SEL_SUBMESHES: &[u8] = b"submeshes\0";
    pub const SEL_SET_SUBMESHES: &[u8] = b"setSubmeshes:\0";
    pub const SEL_ALLOCATOR: &[u8] = b"allocator\0";
    pub const SEL_SET_ALLOCATOR: &[u8] = b"setAllocator:\0";
    pub const SEL_VERTEX_ATTRIBUTE_DATA_FOR_ATTRIBUTE_NAMED: &[u8] = b"vertexAttributeDataForAttributeNamed:\0";
}

// ── MDLSubmesh (1 methods, 7 properties) ──
pub mod m_d_l_submesh {
    pub const SEL_INDEX_BUFFER: &[u8] = b"indexBuffer\0";
    pub const SEL_SET_INDEX_BUFFER: &[u8] = b"setIndexBuffer:\0";
    pub const SEL_INDEX_COUNT: &[u8] = b"indexCount\0";
    pub const SEL_SET_INDEX_COUNT: &[u8] = b"setIndexCount:\0";
    pub const SEL_INDEX_TYPE: &[u8] = b"indexType\0";
    pub const SEL_SET_INDEX_TYPE: &[u8] = b"setIndexType:\0";
    pub const SEL_GEOMETRY_TYPE: &[u8] = b"geometryType\0";
    pub const SEL_SET_GEOMETRY_TYPE: &[u8] = b"setGeometryType:\0";
    pub const SEL_MATERIAL: &[u8] = b"material\0";
    pub const SEL_SET_MATERIAL: &[u8] = b"setMaterial:\0";
    pub const SEL_TOPOLOGY: &[u8] = b"topology\0";
    pub const SEL_SET_TOPOLOGY: &[u8] = b"setTopology:\0";
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_INDEX_BUFFER_AS_INDEX_TYPE: &[u8] = b"indexBufferAsIndexType:\0";
}

// ── MDLVertexDescriptor (6 methods, 2 properties) ──
pub mod m_d_l_vertex_descriptor {
    pub const SEL_ATTRIBUTES: &[u8] = b"attributes\0";
    pub const SEL_SET_ATTRIBUTES: &[u8] = b"setAttributes:\0";
    pub const SEL_LAYOUTS: &[u8] = b"layouts\0";
    pub const SEL_SET_LAYOUTS: &[u8] = b"setLayouts:\0";
    pub const SEL_ATTRIBUTE_NAMED: &[u8] = b"attributeNamed:\0";
    pub const SEL_ADD_OR_REPLACE_ATTRIBUTE: &[u8] = b"addOrReplaceAttribute:\0";
    pub const SEL_REMOVE_ATTRIBUTE_NAMED: &[u8] = b"removeAttributeNamed:\0";
    pub const SEL_RESET: &[u8] = b"reset\0";
    pub const SEL_SET_PACKED_STRIDES: &[u8] = b"setPackedStrides\0";
    pub const SEL_SET_PACKED_OFFSETS: &[u8] = b"setPackedOffsets\0";
}

// ── MDLCamera (5 methods, 23 properties) ──
pub mod m_d_l_camera {
    pub const SEL_PROJECTION_MATRIX: &[u8] = b"projectionMatrix\0";
    pub const SEL_SET_PROJECTION_MATRIX: &[u8] = b"setProjectionMatrix:\0";
    pub const SEL_PROJECTION: &[u8] = b"projection\0";
    pub const SEL_SET_PROJECTION: &[u8] = b"setProjection:\0";
    pub const SEL_NEAR_VISIBILITY_DISTANCE: &[u8] = b"nearVisibilityDistance\0";
    pub const SEL_SET_NEAR_VISIBILITY_DISTANCE: &[u8] = b"setNearVisibilityDistance:\0";
    pub const SEL_FAR_VISIBILITY_DISTANCE: &[u8] = b"farVisibilityDistance\0";
    pub const SEL_SET_FAR_VISIBILITY_DISTANCE: &[u8] = b"setFarVisibilityDistance:\0";
    pub const SEL_WORLD_TO_METERS_CONVERSION_SCALE: &[u8] = b"worldToMetersConversionScale\0";
    pub const SEL_SET_WORLD_TO_METERS_CONVERSION_SCALE: &[u8] = b"setWorldToMetersConversionScale:\0";
    pub const SEL_BARREL_DISTORTION: &[u8] = b"barrelDistortion\0";
    pub const SEL_SET_BARREL_DISTORTION: &[u8] = b"setBarrelDistortion:\0";
    pub const SEL_FISHEYE_DISTORTION: &[u8] = b"fisheyeDistortion\0";
    pub const SEL_SET_FISHEYE_DISTORTION: &[u8] = b"setFisheyeDistortion:\0";
    pub const SEL_OPTICAL_VIGNETTING: &[u8] = b"opticalVignetting\0";
    pub const SEL_SET_OPTICAL_VIGNETTING: &[u8] = b"setOpticalVignetting:\0";
    pub const SEL_CHROMATIC_ABERRATION: &[u8] = b"chromaticAberration\0";
    pub const SEL_SET_CHROMATIC_ABERRATION: &[u8] = b"setChromaticAberration:\0";
    pub const SEL_FOCAL_LENGTH: &[u8] = b"focalLength\0";
    pub const SEL_SET_FOCAL_LENGTH: &[u8] = b"setFocalLength:\0";
    pub const SEL_FOCUS_DISTANCE: &[u8] = b"focusDistance\0";
    pub const SEL_SET_FOCUS_DISTANCE: &[u8] = b"setFocusDistance:\0";
    pub const SEL_FIELD_OF_VIEW: &[u8] = b"fieldOfView\0";
    pub const SEL_SET_FIELD_OF_VIEW: &[u8] = b"setFieldOfView:\0";
    pub const SEL_F_STOP: &[u8] = b"fStop\0";
    pub const SEL_SET_F_STOP: &[u8] = b"setFStop:\0";
    pub const SEL_APERTURE_BLADE_COUNT: &[u8] = b"apertureBladeCount\0";
    pub const SEL_SET_APERTURE_BLADE_COUNT: &[u8] = b"setApertureBladeCount:\0";
    pub const SEL_MAXIMUM_CIRCLE_OF_CONFUSION: &[u8] = b"maximumCircleOfConfusion\0";
    pub const SEL_SET_MAXIMUM_CIRCLE_OF_CONFUSION: &[u8] = b"setMaximumCircleOfConfusion:\0";
    pub const SEL_SHUTTER_OPEN_INTERVAL: &[u8] = b"shutterOpenInterval\0";
    pub const SEL_SET_SHUTTER_OPEN_INTERVAL: &[u8] = b"setShutterOpenInterval:\0";
    pub const SEL_SENSOR_VERTICAL_APERTURE: &[u8] = b"sensorVerticalAperture\0";
    pub const SEL_SET_SENSOR_VERTICAL_APERTURE: &[u8] = b"setSensorVerticalAperture:\0";
    pub const SEL_SENSOR_ASPECT: &[u8] = b"sensorAspect\0";
    pub const SEL_SET_SENSOR_ASPECT: &[u8] = b"setSensorAspect:\0";
    pub const SEL_SENSOR_ENLARGEMENT: &[u8] = b"sensorEnlargement\0";
    pub const SEL_SET_SENSOR_ENLARGEMENT: &[u8] = b"setSensorEnlargement:\0";
    pub const SEL_SENSOR_SHIFT: &[u8] = b"sensorShift\0";
    pub const SEL_SET_SENSOR_SHIFT: &[u8] = b"setSensorShift:\0";
    pub const SEL_FLASH: &[u8] = b"flash\0";
    pub const SEL_SET_FLASH: &[u8] = b"setFlash:\0";
    pub const SEL_EXPOSURE_COMPRESSION: &[u8] = b"exposureCompression\0";
    pub const SEL_SET_EXPOSURE_COMPRESSION: &[u8] = b"setExposureCompression:\0";
    pub const SEL_EXPOSURE: &[u8] = b"exposure\0";
    pub const SEL_SET_EXPOSURE: &[u8] = b"setExposure:\0";
    pub const SEL_FRAME_BOUNDING_BOX: &[u8] = b"frameBoundingBox:setNearAndFar:\0";
    pub const SEL_LOOK_AT: &[u8] = b"lookAt:\0";
    pub const SEL_RAY_TO: &[u8] = b"rayTo:forViewPort:\0";
    pub const SEL_BOKEH_KERNEL_WITH_SIZE: &[u8] = b"bokehKernelWithSize:\0";
}

// ── MDLLight (2 methods, 2 properties) ──
pub mod m_d_l_light {
    pub const SEL_LIGHT_TYPE: &[u8] = b"lightType\0";
    pub const SEL_SET_LIGHT_TYPE: &[u8] = b"setLightType:\0";
    pub const SEL_COLOR_SPACE: &[u8] = b"colorSpace\0";
    pub const SEL_SET_COLOR_SPACE: &[u8] = b"setColorSpace:\0";
    pub const SEL_IRRADIANCE_AT_POINT: &[u8] = b"irradianceAtPoint:\0";
}

// ── MDLMaterial (10 methods, 5 properties) ──
pub mod m_d_l_material {
    pub const SEL_SCATTERING_FUNCTION: &[u8] = b"scatteringFunction\0";
    pub const SEL_SET_SCATTERING_FUNCTION: &[u8] = b"setScatteringFunction:\0";
    pub const SEL_BASE_MATERIAL: &[u8] = b"baseMaterial\0";
    pub const SEL_SET_BASE_MATERIAL: &[u8] = b"setBaseMaterial:\0";
    pub const SEL_MATERIAL_FACE: &[u8] = b"materialFace\0";
    pub const SEL_SET_MATERIAL_FACE: &[u8] = b"setMaterialFace:\0";
    pub const SEL_SET_PROPERTY: &[u8] = b"setProperty:\0";
    pub const SEL_REMOVE_PROPERTY: &[u8] = b"removeProperty:\0";
    pub const SEL_PROPERTY_NAMED: &[u8] = b"propertyNamed:\0";
    pub const SEL_PROPERTY_WITH_SEMANTIC: &[u8] = b"propertyWithSemantic:\0";
    pub const SEL_PROPERTIES_WITH_SEMANTIC: &[u8] = b"propertiesWithSemantic:\0";
    pub const SEL_REMOVE_ALL_PROPERTIES: &[u8] = b"removeAllProperties\0";
    pub const SEL_RESOLVE_TEXTURES_WITH_RESOLVER: &[u8] = b"resolveTexturesWithResolver:\0";
    pub const SEL_LOAD_TEXTURES_USING_RESOLVER: &[u8] = b"loadTexturesUsingResolver:\0";
    pub const SEL_OBJECT_FOR_KEYED_SUBSCRIPT: &[u8] = b"objectForKeyedSubscript:\0";
}

// Total: 140 selector constants
