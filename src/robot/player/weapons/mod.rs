use std::time::Duration;

use crate::robot::{Health, Hitbox, PhysicsLayers};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, GravityScale, LinearVelocity, PhysicsLayer,
    RigidBody, Sensor,
};
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
        Transform::from_translation(Vec3 {
            x: 100.0,
            y: 0.0,
            z: 0.0,
        }),
    )
}
pub fn add_melee_weapon() -> impl Bundle {
    (add_weapon(), Hitbox, Collider::rectangle(1920.0, 440.0))
}

#[derive(Component)]
#[require(LinearVelocity, Sprite)]
pub struct Projectile;
fn add_projectile() -> impl Bundle {
    (
        Projectile,
        //CollisionLayers::new(PhysicsLayers::PlayerProjectile, PhysicsLayers::Enemy),
        Hitbox,
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        GravityScale(0.0),
    )
}

#[derive(Component)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct RotationCenter;

#[derive(Component)]
pub struct WeaponTip;

#[derive(Resource)]
pub struct CooldownFinished(pub bool);
