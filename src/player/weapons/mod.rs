use std::time::Duration;

use avian2d::prelude::{Collider, GravityScale, LinearVelocity, RigidBody, Sensor};
use bevy::prelude::*;

pub mod attack;

#[derive(Component)]
pub struct UseTime(Timer);

#[derive(Component)]
pub struct Weapon;

#[derive(Bundle)]
pub struct WeaponBundle {
    damage: Damage,
    use_time: UseTime,
    weapon: Weapon,
    sensor: Sensor,
    transform: Transform,
}
impl Default for WeaponBundle {
    fn default() -> Self {
        Self {
            weapon: Weapon,
            damage: Damage(30),
            use_time: UseTime(Timer::new(Duration::from_millis(500), TimerMode::Once)),
            sensor: Sensor,
            transform: Transform::from_translation(Vec3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            }),
        }
    }
}

#[derive(Bundle)]
pub struct MeleeWeapon {
    weapon: WeaponBundle,
    collider: Collider,
}
impl Default for MeleeWeapon {
    fn default() -> Self {
        Self {
            weapon: WeaponBundle::default(),
            collider: Collider::rectangle(1920.0, 440.0),
        }
    }
}

/* #[derive(Bundle)]
pub struct RangedWeapon {
    weapon: WeaponBundle,
    weapon_tip: WeaponTip,
}
impl Default for RangedWeapon {
    fn default() -> Self {
        Self {
            weapon: Weapon::default(),
            weapon_tip
        }
    }
} */

#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    collider: Collider,
    body: RigidBody,
    damage: Damage,
    velocity: LinearVelocity,
    gravity_scale: GravityScale,
    projectile: Projectile,
    sensor: Sensor,
}
impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            collider: Collider::circle(50.0),
            body: RigidBody::Dynamic,
            damage: Damage(30),
            gravity_scale: GravityScale(0.0),
            velocity: LinearVelocity(Vec2 { x: 0.0, y: 0.0 }),
            projectile: Projectile,
            sensor: Sensor,
        }
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
