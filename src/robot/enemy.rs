use bevy::prelude::*;
use crate::robot::*;
#[derive(Component, PartialEq)]
pub struct Enemy;

#[derive(Component, PartialEq)]
pub struct EnemyCollider;
pub fn add_enemy(asset_server: &AssetServer) -> impl Bundle {
    (
        Enemy,
        Transform::from_xyz(-100.0, 500.0, 0.0),
        (
            RigidBody::Dynamic,
            children![(EnemyCollider, robot_collider())],
        ),
        robot(asset_server),
    )
}
