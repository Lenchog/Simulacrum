use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::Actions;

use crate::{
    Recoil,
    robot::{
        PhysicsLayers, Robot, RobotCollider,
        health::*,
        player::{
            input::NormalMovement,
            movement::DashTimer,
            weapons::{RotationCenter, WeaponTip},
        },
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
    DashTimer = DashTimer(Timer::new(Duration::from_millis(500), TimerMode::Once))
)]
pub struct Player;

pub fn add_player() -> impl Bundle {
    (
        Player,
        (
            RigidBody::Dynamic,
            children![PlayerCollider, (RotationCenter, children!(WeaponTip))],
        ),
    )
}
