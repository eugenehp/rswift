//! Full RealityKit bindings for Rust — no setup required.
//!
//! `build.rs` automatically compiles `swift/bridge.swift` and links
//! `librealitykit_bridge.dylib` via [`realitykit_sys`].  Just add the crate
//! as a dependency and start using the API — no `load_auto()` init call is
//! needed anymore.
//!
//! ## Quick start
//!
//! ```ignore
//! use realitykit::prelude::*;
//!
//! // Meshes & materials
//! let mesh = MeshResource::sphere(0.4);
//! let mat  = Material::pbr()
//!     .base_color(0.2, 0.6, 1.0, 1.0)
//!     .roughness(0.3)
//!     .build();
//!
//! // Entities
//! let ball  = Entity::model(&mesh, &mat).at(0.0, 1.0, 0.0);
//! let light = Entity::point_light(1.0, 0.95, 0.8, 1200.0, 15.0).at(2.0, 4.0, 2.0);
//! let floor = Entity::model(&MeshResource::plane(8.0, 8.0),
//!                           &Material::simple(0.4, 0.4, 0.4));
//!
//! // Physics
//! ball.set_physics_body(PhysicsBodyMode::Dynamic, 1.0, 0.5, 0.4);
//! ball.set_collision_sphere(0.4, 0.0, 0.0, 0.0);
//! ball.apply_linear_impulse(0.0, 4.0, 0.0);
//!
//! // Scene (macOS)
//! let scene  = Scene::new(800.0, 600.0);
//! let anchor = Entity::anchor(0.0, 0.0, -3.0);
//! anchor.add_child(&floor).add_child(&ball).add_child(&light);
//! scene.add_anchor(&anchor);
//! ```
//!
//! ## Features
//! - `visionos` — `InputTargetComponent`, `HoverEffectComponent`,
//!   `PortalComponent`, `WorldComponent`, `SurroundingsEffect`, hand/head anchors.
//! - `macos-26` — `BillboardComponent`, `ForceEffectComponent` (WWDC 2025).
//!
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

pub mod entity;
pub mod mesh;
pub mod material;
pub mod physics;
pub mod resource;
pub mod animation;
pub mod audio;
pub mod scene;
pub mod component;
pub mod particle;

#[cfg(any(feature = "visionos", target_os = "xros", doc))]
pub mod visionos;

mod tests;

pub use entity::{Entity, AnchorType, PlaneType};
pub use resource::EnvironmentResource;
pub use mesh::MeshResource;
pub use material::{Material, MaterialBuilder, TextureResource};
pub use physics::{PhysicsBodyMode, CollisionMode};
pub use animation::{AnimationController, TimingFunction, AnimationRepeatMode,
                    AnimationResource, AnimationDefinition};
pub use audio::{AudioResource, AudioGroupResource, AudioController, AudioInputMode};
pub use scene::{Scene, RenderOptions, RaycastHit, Subscription};

/// Backward-compatibility no-op.
///
/// The Swift bridge is now compiled automatically by `build.rs` and linked
/// at compile time — no runtime init step is needed.  Existing call sites
/// that call `realitykit::load_auto()` continue to compile and work.
pub fn load_auto() -> Result<(), String> {
    realitykit_sys::load_auto()
}

/// Backward-compatibility no-op — see [`load_auto`].
pub fn load(path: &str) -> Result<(), String> {
    realitykit_sys::load(path)
}

pub mod prelude {
    pub use crate::{
        load_auto, load,
        Entity, AnchorType, PlaneType,
        EnvironmentResource,
        MeshResource,
        Material, MaterialBuilder, TextureResource,
        PhysicsBodyMode, CollisionMode,
        AnimationController, TimingFunction, AnimationRepeatMode,
        AnimationResource, AnimationDefinition,
        AudioResource, AudioGroupResource, AudioController, AudioInputMode,
        Scene, RenderOptions, RaycastHit, Subscription,
    };
    #[cfg(any(feature = "visionos", target_os = "xros"))]
    pub use crate::visionos;
}


