use crate::prelude::*;

pub mod enemy;
pub mod hits;
pub mod player;
pub mod ui;

#[derive(Component)]
pub struct Health(pub u32);
impl Default for Health {
    fn default() -> Self {
        Self(250)
    }
}

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
    Spikes,
}

#[derive(Component, Default)]
#[require(
    Direction,
    Health,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    PointLight2d {
        intensity: 1.0,
        radius: 500.0,
        ..default()
    },
)]
pub struct Robot;

#[derive(Component, Default)]
#[require(
    Transform = Transform::from_xyz(8.0, -80.0, 1.0),
    Collider = Collider::capsule(50.0,60.0),
)]
pub struct RobotCollider;
