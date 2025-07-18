use crate::robot::PhysicsLayers;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
#[require(
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Ground,
        [
            PhysicsLayers::Enemy,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerHitbox,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
    RigidBody = RigidBody::Static,
    Collider = Collider::rectangle(128.0, 128.0),
    CollisionMargin = CollisionMargin(3.0),
)]
struct Wall;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    wall: Wall,
}
