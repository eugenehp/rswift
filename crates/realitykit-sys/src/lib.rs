//! Raw FFI bindings to RealityKit — compiled and linked automatically.
//!
//! The Swift bridge (`swift/bridge.swift`) is compiled by `build.rs` using
//! `swift_helper_build::SwiftBridge` and linked as a dylib at compile time.
//! No manual `build.sh` step and no runtime `dlopen` are needed.
//!
//! **Usage**: just add `realitykit-sys` as a dependency — `cargo build` does
//! the rest.  The ergonomic wrappers in the `realitykit` crate are preferred
//! over calling these raw functions directly.
//!
//! ## Platform support
//! macOS 12+, iOS 15+, visionOS 1+.
//!
//! ## License
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

#![allow(non_snake_case, dead_code, clippy::too_many_arguments)]

use core::ffi::c_void;

pub type Handle = *mut c_void;

// ─── Extern declarations ──────────────────────────────────────────────────────
// Resolved at link time against librealitykit_bridge.dylib compiled by build.rs.
// No #[link] attribute needed — cargo:rustc-link-lib from build.rs handles it.
unsafe extern "C" {
    // Lifecycle
    pub fn rk_release(h: Handle);
    pub fn rk_retain(h: Handle);

    // Entity core
    pub fn rk_entity_new() -> Handle;
    pub fn rk_entity_clone(h: Handle) -> Handle;
    pub fn rk_entity_set_name(h: Handle, ptr: *const u8, len: usize);
    pub fn rk_entity_get_name(h: Handle, buf: *mut u8, len: usize) -> usize;
    pub fn rk_entity_add_child(parent: Handle, child: Handle);
    pub fn rk_entity_remove_child(parent: Handle, child: Handle);
    pub fn rk_entity_remove_from_parent(h: Handle);
    pub fn rk_entity_child_count(h: Handle) -> usize;
    pub fn rk_entity_find_child_named(h: Handle, ptr: *const u8, len: usize, recursive: bool) -> *mut c_void;

    // Transform
    pub fn rk_entity_set_position(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_get_position(h: Handle, out: *mut f32);
    pub fn rk_entity_set_rotation_quat(h: Handle, x: f32, y: f32, z: f32, w: f32);
    pub fn rk_entity_get_rotation_quat(h: Handle, out: *mut f32);
    pub fn rk_entity_set_scale(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_get_scale(h: Handle, out: *mut f32);
    pub fn rk_entity_set_uniform_scale(h: Handle, s: f32);
    pub fn rk_entity_set_transform(h: Handle, tx:f32,ty:f32,tz:f32, rx:f32,ry:f32,rz:f32,rw:f32, sx:f32,sy:f32,sz:f32);
    pub fn rk_entity_get_transform(h: Handle, out_pos: *mut f32, out_rot: *mut f32, out_scale: *mut f32);
    pub fn rk_entity_set_is_enabled(h: Handle, v: bool);
    pub fn rk_entity_is_enabled(h: Handle) -> bool;
    pub fn rk_entity_is_enabled_in_hierarchy(h: Handle) -> bool;
    pub fn rk_entity_look_at(h: Handle, tx:f32,ty:f32,tz:f32, ux:f32,uy:f32,uz:f32);

    // ModelEntity
    pub fn rk_model_entity_new(mesh: Handle, mat: Handle) -> Handle;
    pub fn rk_model_entity_new_many(mesh: Handle, mats: *const Handle, count: usize) -> Handle;
    pub fn rk_model_entity_set_material(h: Handle, mat: Handle, slot: usize);
    pub fn rk_model_entity_material_count(h: Handle) -> usize;
    pub fn rk_model_entity_set_mesh(h: Handle, mesh: Handle);

    // AnchorEntity
    pub fn rk_anchor_entity_new() -> Handle;
    pub fn rk_anchor_entity_world(x: f32, y: f32, z: f32) -> Handle;
    pub fn rk_anchor_entity_plane_any() -> Handle;
    pub fn rk_anchor_entity_plane_horizontal() -> Handle;
    pub fn rk_anchor_entity_plane_vertical() -> Handle;
    pub fn rk_anchor_entity_image(gp: *const u8, gl: usize, np: *const u8, nl: usize) -> Handle;
    pub fn rk_anchor_entity_face() -> *mut c_void;
    pub fn rk_anchor_entity_body() -> *mut c_void;
    pub fn rk_anchor_entity_head() -> Handle;
    pub fn rk_anchor_entity_left_hand() -> Handle;
    pub fn rk_anchor_entity_right_hand() -> Handle;

    // Mesh
    pub fn rk_mesh_box(w: f32, h: f32, d: f32) -> Handle;
    pub fn rk_mesh_box_chamfer(w: f32, h: f32, d: f32, c: f32) -> Handle;
    pub fn rk_mesh_sphere(r: f32) -> Handle;
    pub fn rk_mesh_plane(w: f32, d: f32) -> Handle;
    pub fn rk_mesh_plane_corner_radius(w: f32, d: f32, r: f32) -> Handle;
    pub fn rk_mesh_cone(h: f32, r: f32) -> Handle;
    pub fn rk_mesh_cylinder(h: f32, r: f32) -> Handle;
    pub fn rk_mesh_capsule(h: f32, r: f32) -> Handle;
    pub fn rk_mesh_torus(ring: f32, pipe: f32) -> Handle;
    pub fn rk_mesh_text(ptr: *const u8, len: usize, depth: f32, font_size: f32) -> Handle;

    // Materials
    pub fn rk_material_simple(r: f32, g: f32, b: f32, roughness: f32, metallic: bool) -> Handle;
    pub fn rk_material_simple_alpha(r: f32, g: f32, b: f32, a: f32, roughness: f32, metallic: bool) -> Handle;
    pub fn rk_material_unlit(r: f32, g: f32, b: f32) -> Handle;
    pub fn rk_material_unlit_alpha(r: f32, g: f32, b: f32, a: f32) -> Handle;
    pub fn rk_material_occlusion() -> Handle;
    pub fn rk_material_pbr_new() -> Handle;
    pub fn rk_material_pbr_set_base_color(h: Handle, r: f32, g: f32, b: f32, a: f32);
    pub fn rk_material_pbr_set_roughness(h: Handle, v: f32);
    pub fn rk_material_pbr_set_metallic(h: Handle, v: f32);
    pub fn rk_material_pbr_set_emissive(h: Handle, r: f32, g: f32, b: f32, intensity: f32);
    pub fn rk_material_pbr_set_opacity(h: Handle, v: f32);
    pub fn rk_material_pbr_set_clearcoat(h: Handle, cc: f32, rough: f32);
    pub fn rk_material_pbr_set_sheen(h: Handle, r: f32, g: f32, b: f32);
    pub fn rk_material_pbr_set_specular(h: Handle, v: f32);
    pub fn rk_material_pbr_set_anisotropy(h: Handle, v: f32);
    pub fn rk_material_pbr_set_base_color_texture(h: Handle, tex: Handle);
    pub fn rk_material_pbr_set_normal_texture(h: Handle, tex: Handle);
    pub fn rk_material_pbr_set_roughness_texture(h: Handle, tex: Handle);
    pub fn rk_material_pbr_set_metallic_texture(h: Handle, tex: Handle);
    pub fn rk_texture_load(ptr: *const u8, len: usize, err: *mut u8, err_len: usize) -> *mut c_void;

    // Lights
    pub fn rk_point_light(r: f32, g: f32, b: f32, intensity: f32, atten: f32) -> Handle;
    pub fn rk_point_light_set_color(h: Handle, r: f32, g: f32, b: f32);
    pub fn rk_point_light_set_intensity(h: Handle, v: f32);
    pub fn rk_point_light_set_attenuation_radius(h: Handle, v: f32);
    pub fn rk_directional_light(r: f32, g: f32, b: f32, intensity: f32, casts: bool) -> Handle;
    pub fn rk_directional_light_set_color(h: Handle, r: f32, g: f32, b: f32);
    pub fn rk_directional_light_set_intensity(h: Handle, v: f32);
    pub fn rk_directional_light_set_casts_shadow(h: Handle, v: bool);
    pub fn rk_spot_light(r: f32, g: f32, b: f32, intensity: f32, inner: f32, outer: f32, radius: f32) -> Handle;
    pub fn rk_spot_light_set_angles(h: Handle, inner: f32, outer: f32);
    pub fn rk_spot_light_set_color(h: Handle, r: f32, g: f32, b: f32);
    pub fn rk_spot_light_set_intensity(h: Handle, v: f32);
    pub fn rk_image_based_light_new(env: Handle, intensity_exp: f32) -> Handle;
    pub fn rk_entity_set_image_based_light_receiver(target: Handle, ibl: Handle);
    pub fn rk_entity_remove_image_based_light_receiver(h: Handle);
    pub fn rk_entity_set_environment_lighting_weight(h: Handle, w: f32);
    pub fn rk_entity_remove_environment_lighting_config(h: Handle);

    // Physics body
    pub fn rk_entity_set_physics_body(h: Handle, mode: i32, mass: f32, friction: f32, restitution: f32, lin_damp: f32, ang_damp: f32);
    pub fn rk_entity_set_physics_body_mode(h: Handle, mode: i32);
    pub fn rk_entity_set_physics_body_mass(h: Handle, mass: f32);
    pub fn rk_entity_set_physics_body_material(h: Handle, friction: f32, restitution: f32);
    pub fn rk_entity_is_resting(h: Handle) -> bool;
    pub fn rk_entity_remove_physics_body(h: Handle);

    // Collision
    pub fn rk_entity_set_collision_box(h: Handle, w: f32, ht: f32, d: f32, ox: f32, oy: f32, oz: f32);
    pub fn rk_entity_set_collision_sphere(h: Handle, r: f32, ox: f32, oy: f32, oz: f32);
    pub fn rk_entity_set_collision_capsule(h: Handle, height: f32, radius: f32, ox: f32, oy: f32, oz: f32);
    pub fn rk_entity_set_collision_convex_hull(h: Handle);
    pub fn rk_entity_set_collision_mode(h: Handle, mode: i32);
    pub fn rk_entity_remove_collision(h: Handle);

    // Physics forces & velocity
    pub fn rk_entity_set_linear_velocity(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_get_linear_velocity(h: Handle, out: *mut f32);
    pub fn rk_entity_set_angular_velocity(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_get_angular_velocity(h: Handle, out: *mut f32);
    pub fn rk_entity_apply_linear_impulse(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_apply_angular_impulse(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_add_force(h: Handle, fx: f32, fy: f32, fz: f32, px: f32, py: f32, pz: f32);
    pub fn rk_entity_add_torque(h: Handle, x: f32, y: f32, z: f32);
    pub fn rk_entity_reset_physics(h: Handle);
    pub fn rk_entity_set_physics_simulation(h: Handle, gx: f32, gy: f32, gz: f32);
    pub fn rk_entity_remove_physics_simulation(h: Handle);

    // Animation
    pub fn rk_entity_move_to(h: Handle, tx:f32,ty:f32,tz:f32, rx:f32,ry:f32,rz:f32,rw:f32, sx:f32,sy:f32,sz:f32, duration:f64, timing:i32) -> Handle;
    pub fn rk_entity_animation_count(h: Handle) -> usize;
    pub fn rk_entity_play_animation(h: Handle, idx: usize, transition: f64, paused: bool) -> *mut c_void;
    pub fn rk_entity_stop_all_animations(h: Handle);
    pub fn rk_animation_controller_pause(h: Handle);
    pub fn rk_animation_controller_resume(h: Handle);
    pub fn rk_animation_controller_stop(h: Handle);
    pub fn rk_animation_controller_is_playing(h: Handle) -> bool;
    pub fn rk_animation_controller_is_paused(h: Handle) -> bool;

    // Audio
    pub fn rk_audio_resource_load(ptr: *const u8, len: usize, mode: i32, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_audio_resource_load_named(ptr: *const u8, len: usize, mode: i32, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_entity_play_audio(entity: Handle, resource: Handle) -> Handle;
    pub fn rk_entity_stop_all_audio(h: Handle);
    pub fn rk_audio_controller_pause(h: Handle);
    pub fn rk_audio_controller_resume(h: Handle);
    pub fn rk_audio_controller_stop(h: Handle);
    pub fn rk_audio_controller_is_playing(h: Handle) -> bool;
    pub fn rk_audio_controller_get_gain(h: Handle) -> f64;
    pub fn rk_audio_controller_set_gain(h: Handle, v: f64);
    pub fn rk_audio_controller_get_speed(h: Handle) -> f64;
    pub fn rk_audio_controller_set_speed(h: Handle, v: f64);
    pub fn rk_entity_set_spatial_audio(h: Handle, direct_db: f64, reverb_db: f64);
    pub fn rk_entity_remove_spatial_audio(h: Handle);
    pub fn rk_entity_set_ambient_audio(h: Handle, gain: f64);
    pub fn rk_entity_remove_ambient_audio(h: Handle);
    pub fn rk_entity_set_channel_audio(h: Handle, gain: f64);
    pub fn rk_entity_remove_channel_audio(h: Handle);

    // Visual components
    pub fn rk_entity_set_opacity(h: Handle, v: f32);
    pub fn rk_entity_get_opacity(h: Handle) -> f32;
    pub fn rk_entity_remove_opacity(h: Handle);
    pub fn rk_entity_set_grounding_shadow(h: Handle, casts: bool);
    pub fn rk_entity_remove_grounding_shadow(h: Handle);

    // Camera
    pub fn rk_perspective_camera_new(fov: f32, near: f32, far: f32) -> Handle;
    pub fn rk_perspective_camera_set_fov(h: Handle, fov: f32);
    pub fn rk_perspective_camera_set_clip(h: Handle, near: f32, far: f32);
    pub fn rk_orthographic_camera_new(scale: f32, near: f32, far: f32) -> Handle;
    pub fn rk_orthographic_camera_set_scale(h: Handle, scale: f32);
    pub fn rk_entity_remove_camera(h: Handle);

    // Character controller
    pub fn rk_entity_set_character_controller(h: Handle, radius: f32, height: f32, slope: f32, step: f32);
    pub fn rk_entity_character_move(h: Handle, dx: f32, dy: f32, dz: f32, dt: f32);
    pub fn rk_entity_remove_character_controller(h: Handle);

    // Text (3-D mesh)
    pub fn rk_entity_set_text(h: Handle, ptr: *const u8, len: usize, font_size: f32, depth: f32, r: f32, g: f32, b: f32, a: f32);

    // Particles (macOS 14+)
    pub fn rk_entity_set_particle_emitter(h: Handle, birth_rate: f32, speed: f32, lifetime: f32, size: f32, r: f32, g: f32, b: f32, a: f32);
    pub fn rk_entity_particle_set_birth_rate(h: Handle, v: f32);
    pub fn rk_entity_particle_set_speed(h: Handle, v: f32);
    pub fn rk_entity_particle_set_color(h: Handle, r: f32, g: f32, b: f32, a: f32);
    pub fn rk_entity_particle_set_lifetime(h: Handle, v: f32);
    pub fn rk_entity_particle_set_size(h: Handle, v: f32);
    pub fn rk_entity_remove_particle_emitter(h: Handle);

    // Billboard (macOS 26+, iOS 26+, visionOS 3+)
    pub fn rk_entity_set_billboard(h: Handle);
    pub fn rk_entity_remove_billboard(h: Handle);

    // Force effects (macOS 26+, iOS 26+, visionOS 3+)
    pub fn rk_entity_set_constant_force_effect(h: Handle, fx: f32, fy: f32, fz: f32, radius: f32);
    pub fn rk_entity_set_radial_force_effect(h: Handle, strength: f32, radius: f32, falloff: f32);
    pub fn rk_entity_set_vortex_force_effect(h: Handle, strength: f32, radius: f32);
    pub fn rk_entity_set_drag_force_effect(h: Handle, linear: f32, angular: f32);
    pub fn rk_entity_remove_force_effect(h: Handle);

    // Network sync
    pub fn rk_entity_set_network_sync(h: Handle, allows_transfer: bool);
    pub fn rk_entity_remove_network_sync(h: Handle);

    // Environment resource
    pub fn rk_environment_resource_load(ptr: *const u8, len: usize, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_environment_resource_load_named(ptr: *const u8, len: usize, err: *mut u8, err_len: usize) -> *mut c_void;

    // Asset loading
    pub fn rk_entity_load_from_file(ptr: *const u8, len: usize, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_entity_load_named(ptr: *const u8, len: usize, err: *mut u8, err_len: usize) -> *mut c_void;

    // Scene / ARView
    pub fn rk_arview_new(w: f32, h: f32) -> *mut c_void;
    pub fn rk_arview_scene_add_anchor(view: Handle, anchor: Handle);
    pub fn rk_arview_scene_remove_anchor(view: Handle, anchor: Handle);
    pub fn rk_arview_set_camera_mode(view: Handle, mode: i32);
    pub fn rk_arview_set_environment_intensity(view: Handle, v: f32);
    pub fn rk_arview_set_render_options(view: Handle, flags: u32);
    pub fn rk_arview_scene_anchor_count(view: Handle) -> usize;
    pub fn rk_arview_scene_get_anchor(view: Handle, idx: usize) -> *mut c_void;
    pub fn rk_arview_find_entity_named(view: Handle, ptr: *const u8, len: usize) -> *mut c_void;

    // ── Custom mesh from buffers ──────────────────────────────────────────────
    pub fn rk_mesh_from_buffers(
        pos_ptr: *const f32, pos_count: usize,
        norm_ptr: *const f32, norm_count: usize,
        uv_ptr: *const f32, uv_count: usize,
        idx_ptr: *const u32, idx_count: usize,
        err: *mut u8, err_len: usize,
    ) -> *mut c_void;

    // ── Texture from raw RGBA8 bytes ──────────────────────────────────────────
    pub fn rk_texture_from_rgba8(
        ptr: *const u8, len: usize,
        width: i32, height: i32,
        err: *mut u8, err_len: usize,
    ) -> *mut c_void;

    // ── Animation definition builders ─────────────────────────────────────────
    pub fn rk_animdef_orbit(
        axis_x: f32, axis_y: f32, axis_z: f32,
        rotation_count: f32, clockwise: bool, orient_to_path: bool,
        duration: f64, repeat_mode: i32, is_additive: bool,
    ) -> Handle;
    pub fn rk_animdef_from_to_transform(
        ftx: f32, fty: f32, ftz: f32,
        frx: f32, fry: f32, frz: f32, frw: f32,
        fsx: f32, fsy: f32, fsz: f32,
        ttx: f32, tty: f32, ttz: f32,
        trx: f32, try_: f32, trz: f32, trw: f32,
        tsx: f32, tsy: f32, tsz: f32,
        duration: f64, timing: i32, repeat_mode: i32, is_additive: bool,
    ) -> Handle;
    pub fn rk_animation_resource_from_def(def: Handle, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_animation_resource_group(defs: *const Handle, count: usize, speed: f32, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_entity_play_animation_resource(entity: Handle, resource: Handle, transition: f64, paused: bool) -> Handle;

    // ── Scene event subscriptions ─────────────────────────────────────────────
    pub fn rk_scene_subscribe_update(view: Handle, cb: unsafe extern "C" fn(f32, *mut c_void), ud: *mut c_void) -> Handle;
    pub fn rk_scene_subscribe_collision_began(view: Handle, cb: unsafe extern "C" fn(Handle, Handle, f32, *mut c_void), ud: *mut c_void) -> Handle;
    pub fn rk_scene_subscribe_collision_ended(view: Handle, cb: unsafe extern "C" fn(Handle, Handle, *mut c_void), ud: *mut c_void) -> Handle;
    pub fn rk_scene_subscribe_animation_completed(view: Handle, cb: unsafe extern "C" fn(*mut c_void, *mut c_void), ud: *mut c_void) -> Handle;
    pub fn rk_scene_subscribe_audio_completed(view: Handle, cb: unsafe extern "C" fn(*mut c_void, *mut c_void), ud: *mut c_void) -> Handle;

    // ── Scene raycasting ──────────────────────────────────────────────────────
    pub fn rk_scene_raycast(
        view: Handle,
        ox: f32, oy: f32, oz: f32,
        dx: f32, dy: f32, dz: f32,
        length: f32, query_mode: i32,
        out_entities: *mut *mut c_void,
        out_positions: *mut f32,
        out_normals: *mut f32,
        out_distances: *mut f32,
        max_hits: usize,
    ) -> usize;
    pub fn rk_scene_convex_cast(
        view: Handle,
        ox: f32, oy: f32, oz: f32,
        ex: f32, ey: f32, ez: f32,
        shape: Handle,
        out_entities: *mut *mut c_void,
        out_positions: *mut f32,
        out_normals: *mut f32,
        out_distances: *mut f32,
        max_hits: usize,
    ) -> usize;
    pub fn rk_scene_query_entities(view: Handle, component_id: i32, out: *mut *mut c_void, max_count: usize) -> usize;

    // ── Missing components ────────────────────────────────────────────────────
    pub fn rk_entity_set_text_component(h: Handle, text: *const u8, text_len: usize, font: *const u8, font_len: usize, font_size: f32, r: f32, g: f32, b: f32, a: f32);
    pub fn rk_entity_remove_text_component(h: Handle);
    pub fn rk_entity_set_debug_options(h: Handle, mode: i32);
    pub fn rk_entity_remove_debug_options(h: Handle);
    pub fn rk_entity_set_accessibility(h: Handle, label: *const u8, label_len: usize, is_hidden: bool);
    pub fn rk_entity_remove_accessibility(h: Handle);
    pub fn rk_entity_set_anchoring(h: Handle, target: i32, matrix: *const f32);
    pub fn rk_entity_remove_anchoring(h: Handle);
    pub fn rk_entity_set_synchronization(h: Handle);
    pub fn rk_entity_synchronization_is_owner(h: Handle) -> bool;
    pub fn rk_entity_remove_synchronization(h: Handle);
    pub fn rk_entity_audio_library_add(h: Handle, name: *const u8, name_len: usize, resource: Handle);
    pub fn rk_entity_audio_library_remove(h: Handle);
    pub fn rk_entity_set_video_player(h: Handle, url: *const u8, url_len: usize);
    pub fn rk_entity_video_player_play(h: Handle);
    pub fn rk_entity_video_player_pause(h: Handle);
    pub fn rk_entity_video_player_seek(h: Handle, seconds: f64);
    pub fn rk_entity_remove_video_player(h: Handle);

    // ── Additional materials ──────────────────────────────────────────────────
    pub fn rk_material_video(url: *const u8, url_len: usize, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_material_video_play(h: Handle);
    pub fn rk_material_video_pause(h: Handle);
    pub fn rk_material_portal_new() -> Handle;
    pub fn rk_material_shader_graph_from_file(
        material_name: *const u8,
        material_name_len: usize,
        file_path: *const u8,
        file_path_len: usize,
        err: *mut u8,
        err_len: usize,
    ) -> *mut c_void;

    // ── AudioFileGroupResource ────────────────────────────────────────────────
    pub fn rk_audio_group_load_named(name: *const u8, name_len: usize, err: *mut u8, err_len: usize) -> *mut c_void;
    pub fn rk_entity_play_audio_group(entity: Handle, group: Handle) -> Handle;

    // visionOS (stubs on macOS/iOS)
    pub fn rk_entity_set_input_target(h: Handle, direct: bool, indirect: bool);
    pub fn rk_entity_remove_input_target(h: Handle);
    pub fn rk_entity_set_hover_effect(h: Handle, r: f32, g: f32, b: f32, strength: f32);
    pub fn rk_entity_remove_hover_effect(h: Handle);
    pub fn rk_portal_entity_new(world: Handle) -> Handle;
    pub fn rk_entity_set_world_component(h: Handle);
    pub fn rk_entity_remove_world_component(h: Handle);
    pub fn rk_entity_set_surroundings_effect(h: Handle, intensity: f32);
    pub fn rk_entity_remove_surroundings_effect(h: Handle);
}

/// Backward-compatibility no-op — bridge is linked at compile time by build.rs.
pub fn load_auto() -> Result<(), String> { Ok(()) }

/// Backward-compatibility no-op — see [`load_auto`].
pub fn load(_path: &str) -> Result<(), String> { Ok(()) }
