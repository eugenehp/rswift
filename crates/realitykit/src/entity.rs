//! Entity — the fundamental scene-graph node in RealityKit.


use crate::animation::{AnimationController, TimingFunction};
use crate::audio::{AudioResource, AudioController};
use crate::mesh::MeshResource;
use crate::material::Material;
use crate::physics::{PhysicsBodyMode, CollisionMode};
use crate::resource::EnvironmentResource;
use core::ffi::c_void;

/// Opaque handle to any RealityKit entity (Entity, ModelEntity, AnchorEntity,
/// PointLight, SpotLight, DirectionalLight, camera entity, etc.).
pub struct Entity {
    pub(crate) ptr: *mut c_void,
}

// SAFETY: RealityKit entities are ObjC objects with thread-safe retain/release.
unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}

impl Clone for Entity {
    /// Increment Swift retain count; both handles refer to the same entity.
    fn clone(&self) -> Self {
        unsafe { realitykit_sys::rk_retain(self.ptr) };
        Entity { ptr: self.ptr }
    }
}
impl Drop for Entity {
    fn drop(&mut self) {
        unsafe { realitykit_sys::rk_release(self.ptr) };
    }
}

// ─── Plane classification for anchors ────────────────────────────────────────

/// Which surfaces to anchor to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaneType {
    Any, Horizontal, Vertical,
}

/// How the anchor tracks the real world.
#[derive(Clone, Debug)]
pub enum AnchorType {
    /// World-space position (x, y, z in meters).
    World(f32, f32, f32),
    /// Any horizontal or vertical plane.
    Plane(PlaneType),
    /// Image marker (AR Resource Group, image name).
    Image { group: String, name: String },
    /// Face tracking (iOS / visionOS only).
    Face,
    /// Full-body tracking (iOS only).
    Body,
    /// Head anchor (visionOS 3+).
    Head,
    /// Left-hand palm anchor (visionOS 3+).
    LeftHand,
    /// Right-hand palm anchor (visionOS 3+).
    RightHand,
}

// ─── Constructors ─────────────────────────────────────────────────────────────

impl Entity {
    /// Plain empty entity.
    pub fn new() -> Self {
        let ptr = unsafe { realitykit_sys::rk_entity_new() };
        Entity { ptr }
    }

    /// Deep clone (recursive).
    pub fn clone_recursive(&self) -> Self {
        let ptr = unsafe { realitykit_sys::rk_entity_clone(self.ptr) };
        Entity { ptr }
    }

    /// `ModelEntity` with one mesh and one material.
    pub fn model(mesh: &MeshResource, material: &Material) -> Self {
        let ptr = unsafe { realitykit_sys::rk_model_entity_new(mesh.ptr, material.ptr) };
        Entity { ptr }
    }

    /// `ModelEntity` with one mesh and multiple materials (one per submesh).
    pub fn model_multi(mesh: &MeshResource, materials: &[&Material]) -> Self {
        let handles: Vec<*mut c_void> = materials.iter().map(|m| m.ptr).collect();
        let ptr = unsafe { realitykit_sys::rk_model_entity_new_many(mesh.ptr, handles.as_ptr(), handles.len()) };
        Entity { ptr }
    }

    /// `AnchorEntity` variant.
    pub fn anchor(x: f32, y: f32, z: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_anchor_entity_world(x, y, z) };
        Entity { ptr }
    }

    /// `AnchorEntity` at world origin.
    pub fn anchor_origin() -> Self {
        let ptr = unsafe { realitykit_sys::rk_anchor_entity_new() };
        Entity { ptr }
    }

    /// Typed anchor (plane, image, face, body, head, hand).
    pub fn anchor_typed(kind: AnchorType) -> Self {
        let ptr = match kind {
            AnchorType::World(x, y, z) => unsafe { realitykit_sys::rk_anchor_entity_world(x, y, z) },
            AnchorType::Plane(PlaneType::Any)        => unsafe { realitykit_sys::rk_anchor_entity_plane_any() },
            AnchorType::Plane(PlaneType::Horizontal) => unsafe { realitykit_sys::rk_anchor_entity_plane_horizontal() },
            AnchorType::Plane(PlaneType::Vertical)   => unsafe { realitykit_sys::rk_anchor_entity_plane_vertical() },
            AnchorType::Image { ref group, ref name } => unsafe {
                realitykit_sys::rk_anchor_entity_image(
                    group.as_ptr(), group.len(),
                    name.as_ptr(),  name.len(),
                )
            },
            AnchorType::Face      => {
                let p = unsafe { realitykit_sys::rk_anchor_entity_face() };
                if p.is_null() { unsafe { realitykit_sys::rk_anchor_entity_new() } } else { p }
            }
            AnchorType::Body      => {
                let p = unsafe { realitykit_sys::rk_anchor_entity_body() };
                if p.is_null() { unsafe { realitykit_sys::rk_anchor_entity_new() } } else { p }
            }
            AnchorType::Head       => unsafe { realitykit_sys::rk_anchor_entity_head() },
            AnchorType::LeftHand   => unsafe { realitykit_sys::rk_anchor_entity_left_hand() },
            AnchorType::RightHand  => unsafe { realitykit_sys::rk_anchor_entity_right_hand() },
        };
        Entity { ptr }
    }

    // ── Lights ────────────────────────────────────────────────────────────

    /// Create a point light entity.
    pub fn point_light(r: f32, g: f32, b: f32, intensity: f32, attenuation_radius: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_point_light(r, g, b, intensity, attenuation_radius) };
        Entity { ptr }
    }

    /// Create a directional light entity.
    pub fn directional_light(r: f32, g: f32, b: f32, intensity: f32, casts_shadow: bool) -> Self {
        let ptr = unsafe { realitykit_sys::rk_directional_light(r, g, b, intensity, casts_shadow) };
        Entity { ptr }
    }

    /// Create a spot light entity.
    pub fn spot_light(r: f32, g: f32, b: f32, intensity: f32, inner_angle: f32, outer_angle: f32, attenuation_radius: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_spot_light(r, g, b, intensity, inner_angle, outer_angle, attenuation_radius) };
        Entity { ptr }
    }

    /// Create an image-based light entity from a loaded [`EnvironmentResource`].
    pub fn image_based_light(env: &EnvironmentResource, intensity_exponent: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_image_based_light_new(env.ptr, intensity_exponent) };
        Entity { ptr }
    }

    /// Create a perspective camera entity.
    pub fn perspective_camera(fov_degrees: f32, near: f32, far: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_perspective_camera_new(fov_degrees, near, far) };
        Entity { ptr }
    }

    /// Create an orthographic camera entity.
    pub fn orthographic_camera(scale: f32, near: f32, far: f32) -> Self {
        let ptr = unsafe { realitykit_sys::rk_orthographic_camera_new(scale, near, far) };
        Entity { ptr }
    }

    // ── Asset loading ─────────────────────────────────────────────────────

    /// Synchronously load an entity from a file path (USDZ, Reality, etc.).
    pub fn load(path: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe { realitykit_sys::rk_entity_load_from_file(path.as_ptr(), path.len(), err.as_mut_ptr(), err.len()) };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(Entity { ptr: p })
        }
    }

    /// Load a named entity from the main bundle.
    pub fn load_named(name: &str) -> Result<Self, String> {
        let mut err = [0u8; 2048];
        let p = unsafe { realitykit_sys::rk_entity_load_named(name.as_ptr(), name.len(), err.as_mut_ptr(), err.len()) };
        if p.is_null() {
            let n = err.iter().position(|&b| b == 0).unwrap_or(err.len());
            Err(String::from_utf8_lossy(&err[..n]).into())
        } else {
            Ok(Entity { ptr: p })
        }
    }

    // ── visionOS portals ──────────────────────────────────────────────────

    /// Create a portal entity that shows the contents of a "world" entity.
    /// Requires visionOS.
    pub fn portal(world: &Entity) -> Self {
        let ptr = unsafe { realitykit_sys::rk_portal_entity_new(world.ptr) };
        Entity { ptr }
    }
}

impl Default for Entity {
    fn default() -> Self { Entity::new() }
}

// ─── Name & Hierarchy ─────────────────────────────────────────────────────────

impl Entity {
    /// Set the entity's display name.
    pub fn set_name(&self, name: &str) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_name(self.ptr, name.as_ptr(), name.len()) };
        self
    }

    /// Read the entity's name (up to 512 bytes).
    pub fn name(&self) -> String {
        let mut buf = [0u8; 512];
        let n = unsafe { realitykit_sys::rk_entity_get_name(self.ptr, buf.as_mut_ptr(), buf.len()) };
        String::from_utf8_lossy(&buf[..n]).into()
    }

    /// Add a child entity (retains child in RealityKit's scene graph).
    pub fn add_child(&self, child: &Entity) -> &Self {
        unsafe { realitykit_sys::rk_entity_add_child(self.ptr, child.ptr) };
        self
    }

    /// Remove a direct child.
    pub fn remove_child(&self, child: &Entity) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_child(self.ptr, child.ptr) };
        self
    }

    /// Detach from parent.
    pub fn remove_from_parent(&self) { unsafe { realitykit_sys::rk_entity_remove_from_parent(self.ptr) }; }

    /// Number of direct children.
    pub fn child_count(&self) -> usize { unsafe { realitykit_sys::rk_entity_child_count(self.ptr) } }

    /// Find a descendant by name.  Returns `None` if not found.
    pub fn find_child_named(&self, name: &str) -> Option<Entity> {
        let p = unsafe { realitykit_sys::rk_entity_find_child_named(self.ptr, name.as_ptr(), name.len(), true) };
        if p.is_null() { None } else { Some(Entity { ptr: p }) }
    }

    /// Enable or disable the entity (and its children).
    pub fn set_enabled(&self, v: bool) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_is_enabled(self.ptr, v) }; self
    }
    pub fn is_enabled(&self) -> bool { unsafe { realitykit_sys::rk_entity_is_enabled(self.ptr) } }
    pub fn is_enabled_in_hierarchy(&self) -> bool { unsafe { realitykit_sys::rk_entity_is_enabled_in_hierarchy(self.ptr) } }
}

// ─── Transform ────────────────────────────────────────────────────────────────

impl Entity {
    /// Set world-space position (meters).
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_position(self.ptr, x, y, z) }; self
    }
    /// Convenience: chain `.at(x,y,z)`.
    pub fn at(self, x: f32, y: f32, z: f32) -> Self {
        unsafe { realitykit_sys::rk_entity_set_position(self.ptr, x, y, z) }; self
    }
    pub fn position(&self) -> [f32; 3] {
        let mut v = [0f32; 3];
        unsafe { realitykit_sys::rk_entity_get_position(self.ptr, v.as_mut_ptr()) }; v
    }

    /// Set orientation as a quaternion `[x, y, z, w]`.
    pub fn set_rotation(&self, x: f32, y: f32, z: f32, w: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_rotation_quat(self.ptr, x, y, z, w) }; self
    }
    pub fn rotation(&self) -> [f32; 4] {
        let mut v = [0f32; 4];
        unsafe { realitykit_sys::rk_entity_get_rotation_quat(self.ptr, v.as_mut_ptr()) }; v
    }

    /// Set non-uniform scale.
    pub fn set_scale(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_scale(self.ptr, x, y, z) }; self
    }
    pub fn scale(&self) -> [f32; 3] {
        let mut v = [0f32; 3];
        unsafe { realitykit_sys::rk_entity_get_scale(self.ptr, v.as_mut_ptr()) }; v
    }
    pub fn set_uniform_scale(&self, s: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_uniform_scale(self.ptr, s) }; self
    }

    /// Set full transform in one call.
    /// `pos` = [x,y,z], `rot` = [x,y,z,w] quaternion, `scale` = [x,y,z].
    pub fn set_transform(&self, pos: [f32; 3], rot: [f32; 4], scale: [f32; 3]) -> &Self {
        unsafe {
            realitykit_sys::rk_entity_set_transform(
                self.ptr,
                pos[0], pos[1], pos[2],
                rot[0], rot[1], rot[2], rot[3],
                scale[0], scale[1], scale[2],
            )
        }; self
    }

    /// Returns `(position [x,y,z], rotation [x,y,z,w], scale [x,y,z])`.
    pub fn transform(&self) -> ([f32; 3], [f32; 4], [f32; 3]) {
        let mut p = [0f32; 3];
        let mut r = [0f32; 4];
        let mut s = [0f32; 3];
        unsafe { realitykit_sys::rk_entity_get_transform(self.ptr, p.as_mut_ptr(), r.as_mut_ptr(), s.as_mut_ptr()) };
        (p, r, s)
    }

    /// Orient the entity so it looks toward `target` with the given `up` vector.
    pub fn look_at(&self, target: [f32; 3], up: [f32; 3]) -> &Self {
        unsafe { realitykit_sys::rk_entity_look_at(self.ptr, target[0],target[1],target[2], up[0],up[1],up[2]) }; self
    }
}

// ─── ModelEntity operations ───────────────────────────────────────────────────

impl Entity {
    /// Replace the material in a slot (for entities created as ModelEntity).
    pub fn set_material(&self, material: &Material, slot: usize) -> &Self {
        unsafe { realitykit_sys::rk_model_entity_set_material(self.ptr, material.ptr, slot) }; self
    }
    pub fn material_count(&self) -> usize {
        unsafe { realitykit_sys::rk_model_entity_material_count(self.ptr) }
    }
    /// Replace the mesh (for ModelEntity).
    pub fn set_mesh(&self, mesh: &MeshResource) -> &Self {
        unsafe { realitykit_sys::rk_model_entity_set_mesh(self.ptr, mesh.ptr) }; self
    }
}

// ─── Lights (property setters on light entities) ──────────────────────────────

impl Entity {
    pub fn set_point_light_color(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { realitykit_sys::rk_point_light_set_color(self.ptr, r, g, b) }; self
    }
    pub fn set_point_light_intensity(&self, v: f32) -> &Self {
        unsafe { realitykit_sys::rk_point_light_set_intensity(self.ptr, v) }; self
    }
    pub fn set_point_light_attenuation_radius(&self, v: f32) -> &Self {
        unsafe { realitykit_sys::rk_point_light_set_attenuation_radius(self.ptr, v) }; self
    }

    pub fn set_directional_light_color(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { realitykit_sys::rk_directional_light_set_color(self.ptr, r, g, b) }; self
    }
    pub fn set_directional_light_intensity(&self, v: f32) -> &Self {
        unsafe { realitykit_sys::rk_directional_light_set_intensity(self.ptr, v) }; self
    }
    pub fn set_directional_light_casts_shadow(&self, v: bool) -> &Self {
        unsafe { realitykit_sys::rk_directional_light_set_casts_shadow(self.ptr, v) }; self
    }

    pub fn set_spot_light_angles(&self, inner: f32, outer: f32) -> &Self {
        unsafe { realitykit_sys::rk_spot_light_set_angles(self.ptr, inner, outer) }; self
    }
    pub fn set_spot_light_color(&self, r: f32, g: f32, b: f32) -> &Self {
        unsafe { realitykit_sys::rk_spot_light_set_color(self.ptr, r, g, b) }; self
    }
    pub fn set_spot_light_intensity(&self, v: f32) -> &Self {
        unsafe { realitykit_sys::rk_spot_light_set_intensity(self.ptr, v) }; self
    }

    /// Make this entity receive IBL lighting from an image-based light entity.
    pub fn set_ibl_receiver(&self, ibl_entity: &Entity) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_image_based_light_receiver(self.ptr, ibl_entity.ptr) }; self
    }
    pub fn remove_ibl_receiver(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_image_based_light_receiver(self.ptr) }; self
    }
    /// Control how strongly the scene IBL affects this entity (0 = none, 1 = full).
    pub fn set_environment_lighting_weight(&self, w: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_environment_lighting_weight(self.ptr, w) }; self
    }
    pub fn remove_environment_lighting_config(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_environment_lighting_config(self.ptr) }; self
    }
}

// ─── Camera operations ────────────────────────────────────────────────────────

impl Entity {
    pub fn set_perspective_fov(&self, fov_degrees: f32) -> &Self {
        unsafe { realitykit_sys::rk_perspective_camera_set_fov(self.ptr, fov_degrees) }; self
    }
    pub fn set_perspective_clip(&self, near: f32, far: f32) -> &Self {
        unsafe { realitykit_sys::rk_perspective_camera_set_clip(self.ptr, near, far) }; self
    }
    pub fn set_orthographic_scale(&self, scale: f32) -> &Self {
        unsafe { realitykit_sys::rk_orthographic_camera_set_scale(self.ptr, scale) }; self
    }
    pub fn remove_camera(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_camera(self.ptr) }; self
    }
}

// ─── Physics ──────────────────────────────────────────────────────────────────

impl Entity {
    /// Attach a `PhysicsBodyComponent`. `mode` controls static/dynamic/kinematic.
    pub fn set_physics_body(&self, mode: PhysicsBodyMode, mass: f32, friction: f32, restitution: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_body(self.ptr, mode as i32, mass, friction, restitution, 0.0, 0.0) }; self
    }
    /// Full physics body with damping parameters.
    pub fn set_physics_body_full(&self, mode: PhysicsBodyMode, mass: f32, friction: f32,
                                  restitution: f32, linear_damping: f32, angular_damping: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_body(self.ptr, mode as i32, mass, friction, restitution, linear_damping, angular_damping) }; self
    }
    pub fn set_physics_body_mode(&self, mode: PhysicsBodyMode) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_body_mode(self.ptr, mode as i32) }; self
    }
    pub fn set_physics_body_mass(&self, mass: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_body_mass(self.ptr, mass) }; self
    }
    pub fn set_physics_material(&self, friction: f32, restitution: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_body_material(self.ptr, friction, restitution) }; self
    }
    pub fn is_resting(&self) -> bool { unsafe { realitykit_sys::rk_entity_is_resting(self.ptr) } }
    pub fn remove_physics_body(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_physics_body(self.ptr) }; self
    }

    /// Box collision shape centred at `(ox, oy, oz)` offset from entity origin.
    pub fn set_collision_box(&self, w: f32, h: f32, d: f32, ox: f32, oy: f32, oz: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_collision_box(self.ptr, w, h, d, ox, oy, oz) }; self
    }
    pub fn set_collision_sphere(&self, radius: f32, ox: f32, oy: f32, oz: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_collision_sphere(self.ptr, radius, ox, oy, oz) }; self
    }
    pub fn set_collision_capsule(&self, height: f32, radius: f32, ox: f32, oy: f32, oz: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_collision_capsule(self.ptr, height, radius, ox, oy, oz) }; self
    }
    /// Generate convex-hull shapes from the entity's mesh.
    pub fn set_collision_convex_hull(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_collision_convex_hull(self.ptr) }; self
    }
    pub fn set_collision_mode(&self, mode: CollisionMode) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_collision_mode(self.ptr, mode as i32) }; self
    }
    pub fn remove_collision(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_collision(self.ptr) }; self
    }

    // Velocity
    pub fn set_linear_velocity(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_linear_velocity(self.ptr, x, y, z) }; self
    }
    pub fn linear_velocity(&self) -> [f32; 3] {
        let mut v = [0f32; 3];
        unsafe { realitykit_sys::rk_entity_get_linear_velocity(self.ptr, v.as_mut_ptr()) }; v
    }
    pub fn set_angular_velocity(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_angular_velocity(self.ptr, x, y, z) }; self
    }
    pub fn angular_velocity(&self) -> [f32; 3] {
        let mut v = [0f32; 3];
        unsafe { realitykit_sys::rk_entity_get_angular_velocity(self.ptr, v.as_mut_ptr()) }; v
    }

    // Forces (requires ModelEntity with PhysicsBodyComponent)
    pub fn apply_linear_impulse(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_apply_linear_impulse(self.ptr, x, y, z) }; self
    }
    pub fn apply_angular_impulse(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_apply_angular_impulse(self.ptr, x, y, z) }; self
    }
    /// Apply a force at a world-space position.
    pub fn add_force(&self, fx: f32, fy: f32, fz: f32, px: f32, py: f32, pz: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_add_force(self.ptr, fx, fy, fz, px, py, pz) }; self
    }
    pub fn add_torque(&self, x: f32, y: f32, z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_add_torque(self.ptr, x, y, z) }; self
    }
    pub fn reset_physics(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_reset_physics(self.ptr) }; self
    }

    /// Attach a `PhysicsSimulationComponent` — makes this entity a physics root
    /// with custom gravity.
    pub fn set_physics_simulation(&self, gravity_x: f32, gravity_y: f32, gravity_z: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_physics_simulation(self.ptr, gravity_x, gravity_y, gravity_z) }; self
    }
    pub fn remove_physics_simulation(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_physics_simulation(self.ptr) }; self
    }
}

// ─── Animation ────────────────────────────────────────────────────────────────

impl Entity {
    /// Smoothly move to a target transform.  Returns a controller.
    pub fn move_to(&self, pos: [f32; 3], rot: [f32; 4], scale: [f32; 3],
                   duration: f64, timing: TimingFunction) -> AnimationController {
        let ptr = unsafe {
            realitykit_sys::rk_entity_move_to(
                self.ptr,
                pos[0], pos[1], pos[2],
                rot[0], rot[1], rot[2], rot[3],
                scale[0], scale[1], scale[2],
                duration, timing as i32,
            )
        };
        AnimationController { ptr }
    }

    /// Number of embedded animations (from loaded USDZ / Reality files).
    pub fn animation_count(&self) -> usize {
        unsafe { realitykit_sys::rk_entity_animation_count(self.ptr) }
    }

    /// Play the animation at `index` in `availableAnimations`.
    pub fn play_animation(&self, index: usize, transition_duration: f64, starts_paused: bool) -> Option<AnimationController> {
        let p = unsafe { realitykit_sys::rk_entity_play_animation(self.ptr, index, transition_duration, starts_paused) };
        if p.is_null() { None } else { Some(AnimationController { ptr: p }) }
    }

    /// Stop all running animations immediately.
    pub fn stop_all_animations(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_stop_all_animations(self.ptr) }; self
    }
}

// ─── Audio ────────────────────────────────────────────────────────────────────

impl Entity {
    pub fn play_audio(&self, resource: &AudioResource) -> AudioController {
        let ptr = unsafe { realitykit_sys::rk_entity_play_audio(self.ptr, resource.ptr) };
        AudioController { ptr }
    }
    /// Play an `AudioFileGroupResource` on this entity.
    pub fn play_audio_group(&self, group: &crate::audio::AudioGroupResource) -> AudioController {
        let ptr = unsafe { realitykit_sys::rk_entity_play_audio_group(self.ptr, group.ptr) };
        AudioController { ptr }
    }
    pub fn stop_all_audio(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_stop_all_audio(self.ptr) }; self
    }
    pub fn set_spatial_audio(&self, direct_db: f64, reverb_db: f64) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_spatial_audio(self.ptr, direct_db, reverb_db) }; self
    }
    pub fn remove_spatial_audio(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_spatial_audio(self.ptr) }; self
    }
    pub fn set_ambient_audio(&self, gain_db: f64) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_ambient_audio(self.ptr, gain_db) }; self
    }
    pub fn remove_ambient_audio(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_ambient_audio(self.ptr) }; self
    }
    pub fn set_channel_audio(&self, gain_db: f64) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_channel_audio(self.ptr, gain_db) }; self
    }
    pub fn remove_channel_audio(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_channel_audio(self.ptr) }; self
    }
}

// ─── Visual components ────────────────────────────────────────────────────────

impl Entity {
    /// Attach `OpacityComponent` (0.0 = transparent, 1.0 = opaque).
    pub fn set_opacity(&self, v: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_opacity(self.ptr, v) }; self
    }
    pub fn opacity(&self) -> f32 { unsafe { realitykit_sys::rk_entity_get_opacity(self.ptr) } }
    pub fn remove_opacity(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_opacity(self.ptr) }; self
    }

    /// Attach `GroundingShadowComponent`.
    pub fn set_grounding_shadow(&self, casts: bool) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_grounding_shadow(self.ptr, casts) }; self
    }
    pub fn remove_grounding_shadow(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_grounding_shadow(self.ptr) }; self
    }

    /// Set 3-D text via `ModelComponent` with a text mesh.
    /// `color` is `[r, g, b, a]` each 0–1.
    pub fn set_text(&self, text: &str, font_size: f32, depth: f32, color: [f32; 4]) -> &Self {
        let [r, g, b, a] = color;
        unsafe { realitykit_sys::rk_entity_set_text(self.ptr, text.as_ptr(), text.len(), font_size, depth, r, g, b, a) }; self
    }
}

// ─── Character controller ─────────────────────────────────────────────────────

impl Entity {
    /// Attach `CharacterControllerComponent` (macOS 13+, iOS 16+, visionOS 1+).
    pub fn set_character_controller(&self, radius: f32, height: f32, slope_limit: f32, step_offset: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_character_controller(self.ptr, radius, height, slope_limit, step_offset) }; self
    }
    pub fn character_move(&self, dx: f32, dy: f32, dz: f32, delta_time: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_character_move(self.ptr, dx, dy, dz, delta_time) }; self
    }
    pub fn remove_character_controller(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_character_controller(self.ptr) }; self
    }
}

// ─── Network sync ─────────────────────────────────────────────────────────────

impl Entity {
    pub fn set_network_sync(&self, allows_ownership_transfer: bool) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_network_sync(self.ptr, allows_ownership_transfer) }; self
    }
    pub fn remove_network_sync(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_network_sync(self.ptr) }; self
    }
}

// ─── macOS 26 / iOS 26 / visionOS 3 ──────────────────────────────────────────

impl Entity {
    /// Attach `BillboardComponent` — entity always faces the camera (macOS 26+).
    #[cfg(any(feature = "macos-26", doc))]
    pub fn set_billboard(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_billboard(self.ptr) }; self
    }
    #[cfg(any(feature = "macos-26", doc))]
    pub fn remove_billboard(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_billboard(self.ptr) }; self
    }

    /// Constant directional force field within `radius` metres (macOS 26+).
    #[cfg(any(feature = "macos-26", doc))]
    pub fn set_constant_force_effect(&self, fx: f32, fy: f32, fz: f32, radius: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_constant_force_effect(self.ptr, fx, fy, fz, radius) }; self
    }
    /// Radial outward / inward force field (macOS 26+). Negative `strength` = attraction.
    #[cfg(any(feature = "macos-26", doc))]
    pub fn set_radial_force_effect(&self, strength: f32, radius: f32, falloff_exponent: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_radial_force_effect(self.ptr, strength, radius, falloff_exponent) }; self
    }
    /// Vortex (swirling) force field (macOS 26+).
    #[cfg(any(feature = "macos-26", doc))]
    pub fn set_vortex_force_effect(&self, strength: f32, radius: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_vortex_force_effect(self.ptr, strength, radius) }; self
    }
    /// Drag / damping force field (macOS 26+).
    #[cfg(any(feature = "macos-26", doc))]
    pub fn set_drag_force_effect(&self, linear_strength: f32, angular_strength: f32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_drag_force_effect(self.ptr, linear_strength, angular_strength) }; self
    }
    #[cfg(any(feature = "macos-26", doc))]
    pub fn remove_force_effect(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_force_effect(self.ptr) }; self
    }
}

// ─── Play AnimationResource ───────────────────────────────────────────────────

impl Entity {
    /// Play a compiled [`AnimationResource`] on this entity.
    ///
    /// Returns an [`AnimationController`] for pause/resume/stop control.
    ///
    /// # Example
    /// ```ignore
    /// let def = AnimationDefinition::orbit([0., 1., 0.], 1.0, true, 2.0,
    ///               AnimationRepeatMode::Repeat, true);
    /// let resource = def.build()?;
    /// let ctrl = entity.play_animation_resource(&resource, 0.3, false);
    /// ```
    pub fn play_animation_resource(
        &self,
        resource:            &crate::animation::AnimationResource,
        transition_duration: f64,
        starts_paused:       bool,
    ) -> AnimationController {
        let ptr = unsafe {
            realitykit_sys::rk_entity_play_animation_resource(
                self.ptr, resource.ptr, transition_duration, starts_paused,
            )
        };
        AnimationController { ptr }
    }
}

// ─── Missing components ───────────────────────────────────────────────────────

impl Entity {
    /// `TextComponent` — display styled text on the entity (macOS 26+ / iOS 26+).
    /// No-op on older OS versions.
    pub fn set_text_component(&self, text: &str, font_size: f32) -> &Self {
        let font = "";
        unsafe {
            realitykit_sys::rk_entity_set_text_component(
                self.ptr,
                text.as_ptr(), text.len(),
                font.as_ptr(), font.len(),
                font_size, 1., 1., 1., 1.,
            )
        };
        self
    }
    pub fn remove_text_component(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_text_component(self.ptr) }; self
    }

    /// `ModelDebugOptionsComponent` — visualization mode for debugging.
    ///
    /// `mode`: 0=none 1=normal 2=baseColor 3=roughness 4=ambientOcclusion
    ///         5=specular 6=emissive 7=textureCoordinates 8=lightingDiffuse 9=lightingSpecular
    pub fn set_debug_options(&self, mode: i32) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_debug_options(self.ptr, mode) }; self
    }
    pub fn remove_debug_options(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_debug_options(self.ptr) }; self
    }

    /// `AccessibilityComponent` — accessibility label visible to screen readers.
    /// (macOS 13+ / iOS 16+; no-op on older OS.)
    pub fn set_accessibility(&self, label: &str, hidden: bool) -> &Self {
        unsafe {
            realitykit_sys::rk_entity_set_accessibility(
                self.ptr, label.as_ptr(), label.len(), hidden,
            )
        };
        self
    }
    pub fn remove_accessibility(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_accessibility(self.ptr) }; self
    }

    /// `AnchoringComponent` — attach to an AR anchor target.
    ///
    /// `target`: 0=world 1=camera.  For world anchors, provide a column-major
    /// 4×4 transform matrix (identity = `[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]`).
    pub fn set_anchoring(&self, target: i32, matrix: &[f32; 16]) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_anchoring(self.ptr, target, matrix.as_ptr()) }; self
    }
    pub fn remove_anchoring(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_anchoring(self.ptr) }; self
    }

    /// `SynchronizationComponent` — mark this entity for multiplayer sync.
    pub fn set_synchronization(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_synchronization(self.ptr) }; self
    }
    /// `true` if the local peer owns this entity.
    pub fn synchronization_is_owner(&self) -> bool {
        unsafe { realitykit_sys::rk_entity_synchronization_is_owner(self.ptr) }
    }
    pub fn remove_synchronization(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_synchronization(self.ptr) }; self
    }

    /// Add an `AudioFileResource` to this entity's `AudioLibraryComponent`
    /// under the given name.  The component is created automatically if absent.
    pub fn audio_library_add(&self, name: &str, resource: &crate::audio::AudioResource) -> &Self {
        unsafe {
            realitykit_sys::rk_entity_audio_library_add(
                self.ptr, name.as_ptr(), name.len(), resource.ptr,
            )
        };
        self
    }
    pub fn audio_library_remove(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_audio_library_remove(self.ptr) }; self
    }

    /// `VideoPlayerComponent` — attach an `AVPlayer` playing `url` to this entity.
    /// (macOS 14+ / iOS 17+; no-op on older OS.)
    pub fn set_video_player(&self, url: &str) -> &Self {
        unsafe { realitykit_sys::rk_entity_set_video_player(self.ptr, url.as_ptr(), url.len()) }; self
    }
    pub fn video_player_play(&self)  -> &Self { unsafe { realitykit_sys::rk_entity_video_player_play(self.ptr) };  self }
    pub fn video_player_pause(&self) -> &Self { unsafe { realitykit_sys::rk_entity_video_player_pause(self.ptr) }; self }
    pub fn video_player_seek(&self, seconds: f64) -> &Self {
        unsafe { realitykit_sys::rk_entity_video_player_seek(self.ptr, seconds) }; self
    }
    pub fn remove_video_player(&self) -> &Self {
        unsafe { realitykit_sys::rk_entity_remove_video_player(self.ptr) }; self
    }
}

// ─── Raw handle ───────────────────────────────────────────────────────────────

impl Entity {
    /// Return the raw `*mut c_void` handle (for interop with other crates).
    pub fn raw(&self) -> *mut c_void { self.ptr }
}


