//! Physics types — PhysicsBodyMode, CollisionMode.

/// Controls how `PhysicsBodyComponent` participates in the simulation.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhysicsBodyMode {
    /// Immovable; collisions affect dynamic bodies.
    Static   = 0,
    /// Fully simulated by the physics engine.
    Dynamic  = 1,
    /// Moved by code; generates collision events but ignores physics forces.
    Kinematic = 2,
}

/// Controls how `CollisionComponent` responds to overlaps.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollisionMode {
    /// Solid — generates contact forces.
    Default = 0,
    /// Trigger — generates overlap events but no forces.
    Trigger = 1,
}
