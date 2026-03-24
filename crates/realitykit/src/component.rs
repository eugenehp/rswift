//! Component documentation and re-exports.
//!
//! RealityKit uses an Entity-Component-System (ECS) architecture.
//! Components are attached/removed through methods on [`Entity`]:
//!
//! ## Core / Transform
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `TransformComponent` | `set_position`, `set_rotation`, `set_scale`, `set_transform`, `look_at` |
//! | `ModelComponent` | `Entity::model(…)`, `set_mesh`, `set_material` |
//!
//! ## Physics & Collision
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `CollisionComponent` | `set_collision_box/sphere/capsule/convex_hull`, `set_collision_mode` |
//! | `PhysicsBodyComponent` | `set_physics_body`, `set_physics_body_mode`, `set_physics_body_mass` |
//! | `PhysicsMotionComponent` | `set_linear_velocity`, `set_angular_velocity` |
//! | `PhysicsSimulationComponent` | `set_physics_simulation` |
//! | `CharacterControllerComponent` | `set_character_controller` (macOS 13+, iOS 16+) |
//!
//! ## Visual / Rendering
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `OpacityComponent` | `set_opacity` |
//! | `GroundingShadowComponent` | `set_grounding_shadow` |
//! | `ModelDebugOptionsComponent` | `set_debug_options(mode)` |
//! | `BillboardComponent` | `set_billboard` (macOS 26+ / iOS 26+ / visionOS 3+, feature `macos-26`) |
//!
//! ## Lighting
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `PerspectiveCameraComponent` | `Entity::perspective_camera(…)` |
//! | `OrthographicCameraComponent` | `Entity::orthographic_camera(…)` |
//! | `PointLightComponent` | `Entity::point_light(…)` |
//! | `DirectionalLightComponent` | `Entity::directional_light(…)` |
//! | `SpotLightComponent` | `Entity::spot_light(…)` |
//! | `ImageBasedLightComponent` | `Entity::image_based_light(env, exp)` |
//! | `ImageBasedLightReceiverComponent` | `set_ibl_receiver` |
//! | `EnvironmentLightingConfigurationComponent` | `set_environment_lighting_weight` |
//!
//! ## Audio
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `SpatialAudioComponent` | `set_spatial_audio` |
//! | `AmbientAudioComponent` | `set_ambient_audio` |
//! | `ChannelAudioComponent` | `set_channel_audio` |
//! | `AudioLibraryComponent` | `audio_library_add(name, resource)` |
//!
//! ## Video
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `VideoPlayerComponent` | `set_video_player(url)`, `video_player_play/pause/seek` (macOS 14+) |
//!
//! ## Text / Content
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `TextComponent` | `set_text_component(text, font_size)` (macOS 26+, iOS 26+) |
//!
//! ## Particles & Effects
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `ParticleEmitterComponent` | `set_particle_emitter` (macOS 14+, iOS 17+) |
//! | `ForceEffectComponent` | `set_constant/radial/vortex/drag_force_effect` (macOS 26+ / iOS 26+ / visionOS 3+) |
//!
//! ## Anchoring & AR
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `AnchoringComponent` | `set_anchoring(target, matrix)` |
//!
//! ## Networking / Multiplayer
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `NetworkSynchronizationComponent` | `set_network_sync` |
//! | `SynchronizationComponent` | `set_synchronization()`, `synchronization_is_owner()` |
//!
//! ## Accessibility
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `AccessibilityComponent` | `set_accessibility(label, hidden)` (macOS 13+, iOS 16+) |
//!
//! ## visionOS
//! | Swift Component | Rust method(s) |
//! |---|---|
//! | `InputTargetComponent` | `set_input_target` |
//! | `HoverEffectComponent` | `set_hover_effect` |
//! | `PortalComponent` | `Entity::portal(world)` |
//! | `WorldComponent` | `set_world_component` |
//! | `SurroundingsEffectComponent` | `set_surroundings_effect` (visionOS 2+) |

pub use crate::scene::RenderOptions;
