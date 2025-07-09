use crate::robot::health::{Damage, Health};
use avian2d::prelude::*;
use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, component::Component},
    prelude::*,
};

pub mod enemy;
pub mod health;
pub mod player;

#[derive(PartialEq, Default)]
pub enum RobotType {
    #[default]
    Enemy,
    Player,
}

#[derive(PhysicsLayer, Default)]
pub enum PhysicsLayers {
    #[default]
    Default,
    Ground,
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}

#[derive(Component)]
#[require(Health = Health(100), Transform = Transform::from_xyz(0.0, 500.0, 1.0), RigidBody = RigidBody::Dynamic)]
pub struct Robot;
fn robot(asset_server: &AssetServer) -> impl Bundle {
    (
        Robot,
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        LockedAxes::ROTATION_LOCKED,
    )
}

pub fn robot_collider() -> impl Bundle {
    (
        Transform::from_xyz(8.0, -80.0, 1.0),
        Collider::capsule(50.0, 60.0),
    )
}

#[derive(Component)]
#[require(Damage = Damage(10), Collider = Collider::circle(50.0))]
pub struct Hitbox;
