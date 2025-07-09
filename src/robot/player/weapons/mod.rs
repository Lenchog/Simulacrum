use std::time::Duration;

use crate::{
    Despawnable,
    robot::{Health, Hitbox},
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub mod attack;

#[derive(Component)]
pub struct UseTime(Timer);

#[derive(Component)]
pub struct Weapon;
pub fn add_weapon() -> impl Bundle {
    (
        Weapon,
        Health(30),
        UseTime(Timer::new(Duration::from_millis(500), TimerMode::Once)),
        Sensor,
        Transform::from_xyz(0.0, 0.0, 0.0),
    )
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Clone)]
pub struct ProjectileBuilder {
    pub collision_layers: CollisionLayers,
    pub linear_velocity: f32,
    pub gravity_scale: f32,
    pub sprite: Sprite,
}
impl ProjectileBuilder {
    fn build(self, direction: Dir2) -> impl Bundle {
        (
            Projectile,
            self.collision_layers,
            GravityScale(self.gravity_scale),
            LinearVelocity(*(direction) * self.linear_velocity),
            self.sprite,
            Hitbox,
            Despawnable,
            RigidBody::Dynamic,
            CollisionEventsEnabled,
        )
    }
}

#[derive(Component)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct RotationCenter;

#[derive(Component)]
pub struct WeaponTip;

#[derive(Resource)]
pub struct CooldownFinished(pub bool);
