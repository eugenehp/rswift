//! Physics simulation demo — stacked boxes falling under gravity.
//!
//! cargo run -p realitykit --example physics

use realitykit::prelude::*;

fn main() {
    println!("=== RealityKit physics demo ===\n");

    let box_mesh  = MeshResource::cube(0.3);
    let ball_mesh = MeshResource::sphere(0.2);
    let floor_mesh = MeshResource::plane(10.0, 10.0);

    let blue  = Material::pbr().base_color(0.2, 0.4, 0.9, 1.0).roughness(0.5).build();
    let red   = Material::pbr().base_color(0.9, 0.2, 0.2, 1.0).roughness(0.3).metallic(0.1).build();
    let grey  = Material::simple(0.4, 0.4, 0.4);

    // Static floor
    let floor = Entity::model(&floor_mesh, &grey);
    floor.set_physics_body(PhysicsBodyMode::Static, 0.0, 0.8, 0.4);
    floor.set_collision_box(10.0, 0.02, 10.0, 0.0, 0.0, 0.0);

    // Dynamic boxes
    let boxes: Vec<Entity> = (0..5).map(|i| {
        let b = Entity::model(&box_mesh, &blue).at(0.0, 0.3 + i as f32 * 0.4, 0.0);
        b.set_physics_body(PhysicsBodyMode::Dynamic, 1.0, 0.4, 0.3);
        b.set_collision_box(0.3, 0.3, 0.3, 0.0, 0.0, 0.0);
        b
    }).collect();

    // A ball with initial impulse
    let ball = Entity::model(&ball_mesh, &red).at(0.5, 2.5, 0.0);
    ball.set_physics_body(PhysicsBodyMode::Dynamic, 0.5, 0.2, 0.7);
    ball.set_collision_sphere(0.2, 0.0, 0.0, 0.0);
    ball.apply_linear_impulse(-1.5, 2.0, 0.0);

    // Kinematic entity driven by code
    let pusher = Entity::model(&box_mesh, &grey).at(-2.0, 0.15, 0.0);
    pusher.set_physics_body(PhysicsBodyMode::Kinematic, 5.0, 0.8, 0.2);
    pusher.set_collision_box(0.3, 0.3, 0.3, 0.0, 0.0, 0.0);
    pusher.set_linear_velocity(0.5, 0.0, 0.0);

    // Custom gravity root
    let root = Entity::anchor(0.0, 0.0, -3.0);
    root.set_physics_simulation(0.0, -9.81, 0.0);
    root.add_child(&floor).add_child(&ball).add_child(&pusher);
    for b in &boxes { root.add_child(b); }

    let v = ball.linear_velocity();
    println!("Ball initial velocity: ({:.2}, {:.2}, {:.2})", v[0], v[1], v[2]);
    println!("Stack of {} boxes assembled", boxes.len());
    println!("Pusher kinematic at {:?}", pusher.position());
    println!("\n✓ Physics scene built — run in an ARView to see simulation.");
}
