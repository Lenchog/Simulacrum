use crate::robot::{PhysicsLayers, player::EquippedWeapons};
use std::time::Duration;

#[derive(Component, Clone)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct Despawnable;

use avian2d::prelude::*;
use bevy::prelude::*;

pub mod attack;

#[derive(Component)]
pub struct UseTime(Timer);

#[derive(Component, Default)]
#[require(Hitbox, RigidBody::Dynamic,

    Collider::circle(50.0),
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::PlayerHitbox,
        [PhysicsLayers::Enemy, PhysicsLayers::Ground],
    )
)]
pub struct PlayerHitbox;

#[derive(Component, Default)]
#[require(Sensor, CooldownFinished(true))]
pub struct Weapon;

#[derive(Component)]
pub struct Equipped;

#[derive(Component, Default)]
pub struct Swingable;

#[derive(Component)]
pub struct MeleeWeaponBuilder {
    pub collider: Collider,
    pub sprite: Sprite,
    pub damage: Damage,
}

#[derive(Component)]
#[require(
    Weapon,
    Swingable,
    PlayerHitbox,
    Visibility::Hidden,
    CollisionLayers::new(PhysicsLayers::PlayerHitbox, PhysicsLayers::Enemy,)
)]
pub struct MeleeWeapon;

impl MeleeWeaponBuilder {
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            ChildOf(tip_entity),
            MeleeWeapon,
            self.sprite,
            self.collider,
            self.damage,
        )
    }
}
#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
#[require(
    Weapon,
    UseTime(Timer::new(Duration::from_millis(500), TimerMode::Once))
)]
pub struct RangedWeapon;

#[derive(Component)]
pub struct RangedWeaponBuilder {
    pub projectile_builder: ProjectileBuilder,
    pub sprite: Sprite,
}
impl RangedWeaponBuilder {
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            RangedWeapon,
            ChildOf(tip_entity),
            self.projectile_builder,
            self.sprite,
        )
    }
}

#[derive(Component, Clone)]
pub struct EnergyCost(pub u32);
impl Default for EnergyCost {
    fn default() -> Self {
        Self(5)
    }
}

#[derive(Component, Clone, Default)]
pub struct ProjectileBuilder {
    pub linear_velocity: f32,
    pub gravity_scale: f32,
    pub sprite: Sprite,
    pub energy_cost: EnergyCost,
}
impl ProjectileBuilder {
    fn build(self, direction: Dir2) -> impl Bundle {
        (
            Projectile,
            GravityScale(self.gravity_scale),
            LinearVelocity(*(direction) * self.linear_velocity),
            self.sprite,
            self.energy_cost,
            Despawnable,
            PlayerHitbox,
        )
    }
}
#[derive(Component, Default)]
#[require(Damage(10), CollisionEventsEnabled, Sensor, CollisionLayers, Collider)]
pub struct Hitbox;

#[derive(Component)]
#[require(Transform, Visibility = Visibility::Inherited)]
pub struct RotationCenter;

#[derive(Component)]
pub struct SwingRotation(f32);

#[derive(Component)]
#[require(Visibility = Visibility::Inherited, Transform = Transform::from_xyz(200.0, 0.0, 0.0))]
pub struct WeaponTip;

#[derive(Component)]
pub struct CooldownFinished(bool);

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
            ..default()
        },
    }
    .build(tip_entity)
}
