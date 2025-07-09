use crate::{Recoil, robot::*};
#[derive(Component, PartialEq)]
pub struct Enemy;

#[derive(Component, PartialEq)]
pub struct EnemyCollider;
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
        layers,
        Transform::from_xyz(-500.0, 500.0, 0.0),
        Recoil,
        (
            RigidBody::Dynamic,
            children![(
                CollisionEventsEnabled,
                Hitbox,
                EnemyCollider,
                layers,
                robot_collider()
            )],
        ),
        robot(asset_server),
    )
}
