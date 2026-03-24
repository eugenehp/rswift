//! ObjC selector constants for SceneKit.
#![allow(dead_code)]

// ── SCNScene (3 methods, 0 properties) ──
pub mod s_c_n_scene {
    pub const CLASS: &[u8] = b"SCNScene\0";
    pub const SEL_ADD_PARTICLE_SYSTEM: &[u8] = b"addParticleSystem:withTransform:\0";
    pub const SEL_REMOVE_ALL_PARTICLE_SYSTEMS: &[u8] = b"removeAllParticleSystems\0";
    pub const SEL_REMOVE_PARTICLE_SYSTEM: &[u8] = b"removeParticleSystem:\0";
}

// ── SCNNode (3 methods, 0 properties) ──
pub mod s_c_n_node {
}

// ── SCNGeometry (9 methods, 4 properties) ──
pub mod s_c_n_geometry {
    pub const SEL_NAME: &[u8] = b"name\0";
    pub const SEL_SET_NAME: &[u8] = b"setName:\0";
    pub const SEL_MATERIALS: &[u8] = b"materials\0";
    pub const SEL_SET_MATERIALS: &[u8] = b"setMaterials:\0";
    pub const SEL_FIRST_MATERIAL: &[u8] = b"firstMaterial\0";
    pub const SEL_SET_FIRST_MATERIAL: &[u8] = b"setFirstMaterial:\0";
    pub const SEL_GEOMETRY_ELEMENT_COUNT: &[u8] = b"geometryElementCount\0";
    pub const SEL_SET_GEOMETRY_ELEMENT_COUNT: &[u8] = b"setGeometryElementCount:\0";
    pub const SEL_GEOMETRY: &[u8] = b"geometry\0";
    pub const SEL_INSERT_MATERIAL: &[u8] = b"insertMaterial:atIndex:\0";
    pub const SEL_REMOVE_MATERIAL_AT_INDEX: &[u8] = b"removeMaterialAtIndex:\0";
    pub const SEL_REPLACE_MATERIAL_AT_INDEX: &[u8] = b"replaceMaterialAtIndex:withMaterial:\0";
    pub const SEL_MATERIAL_WITH_NAME: &[u8] = b"materialWithName:\0";
    pub const SEL_GEOMETRY_WITH_SOURCES: &[u8] = b"geometryWithSources:elements:\0";
    pub const SEL_GEOMETRY_SOURCES_FOR_SEMANTIC: &[u8] = b"geometrySourcesForSemantic:\0";
    pub const SEL_GEOMETRY_ELEMENT_AT_INDEX: &[u8] = b"geometryElementAtIndex:\0";
}

// ── SCNLight (1 methods, 6 properties) ──
pub mod s_c_n_light {
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_COLOR: &[u8] = b"color\0";
    pub const SEL_SET_COLOR: &[u8] = b"setColor:\0";
    pub const SEL_CASTS_SHADOW: &[u8] = b"castsShadow\0";
    pub const SEL_SET_CASTS_SHADOW: &[u8] = b"setCastsShadow:\0";
    pub const SEL_SHADOW_COLOR: &[u8] = b"shadowColor\0";
    pub const SEL_SET_SHADOW_COLOR: &[u8] = b"setShadowColor:\0";
    pub const SEL_SHADOW_RADIUS: &[u8] = b"shadowRadius\0";
    pub const SEL_SET_SHADOW_RADIUS: &[u8] = b"setShadowRadius:\0";
    pub const SEL_LIGHT: &[u8] = b"light\0";
}

// ── SCNCamera (1 methods, 0 properties) ──
pub mod s_c_n_camera {
    pub const SEL_CAMERA_WITH_M_D_L_CAMERA: &[u8] = b"cameraWithMDLCamera:\0";
}

// ── SCNMaterial (1 methods, 0 properties) ──
pub mod s_c_n_material {
    pub const SEL_MATERIAL_WITH_M_D_L_MATERIAL: &[u8] = b"materialWithMDLMaterial:\0";
}

// ── SCNAction (28 methods, 4 properties) ──
pub mod s_c_n_action {
    pub const SEL_DURATION: &[u8] = b"duration\0";
    pub const SEL_SET_DURATION: &[u8] = b"setDuration:\0";
    pub const SEL_TIMING_MODE: &[u8] = b"timingMode\0";
    pub const SEL_SET_TIMING_MODE: &[u8] = b"setTimingMode:\0";
    pub const SEL_TIMING_FUNCTION: &[u8] = b"timingFunction\0";
    pub const SEL_SET_TIMING_FUNCTION: &[u8] = b"setTimingFunction:\0";
    pub const SEL_SPEED: &[u8] = b"speed\0";
    pub const SEL_SET_SPEED: &[u8] = b"setSpeed:\0";
    pub const SEL_REVERSED_ACTION: &[u8] = b"reversedAction\0";
    pub const SEL_MOVE_BY_X: &[u8] = b"moveByX:y:z:duration:\0";
    pub const SEL_MOVE_BY: &[u8] = b"moveBy:duration:\0";
    pub const SEL_MOVE_TO: &[u8] = b"moveTo:duration:\0";
    pub const SEL_ROTATE_BY_X: &[u8] = b"rotateByX:y:z:duration:\0";
    pub const SEL_ROTATE_TO_X: &[u8] = b"rotateToX:y:z:duration:\0";
    pub const SEL_ROTATE_BY_ANGLE: &[u8] = b"rotateByAngle:aroundAxis:duration:\0";
    pub const SEL_ROTATE_TO_AXIS_ANGLE: &[u8] = b"rotateToAxisAngle:duration:\0";
    pub const SEL_SCALE_BY: &[u8] = b"scaleBy:duration:\0";
    pub const SEL_SCALE_TO: &[u8] = b"scaleTo:duration:\0";
    pub const SEL_SEQUENCE: &[u8] = b"sequence:\0";
    pub const SEL_GROUP: &[u8] = b"group:\0";
    pub const SEL_REPEAT_ACTION: &[u8] = b"repeatAction:count:\0";
    pub const SEL_REPEAT_ACTION_FOREVER: &[u8] = b"repeatActionForever:\0";
    pub const SEL_FADE_IN_WITH_DURATION: &[u8] = b"fadeInWithDuration:\0";
    pub const SEL_FADE_OUT_WITH_DURATION: &[u8] = b"fadeOutWithDuration:\0";
    pub const SEL_FADE_OPACITY_BY: &[u8] = b"fadeOpacityBy:duration:\0";
    pub const SEL_FADE_OPACITY_TO: &[u8] = b"fadeOpacityTo:duration:\0";
    pub const SEL_HIDE: &[u8] = b"hide\0";
    pub const SEL_UNHIDE: &[u8] = b"unhide\0";
    pub const SEL_WAIT_FOR_DURATION: &[u8] = b"waitForDuration:\0";
    pub const SEL_REMOVE_FROM_PARENT_NODE: &[u8] = b"removeFromParentNode\0";
    pub const SEL_JAVA_SCRIPT_ACTION_WITH_SCRIPT: &[u8] = b"javaScriptActionWithScript:duration:\0";
    pub const SEL_PLAY_AUDIO_SOURCE: &[u8] = b"playAudioSource:waitForCompletion:\0";
}

// ── SCNPhysicsBody (10 methods, 17 properties) ──
pub mod s_c_n_physics_body {
    pub const SEL_MASS: &[u8] = b"mass\0";
    pub const SEL_SET_MASS: &[u8] = b"setMass:\0";
    pub const SEL_CHARGE: &[u8] = b"charge\0";
    pub const SEL_SET_CHARGE: &[u8] = b"setCharge:\0";
    pub const SEL_FRICTION: &[u8] = b"friction\0";
    pub const SEL_SET_FRICTION: &[u8] = b"setFriction:\0";
    pub const SEL_RESTITUTION: &[u8] = b"restitution\0";
    pub const SEL_SET_RESTITUTION: &[u8] = b"setRestitution:\0";
    pub const SEL_ROLLING_FRICTION: &[u8] = b"rollingFriction\0";
    pub const SEL_SET_ROLLING_FRICTION: &[u8] = b"setRollingFriction:\0";
    pub const SEL_PHYSICS_SHAPE: &[u8] = b"physicsShape\0";
    pub const SEL_SET_PHYSICS_SHAPE: &[u8] = b"setPhysicsShape:\0";
    pub const SEL_IS_RESTING: &[u8] = b"isResting\0";
    pub const SEL_SET_IS_RESTING: &[u8] = b"setIsResting:\0";
    pub const SEL_ALLOWS_RESTING: &[u8] = b"allowsResting\0";
    pub const SEL_SET_ALLOWS_RESTING: &[u8] = b"setAllowsResting:\0";
    pub const SEL_VELOCITY: &[u8] = b"velocity\0";
    pub const SEL_SET_VELOCITY: &[u8] = b"setVelocity:\0";
    pub const SEL_ANGULAR_VELOCITY: &[u8] = b"angularVelocity\0";
    pub const SEL_SET_ANGULAR_VELOCITY: &[u8] = b"setAngularVelocity:\0";
    pub const SEL_DAMPING: &[u8] = b"damping\0";
    pub const SEL_SET_DAMPING: &[u8] = b"setDamping:\0";
    pub const SEL_ANGULAR_DAMPING: &[u8] = b"angularDamping\0";
    pub const SEL_SET_ANGULAR_DAMPING: &[u8] = b"setAngularDamping:\0";
    pub const SEL_VELOCITY_FACTOR: &[u8] = b"velocityFactor\0";
    pub const SEL_SET_VELOCITY_FACTOR: &[u8] = b"setVelocityFactor:\0";
    pub const SEL_ANGULAR_VELOCITY_FACTOR: &[u8] = b"angularVelocityFactor\0";
    pub const SEL_SET_ANGULAR_VELOCITY_FACTOR: &[u8] = b"setAngularVelocityFactor:\0";
    pub const SEL_CATEGORY_BIT_MASK: &[u8] = b"categoryBitMask\0";
    pub const SEL_SET_CATEGORY_BIT_MASK: &[u8] = b"setCategoryBitMask:\0";
    pub const SEL_COLLISION_BIT_MASK: &[u8] = b"collisionBitMask\0";
    pub const SEL_SET_COLLISION_BIT_MASK: &[u8] = b"setCollisionBitMask:\0";
    pub const SEL_STATIC_BODY: &[u8] = b"staticBody\0";
    pub const SEL_DYNAMIC_BODY: &[u8] = b"dynamicBody\0";
    pub const SEL_KINEMATIC_BODY: &[u8] = b"kinematicBody\0";
    pub const SEL_BODY_WITH_TYPE: &[u8] = b"bodyWithType:shape:\0";
    pub const SEL_APPLY_FORCE: &[u8] = b"applyForce:impulse:\0";
    pub const SEL_APPLY_TORQUE: &[u8] = b"applyTorque:impulse:\0";
    pub const SEL_CLEAR_ALL_FORCES: &[u8] = b"clearAllForces\0";
    pub const SEL_RESET_TRANSFORM: &[u8] = b"resetTransform\0";
    pub const SEL_SET_RESTING: &[u8] = b"setResting:\0";
}

// ── SCNPhysicsWorld (8 methods, 5 properties) ──
pub mod s_c_n_physics_world {
    pub const SEL_GRAVITY: &[u8] = b"gravity\0";
    pub const SEL_SET_GRAVITY: &[u8] = b"setGravity:\0";
    pub const SEL_TIME_STEP: &[u8] = b"timeStep\0";
    pub const SEL_SET_TIME_STEP: &[u8] = b"setTimeStep:\0";
    pub const SEL_CONTACT_DELEGATE: &[u8] = b"contactDelegate\0";
    pub const SEL_SET_CONTACT_DELEGATE: &[u8] = b"setContactDelegate:\0";
    pub const SEL_ALL_BEHAVIORS: &[u8] = b"allBehaviors\0";
    pub const SEL_SET_ALL_BEHAVIORS: &[u8] = b"setAllBehaviors:\0";
    pub const SEL_ADD_BEHAVIOR: &[u8] = b"addBehavior:\0";
    pub const SEL_REMOVE_BEHAVIOR: &[u8] = b"removeBehavior:\0";
    pub const SEL_REMOVE_ALL_BEHAVIORS: &[u8] = b"removeAllBehaviors\0";
    pub const SEL_RAY_TEST_WITH_SEGMENT_FROM_POINT: &[u8] = b"rayTestWithSegmentFromPoint:toPoint:options:\0";
    pub const SEL_CONTACT_TEST_BETWEEN_BODY: &[u8] = b"contactTestBetweenBody:andBody:options:\0";
    pub const SEL_CONTACT_TEST_WITH_BODY: &[u8] = b"contactTestWithBody:options:\0";
    pub const SEL_CONVEX_SWEEP_TEST_WITH_SHAPE: &[u8] = b"convexSweepTestWithShape:fromTransform:toTransform:options:\0";
    pub const SEL_UPDATE_COLLISION_PAIRS: &[u8] = b"updateCollisionPairs\0";
}

// ── SCNView (4 methods, 4 properties) ──
pub mod s_c_n_view {
    pub const SEL_SCENE: &[u8] = b"scene\0";
    pub const SEL_SET_SCENE: &[u8] = b"setScene:\0";
    pub const SEL_RENDERS_CONTINUOUSLY: &[u8] = b"rendersContinuously\0";
    pub const SEL_SET_RENDERS_CONTINUOUSLY: &[u8] = b"setRendersContinuously:\0";
    pub const SEL_BACKGROUND_COLOR: &[u8] = b"backgroundColor\0";
    pub const SEL_SET_BACKGROUND_COLOR: &[u8] = b"setBackgroundColor:\0";
    pub const SEL_ALLOWS_CAMERA_CONTROL: &[u8] = b"allowsCameraControl\0";
    pub const SEL_SET_ALLOWS_CAMERA_CONTROL: &[u8] = b"setAllowsCameraControl:\0";
    pub const SEL_SNAPSHOT: &[u8] = b"snapshot\0";
    pub const SEL_PLAY: &[u8] = b"play:\0";
    pub const SEL_PAUSE: &[u8] = b"pause:\0";
    pub const SEL_STOP: &[u8] = b"stop:\0";
}

// ── SCNHitTestResult (1 methods, 8 properties) ──
pub mod s_c_n_hit_test_result {
    pub const SEL_NODE: &[u8] = b"node\0";
    pub const SEL_SET_NODE: &[u8] = b"setNode:\0";
    pub const SEL_GEOMETRY_INDEX: &[u8] = b"geometryIndex\0";
    pub const SEL_SET_GEOMETRY_INDEX: &[u8] = b"setGeometryIndex:\0";
    pub const SEL_FACE_INDEX: &[u8] = b"faceIndex\0";
    pub const SEL_SET_FACE_INDEX: &[u8] = b"setFaceIndex:\0";
    pub const SEL_LOCAL_COORDINATES: &[u8] = b"localCoordinates\0";
    pub const SEL_SET_LOCAL_COORDINATES: &[u8] = b"setLocalCoordinates:\0";
    pub const SEL_WORLD_COORDINATES: &[u8] = b"worldCoordinates\0";
    pub const SEL_SET_WORLD_COORDINATES: &[u8] = b"setWorldCoordinates:\0";
    pub const SEL_LOCAL_NORMAL: &[u8] = b"localNormal\0";
    pub const SEL_SET_LOCAL_NORMAL: &[u8] = b"setLocalNormal:\0";
    pub const SEL_WORLD_NORMAL: &[u8] = b"worldNormal\0";
    pub const SEL_SET_WORLD_NORMAL: &[u8] = b"setWorldNormal:\0";
    pub const SEL_MODEL_TRANSFORM: &[u8] = b"modelTransform\0";
    pub const SEL_SET_MODEL_TRANSFORM: &[u8] = b"setModelTransform:\0";
    pub const SEL_TEXTURE_COORDINATES_WITH_MAPPING_CHANNEL: &[u8] = b"textureCoordinatesWithMappingChannel:\0";
}

// Total: 161 selector constants
