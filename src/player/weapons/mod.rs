use bevy::prelude::*;

#[derive(Bundle)]
pub struct Weapon {
    damage: Damage,
}

#[derive(Component)]
pub struct Damage(u32);
