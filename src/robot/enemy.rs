use crate::robot::{player::weapons::Hitbox, *};
#[derive(Component, PartialEq)]
pub struct Enemy;

pub fn add_enemy(asset_server: &AssetServer) -> impl Bundle {
    let layers = CollisionLayers::new(
        PhysicsLayers::Enemy,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerProjectile,
        ],
    );
    (
        Enemy,
        Transform::from_xyz(-500.0, 500.0, 0.0),
        (
            RigidBody::Dynamic,
            children![(layers, Hitbox, robot_collider())],
        ),
        robot(asset_server),
    )
}
