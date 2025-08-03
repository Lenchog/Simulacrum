use crate::{Hitbox, prelude::*};

#[derive(Component, Default)]
#[require(
    CollisionLayers::new(
        PhysicsLayers::Ground,
        [
            PhysicsLayers::Enemy,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerHitbox,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
    RigidBody::Static,
    Collider::rectangle(128.0, 128.0),
)]
struct Wall;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default)]
#[require(
    Wall,
    Hitbox,
    CollisionLayers::new(
        PhysicsLayers::Spikes,
        [PhysicsLayers::Enemy, PhysicsLayers::Player]
    )
)]
pub struct Spike;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct SpikeBundle {
    spike: Spike,
}
