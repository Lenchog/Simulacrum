use bevy_simple_subsecond_system::hot;
use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::Actions;

use crate::robot::{
    PhysicsLayers, Robot, RobotCollider,
    health::*,
    player::{
        input::NormalMovement,
        movement::DashCooldown,
        movement::DashTimer,
        weapons::{Recoil, RotationCenter, WeaponTip},
    },
};

pub mod input;
pub mod movement;
pub mod weapons;

#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Resource)]
pub struct EquippedWeapons {
    pub left: Option<Entity>,
    pub right: Option<Entity>,
}

#[derive(Component, PartialEq)]
#[require(
    RobotCollider,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Player,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
)]
pub struct PlayerCollider;

#[derive(Component, PartialEq, Default)]
#[require(
    Robot,
    Recoil,
    Health = Health(500),
    Actions<NormalMovement>,
    DashCooldown = DashCooldown(Timer::new(Duration::from_millis(300), TimerMode::Once)),
    DashTimer = DashTimer(Timer::new(Duration::from_millis(150), TimerMode::Once)),
)]
pub struct Player;

#[hot]
pub fn add_player() -> impl Bundle {
    (
        Player,
        (
            RigidBody::Dynamic,
            children![PlayerCollider, (RotationCenter, children!(WeaponTip))],
        ),
    )
}
