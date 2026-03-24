//! RealityKit scene builder — demonstrates the full Rust API.
//!
//! cargo run -p realitykit --example scene_builder

use realitykit::prelude::*;

fn main() {

    println!("=== RealityKit scene builder ===\n");

    // ── Meshes ────────────────────────────────────────────────────────────
    let sphere_mesh   = MeshResource::sphere(0.4);
    let box_mesh      = MeshResource::box_(0.4, 0.4, 0.4);
    let chamfer_mesh  = MeshResource::box_chamfer(0.6, 0.3, 0.6, 0.04);
    let floor_mesh    = MeshResource::plane(8.0, 8.0);
    let cone_mesh     = MeshResource::cone(0.8, 0.25);
    let cyl_mesh      = MeshResource::cylinder(1.0, 0.15);
    let caps_mesh     = MeshResource::capsule(0.8, 0.2);
    let torus_mesh    = MeshResource::torus(0.5, 0.12);
    let text_mesh     = MeshResource::text("Hello 🦀", 0.05, 18.0);

    println!("✓ 9 meshes created");

    // ── Materials ─────────────────────────────────────────────────────────
    let red_pbr = Material::pbr()
        .base_color(0.9, 0.1, 0.1, 1.0)
        .roughness(0.3)
        .metallic(0.0)
        .build();

    let metal = Material::pbr()
        .base_color(0.8, 0.8, 0.9, 1.0)
        .roughness(0.1)
        .metallic(1.0)
        .clearcoat(0.5, 0.1)
        .build();

    let glass = Material::pbr()
        .base_color(0.7, 0.85, 1.0, 0.25)
        .roughness(0.05)
        .metallic(0.0)
        .opacity(0.25)
        .build();

    let floor_mat = Material::simple(0.35, 0.35, 0.35);
    let unlit_mat = Material::unlit(0.2, 1.0, 0.4);
    let occl_mat  = Material::occlusion();
    println!("✓ 6 materials created");

    // ── Entities ──────────────────────────────────────────────────────────
    let sphere = Entity::model(&sphere_mesh, &red_pbr).at(0.0, 1.2, 0.0);
    sphere.set_name("RedSphere");
    sphere.set_opacity(0.9);

    let metal_box = Entity::model(&box_mesh, &metal).at(1.2, 0.2, 0.0);
    let glass_cyl = Entity::model(&cyl_mesh, &glass).at(-1.0, 0.5, 0.0);
    let cone_e    = Entity::model(&cone_mesh, &unlit_mat).at(0.0, 0.4, 1.5);
    let caps_e    = Entity::model(&caps_mesh, &red_pbr).at(2.0, 0.5, 0.5);
    let torus_e   = Entity::model(&torus_mesh, &metal).at(-0.5, 0.5, 2.0);
    let chamf_e   = Entity::model(&chamfer_mesh, &floor_mat).at(1.5, 0.15, -1.0);
    let text_e    = Entity::model(&text_mesh, &unlit_mat).at(-0.8, 1.8, 0.0);
    let floor     = Entity::model(&floor_mesh, &floor_mat);

    println!("✓ 9 model entities created");

    // ── Transform queries ─────────────────────────────────────────────────
    let pos   = sphere.position();
    let rot   = sphere.rotation();
    let scale = sphere.scale();
    println!("  sphere pos={:?} rot={:?} scale={:?}", pos, rot, scale);

    sphere.set_rotation(0.0, 0.707, 0.0, 0.707);
    sphere.set_uniform_scale(1.1);
    sphere.look_at([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

    // ── Lights ────────────────────────────────────────────────────────────
    let sun   = Entity::directional_light(1.0, 0.95, 0.85, 2500.0, true).at(0.0, 5.0, 0.0);
    let point = Entity::point_light(1.0, 0.8, 0.6, 1200.0, 12.0).at(2.0, 3.0, 2.0);
    let spot  = Entity::spot_light(0.9, 0.9, 1.0, 800.0, 15.0, 40.0, 10.0).at(-2.0, 4.0, 0.0);
    println!("✓ 3 lights created");

    // ── Cameras ───────────────────────────────────────────────────────────
    let cam = Entity::perspective_camera(60.0, 0.01, 200.0).at(0.0, 2.0, 6.0);
    cam.look_at([0.0, 0.5, 0.0], [0.0, 1.0, 0.0]);
    println!("✓ perspective camera at (0, 2, 6)");

    // ── Physics ───────────────────────────────────────────────────────────
    sphere.set_physics_body(PhysicsBodyMode::Dynamic, 1.0, 0.6, 0.4);
    sphere.set_collision_sphere(0.4, 0.0, 0.0, 0.0);

    metal_box.set_physics_body(PhysicsBodyMode::Dynamic, 2.0, 0.5, 0.3);
    metal_box.set_collision_box(0.4, 0.4, 0.4, 0.0, 0.0, 0.0);

    floor.set_physics_body(PhysicsBodyMode::Static, 0.0, 0.8, 0.3);
    floor.set_collision_box(8.0, 0.01, 8.0, 0.0, 0.0, 0.0);

    sphere.apply_linear_impulse(0.5, 3.0, 0.2);
    metal_box.set_linear_velocity(0.3, 0.0, -0.2);

    // Physics root
    let phys_root = Entity::new();
    phys_root.set_physics_simulation(0.0, -9.81, 0.0);
    println!("✓ physics configured");

    // ── Animation ─────────────────────────────────────────────────────────
    let anim_ctrl = cone_e.move_to(
        [0.0, 1.5, 1.5],
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        2.0,
        TimingFunction::EaseInOut,
    );
    println!("  animation playing={}", anim_ctrl.is_playing());

    // ── Grounding shadow & opacity ────────────────────────────────────────
    sphere.set_grounding_shadow(true);
    glass_cyl.set_opacity(0.5);
    println!("✓ shadow and opacity set");

    // ── Particles ─────────────────────────────────────────────────────────
    let emitter = Entity::new().at(0.0, 0.0, 0.0);
    use realitykit::particle::ParticleEmitterParams;
    emitter.set_particle_emitter(
        ParticleEmitterParams::new().birth_rate(50.0).speed(1.5).lifetime(2.0).size(0.05)
            .color(1.0, 0.5, 0.1, 1.0)
    );
    emitter.particle_set_color(1.0, 0.8, 0.2, 0.9);
    println!("✓ particle emitter attached");

    // ── Network sync ──────────────────────────────────────────────────────
    sphere.set_network_sync(true);

    // ── Scene graph assembly ──────────────────────────────────────────────
    let anchor = Entity::anchor(0.0, 0.0, -3.0);
    anchor
        .add_child(&floor)
        .add_child(&sphere)
        .add_child(&metal_box)
        .add_child(&glass_cyl)
        .add_child(&cone_e)
        .add_child(&caps_e)
        .add_child(&torus_e)
        .add_child(&chamf_e)
        .add_child(&text_e)
        .add_child(&sun)
        .add_child(&point)
        .add_child(&spot)
        .add_child(&cam)
        .add_child(&emitter)
        .add_child(&phys_root);

    println!("\n✓ scene graph assembled ({} children)", anchor.child_count());

    // ── Name / find ───────────────────────────────────────────────────────
    if let Some(found) = anchor.find_child_named("RedSphere") {
        println!("  found entity named 'RedSphere' at {:?}", found.position());
    }

    // ── Hierarchy management ──────────────────────────────────────────────
    let extra = Entity::new();
    anchor.add_child(&extra);
    anchor.remove_child(&extra);

    // ── Multi-material model ──────────────────────────────────────────────
    let multi = Entity::model_multi(&box_mesh, &[&red_pbr, &metal, &glass]);
    println!("  multi-material model: {} slots", multi.material_count());

    println!("\n=== All done — RealityKit parity demonstration complete ===");
}
