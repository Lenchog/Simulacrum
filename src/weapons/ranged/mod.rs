use crate::{prelude::*, weapons::prelude::*};

pub mod general_ranged;
pub mod grappling_hook;
pub mod rocket_launcher;
pub mod shoot;

#[derive(Event)]
pub struct ShootEvent(pub Entity);

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Default)]
pub struct RangedWeaponBuilder {
    pub projectile_builder: ProjectileBuilder,
    pub sprite: Sprite,
    pub usetime: UseTime,
}
impl RangedWeaponBuilder {
    fn build(self, tip_entity: Entity, weapon_type: WeaponType) -> impl Bundle {
        (
            Weapon(weapon_type),
            ChildOf(tip_entity),
            self.usetime,
            self.projectile_builder,
            self.sprite,
        )
    }
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub enum ProjectileType {
    #[default]
    Normal,
    Rocket,
    Hook,
}

#[derive(Component, Clone, Default)]
pub struct ProjectileBuilder {
    pub speed: f32,
    pub gravity_scale: f32,
    pub sprite: Sprite,
    pub energy_cost: EnergyCost,
    pub damage: Damage,
    pub projectile_type: ProjectileType,
}
impl ProjectileBuilder {
    fn build(self, direction: Dir2) -> impl Bundle {
        (
            Projectile,
            Sensor,
            GravityScale(self.gravity_scale),
            LinearVelocity(*(direction) * self.speed),
            self.damage,
            self.sprite,
            self.energy_cost,
            self.projectile_type,
            ProjectileSpeed(self.speed),
            PlayerHitbox,
        )
    }
}
