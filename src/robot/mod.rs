use crate::{general_movement::Direction, robot::health::Health};
use avian2d::prelude::*;
use bevy::{
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
    EnemyHitbox,
}

#[derive(Component, Default)]
#[require(
    Health = Health(100),
    Direction = Direction(1.0),
    RigidBody = RigidBody::Dynamic,
    LockedAxes = LockedAxes::ROTATION_LOCKED
)]
pub struct Robot;

#[derive(Component, Default)]
#[require(
    Transform = Transform::from_xyz(8.0, -80.0, 1.0),
    Collider = Collider::capsule(50.0,60.0),
)]
pub struct RobotCollider;
