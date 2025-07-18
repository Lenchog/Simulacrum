use crate::general_movement::Direction;
use avian2d::prelude::*;
use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::*,
};

pub mod enemy;
pub mod hits;
pub mod player;
pub mod ui;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component, Default)]
pub struct Recoil;

#[derive(PhysicsLayer, Default)]
pub enum PhysicsLayers {
    #[default]
    Default,
    Ground,
    Player,
    Enemy,
    PlayerHitbox,
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
