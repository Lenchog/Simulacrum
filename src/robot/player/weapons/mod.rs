use std::time::Duration;

use crate::{
    Despawnable,
    robot::{Health, PhysicsLayers, health::Damage, player::EquippedWeapons},
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub mod attack;

#[derive(Component)]
pub struct UseTime(Timer);

#[derive(Component)]
pub struct Weapon;
pub fn add_weapon(tip_entity: Entity) -> impl Bundle {
    (
        Weapon,
        Health(30),
        UseTime(Timer::new(Duration::from_millis(500), TimerMode::Once)),
        Sensor,
        Transform::from_xyz(0.0, 0.0, 0.0),
        CooldownFinished(true),
        ChildOf(tip_entity),
    )
}

#[derive(Component)]
pub enum WeaponType {
    Sword,
    LaserGun,
}

pub fn sword(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    MeleeWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
        collider: Collider::rectangle(200.0, 50.0),
        damage: Damage(50),
    }
    .build(tip_entity)
}

pub fn lazer_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            gravity_scale: 0.0,
            linear_velocity: 5000.0,
        },
    }
    .build(tip_entity)
}

#[derive(Component)]
pub struct MeleeWeaponBuilder {
    pub collider: Collider,
    pub sprite: Sprite,
    pub damage: Damage,
}

#[derive(Component)]
pub struct MeleeWeapon;

impl MeleeWeaponBuilder {
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            add_weapon(tip_entity),
            MeleeWeapon,
            Hitbox,
            RigidBody::Dynamic,
            self.damage,
            self.sprite,
            self.collider,
        )
    }
}
#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct RangedWeapon;

#[derive(Component)]
pub struct RangedWeaponBuilder {
    pub projectile_builder: ProjectileBuilder,
    pub sprite: Sprite,
}
impl RangedWeaponBuilder {
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            add_weapon(tip_entity),
            self.projectile_builder,
            self.sprite,
            RangedWeapon,
        )
    }
}

#[derive(Component, Clone)]
pub struct ProjectileBuilder {
    pub linear_velocity: f32,
    pub gravity_scale: f32,
    pub sprite: Sprite,
}
impl ProjectileBuilder {
    fn build(self, direction: Dir2) -> impl Bundle {
        (
            Hitbox,
            Projectile,
            GravityScale(self.gravity_scale),
            LinearVelocity(*(direction) * self.linear_velocity),
            self.sprite,
            RigidBody::Dynamic,
            Despawnable,
        )
    }
}
#[derive(Component, Default)]
#[require(
    Damage = Damage(10),
    Collider = Collider::circle(50.0),
    CollisionEventsEnabled,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::PlayerProjectile,
        [PhysicsLayers::Enemy, PhysicsLayers::Ground],
    )
)]
pub struct Hitbox;

#[derive(Component)]
#[require(Transform, Visibility = Visibility::Inherited)]
pub struct RotationCenter;

#[derive(Component)]
#[require(Visibility = Visibility::Inherited, Transform = Transform::from_xyz(200.0, 0.0, 0.0))]
pub struct WeaponTip;

#[derive(Component)]
pub struct CooldownFinished(bool);
